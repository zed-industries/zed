<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

# Changelog

The latest published Linebender Resource Handle release is [0.1.1](#011-2025-09-16) which was released on 2025-09-16.
You can find its changes [documented below](#011-2025-09-16).

## [Unreleased][]

This release has an [MSRV][] of 1.70.

## [0.1.1][] (2025-09-16)

This release has an [MSRV][] of 1.70.

### Fixes

- Allow compilation to targets without 64-bit atomics. ([#11][] by [@nicoburns][])

## 0.1.0 (2025-09-09)

This release has an [MSRV][] of 1.70.

This is the initial release.

- Rename `Font` to `FontData`. ([#5][] by [@nicoburns][])
- Add the `Font`, `Blob`, and `WeakBlob` types. (Initial commits by [@waywardmonkeys][])

[@nicoburns]: https://github.com/nicoburns
[@waywardmonkeys]: https://github.com/waywardmonkeys

[#5]: https://github.com/linebender/raw_resource_handle/pull/5
[#11]: https://github.com/linebender/raw_resource_handle/pull/11

[Unreleased]: https://github.com/linebender/anymore/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/linebender/parley/compare/v0.1.0...v0.1.1

[MSRV]: README.md#minimum-supported-rust-version-msrv
