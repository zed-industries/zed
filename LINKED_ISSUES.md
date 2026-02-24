# Potentially Related GitHub Issues

## High Confidence

- [#46758](https://github.com/zed-industries/zed/pull/46758) — gpui: Remove blade, reimplement linux renderer with wgpu
  - Why: This PR removed the blade_graphics dependency that contained the crashing `unwrap()` call and reimplemented the Linux renderer with wgpu which handles surface validation more gracefully.
  - Evidence: The crash stack trace shows `blade_graphics::hal::surface::<T>::reconfigure_surface` failing with "validation failed", which is directly addressed by removing blade. The PR description explicitly mentions fixing issues related to the blade graphics library.

## Medium Confidence

- None found

## Low Confidence

- [#43070](https://github.com/zed-industries/zed/pull/43070) — gpui: Implement GPU device loss recovery for Linux X11
  - Why: Related to GPU surface handling on Linux, though focused on device loss rather than initial surface creation.
  - Evidence: Both involve GPU surface management on Linux, but different failure modes.

- [#46281](https://github.com/zed-industries/zed/pull/46281) — Use transparent clear color for opaque windows on Linux
  - Why: Related to window surface configuration on Linux with blade renderer.
  - Evidence: Involves blade renderer surface configuration, but addresses transparency rather than validation failures.

## Reviewer Checklist

- [x] Confirm High confidence issues should be referenced in PR body
- [ ] Confirm any issue should receive closing keywords (`Fixes #...`)
- [x] Reject false positives before merge

## Notes

The primary crash (ZED-4HJ) has already been addressed by the blade-to-wgpu migration in PR #46758 which was merged on 2026-02-13. Users on stable version 0.224.11 are still experiencing this crash and should upgrade to a version that includes the wgpu renderer.

The current PR adds a defensive fix to ensure the wgpu renderer also handles zero-size surface dimensions consistently, matching the existing pattern in `update_drawable_size()`.
