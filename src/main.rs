use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use grey_owl::{config::Config, output, validate, workspace::Workspace};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("growl: {message}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<u8, String> {
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        print_help();
        return Ok(0);
    }

    if args[0] != "check" {
        return Err(format!("unknown command '{}'; try 'growl --help'", args[0]));
    }

    let mut wiki_path = None;
    let mut config_path = None;
    let mut format = output::OutputFormat::Human;
    let mut index = 1;

    while index < args.len() {
        match args[index].as_str() {
            "--config" => {
                index += 1;
                config_path =
                    Some(PathBuf::from(args.get(index).ok_or_else(|| "--config requires a file path".to_string())?));
            }
            "--format" => {
                index += 1;
                format = args.get(index).ok_or_else(|| "--format requires 'human' or 'json'".to_string())?.parse()?;
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown option '{value}'"));
            }
            value if wiki_path.is_none() => wiki_path = Some(PathBuf::from(value)),
            value => return Err(format!("unexpected argument '{value}'")),
        }
        index += 1;
    }

    let wiki_path = wiki_path.ok_or_else(|| "check requires a wiki path".to_string())?;
    let config = match config_path {
        Some(path) => Config::from_path(&path)?,
        None => Config::default(),
    };
    let scanned = Workspace::new(wiki_path).scan()?;
    let diagnostics = validate::validate(&scanned, &config);
    output::print_diagnostics(&diagnostics, format)?;

    Ok(if diagnostics.iter().any(|diagnostic| diagnostic.is_error()) { 1 } else { 0 })
}

fn print_help() {
    println!(
        "growl — Grey Owl wiki validator\n\n\
Usage:\n  growl check <wiki-path> [--config <file>] [--format human|json]\n\n\
Options:\n  --config <file>       YAML configuration file\n  --format <format>     human (default) or json\n  -h, --help            Show this help"
    );
}
