# Grey Owl Wiki Skills

Use these workflows to inspect, search, validate, and maintain a structured
Markdown wiki with Grey Owl.

## Skills

- `wiki-overview`: Inspect the wiki structure and current health.
- `wiki-config`: Build and validate a Wiki configuration through a guided conversation.
- `wiki-add-article`: Add an article from a source and validate the result.
- `wiki-maintenance`: Find and assess maintenance candidates.
- `wiki-search`: Search structured metadata, links, and body text.

## General rules

- Inspect the configuration and current wiki state before making decisions.
- Do not invent types, fields, relationships, or placement rules.
- Run `growl check` after every file change.
- Never report a change as successful while validation errors remain.

## CLI workflow

Use the following commands when the generated CLI is available:

```text
growl overview directories
growl overview types
growl overview types --type <type>
growl search --query 'field:value'
growl check
growl check --file <path>
growl graph
growl schema --format json
growl maintain --dry-run --format json
```

Commands resolve the wiki root from `growl.yml` in the current directory by
default. Use `--config <path>` to select another configuration file. Use `rg`
for searching Markdown body text; `growl search` is for structured frontmatter.

## Safety

- Use `growl graph` to inspect relationships, broken references, orphan pages,
  and unreachable pages.
- Use `growl schema --format json` to obtain the current configuration and
  configuration diagnostics.
- `growl maintain --dry-run` only reports candidates; it never changes files.
- Obtain explicit approval before deleting, moving, or rewriting files.
