use std::str::FromStr;

use growl_core::diagnostic::Diagnostic;

#[derive(Clone, Copy)]
pub enum OutputFormat {
    Human,
    Json,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "human" => Ok(Self::Human),
            "json" => Ok(Self::Json),
            _ => Err(format!("unknown format '{value}'; expected 'human' or 'json'")),
        }
    }
}

pub fn print_diagnostics(diagnostics: &[Diagnostic], format: OutputFormat) -> Result<(), String> {
    match format {
        OutputFormat::Human => {
            if diagnostics.is_empty() {
                println!("OK: no issues found");
            } else {
                for diagnostic in diagnostics {
                    let path = diagnostic.path.as_deref().unwrap_or("-");
                    println!("{} [{}] {}: {}", path, diagnostic.code, severity_name(diagnostic), diagnostic.message);
                }
                println!("{} diagnostic(s)", diagnostics.len());
            }
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(diagnostics)
                    .map_err(|error| format!("cannot serialize diagnostics: {error}"))?
            );
        }
    }
    Ok(())
}

fn severity_name(diagnostic: &Diagnostic) -> &'static str {
    if diagnostic.is_error() { "error" } else { "info" }
}
