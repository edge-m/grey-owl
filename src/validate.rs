use std::collections::BTreeMap;

use serde_yaml::Value;

use crate::config::{Config, FieldRule, ValueType};
use crate::diagnostic::Diagnostic;
use crate::document::string_value;
use crate::workspace::ScanResult;

pub fn validate(scan: &ScanResult, config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = scan.diagnostics.clone();
    let mut identifiers = BTreeMap::<String, String>::new();

    for document in &scan.documents {
        validate_fields(&document.path, &document.metadata, &config.common_fields, &mut diagnostics);

        let document_type = string_value(&document.metadata, "type");
        if let Some(document_type) = &document_type {
            match config.types.get(document_type) {
                Some(type_config) => {
                    validate_fields(&document.path, &document.metadata, &type_config.fields, &mut diagnostics);
                }
                None if !config.types.is_empty() => diagnostics.push(Diagnostic::error(
                    "unknown-document-type",
                    Some(document.path.clone()),
                    format!("unknown document type '{document_type}'"),
                )),
                None => {}
            }
        }

        if let Some(identifier) = string_value(&document.metadata, "id")
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

fn validate_fields(
    path: &str, metadata: &serde_yaml::Mapping, rules: &BTreeMap<String, FieldRule>, diagnostics: &mut Vec<Diagnostic>,
) {
    for (field, rule) in rules {
        match metadata.get(Value::String(field.clone())) {
            None if !rule.optional => diagnostics.push(missing_field(path, field)),
            Some(value) => validate_value(path, field, value, rule, diagnostics),
            None => {}
        }
    }
}

fn validate_value(path: &str, field: &str, value: &Value, rule: &FieldRule, diagnostics: &mut Vec<Diagnostic>) {
    if !matches_type(value, &rule.value_type) {
        diagnostics.push(Diagnostic::error(
            "invalid-field-type",
            Some(path.to_string()),
            format!("field '{field}' has an invalid value type"),
        ));
    }
    if !rule.values.is_empty() {
        let valid = value.as_str().is_some_and(|value| rule.values.iter().any(|allowed| allowed == value));
        if !valid {
            diagnostics.push(Diagnostic::error(
                "invalid-field-value",
                Some(path.to_string()),
                format!("field '{field}' has a value outside the configured values"),
            ));
        }
    }
}

fn missing_field(path: &str, field: &str) -> Diagnostic {
    Diagnostic::error("missing-required-field", Some(path.to_string()), format!("required field '{field}' is missing"))
}

fn matches_type(value: &Value, expected: &ValueType) -> bool {
    match expected {
        ValueType::String => value.is_string(),
        ValueType::Boolean => value.is_bool(),
        ValueType::Number => value.is_number(),
        ValueType::Array => value.is_sequence(),
        ValueType::Object => value.is_mapping(),
    }
}
