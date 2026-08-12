use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::diagnostic::Diagnostic;
use crate::document::{self, Document};

pub struct ScanResult {
    pub documents: Vec<Document>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn scan(&self) -> Result<ScanResult, String> {
        if !self.root.is_dir() {
            return Err(format!("wiki path is not a directory: {}", self.root.display()));
        }

        let mut documents = Vec::new();
        let mut diagnostics = Vec::new();
        for entry in WalkDir::new(&self.root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() || !is_markdown(entry.path()) {
                continue;
            }
            let relative = relative_path(&self.root, entry.path())?;
            let content = fs::read_to_string(entry.path())
                .map_err(|error| format!("cannot read {}: {error}", entry.path().display()))?;
            match document::parse(relative, &content) {
                Ok(document) => documents.push(document),
                Err(diagnostic) => diagnostics.push(diagnostic),
            }
        }

        Ok(ScanResult { documents, diagnostics })
    }
}

fn is_markdown(path: &Path) -> bool {
    path.extension().and_then(|extension| extension.to_str()) == Some("md")
}

fn relative_path(root: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(root)
        .map_err(|error| format!("cannot calculate relative path: {error}"))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
}
