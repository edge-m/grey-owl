# Grey Owl Wiki Maintenance Skill

Use this workflow to find and assess maintenance candidates without changing
the wiki.

## Workflow

1. Run `growl maintain --dry-run --format json` to collect candidates. Add
   `--stale-before YYYY-MM-DD` when an age threshold is requested.
2. Run `growl graph` to verify the evidence for broken references, orphan
   pages, and unreachable pages.
3. Run `growl check --format json` to verify frontmatter, types, values, and
   duplicate identifiers.
4. Report each candidate, its reason, path, and likely impact.
5. Wait for explicit approval before changing, moving, archiving, or deleting
   any file. The current CLI provides detection only.
