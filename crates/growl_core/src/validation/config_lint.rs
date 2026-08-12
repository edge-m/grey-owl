use std::collections::BTreeMap;

use crate::config::{Config, FieldRule, ValueType};
use crate::diagnostic::Diagnostic;

pub fn lint(config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    lint_fields("common_fields", &config.common_fields, &mut diagnostics);

    for (document_type, type_config) in &config.types {
        lint_fields(&format!("types.{document_type}.fields"), &type_config.fields, &mut diagnostics);
    }

    diagnostics
}

fn lint_fields(scope: &str, fields: &BTreeMap<String, FieldRule>, diagnostics: &mut Vec<Diagnostic>) {
    for (field, rule) in fields {
        if !rule.values.is_empty() && !matches!(rule.value_type, ValueType::String) {
            diagnostics.push(Diagnostic::error(
                "config-values-require-string",
                Some(format!("{scope}.{field}")),
                "allowed values are currently supported only for string fields",
            ));
        }
    }
}
