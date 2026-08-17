CHANGELOG
=========

## 0.15.0

- Support `CSI ? 5 W` to reset tabs stops to every 8th column
- Replaced `no_std` with a new `std` feature
- Changed default features to include `std`

## 0.14.1

- Crash when partial advance buffer stopped inside some grapheme boundaries

## 0.14.0

- `Parser::advance` now takes byte slices, instead of individual bytes
- `Parser::advance_until_terminated` allows premature termination,
    by checking for `Perform::terminated` after each dispatch

## 0.13.1

- Add SCP control support
- Improve SGR performance

## 0.13.0

- Reexport `cursor_icon` crate in `ansi`
- Split-out private modes from `Mode` into `PrivateMode`
- Add `unset_private_mode` and `set_private_mode`
- Add `report_mode` and `report_private_mode` to handle DECRPM/DECRQM

## 0.12.0

- Add support for OSC 22
- Add support for kitty keyboard protocol
- Add support for XTerm's modifyOtherKeys protocol

## 0.11.1

- Minimum rust version has been bumped to 1.62.1
- Support for ANSI terminal stream parsing under the `ansi` feature.
- Addition of the `serde` feature which derives `Serialize` and `Deserialize`
  for the types provided in the `ansi` module.

## 0.11.0

- Minimum rust version has been bumped to 1.56.0
- Fixed infinite loop in `Params` iterator when 32nd parameter is a subparameter

## 0.10.1

- Fixed invalid intermediates when transitioning from DCS to ESC

## 0.10.0

- Changed the type of CSI parameters from i64 to u16
- All methods of the `Perform` trait are now optional

## 0.9.0

- Added CSI subparameter support; required changes can be seen in Alacritty:
    https://github.com/alacritty/alacritty/commit/576252294d09c1f52ec73bde03652349bdf5a529#diff-49ac9e6f6e6a855312bfcd393201f18ca53e6148c4a22a3a4949f1f9d1d137a8

## 0.8.0

- Remove C1 ST support in OSCs, fixing OSCs with ST in the payload

## 0.7.1

- Out of bounds when parsing a DCS with more than 16 parameters

## 0.7.0

- Fix params reset between escapes
- Removed unused parameter from `esc_dispatch`

## 0.6.0

- Fix build failure on Rust 1.36.0
- Add `bool_terminated` parameter to osc dispatch

## 0.5.0

- Support for dynamically sized escape buffers without feature `no_std`
- Improved UTF8 parser performance
- Migrate to Rust 2018

## 0.4.0

- Fix handling of DCS escapes

## 0.3.3

- Fix off-by-one error in CSI parsing when params list was at max length
  (previously caused a panic).
- Support no_std

## 0.2.0

- Removes `osc_start`, `osc_put`, and `osc_end`
- Adds `osc_dispatch` which simply receives a list of parameters
- Removes `byte: u8` parameter from `hook` and `unhook` because it's always
  zero.
