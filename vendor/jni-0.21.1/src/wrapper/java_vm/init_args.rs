use std::{borrow::Cow, ffi::CStr, io, os::raw::c_void, ptr};

use thiserror::Error;

use crate::{
    sys::{JavaVMInitArgs, JavaVMOption},
    JNIVersion,
};

use cfg_if::cfg_if;

mod char_encoding_generic;

#[cfg(windows)]
mod char_encoding_windows;

/// Errors that can occur when invoking a [`JavaVM`](super::vm::JavaVM) with the
/// [Invocation API](https://docs.oracle.com/en/java/javase/12/docs/specs/jni/invocation.html).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JvmError {
    /// [`InitArgsBuilder::option`] or [`InitArgsBuilder::try_option`] was used, but the supplied
    /// string contains a U+0000 code point (except at the end).
    ///
    /// This error is not raised if the string has a single U+0000 code point at the end.
    ///
    /// [`InitArgsBuilder::option_encoded`] never raises this error.
    #[error("internal null in option: {0}")]
    NullOptString(String),

    /// [`InitArgsBuilder::option`] or [`InitArgsBuilder::try_option`] was used, but the option
    /// string is too long.
    ///
    /// Currently, this error only occurs on Windows, where string length is limited to 1MB to
    /// avoid overflow in [`WideCharToMultiByte`] (see [discussion]). String length is not
    /// currently limited (other than by available memory) on other platforms.
    ///
    /// [`InitArgsBuilder::option_encoded`] never raises this error, regardless of platform.
    ///
    /// [discussion]: https://github.com/jni-rs/jni-rs/pull/414
    /// [`WideCharToMultiByte`]: https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte
    #[error("option is too long: {opt_string}")]
    #[non_exhaustive]
    OptStringTooLong {
        /// The option string.
        opt_string: String,
    },

    /// [`InitArgsBuilder::option`] or [`InitArgsBuilder::try_option`] was used, but the option
    /// string is not representable in the platform default character encoding.
    ///
    /// [`InitArgsBuilder::option_encoded`] never raises this error.
    #[error(
        "option {opt_string:?} is not representable in the platform default character encoding"
    )]
    #[non_exhaustive]
    OptStringNotRepresentable {
        /// The option string.
        opt_string: String,
    },

    /// [`InitArgsBuilder::option`] or [`InitArgsBuilder::try_option`] was used, but the platform
    /// reported an error converting it to its default character encoding.
    ///
    /// [`InitArgsBuilder::option_encoded`] never raises this error.
    #[error("couldn't convert option {opt_string:?} to the platform default character encoding: {error}")]
    #[non_exhaustive]
    OptStringTranscodeFailure {
        /// The option string.
        opt_string: String,

        /// The error reported by the platform's character encoding conversion routine.
        #[source]
        error: io::Error,
    },
}

impl JvmError {
    /// Returns the JVM option that caused the error, if it was caused by one.
    pub fn opt_string(&self) -> Option<&str> {
        match self {
            Self::NullOptString(opt_string) => Some(opt_string),
            Self::OptStringTooLong { opt_string, .. } => Some(opt_string),
            Self::OptStringNotRepresentable { opt_string, .. } => Some(opt_string),
            Self::OptStringTranscodeFailure { opt_string, .. } => Some(opt_string),
        }
        .map(String::as_str)
    }

    #[cfg(all(test, windows))]
    fn opt_string_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::NullOptString(opt_string) => Some(opt_string),
            Self::OptStringTooLong { opt_string, .. } => Some(opt_string),
            Self::OptStringNotRepresentable { opt_string, .. } => Some(opt_string),
            Self::OptStringTranscodeFailure { opt_string, .. } => Some(opt_string),
        }
    }
}

const SPECIAL_OPTIONS: &[&str] = &["vfprintf", "abort", "exit"];

const SPECIAL_OPTIONS_C: &[&CStr] = unsafe {
    &[
        CStr::from_bytes_with_nul_unchecked(b"vfprintf\0"),
        CStr::from_bytes_with_nul_unchecked(b"abort\0"),
        CStr::from_bytes_with_nul_unchecked(b"exit\0"),
    ]
};

/// Builder for JavaVM InitArgs.
///
/// *This API requires "invocation" feature to be enabled,
/// see ["Launching JVM from Rust"](struct.JavaVM.html#launching-jvm-from-rust).*
#[derive(Debug)]
pub struct InitArgsBuilder<'a> {
    opts: Result<Vec<Cow<'a, CStr>>, JvmError>,
    ignore_unrecognized: bool,
    version: JNIVersion,
}

impl<'a> Default for InitArgsBuilder<'a> {
    fn default() -> Self {
        InitArgsBuilder {
            opts: Ok(vec![]),
            ignore_unrecognized: false,
            version: JNIVersion::V8,
        }
    }
}

impl<'a> InitArgsBuilder<'a> {
    /// Create a new default InitArgsBuilder
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a JVM option, such as `-Djavax.net.debug=all`.
    ///
    /// See [the JNI specification][jni-options] for details on which options are accepted.
    ///
    /// The `vfprintf`, `abort`, and `exit` options are unsupported at this time. Setting one of
    /// these options has no effect.
    ///
    /// The option must not contain any U+0000 code points except one at the end. A U+0000 code
    /// point at the end is not required, but on platforms where UTF-8 is the default character
    /// encoding, including one U+0000 code point at the end will make this method run slightly
    /// faster.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    ///
    /// * `opt_string` contains a U+0000 code point before the end.
    /// * `opt_string` cannot be represented in the platform default character encoding.
    /// * the platform's character encoding conversion API reports some other error.
    /// * `opt_string` is too long. (In the current implementation, the maximum allowed length is
    ///   1048576 bytes on Windows. There is currently no limit on other platforms.)
    ///
    /// Errors raised by this method are deferred. If an error occurs, it is returned from
    /// [`InitArgsBuilder::build`] instead.
    ///
    /// [jni-options]: https://docs.oracle.com/en/java/javase/11/docs/specs/jni/invocation.html#jni_createjavavm
    pub fn option(mut self, opt_string: impl AsRef<str> + Into<Cow<'a, str>>) -> Self {
        if let Err(error) = self.try_option(opt_string) {
            self.opts = Err(error);
        }

        self
    }

    /// Adds a JVM option, such as `-Djavax.net.debug=all`. Returns an error immediately upon
    /// failure.
    ///
    /// This is an alternative to [`InitArgsBuilder::option`] that does not defer errors. See
    /// below for details.
    ///
    /// See [the JNI specification][jni-options] for details on which options are accepted.
    ///
    /// The `vfprintf`, `abort`, and `exit` options are unsupported at this time. Setting one of
    /// these options has no effect.
    ///
    /// The option must not contain any U+0000 code points except one at the end. A U+0000 code
    /// point at the end is not required, but on platforms where UTF-8 is the default character
    /// encoding, including one U+0000 code point at the end will make this method run slightly
    /// faster.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    ///
    /// * `opt_string` contains a U+0000 code point before the end.
    /// * `opt_string` cannot be represented in the platform default character encoding.
    /// * the platform's character encoding conversion API reports some other error.
    /// * `opt_string` is too long. (In the current implementation, the maximum allowed length is
    ///   1048576 bytes on Windows. There is currently no limit on other platforms.)
    ///
    /// Unlike the `option` method, this one does not defer errors. If the `opt_string` cannot be
    /// used, then this method returns `Err` and `self` is not changed. If there is already a
    /// deferred error, however, then this method does nothing.
    ///
    /// [jni-options]: https://docs.oracle.com/en/java/javase/11/docs/specs/jni/invocation.html#jni_createjavavm
    pub fn try_option(&mut self, opt_string: impl Into<Cow<'a, str>>) -> Result<(), JvmError> {
        let opt_string = opt_string.into();

        // If there is already a deferred error, do nothing.
        let opts = match &mut self.opts {
            Ok(ok) => ok,
            Err(_) => return Ok(()),
        };

        // If the option is the empty string, then skip everything else and pass a constant empty
        // C string. This isn't just an optimization; Win32 `WideCharToMultiByte` will **fail** if
        // passed an empty string, so we have to do this check first.
        if matches!(opt_string.as_ref(), "" | "\0") {
            opts.push(Cow::Borrowed(unsafe {
                // Safety: This string not only is null-terminated without any interior null bytes,
                // it's nothing but a null terminator.
                CStr::from_bytes_with_nul_unchecked(b"\0")
            }));
            return Ok(());
        }
        // If this is one of the special options, do nothing.
        else if SPECIAL_OPTIONS.contains(&&*opt_string) {
            return Ok(());
        }

        let encoded: Cow<'a, CStr> = {
            cfg_if! {
                if #[cfg(windows)] {
                    char_encoding_windows::str_to_cstr_win32_default_codepage(opt_string)?
                }
                else {
                    // Assume UTF-8 on all other platforms.
                    char_encoding_generic::utf8_to_cstr(opt_string)?
                }
            }
        };

        opts.push(encoded);
        Ok(())
    }

    /// Adds a JVM option, such as `-Djavax.net.debug=all`. The option must be a `CStr` encoded in
    /// the platform default character encoding.
    ///
    /// This is an alternative to [`InitArgsBuilder::option`] that does not do any encoding. This
    /// method is not `unsafe` as it cannot cause undefined behavior, but the option will be
    /// garbled (that is, become [mojibake](https://en.wikipedia.org/wiki/Mojibake)) if not
    /// encoded correctly.
    ///
    /// See [the JNI specification][jni-options] for details on which options are accepted.
    ///
    /// The `vfprintf`, `abort`, and `exit` options are unsupported at this time. Setting one of
    /// these options has no effect.
    ///
    /// This method does not fail, and will neither return nor defer an error.
    ///
    /// [jni-options]: https://docs.oracle.com/en/java/javase/11/docs/specs/jni/invocation.html#jni_createjavavm
    pub fn option_encoded(mut self, opt_string: impl Into<Cow<'a, CStr>>) -> Self {
        let opt_string = opt_string.into();

        // If there is already a deferred error, do nothing.
        let opts = match &mut self.opts {
            Ok(ok) => ok,
            Err(_) => return self,
        };

        // If this is one of the special options, do nothing.
        if SPECIAL_OPTIONS_C.contains(&&*opt_string) {
            return self;
        }

        // Add the option.
        opts.push(opt_string);

        self
    }

    /// Set JNI version for the init args
    ///
    /// Default: V8
    pub fn version(self, version: JNIVersion) -> Self {
        let mut s = self;
        s.version = version;
        s
    }

    /// Set the `ignoreUnrecognized` init arg flag
    ///
    /// If ignoreUnrecognized is true, JavaVM::new ignores all unrecognized option strings that
    /// begin with "-X" or "_". If ignoreUnrecognized is false, JavaVM::new returns Err as soon as
    /// it encounters any unrecognized option strings.
    ///
    /// Default: `false`
    pub fn ignore_unrecognized(self, ignore: bool) -> Self {
        let mut s = self;
        s.ignore_unrecognized = ignore;
        s
    }

    /// Build the `InitArgs`
    ///
    /// # Errors
    ///
    /// If a call to [`InitArgsBuilder::option`] caused a deferred error, it is returned from this
    /// method.
    pub fn build(self) -> Result<InitArgs<'a>, JvmError> {
        let opt_strings = self.opts?;

        let opts: Vec<JavaVMOption> = opt_strings
            .iter()
            .map(|opt_string| JavaVMOption {
                optionString: opt_string.as_ptr() as _,
                extraInfo: ptr::null_mut(),
            })
            .collect();

        Ok(InitArgs {
            inner: JavaVMInitArgs {
                version: self.version.into(),
                ignoreUnrecognized: self.ignore_unrecognized as _,
                options: opts.as_ptr() as _,
                nOptions: opts.len() as _,
            },
            _opts: opts,
            _opt_strings: opt_strings,
        })
    }

    /// Returns collected options.
    ///
    /// If a call to [`InitArgsBuilder::option`] caused a deferred error, then this method returns
    /// a reference to that error.
    pub fn options(&self) -> Result<&[Cow<'a, CStr>], &JvmError> {
        self.opts.as_ref().map(Vec::as_slice)
    }
}

/// JavaVM InitArgs.
///
/// *This API requires "invocation" feature to be enabled,
/// see ["Launching JVM from Rust"](struct.JavaVM.html#launching-jvm-from-rust).*
pub struct InitArgs<'a> {
    inner: JavaVMInitArgs,

    // `JavaVMOption` structures are stored here. The JVM accesses this `Vec`'s contents through a
    // raw pointer.
    _opts: Vec<JavaVMOption>,

    // Option strings are stored here. This ensures that any that are owned aren't dropped before
    // the JVM is finished with them.
    _opt_strings: Vec<Cow<'a, CStr>>,
}

impl<'a> InitArgs<'a> {
    pub(crate) fn inner_ptr(&self) -> *mut c_void {
        &self.inner as *const _ as _
    }
}
