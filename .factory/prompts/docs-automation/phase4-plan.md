# Phase 4: Plan Documentation Impact

You are determining whether and how documentation should be updated based on code changes.

## Objective
Produce a structured documentation plan that will guide Phase 5 execution.

## Documentation System
This is an **mdBook** site (https://rust-lang.github.io/mdBook/):
- `docs/src/SUMMARY.md` defines book structure per https://rust-lang.github.io/mdBook/format/summary.html
- If adding new pages, they MUST be added to SUMMARY.md
- Use `{#kb action::ActionName}` syntax for keybindings (custom preprocessor expands these)
- Prettier formatting (80 char width) will be applied automatically

## Input
You will receive:
- Change analysis from Phase 3
- Repository structure from Phase 2
- Documentation guidelines from `docs/AGENTS.md`

## Instructions

1. **Review AGENTS.md**
   - Load and apply all rules from `docs/AGENTS.md`
   - Respect scope boundaries (in-scope vs out-of-scope)

2. **Evaluate Documentation Impact**
   For each behavioral change from Phase 3:
   - Does existing documentation cover this area?
   - Is the documentation now inaccurate or incomplete?
   - Classify per AGENTS.md "Change Classification" section

3. **Identify Specific Updates**
   For each required update:
   - Exact file path
   - Specific section or heading
   - Type of change (update existing, add new, deprecate)
   - Description of the change

4. **Flag Uncertainty**
   Explicitly mark:
   - Assumptions you're making
   - Areas where human confirmation is needed
   - Ambiguous requirements

5. **Output Format**
Use the exact format specified in `docs/AGENTS.md` Phase 4 section:

```markdown
## Documentation Impact Assessment

### Summary
Brief description of code changes analyzed.

### Documentation Updates Required: [Yes/No]

### Planned Changes

#### 1. [File Path]
- **Section**: [Section name or "New section"]
- **Change Type**: [Update/Add/Deprecate]
- **Reason**: Why this change is needed
- **Description**: What will be added/modified

### Uncertainty Flags
- [ ] [Description of any assumptions or areas needing confirmation]

### No Changes Needed
- [List files reviewed but not requiring updates, with brief reason]
```

## Constraints
- Read-only: Do not modify any files
- Conservative: When uncertain, flag for human review rather than planning changes
- Scoped: Only plan changes that trace directly to code changes from Phase 3
- No scope expansion: Do not plan "improvements" unrelated to triggering changes
