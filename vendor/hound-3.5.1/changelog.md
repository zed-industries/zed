Changelog
=========

3.5.1
-----

Released 2023-09-25.

**Compatibility**:

 * Ensures compatibility with Rust 1.40.0 through 1.72.1. This bumps the minimum
   supported Rust version from 1.16 to 1.40.

Changes:

 * Soundness: Wrap writes to uninitialized memory in `mem::MaybeUninit`. The
   unsoundness was present in all versions since 0.2.0. There is no evidence
   that rustc took advantage of the unsoundness to compile programs in a
   problematic way.

Thanks to Cam Lloyd for originally contributing these changes, and thanks to
Maxwell McKinnon for rebasing them on top of 3.5.0.

3.5.0
-----

Released 2022-09-09.

This is a maintenance release that includes most of the bugfixes and features
that have been contributed since 3.4.0, which could be cherry-picked on top of
3.4.0. Some other contributions with more far-reaching changes remain unreleased
as of yet.

**Compatibility**:

 * Ensures compatibility with Rust 1.16 through 1.63 stable. Previously the
   minimum supported Rust version was 1.4. Cargo from 1.4 is no longer
   compatible with the current crates.io registry, and Rustup fails signature
   verification for these binaries, so it is infeasible to continue to support
   it.

New features:

 * Add support for `S24_LE` files, which store 24 bits in 4 bytes ([#40][40],
   [#41][41])
 * Add `WavWriter::new_with_spec_ex` ([#42][42])
 * Add `WavSpec::into_header_for_infinite_file` ([#33][33], [#36][36])

Bugfixes and compatibility improvements:

 * Handle files that have the `wValidBitsPerSample` field set to zero
   ([#50][50], [#51][51])
 * Avoid overflow in the channel mask when writing file with more than 32
   channels ([#59][59], [#60][60])

[33]: https://github.com/ruuda/hound/pull/33
[36]: https://github.com/ruuda/hound/pull/36
[40]: https://github.com/ruuda/hound/pull/40
[41]: https://github.com/ruuda/hound/pull/41
[42]: https://github.com/ruuda/hound/pull/42
[50]: https://github.com/ruuda/hound/pull/50
[51]: https://github.com/ruuda/hound/pull/51
[59]: https://github.com/ruuda/hound/pull/59
[60]: https://github.com/ruuda/hound/pull/60

Many thanks to Diffuse, Fletcher Woodruff, Matt Wilkinson, Vitaly Vi Shukela,
and Tuckerrrrrrrrrr for contributing to this release.

3.4.0
-----

Released 2018-04-07.

**Breaking changes**:

- None.

Release highlights:

- Exposes `read_wave_header()`, to quickly determine whether a file could be
  a wav file.
- Adds support for appending to an existing file. See `WavWriter::append()` for
  constructing a writer that appends to a file, and `WavWriter::new_append()`
  for the generic case.
- Adds `WavWriter::flush()` to flush the underlying writer and update the
  header. This can be used to minimize data loss when writing a large file.
- Adds `WavWriter::duration()`, `WavWriter::len()`, and `WavWriter::spec()` to
  obtain the duration and number of samples written so far, and the spec of the
  file being written. The latter is useful when appending.
- Hound now fails earlier when requesting to write an unsupported spec:
  `WavWriter::new()` will already return `Error::Unsupported`. Previously this
  error was returned when writing a sample.
- Hound now verifies that the data chunk has no trailing bytes.
- `WavWriter::finalize()` now performs a flush as its last operation, to be able
  to observe errors when using a buffered writer.
- Ensures compatibility with Rust 1.4 through 1.25 stable.

3.3.1
-----

Released 2018-02-18.

**Breaking changes**:

- None.

Release highlights:

- Hound now reads certain WAVEFORMATEX files that were previously
  rejected incorrectly.
- Ensures compatibility with Rust 1.4 through 1.24 stable.

3.3.0
-----

Released 2017-12-02.

**Breaking changes**:

- None.

Release highlights:

- Hound now supports seeking to a particular time in the file.
  See `WavReader::seek()`.
- Ensures compatibility with Rust 1.4 through 1.22 stable.

Many thanks to Mitchell Nordine for contributing to this release.

3.2.0
-----

Released 2017-10-14.

**Breaking changes**:

- None.

Release highlights:

- Hound will now write the older PCMWAVEFORMAT format whenever possible, rather
  than the newer WAVEFORMATEXTENSIBLE, to improve compatibility.
- Certain nonstandard files (produced among others by “Pro Tools”) can now
  be read.
- Ensures compatibility with Rust 1.4 through 1.21 stable.

Many thanks to Denis Kolodin for contributing to this release.

3.1.0
-----

Released 2017-04-09.

**Breaking changes**:

- None.

Release highlights:

- Support for writing IEEE float was added.
- The cpal example was updated, and it now compiles on OS X.
- An OS X target was added to the CI configuration.
- Ensures compatibility with Rust 1.4 through 1.16 stable.

Many thanks to Alex Zywicki for contributing to this release.

3.0.1
-----

Released 2017-04-01.

This release fixes a few bugs discovered through fuzzing.

**Breaking changes**:

- None.

Release highlights:

- Fixes high memory usage issue that could occur when reading unknown blocks.
- Resolve various division by zero and arithmetic overflow errors.
- Ensures compatibility with Rust 1.4 through 1.16 stable.

3.0.0
-----

Released 2016-11-27.

This release focuses on improving write performance.

**Breaking changes**:

- When a `WavWriter` is constructed, the header is now written immediately,
  therefore the constructor now returns a `Result`.

Other changes:

- `WavWriter` no longer maintains a buffer internally.
  `WavWriter::create()` does still wrap the file it opens in a buffered writer.
- Adds `SampleWriter16` for fast writing of 16-bit samples. Dedicated
  writers for other bit depths might be added in future releases.

Upgrading requires dealing with the `Result` in `WavWriter::new()`
and `WavWriter::create()`. In many cases this should be as simple as
wrapping the call in a `try!()`, or appending a `?` on recent versions
of Rust.

2.0.0
-----

Released 2016-07-31.

**Breaking changes**:

- Support for Rust 1.0 through 1.3 has been dropped.
- The `WavSpec` struct gained a new `sample_format` member. To upgrade,
  add `sample_format: hound::SampleFormat::Int` to places where a `WavSpec`
  is constructed.

Release highlights:

- Ensures compatibility with Rust 1.4 through 1.10.
- Adds support for reading files with 32-bit IEEE float samples.

Many thanks to Mitchell Nordine for his contributions to this release.

1.1.0
-----

Released 2015-09-14.

Release highlights:

- New `WavReader::into_inner` method for consistency with the standard library.
- New `WavReader::into_samples` method for ergonomics and consistency.
- Ensures compatibility with Rust 1.4.

Many thanks to Pierre Krieger for his contributions to this release.

1.0.0
-----

Released 2015-07-21.

This is the first stable release of Hound. Only small changes have been made
with respect to v0.4.0. Release highlights:

- `WavWriter::create` now wraps the writer in a `BufWriter`.
- `WavSamples` now implements `ExactSizeIterator`.
- `WavReader::spec` now returns the spec by value.
- Internal cleanups

0.4.0
-----

Released 2015-05-16.

Release highlights:

- Works with Rust 1.0.0.
- Hound can now read and write files with 8, 16, 24, or 32 bits per sample.
- Better error reporting
- Improved documentation
- An improved test suite

0.3.0
-----

Released 2015-05-05.

Release highlights:

- Hound can now read WAVEFORMATEXTENSIBLE, so it can read the files it writes.
- Hound can now read files with PCMWAVEFORMAT and WAVEFORMATEX header.
- Hound now uses a custom error type.
- New convenient filename-based constructors for `WavReader` and `WavWriter`.
- More examples
- An improved test suite

0.2.0
-----

Released 2015-04-09.

This version adds support for decoding wav files in addition to writing them.

0.1.0
-----

Released 2015-04-01.

Initial release with only write support.
