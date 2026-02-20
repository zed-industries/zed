# Potentially Related GitHub Issues

## High Confidence
- None found

No issues in `zed-industries/zed` directly reference the same crash site (`clean_zeta1_model_output`) or the exact error message pattern from this Sentry crash.

## Medium Confidence
- None found

Web search did not surface any `zed-industries/zed` issues with partial overlap (same subsystem + UTF-8 char boundary panic symptom).

## Low Confidence
- [#27164](https://github.com/zed-industries/zed/issues/27164) — Zed crashes when triggering inline assistant while edit predictions are shown
  - Why: Same general area (edit predictions), but the crash mechanism differs (inline assistant interaction vs. UTF-8 processing)
  - Evidence: Different code path, no UTF-8 char boundary error involved

- [#46880](https://github.com/zed-industries/zed/issues/46880) — Copilot edit predictions broken with v0.219.4
  - Why: Related to edit prediction functionality, but appears to be about functionality failure rather than a crash
  - Evidence: No UTF-8 or char boundary error mentioned

## Notes

The UTF-8 char boundary panic pattern is well-known in the Rust ecosystem, appearing in similar forms in:
- helix-editor/helix#7273 (Cyrillic text handling crash)
- rust-lang/rustfmt#1464 (multibyte character boundary panic)

However, no direct matches were found in `zed-industries/zed`. This may be the first report of this specific crash in Zed's edit prediction system.

## Reviewer Checklist
- [ ] Confirm High confidence issues should be referenced in PR body
- [ ] Confirm any issue should receive closing keywords (`Fixes #...`)
- [ ] Reject false positives before merge
