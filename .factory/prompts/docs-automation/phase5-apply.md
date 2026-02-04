# Phase 5: Apply Documentation Plan

You are executing a pre-approved documentation plan for an **mdBook** documentation site.

## Objective
Implement exactly the changes specified in the documentation plan from Phase 4.

## Documentation System
- **mdBook**: https://rust-lang.github.io/mdBook/
- **SUMMARY.md**: Follows mdBook format (https://rust-lang.github.io/mdBook/format/summary.html)
- **Prettier**: Will be run automatically after this phase (80 char line width)
- **Custom preprocessor**: Use `{#kb action::ActionName}` for keybindings instead of hardcoding

## Input
You will receive:
- Documentation plan from Phase 4
- Documentation guidelines from `docs/AGENTS.md`
- Style rules from `docs/.rules`

## Instructions

1. **Validate Plan**
   - Confirm all planned files are within scope per AGENTS.md
   - Verify no out-of-scope files are targeted

2. **Execute Each Planned Change**
   For each item in "Planned Changes":
   - Navigate to the specified file
   - Locate the specified section
   - Apply the described change
   - Follow style rules from `docs/.rules`

3. **Style Compliance**
   Every edit must follow `docs/.rules`:
   - Second person, present tense
   - No hedging words ("simply", "just", "easily")
   - Proper keybinding format (`Cmd+Shift+P`)
   - Settings Editor first, JSON second
   - Correct terminology (folder not directory, etc.)

4. **Preserve Context**
   - Maintain surrounding content structure
   - Keep consistent heading levels
   - Preserve existing cross-references

## Constraints
- Execute ONLY changes listed in the plan
- Do not discover new documentation targets
- Do not make stylistic improvements outside planned sections
- Do not expand scope beyond what Phase 4 specified
- If a planned change cannot be applied (file missing, section not found), skip and note it

## Output
After applying changes, output a summary:

```markdown
## Applied Changes

### Successfully Applied
- `path/to/file.md`: [Brief description of change]

### Skipped (Could Not Apply)
- `path/to/file.md`: [Reason - e.g., "Section not found"]

### Warnings
- [Any issues encountered during application]
```
