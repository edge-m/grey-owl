use std::path::PathBuf;

use clap::{Args as ClapArgs, ValueEnum};

use crate::output;
use growl_core::{validation, workspace::Workspace};

use super::context;

#[derive(Debug, ClapArgs)]
pub struct Args {
    #[arg(long, help = "YAML configuration file")]
    pub config: Option<PathBuf>,
    #[arg(long, help = "Single wiki file to validate")]
    pub file: Option<PathBuf>,
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
    let config_context = context::load(args.config.as_deref())?;
    let config_diagnostics = validation::lint_config::lint(config_context.config());
    if !config_diagnostics.is_empty() {
        output::print_diagnostics(&config_diagnostics, args.format.into())?;
        if config_diagnostics.iter().any(|diagnostic| diagnostic.is_error()) {
            return Ok(2);
        }
    }
    let scanned = Workspace::new(config_context.wiki_root().to_path_buf()).scan()?;
    let mut diagnostics = validation::lint_wiki::lint(&scanned, config_context.config());
    diagnostics.extend(validation::orphans::find(&scanned));
    if let Some(file) = &args.file {
        let target = file.to_string_lossy().replace('\\', "/");
        if !scanned.documents.iter().any(|document| document.relative_file_path_from_wiki_root == target)
            && !scanned.diagnostics.iter().any(|diagnostic| diagnostic.path.as_deref() == Some(target.as_str()))
        {
            diagnostics.push(growl_core::diagnostic::Diagnostic::error(
                "file-not-found",
                Some(target.clone()),
                "file does not exist or is not a Markdown document",
            ));
        }
        diagnostics.retain(|diagnostic| diagnostic.path.as_deref() == Some(target.as_str()));
    }
    output::print_diagnostics(&diagnostics, args.format.into())?;

    Ok(if diagnostics.iter().any(|diagnostic| diagnostic.is_error()) { 1 } else { 0 })
}
