# growl

`growl` is a wiki linter for Open Knowledge Format (OKF).

The repository is named `owl` to avoid a name collision. The project name is based on
**OWL** (**O**KF **W**iki **L**inter), while the command is named `growl`.

## MVP usage

Create a YAML configuration file such as:

```yaml
common_fields:
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
```

Then validate a wiki:

```sh
growl check ./wiki --config ./growl.yml
growl check ./wiki --config ./growl.yml --format json
```

新しいWikiの設定ファイルを作成するには、Wikiのルートで実行します。

```sh
growl init
```

カレントディレクトリに `growl.yml` が生成されます。既存の設定ファイルは上書きしません。

The MVP checks YAML frontmatter, required fields, document types, configured field
types and values, and duplicate identifiers. Markdown body text is not semantically
evaluated.

## Development

```sh
cargo run
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Agent向けSkillを指定ディレクトリへ出力するには、次を実行します。

```sh
growl skill /path/to/skills
```

`/path/to/skills/growl/SKILL.md` が生成されます。

プロジェクトではstable Rust（現在の検証環境では1.95.0）を使用し、
`cargo check-all` と `cargo test-all` を検証用のショートカットとして利用できます。
