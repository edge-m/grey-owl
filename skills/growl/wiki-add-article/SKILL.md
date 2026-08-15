# Grey Owl Wiki Add Article Skill

Use this workflow when an article is created from a source file. The agent or
user creates the Markdown file; this workflow validates the resulting file and
does not silently create, move, or delete files.

## Workflow

1. Inspect the current rules with `growl schema --format json`.
2. Select a configured type with `growl overview types --type <type>`.
3. Select a configured destination with `growl overview directories`.
4. Create the Markdown article using the required frontmatter. Preserve the
   source-to-article relationship when the user requires traceability.
5. Validate only the new file with `growl validate --file <path>`.
6. Run `growl validate` for the whole wiki.
7. Report the path, selected type, validation diagnostics, and any remaining
   broken links or orphan-page diagnostics.

Do not guess required fields, identifiers, type names, or placement rules.
Ask for clarification when the source does not provide enough information.
