# Crash Fix

You are fixing a crash that has been analyzed and has a reproduction test case. Your goal is to implement a minimal, correct fix that resolves the root cause and makes the reproduction test pass.

## Inputs

Before starting, you should have:

1. **ANALYSIS.md** — the crash analysis from the investigation phase. Read it thoroughly.
2. **A failing test** — a reproduction test that triggers the crash. Run it first to confirm it fails as expected.

If either is missing, ask the user to provide them or run the investigation phase first (`/prompt crash/investigate`).

## Workflow

### Step 1: Confirm the Failing Test

Run the reproduction test and verify it fails with the expected crash:

```
cargo test -p <crate> <test_name>
```

Read the failure output. Confirm the panic message and stack trace match what ANALYSIS.md describes. If the test doesn't fail, or fails differently than expected, stop and reassess before proceeding.

### Step 2: Understand the Fix

Read the "Suggested Fix" section of ANALYSIS.md and the relevant source code. Before writing any code, be clear on:

1. **What invariant is being violated** — what property of the data does the crashing code assume?
2. **Where the invariant breaks** — which function produces the bad state?

### Step 3: Implement the Fix

Apply the minimal change needed to resolve the root cause. Guidelines:

- **Fix the root cause, not the symptom.** Don't just catch the panic with a bounds check if the real problem is an incorrect offset calculation. Fix the calculation.
- **Preserve existing behavior** for all non-crashing cases. The fix should only change what happens in the scenario that was previously crashing.
- **Don't add unnecessary changes.** No drive-by improvements, keep the diff focused.
- **Add a comment only if the fix is non-obvious.** If a reader might wonder "why is this check here?", a brief comment explaining the crash scenario is appropriate.
- **Consider long term maintainability** Please make a targeted fix while being sure to consider the long term maintainability and reliability of the codebase

### Step 4: Verify the Fix

Run the reproduction test and confirm it passes:

```
cargo test -p <crate> <test_name>
```

Then run the full test suite for the affected crate to check for regressions:

```
cargo test -p <crate>
```

If any tests fail, determine whether the fix introduced a regression. Fix regressions before proceeding.

### Step 5: Run Clippy

```
./script/clippy
```

Address any new warnings introduced by your change.

### Step 6: Summarize

Write a brief summary of the fix for use in a PR description. Include:

- **What was the bug** — one sentence on the root cause.
- **What the fix does** — one sentence on the change.
- **How it was verified** — note that the reproduction test now passes.
- **Sentry issue link** — if available from ANALYSIS.md.

We use the following template for pull request descriptions. Please add information to answer the relevant sections, especially for release notes.

```
<Description of change, what the issue was and the fix.>

Release Notes:

- N/A *or* Added/Fixed/Improved ...
```
