use std::collections::BTreeMap;

use serde_yaml::{Mapping, Value};

use crate::diagnostic::Diagnostic;

#[derive(Debug)]
pub struct Document {
    pub path: String,
    pub metadata: Mapping,
    pub body: String,
}

pub fn parse(path: String, content: &str) -> Result<Document, Diagnostic> {
    let (frontmatter, body) = split_frontmatter(content).ok_or_else(|| {
        Diagnostic::error(
            "invalid-frontmatter",
            Some(path.clone()),
            "frontmatter must start with '---' and end with a matching delimiter",
        )
    })?;

    let metadata: Value = serde_yaml::from_str(frontmatter).map_err(|error| {
        Diagnostic::error("invalid-frontmatter", Some(path.clone()), format!("cannot parse YAML frontmatter: {error}"))
    })?;
    let metadata = match metadata {
        Value::Mapping(mapping) => mapping,
        _ => {
            return Err(Diagnostic::error(
                "invalid-frontmatter",
                Some(path),
                "frontmatter must contain a YAML mapping",
            ));
        }
    };

    Ok(Document { path, metadata, body: body.to_string() })
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let mut lines = content.split_inclusive('\n');
    let first = lines.next()?;
    if first.trim() != "---" {
        return None;
    }
    let frontmatter_start = first.len();
    let mut offset = frontmatter_start;
    for line in lines {
        let line_start = offset;
        offset += line.len();
        if line.trim() == "---" {
            let content_end = line_start;
            let body_start = offset;
            return Some((&content[frontmatter_start..content_end], &content[body_start..]));
        }
    }
    None
}

pub fn string_value(metadata: &Mapping, field: &str) -> Option<String> {
    metadata.get(Value::String(field.to_string())).and_then(Value::as_str).map(ToOwned::to_owned)
}

pub fn has_field(metadata: &Mapping, field: &str) -> bool {
    metadata.contains_key(Value::String(field.to_string()))
}

pub fn values(metadata: &Mapping) -> BTreeMap<String, &Value> {
    metadata.iter().filter_map(|(key, value)| Some((key.as_str()?.to_string(), value))).collect()
}
