# Grey Owl Wiki Search Skill

Use this workflow to search structured wiki metadata and relationships before
falling back to Markdown body text.

## Workflow

- Search frontmatter with `growl search --query 'field:value'`.
- Inspect page relationships with the nodes and edges from `growl graph`.
- Search Markdown body text with `rg` when structured search is insufficient.
- Use `growl schema --format json` to confirm valid fields and types before
  constructing a query.
- If search results will support a file change, run `growl check` afterward.

The current query syntax supports one `field:value` condition. Do not assume
that Boolean operators or body-text search are supported by `growl search`.
