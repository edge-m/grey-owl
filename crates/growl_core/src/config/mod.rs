mod schema;

use std::fs;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub use schema::{
    ConfigLintConfig, DirectoryConfig, FieldRule, MandatoryFieldRule, TypeConfig, ValueType, WikiLintConfig,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Wiki root, relative to the configuration file when it is not absolute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_root: Option<PathBuf>,
    /// Directory roles keyed by their path relative to the wiki root.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub directories: IndexMap<String, DirectoryConfig>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub mandatory_fields: IndexMap<String, MandatoryFieldRule>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub types: IndexMap<String, TypeConfig>,
    pub wiki_lint: WikiLintConfig,
    pub config_lint: ConfigLintConfig,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|error| format!("cannot read config {}: {error}", path.display()))?;
        serde_yaml::from_str(&content).map_err(|error| format!("cannot parse config {}: {error}", path.display()))
    }

    pub fn wiki_root_path(&self, config_path: &Path) -> Option<PathBuf> {
        self.wiki_root.as_ref().map(|wiki_root| {
            if wiki_root.is_absolute() {
                wiki_root.clone()
            } else {
                config_path.parent().unwrap_or_else(|| Path::new(".")).join(wiki_root)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_is_rejected_in_mandatory_fields() {
        let result: Result<Config, _> =
            serde_yaml::from_str("mandatory_fields:\n  title:\n    type: string\n    optional: true\n");

        assert!(result.is_err());
    }
}
