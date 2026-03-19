# Crash Investigation

You are investigating a crash that was observed in the wild. Your goal is to understand the root cause and produce a minimal reproduction test case that triggers the same crash. This test will be used to verify a fix and prevent regressions.

## Workflow

### Step 1: Get the Crash Report

If given a Sentry issue ID (like `ZED-4VS` or a numeric ID), there are several ways to fetch the crash data:

**Option A: Sentry MCP server (preferred if available)**
If the Sentry MCP server is configured as a context server, use its tools directly (e.g., `get_sentry_issue`) to fetch the issue details and stack trace. This is the simplest path — no tokens or scripts needed.

**Option B: Fetch script**
Run the fetch script from the terminal:

```
script/sentry-fetch <issue-id>
```

This reads authentication from `~/.sentryclirc` (set up via `sentry-cli login`) or the `SENTRY_AUTH_TOKEN` environment variable.

**Option C: Crash report provided directly**
If the crash report was provided inline or as a file, read it carefully before proceeding.

### Step 2: Analyze the Stack Trace

Read the stack trace bottom-to-top (from crash site upward) and identify:

1. **The crash site** — the exact function and line where the panic/abort occurs.
2. **The immediate cause** — what operation failed (e.g., slice indexing on a non-char-boundary, out-of-bounds access, unwrap on None).
3. **The relevant application frames** — filter out crash handler, signal handler, parking_lot, and stdlib frames. Focus on frames marked "(In app)".
4. **The data flow** — trace how the invalid data reached the crash site. What computed the bad index, the None value, etc.?

Find the relevant source files in the repository and read them. Pay close attention to:
- The crashing function and its callers
- How inputs to the crashing operation are computed
- Any assumptions the code makes about its inputs (string encoding, array lengths, option values)

### Step 3: Identify the Root Cause

Work backwards from the crash site to determine **what sequence of events or data conditions** produces the invalid state.

Ask yourself: *What user action or sequence of actions could lead to this state?* The crash came from a real user, so there is some natural usage pattern that triggers it.

### Step 4: Write a Reproduction Test

Write a minimal test case that:

1. **Mimics user actions** rather than constructing corrupt state directly. Work from the top down: what does the user do (open a file, type text, trigger a completion, etc.) that eventually causes the internal state to become invalid?
2. **Exercises the same code path** as the crash. The test should fail in the same function with the same kind of error (e.g., same panic message pattern).
3. **Is minimal** — include only what's necessary to trigger the crash. Remove anything that isn't load-bearing.
4. **Lives in the right place** — add the test to the existing test module of the crate where the bug lives. Follow the existing test patterns in that module.
5. **Avoid overly verbose comments** - the test should be self-explanatory and concise. More detailed descriptions of the test can go in ANALYSIS.md (see the next section).

When the test fails, its stack trace should share the key application frames from the original crash report. The outermost frames (crash handler, signal handling) will differ since we're in a test environment — that's expected.

If you can't reproduce the exact crash but can demonstrate the same class of bug (e.g., same function panicking with a similar invalid input), that is still valuable. Note the difference in your analysis.

### Step 5: Write the Analysis

Create an `ANALYSIS.md` file (in the working directory root, or wherever instructed) with these sections:

```markdown
# Crash Analysis: <short description>

## Crash Summary
- **Sentry Issue:** <ID and link if available>
- **Error:** <the panic/error message>
- **Crash Site:** <function name and file>

## Root Cause
<Explain what goes wrong and why. Be specific about the data flow.>

## Reproduction
<Describe what the test does and how it triggers the same crash.
Include the exact command to run the test, e.g.:
`cargo test -p <crate> <test_name>`>

## Suggested Fix
<Describe the fix approach. Be specific: which function, what check to add,
what computation to change. If there are multiple options, list them with tradeoffs.>
```

## Guidelines

- **Don't guess.** If you're unsure about a code path, read the source. Use `grep` to find relevant functions, types, and call sites.
- **Check the git history.** If the crash appeared in a specific version, `git log` on the relevant files may reveal a recent change that introduced the bug.
- **Look at existing tests.** The crate likely has tests that show how to set up the relevant subsystem. Follow those patterns rather than inventing new test infrastructure.
