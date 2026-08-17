## Version 0.14.1

- Readd support for caching intra costs
- Expose structs for advanced API users

## Version 0.14.0

- [Breaking/Feature] Add `SceneDetectionSpeed::None`, which will only place keyframes at fixed intervals without running dynamic detection
- Migrate detection code from rav1e into this crate

## Version 0.13.0

- [Breaking] Update ffmpeg-the-third to 3.x
- Update all other dependencies to latest version

## Version 0.12.2

- Enable threading for ffmpeg decoder, should greatly improve speed

## Version 0.12.0

- [Breaking] Move `VideoDetails` struct from `y4m` module to `decoder` module, since it is not specific to y4m
- Add support for Ffmpeg decoder (requires Cargo `ffmpeg` feature, disabled by default)

## Version 0.11.0

- Add support for Vapoursynth decoder (requires Cargo `vapoursynth` feature, disabled by default)
- Breaking change required to add a wrapper enum defining which decoder is being used

## Version 0.10.0

- Bump `rav1e` dependency to `0.7`

## Version 0.9.0

- Bump `y4m` dependency to `0.8`

## Version 0.8.1

- Finally release a new version because we can depend on rav1e 0.6.1

## Version 0.8.0

- Upgrade clap to 4.0
- Add frame limit arg to API
- [Breaking] Change `progress_callback` to take a &dyn Fn
- Misc improvements including some speedups from rav1e
- Update to Rust edition 2021

## Version 0.7.2

- Bump to the final release of rav1e 0.5
- Bump other dependencies to latest versions
- Fix another inconsistency with rav1e's scene detection
- Improve precision of FPS calculation

## Version 0.7.1

- Fix an inconsistency with how rav1e's scene detection works
- Fix some CLI help text

## Version 0.7.0

- Bump rav1e dependency to 0.5-beta.2, which brings a new, improved scenechange algorithm.
  Medium is equivalent to the old slow level, but with improvements. The fast level
  also has improvements. The new slow level is a new algorithm with a higher accuracy
  than the previous two algorithms.
- The `--fast-mode` CLI argument is removed in favor of a `--speed` or `-s` argument,
  which takes a 0, 1, or 2 (for slow, medium, or fast). The default is 0 for slow.

## Version 0.6.0

- Bump rav1e dependency to 0.5. This should bring significant performance improvements,
  but may cause breaking changes.

## Version 0.5.0

- Bump rav1e dependency to 0.4
- Expose `new_detector` and `detect_scene_changes` since these
  may be useful in some situations to use directly

## Version 0.4.2

- Fix compilation on non-x86 targets
- Bump various dependencies

## Version 0.4.1

- Improve performance and memory usage

## Version 0.4.0

- [Breaking, New Feature] `detect_scene_changes` returns a `DetectionOptions` struct,
  which includes the list of scenecut frames, and the total count
  of frames in the video. The CLI output will reflect this as well.
- [Breaking] Replace the default algorithm with an 8x8-block cost-based algorithm.
  This is more accurate in many cases.
- [Breaking] As a result of the above change, now requires nasm for compilation.
  No action is needed if you use a prebuilt binary.
- [Breaking] Replace the `use_chroma` option with a `fast_analysis` option.
  The new name is more accurate, as the updated algorithm will always analyze
  only the luma plane.
- [Breaking] Move the `progress_callback` parameter from `DetectionOptions`
  to `detect_scene_changes`, since it only applies to that interface.
- [New Feature] Expose the `SceneChangeDetector` struct, which allows
  going frame-by-frame to analyze a clip. Needed for some use cases.
  `detect_scene_changes` is the simpler, preferred interface.
- The library for inputting frame data has been replaced
  with one that matches rav1e.
- Simplify/optimize some internal code.

## Version 0.3.0

- [Breaking, New Feature] Add the ability to pass a `progress_callback` function
  to the `DetectionOptions`.

## Version 0.2.0

- [Breaking] Update `y4m` dependency to 0.5

## Version 0.1.0

- Initial release
