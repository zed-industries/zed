# Changelog

All notable changes to this project will be documented in this file.

Migration guides for incompatible versions can be found in `UPGRADE.md` file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `Chirp` and `Empty` now implement `Iterator::size_hint` and `ExactSizeIterator`.
- `SamplesBuffer` now implements `ExactSizeIterator`.
- `Zero` now implements `try_seek`, `total_duration` and `Copy`.
- Added `Source::is_exhausted()` helper method to check if a source has no more samples.
- Added `Red` noise generator that is more practical than `Brownian` noise.
- Added `std_dev()` to `WhiteUniform` and `WhiteTriangular`.
- Added a macro `nz!` which facilitates creating NonZero's for `SampleRate` and
  `ChannelCount`.
- Adds a new input source: Microphone.
- Adds a new method on source: record which collects all samples into a
  SamplesBuffer.
- Adds `wav_to_writer` which writes a `Source` to a writer.
- Added supported for `I24` output (24-bit samples on 4 bytes storage).
- Added audio dithering support with `dither` feature (enabled by default):
  - Four dithering algorithms: `TPDF`, `RPDF`, `GPDF`, and `HighPass`
  - `DitherAlgorithm` enum for algorithm selection
  - `Source::dither()` function for applying dithering
- Added `64bit` feature to opt-in to 64-bit sample precision (`f64`).
- Added `SampleRateConverter::inner` to get underlying iterator by ref.

### Fixed
- docs.rs will now document all features, including those that are optional.
- `Chirp::next` now returns `None` when the total duration has been reached, and will work
  correctly for a number of samples greater than 2^24.
- `PeriodicAccess` is slightly more accurate for 44.1 kHz sample rate families.
- Fixed audio distortion when queueing sources with different sample rates/channel counts or transitioning from empty queue.
- Fixed `SamplesBuffer` to correctly report exhaustion and remaining samples.
- Improved precision in `SkipDuration` to avoid off-by-a-few-samples errors.
- Fixed channel misalignment in queue with non-power-of-2 channel counts (e.g., 6 channels) by ensuring frame-aligned span lengths.
- Fixed channel misalignment when sources end before their promised span length by padding with silence to complete frames.
- Fixed `Empty` source to properly report exhaustion.
- Fixed `Zero::current_span_len` returning remaining samples instead of span length.

### Changed
- Breaking: _Sink_ terms are replaced with _Player_ and _Stream_ terms replaced
  with _Sink_. This is a simple rename, functionality is identical.
    - `OutputStream` is now `MixerDeviceSink` (in anticipation of future
      `QueueDeviceSink`)
    - `OutputStreamBuilder` is now `DeviceSinkBuilder`
    - `open_stream_or_fallback` is now `open_sink_or_fallback`
    - `open_default_stream` is now `open_default_sink`
    - `open_stream` is now `open_mixer` (in anticipation of future `open_queue`)
    - `Sink` is now `Player`
    - `SpatialSink` is now `SpatialPlayer`
    - `StreamError` is now `OsSinkError`
    - `SamplesBuffer::new(1, 16000, samples)` would now look like `SamplesBuffer::new(nz!(1), nz!(16000), samples)` using the new `nz!` macro
- `output_to_wav` renamed to `wav_to_file` and now takes ownership of the `Source`.
- `Blue` noise generator uses uniform instead of Gaussian noise for better performance.
- `Gaussian` noise generator has standard deviation of 0.6 for perceptual equivalence.
- `Velvet` noise generator takes density in Hz as `usize` instead of `f32`.
- Upgraded `cpal` to v0.17.
- Clarified `Source::current_span_len()` contract documentation.
- Improved queue, mixer and sample rate conversion performance.

## Version [0.21.1] (2025-07-14)

### Changed
- Upgrade `cpal` to v0.16.
- Update dependencies.

### Fixed
- Fix clippy warnings with `Source::white` and `Source::pink` functions.

## Version [0.21.0] (2025-07-12)

### Added
- Added `Source::amplify_decibel()` method to control volume by decibels.
- Added `Source::amplify_normalized()` method to perceptually modify volume.
- Adds a function to write a `Source` to a `wav` file, enable the `wav_output` feature and see
  `output_to_wav`.
- Output audio stream buffer size can now be adjusted.
- Sources for directly generating square waves, triangle waves, square waves, and
  sawtooths have been added.
- An interface for defining `SignalGenerator` patterns with an `fn`, see
  `GeneratorFunction`.
- Minimal builds without `cpal` audio output are now supported.
  See `README.md` for instructions. (#349)
- Added `Sample::is_zero()` method for checking zero samples.
- Added `DecoderBuilder` for improved configuration.
- Added `Pausable::is_paused()` method for checking if source is paused.
- Using `Decoder::TryFrom` for `File` now automatically wraps in `BufReader` and sets `byte_len`.
  `TryFrom<Cursor<T>>` and `TryFrom<BufReader>` are also supported.
- Added `Source::distortion()` method to control distortion effect by `gain` and `threshold`.
- Added `OutputStream::config()` method to access an `OutputStream`'s `OutputStreamConfig` once
  an `OutputStream` has been built.
- Added `OutputStreamConfig::channel_count()`, `OutputStreamConfig::sample_rate()`,
  `OutputStreamConfig::buffer_size()` and `OutputStreamConfig::sample_format()` getters to access
  an `OutputStreamConfig`'s channel count, sample rate, buffer size and sample format values.
- Added `Source::limit()` method for limiting the maximum amplitude of a source.
- Added more noise generators: `WhiteGaussian`, `WhiteTriangular`, `Blue`, `Brownian`, `Violet`,
  and `Velvet`.

### Changed
- Breaking: `OutputStreamBuilder` should now be used to initialize an audio output stream.
- Breaking: `OutputStreamHandle` removed, use `OutputStream` and `OutputStream::mixer()` instead.
- Breaking: `DynamicMixerController` renamed to `Mixer`, `DynamicMixer` renamed to `MixerSource`.
- Breaking: `Sink::try_new` renamed to `connect_new` and does not return error anymore.
            `Sink::new_idle` was renamed to `new`.
- Breaking: `symphonia::SeekError` has a new variant `AccurateSeekNotSupported`
  and variants `Retrying` and `Refining` have been removed. Catching this error
  may allow a caller to retry in coarse seek mode.
- Breaking: `symphonia::SeekError` has a new variant `RandomAccessNotSupported`. This error usually means that you are trying to seek backward without `is_seekable` or `byte_len` set: use `Decoder::try_from` or `DecoderBuilder` for that.
- Breaking: In the `Source` trait, the method `current_frame_len()` was renamed to `current_span_len()`.
- Breaking: `Decoder` now outputs `f32` samples.
- Breaking: The term 'frame' was renamed to 'span' in the crate and documentation.
- Breaking: `LoopedDecoder` now returns `None` if seeking fails during loop reset.
- Breaking: `ReadSeekSource::new()` now takes `Settings`.
- Breaking: Sources now use `f32` samples. To convert to and from other types of samples use functions from `dasp_sample` crate. For example `DaspSample::from_sample(sample)`.
- Breaking: `WhiteNoise` and `PinkNoise` have been renamed to `noise::WhiteUniform` and
  `noise::Pink`.
- Breaking: As optional features are now available: CAF and MKV containers, MP1/MP2 and ADPCM decoders. Previously, the ADPCM decoder was enabled when `symphonia-wav` was.
- docs.rs will now document all features, including those that are not enabled by default.
- `OutputStreamConfig` is now public.
- `OutputStream` now prints when it is dropped, can be disabled with `OutputStream::log_on_drop(false)`.
- Update `cpal` to [0.16](https://github.com/RustAudio/cpal/blob/master/CHANGELOG.md#version-0160-2025-06-07).
- The default decoders have changed to Symphonia. The previous decoders are still available as optional features: use `claxon` for FLAC, `lewton` for Vorbis, and `hound` for WAV.
- Support for decoding MP4 containers with AAC audio is now enabled by default.

### Fixed
- `ChannelVolume` no longer clips/overflows when converting from many channels to
  fewer.
- Symphonia decoder `total_duration` incorrect value caused by conversion from `Time` to `Duration`.
- An issue with `SignalGenerator` that caused it to create increasingly distorted waveforms
  over long run times has been corrected. (#201)
- WAV and FLAC decoder duration calculation now calculated once and handles very large files
  correctly.
- Removed unwrap() calls in MP3, WAV, FLAC and Vorbis format detection for better error handling.
- `LoopedDecoder::size_hint` now correctly indicates an infinite stream.
- Symphonia decoder `total_duration` no longer returns `None` when it could
  return `Some`
- Symphonia decoder for MP4 now seeks correctly (#577).
- White noise was not correctly uniformly distributed.
- Pink noise was not correctly distributed on sampling rates other than 44100 Hz.

### Deprecated
- Deprecated `Sample::zero_value()` function in favor of `Sample::ZERO_VALUE` constant.
- Deprecated `white()` and `pink()` methods in favor of `noise::WhiteUniform::new()` and `noise::Pink::new()`.

### Removed
- Breaking: Removed `Mp4Type` enum in favor of using MIME type string "audio/mp4" for MP4 format detection with `Decoder::new_mp4` (#612).

## Version [0.20.1] - 2024-11-08

### Fixed
- Builds without the `symphonia` feature did not compile

## Version [0.20.0] - 2024-11-08

### Added
- Support for *ALAC/AIFF*
- Add `automatic_gain_control` source for dynamic audio level adjustment.
- New test signal generator sources:
    - `SignalGenerator` source generates a sine, triangle, square wave or sawtooth
      of a given frequency and sample rate.
    - `Chirp` source generates a sine wave with a linearly-increasing
      frequency over a given frequency range and duration.
    - `white` and `pink` generate white or pink noise, respectively. These
      sources depend on the `rand` crate and are guarded with the "noise"
      feature.
    - Documentation for the "noise" feature has been added to `lib.rs`.
- New Fade and Crossfade sources:
    - `fade_out` fades an input out using a linear gain fade.
    - `linear_gain_ramp` applies a linear gain change to a sound over a
      given duration. `fade_out` is implemented as a `linear_gain_ramp` and
      `fade_in` has been refactored to use the `linear_gain_ramp`
      implementation.

### Fixed
- `player.try_seek` now updates `controls.position` before returning. Calls to `player.get_pos`
  done immediately after a seek will now return the correct value.

### Changed
- `SamplesBuffer` is now `Clone`

## Version [0.19.0] - 2024-06-29

### Added
- Adds a new source `track_position`. It keeps track of duration since the
  beginning of the underlying source.

### Fixed
- Mp4a with decodable tracks after undecodable tracks now play. This matches
  VLC's behaviour.

## Version [0.18.1] - 2024-05-23

### Fixed
- Seek no longer hangs if the sink is empty.

## Version [0.18.0] - 2024-05-05

### Changed
- `Source` trait is now also implemented for `Box<dyn Source>` and `&mut Source`
- `fn new_vorbis` is now also available when the `symphonia-vorbis` feature is enabled

### Added
- Adds a new method `try_seek` to all sources. It returns either an error or
  seeks to the given position. A few sources are "unsupported" they return the
  error `Unsupported`.
- Adds `SpatialSink::clear()` bringing it in line with `Sink`

### Fixed
- channel upscaling now follows the 'WAVEFORMATEXTENSIBLE' format and no longer
  repeats the last source channel on all extra output channels.
  Stereo content playing on a 5.1 speaker set will now only use the front left
  and front right speaker instead of repeating the right sample on all speakers
  except the front left one.
- `mp3::is_mp3()` no longer changes the position in the stream when the stream
  is mp3

## Version [0.17.3] - 2023-10-23

- Build fix for `minimp3` backend.

## Version [0.17.2] - 2023-10-17

- Add `EmptyCallback` source.
- Fix index out of bounds bug.
- Use non-vulnerable `minimp3` fork.
- Add filter functions with additional q parameter.

## Version [0.17.1] - 2023-02-25

- Disable `symphonia`'s default features.

## Version [0.17.0] - 2023-02-17

- Update `cpal` to [0.15](https://github.com/RustAudio/cpal/blob/master/CHANGELOG.md#version-0150-2022-01-29).
- Default to `symphonia` for mp3 decoding.

## Version [0.16.0] - 2022-09-14

- Update `cpal` to [0.14](https://github.com/RustAudio/cpal/blob/master/CHANGELOG.md#version-0140-2022-08-22).
- Update `symphonia` to [0.5](https://github.com/pdeljanov/Symphonia/releases/tag/v0.5.1).

## Version [0.15.0] - 2022-01-23

- Remove requirement that the argument `Decoder::new` and `LoopedDecoder::new` implement `Send`.
- Add optional symphonia backend.
- `WavDecoder`'s `total_duration` now returns the total duration of the sound rather than the remaining duration.
- Add 32-bit signed in WAV decoding.
- `SineWave::new()` now takes a `f32` instead of a `u32`.
- Add `len()` method to `SpatialSink`.

## Version [0.14.0] - 2021-05-21

- Re-export `cpal` in full.
- Replace panics when calling `OutputStream::try_default`, `OutputStream::try_from_device` with new
  `StreamError` variants.
- `OutputStream::try_default` will now fallback to non-default output devices if an `OutputStream`
  cannot be created from the default device.

## Version [0.13.1] - 2021-03-28

- Fix panic when no `pulseaudio-alsa` was installed.

## Version [0.13.0] - 2020-11-03

- Update `cpal` to [0.13](https://github.com/RustAudio/cpal/blob/master/CHANGELOG.md#version-0130-2020-10-28).
- Add Android support.

## Version [0.12.0] - 2020-10-05

- Breaking: Update `cpal` to [0.12](https://github.com/RustAudio/cpal/blob/master/CHANGELOG.md#version-0120-2020-07-09).
- Breaking: Rework API removing global "rodio audio processing" thread & adapting to the upstream cpal API changes.
- Add new_X format specific methods to Decoder.
- Fix resampler dependency on internal `Vec::capacity` behaviour.

## Version [0.11.0] - 2020-03-16

- Update `lewton` to [0.10](https://github.com/RustAudio/lewton/blob/master/CHANGELOG.md#release-0100---january-30-2020).
- Breaking: Update `cpal` to [0.11](https://github.com/RustAudio/cpal/blob/master/CHANGELOG.md#version-0110-2019-12-11)

## Version [0.10.0] - 2019-11-16

- Removal of nalgebra in favour of own code.
- Fix a bug that switched channels when resuming after having paused.
- Attempt all supported output formats if the default format fails in `Sink::new`.
- Breaking: Update `cpal` to [0.10](https://github.com/RustAudio/cpal/blob/master/CHANGELOG.md#version-0100-2019-07-05).

## Version [0.9.0] - 2019-06-08

- Remove exclusive `&mut` borrow requirements in `Sink` & `SpatialSink` setters.
- Use `nalgebra` instead of `cgmath` for `Spatial` source.

## Version [0.8.1] - 2018-09-18

- Update `lewton` dependency to [0.9](https://github.com/RustAudio/lewton/blob/master/CHANGELOG.md#release-090---august-16-2018)
- Change license from `Apache-2.0` only to `Apache-2.0 OR MIT`

## Version [0.8.0] - 2018-06-22

- Add mp3 decoding capabilities via `minimp3`

## Version [0.7.0] - 2018-04-19

- Update `cpal` dependency to 0.8, and adopt the new naming convention
- BREAKING CHANGES:
    - renamed `Endpoint` to `Device`
    - split `default_endpoint()` into `default_output_device()` and `default_input_device()`
    - renamed `endpoints()` to `devices()`
    - introduced `output_devices()` and `input_devices()`
