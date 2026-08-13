# Grey Owl Wiki Overview Skill

Use this workflow to understand the wiki structure, document types, page
counts, relationships, and current validation state.

## Workflow

1. Run `growl schema --format json` to obtain the wiki root, configured types,
   fields, directory roles, and configuration diagnostics.
2. Run `growl overview directories --statistics` and
   `growl overview types --statistics` to collect actual page counts.
3. Run `growl graph` to inspect links, broken references, orphan pages, and
   unreachable pages.
4. Run `growl check --format json` and report the current validation state.

Do not infer missing configuration from directory names or page content.
