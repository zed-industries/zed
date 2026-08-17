# Changelog

Newer changelog entries can be found [on
GitHub](https://github.com/console-rs/console/releases).

## 0.15.8

### Enhancements

* Added `wasm32-unknown-emscripten` target. (#179)
* `read_line_initial_text` now retains the initial prefix. (#190)
* Reading raw input now traps Ctrl+C. (#189)

### Bugfixes

* Properly use configured output of `Term` to get terminal size (#186)
* Aligned `read_line` and `read_line_initial_text`'s behavior. (#181)
* Fixed soundness issue in `msys_tty_on`. (#183)

## 0.15.7

### Enhancements

* Set an appropriate lower version of libc for macos changes.
* Improved behavior of `read_single_key` so it does not disturb other
  threads quite as much. (#165)
* More reliably reset raw mode in terminal. (#171)

## 0.15.6

### Enhancements

* Switch to `select()` on macOS for polling on TTYs to work around
  a macOS bug. (#169)
* Added blink fast and strikethrough attributes. (#159)

## 0.15.5

### Enhancements

* Removed `regex` dependency. (#153)
* Clarified that `clicolors-control` is no longer used.
* Handle non-tty terminals in `read_char`. (#124)

## 0.15.4

### Enhancements

* Fix for regression where console size was misreported on windows. (#151)

## 0.15.3

### Enhancements

* Dropped `terminal_size` dependency.

## 0.15.2

### Enhancements

* Dropped `once_cell` dependency to support MSRV again.

## 0.15.1

### Enhancements

* ANSI support no longer depends on `regex` crate.
* Crate now supports `minver`.

## 0.15.0

### Enhancements

* Added more key recognitions
* Exposed `pad_str_with` to public API
* Added `ReadWritePair`
* Support `color256` in `Style::from_dotted_str`

### BREAKING

* Added `ReadWritePair` to `TermTarget` to allow arbitrary read write pairs behave as a term
* Removed `Copy` and `PartialEq` from `TermTarget`

## 0.14.1

### Enhancements

* Added `NO_COLOR` support
* Added some more key recognitions
* Undeprecate `Term::is_term`

## 0.14.0

### Enhancements

* Added emoji support for newer Windows terminals.

### BREAKING

* Made the windows terminal emulation a non default feature (`windows-console-colors`)

## 0.13.0

### Enhancements

* Added `user_attended_stderr` for checking if stderr is a terminal
* Removed `termios` dependency

### Bug Fixes

* Better handling of key recognition on unix
* `Term::terminal_size()` on stderr terms correctly returns stderr term info

### Deprecated

* Deprecate `Term::is_term()` in favor of `Term::features().is_attended()`

### BREAKING

* Remove `Term::want_emoji()` in favor of `Term::features().wants_emoji()`
