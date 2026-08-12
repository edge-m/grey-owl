use std::fs;
use std::path::PathBuf;

use crate::config::{CONFIG_NAME, default_config};

pub fn run(args: &[String]) -> Result<u8, String> {
    if !args.is_empty() {
        return Err("usage: growl init".to_string());
    }

    let config_path = PathBuf::from(CONFIG_NAME);
    if config_path.exists() {
        return Err(format!("configuration file already exists: {}", config_path.display()));
    }

    fs::write(&config_path, default_config())
        .map_err(|error| format!("cannot write configuration {}: {error}", config_path.display()))?;
    println!("wrote {}", config_path.display());

    Ok(0)
}
