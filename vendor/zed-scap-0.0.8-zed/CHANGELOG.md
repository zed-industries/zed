# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.8](https://github.com/CapSoftware/scap/compare/v0.0.7...v0.0.8) - 2024-12-10

### Other

- rewind version
- Update mod.rs
- Bump version (backwards compatible bug fix)
- Change as_raw_nopadding_buffer to as_nopadding_buffer
- Make GraphicsCaptureApiHandler's fn `new` implementation for Capturer have the correct signature
- handle macos stream errors with error flag
- expose CMSampleBuffer

## [0.0.7](https://github.com/CapSoftware/scap/compare/v0.0.6...v0.0.7) - 2024-11-07

### Features

- Adds `RawCapturer::get_next_pixel_buffer`, a macOS-specific method to get the next frame as its raw pixel buffer, allowing allocations from pixel format conversion to be avoided.

### Fixed

- `windows` crate versions updated (#120)

## [0.0.6](https://github.com/CapSoftware/scap/compare/v0.0.5...v0.0.6) - 2024-11-05

### Added

- adds correct crop_area
- get_crop_area for specific targets
- adds scale_factor support for windows and displays on mac
- get_main_display func improved
- add unique identifier to unknown displays on mac
- adds correct name of displays on macos
- make scale_factor f64
- exclude windows without title
- adds windows as targets on mac
- restructure util functions and add display name windows

### Fixed

- Revert to DrawBorderSettings::Default on Windows
- modified get_crop_area to include scale_factor for windows
- minor change for scale factor
- use cg types from sckit_sys
- output frame size target
- windows tweaks
- macos imports after restructure

### Other

- backwards compatability + enum error
- vendor apple-sys bindings
- Merge pull request [#95](https://github.com/CapSoftware/scap/pull/95) from MAlba124/main
- Make STREAM_STATE_CHANGED_TO_ERROR reset on stop_capture
- Fix restart on pipewire capturer
- Merge pull request [#89](https://github.com/CapSoftware/scap/pull/89) from MAlba124/main
- update .all-contributorsrc
- update README.md
- cleanup deps and remove cgtype in favor of area
- update readme and add todo for windows
- Merge branch 'feat/solo-target' into feat/use-targets-mac
- Merge branch 'feat/solo-target' into feat/mac-targets-scale-factor
- Merge pull request [#81](https://github.com/CapSoftware/scap/pull/81) from helmerapp/feat/windows-improvements
- Merge branch 'main' into feat/windows-targets
- Merge branch 'feat/windows-targets' of https://github.com/helmerapp/scap into feat/windows-targets
- extract pixelformat conversions to different file
- source rect simplifier
- shorten width, height
- windows engine
- tweak example app
- updates readme

## [0.0.5](https://github.com/helmerapp/scap/compare/v0.0.4...v0.0.5) - 2024-05-25

### Other
- don't build before releasing
- remove CHANGELOG
