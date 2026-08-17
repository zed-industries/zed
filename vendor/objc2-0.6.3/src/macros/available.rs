/// Check if APIs from the given operating system versions are available.
///
/// Apple adds new APIs with new OS releases, and as a developer, you often
/// want to use those to give your users the best behaviour, while still
/// supporting older OS versions that don't have those APIs (instead of
/// crashing e.g. because of an undefined selector).
///
/// This macro allows you to conditionally execute code depending on if the
/// current OS version is higher than or equal to the version given in the
/// macro.
///
/// If no version is specified for a certain OS, the API will be assumed to be
/// unavailable there. This default can be changed by adding a trailing `..`
/// to the macro invocation.
///
/// This is very similar to `@available` in Objective-C and `#available` in
/// Swift, see [Apple's documentation][apple-doc]. Another great introduction
/// to availability can be found in [here][epir-availability].
///
/// [apple-doc]: https://developer.apple.com/documentation/xcode/running-code-on-a-specific-version#Require-a-minimum-operating-system-version-for-a-feature
/// [epir-availability]: https://epir.at/2019/10/30/api-availability-and-target-conditionals/
///
///
/// # Operating systems
///
/// The operating system names this macro accepts, the standard environment
/// variables that you use to raise the deployment target (the minimum
/// supported OS version) and the current default versions are all summarized
/// in the table below.
///
/// | OS Value   | Name                    | Environment Variable         | Default |
/// | ---------- | ----------------------- | ---------------------------- | ------- |
/// | `ios`      | iOS/iPadOS/Mac Catalyst | `IPHONEOS_DEPLOYMENT_TARGET` | 10.0    |
/// | `macos`    | macOS                   | `MACOSX_DEPLOYMENT_TARGET`   | 10.12   |
/// | `tvos`     | tvOS                    | `TVOS_DEPLOYMENT_TARGET`     | 10.0    |
/// | `visionos` | visionOS                | `XROS_DEPLOYMENT_TARGET`     | 1.0     |
/// | `watchos`  | watchOS                 | `WATCHOS_DEPLOYMENT_TARGET`  | 5.0     |
///
/// The default version is the same as that of `rustc` itself.
///
///
/// # Optimizations
///
/// This macro will statically be set to `true` when the deployment target is
/// high enough.
///
/// If a runtime check is deemed necessary, the version lookup will be cached.
///
///
/// # Alternatives
///
/// Instead of checking the version at runtime, you could do one of the
/// following instead:
///
/// 1. Check statically that you're compiling for a version where the API is
///    available, e.g. by checking the `*_DEPLOYMENT_TARGET` variables in a
///    build script or at `const` time.
///
/// 2. Check at runtime that a class, method or symbol is available, using
///    e.g. [`AnyClass::get`], [`respondsToSelector`] or [weak linking].
///
/// [`AnyClass::get`]: crate::runtime::AnyClass::get
/// [`respondsToSelector`]: crate::runtime::NSObjectProtocol::respondsToSelector
/// [weak linking]: https://github.com/rust-lang/rust/issues/29603
///
///
/// # Examples
///
/// Use the [`effectiveAppearance`] API that was added in macOS 10.14.
///
/// ```
/// # #[cfg(available_in_frameworks)]
/// use objc2_app_kit::{NSApplication, NSAppearance, NSAppearanceNameAqua};
/// use objc2::available;
///
/// let appearance = if available!(macos = 10.14) {
///     // Dark mode and `effectiveAppearance` was added in macOS 10.14.
///     # #[cfg(available_in_frameworks)]
///     NSApplication::sharedApplication(mtm).effectiveAppearance()
/// } else {
///     // Fall back to `NSAppearanceNameAqua` on macOS 10.13 and below.
///     # #[cfg(available_in_frameworks)]
///     NSAppearance::appearanceNamed(NSAppearanceNameAqua).unwrap()
/// };
/// ```
///
/// Use an API added in Xcode 16.0 SDKs.
///
/// We use `..` here in case Apple adds a new operating system in the future,
/// then we probably also want the branch to be taken there.
///
/// ```
/// use objc2::available;
///
/// if available!(ios = 18.0, macos = 15.0, tvos = 18.0, visionos = 2.0, watchos = 11.0, ..) {
///     // Use some recent API here.
/// }
/// ```
///
/// Set the [`wantsExtendedDynamicRangeContent`] property, which is available
/// since iOS 16.0, macOS 10.11 and visionOS 1.0, but is not available on tvOS
/// and watchOS.
///
/// ```
/// use objc2::available;
///
/// if available!(ios = 16.0, macos = 10.11, visionos = 1.0) {
///     # #[cfg(available_in_frameworks)]
///     layer.setWantsExtendedDynamicRangeContent(true);
/// }
/// ```
///
/// Work around problems in a specific range of versions (an example of this
/// in the real world can be seen in [#662]).
///
/// ```
/// use objc2::available;
///
/// if available!(macos = 15.0) && !available!(macos = 15.1) {
///     // Do something on macOS 15.0 and 15.0.1.
/// } else {
///     // Do something else on all other versions.
/// }
/// ```
///
/// [`effectiveAppearance`]: https://developer.apple.com/documentation/appkit/nsapplication/2967171-effectiveappearance?language=objc
/// [`wantsExtendedDynamicRangeContent`]: https://developer.apple.com/documentation/quartzcore/cametallayer/1478161-wantsextendeddynamicrangecontent
/// [#662]: https://github.com/madsmtm/objc2/issues/662
#[doc(alias = "@available")] // Objective-C
#[doc(alias = "#available")] // Swift
#[macro_export]
macro_rules! available {
    (
        // Returns `false` on unspecified platforms.
        $(
            $os:ident $(= $major:literal $(. $minor:literal $(. $patch:literal)?)?)?
        ),* $(,)?
    ) => {
        $crate::__macro_helpers::is_available({
            // TODO: Use inline const once in MSRV
            #[allow(clippy::needless_update)]
            const VERSION: $crate::__macro_helpers::AvailableVersion = $crate::__macro_helpers::AvailableVersion {
                $(
                    // Doesn't actually parse versions this way, but is
                    // helpful to write it like this for documentation.
                    //
                    // We use optionality for the version here, to allow
                    // rust-analyzer to work with partially filled macros.
                    $os: $($crate::__available_version!($major $(. $minor $(. $patch)?)?))?,
                )*
                // A version this high will never be lower than the deployment
                // target, and hence will always return `false` from
                // `is_available`.
                .. $crate::__macro_helpers::AvailableVersion::MAX
            };
            VERSION
        })
    };
    (
        // Returns `true` on unspecified platforms because of the trailing `..`.
        $(
            $os:ident $(= $major:literal $(. $minor:literal $(. $patch:literal)?)?)?,
        )*
        ..
    ) => {
        $crate::__macro_helpers::is_available({
            #[allow(clippy::needless_update)]
            const VERSION: $crate::__macro_helpers::AvailableVersion = $crate::__macro_helpers::AvailableVersion {
                $(
                    $os: $($crate::__available_version!($major $(. $minor $(. $patch)?)?))?,
                )*
                // A version of 0.0.0 will always be lower than the deployment
                // target, and hence will always return `true` from
                // `is_available`.
                //
                // We do this when `..` is specified.
                .. $crate::__macro_helpers::AvailableVersion::MIN
            };
            VERSION
        })
    };
}

/// Both `tt` and `literal` matches either `$major` as an integer, or
/// `$major.$minor` as a float.
///
/// As such, we cannot just take `$major:tt . $minor:tt . $patch:tt` and
/// convert that to `OSVersion` directly, we must convert it to a string
/// first, and then parse that.
///
/// We also _have_ to do string parsing, floating point parsing wouldn't be
/// enough (because e.g. `10.10` would result in the float `10.1` and parse
/// wrongly).
///
/// Note that we intentionally `stringify!` before passing to `concat!`, as
/// that seems to properly preserve all zeros in the literal.
#[doc(hidden)]
#[macro_export]
macro_rules! __available_version {
    // Just in case rustc's parsing changes in the future, let's handle this
    // generically, instead of trying to split each part into separate `tt`.
    ($($version_part_or_period:tt)*) => {
        $crate::__macro_helpers::OSVersion::from_str($crate::__macro_helpers::concat!($(
            $crate::__macro_helpers::stringify!($version_part_or_period),
        )*))
    };
}
