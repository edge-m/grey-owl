---
name: using-wiki
description: Establish the Grey Owl Wiki workflow before any Wiki task, inspect the current schema and state, and select the appropriate operation skill.
---

# Using Grey Owl Wiki

Use this skill once at the beginning of a task involving a Grey Owl Wiki,
before loading an operation-specific skill.

## Workflow

1. Identify the Wiki root and locate `growl.yml`.
2. Run `growl schema --format json` to inspect the current configuration.
3. Run `growl overview directories` and `growl overview types` when the task
   depends on Wiki structure or document types.
4. Select the operation skill that matches the user's request.
5. After any file change, run `growl validate` and report remaining diagnostics.

## Operation selection

- Use `wiki-config` to design or revise `growl.yml`.
- Use `wiki-overview` to understand structure, counts, and health.
- Use `wiki-search` to find structured metadata, links, or body text.
- Use `wiki-ingest` when preserving a source in `raw/` and deriving a page.
- Use `wiki-maintenance` to assess broken links, orphans, and stale content.

## Safety rules

- Do not invent types, fields, relationships, or placement rules.
- Do not overwrite, move, or delete existing files without explicit approval.
- Treat `growl` as the deterministic validator; the Agent makes semantic
  decisions and edits content.
