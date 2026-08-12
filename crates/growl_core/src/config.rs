use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Wiki root, relative to the configuration file when it is not absolute.
    #[serde(alias = "wiki_root")]
    pub root: Option<PathBuf>,
    /// Directory roles keyed by their path relative to the wiki root.
    #[serde(default)]
    pub directories: BTreeMap<String, DirectoryConfig>,
    pub common_fields: BTreeMap<String, FieldRule>,
    pub types: BTreeMap<String, TypeConfig>,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|error| format!("cannot read config {}: {error}", path.display()))?;
        serde_yaml::from_str(&content).map_err(|error| format!("cannot parse config {}: {error}", path.display()))
    }

    pub fn root_path(&self, config_path: &Path) -> Option<PathBuf> {
        self.root.as_ref().map(|root| {
            if root.is_absolute() {
                root.clone()
            } else {
                config_path.parent().unwrap_or_else(|| Path::new(".")).join(root)
            }
        })
    }
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
