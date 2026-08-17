---
name: wiki-maintenance
description: Find and assess Grey Owl Wiki maintenance candidates such as broken links, orphan pages, unreachable pages, and stale content without changing files.
---

# Grey Owl Wiki Maintenance Skill

Use this workflow to find and assess maintenance candidates without changing
the wiki.

## Workflow

1. Run `growl validate --details --format json` to collect diagnostics. Add
   `--stale-before YYYY-MM-DD` when an age threshold is requested.
2. Run `growl graph` to verify the evidence for broken references, orphan
   pages, and unreachable pages.
3. Run `growl validate` to verify frontmatter, types, values, and
   duplicate identifiers.
4. Report each candidate, its reason, path, and likely impact.
5. Wait for explicit approval before changing, moving, archiving, or deleting
   any file. The current CLI provides detection only.
