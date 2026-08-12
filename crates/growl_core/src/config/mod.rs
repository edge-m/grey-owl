mod schema;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

pub use schema::{ConfigLintConfig, DirectoryConfig, FieldRule, TypeConfig, ValueType, WikiLintConfig};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Wiki root, relative to the configuration file when it is not absolute.
    pub wiki_root: Option<PathBuf>,
    /// Directory roles keyed by their path relative to the wiki root.
    pub directories: BTreeMap<String, DirectoryConfig>,
    pub common_fields: BTreeMap<String, FieldRule>,
    pub types: BTreeMap<String, TypeConfig>,
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
