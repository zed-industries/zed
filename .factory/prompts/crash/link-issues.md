# Crash Issue Linking

You are linking a crash to potentially related GitHub issues so human reviewers can quickly validate whether a fix may resolve multiple reports.

## Inputs

Before starting, you should have:

1. **Crash report** (from `script/sentry-fetch <issue-id>` or Sentry MCP)
2. **ANALYSIS.md** from investigation phase, including root cause and crash site

If either is missing, stop and report what is missing.

## Goal

Search GitHub issues and produce a reviewer-ready shortlist grouped by confidence:

- **High confidence**
- **Medium confidence**
- **Low confidence**

The output is advisory only. Humans must confirm before adding closing keywords or making release claims.

## Workflow

### Step 1: Build Search Signals

Extract concrete signals from the crash + analysis:

1. Crash site function, file, and crate
2. Error message / panic text
3. Key stack frames (especially in-app)
4. Reproduction trigger phrasing (user actions)
5. Affected platform/version tags if available

### Step 2: Search GitHub Issues

Search **only** issues in `zed-industries/zed` (prefer `gh issue list` / `gh issue view` / GraphQL if available) by:

1. Panic/error text
2. Function/file names
3. Crate/module names + symptom keywords
4. Similar reproduction patterns

Check both open and recently closed issues in `zed-industries/zed`.

### Step 3: Score Confidence

Assign confidence based on evidence quality:

- **High:** direct technical overlap (same crash site or same invariant violation with matching repro language)
- **Medium:** partial overlap (same subsystem and symptom, but indirect stack/repro match)
- **Low:** thematic similarity only (same area/keywords without solid technical match)

Avoid inflated confidence. If uncertain, downgrade.

### Step 4: Produce Structured Output

Write `LINKED_ISSUES.md` using this exact structure:

```markdown
# Potentially Related GitHub Issues

## High Confidence
- [#12345](https://github.com/zed-industries/zed/issues/12345) — <title>
  - Why: <1-2 sentence evidence-backed rationale>
  - Evidence: <stack frame / error text / repro alignment>

## Medium Confidence
- ...

## Low Confidence
- ...

## Reviewer Checklist
- [ ] Confirm High confidence issues should be referenced in PR body
- [ ] Confirm any issue should receive closing keywords (`Fixes #...`)
- [ ] Reject false positives before merge
```

If no credible matches are found, keep sections present and write `- None found` under each.

## Rules

- Do not fabricate issues or URLs.
- Do not include issues from any repository other than `zed-industries/zed`.
- Do not add closing keywords automatically.
- Keep rationale short and evidence-based.
- Favor precision over recall.
