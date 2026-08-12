use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use grey_owl::{config::Config, output, validate, workspace::Workspace};

const SKILL_NAME: &str = "growl";
const SKILL_CONTENT: &str = include_str!("../skills/growl/SKILL.md");
const CONFIG_NAME: &str = "growl.yml";
const DEFAULT_CONFIG: &str = r#"root: .

directories:
  raw:
    description: Raw data source; files can be added here freely
    directories:
      inbox:
        description: Incoming raw files

common_fields:
  id:
    type: string
  type:
    type: string
types:
  note:
    description: A general-purpose note
    fields:
      title:
        type: string
      status:
        type: string
        optional: true
        values: [draft, active]
"#;

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

    match args[0].as_str() {
        "check" => run_check(&args[1..]),
        "init" => run_init(&args[1..]),
        "skill" => run_skill(&args[1..]),
        command => Err(format!("unknown command '{command}'; try 'growl --help'")),
    }
}

fn run_init(args: &[String]) -> Result<u8, String> {
    if !args.is_empty() {
        return Err("usage: growl init".to_string());
    }

    let config_path = PathBuf::from(CONFIG_NAME);
    if config_path.exists() {
        return Err(format!("configuration file already exists: {}", config_path.display()));
    }

    fs::write(&config_path, DEFAULT_CONFIG)
        .map_err(|error| format!("cannot write configuration {}: {error}", config_path.display()))?;
    println!("wrote {}", config_path.display());

    Ok(0)
}

fn run_check(args: &[String]) -> Result<u8, String> {
    let mut wiki_path = None;
    let mut config_path = None;
    let mut format = output::OutputFormat::Human;
    let mut index = 0;

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

    let config = match config_path {
        Some(ref path) => Config::from_path(path)?,
        None => Config::default(),
    };
    let wiki_path = match wiki_path {
        Some(path) => path,
        None => config_path
            .as_deref()
            .and_then(|path| config.root_path(path))
            .ok_or_else(|| "check requires a wiki path or a root in the configuration".to_string())?,
    };
    let scanned = Workspace::new(wiki_path).scan()?;
    let diagnostics = validate::validate(&scanned, &config);
    output::print_diagnostics(&diagnostics, format)?;

    Ok(if diagnostics.iter().any(|diagnostic| diagnostic.is_error()) { 1 } else { 0 })
}

fn run_skill(args: &[String]) -> Result<u8, String> {
    if args.len() != 1 || args[0] == "--help" || args[0] == "-h" {
        return Err("usage: growl skill <output-directory>".to_string());
    }

    let output_directory = PathBuf::from(&args[0]);
    let skill_directory = output_directory.join(SKILL_NAME);
    fs::create_dir_all(&skill_directory)
        .map_err(|error| format!("cannot create skill directory {}: {error}", skill_directory.display()))?;

    let skill_path = skill_directory.join("SKILL.md");
    fs::write(&skill_path, SKILL_CONTENT)
        .map_err(|error| format!("cannot write skill {}: {error}", skill_path.display()))?;
    println!("wrote {}", skill_path.display());

    Ok(0)
}

fn print_help() {
    println!(
        "growl — Grey Owl wiki validator\n\n\
Usage:\n  growl init\n  growl check [<wiki-path>] [--config <file>] [--format human|json]\n  growl skill <output-directory>\n\n\
Options:\n  --config <file>       YAML configuration file\n  --format <format>     human (default) or json\n  -h, --help            Show this help"
    );
}
