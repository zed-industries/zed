This guide will help you update your code when upgrading from older versions of
rodio. While we did our best, we might have missed things. PRs that improve this
guide are very welcome!

The list below only contains required code changes. For a complete list of
changes and new features, see [CHANGELOG.md](CHANGELOG.md).

# rodio 0.21.1 to current GitHub version

No changes are required.

# rodio 0.20 or earlier to 0.21.1

## Features
- Playback logic has been turned into a feature that is enabled by default.
  If you have `default_features = false` in your `Cargo.toml` and want audio
  playback, you need to also set `features = ["playback"]`.
- The default decoders have changed to Symphonia, which itself is licensed
  under MPL. If you want to revert to the old decoders, you need to set
  `default_features = false` and enable the `claxon`, `hound` and `lewton`
  features in `Cargo.toml` for respectively FLAC, WAV and Ogg Vorbis.

## OutputStream
- `OutputStreamHandle` no longer exists, you can remove it from your code.
- `OutputStreamHandle::play_raw` has been removed, instead use `OutputStream.mixer().add()`.
- The output stream is now more configurable. Where you used
  `OutputStream::try_default()`, you need to change to either:
    - *(recommended)* `OutputStreamBuilder::open_default_stream()?` which tries
      to open a new output stream for the default output device with its default
      configuration. Failing that it attempt to open an output stream with
      alternative configuration and/or non default output devices. Returns the
      stream for the first configurations tried that succeeds. If all attempts
      fail returns the initial error.
    - *(org behavior)* `open_stream_or_fallback()?` which is used as follows:
      ```rust
        let default_device = cpal::default_host()
            .default_output_device()
            .ok_or("No default audio output device is found.")?;
        rodio::OutputStreamBuilder::from_device(default_device)?
            .open_stream_or_fallback()?;
      ```
      That tries to opening a output stream with the default configuration on
      the default device. Failing that attempt to open a stream with other
      available configurations supported by the default device. If all attempts
      fail returns initial error.
- The output stream now prints to stderr or logs a message on drop, if that breaks your
  CLI/UI use `stream.log_on_drop(false)`.

## Sink & SpatialSink
- Replace `Sink::try_new` with `Sink::connect_new`, which takes an `&Mixer`
  instead of a `OutputStreamHandle`. You get an `&Mixer` by calling `mixer()` on
  `OutputStream`.
- Replace `Sink::new_idle` with `Sink::new`.

### Example
The following Rodio *0.20* code:
```rust
let (_stream, handle) = rodio::OutputStream::try_default()?;
let player = rodio::Player::try_new(&handle)?;
```
Should be written like this in Rodio *0.21*:
```rust
let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
let player = rodio::Player::connect_new(stream_handle.mixer());
```

The `SpatialSink` changes mirror those in `Sink` described above.

## Decoder
- `Decoder::new_mp4` no longer takes an `Mp4Type` as hint. Remove the hint.
- The Symphonia decoders no longer assumes all sources are seekable. Use
  `DecoderBuilder::with_seekable` or `try_from` on a `File`. You do not need
  to wrap it into a `BufReader` anymore.

The following Rodio *0.20* code
```rust
let file = File::open("music.ogg")?;
let reader = BufReader::new(file);
let source = Decoder::new(reader);
```
Should be written like this in Rodio *0.21*:
```rust
let file = File::open("music.ogg")?;
let source = Decoder::try_from(file)?;
```

## DynamicMixer
- Replace `DynamicMixerController` with `Mixer` and `DynamicMixer` with
  `MixerSource`.

## Noise
- The `Source::white` and `Source::pink` methods have been deprecated. Use
  `WhiteUniform::new` and `Pink::new` instead.

## Source trait implementations
- The `Source` trait had a required method `current_frame_len`, which has been
  renamed to `current_span_len`. Rename every occurrence.
- `Source` was generic over sample types `f32`, `u16`, and `i16`. It no longer
  is; rodio now works with `f32` everywhere. This means:
    - Remove any generics (`::<f32>`, `::<u16>` or `::<i16>`) that cause errors.
    - Remove any use of `SampleConvertor`.
    - Remove any calls to `convert_samples`.
