# [Docs.rs](https://docs.rs/safe_arch)

# safe_arch

Exposes arch-specific intrinsics as safe function.

* SIMD types are newtype'd (with a `pub` field) and given appropriate trait
  impls such as `From`, `Into`, `Default`, etc.
* Each intrinsic gets either a function or macro so that you can safely use it
  as directly as possible.
  * Functions are used when all arguments are runtime arguments.
  * Macros are used when one of the arguments must be a compile time constant,
    because Rust doesn't let you "pass through" compile time constants.
* There's hundreds and hundreds of intrinsics, so the names of functions and
  macros tend to be very long and specific because there's often many similar
  ways to do nearly the same thing.
  * This crate isn't really intended for "everyday users". It is intended to be
    an "unopinionated" middle layer crate that just provides the safety. Higher
    level abstractions should mostly come from some other crate that wraps over
    this crate.

All function and macro availability is done purely at compile time via
`#[cfg()]` attributes on the various modules. If a CPU feature isn't enabled for
the build then those functions or macros won't be available. If you'd like to
determine what CPU features are available at runtime and then call different
code accordingly, this crate is not for you.

See the [crate docs](https://docs.rs/safe_arch) for more details.

## Additional Resources

* [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/)
  * [Raw Xml v3.5.2](https://software.intel.com/sites/landingpage/IntrinsicsGuide/files/data-3.5.2.xml) and you can check their [release notes](https://software.intel.com/sites/landingpage/IntrinsicsGuide/files/ReleaseNotes.html) to see if a later version has been put out since this readme file was last updated.
