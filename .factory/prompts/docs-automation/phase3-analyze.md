# Phase 3: Analyze Changes

You are analyzing code changes to understand their nature and scope.

## Objective
Produce a clear, neutral summary of what changed in the codebase.

## Input
You will receive:
- List of changed files from the triggering commit/PR
- Repository structure from Phase 2

## Instructions

1. **Categorize Changed Files**
   - Source code (which crates/modules)
   - Configuration
   - Tests
   - Documentation (already existing)
   - Other

2. **Analyze Each Change**
   - Review diffs for files likely to impact documentation
   - Focus on: public APIs, settings, keybindings, commands, user-visible behavior

3. **Identify What Did NOT Change**
   - Note stable interfaces or behaviors
   - Important for avoiding unnecessary documentation updates

4. **Output Format**
Produce a markdown summary:

```markdown
## Change Analysis

### Changed Files Summary
| Category | Files | Impact Level |
| --- | --- | --- |
| Source - [crate] | file1.rs, file2.rs | High/Medium/Low |
| Settings | settings.json | Medium |
| Tests | test_*.rs | None |

### Behavioral Changes
- **[Feature/Area]**: Description of what changed from user perspective
- **[Feature/Area]**: Description...

### Unchanged Areas
- [Area]: Confirmed no changes to [specific behavior]

### Files Requiring Deeper Review
- `path/to/file.rs`: Reason for deeper review
```

## Constraints
- Read-only: Do not modify any files
- Neutral tone: Describe what changed, not whether it's good/bad
- Do not propose documentation changes yet
