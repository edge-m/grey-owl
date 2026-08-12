use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub common_fields: BTreeMap<String, FieldRule>,
    pub types: BTreeMap<String, TypeConfig>,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|error| format!("cannot read config {}: {error}", path.display()))?;
        serde_yaml::from_str(&content).map_err(|error| format!("cannot parse config {}: {error}", path.display()))
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TypeConfig {
    pub fields: BTreeMap<String, FieldRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FieldRule {
    #[serde(rename = "type")]
    pub value_type: ValueType,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValueType {
    String,
    Boolean,
    Number,
    Array,
    Object,
}
