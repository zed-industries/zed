## [1.8.1] - 2024-05-28

### Added
- Added support for `clang` 18.0.x

### Fixed
- Improve DLL search on Windows to take target architecture into account (e.g., ARM64 vs x86-64)
- Improved detection of `libclang` installed with Visual Studio on Windows

## [1.8.0] - 2024-05-26

### Changed
- Bumped minimum supported Rust version (MSRV) to 1.60.0
- Added error logging when `CLANG_PATH` set but it isn't a full path to an executable
- Removed reference to `libclang` 3.5 in error message for attempting to call an unsupported function

### Added
- Added `libcpp` Cargo feature which enables linking to `libc++` instead of `libstdc++` when linking to `libclang` statically on Linux or Haiku

### Fixed
- Fixed handling of paths that contain characters that have special meaning in
glob patterns (e.g., `[` or `]`)
- Fixed `Clang::find` to support both the `-target` and `--target` arguments
when using target-prefixed `clang` binaries

## [1.7.0] - 2023-12-31

### Added
- Added support for `clang` 17.0.x

## [1.6.1] - 2023-03-29

### Fixed
- Improved error message when calling a `libclang` function that is not supported by the loaded `libclang` instance (https://github.com/rust-lang/rust-bindgen/issues/2446)

## [1.6.0] - 2023-02-18

### Changed
- MinGW directories are not searched for `libclang` instances on Windows when
compiling for an MSVC target
- Bumped minimum supported Rust version (MSRV) to 1.51.0
- Changed Windows search directory preferences (`libclang` instances from
Visual Studio installs are now the lowest priority rather than the second
highest)

## ~~[1.5.1] - 2023-02-05~~ (YANKED)

### Changed
- MinGW directories are not searched for `libclang` instances on Windows when
compiling for an MSVC target

## ~~[1.5.0] - 2023-02-05~~ (YANKED)

### Changed
- Bumped minimum supported Rust version (MSRV) to 1.51.0
- Changed Windows search directory preferences (`libclang` instances from
Visual Studio installs are now the lowest priority rather than the second
highest)

### Added
- Added additional support for `clang` 16.0.x

## [1.4.0] - 2022-09-22

### Changed
- The value of an `EntityKind` enum variant
(`EntityKind::CXCursor_TranslationUnit`) has been updated for Clang 15.0 and
later to match the
[breaking change made in `libclang`](https://github.com/llvm/llvm-project/commit/bb83f8e70bd1d56152f02307adacd718cd67e312#diff-674613a0e47f4e66cc19061e28e3296d39be2d124dceefb68237b30b8e241e7c)

### Added
- Added support for `clang` 16.0.x
- Added support for `clang` 15.0.x
- Added support for `clang` 14.0.x

## [1.3.3] - 2022-05-28

### Fixed
- Fixed `Clang::find` to check that `CLANG_PATH` is an executable file before
selecting it

## [1.3.2] - 2022-05-18

### Added
- Added support for illumos and derivatives

## [1.3.1] - 2022-02-03

### Added
- Added missing `clang_getToken` function

## [1.3.0] - 2021-10-31

### Added
- Added support for `clang` 13.0.x
- Added support for `clang` 12.0.x
- Added support for the Haiku operating system

## [1.2.2] - 2021-09-02

### Fixed
- Fixed handling of paths that contain characters that have special meaning in
glob patterns (e.g., `[` or `]`)

## [1.2.1] - 2021-08-24

### Changed
- Updated build script to check the install location used by the
[Scoop](https://scoop.sh/) command-line installer on Windows

### Fixed
- Updated build script to support environments where the `PATH` environment
variable is not set

## [1.2.0] - 2021-04-08

### Changed
- Changed `Clang::find` to prefer target-prefixed binaries when a `-target`
argument is provided (e.g., if the arguments `-target` and
`x86_64-unknown-linux-gnu` are provided, a target-prefixed Clang executable
such as `x86_64-unknown-linux-gnu-clang` will be preferred over a non-target
prefixed Clang executable)

### Fixed
- Fixed build script to split paths in environment variables (e.g.,
`LD_LIBRARY_PATH`) using the appropriate separator for the platform (previously
`:` was used as the separator but some platforms such as Windows use `;`)

## [1.1.1] - 2021-02-19

### Changed
- Bumped `libloading` version to `0.7`

## [1.1.0] - 2021-02-09

### Changed
- Added Visual Studio LLVM component directory to search paths on Windows
([#121](https://github.com/KyleMayes/clang-sys/issues/121))

### Added
- Added support for `clang` 11.0.x

## [1.0.3] - 2020-11-19

### Fixed
- Fixed `Clang::find` panicking when `llvm-config` or `xcode-build` don't output anything to `stdout`

## [1.0.2] - 2020-11-17

### Fixed
- Fixed `Clang::find` to properly search directories returned by the
`llvm-config --bindir` and `xcodebuild -find clang` commands
- Improved version selection algorithm in the case where there are multiple
instances of `libclang` with the highest version found; previously the lowest
priority instance would be selected instead of the highest priority instance
(e.g., the versions found by searching the fallback directories were preferred
over the versions found by searching the `llvm-config --prefix` directory)

## [1.0.1] - 2020-10-01

### Changed
- Improved panic error message when calling an unloaded function

## [1.0.0] - 2020-07-14

### Changed
- Bumped `libloading` version to `0.6.0`
- Updated build script to not print warnings about failures to execute
`llvm-config` and `xcode-select` unless an instance of `libclang` is not found

### Added
- Added support for `clang` 10.0.x

### Removed
- Removed `gte_clang_*` Cargo features (these were an implementation detail)

## [0.29.3] - 2020-03-31

### Added
- Added ability to determine version of runtime-linked instance of `libclang`

## [0.29.2] - 2020-03-09

### Added
- Revert unnecessary increase of minimum version of `libc` and `libloading`

## [0.29.2] - 2020-03-09

### Added
- Revert unnecessary increase of minimum version of `libc` and `libloading`

## [0.29.1] - 2020-03-06

### Added
- Added support for finding instances of `libclang` matching `libclang-*.so.*`

## [0.29.0] - 2020-02-17

### Changed
- Wrapped function pointer fields in `Option` in the `CXCursorAndRangeVisitor`
and `IndexerCallbacks` structs (to permit nullability and to avoid undefined
behavior caused by `Default` implementations for these structs which returns a
zeroed value)

### Added
- Added support for `clang` 9.0.x
- Added missing `CXCallingConv_AArch64VectorCall` variant to `CXCallingConv` enum
- Added missing `clang_CompileCommand_getNumMappedSources` function

## [0.28.1] - 2019-07-28

### Changed
- Bumped `glob` version to `0.3.0`
- Improved error message when an invocation of an executable is not successful
- Allowed `LIBCLANG_PATH` to refer to a specific `libclang` instance (e.g.,
  `/usr/local/lib/libclang.so.10`)

### Fixed
- Fixed
  [`libclang-cpp`](https://github.com/llvm-mirror/clang/commit/90d6722bdcbc2af52306f7e948c556ad6185ac48)
  being linked instead of `libclang`

## [0.28.0] - 2019-02-17

### Changed
- Changed `llvm-config` to be first search candidate on macOS

### Added
- Added support for `clang` 8.0.x

### Removed
- Removed `assert-minimum` feature
- Removed version detection for libraries without versions embedded in the filename

## [0.27.0] - 2019-01-10

### Changed
- Added version detection for libraries without versions embedded in the filename

### Added
- Added `assert-minimum` feature (see `README.md` for details)

## [0.26.4] - 2018-12-29

### Changed
- Added shared library path to `SharedLibrary` struct

## [0.26.3] - 2018-11-14

### Changed
- Disable default features of `libc` dependency

## [0.26.2] - 2018-11-03

### Fixed
- Fixed dynamic linking on macOS

## [0.26.1] - 2018-10-10

### Fixed
- Fixed support for finding libraries in `bin` directories on Windows

## [0.26.0] - 2018-10-07

### Changed
- Added support for finding libraries with version suffixes on Linux when using runtime linking (e.g., `libclang.so.1`)

## [0.25.0] - 2018-10-06

### Changed
- Added support for versioned libraries on BSDs

## [0.24.0] - 2018-09-15

### Changed
- Reworked finding of libraries (see `README.md` for details)

### Added
- Added support for `clang` 7.0.x

## [0.23.0] - 2018-06-16

### Changed
- Changed `Clang::find` to skip dynamic libraries for an incorrect architecture on Windows

## [0.22.0] - 2018-03-11

### Added
- Added support for `clang` 6.0.x
- Bumped `libc` version to `0.2.39`
- Bumped `libloading` version to `0.5.0`

## [0.21.2] - 2018-02-17

### Changed
- Added original errors to error messages
- Added support for searching for libraries in `LD_LIBRARY_PATH` directories

## [0.21.1] - 2017-11-24

### Changed
- Improved finding of versioned libraries (e.g., `libclang-3.9.so`)

### Fixed
* Fixed compilation failures on the beta and nightly channels caused by a [compiler bug](https://github.com/KyleMayes/clang-sys/pull/69)

## [0.21.0] - 2017-10-11

### Changed
* Replaced `bitflags` usage with constants which avoids crashes on 32-bit Linux platforms

## [0.20.1] - 2017-09-16

### Fixed
- Fixed static linking

## [0.20.0] - 2017-09-14

### Added
- Added support for `clang` 5.0.x
- Added `clang` as a link target of this package
- Added dummy implementations of `is_loaded` for builds with the `static` Cargo feature enabled

## [0.19.0] - 2017-07-02

### Changed
- Bumped `bitflags` version to `0.9.1`
- Added `args` parameter to `Clang::new` function which passes arguments to the Clang executable

## [0.18.0] - 2017-05-16

### Changed
- Improved finding of versioned libraries (e.g., `libclang.so.3.9`)

## [0.17.0] - 2017-05-08

### Changed
- Changed storage type of include search paths from `Vec<PathBuf>` to `Option<Vec<PathBuf>>`

## [0.16.0] - 2017-05-02

### Changed
- Bumped `libloading` version to `0.4.0`

## [0.15.2] - 2017-04-28

### Fixed
- Fixed finding of `libclang.so.1` on Linux

## [0.15.1] - 2017-03-29

### Fixed
- Fixed static linking when libraries are in [different directories](https://github.com/KyleMayes/clang-sys/issues/50)

## [0.15.0] - 2017-03-13

### Added
- Added support for `clang` 4.0.x

### Changed
- Changed functions in the `Functions` struct to be `unsafe` (`runtime` feature only)
- Changed `Clang::find` method to ignore directories and non-executable files
- Changed `Clang::find` to skip dynamic libraries for an incorrect architecture on FreeBSD and Linux
- Bumped `bitflags` version to `0.7.0`

## [0.14.0] - 2017-01-30

### Changed
- Changed all enum types from tuple structs to raw integers to avoid
  [segmentation faults](https://github.com/rust-lang/rust/issues/39394) on some platforms

## [0.13.0] - 2017-01-29

### Changed
- Changed all opaque pointers types from tuple structs to raw pointers to avoid
  [segmentation faults](https://github.com/rust-lang/rust/issues/39394) on some platforms

## [0.12.0] - 2016-12-13

### Changed
- Altered the runtime linking API to allow for testing the presence of functions

## [0.11.1] - 2016-12-07

### Added
- Added support for linking to Clang on Windows from unofficial LLVM sources such as MSYS and MinGW

## [0.11.0] - 2016-10-07

### Changed
- Changed all enums from Rust enums to typed constants to avoid
  [undefined behavior](https://github.com/KyleMayes/clang-sys/issues/42)

## [0.10.1] - 2016-08-21

### Changed
- Changed static linking on FreeBSD and macOS to link against `libc++` instead of `libstd++`

## [0.10.0] - 2016-08-01

### Changed
- Added `runtime` Cargo feature that links to `libclang` shared library at runtime
- Added `from_raw` method to `CXTypeLayoutError` enum
- Added implementations of `Deref` for opaque FFI structs
- Changed `Default` implementations for structs to zero out the struct

## [0.9.0] - 2016-07-21

### Added
- Added documentation bindings

## [0.8.1] - 2016-07-20

### Changed
- Added `CLANG_PATH` environment variable for providing a path to `clang` executable
- Added usage of `llvm-config` to search for `clang`
- Added usage of `xcodebuild` to search for `clang` on macOS

## [0.8.0] - 2016-07-18

### Added
- Added support for `clang` 3.9.x

### Changed
- Bumped `libc` version to `0.2.14`

### Fixed
- Fixed `LIBCLANG_PATH` usage on Windows to search both the `bin` and `lib` directories
- Fixed search path parsing on macOS
- Fixed search path parsing on Windows
- Fixed default search path ordering on macOS

## [0.7.2] - 2016-06-17

### Fixed
- Fixed finding of `clang` executables when system has executables matching `clang-*`
  (e.g., `clang-format`)

## [0.7.1] - 2016-06-10

### Changed
- Bumped `libc` version to `0.2.12`

### Fixed
- Fixed finding of `clang` executables suffixed by their version (e.g., `clang-3.5`)

## [0.7.0] - 2016-05-31

### Changed
- Changed `Clang` struct `version` field type to `Option<CXVersion>`

## [0.6.0] - 2016-05-26

### Added
- Added `support` module

### Fixed
- Fixed `libclang` linking on FreeBSD
- Fixed `libclang` linking on Windows with the MSVC toolchain
- Improved `libclang` static linking

## [0.5.4] - 20160-5-19

### Changed
- Added implementations of `Default` for FFI structs

## [0.5.3] - 2016-05-17

### Changed
- Bumped `bitflags` version to `0.7.0`

## [0.5.2] - 2016-05-12

### Fixed
- Fixed `libclang` static linking

## [0.5.1] - 2016-05-10

### Fixed
- Fixed `libclang` linking on macOS
- Fixed `libclang` linking on Windows

## [0.5.0] - 2016-05-10

### Removed
- Removed `rustc_version` dependency
- Removed support for `LIBCLANG_STATIC` environment variable

### Changed
- Bumped `bitflags` version to `0.6.0`
- Bumped `libc` version to `0.2.11`
- Improved `libclang` search path
- Improved `libclang` static linking

## [0.4.2] - 2016-04-20

### Changed
- Bumped `libc` version to `0.2.10`

## [0.4.1] - 2016-04-02

### Changed
- Bumped `libc` version to `0.2.9`
- Bumped `rustc_version` version to `0.1.7`

## [0.4.0] - 2016-03-28

### Removed
- Removed support for `clang` 3.4.x

## [0.3.1] - 2016-03-21

### Added
- Added support for finding `libclang`

## [0.3.0] - 2016-03-16

### Removed
- Removed build system types and functions

### Added
- Added support for `clang` 3.4.x

### Changed
- Bumped `bitflags` version to `0.5.0`
- Bumped `libc` version to `0.2.8`

## [0.2.1] - 2016-02-13

### Changed
- Simplified internal usage of conditional compilation
- Bumped `bitflags` version to `0.4.0`
- Bumped `libc` version to `0.2.7`
- Bumped `rustc_version` version to `0.1.6`

## [0.2.0] - 2016-02-12

### Added
- Added support for `clang` 3.8.x

## [0.1.2] - 2015-12-29

### Added
- Added derivations of `Debug` for FFI structs

## [0.1.1] - 2015-12-26

### Added
- Added derivations of `PartialOrd` and `Ord` for FFI enums

## [0.1.0] - 2015-12-22
- Initial release
