# Potentially Related GitHub Issues

## High Confidence
- None found

## Medium Confidence
- [#49237](https://github.com/zed-industries/zed/pull/49237) — multi_buffer: Guard unresolved anchors during summary resolution
  - Why: Addresses stale anchor resolution in MultiBuffer after excerpt replacement, which is the same category of bug.
  - Evidence: Both crashes involve anchor comparison with stale/unresolved excerpts. This PR guards `can_resolve()` before anchor operations - a similar pattern could apply to `disjoint_in_range`.

- [#49047](https://github.com/zed-industries/zed/pull/49047) — multi_buffer: Fix "cannot seek backward" crash in summaries_for_anchors
  - Why: Same root cause pattern - stale excerpt locators causing ordering violations during anchor resolution.
  - Evidence: After `update_path_excerpts` replaces excerpts, stale anchors can violate ordering assumptions in binary search-like operations. Similar to how `disjoint_in_range` binary search can produce `start_ix > end_ix`.

- [#40249](https://github.com/zed-industries/zed/pull/40249) — editor: Fix `SelectionsCollection::disjoint` not being ordered correctly  
  - Why: Directly addresses ordering issues in SelectionsCollection::disjoint that can cause panics.
  - Evidence: Fixed "cannot seek backwards" panics within SelectionsCollection by enforcing selection ordering invariants. Related to ZED-253, ZED-ZJ, and other selection-related crashes.

## Low Confidence
- [#25141](https://github.com/zed-industries/zed/pull/25141) — editor: Fix highlight selection panic
  - Why: Different selection-related crash, but shows the pattern of selection state becoming invalid.
  - Evidence: Selection crash during highlighting - different trigger but same general area of code.

## Reviewer Checklist
- [ ] Confirm High confidence issues should be referenced in PR body
- [ ] Confirm any issue should receive closing keywords (`Fixes #...`)
- [ ] Reject false positives before merge
