//! Provides functions for performing shell-like expansions in strings.
//!
//! In particular, the following expansions are supported:
//!
//! * tilde expansion, when `~` in the beginning of a string, like in `"~/some/path"`,
//!   is expanded into the home directory of the current user;
//! * environment expansion, when `$A` or `${B}`, like in `"~/$A/${B}something"`,
//!   are expanded into their values in some environment.
//!
//! Environment expansion also supports default values with the familiar shell syntax,
//! so for example `${UNSET_ENV:-42}` will use the specified default value, i.e. `42`, if
//! the `UNSET_ENV` variable is not set in the environment.
//!
//! The source of external information for these expansions (home directory and environment
//! variables) is called their *context*. The context is provided to these functions as a closure
//! of the respective type.
//!
//! This crate provides both customizable functions, which require their context to be provided
//! explicitly, and wrapper functions which use [`dirs::home_dir()`] and [`std::env::var()`]
//! for obtaining home directory and environment variables, respectively.
//!
//! Also there is a "full" function which performs both tilde and environment
//! expansion, but does it correctly, rather than just doing one after another: for example,
//! if the string starts with a variable whose value starts with a `~`, then this tilde
//! won't be expanded.
//!
//! All functions return [`Cow<str>`][Cow] because it is possible for their input not to contain anything
//! which triggers the expansion. In that case performing allocations can be avoided.
//!
//! Please note that by default unknown variables in environment expansion are left as they are
//! and are not, for example, substituted with an empty string:
//!
//! ```
//! fn context(_: &str) -> Option<String> { None }
//!
//! assert_eq!(
//!     shellexpand::env_with_context_no_errors("$A $B", context),
//!     "$A $B"
//! );
//! ```
//!
//! Environment expansion context allows for a very fine tweaking of how results should be handled,
//! so it is up to the user to pass a context function which does the necessary thing. For example,
//! [`env()`] and [`full()`] functions from this library pass all errors returned by [`std::env::var()`]
//! through, therefore they will also return an error if some unknown environment
//! variable is used, because [`std::env::var()`] returns an error in this case:
//!
//! ```
//! use std::env;
//!
//! // make sure that the variable indeed does not exist
//! env::remove_var("MOST_LIKELY_NONEXISTING_VAR");
//!
//! assert_eq!(
//!     shellexpand::env("$MOST_LIKELY_NONEXISTING_VAR"),
//!     Err(shellexpand::LookupError {
//!         var_name: "MOST_LIKELY_NONEXISTING_VAR".into(),
//!         cause: env::VarError::NotPresent
//!     })
//! );
//! ```
//!
//! The author thinks that this approach is more useful than just substituting an empty string
//! (like, for example, does Go with its [os.ExpandEnv](https://golang.org/pkg/os/#ExpandEnv)
//! function), but if you do need `os.ExpandEnv`-like behavior, it is fairly easy to get one:
//!
//! ```
//! use std::env;
//! use std::borrow::Cow;
//!
//! fn context(s: &str) -> Result<Option<Cow<'static, str>>, env::VarError> {
//!     match env::var(s) {
//!         Ok(value) => Ok(Some(value.into())),
//!         Err(env::VarError::NotPresent) => Ok(Some("".into())),
//!         Err(e) => Err(e)
//!     }
//! }
//!
//! // make sure that the variable indeed does not exist
//! env::remove_var("MOST_LIKELY_NONEXISTING_VAR");
//!
//! assert_eq!(
//!     shellexpand::env_with_context("a${MOST_LIKELY_NOEXISTING_VAR}b", context).unwrap(),
//!     "ab"
//! );
//! ```
//!
//! The above example also demonstrates the flexibility of context function signatures: the context
//! function may return anything which can be `AsRef`ed into a string slice.
//!
//! [Cow]: std::borrow::Cow

mod strings;
pub use self::strings::funcs::*;

#[cfg(feature = "path")]
pub mod path;

#[cfg(not(feature = "base-0"))]
compile_error!("You must enable the base-0 feature.  See the crate-level README.");
