# Grey Owl Wiki Skill

Use this skill when working with a wiki validated by Grey Owl (`growl`). It
defines a safe, repeatable workflow for inspecting, creating, updating, and
maintaining wiki documents.

## Start every task by reading the wiki guidance

Before changing a wiki, inspect these files when they exist:

1. `.grey-owl/agent-guide.md` for wiki-specific rules and conventions.
2. `.grey-owl/schema.json` for document types, fields, relationships, and
   placement rules.
3. `.grey-owl/diagnostics.json` for the latest validation results.

Do not guess a document type, field, identifier rule, or relationship when the
wiki guidance does not define it. Look for an existing document with the same
kind and ask for clarification when the intended structure remains unclear.

## Validation

Run the validator after creating or updating documents:

```sh
growl check <wiki-path>
growl check <wiki-path> --format json
```

Use JSON output when diagnostics need to be inspected programmatically. Do not
report a task as complete while validation errors are unresolved. If the
`growl` binary is not installed, run the project command from its repository as
appropriate, for example `cargo run -- check <wiki-path>`.

## Add a document

1. Identify the document's purpose and type.
2. Confirm its type, required frontmatter, location, and filename rules from
   the wiki guidance or schema.
3. Choose an identifier that does not already exist.
4. Create the smallest valid frontmatter and Markdown body.
5. Record configured relationships in their defined format.
6. Run `growl check` and review all diagnostics.
7. Report the created path, metadata, and validation result.

## Update a document

1. Locate the target by identifier, path, type, or relationship and verify it
   is the intended document.
2. Re-read the applicable schema before changing metadata.
3. Keep the change limited to the requested scope; do not rewrite unrelated
   body text.
4. Check for affected identifiers and relationships when metadata changes.
5. Run `growl check` and report the change and its validation result.

## Search and maintenance

Prefer structured information such as identifiers, types, fields, and
relationships. Use text search only when structure is insufficient, then
verify the matching path and metadata.

Maintenance is performed in this order:

```text
detect -> review candidates -> assess impact -> prepare a plan or dry-run -> apply explicitly
```

An orphaned, old, or apparently unused document must not be deleted solely for
that reason. Check for implicit references and reversibility first. Do not
perform broad or destructive changes without an explicit request and a clear
list of affected files.

## Safety rules

- Inspect target files before changing them.
- Do not invent undefined types, fields, or relationships.
- Do not modify documents outside the requested scope.
- Treat metadata changes as potentially higher impact than body changes.
- Never silently ignore validation errors.
- Do not delete documents automatically.
- Keep detection, planning, and applying changes as separate steps.
- Report what changed, what was not changed, and the validation result.
