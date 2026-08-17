# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to Semantic Versioning.

## [Unreleased]

## 0.9.1 2025-03-23
- Add support to connect to a child nvim with a handshake message, ignoring
  additional data in stdio (thanks @fredizzimo)

## 0.9.0 20250-03-01
- Mistakenly change the version to 0.9.0 instead of 0.8.0 (sorry)
- Remove dependency on `parity-tokio-ipc` (thanks @sdroege)
- Update the neovim API

## 0.7.0 2024-02-03
- Create our own Stdout for connecting to a parent neovim to avoid line buffering
- Updated API
- Updated dependencies

## 0.6.0 2023-09-16
- Updated dependencies
- Updated API
- Extended UiOptions (thanks @fredizzimo)

## 0.5.0  2022-10-08

- Updated dependencies
- Updated API (some breakage from neovim's side here)
- Improved some docs 


## 0.4.0 - 2021-12-16

### Added
- Added support for `ext_termcolors` (thanks @Lyude)

### Changed
- Updated dependencies
- Updated API (some breakage from neovim's side here)


## 0.3.0 - 2021-08-28

- Updated tokio to 1.\*
- Added UiOption::ExtMessages
- Removed create::tokio::new_unix in favor of create::tokio::new_path, which also
  works on windows
- Requests/notifications are now handled in order of arrival (which is mainly important
  for notifications)
- Removed LoopError::SpawnError


## 0.2.0 - 2020-08-29

### Added
- Connecting to neovim via tcp or a unix-socket (unix only) is now supported again

- The API has been updated to reflect neovim HEAD as of commit 161cdba.

### Changed
- The crate is now based on [`futures`](https://crates.io/crates/futures)
  rather than [`tokio`](https://crates.io/crates/tokio) to allow for different
  runtimes as far as possible. The features [`use_tokio`] or [`use_async-std`]
  can be used to get support for the 2 most popular rust runtimes, and give
  access to the `create::tokio` or `create::async_std` submodules that supply
  functionality to actually connect to neovim (depending on the features
  provided by the runtime library).

- The `Handler` trait now depends on `Clone`. The library used to `Arc`-wrap
  the handler anyways, so now the user has the possibility of using types that
  are cheaper to clone.

- `CallError` has a new variant `WrongType` to indicate that a message from
  neovim contained a value of the wrong type. Previously, the lib would panic
  in this case, now the user has the choice to handle it (or, more probably,
  log it properly and quit).

- `LoopError` has an additional variant `IoSpawn` that indicates that spawning
  another task with the handler has failed.

- The trait `FromVal` has been replaced by `TryUnpack`.

- As a substitute for directly passing a runtime around, the `Handler` now
  needs to implement `nvim-rs::create::Spawner`

- The function `new_parent` to connect to a parent neovim instance is now
  `async`.

## 0.1.0 - 2020-02-01
- Initial release
