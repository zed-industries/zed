# SEC-264 implementation plan

## Goal

Fix terminal permission matching so that regexes see environment-variable prefixes as part of the command they are matching, while also making shell substitutions/interpolations invalid terminal tool calls unless terminal permissions are in unconditional allow-all mode.

The implementation must be test-first:

1. Add a detailed failing test suite first.
2. Run the tests and confirm they fail before the implementation exists.
3. Implement the feature without modifying those tests.
4. Re-run the same tests and confirm they now pass.

Because this is a security feature, favor conservative behavior when there is uncertainty.

## Current status

### Implemented

- `crates/shell_command_parser/src/shell_command_parser.rs`
  - Added structured terminal command-prefix extraction.
  - Added parser-backed terminal command validation with `Safe` / `Unsafe` / `Unknown` outcomes.
  - Extended normalized extracted commands to include scalar env-var prefixes in order.
  - Preserved quoted assignment values when required.
  - Ignored array assignments for prefix matching output.
  - Added lower-level tests covering env-var prefixes, assignment normalization, and forbidden syntax detection.

- `crates/agent/src/pattern_extraction.rs`
  - Updated terminal pattern extraction to use structured parser output.
  - Included env-var prefixes in generated terminal patterns.
  - Preserved display whitespace in the UI label path.
  - Normalized inter-token regex boundaries to `\\s+` while preserving quoted assignment values as single tokens.
  - Kept rejecting path-like commands.

- `crates/agent/src/tool_permissions.rs`
  - Added invalid-terminal-command rejection for forbidden substitutions/interpolations.
  - Added unconditional allow-all handling for:
    - no terminal-specific rules + global default `Allow`
    - terminal-specific effective default `Allow` with empty pattern lists
  - Preserved hardcoded denial precedence over unconditional allow-all.
  - Updated permission tests to reflect the new deny behavior for invalid terminal syntax.
  - Added tests for env-prefixed allow patterns and unconditional allow-all bypass behavior.

- `crates/agent/src/tests/mod.rs`
  - Exposed the minimal fake terminal helpers needed for reuse from other test modules.

### Verified so far

The following targeted test suites have been run and are passing:

- `cargo test -p shell_command_parser`
- `cargo test -p agent tool_permissions`
- `cargo test -p agent terminal_tool`

`cargo test -p agent pattern_extraction` also passes after aligning the quoted-assignment expectation with the actual escaped regex output.

### Still remaining

- Add the primary end-to-end SEC-264 regression suite in `crates/agent/src/tools/terminal_tool.rs` that directly exercises `TerminalTool::run`.
- Add the command-description/schema assertions locking in the prohibition text in the terminal tool description.
- Update the `TerminalToolInput` doc comments / generated tool description so the model is explicitly told not to generate commands containing substitutions/interpolations.
- Decide whether `crates/agent/src/thread.rs` needs any additional tests beyond the current lower-level coverage.
- Do the broader verification pass described in Step 7 once the remaining terminal-tool/doc work is finished.

---

## Agreed behavior to implement

### Permission matching behavior

- Terminal permission matching must evaluate regexes against strings that include scalar env-var prefixes in order, followed by the command and subcommand text.
- Example: `PAGER=blah git status` must be matchable by a regex that explicitly includes `PAGER=blah`.
- Anchored patterns like `^git\b` are allowed to stop matching when env-var prefixes are present. This compatibility break is intentional.
- Array assignments in command prefixes should be ignored for this feature.
- Matching should still cover every extracted command in chained/nested command structures, as it does today.

### Assignment normalization behavior

- Scalar assignment values should be normalized like ordinary words when doing so is safe.
- Preserve quotes in assignment values when removing them would change the shell meaning of the assignment value.
- This must at least preserve quotes for values containing whitespace or command separators / control operators such as `;`.
- Example outcomes:
  - `PAGER='curl' git log` -> normalized match target includes `PAGER=curl`
  - `PAGER='less -R' git log` -> normalized match target preserves quoting for the assignment value
  - `PAGER='a;b' git log` -> normalized match target preserves quoting for the assignment value
- Do not implement this by trying to guess shell semantics in an ad hoc way. Use a conservative lexical rule: only drop quotes when the resulting assignment value can be represented safely as an unquoted assignment word; otherwise preserve the quoted source form.

### Forbidden terminal syntax

Unless terminal permissions are in unconditional allow-all mode, reject terminal commands containing any parsed shell constructs for:

- parameter expansion: `$FOO`, `${FOO}`
- special parameters: `$1`, `$@`, `$*`, `$?`, `$$`, `$!`, etc.
- command substitution: `$(...)`
- backticks: `` `...` ``
- arithmetic expansion: `$((...))`
- process substitution: `<(...)` and `>(...)`

Rules for this validation:

- Use parsed shell semantics, not raw-text substring matching.
- Apply the validation across the entire command, not just env-var assignments.
- If the parser cannot certify that the command is free of these constructs, treat the command as invalid unless terminal permissions are in unconditional allow-all mode.
- Treat all shells the same way for this rule. If parsing/validation cannot prove the command is safe, reject it unless unconditional allow-all applies.
- Hardcoded security denials must still win even in unconditional allow-all mode.

### Invalid-tool-call behavior

When forbidden syntax is present and unconditional allow-all does not apply:

- fail the terminal tool call before spawning a process
- do not show a terminal card
- do not show a permission prompt
- return an explicit error message that explains that terminal does not allow substitutions/interpolations and lists examples of the forbidden constructs

### Unconditional allow-all behavior

Define and use a helper for the specific exception above.

Treat terminal as being in unconditional allow-all mode only when the effective terminal permission configuration is equivalent to “allow every terminal call by default, with no terminal-specific allow/deny/confirm patterns constraining behavior.”

This should cover:

- no terminal-specific rules and global default is `Allow`
- terminal-specific default effectively resolves to `Allow` and terminal-specific pattern lists are empty

This should **not** cover cases where terminal has any deny/confirm/allow-pattern rules that make behavior input-dependent.

### Always-allow pattern generation and UI behavior

- The always-allow terminal pattern must incorporate env-var prefixes plus the command/subcommand portion.
- Continue including the subcommand when present.
- UI text for the permission option should preserve the user’s original whitespace.
- The stored regex should normalize token boundaries to `\s+` so it matches the normalized permission-check string rather than the exact original spacing.
- Path-like commands such as `./script.sh` or `/usr/bin/python` should continue to return no auto-generated terminal pattern.

### Tool description behavior

Update the terminal tool description/schema text so the model is explicitly told not to generate terminal commands containing substitutions/interpolations, with examples of the forbidden forms.

---

## Files that will likely change

### Primary implementation files

- `crates/shell_command_parser/src/shell_command_parser.rs`
- `crates/agent/src/pattern_extraction.rs`
- `crates/agent/src/tool_permissions.rs`
- `crates/agent/src/tools/terminal_tool.rs`
- `crates/agent/src/thread.rs`

### Test support that may need light refactoring for reuse

- `crates/agent/src/tests/mod.rs`

---

## Step 1: add the failing test suite first

The first implementation step is to add tests before changing behavior.

### 1.1 Add the main end-to-end security regression tests to `crates/agent/src/tools/terminal_tool.rs`

Status: not done yet.

Add a `#[cfg(test)]` async test suite in `terminal_tool.rs` that exercises `TerminalTool::run` directly. Reuse existing fake terminal infrastructure from `crates/agent/src/tests/mod.rs` by exposing the minimal needed helpers as `pub(crate)` under `#[cfg(test)]`, rather than duplicating a second fake terminal stack.

The tests in `terminal_tool.rs` should be the primary regression suite for this feature.

Partial progress:
- The minimal fake terminal helpers have already been exposed from `crates/agent/src/tests/mod.rs`.
- Existing terminal-tool coverage in `crates/agent/src/tests/mod.rs` still passes, but the dedicated SEC-264 direct `TerminalTool::run` suite described here has not been added yet.

#### Required rejection tests

Add tests asserting that, with non-allow-all settings, terminal rejects commands containing each forbidden construct and returns an explicit invalid-command error **before** execution:

- `echo $HOME`
- `echo ${HOME}`
- `echo $1`
- `echo $?`
- `echo $$`
- `echo $@`
- `echo $(whoami)`
- `echo \`whoami\``
- `echo $((1 + 1))`
- `cat <(ls)`
- `ls >(cat)`
- env-var prefix cases:
  - `PAGER=$HOME git log`
  - `PAGER=$(whoami) git log`
  - `GIT_SEQUENCE_EDITOR=${EDITOR} git rebase -i HEAD~2`
- multiline cases where the forbidden construct appears anywhere:
  - `echo ok\necho $HOME`
  - `PAGER=less git log\necho $(whoami)`
- nested cases:
  - `echo $(cat $(whoami).txt)`

For each of these tests, assert all of the following:

- the tool returns `Err(...)`
- the error message explicitly mentions that substitutions/interpolations are not allowed
- no terminal process is created
- no authorization prompt is emitted

#### Required allow-all exception tests

Add tests asserting that the same forbidden-construct commands are allowed to proceed when terminal is in unconditional allow-all mode, except for hardcoded non-bypassable denials.

Cover at least these configurations:

- no terminal-specific rules, global default `Allow`
- terminal-specific default resolves to `Allow` with empty pattern lists

For these tests, assert that:

- the tool proceeds to terminal creation
- there is no validation rejection

#### Required hardcoded-denial precedence tests

Add tests asserting that hardcoded non-bypassable security denials still win even when unconditional allow-all mode is active.

Include at least one command that combines a forbidden construct with a hardcoded-denied dangerous command path, and assert that the hardcoded denial still blocks it.

#### Required env-prefix permission-flow tests

Add end-to-end tests that verify the new env-prefix-aware permission behavior from `TerminalTool::run`:

- default deny + allow pattern matching `PAGER=blah git log` allows `PAGER=blah git log --oneline`
- the same pattern does **not** allow a different env-var value
- an old anchored pattern like `^git\b` no longer auto-allows `PAGER=blah git log`
- multiple scalar assignments are preserved in order for matching, e.g. `A=1 B=2 git log`
- scalar assignment with quoted whitespace value still matches only when the generated / configured pattern includes the preserved quoted form

#### Required command-description tests

Add tests that lock in the terminal tool description/schema text so future changes do not remove the explicit prohibition on substitutions/interpolations.

Assert that the description contains examples covering:

- `$VAR`
- `${VAR}`
- `$(...)`
- backticks
- `$((...))`
- process substitution

### 1.2 Add lower-level unit tests to support the end-to-end suite

Status: largely done.

Add focused unit tests in the lower-level files so failures point to the right layer:

#### In `crates/shell_command_parser/src/shell_command_parser.rs`

Add tests for:

- scalar env-var prefixes included in extracted command strings
- multiple scalar assignments preserved in order
- assignment quoting dropped when safe
- assignment quoting preserved when required for whitespace
- assignment quoting preserved when required for `;`
- array assignments ignored for prefix matching output
- forbidden constructs detected by the new validation helper
- parser/validation failure surfaces distinctly enough for terminal invalid-call handling

#### In `crates/agent/src/pattern_extraction.rs`

Add tests for:

- extracting terminal patterns that include env-var prefixes and subcommands
- preserving original whitespace in display strings
- generating regexes with `\s+` token boundaries instead of exact original spacing
- rejecting path-like commands even when env-var prefixes exist
- preserving quoted assignment values in the display/pattern source when required

#### In `crates/agent/src/tool_permissions.rs`

Add tests for:

- invalid substitution-bearing commands deny by default / confirm mode
- unconditional allow-all bypasses invalid-command rejection
- old anchored patterns no longer match env-prefixed commands
- env-prefixed allow patterns require all extracted commands to match, just like non-prefixed commands do today
- hardcoded security denials still override unconditional allow-all

### 1.3 Run the tests and confirm they fail before implementation

Status: partially done.

After adding the tests, run targeted test commands and confirm the new tests fail for the expected reasons.

What has happened so far:
- The lower-level test additions were exercised during implementation and used to drive fixes in parser, pattern-extraction, and permission behavior.
- The dedicated Step 1.1 terminal-tool regression suite still needs to be added, so this step is not fully complete against the original plan wording.

Suggested commands:

- `cargo test -p agent terminal_tool`
- `cargo test -p agent tool_permissions`
- `cargo test -p agent pattern_extraction`
- `cargo test -p shell_command_parser`

Do **not** modify the tests after this point except to fix broken test setup. The exact same assertions should be used to validate the implementation later.

---

## Step 2: introduce structured terminal-command parsing helpers

Status: done.

The current `extract_commands` API is too lossy for this feature because it throws away:

- whether env-var prefixes were present
- when assignment quoting must be preserved
- original whitespace for UI display
- whether forbidden substitutions/interpolations appeared anywhere in the parsed command

Introduce a structured helper layer in `shell_command_parser` rather than bolting more behavior onto raw `Vec<String>` output.

### 2.1 Add a structured representation for a simple command prefix

Add a small internal data structure that captures, for the first simple command in a permission target:

- ordered scalar assignment prefixes
- whether each assignment value can be safely emitted unquoted
- normalized text for permission matching
- display text with preserved original whitespace for the allow-button label
- command token
- optional subcommand token

Avoid creating a new file unless it becomes clearly necessary; prefer extending `shell_command_parser.rs`.

### 2.2 Add a validation helper for forbidden constructs

Add a new parser-backed validation helper that walks the parsed AST/word pieces and returns either:

- success: no forbidden substitution/interpolation constructs were found
- failure: one or more forbidden constructs were found
- failure: parsing/validation could not certify the input as safe

The validator should inspect all command words, env-var assignments, redirect targets, nested substitutions, and any other word-bearing syntax that can contain the forbidden constructs.

### 2.3 Keep `extract_commands` working, but extend its normalization

Update `extract_commands_from_simple_command` so extracted permission-match strings include scalar env-var prefixes in order.

Key points:

- include scalar assignments in the same extracted command string as the command they prefix
- ignore array assignments for this feature
- preserve assignment quoting only when required by the conservative lexical rule
- continue extracting nested commands from command substitutions/process substitutions so hardcoded-denial logic still sees them where appropriate

---

## Step 3: implement invalid-command handling in the terminal permission path

Status: done.

Implement the invalid-command behavior in the permission layer, not as a late runtime check after process creation.

### 3.1 Add an unconditional-allow-all helper

In `tool_permissions.rs`, add a helper that answers whether terminal is effectively in unconditional allow-all mode.

This helper should be narrow and explicit. It should only return true when the effective terminal configuration really means “allow every terminal call by default unless blocked by hardcoded rules.”

### 3.2 Integrate validation into terminal permission decisions

Update `ToolPermissionDecision::from_input` so that for terminal inputs it:

1. still checks hardcoded non-bypassable denials first
2. validates the command for forbidden substitution/interpolation constructs
3. if the command is invalid:
   - return an explicit `Deny(...)` with the new invalid-command message, unless unconditional allow-all applies
   - if unconditional allow-all applies, continue evaluating normally
4. only then continue with the existing allow/deny/confirm matching flow

### 3.3 Use an explicit invalid-command error message

Make the returned denial text explicit and actionable. It should mention that terminal does not allow shell substitutions/interpolations and give concrete examples.

---

## Step 4: update terminal pattern extraction and permission-option UI

Status: mostly done, with possible follow-up verification in `thread.rs`.

### 4.1 Refactor terminal pattern extraction to use structured parsing data

`extract_terminal_pattern` and `extract_terminal_pattern_display` should stop depending only on the whitespace-split normalized command string.

Instead, they should use the new structured command-prefix helper so they can:

- include env-var prefixes in the auto-generated pattern
- include the subcommand when present
- preserve assignment quoting when required
- preserve original whitespace in the display text
- still reject path-like commands

### 4.2 Preserve UI whitespace while normalizing regex token boundaries

Implement the split behavior explicitly:

- display/button label preserves original whitespace from the user input for the env-prefix + command/subcommand portion
- regex pattern uses normalized `\s+` between tokens so it matches the normalized permission-check string

### 4.3 Verify permission option generation in `thread.rs`

Confirm that `ToolPermissionContext::build_permission_options` picks up the new extractor behavior without requiring semantic changes outside of the extractor. Add or update tests only if the new lower-level tests do not already lock this in sufficiently.

---

## Step 5: update the terminal tool description and input schema docs

Status: not done yet.

Update the `TerminalToolInput` doc comments in `crates/agent/src/tools/terminal_tool.rs` so the generated tool description tells the model not to generate commands containing substitutions/interpolations.

The description should explicitly mention that terminal commands may not include examples such as:

- `$VAR`
- `${VAR}`
- `$(...)`
- backticks
- `$((...))`
- `<(...)`
- `>(...)`

The message should also make clear that the model should instead resolve values itself before calling the tool, or ask the user.

---

## Step 6: implement without changing the tests

Status: partially done.

After Steps 2–5 are in place, make the implementation changes needed to satisfy the Step 1 tests.

Current state:
- The parser, pattern extraction, and terminal permission implementation work described in Steps 2–4 has been completed.
- The remaining incomplete work is the direct terminal-tool regression suite and the terminal tool description/schema updates from Steps 1.1 and 5.

Important constraint:

- do not weaken, delete, or rewrite the new security tests in order to make them pass
- if a test fails unexpectedly, fix the implementation or test setup, not the intended assertion

---

## Step 7: verification

Status: partially done.

Run the same test suite from Step 1 again, without modifying those tests.

Completed verification so far:
- `cargo test -p shell_command_parser`
- `cargo test -p agent tool_permissions`
- `cargo test -p agent pattern_extraction`
- `cargo test -p agent terminal_tool`

Still to do:
- Re-run the final verification after the remaining Step 1.1 and Step 5 work is complete.
- Run the broader targeted pass (`cargo test -p agent`) and clippy pass if the remaining changes introduce any new warnings or lints.

Suggested commands:

- `cargo test -p agent terminal_tool`
- `cargo test -p agent tool_permissions`
- `cargo test -p agent pattern_extraction`
- `cargo test -p shell_command_parser`

Then run a broader targeted pass for safety:

- `cargo test -p agent`
- `cargo test -p shell_command_parser`

If warnings or clippy issues are introduced during implementation, run:

- `./script/clippy -p agent -p shell_command_parser`

---

## Implementation notes / likely pitfalls

- The current `extract_commands` return type is too lossy to support both regex matching and UI display requirements on its own.
- Preserve the existing hardcoded-denial precedence exactly.
- Be careful not to re-enable allow-pattern bypasses through parse failures or unsupported shell cases.
- Do not silently fall back to raw-string allow matching when parser-backed validation fails.
- Keep command-substitution extraction for hardcoded denial checks even while invalidating such commands for normal terminal use.
- Avoid introducing panics when walking assignment/word AST nodes.
- Prefer conservative preservation of assignment quoting when the lexical rule is uncertain.
- Keep the implementation in existing files unless there is a clear need for a new logical component.

---

## Completion criteria

This work is done when all of the following are true:

Current checkpoint:
- The parser, pattern extraction, and permission-layer behavior are in place and covered by targeted tests.
- The remaining blocking items are:
  - dedicated direct `TerminalTool::run` SEC-264 regression tests
  - terminal tool description/schema text updates
  - final broad verification after those are complete

- the new failing security tests were added first
- those tests were confirmed to fail before implementation
- the implementation was completed without changing those tests
- the same tests now pass
- terminal permission matching includes env-var prefixes
- terminal auto-generated allow patterns include env-var prefixes and subcommands
- terminal invalidates substitution/interpolation-bearing commands unless unconditional allow-all applies
- hardcoded denials still override everything
- terminal tool description explicitly discourages forbidden shell syntax
