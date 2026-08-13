# Grey Owl

![Grey Owl logo](docs/img/wide.webp)

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
growl check --config ./growl.yml
growl check --config ./growl.yml --format json
```

`growl init` creates `growl.yml` and never overwrites an existing file.

## Configuration

The generated configuration is a small YAML schema. `wiki_root` is resolved
relative to the configuration file and is used by `growl` commands to locate
the wiki. `directories` describes the directory structure with
nestable descriptions. The `raw` directory can be used as a data source where
files are added freely. Type descriptions can be written with `description`.
`mandatory_fields` are required on every document and cannot use `optional: true`.
Type-specific fields are also required unless they explicitly use
`optional: true`.

```yaml
wiki_root: .

directories:
  raw:
    description: Raw data source; files can be added here freely
    directories:
      inbox:
        description: Incoming raw files

mandatory_fields:
  type:
    type: string
  title:
    type: string
  description:
    type: string
  tags:
    type: array
    items:
      type: string
  sources:
    type: array
    items:
      type: string
  generated:
    type: object
    fields:
      at:
        type: datetime
      by:
        type: string
  stale_after:
    type: date

types:
  note:
    description: A general-purpose note

wiki_lint: {}
config_lint:
  max_nesting_depth: 1
```

Unknown frontmatter fields are preserved and do not produce an error. Directory
entries are descriptive only and may be nested. To use the configured wiki root,
run `growl check --config ./growl.yml`; `--config` is the path to the
configuration file, while `wiki_root` is the path inside that configuration.
`wiki_lint` contains wiki validation settings, while `config_lint` contains
settings for validating the configuration schema itself. Nested array and
object fields use `items` and `fields` respectively.

## Commands

```text
growl init
growl config lint [--config <file>] [--format human|json]
growl graph [--config <file>]
growl schema [--config <file>] [--format text|json]
growl maintain [--config <file>] [--stale-before YYYY-MM-DD] [--dry-run] [--format human|json]
growl overview directories [--config <file>] [--statistics]
growl overview types [--config <file>] [--statistics] [--type <type>]
growl search [--config <file>] --query <query>
growl check [--config <file>] [--file <path>] [--format human|json]
growl skill <output-directory>
```

When `--config` is omitted, commands look for `growl.yml` in the current
directory and resolve `wiki_root` from it. `check` without `--file` validates
the whole wiki, including pages that cannot be reached from `Index.md`.

`growl skill` writes the static Agent Skill to:

```text
<output-directory>/growl/SKILL.md
<output-directory>/growl/wiki-overview/SKILL.md
<output-directory>/growl/wiki-add-article/SKILL.md
<output-directory>/growl/wiki-maintenance/SKILL.md
<output-directory>/growl/wiki-search/SKILL.md
```

The generated Skills currently provide outlines and explicit TODOs for the
Wiki overview, article-addition validation, maintenance checks, and search
workflows.

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

## Build profiles

Development builds use 128 codegen units to keep rebuilds responsive. Release
builds use maximum link-time optimization (`lto = true`), `opt-level = 3`, and
a single codegen unit for the best runtime optimization. The release profile
does not alter stripping or panic behavior.
