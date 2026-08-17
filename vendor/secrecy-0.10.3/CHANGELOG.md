# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.3 (2024-10-09)
### Added
- Make integer primitive `SecretSlice`s cloneable ([#1236])
- Impl `From<&str>` for `SecretString` ([#1237])

[#1236]: https://github.com/iqlusioninc/crates/pull/1236
[#1237]: https://github.com/iqlusioninc/crates/pull/1237

## 0.10.2 (2024-09-20)
### Added
- Impl `Deserialize` for `SecretString` ([#1220])

[#1220]: https://github.com/iqlusioninc/crates/pull/1220

## 0.10.1 (2024-09-18)
### Added
- Impl `Clone` for `SecretString` ([#1217])

[#1217]: https://github.com/iqlusioninc/crates/pull/1217

## 0.10.0 (2024-09-17)

This release represents a significant redesign of the `secrecy` crate. We will update this section
with upgrade instructions based on feedback from people upgrading, as it's been a long time since
the previous release, and this release includes a number of breaking changes.

The most notable change is the generic `Secret<T>` type has been removed: instead use `SecretBox<T>`
which stores secrets on the heap instead of the stack. Many of the other changes fall out of this
change and things which were previously type aliases of `Secret<T>` are now type aliases of
`SecretBox<T>`.

This unfortunately means this crate no longer has support for "heapless" `no_std` targets. We don't
have a good solution for these targets, which was a motivation for this change in the first place.

### Added
- `SecretBox::{init_with, try_init_with}` ([#1212])
- `SecretBox::init_with_mut` ([#1213])
- `?Sized` bounds for `SecretBox` ([#1213])
- `SecretSlice<T>` ([#1214])

### Changed
- Rust 2021 edition upgrade ([#889])
- MSRV 1.60 ([#1105])
- `SecretBox<T>` is now a newtype rather than a type alias of `Secret<Box<T>> ([#1140])
- `SecretString` is now a type alias for `SecretBox<str>` ([#1213])
- Disable `serde` default features ([#1194])

### Removed
- `alloc` feature: now a hard dependency ([#1140])
- `bytes` crate integration: no replacement ([#1140])
- `DebugSecret` trait: no replacement ([#1140])
- `Secret<T>`: use `SecretBox<T>` instead ([#1140])

[#889]: https://github.com/iqlusioninc/crates/pull/889
[#1105]: https://github.com/iqlusioninc/crates/pull/1105
[#1140]: https://github.com/iqlusioninc/crates/pull/1140
[#1194]: https://github.com/iqlusioninc/crates/pull/1194
[#1212]: https://github.com/iqlusioninc/crates/pull/1212
[#1213]: https://github.com/iqlusioninc/crates/pull/1213
[#1214]: https://github.com/iqlusioninc/crates/pull/1214

## 0.9.0 (Skipped)

## 0.8.0 (2021-07-18)

NOTE: This release includes an MSRV bump to Rust 1.56. Please use `secrecy = "0.7.0"`
if you would like to support older Rust versions.

### Added
- impl `From<T>` for `Secret` ([#482])

### Changed
- Bump `bytes` to v1.0 ([#592])
- Switch to `resolver = "2"`; MSRV 1.56 ([#755])

[#482]: https://github.com/iqlusioninc/crates/pull/482
[#592]: https://github.com/iqlusioninc/crates/pull/592
[#755]: https://github.com/iqlusioninc/crates/pull/755

## 0.7.0 (2020-07-08)
### Added
- Re-export zeroize ([#466])
- `rustdoc` improvements ([#464], [#465])

### Changed
- Have `DebugSecret` take a formatter ([#467])
- Make `FromStr` impl for `SecretString` be `Infallible` ([#323])

### Fixed
- Use `SerializableSecret` in `Serialize` bounds ([#463])

[#467]: https://github.com/iqlusioninc/crates/pull/467
[#466]: https://github.com/iqlusioninc/crates/pull/466
[#465]: https://github.com/iqlusioninc/crates/pull/465
[#464]: https://github.com/iqlusioninc/crates/pull/464
[#463]: https://github.com/iqlusioninc/crates/pull/463
[#323]: https://github.com/iqlusioninc/crates/pull/323

## 0.6.0 (2019-12-12)

- Impl `CloneableSecret` for `Secret<[T; N]>` where `T: Clone` ([#311])
- Impl `DebugSecret` for `[T; N]` where `N` <= 64 ([#310])
- Impl `FromStr` for `SecretString` ([#309])
- Upgrade to `bytes` v0.5 ([#301], [#308], [#312])

[#312]: https://github.com/iqlusioninc/crates/pull/312
[#311]: https://github.com/iqlusioninc/crates/pull/311
[#310]: https://github.com/iqlusioninc/crates/pull/310
[#309]: https://github.com/iqlusioninc/crates/pull/309
[#308]: https://github.com/iqlusioninc/crates/pull/308
[#301]: https://github.com/iqlusioninc/crates/pull/301

## 0.5.2 (2019-12-18)

- Backport Impl `FromStr` for `SecretString` ([#309])

[#309]: https://github.com/iqlusioninc/crates/pull/309

## 0.5.1 (2019-11-30)

- Change default `DebugSecret` string to `[REDACTED]` ([#290])

[#290]: https://github.com/iqlusioninc/crates/pull/290

## 0.5.0 (2019-10-13)

- Upgrade to `zeroize` v1.0.0 ([#279])

[#279]: https://github.com/iqlusioninc/crates/pull/279

## 0.4.1 (2019-10-13)

- Upgrade to `zeroize` v1.0.0-pre ([#268])

[#268]: https://github.com/iqlusioninc/crates/pull/268

## 0.4.0 (2019-09-03)

- Add `SerializableSecret` ([#262])
- Add (optional) concrete `SecretBytes` type ([#258], [#259], [#260], [#261])

[#262]: https://github.com/iqlusioninc/crates/pull/262
[#261]: https://github.com/iqlusioninc/crates/pull/261
[#260]: https://github.com/iqlusioninc/crates/pull/260
[#259]: https://github.com/iqlusioninc/crates/pull/259
[#258]: https://github.com/iqlusioninc/crates/pull/258

## 0.3.1 (2019-08-26)

- Impl `CloneableSecret` for `String` ([#256])

[#256]: https://github.com/iqlusioninc/crates/pull/256

## 0.3.0 (2019-08-20)

- Add support for `alloc` types ([#253])
- `zeroize` v0.10.0 ([#248])
- Add a default impl for `DebugSecret` trait ([#241])

[#253]: https://github.com/iqlusioninc/crates/pull/253
[#248]: https://github.com/iqlusioninc/crates/pull/248
[#241]: https://github.com/iqlusioninc/crates/pull/241

## 0.2.2 (2019-06-28)

- README.md: add Gitter badges; update image links ([#221])

[#221]: https://github.com/iqlusioninc/crates/pull/221

## 0.2.1 (2019-06-04)

- `zeroize` v0.9.0 ([#215])

[#215]: https://github.com/iqlusioninc/crates/pull/215

## 0.2.0 (2019-05-29)

- Add `CloneableSecret` marker trait ([#210])

[#210]: https://github.com/iqlusioninc/crates/pull/210

## 0.1.0 (2019-05-23)

- Initial release
