use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use clap::Args as ClapArgs;
use growl_core::config::{Config, ConfigLintConfig, DirectoryConfig, MandatoryFieldRule, TypeConfig, ValueType};

const CONFIG_NAME: &str = "growl.yml";

#[derive(Debug, ClapArgs)]
pub struct Args {}

fn mandatory_field(value_type: ValueType) -> MandatoryFieldRule {
    MandatoryFieldRule { value_type, values: Vec::new(), items: None, fields: BTreeMap::new() }
}

pub fn run(_args: &Args) -> Result<u8, String> {
    let config_path = PathBuf::from(CONFIG_NAME);
    if config_path.exists() {
        return Err(format!("configuration file already exists: {}", config_path.display()));
    }

    let config = Config {
        wiki_root: Some(PathBuf::from(".")),
        directories: BTreeMap::from([(
            "raw".to_string(),
            DirectoryConfig {
                description: Some("Raw data source; files can be added here freely".to_string()),
                directories: BTreeMap::from([(
                    "inbox".to_string(),
                    DirectoryConfig { description: Some("Incoming raw files".to_string()), ..Default::default() },
                )]),
            },
        )]),
        mandatory_fields: BTreeMap::from([
            ("type".to_string(), mandatory_field(ValueType::String)),
            ("title".to_string(), mandatory_field(ValueType::String)),
            ("description".to_string(), mandatory_field(ValueType::String)),
            (
                "tags".to_string(),
                MandatoryFieldRule {
                    items: Some(Box::new(mandatory_field(ValueType::String))),
                    ..mandatory_field(ValueType::Array)
                },
            ),
            (
                "sources".to_string(),
                MandatoryFieldRule {
                    items: Some(Box::new(mandatory_field(ValueType::String))),
                    ..mandatory_field(ValueType::Array)
                },
            ),
            (
                "generated".to_string(),
                MandatoryFieldRule {
                    fields: BTreeMap::from([
                        ("at".to_string(), mandatory_field(ValueType::Datetime)),
                        ("by".to_string(), mandatory_field(ValueType::String)),
                    ]),
                    ..mandatory_field(ValueType::Object)
                },
            ),
            ("stale_after".to_string(), mandatory_field(ValueType::Date)),
        ]),
        types: BTreeMap::from([(
            "note".to_string(),
            TypeConfig { description: Some("A general-purpose note".to_string()), fields: BTreeMap::new() },
        )]),
        config_lint: ConfigLintConfig { max_nesting_depth: Some(1) },
        ..Default::default()
    };
    let content =
        serde_yaml::to_string(&config).map_err(|error| format!("cannot serialize default configuration: {error}"))?;

    fs::write(&config_path, content)
        .map_err(|error| format!("cannot write configuration {}: {error}", config_path.display()))?;
    println!("wrote {}", config_path.display());

    Ok(0)
}
