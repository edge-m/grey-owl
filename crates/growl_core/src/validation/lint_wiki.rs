use std::collections::BTreeMap;

use serde_yaml::Value;

use crate::config::{Config, FieldRule, ValueType};
use crate::diagnostic::Diagnostic;
use crate::document::{self, Document};
use crate::workspace::ScanResult;

pub fn lint(scan: &ScanResult, config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = scan.diagnostics.clone();
    diagnostics.extend(scan.documents.iter().flat_map(|document| lint_document(document, config)));
    diagnostics.extend(lint_workspace(scan));
    diagnostics
}

fn lint_document(document: &Document, config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    lint_fields(
        &document.relative_file_path_from_wiki_root,
        &document.frontmatter,
        &config.common_fields,
        &mut diagnostics,
    );

    if let Some(document_type) = document::string_value(&document.frontmatter, "type") {
        match config.types.get(&document_type) {
            Some(type_config) => lint_fields(
                &document.relative_file_path_from_wiki_root,
                &document.frontmatter,
                &type_config.fields,
                &mut diagnostics,
            ),
            None if !config.types.is_empty() => diagnostics.push(Diagnostic::error(
                "unknown-document-type",
                Some(document.relative_file_path_from_wiki_root.clone()),
                format!("unknown document type '{document_type}'"),
            )),
            None => {}
        }
    }

    diagnostics
}

fn lint_fields(
    path: &str, frontmatter: &serde_yaml::Mapping, rules: &BTreeMap<String, FieldRule>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (field, rule) in rules {
        match frontmatter.get(Value::String(field.clone())) {
            None if !rule.optional => diagnostics.push(Diagnostic::error(
                "missing-required-field",
                Some(path.to_string()),
                format!("required field '{field}' is missing"),
            )),
            Some(value) => lint_value(path, field, value, rule, diagnostics),
            None => {}
        }
    }
}

fn lint_value(path: &str, field: &str, value: &Value, rule: &FieldRule, diagnostics: &mut Vec<Diagnostic>) {
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

fn matches_type(value: &Value, expected: &ValueType) -> bool {
    match expected {
        ValueType::String => value.is_string(),
        ValueType::Boolean => value.is_bool(),
        ValueType::Number => value.is_number(),
        ValueType::Array => value.is_sequence(),
        ValueType::Object => value.is_mapping(),
    }
}

fn lint_workspace(scan: &ScanResult) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut identifiers = BTreeMap::<String, String>::new();

    for document in &scan.documents {
        if let Some(identifier) = document::string_value(&document.frontmatter, "id")
            && let Some(previous) =
                identifiers.insert(identifier.clone(), document.relative_file_path_from_wiki_root.clone())
        {
            diagnostics.push(Diagnostic::error(
                "duplicate-identifier",
                Some(document.relative_file_path_from_wiki_root.clone()),
                format!("identifier '{identifier}' is already used by {previous}"),
            ));
        }
    }

    diagnostics
}
