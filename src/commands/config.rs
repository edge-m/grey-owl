use std::path::PathBuf;

use clap::Args as ClapArgs;

use crate::output;

use super::check::Format;
use super::context;

#[derive(Debug, ClapArgs)]
pub struct LintArgs {
    #[arg(long, help = "YAML configuration file")]
    pub config: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = Format::Human, help = "Diagnostic output format")]
    pub format: Format,
}

pub fn lint(args: &LintArgs) -> Result<u8, String> {
    let config_context = context::load(args.config.as_deref())?;
    let diagnostics = growl_core::validation::lint_config::lint(config_context.config());
    output::print_diagnostics(&diagnostics, args.format.into())?;

    Ok(if diagnostics.iter().any(|diagnostic| diagnostic.is_error()) { 1 } else { 0 })
}
