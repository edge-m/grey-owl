use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture(name: &str) -> PathBuf {
    let timestamp =
        SystemTime::now().duration_since(UNIX_EPOCH).expect("system clock must be after unix epoch").as_nanos();
    let path = std::env::temp_dir().join(format!("grey-owl-{name}-{}-{timestamp}", std::process::id()));
    fs::create_dir_all(&path).expect("fixture directory should be created");
    path
}

fn write_config(root: &Path) {
    fs::write(
        root.join("growl.yml"),
        r#"wiki_root: .
mandatory_fields:
  type:
    type: string
types:
  note:
    fields:
      title:
        type: string
      status:
        type: string
        optional: true
        values: [draft, active]
"#,
    )
    .expect("config should be written");
}

fn run_check(root: &Path, format: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_growl"))
        .args([
            "check",
            "--config",
            root.join("growl.yml").to_str().expect("config path should be utf-8"),
            "--format",
            format,
        ])
        .output()
        .expect("growl should run")
}

#[test]
fn valid_wiki_returns_success() {
    let root = fixture("valid");
    write_config(&root);
    fs::write(root.join("Index.md"), "---\ntype: note\ntitle: Index\n---\n[Welcome](note.md)\n")
        .expect("index should be written");
    fs::write(root.join("note.md"), "---\nid: welcome\ntype: note\ntitle: Welcome\nstatus: active\n---\n# Welcome\n")
        .expect("document should be written");

    let output = run_check(&root, "json");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"[]\n");
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn overview_and_search_return_structured_results() {
    let root = fixture("overview-search");
    write_config(&root);
    fs::write(root.join("Index.md"), "---\ntype: note\ntitle: Index\n---\n[Welcome](note.md)\n")
        .expect("index should be written");
    fs::write(root.join("note.md"), "---\ntype: note\ntitle: Welcome\nstatus: active\n---\n")
        .expect("document should be written");

    let config = root.join("growl.yml");
    let overview = Command::new(env!("CARGO_BIN_EXE_growl"))
        .args(["overview", "types", "--config", config.to_str().unwrap(), "--type", "note"])
        .output()
        .expect("overview should run");
    assert!(overview.status.success());
    let overview_json: serde_json::Value = serde_json::from_slice(&overview.stdout).expect("valid JSON");
    assert_eq!(overview_json[0]["name"], "note");
    assert_eq!(overview_json[0]["schema"]["required_fields"]["type"]["type"], "string");

    let search = Command::new(env!("CARGO_BIN_EXE_growl"))
        .args(["search", "--config", config.to_str().unwrap(), "--query", "status:active"])
        .output()
        .expect("search should run");
    assert!(search.status.success());
    let search_json: serde_json::Value = serde_json::from_slice(&search.stdout).expect("valid JSON");
    assert_eq!(search_json[0]["page_id"], "note");
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn check_reports_orphans_and_can_filter_to_one_file() {
    let root = fixture("orphans");
    write_config(&root);
    fs::write(root.join("Index.md"), "---\ntype: note\ntitle: Index\n---\n[Welcome](note.md)\n")
        .expect("index should be written");
    fs::write(root.join("note.md"), "---\ntype: note\ntitle: Welcome\n---\n").expect("document should be written");
    fs::write(root.join("orphan.md"), "---\ntype: note\ntitle: Orphan\n---\n").expect("document should be written");

    let config = root.join("growl.yml");
    let all = Command::new(env!("CARGO_BIN_EXE_growl"))
        .args(["check", "--config", config.to_str().unwrap(), "--format", "json"])
        .output()
        .expect("check should run");
    let all_json: Vec<serde_json::Value> = serde_json::from_slice(&all.stdout).expect("valid JSON");
    assert!(all_json.iter().any(|diagnostic| diagnostic["code"] == "orphan-page"));

    let one = Command::new(env!("CARGO_BIN_EXE_growl"))
        .args(["check", "--config", config.to_str().unwrap(), "--file", "orphan.md", "--format", "json"])
        .output()
        .expect("file check should run");
    let one_json: Vec<serde_json::Value> = serde_json::from_slice(&one.stdout).expect("valid JSON");
    assert_eq!(one_json.len(), 1);
    assert_eq!(one_json[0]["code"], "orphan-page");
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn missing_field_is_reported_as_json() {
    let root = fixture("missing-field");
    write_config(&root);
    fs::write(root.join("Index.md"), "---\ntype: note\ntitle: Index\n---\n[Missing](note.md)\n")
        .expect("index should be written");
    fs::write(root.join("note.md"), "---\nid: missing-title\ntype: note\n---\n").expect("document should be written");

    let output = run_check(&root, "json");
    assert_eq!(output.status.code(), Some(1));
    let diagnostics: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert_eq!(diagnostics[0]["code"], "missing-required-field");
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn duplicate_identifier_and_invalid_frontmatter_are_reported() {
    let root = fixture("invalid");
    write_config(&root);
    fs::write(root.join("Index.md"), "---\ntype: note\ntitle: Index\n---\n[One](one.md) [Two](two.md)\n")
        .expect("index should be written");
    fs::write(root.join("one.md"), "---\nid: same\ntype: note\ntitle: One\n---\n")
        .expect("document should be written");
    fs::write(root.join("two.md"), "---\nid: same\ntype: note\ntitle: Two\n---\n")
        .expect("document should be written");
    fs::write(root.join("broken.md"), "---\nid: broken\ntype: [note\n---\n").expect("document should be written");

    let output = run_check(&root, "json");
    assert_eq!(output.status.code(), Some(1));
    let diagnostics: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let codes: Vec<&str> = diagnostics.iter().filter_map(|diagnostic| diagnostic["code"].as_str()).collect();
    assert!(codes.contains(&"invalid-frontmatter"));
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn skill_command_writes_skill_file() {
    let root = fixture("skill");
    let output = Command::new(env!("CARGO_BIN_EXE_growl"))
        .args(["skill", root.to_str().expect("fixture path should be utf-8")])
        .output()
        .expect("growl should run");

    assert!(output.status.success());
    let skill_path = root.join("growl/SKILL.md");
    let skill = fs::read_to_string(&skill_path).expect("skill should be written");
    assert!(skill.contains("# Grey Owl Wiki Skills"));
    assert!(root.join("growl/wiki-overview/SKILL.md").is_file());
    assert!(root.join("growl/wiki-add-article/SKILL.md").is_file());
    assert!(root.join("growl/wiki-maintenance/SKILL.md").is_file());
    assert!(root.join("growl/wiki-search/SKILL.md").is_file());
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn init_command_writes_default_config() {
    let root = fixture("init");
    let output =
        Command::new(env!("CARGO_BIN_EXE_growl")).arg("init").current_dir(&root).output().expect("growl should run");

    assert!(output.status.success());
    let config = fs::read_to_string(root.join("growl.yml")).expect("config should be written");
    assert!(config.contains("mandatory_fields:"));
    assert!(config.contains("wiki_root: ."));
    assert!(config.contains("directories:"));
    assert!(config.contains("description:"));
    assert!(config.contains("types:"));
    assert!(config.contains("wiki_lint:"));
    assert!(config.contains("config_lint:"));
    assert!(config.contains("max_nesting_depth: 1"));
    assert!(config.contains("# Wiki root path used by Grey Owl commands.\nwiki_root:"));
    assert!(config.contains("\n\n# Directory structure and descriptions.\ndirectories:"));
    assert!(config.contains("\n\n# Fields required on every document.\nmandatory_fields:"));
    let mandatory_fields = config.split("mandatory_fields:").nth(1).expect("mandatory fields should be written");
    let field_order = ["type:", "title:", "description:", "tags:", "sources:", "generated:", "stale_after:"];
    let mut previous = 0;
    for field in field_order {
        let position = mandatory_fields
            .lines()
            .scan(0, |offset, line| {
                let current = *offset;
                *offset += line.len() + 1;
                Some((current, line))
            })
            .find_map(|(position, line)| (line == format!("  {field}")).then_some(position))
            .expect("default field should be written");
        assert!(position >= previous, "field {field} is out of order");
        previous = position;
    }

    let second_output =
        Command::new(env!("CARGO_BIN_EXE_growl")).arg("init").current_dir(&root).output().expect("growl should run");
    assert_eq!(second_output.status.code(), Some(2));
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn configured_root_and_nested_directories_are_supported() {
    let root = fixture("configured-root");
    fs::create_dir_all(root.join("wiki/raw/inbox")).expect("wiki directory should be created");
    fs::write(
        root.join("growl.yml"),
        "wiki_root: wiki\ndirectories:\n  raw:\n    description: Raw source\n    directories:\n      inbox:\n        description: Incoming files\nmandatory_fields:\n  type:\n    type: string\ntypes:\n  note:\n    description: A note\n    fields: {}\n",
    )
    .expect("config should be written");
    fs::write(root.join("wiki/Index.md"), "---\ntype: note\n---\n[Wrong](raw/inbox/wrong.md)\n")
        .expect("index should be written");
    fs::write(root.join("wiki/raw/inbox/wrong.md"), "---\nid: wrong\ntype: note\n---\n")
        .expect("document should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_growl"))
        .args([
            "check",
            "--config",
            root.join("growl.yml").to_str().expect("path should be utf-8"),
            "--format",
            "json",
        ])
        .output()
        .expect("growl should run");
    assert!(output.status.success());
    let diagnostics: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert!(diagnostics.is_empty());
    fs::remove_dir_all(root).expect("fixture should be removed");
}
