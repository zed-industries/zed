<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

# Changelog

The latest published Kurbo release is [0.4.0](#040-2025-10-23) which
was released on 2025-10-23. You can find its changes
[documented below](#040-2025-10-23).

## [0.4.0] (2025-10-23)

This release has an [MSRV][] of 1.85.
It is the first release since `polycool` was transferred to the Linebender organization.

## Changed

- Rename `Poly::magnitude` to `Poly::max_abs_coefficient`. ([#502][] by [@jneem][])

## Fixed

- Avoid potential underflow in `DynPoly::deriv` ([#502][] by [@jneem])
- Avoid NaN during Newton iteration ([#502][] by [@jneem])

Note: A changelog was not kept before this release

[@jneem]: https://github.com/jneem

[#502]: https://github.com/linebender/kurbo/pull/502

[0.4.0]: https://github.com/linebender/kurbo/releases/tag/v0.4.0-polycool

[MSRV]: README.md#minimum-supported-rust-version-msrv
