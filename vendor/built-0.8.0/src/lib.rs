// MIT License
//
// Copyright (c) Lukas Lueg <lukas.lueg@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#![allow(clippy::needless_doctest_main)]

//! Provides a crate with information from the time it was built.
//!
//! `built` is used as a build-time dependency to collect various information
//! about the build-environment, serialize this information into Rust-code and
//! provide that to the crate. The information collected by `built` include:
//!
//!  * Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
//!  * The tag or commit id if the crate was being compiled from within a Git repository.
//!  * The values of `CARGO_CFG_*` build script environment variables, like `CARGO_CFG_TARGET_OS` and `CARGO_CFG_TARGET_ARCH`.
//!  * The features the crate was compiled with.
//!  * The various dependencies, dependencies of dependencies and their versions Cargo ultimately chose to compile.
//!  * The presence of a CI-platform like `Github Actions`, `Travis CI` and `AppVeyor`.
//!  * The compiler and it's version; the documentation-generator and it's version.
//!
//! `built` does not add any further runtime-dependencies to a crate; all information
//! is serialized as types from `stdlib`. One can include `built` as a
//! runtime-dependency and use it's convenience functions.
//!
//! To add `built` to a crate, add it as a build-time dependency, use a build-script
//! to collect and serialize the build-time information, and `include!` the generated code.
//!
//! Add this to `Cargo.toml`:
//!
//! ```toml
//! [package]
//! build = "build.rs"
//!
//! [build-dependencies]
//! built = "0.7"
//! ```
//!
//! Add or modify a build-script. In `build.rs`:
//!
//! ```rust,no_run
//! fn main() {
//!     built::write_built_file().expect("Failed to acquire build-time information");
//! }
//! ```
//!
//! The build-script will by default write a file named `built.rs` into Cargo's output
//! directory. It can be picked up in `main.rs` (or anywhere else) like this:
//!
//! ```rust,ignore
//! // Use of a mod or pub mod is not actually necessary.
//! pub mod built_info {
//!    // The file has been placed there by the build script.
//!    include!(concat!(env!("OUT_DIR"), "/built.rs"));
//! }
//! ```
//!
//! ...and then used somewhere in the crate's code:
//!
//! ```rust
//! # mod built_info {
//! #    pub static PKG_VERSION_PRE: &str = "";
//! #    pub static CI_PLATFORM: Option<&str> = None;
//! #    pub static GIT_VERSION: Option<&str> = None;
//! #    pub static DEPENDENCIES: [(&str, &str); 0] = [];
//! #    pub static BUILT_TIME_UTC: &str = "Tue, 14 Feb 2017 05:21:41 GMT";
//! # }
//! #
//! # enum LogLevel { TRACE, ERROR };
//! /// Determine if current version is a pre-release or was built from a git-repo
//! fn release_is_unstable() -> bool {
//!     return !built_info::PKG_VERSION_PRE.is_empty() || built_info::GIT_VERSION.is_some()
//! }
//!
//! /// Default log-level, enhanced on CI
//! fn default_log_level() -> LogLevel {
//!     if built_info::CI_PLATFORM.is_some() {
//!         LogLevel::TRACE
//!     } else {
//!         LogLevel::ERROR
//!     }
//! }
//!
//! /// The time this crate was built
//! #[cfg(feature = "chrono")]
//! fn built_time() -> built::chrono::DateTime<built::chrono::Local> {
//!     built::util::strptime(built_info::BUILT_TIME_UTC)
//!         .with_timezone(&built::chrono::offset::Local)
//! }
//!
//! /// If another crate pulls in a dependency we don't like, print a warning
//! #[cfg(feature = "semver")]
//! fn check_sane_dependencies() {
//!     if built::util::parse_versions(&built_info::DEPENDENCIES)
//!                     .any(|(name, ver)| name == "DeleteAllMyFiles"
//!                                        && ver < built::semver::Version::parse("1.1.4").unwrap()) {
//!         eprintln!("DeleteAllMyFiles < 1.1.4 may not delete all your files. Beware!");
//!     }
//! }
//! ```
//!
//!
//! ## Overrides
//!
//! Most values otherwise detected by `built` can be manually set using environment variables. The
//! primary use-case for this is to allow automated build-systems and package-managers to enforce
//! certain values, e.g. to hide the presence of a CI-platform and/or allow for reproducible
//! builds.
//!
//! The values set via the environment take precedence over what `built` would otherwise detect.
//! There is no mechanism to ensure that values derived from override-variables are sensible,
//! besides enforcing the correct type.
//!
//! Override-variables are prefixed by `BUILT_OVERRIDE_{PKG_NAME}_`, where `{PKG_NAME}` is the name
//! of the package as reported by `cargo`. For example, if the package is named "mypkg", and the value
//! to be overridden is named "CI_PLATFORM", then the override variable is named
//! "BUILT_OVERRIDE_mypkg_CI_PLATFORM". Remember that more than one package in a dependency-graph
//! might use `built` internally, so you might have to override multiple instances of the same
//! value.
//! Unused override variables result in a warning at compile time, albeit cargo only reports those
//! for path-specific (e.g. local) packages.
//!
//! An override-variable's text-presentation must parse to the value's respective type:
//! * Strings are used as-is.
//! * Integers and `bool` must parse via their `std::str::FromStr`-implementation.
//! * `std::option::Option<T>` parses the string `BUILT_OVERRIDE_NONE` as `Option::None`; every other value
//!   must parse as `T`. For example, `BUILT_OVERRIDE_mypkg_CI_PLATFORM=BUILT_OVERRIDE_NONE` forces
//!   `CI_PLATFORM.is_none()`.
//! * List-like values are parsed as comma-separated values.
//!
//! If an override-variable can't be parsed, the build-process will abort with a `panic!()`.
//!
//! Notice that values that were overridden are recorded in `OVERRIDE_VARIABLES_USED`.
//!
//! Please refer to the respective item's documentation for more information on overrides.
//!
//! ## Feature flags
//! The information that `built` collects and makes available in `built.rs` depends
//! on the features that were enabled on the build-time dependency.
//!
//! ### _Always available_
//! The following information is available regardless of feature-flags.
//!
//! ```
//! /// The Continuous Integration platform detected during compilation.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_CI_PLATFORM`.
//! pub static CI_PLATFORM: Option<&str> = None;
//!
//! /// The full version.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_VERSION`.
//! pub static PKG_VERSION: &str = "0.1.0";
//! /// The major version.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_MAJOR`.
//! pub static PKG_VERSION_MAJOR: &str = "0";
//! /// The minor version.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_MINOR`.
//! pub static PKG_VERSION_MINOR: &str = "1";
//! /// "The patch version.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_PATCH`.
//! pub static PKG_VERSION_PATCH: &str = "0";
//! /// "The pre-release version.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_PRE`.
//! pub static PKG_VERSION_PRE: &str = "";
//!
//! /// "A colon-separated list of authors.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_AUTHORS`.
//! pub static PKG_AUTHORS: &str = "Lukas Lueg <lukas.lueg@gmail.com>";
//!
//! /// The name of the package.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_NAME`.
//! pub static PKG_NAME: &str = "example_project";
//! /// "The description.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_DESCRIPTION`.
//! pub static PKG_DESCRIPTION: &str = "";
//! /// "The homepage.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_HOMEPAGE`.
//! pub static PKG_HOMEPAGE: &str = "";
//! /// "The license.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_LICENSE`.
//! pub static PKG_LICENSE: &str = "MIT";
//! /// The source repository as advertised in Cargo.toml.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PKG_REPOSITORY`.
//! pub static PKG_REPOSITORY: &str = "";
//!
//! /// The target triple that was being compiled for.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_TARGET`.
//! pub static TARGET: &str = "x86_64-unknown-linux-gnu";
//! /// The host triple of the rust compiler.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_HOST`.
//! pub static HOST: &str = "x86_64-unknown-linux-gnu";
//! /// `release` for release builds, `debug` for other builds.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_PROFILE`.
//! pub static PROFILE: &str = "debug";
//!
//! /// The compiler that cargo resolved to use.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_RUSTC`; notice that
//! /// if `RUSTC` is overridden, `RUSTC_VERSION` *must* also be overridden.
//! pub static RUSTC: &str = "rustc";
//! /// The documentation-generator that cargo resolved to use.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_RUSTDOC`; notice that if `RUSTDOC`
//! /// is overridden, `RUSTDOC_VERSION` *must* also be overridden.
//! pub static RUSTDOC: &str = "rustdoc";
//! /// The output of `rustc -V`
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_RUSTC_VERSION`.
//! pub static RUSTC_VERSION: &str = "rustc 1.43.1 (8d69840ab 2020-05-04)";
//! /// The output of `rustdoc -V`
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_RUSTDOC_VERSION`.
//! pub static RUSTDOC_VERSION: &str = "rustdoc 1.43.1 (8d69840ab 2020-05-04)";
//!
//! /// Value of OPT_LEVEL for the profile used during compilation.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_OPT_LEVEL`.
//! pub static OPT_LEVEL: &str = "0";
//! /// The parallelism that was specified during compilation.
//! /// If `SOURCE_DATE_EPOCH` is set, `NUM_JOBS` is forced to `1`.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_NUM_JOBS`.
//! pub static NUM_JOBS: u32 = 8;
//! /// "Value of DEBUG for the profile used during compilation.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_NUM_DEBUG`.
//! pub static DEBUG: bool = true;
//!
//! /// The features that were enabled during compilation.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_FEATURES`.
//! pub static FEATURES: [&str; 0] = [];
//! /// The features as a comma-separated string.
//! pub static FEATURES_STR: &str = "";
//! /// The features as above, as lowercase strings.
//! pub static FEATURES_LOWERCASE: [&str; 0] = [];
//! /// The feature-string as above, from lowercase strings.
//! pub static FEATURES_LOWERCASE_STR: &str = "";
//!
//! /// The target architecture, given by `CARGO_CFG_TARGET_ARCH`.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_CFG_TARGET_ARCH`.
//! pub static CFG_TARGET_ARCH: &str = "x86_64";
//! /// The endianness, given by `CARGO_CFG_TARGET_ENDIAN`.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_CFG_ENDIAN`.
//! pub static CFG_ENDIAN: &str = "little";
//! /// The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_CFG_ENV`.
//! pub static CFG_ENV: &str = "gnu";
//! /// The OS-family, given by `CARGO_CFG_TARGET_FAMILY`.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_CFG_FAMILY`.
//! pub static CFG_FAMILY: &str = "unix";
//! /// The operating system, given by `CARGO_CFG_TARGET_OS`.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_CFG_OS`.
//! pub static CFG_OS: &str = "linux";
//! /// The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_CFG_POINTER_WIDTH`.
//! pub static CFG_POINTER_WIDTH: &str = "64";
//!
//! /// The override-variables that were used during compilation.
//! pub static OVERRIDE_VARIABLES_USED: [&str; 0] = [];
//! ```
//!
//! ### `cargo-lock`
//! Parses `Cargo.lock`and generates representations of  dependencies and their versions.
//!
//! For this to work, `Cargo.lock` needs to actually be there; this is (usually)
//! only true for executables and not for libraries. Cargo will only create a
//! `Cargo.lock` for the top-level crate in a dependency-tree. In case
//! of a library, the top-level crate will decide which crate/version
//! combination to compile and there will be no `Cargo.lock` while the library
//! gets compiled as a dependency.
//!
//! Parsing `Cargo.lock` instead of `Cargo.toml` allows to serialize the
//! precise versions Cargo chose to compile. One can't, however, distinguish
//! `build-dependencies`, `dev-dependencies` and `dependencies`. Furthermore,
//! some dependencies never show up if Cargo had not been forced to
//! actually use them (e.g. `dev-dependencies` with `cargo test` never
//! having been executed).
//!
//! Note that if the `dependency-tree`-feature is not active, the list of dependencies
//! contains the root-package(s) as well.
//!
//! ```
//! /// An array of effective dependencies as documented by `Cargo.lock`.
//! pub static DEPENDENCIES: [(&str, &str); 37] = [("autocfg", "1.0.0"), ("bitflags", "1.2.1"), ("built", "0.4.1"), ("cargo-lock", "4.0.1"), ("cc", "1.0.54"), ("cfg-if", "0.1.10"), ("chrono", "0.4.11"), ("example_project", "0.1.0"), ("git2", "0.13.6"), ("idna", "0.2.0"), ("jobserver", "0.1.21"), ("libc", "0.2.71"), ("libgit2-sys", "0.12.6+1.0.0"), ("libz-sys", "1.0.25"), ("log", "0.4.8"), ("matches", "0.1.8"), ("num-integer", "0.1.42"), ("num-traits", "0.2.11"), ("percent-encoding", "2.1.0"), ("pkg-config", "0.3.17"), ("proc-macro2", "1.0.17"), ("quote", "1.0.6"), ("semver", "1.0.0"), ("serde", "1.0.110"), ("serde_derive", "1.0.110"), ("smallvec", "1.4.0"), ("syn", "1.0.25"), ("time", "0.1.43"), ("toml", "0.5.6"), ("unicode-bidi", "0.3.4"), ("unicode-normalization", "0.1.12"), ("unicode-xid", "0.2.0"), ("url", "2.1.1"), ("vcpkg", "0.2.8"), ("winapi", "0.3.8"), ("winapi-i686-pc-windows-gnu", "0.4.0"), ("winapi-x86_64-pc-windows-gnu", "0.4.0")];
//! /// The effective dependencies as a comma-separated string.
//! pub static DEPENDENCIES_STR: &str = "autocfg 1.0.0, bitflags 1.2.1, built 0.4.1, cargo-lock 4.0.1, cc 1.0.54, cfg-if 0.1.10, chrono 0.4.11, example_project 0.1.0, git2 0.13.6, idna 0.2.0, jobserver 0.1.21, libc 0.2.71, libgit2-sys 0.12.6+1.0.0, libz-sys 1.0.25, log 0.4.8, matches 0.1.8, num-integer 0.1.42, num-traits 0.2.11, percent-encoding 2.1.0, pkg-config 0.3.17, proc-macro2 1.0.17, quote 1.0.6, semver 1.0.0, serde 1.0.110, serde_derive 1.0.110, smallvec 1.4.0, syn 1.0.25, time 0.1.43, toml 0.5.6, unicode-bidi 0.3.4, unicode-normalization 0.1.12, unicode-xid 0.2.0, url 2.1.1, vcpkg 0.2.8, winapi 0.3.8, winapi-i686-pc-windows-gnu 0.4.0, winapi-x86_64-pc-windows-gnu 0.4.0";
//! ```
//!
//! ### `dependency-tree` (implies `cargo-lock`)
//! Solve the dependency-graph in `Cargo.lock` to discern direct and indirect
//! dependencies.
//!
//! "Direct" dependencies are those which the root-package(s) depends on.
//! "Indirect" dependencies are those which are not direct dependencies.
//!
//! ```
//! /// An array of direct dependencies as documented by `Cargo.lock`.
//! pub static DIRECT_DEPENDENCIES: [(&str, &str); 1] = [("built", "0.6.1")];
//! /// The direct dependencies as a comma-separated string.
//! pub static DIRECT_DEPENDENCIES_STR: &str = r"built 0.6.1";
//!
//! /// An array of indirect dependencies as documented by `Cargo.lock`.
//! pub static INDIRECT_DEPENDENCIES: [(&str, &str); 64] = [("android-tzdata", "0.1.1"), ("android_system_properties", "0.1.5"), ("autocfg", "1.1.0"), ("bitflags", "2.4.0"), ("bumpalo", "3.13.0"), ("cargo-lock", "9.0.0"), ("cc", "1.0.83"), ("cfg-if", "1.0.0"), ("chrono", "0.4.29"), ("core-foundation-sys", "0.8.4"), ("equivalent", "1.0.1"), ("example_project", "0.1.0"), ("fixedbitset", "0.4.2"), ("form_urlencoded", "1.2.0"), ("git2", "0.18.0"), ("hashbrown", "0.14.0"), ("iana-time-zone", "0.1.57"), ("iana-time-zone-haiku", "0.1.2"), ("idna", "0.4.0"), ("indexmap", "2.0.0"), ("jobserver", "0.1.26"), ("js-sys", "0.3.64"), ("libc", "0.2.147"), ("libgit2-sys", "0.16.1+1.7.1"), ("libz-sys", "1.1.12"), ("log", "0.4.20"), ("memchr", "2.6.3"), ("num-traits", "0.2.16"), ("once_cell", "1.18.0"), ("percent-encoding", "2.3.0"), ("petgraph", "0.6.4"), ("pkg-config", "0.3.27"), ("proc-macro2", "1.0.66"), ("quote", "1.0.33"), ("semver", "1.0.18"), ("serde", "1.0.188"), ("serde_derive", "1.0.188"), ("serde_spanned", "0.6.3"), ("syn", "2.0.31"), ("tinyvec", "1.6.0"), ("tinyvec_macros", "0.1.1"), ("toml", "0.7.6"), ("toml_datetime", "0.6.3"), ("toml_edit", "0.19.14"), ("unicode-bidi", "0.3.13"), ("unicode-ident", "1.0.11"), ("unicode-normalization", "0.1.22"), ("url", "2.4.1"), ("vcpkg", "0.2.15"), ("wasm-bindgen", "0.2.87"), ("wasm-bindgen-backend", "0.2.87"), ("wasm-bindgen-macro", "0.2.87"), ("wasm-bindgen-macro-support", "0.2.87"), ("wasm-bindgen-shared", "0.2.87"), ("windows", "0.48.0"), ("windows-targets", "0.48.5"), ("windows_aarch64_gnullvm", "0.48.5"), ("windows_aarch64_msvc", "0.48.5"), ("windows_i686_gnu", "0.48.5"), ("windows_i686_msvc", "0.48.5"), ("windows_x86_64_gnu", "0.48.5"), ("windows_x86_64_gnullvm", "0.48.5"), ("windows_x86_64_msvc", "0.48.5"), ("winnow", "0.5.15")];
//! /// The indirect dependencies as a comma-separated string.
//! pub static INDIRECT_DEPENDENCIES_STR: &str = r"android-tzdata 0.1.1, android_system_properties 0.1.5, autocfg 1.1.0, bitflags 2.4.0, bumpalo 3.13.0, cargo-lock 9.0.0, cc 1.0.83, cfg-if 1.0.0, chrono 0.4.29, core-foundation-sys 0.8.4, equivalent 1.0.1, example_project 0.1.0, fixedbitset 0.4.2, form_urlencoded 1.2.0, git2 0.18.0, hashbrown 0.14.0, iana-time-zone 0.1.57, iana-time-zone-haiku 0.1.2, idna 0.4.0, indexmap 2.0.0, jobserver 0.1.26, js-sys 0.3.64, libc 0.2.147, libgit2-sys 0.16.1+1.7.1, libz-sys 1.1.12, log 0.4.20, memchr 2.6.3, num-traits 0.2.16, once_cell 1.18.0, percent-encoding 2.3.0, petgraph 0.6.4, pkg-config 0.3.27, proc-macro2 1.0.66, quote 1.0.33, semver 1.0.18, serde 1.0.188, serde_derive 1.0.188, serde_spanned 0.6.3, syn 2.0.31, tinyvec 1.6.0, tinyvec_macros 0.1.1, toml 0.7.6, toml_datetime 0.6.3, toml_edit 0.19.14, unicode-bidi 0.3.13, unicode-ident 1.0.11, unicode-normalization 0.1.22, url 2.4.1, vcpkg 0.2.15, wasm-bindgen 0.2.87, wasm-bindgen-backend 0.2.87, wasm-bindgen-macro 0.2.87, wasm-bindgen-macro-support 0.2.87, wasm-bindgen-shared 0.2.87, windows 0.48.0, windows-targets 0.48.5, windows_aarch64_gnullvm 0.48.5, windows_aarch64_msvc 0.48.5, windows_i686_gnu 0.48.5, windows_i686_msvc 0.48.5, windows_x86_64_gnu 0.48.5, windows_x86_64_gnullvm 0.48.5, windows_x86_64_msvc 0.48.5, winnow 0.5.15";
//! ```
//!
//! ### `git2`
//! Try to open the git-repository at `manifest_location` and retrieve `HEAD`
//! tag or commit id.
//!
//! Notice that `GIT_HEAD_REF` is `None` if `HEAD` is detached or not valid UTF-8.
//!
//! Continuous Integration platforms like `Travis` and `AppVeyor` will
//! do shallow clones, causing `libgit2` to be unable to get a meaningful
//! result. `GIT_VERSION` and `GIT_DIRTY` will therefore always be `None` if
//! a CI-platform is detected.
//! ```
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_VERSION` contains HEAD's tag. The short commit id is used
//! /// if HEAD is not tagged.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_GIT_VERSION`.
//! pub static GIT_VERSION: Option<&str> = Some("0.4.1-10-gca2af4f");
//!
//! /// If the repository had dirty/staged files.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_GIT_DIRTY`.
//! pub static GIT_DIRTY: Option<bool> = Some(true);
//!
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_HEAD_REF` contains full name to the reference pointed to by
//! /// HEAD (e.g.: `refs/heads/master`). If HEAD is detached or the branch
//! /// name is not valid UTF-8 `None` will be stored.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_GIT_HEAD_REF`.
//! pub static GIT_HEAD_REF: Option<&str> = Some("refs/heads/master");
//!
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_COMMIT_HASH` contains HEAD's full commit SHA-1 hash.
//! /// Can be overridden with `BUILT_OVERRIDE_{pkg_name}_GIT_GIT_COMMIT_HASH`.
//! pub static GIT_COMMIT_HASH: Option<&str> = Some("ca2af4f11bb8f4f6421c4cccf428bf4862573daf");
//!
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_COMMIT_HASH_SHORT` contains HEAD's short commit SHA-1 hash.
//! /// Can be overriden using `BUILT_OVERRIDE_{pkg_name}_GIT_COMMIT_HASH_SHORT`.
//! pub static GIT_COMMIT_HASH_SHORT: Option<&str> = Some("ca2af4f");
//! ```
//!
//! ### `chrono`
//!
//! The build-time is recorded as `BUILT_TIME_UTC`. If `built` is included as a runtime-dependency,
//! it can parse the string-representation into a `time:Tm` with the help of
//! `built::util::strptime()`.
//!
//! `built` honors the environment variable `SOURCE_DATE_EPOCH`. If the variable is defined and
//! parses to a valid UTC timestamp, that build-time is used instead of the current local time.
//! The variable is silently ignored if defined but but does not parse to a valid UTC timestamp.
//!
//! ```
//! /// The built-time in RFC2822, UTC
//! /// Can be overridden with`BUILT_OVERRIDE_BUILT_TIME_UTC`; the override takes precedence
//! /// over `SOURCE_DATE_EPOCH`; it *must* parse via `chrono::DateTime::parse_from_rfc2822()`.
//! pub static BUILT_TIME_UTC: &str = "Wed, 27 May 2020 18:12:39 +0000";
//! ```

#[cfg(feature = "cargo-lock")]
mod dependencies;
mod environment;
#[cfg(feature = "git2")]
mod git;
#[cfg(feature = "chrono")]
mod krono;
pub mod util;

use std::{env, fmt, fs, io, io::Write, path};

#[cfg(feature = "semver")]
pub use semver;

#[cfg(feature = "chrono")]
pub use chrono;

pub use environment::CIPlatform;

#[doc = include_str!("../README.md")]
#[allow(dead_code)]
type _READMETEST = ();

/// If `SOURCE_DATE_EPOCH` is defined, it's value is used instead of
/// `chrono::..::now()` as `BUILT_TIME_UTC`.
/// The presence of `SOURCE_DATE_EPOCH` also soft-indicates that a
/// reproducible build is desired, which we may or may not be able
/// to honor.
const SOURCE_DATE_EPOCH: &str = "SOURCE_DATE_EPOCH";

macro_rules! write_variable {
    ($writer:expr, $name:expr, $datatype:expr, $value:expr, $doc:expr) => {
        writeln!(
            $writer,
            "#[allow(clippy::needless_raw_string_hashes)]\n#[doc=r#\"{}\"#]\n#[allow(dead_code)]\npub static {}: {} = {};",
            $doc, $name, $datatype, $value
        )?;
    };
}
pub(crate) use write_variable;

macro_rules! write_str_variable {
    ($writer:expr, $name:expr, $value:expr, $doc:expr) => {
        write_variable!(
            $writer,
            $name,
            "&str",
            format_args!("\"{}\"", $value.escape_default()),
            $doc
        );
    };
}
pub(crate) use write_str_variable;

pub(crate) fn fmt_option_str<S: fmt::Display>(o: Option<S>) -> String {
    match o {
        Some(s) => format!("Some(\"{s}\")"),
        None => "None".to_owned(),
    }
}

/// Writes rust-code describing the crate at `manifest_location` to a new file named `dst`.
///
/// # Errors
/// The function returns an error if the file at `dst` already exists or can't
/// be written to. This should not be a concern if the filename points to
/// `OUR_DIR`.
pub fn write_built_file_with_opts(
    #[cfg(any(feature = "cargo-lock", feature = "git2"))] manifest_location: Option<&path::Path>,
    dst: &path::Path,
) -> io::Result<()> {
    let mut built_file = fs::File::create(dst)?;
    built_file.write_all(
        r#"//
// EVERYTHING BELOW THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
"#
        .as_ref(),
    )?;

    let envmap = environment::EnvironmentMap::new();
    envmap.write_ci(&built_file)?;
    envmap.write_env(&built_file)?;
    envmap.write_features(&built_file)?;
    envmap.write_compiler_version(&built_file)?;
    envmap.write_cfg(&built_file)?;

    #[cfg(feature = "git2")]
    {
        if let Some(manifest_location) = manifest_location {
            git::write_git_version(manifest_location, &envmap, &built_file)?;
        }
    }

    #[cfg(feature = "cargo-lock")]
    if let Some(manifest_location) = manifest_location {
        dependencies::write_dependencies(manifest_location, &built_file)?;
    }

    #[cfg(feature = "chrono")]
    krono::write_time(&built_file, &envmap)?;

    let mut used_override_vars = envmap.used_override_vars().collect::<Vec<_>>();
    used_override_vars.sort_unstable();
    write_variable!(
        built_file,
        "OVERRIDE_VARIABLES_USED",
        format_args!("[&str; {}]", used_override_vars.len()),
        util::ArrayDisplay(&used_override_vars, |t, f| write!(
            f,
            "\"{}\"",
            t.escape_default()
        )),
        "The override-variables that were used during compilation."
    );

    built_file.write_all(
        r#"//
// EVERYTHING ABOVE THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
"#
        .as_ref(),
    )?;

    let unused_override_vars = envmap.unused_override_vars().collect::<Vec<_>>().join(", ");
    if !unused_override_vars.is_empty() {
        println!("cargo::warning=At least one environment variable looks like an override-variable but was ignored by built: `{unused_override_vars}`. Typo?");
    }

    Ok(())
}

/// A shorthand for calling `write_built_file_with_opts()` with `CARGO_MANIFEST_DIR` and
/// `[OUT_DIR]/built.rs`.
///
/// # Errors
/// Same as `write_built_file_with_opts()`.
///
/// # Panics
/// If `CARGO_MANIFEST_DIR` or `OUT_DIR` are not set.
pub fn write_built_file() -> io::Result<()> {
    let dst = path::Path::new(&env::var("OUT_DIR").expect("OUT_DIR not set")).join("built.rs");
    write_built_file_with_opts(
        #[cfg(any(feature = "cargo-lock", feature = "git2"))]
        Some(
            env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR")
                .as_ref(),
        ),
        &dst,
    )?;
    Ok(())
}
