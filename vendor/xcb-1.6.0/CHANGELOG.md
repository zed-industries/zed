# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] -  - various authors

## [1.6.0] - 2025-08-15 - various authors
### Added
 - export well-known RandR output property names, this time for real

### Fixed
 - fixing further unalignemnt issues (#277)
 - hand-writen `damage::ReportLevel` enum to handle the DamageNotifyMore bit (#280)
 - fix unsound API `xcb::Connection::connect_to_fd*` (#283)

## [1.5.0] - 2024-11-09 - various authors
### Added
 - `UnknownError` and no more panic for failed resolution (#266)
 - unsafe public API to resolve FFI events and errors (#266)
 - `Reply::as_raw` (#267)
 - `GetModifierMappingReply::keycodes_per_modifier` is public (#271)

### Fixed
 - Compilation warnings (#275)

## [1.4.0] - 2024-04-23 - various authors
### Added
 - `BaseEvent::is_from_send_event` (#262)
 - feature `libxcb_v1_14` (enabled by default) (#250)

### Fixed
 - Special event API (#261)
 - DisplayInfo members visibility (#263)

## [1.3.0] - 2023-12-09 - various authors
### Added
 - add `Connection::poll_for_reply` and `Connection::poll_for_reply_unchecked` (#245)

### Fixed
 - fix more alignment panic (#246, #247)

## [1.2.2] - 2023-07-22 - various authors
### Fixed
 - fix serialization panic (impl WiredOut #226, #227)
 - fix `Send` and `Sync` for events (#233)

## [1.2.1] - 2023-04-23 - various authors
### Fixed
 - updated `quick-xml` as the old one will no longer be compatible with future rust versions
 - fixed alignment issue

### Changed
 - Implement `AsRawXcbConnection` for `Connection`

## [1.2.0] - 2022-11-03 - various authors
### Fixed
 - Segfault when DISPLAY is unset when using xlib
 - Return `Unknown` instead of panic for unknown events (disabled feature) (#195)

### Changed
 - `resolve_event` returns `Event::Unknown` instead of panicking when the event could not be resolved.
 - Types such as `x::Cw` implement `Ord` and can be sorted dynamically (i.e. for `x::CreateWindow`)
 - `atoms_struct!` macro can specify `only_if_exists`
 - add `Event::as_raw()`, `Connection::resolve_event()`

## [1.1.1] - 2022-03-19 - rtbo
### Fixed
 - Padding offset in events constructors ([x11-clipboard/#26](https://github.com/quininer/x11-clipboard/pull/26))

## [1.1.0] - 2022-03-16 - rtbo
### Added
- Xid types (`x::Atom`, `x::Window`, ...) implement `Hash`
- `atoms_struct!` macro
- `Connection::send_and_check_request`

### Changed
- Some documentation improvements.

## [1.0.0] - 2022-03-06 - rtbo
- new 1.0 API
  - implement the extensions in RUST directly (no need to link to C extension libraries)
  - XCB resolves protocol events and errors to safe enumerations
  - requests are structures that can be passed to `Connection::send_request`
  - typesafe cookies for checked/unchecked and void/non-void requests
  - typesafe XIDs
  - `bitflags` masks
  - `debug_atom_names` feature
  - improvements of the `Debug` implementations
  - many, many other improvement and fixes, (~90% of github issues are resolved)
- support for xcb-1.14
- support for xinput extension
- get rid of the `log` dependency

## [0.10.1] - 2021-09-23 - rtbo
- fix some code generation affecting the `present` extension
- fix compilation warnings about uninhabited type values

## [0.10.0] - 2021-09-19 - rtbo
### Changed
- build script is written in Rust. Python no longer needed. (#56, #62, #89, #99)
- moved CI to github actions
### Fixed
- fixed some member names (e.g. `size_i_d` => `size_id` / `num__f_b_configs` => `num_fb_configs`)

## [0.9.0] - 2019-11-12 - Lompik
### Fixed
- get_reply consume cookies and impl Drop on Cookies (#57)

## [0.8.3] - 2019-11-12 - Lompik
### Fixed
- fix use after free when connecting with display names (#65)

## [0.8.2] - 2018-02-13 - chrisduerr/myfreeweb/yamnikov-oleg/eigebong/rtbo
- move to log 0.4 (#55)
- improve missing python error in build.rs (#49)
- add Connection.into_raw_conn
- make source generation deterministic (#43)

## [0.8.1] - 2017-08-15 - /rtbo/main--/chrisduerr
- fix lifetime inconsistency (#40)
- impl AsRawFd for Connection (#39)

## [0.8.0] - 2017-07-11 - mjkillough/eduardosm/rtbo
- error trait and unsafe cast_error (#32) - mjkillough
- unsafe cast_event - rtbo
- allow xcb::connect without xlib_xcb feature
(fixes also doc generation) (#35) - eduardosm

## [0.7.8] - 2019/11/12 - Lompik
- fix use after free when connecting with display names (#65) (backporting from 0.8)

 ## [0.7.7] - 2017-08-15 - rtbo/mjkillough
- branch 0.7.x to support servo
- implement Error/Display for GenericError and ConnError
- fix lifetime inconsistencies (#40)
- Send and Sync implemented regardless of thread feature

## [0.7.6] - 2016-11-14 - rtbo/ibabushkin
- much better handling of union accessors (#27) Credits to Inokentiy Babushkin
- other minor fixes

## [0.7.5] - 2016-08 - rtbo
- multi-threading support (#23)
- other bug fixes

## [0.7.4] - 2016-06 - rtbo
- templating send_event* to take event obj instead of str
- correct iterator attribute lifetime (#16)

## [0.7.3] - 2016-04-10 - rtbo
- templating some accessors

## [0.7.2] - 2016-04-02 - rtbo
- fix #14

## [0.7.1] - 2016-03-29 - rtbo
- module names closer to ffi
- fix #13

## [0.7.0] - 2016-03-28 - rtbo
- fix connection with strings (#9)
- assign response_type in *Event::new (#10)
- Connection::connect returns Result (#11)
- Some documentation (#12)

## [0.6.2] - 2016-03-04 - rtbo
- fix: correct names for DRI2 and 3 FFI constants

## [0.6.1] - 2016-03-02 - rtbo
- fix: correct names for 'xtest' extension

## [0.6.0] - 2016-02-22 - rtbo
- xlib_xcb: Connection owns the xlib::Display and calls XCloseDisplay
- requests accept template slices
- POD types distinction

## [0.5.0] - 2016-02-07 - rtbo
- adding xlib_xcb
- show how to create an opengl enabled window

## [0.4.1] - 2016-02-07 - rtbo
- generating union accessors
- handling of bool parameters in the wrapper API
- rewrite of wrappers structures (pub type instead of struct with base field)
- module clean-up and export
- Travis CI

## [0.4.0] - 2016-03-02 - rtbo/laumann
- first fully functional wrappers
- rewritten rs_client.py
- new examples
- made ffi very close to C
- fixed wrappers segfaults

 ## [0.3.0] - 2013 - Aatch
