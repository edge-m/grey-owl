use growl_core::diagnostic::Diagnostic;

pub fn print_diagnostics(diagnostics: &[Diagnostic]) {
    if diagnostics.is_empty() {
        println!("OK: no issues found");
    } else {
        println!("Found {} diagnostic(s):", diagnostics.len());
        println!();
        for diagnostic in diagnostics {
            println!("{}: [{}]", severity_name(diagnostic), diagnostic.code);
            if let Some(path) = diagnostic.path.as_deref() {
                println!("  path: {path}");
            }
            println!("  message: {}", diagnostic.message);
            if let Some(help) = diagnostic_help(diagnostic) {
                println!("  help: {help}");
            }
            println!();
        }
    }
}

fn severity_name(diagnostic: &Diagnostic) -> &'static str {
    if diagnostic.is_error() { "error" } else { "info" }
}

fn diagnostic_help(diagnostic: &Diagnostic) -> Option<&'static str> {
    match diagnostic.code.as_str() {
        "config-values-require-string" => Some("use 'values' only with type: string"),
        "config-items-require-array" => Some("use 'items' only with type: array"),
        "config-fields-require-object" => Some("use 'fields' only with type: object"),
        "config-growl-version-missing" => Some("add growl_version: 0.1.0 to the configuration"),
        "config-growl-version-incompatible" => Some("set growl_version to a version supported by this binary"),
        _ => None,
    }
}
