use std::path::PathBuf;

use crate::{config::Config, output, validation, workspace::Workspace};

pub fn run(args: &[String]) -> Result<u8, String> {
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
            value if value.starts_with('-') => return Err(format!("unknown option '{value}'")),
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
    let diagnostics = validation::validate(&scanned, &config);
    output::print_diagnostics(&diagnostics, format)?;

    Ok(if diagnostics.iter().any(|diagnostic| diagnostic.is_error()) { 1 } else { 0 })
}
