# Potentially Related GitHub Issues

## High Confidence
- None found

No open issues in zed-industries/zed directly match the BiDi assertion crash in cosmic_text. The crash occurs in a third-party dependency, not in Zed's code.

## Medium Confidence
- [#39385](https://github.com/zed-industries/zed/pull/39385) — gpui: Fix displaying of RTL text (CLOSED)
  - Why: Related to RTL text rendering, though this was a display fix rather than a crash fix
  - Evidence: Same general area (RTL text handling on Linux)

- [#35613](https://github.com/zed-industries/zed/pull/35613) — Implement Bidirectionality (CLOSED)
  - Why: Related to BiDi layout support
  - Evidence: Same general area (bidirectional text), though focused on UI layout direction

## Low Confidence
- [#48696](https://github.com/zed-industries/zed/pull/48696) — Fix Thai character rendering in terminal (OPEN)
  - Why: Related to complex script rendering issues
  - Evidence: Both involve text rendering with non-Latin scripts

## External References
- [cosmic-text#442](https://github.com/pop-os/cosmic-text/issues/442) — Allow breaking up shaping runs
  - Why: Directly related issue in the upstream cosmic_text library
  - Evidence: Discusses the assertion panic when handling BiDi text with different shaping run requirements

- [cosmic-text#252](https://github.com/pop-os/cosmic-text/issues/252) — Bidirectional text overflows buffer instead of wrapping
  - Why: Related upstream issue about BiDi text handling bugs in cosmic_text
  - Evidence: Same upstream library, same general problem area (BiDi text)

## Reviewer Checklist
- [ ] Confirm High confidence issues should be referenced in PR body
- [ ] Confirm any issue should receive closing keywords (`Fixes #...`)
- [ ] Reject false positives before merge
