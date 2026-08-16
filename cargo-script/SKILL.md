---
name: cargo-script
description: Create and update Rust cargo scripts (aka rust scripts) that run via cargo +nightly -Zscript, including adding the standard shebang, placing files where requested (default to the current working directory), and formatting with rustfmt. Use for requests mentioning cargo scripts, rust scripts, or generic "scripts" when the Rust context is clear.
---

# Cargo Script

## Overview

Create executable Rust script files that run via cargo -Zscript, add the required shebang, and format them with rustfmt.

## Quick Start (Preferred)

Use the helper script:

```bash
scripts/new_cargo_script.rs <file>
```

It creates the file with the required shebang, the frontmatter with a section for dependencies, and a simple hello world function. The script is then also marked as executable.

## Workflow

1. Determine the target path. If the user does not specify one, default to the current working directory with a sensible name (e.g., `script.rs`).
2. Create the file using the helper script.
3. Format the file with `rustfmt <file>` after creation.

## Notes & Conventions

- Treat "cargo script", "rust script", and contextually implied "script" requests as triggers for this workflow.
- Keep the shebang exact: `#!/usr/bin/env -S cargo +nightly -Zscript --quiet`.
- Prefer `.rs` for filenames unless the user requests otherwise.

## Tutorial Reference

Use `references/rust-script-tutorial.md` for small examples covering Paths, file IO, directory walks, invoking other programs, parallelization hints, CLI parsing (simple match vs clap), and `cliclack` prompts.

## Resources

- `scripts/new_cargo_script.rs` creates a new cargo script file with the standard shebang and optionally runs rustfmt.
- `references/rust-script-tutorial.md` contains the requested tutorial snippets and guidance.
