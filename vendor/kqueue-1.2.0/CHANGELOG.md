# Changelog

## 1.2.0

Adds:

* Bump kqueue-sys to 1.1.1
* Handle `NOTE_READ` on FreeBSD
* Better documentation of errors
* Add fd-based benchmarks

Fixes:

* Remove all panics, returning errors instead
* Do not close `File` or fd's on drop, since we do not own them
* `Vnode::Extend` has nothing to do with `truncate(2)`
* Allow removal of non-utf8 pathnames (although imperfect)
* Remove a bunch of unchecked casts and fail if data is out-of-range
* `from_error`, `is_err`, `poll`, `poll_forever` and `iter` are `#[must_use]`
* Internally use `OwnedFd` to track our `kqueue` object
* `Ident.PartialEq` did not compare enum discriminants
* Track watches in a `HashMap`, leading to much faster `remove_*` performance
* Preallocate internal vec when watching objects, reduces memory usage and
  increase perf
* Fast fail returning events from unstarted `Watcher`s
* Don't unconditionally declare ourselves as started
* Don't include drops in benchmark results

## 1.1.1

Fixes:

* Don't try and remove file descriptor 0 when removing unwatched filenames

## 1.1.0

Adds:

* Better error messages for panics
* Rust Edition 2021

Fixes:

* Don't leak file descriptors when removing filenames from being watched
* Reduce memory usage when starting a new watcher
* Don't leak file descriptors when removing fd's from being watched (see commit
  for rationale)

## 1.0.8

Adds:

* ios support

Fixes:

* panic on overflow on more than max 32bit files

## 1.0.7

Fixes:

* Fixes broken 32bit builds by matching `timespec` defs to libc

## 1.0.6

Fixes:

* marks `Vnode` enum `non_exhaustive` to fix backwards compatibility in 1.x

## 1.0.5

Adds:

* docs.rs support
* added new enum variants specific to FreeBSD (broke backwards compatibility)

Fixes:

* Fixes broken 32bit builds

## 1.0.4

Fixes:

* Fixes broken NetBSD build

## 1.0.3

Adds:

* #6: Adds a new `Watcher.poll_forever()` method which blocks on new events. This works
  around buggy behavior in the original `Watcher.poll()` method.
* !3: Adds an implementation for `std::os::unix::io::AsRawFd` for `Watcher` for
  nested kqueues.

## 1.0.2

* Fixed #4: Fix bug where wrong data types were used on i386 FreeBSD

## 1.0.1

* Merged !1 as a fix for #3. We properly fill in the `ext` field for `kqueue`
  extensions on FreeBSD.

## 1.0.0

### Breaking changes

* Bumped `bitflags` in `rust-kqueue-sys`: Now all bitflag constants must be qualified:

`EV_DELETE` -> `EventFlag::EV_DELETE`
`NOTE_WRITE` > `FilterFlag::NOT_WRITE`

### Other changes

* 2018 edition and clippy changes
