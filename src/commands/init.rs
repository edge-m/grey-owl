use std::fs;
use std::path::PathBuf;

use clap::Args as ClapArgs;

const CONFIG_NAME: &str = "growl.yml";
const DEFAULT_CONFIG: &str = r#"wiki_root: .

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

wiki_lint: {}
config_lint:
  max_nesting_depth: 1
"#;

#[derive(Debug, ClapArgs)]
pub struct Args {}

pub fn run(_args: &Args) -> Result<u8, String> {
    let config_path = PathBuf::from(CONFIG_NAME);
    if config_path.exists() {
        return Err(format!("configuration file already exists: {}", config_path.display()));
    }

    fs::write(&config_path, DEFAULT_CONFIG)
        .map_err(|error| format!("cannot write configuration {}: {error}", config_path.display()))?;
    println!("wrote {}", config_path.display());

    Ok(0)
}
