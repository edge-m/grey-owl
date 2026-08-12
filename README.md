# Grey Owl

Grey Owl is a fast, configuration-driven validator for structured Markdown
wikis. It checks YAML frontmatter and reports actionable diagnostics in human-
readable or JSON form.

The project and library are named `grey-owl`; `growl` is the CLI binary.

## Why Grey Owl?

The name comes from **OWL** — **O**KF **W**iki **L**inter. Grey Owl started
with Open Knowledge Format (OKF) wiki conventions, but its data model and
validation rules are intentionally not tied to OKF or any particular wiki
product.

## What it does

- Recursively scans Markdown files below a wiki root
- Parses YAML frontmatter
- Validates required fields, document types, value types, and allowed values
- Detects missing and duplicate identifiers
- Reports diagnostics as human-readable text or JSON
- Provides stable exit codes for scripts and CI
- Generates a starter configuration and an Agent Skill

Markdown body text is treated as opaque content; Grey Owl does not judge its
meaning or writing quality.

## Quick start

Install the current checkout locally:

```sh
make dev-install
```

Create a configuration in your wiki directory:

```sh
cd path/to/wiki
growl init
```

Validate the wiki:

```sh
growl check . --config ./growl.yml
growl check . --config ./growl.yml --format json
```

`growl init` creates `growl.yml` and never overwrites an existing file.

## Configuration

The generated configuration is a small YAML schema. `root` is resolved
relative to the configuration file and is used when the wiki path is omitted
from `growl check`. `directories` describes the directory structure with
nestable descriptions. The `raw` directory can be used as a data source where
files are added freely. Type descriptions can be written with `description`.
`id` and `type` are reserved fields. Fields without `optional: true` are
required.

```yaml
root: .

directories:
  raw:
    description: Raw data source; files can be added here freely
    directories:
      inbox:
        description: Incoming raw files

common_fields:
  id:
    type: string
  type:
    type: string

types:
  note:
    description: A general-purpose note
    fields:
      title:
        type: string
      status:
        type: string
        optional: true
        values: [draft, active]
```

Unknown frontmatter fields are preserved and do not produce an error. Directory
entries are descriptive only and may be nested. To use the configured root,
run `growl check --config ./growl.yml`; `--config` is the path to the
configuration file, while `root` is the path inside that configuration.

## Commands

```text
growl init
growl check [<wiki-path>] [--config <file>] [--format human|json]
growl skill <output-directory>
```

`growl skill` writes the static Agent Skill to:

```text
<output-directory>/growl/SKILL.md
```

Exit codes:

- `0` — validation completed without errors
- `1` — validation completed and found errors
- `2` — the command could not run because of invalid arguments, configuration,
  or I/O

## Development

The project uses stable Rust. From the repository root:

```sh
make format-check
make lint
make test
```

Other useful targets are `make build`, `make check`, `make format`, and
`make dev-install`. See [AGENTS.md](AGENTS.md) for repository contribution and
verification rules.

The Japanese README is available at
[`docs/i18n/README.ja.md`](docs/i18n/README.ja.md).
