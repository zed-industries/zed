# Phase 2: Explore Repository

You are analyzing a codebase to understand its structure before reviewing documentation impact.

## Objective
Produce a structured overview of the repository to inform subsequent documentation analysis.

## Instructions

1. **Identify Primary Languages and Frameworks**
   - Scan for Cargo.toml, package.json, or other manifest files
   - Note the primary language(s) and key dependencies

2. **Map Documentation Structure**
   - This project uses **mdBook** (https://rust-lang.github.io/mdBook/)
   - Documentation is in `docs/src/`
   - Table of contents: `docs/src/SUMMARY.md` (mdBook format: https://rust-lang.github.io/mdBook/format/summary.html)
   - Style guide: `docs/.rules`
   - Agent guidelines: `docs/AGENTS.md`
   - Formatting: Prettier (config in `docs/.prettierrc`)

3. **Identify Build and Tooling**
   - Note build systems (cargo, npm, etc.)
   - Identify documentation tooling (mdbook, etc.)

4. **Output Format**
Produce a JSON summary:

```json
{
  "primary_language": "Rust",
  "frameworks": ["GPUI"],
  "documentation": {
    "system": "mdBook",
    "location": "docs/src/",
    "toc_file": "docs/src/SUMMARY.md",
    "toc_format": "https://rust-lang.github.io/mdBook/format/summary.html",
    "style_guide": "docs/.rules",
    "agent_guidelines": "docs/AGENTS.md",
    "formatter": "prettier",
    "formatter_config": "docs/.prettierrc",
    "custom_preprocessor": "docs_preprocessor (handles {#kb action::Name} syntax)"
  },
  "key_directories": {
    "source": "crates/",
    "docs": "docs/src/",
    "extensions": "extensions/"
  }
}
```

## Constraints
- Read-only: Do not modify any files
- Focus on structure, not content details
- Complete within 2 minutes
