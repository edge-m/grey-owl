use std::collections::BTreeMap;

use crate::diagnostic::Diagnostic;
use crate::document;
use crate::workspace::ScanResult;

pub fn validate(scan: &ScanResult, _config: &crate::config::Config) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut identifiers = BTreeMap::<String, String>::new();

    for document in &scan.documents {
        if let Some(identifier) = document::string_value(&document.metadata, "id")
            && let Some(previous) = identifiers.insert(identifier.clone(), document.path.clone())
        {
            diagnostics.push(Diagnostic::error(
                "duplicate-identifier",
                Some(document.path.clone()),
                format!("identifier '{identifier}' is already used by {previous}"),
            ));
        }
    }

    diagnostics
}
