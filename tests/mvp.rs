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
        root.join("grey-owl.yml"),
        r#"common_fields:
  id:
    type: string
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
            root.to_str().expect("fixture path should be utf-8"),
            "--config",
            root.join("grey-owl.yml").to_str().expect("config path should be utf-8"),
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
    fs::write(root.join("note.md"), "---\nid: welcome\ntype: note\ntitle: Welcome\nstatus: active\n---\n# Welcome\n")
        .expect("document should be written");

    let output = run_check(&root, "json");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"[]\n");
    fs::remove_dir_all(root).expect("fixture should be removed");
}

#[test]
fn missing_field_is_reported_as_json() {
    let root = fixture("missing-field");
    write_config(&root);
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
    fs::write(root.join("one.md"), "---\nid: same\ntype: note\ntitle: One\n---\n")
        .expect("document should be written");
    fs::write(root.join("two.md"), "---\nid: same\ntype: note\ntitle: Two\n---\n")
        .expect("document should be written");
    fs::write(root.join("broken.md"), "---\nid: broken\ntype: [note\n---\n").expect("document should be written");

    let output = run_check(&root, "json");
    assert_eq!(output.status.code(), Some(1));
    let diagnostics: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let codes: Vec<&str> = diagnostics.iter().filter_map(|diagnostic| diagnostic["code"].as_str()).collect();
    assert!(codes.contains(&"duplicate-identifier"));
    assert!(codes.contains(&"invalid-frontmatter"));
    fs::remove_dir_all(root).expect("fixture should be removed");
}
