mod document;
mod workspace;

use crate::config::Config;
use crate::diagnostic::Diagnostic;
use crate::workspace::ScanResult;

pub fn validate(scan: &ScanResult, config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = scan.diagnostics.clone();
    diagnostics.extend(scan.documents.iter().flat_map(|document| document::validate(document, config)));
    diagnostics.extend(workspace::validate(scan, config));
    diagnostics
}
