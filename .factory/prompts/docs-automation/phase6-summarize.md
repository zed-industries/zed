# Phase 6: Summarize Changes

You are generating a summary of documentation updates for PR review.

## Objective
Create a clear, reviewable summary of all documentation changes made.

## Input
You will receive:
- Applied changes report from Phase 5
- Original change analysis from Phase 3
- Git diff of documentation changes

## Instructions

1. **Gather Change Information**
   - List all modified documentation files
   - Identify the corresponding code changes that triggered each update

2. **Generate Summary**
   Use the format specified in `docs/AGENTS.md` Phase 6 section:

```markdown
## Documentation Update Summary

### Changes Made
| File | Change | Related Code |
| --- | --- | --- |
| docs/src/path.md | Brief description | PR #123 or commit SHA |

### Rationale
Brief explanation of why these updates were made, linking back to the triggering code changes.

### Review Notes
- Items reviewers should pay special attention to
- Any uncertainty flags from Phase 4 that were addressed
- Assumptions made during documentation
```

3. **Add Context for Reviewers**
   - Highlight any changes that might be controversial
   - Note if any planned changes were skipped and why
   - Flag areas where reviewer expertise is especially needed

## Output Format
The summary should be suitable for:
- PR description body
- Commit message (condensed version)
- Team communication

## Constraints
- Read-only (documentation changes already applied in Phase 5)
- Factual: Describe what was done, not justify why it's good
- Complete: Account for all changes, including skipped items
