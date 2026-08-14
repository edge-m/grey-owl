use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Args as ClapArgs, ValueEnum};
use serde::Serialize;

use super::context;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Format {
    Human,
    Json,
}

#[derive(Debug, ClapArgs)]
pub struct Args {
    #[arg(long, help = "YAML configuration file")]
    pub config: Option<PathBuf>,
    #[arg(long, help = "Report files modified before YYYY-MM-DD")]
    pub stale_before: Option<NaiveDate>,
    #[arg(long, value_enum, default_value_t = Format::Human)]
    pub format: Format,
    #[arg(long, help = "Accepted for compatibility; maintenance detection never changes files")]
    pub dry_run: bool,
}

#[derive(Debug, Serialize)]
struct Candidate {
    kind: String,
    page_id: String,
    path: Option<String>,
    reason: String,
}

pub fn run(args: &Args) -> Result<u8, String> {
    let context = context::load(args.config.as_deref())?;
    let scan = growl_core::workspace::Workspace::new(context.wiki_root().to_path_buf())
        .with_excludes(&context.config().wiki_lint.exclude)
        .scan()?;
    let graph = growl_core::graph::WikiGraph::from_scan(&scan);
    let mut candidates = Vec::new();

    for page_id in &graph.orphan_pages {
        candidates.push(Candidate {
            kind: "orphan".to_string(),
            page_id: page_id.clone(),
            path: scan
                .documents
                .iter()
                .find(|document| document.page_id() == *page_id)
                .map(|document| document.relative_file_path_from_wiki_root.clone()),
            reason: "page has no incoming page links".to_string(),
        });
    }
    for page_id in &graph.unreachable_pages {
        candidates.push(Candidate {
            kind: "unreachable".to_string(),
            page_id: page_id.clone(),
            path: scan
                .documents
                .iter()
                .find(|document| document.page_id() == *page_id)
                .map(|document| document.relative_file_path_from_wiki_root.clone()),
            reason: "page is not reachable from index.md".to_string(),
        });
    }
    for reference in &graph.broken_references {
        candidates.push(Candidate {
            kind: "broken-reference".to_string(),
            page_id: reference.source.clone(),
            path: scan
                .documents
                .iter()
                .find(|document| document.page_id() == reference.source)
                .map(|document| document.relative_file_path_from_wiki_root.clone()),
            reason: format!("page link points to missing page '{}'", reference.target),
        });
    }
    if let Some(stale_before) = args.stale_before {
        for document in &scan.documents {
            let path = context.wiki_root().join(&document.relative_file_path_from_wiki_root);
            let modified = std::fs::metadata(&path).and_then(|metadata| metadata.modified());
            let is_stale = modified
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .and_then(|duration| chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0))
                .is_some_and(|datetime| datetime.date_naive() < stale_before);
            if is_stale {
                candidates.push(Candidate {
                    kind: "stale".to_string(),
                    page_id: document.page_id(),
                    path: Some(document.relative_file_path_from_wiki_root.clone()),
                    reason: format!("file was modified before {stale_before}"),
                });
            }
        }
    }
    candidates.sort_by(|left, right| {
        (&left.kind, &left.page_id, &left.reason).cmp(&(&right.kind, &right.page_id, &right.reason))
    });

    match args.format {
        Format::Json => println!("{}", serde_json::to_string_pretty(&candidates).map_err(|error| error.to_string())?),
        Format::Human => {
            if candidates.is_empty() {
                println!("No maintenance candidates found.");
            } else {
                for candidate in &candidates {
                    println!(
                        "{} [{}] {}",
                        candidate.path.as_deref().unwrap_or(&candidate.page_id),
                        candidate.kind,
                        candidate.reason
                    );
                }
                println!("{} candidate(s); no files changed.", candidates.len());
            }
        }
    }
    let _ = args.dry_run;
    Ok(0)
}
