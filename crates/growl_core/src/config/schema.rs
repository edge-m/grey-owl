use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WikiLintConfig {}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ConfigLintConfig {
    pub max_nesting_depth: Option<usize>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectoryConfig {
    pub description: Option<String>,
    pub directories: BTreeMap<String, DirectoryConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TypeConfig {
    pub description: Option<String>,
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
