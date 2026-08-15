# Grey Owl Wiki Skills

Use these workflows to inspect, search, validate, and maintain a structured
Markdown wiki with Grey Owl.

## Skills

- `wiki-overview`: Inspect the wiki structure and current health.
- `wiki-config`: Build and validate a Wiki configuration through a guided conversation.
- `wiki-add-article`: Add an article from a source and validate the result.
- `wiki-ingest`: Preserve a source file in `raw/`, create a derived page with a
  formal source record, and validate the relationship.
- `wiki-maintenance`: Find and assess maintenance candidates.
- `wiki-search`: Search structured metadata, links, and body text.

## General rules

- Inspect the configuration and current wiki state before making decisions.
- Do not invent types, fields, relationships, or placement rules.
- Run `growl validate` after every file change. Use `--details` when individual
  diagnostics are needed.
- Never report a change as successful while validation errors remain.

## CLI workflow

Use the following commands when the generated CLI is available:

```text
growl overview directories
growl overview types
growl overview types --type <type>
growl search --query 'field:value'
growl validate
growl validate --file <path>
growl validate --details
growl graph
growl schema --format json
```

Commands resolve the wiki root from `growl.yml` in the current directory by
default. Use `--config <path>` to select another configuration file. Use `rg`
for searching Markdown body text; `growl search` is for structured frontmatter.

## Safety

- Use `growl graph` to inspect relationships, broken references, orphan pages,
  and unreachable pages.
- Use `growl schema --format json` to obtain the current configuration and
  configuration diagnostics.
- `growl validate` only reports diagnostics; it never changes files.
- Obtain explicit approval before deleting, moving, or rewriting files.
