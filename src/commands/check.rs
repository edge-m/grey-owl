use std::path::PathBuf;

use clap::{Args as ClapArgs, ValueEnum};

use crate::output;
use growl_core::{config::Config, validation, workspace::Workspace};

#[derive(Debug, ClapArgs)]
pub struct Args {
    #[arg(help = "Wiki directory to validate")]
    pub wiki_path: Option<PathBuf>,
    #[arg(long, help = "YAML configuration file")]
    pub config: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = Format::Human, help = "Diagnostic output format")]
    pub format: Format,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Format {
    Human,
    Json,
}

impl From<Format> for output::OutputFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Human => Self::Human,
            Format::Json => Self::Json,
        }
    }
}

pub fn run(args: &Args) -> Result<u8, String> {
    let loaded_config = match args.config {
        Some(ref path) => Config::from_path(path)?,
        None => Config::default(),
    };
    let wiki_path = match &args.wiki_path {
        Some(path) => path.clone(),
        None => args
            .config
            .as_deref()
            .and_then(|path| loaded_config.root_path(path))
            .ok_or_else(|| "check requires a wiki path or a root in the configuration".to_string())?,
    };
    let scanned = Workspace::new(wiki_path).scan()?;
    let diagnostics = validation::validate(&scanned, &loaded_config);
    output::print_diagnostics(&diagnostics, args.format.into())?;

    Ok(if diagnostics.iter().any(|diagnostic| diagnostic.is_error()) { 1 } else { 0 })
}
