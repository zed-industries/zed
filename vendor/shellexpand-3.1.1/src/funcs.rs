//! Implementation - **instantiated twice**
//!
//! **IMPORTANT NOTE TO IMPLEMENTORS**
//!
//! This module is included twice - there are two `mod` statements.
//! The `use super::wtraits::*` line imports *different types* each type.
//!
//! This allows the same code to do double duty: it works with `str`, and also with `Path`.
//! Working with `Path` is quite awkward and complicated - see `path.rs` for the type definitions.
//!
//! The `wtraits` module has all the type names and traits we use,
//! along with documentation of their semantics.
//!
//! But we also allow the use of inherent methods
//! (if they do the right things with both the string and path types).

use std::env::VarError;
use std::error::Error;

use super::wtraits::*;

#[cfg(test)]
mod test;

/// Performs both tilde and environment expansion using the provided contexts.
///
/// `home_dir` and `context` are contexts for tilde expansion and environment expansion,
/// respectively. See [`env_with_context()`] and [`tilde_with_context()`] for more details on
/// them.
///
/// Unfortunately, expanding both `~` and `$VAR`s at the same time is not that simple. First,
/// this function has to track ownership of the data. Since all functions in this crate
/// return [`Cow<str>`], this function takes some precautions in order not to allocate more than
/// necessary. In particular, if the input string contains neither tilde nor `$`-vars, this
/// function will perform no allocations.
///
/// Second, if the input string starts with a variable, and the value of this variable starts
/// with tilde, the naive approach may result into expansion of this tilde. This function
/// avoids this.
///
/// # Examples
///
/// ```
/// use std::path::{PathBuf, Path};
/// use std::borrow::Cow;
///
/// fn home_dir() -> Option<String> { Some("/home/user".into()) }
///
/// fn get_env(name: &str) -> Result<Option<&'static str>, &'static str> {
///     match name {
///         "A" => Ok(Some("a value")),
///         "B" => Ok(Some("b value")),
///         "T" => Ok(Some("~")),
///         "E" => Err("some error"),
///         _ => Ok(None)
///     }
/// }
///
/// // Performs both tilde and environment expansions
/// assert_eq!(
///     shellexpand::full_with_context("~/$A/$B", home_dir, get_env).unwrap(),
///     "/home/user/a value/b value"
/// );
///
/// // Errors from environment expansion are propagated to the result
/// assert_eq!(
///     shellexpand::full_with_context("~/$E/something", home_dir, get_env),
///     Err(shellexpand::LookupError {
///         var_name: "E".into(),
///         cause: "some error"
///     })
/// );
///
/// // Input without starting tilde and without variables does not cause allocations
/// let s = shellexpand::full_with_context("some/path", home_dir, get_env);
/// match s {
///     Ok(Cow::Borrowed(s)) => assert_eq!(s, "some/path"),
///     _ => unreachable!("the above variant is always valid")
/// }
///
/// // Input with a tilde inside a variable in the beginning of the string does not cause tilde
/// // expansion
/// assert_eq!(
///     shellexpand::full_with_context("$T/$A/$B", home_dir, get_env).unwrap(),
///     "~/a value/b value"
/// );
/// ```
pub fn full_with_context<SI: ?Sized, CO, C, E, P, HD>(
    input: &SI,
    home_dir: HD,
    context: C,
) -> Result<Cow<Xstr>, LookupError<E>>
where
    SI: AsRef<Xstr>,
    CO: AsRef<Xstr>,
    C: FnMut(&str) -> Result<Option<CO>, E>,
    P: AsRef<Xstr>,
    HD: FnOnce() -> Option<P>,
{
    env_with_context(input, context).map(|r| match r {
        // variable expansion did not modify the original string, so we can apply tilde expansion
        // directly
        Cow::Borrowed(s) => tilde_with_context(s, home_dir),
        Cow::Owned(s) => {
            // if the original string does not start with a tilde but the processed one does,
            // then the tilde is contained in one of variables and should not be expanded
            // (We must convert the input to WInput here because it might be `AsRef<Path>`.
            // and `Path`'s `starts_with` checks only whole components;
            // and `OsStr` doesn't let us test prefixes at all.)
            if !input.into_winput().starts_with('~') && s.starts_with("~") {
                // return as is
                s.into()
            } else if let Cow::Owned(s) = tilde_with_context(&s, home_dir) {
                s.into()
            } else {
                s.into()
            }
        }
    })
}

/// Same as [`full_with_context()`], but forbids the variable lookup function to return errors.
///
/// This function also performs full shell-like expansion, but it uses
/// [`env_with_context_no_errors()`] for environment expansion whose context lookup function returns
/// just [`Option<CO>`] instead of [`Result<Option<CO>, E>`]. Therefore, the function itself also
/// returns just [`Cow<str>`] instead of [`Result<Cow<str>, LookupError<E>>`]. Otherwise it is
/// identical to [`full_with_context()`].
///
/// # Examples
///
/// ```
/// use std::path::{PathBuf, Path};
/// use std::borrow::Cow;
///
/// fn home_dir() -> Option<String> { Some("/home/user".into()) }
///
/// fn get_env(name: &str) -> Option<&'static str> {
///     match name {
///         "A" => Some("a value"),
///         "B" => Some("b value"),
///         "T" => Some("~"),
///         _ => None
///     }
/// }
///
/// // Performs both tilde and environment expansions
/// assert_eq!(
///     shellexpand::full_with_context_no_errors("~/$A/$B", home_dir, get_env),
///     "/home/user/a value/b value"
/// );
///
/// // Input without starting tilde and without variables does not cause allocations
/// let s = shellexpand::full_with_context_no_errors("some/path", home_dir, get_env);
/// match s {
///     Cow::Borrowed(s) => assert_eq!(s, "some/path"),
///     _ => unreachable!("the above variant is always valid")
/// }
///
/// // Input with a tilde inside a variable in the beginning of the string does not cause tilde
/// // expansion
/// assert_eq!(
///     shellexpand::full_with_context_no_errors("$T/$A/$B", home_dir, get_env),
///     "~/a value/b value"
/// );
/// ```
#[inline]
pub fn full_with_context_no_errors<SI: ?Sized, CO, C, P, HD>(
    input: &SI,
    home_dir: HD,
    mut context: C,
) -> Cow<Xstr>
where
    SI: AsRef<Xstr>,
    CO: AsRef<Xstr>,
    C: FnMut(&str) -> Option<CO>,
    P: AsRef<Xstr>,
    HD: FnOnce() -> Option<P>,
{
    match full_with_context(input, home_dir, move |s| Ok::<Option<CO>, ()>(context(s))) {
        Ok(result) => result,
        Err(_) => unreachable!(),
    }
}

/// Performs both tilde and environment expansions in the default system context.
///
/// This function delegates to [`full_with_context()`], using the default system sources for both
/// home directory and environment, namely [`dirs::home_dir()`] and [`std::env::var()`].
///
/// Note that variable lookup of unknown variables will fail with an error instead of, for example,
/// replacing the unknown variable with an empty string. The author thinks that this behavior is
/// more useful than the other ones. If you need to change it, use [`full_with_context()`] or
/// [`full_with_context_no_errors()`] with an appropriate context function instead.
///
/// This function behaves exactly like [`full_with_context()`] in regard to tilde-containing
/// variables in the beginning of the input string.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// env::set_var("A", "a value");
/// env::set_var("B", "b value");
///
/// let home_dir = dirs::home_dir()
///     .map(|p| p.display().to_string())
///     .unwrap_or_else(|| "~".to_owned());
///
/// // Performs both tilde and environment expansions using the system contexts
/// assert_eq!(
///     shellexpand::full("~/$A/${B}s").unwrap(),
///     format!("{}/a value/b values", home_dir)
/// );
///
/// // Unknown variables cause expansion errors
/// assert_eq!(
///     shellexpand::full("~/$UNKNOWN/$B"),
///     Err(shellexpand::LookupError {
///         var_name: "UNKNOWN".into(),
///         cause: env::VarError::NotPresent
///     })
/// );
/// ```
#[cfg(feature = "tilde")]
#[inline]
pub fn full<SI: ?Sized>(input: &SI) -> Result<Cow<Xstr>, LookupError<VarError>>
where
    SI: AsRef<Xstr>,
{
    full_with_context(input, home_dir, |s| std::env::var(s).map(Some))
}

#[cfg(feature = "tilde")]
fn home_dir() -> Option<XString> {
    let hd = dirs::home_dir()?;

    // If the home directory is not valid Unicode, we might not be able to substitute it.
    // We don't have an error reporting channel suitable for this (very unusual) situation.
    // If it happens, we just return `None`, causing `~`-substitution to not occur.
    //
    // In `shellexpand` 2.x, we use `Path::display()`, instead, which is lossy - so we would
    // use a wrong pathname.  That is definitely worse.
    hd.try_into_xstring()
}

/// Represents a variable lookup error.
///
/// This error is returned by [`env_with_context()`] function (and, therefore, also by [`env()`],
/// [`full_with_context()`] and [`full()`]) when the provided context function returns an error. The
/// original error is provided in the `cause` field, while `name` contains the name of a variable
/// whose expansion caused the error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupError<E> {
    /// The name of the problematic variable inside the input string.
    pub var_name: OString,
    /// The original error returned by the context function.
    pub cause: E,
}

impl<E: fmt::Display> fmt::Display for LookupError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error looking key '")?;
        self.var_name.display_possibly_lossy(f)?;
        write!(f, "' up: {}", self.cause)?;
        Ok(())
    }
}

impl<E: Error + 'static> Error for LookupError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

macro_rules! try_lookup {
    ($name:expr, $e:expr) => {
        match $e {
            Ok(s) => s,
            Err(e) => {
                return Err(LookupError {
                    var_name: $name.to_ostring(),
                    cause: e,
                })
            }
        }
    };
}

fn is_valid_var_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Performs the environment expansion using the provided context.
///
/// This function walks through the input string `input` and attempts to construct a new string by
/// replacing all shell-like variable sequences with the corresponding values obtained via the
/// `context` function. The latter may return an error; in this case the error will be returned
/// immediately, along with the name of the offending variable. Also the context function may
/// return `Ok(None)`, indicating that the given variable is not available; in this case the
/// variable sequence is left as it is in the output string.
///
/// The syntax of variables resembles the one of bash-like shells: all of `$VAR`, `${VAR}`,
/// `$NAME_WITH_UNDERSCORES` are valid variable references, and the form with braces may be used to
/// separate the reference from the surrounding alphanumeric text: `before${VAR}after`. Note,
/// however, that for simplicity names like `$123` or `$1AB` are also valid, as opposed to shells
/// where `$<number>` has special meaning of positional arguments. Also note that "alphanumericity"
/// of variable names is checked with [`std::primitive::char::is_alphanumeric()`], therefore lots of characters which
/// are considered alphanumeric by the Unicode standard are also valid names for variables. When
/// unsure, use braces to separate variables from the surrounding text.
///
/// This function has four generic type parameters: `SI` represents the input string, `CO` is the
/// output of context lookups, `C` is the context closure and `E` is the type of errors returned by
/// the context function. `SI` and `CO` must be types, a references to which can be converted to
/// a string slice. For example, it is fine for the context function to return [`&str`]'s, [`String`]'s or
/// [`Cow<str>`]'s, which gives the user a lot of flexibility.
///
/// If the context function returns an error, it will be wrapped into [`LookupError`] and returned
/// immediately. [`LookupError`], besides the original error, also contains a string with the name of
/// the variable whose expansion caused the error. [`LookupError`] implements [`Error`], [`Clone`] and
/// [`Eq`] traits for further convenience and interoperability.
///
/// If you need to expand system environment variables, you can use [`env()`] or [`full()`] functions.
/// If your context does not have errors, you may use [`env_with_context_no_errors()`] instead of
/// this function because it provides a simpler API.
///
/// # Examples
///
/// ```
/// fn context(s: &str) -> Result<Option<&'static str>, &'static str> {
///     match s {
///         "A" => Ok(Some("a value")),
///         "B" => Ok(Some("b value")),
///         "E" => Err("something went wrong"),
///         _ => Ok(None)
///     }
/// }
///
/// // Regular variables are expanded
/// assert_eq!(
///     shellexpand::env_with_context("begin/$A/${B}s/end", context).unwrap(),
///     "begin/a value/b values/end"
/// );
///
/// // Expand to a default value if the variable is not defined
/// assert_eq!(
///     shellexpand::env_with_context("begin/${UNSET_ENV:-42}/end", context).unwrap(),
///     "begin/42/end"
/// );
///
/// // Unknown variables are left as is
/// assert_eq!(
///     shellexpand::env_with_context("begin/$UNKNOWN/end", context).unwrap(),
///     "begin/$UNKNOWN/end"
/// );
///
/// // Errors are propagated
/// assert_eq!(
///     shellexpand::env_with_context("begin${E}end", context),
///     Err(shellexpand::LookupError {
///         var_name: "E".into(),
///         cause: "something went wrong"
///     })
/// );
/// ```
pub fn env_with_context<SI: ?Sized, CO, C, E>(
    input: &SI,
    mut context: C,
) -> Result<Cow<Xstr>, LookupError<E>>
where
    SI: AsRef<Xstr>,
    CO: AsRef<Xstr>,
    C: FnMut(&str) -> Result<Option<CO>, E>,
{
    let input_str = input.into_winput();
    if let Some(idx) = input_str.find('$') {
        let mut result = OString::with_capacity(input_str.len());

        let mut input_str = input_str.as_wstr();
        let mut next_dollar_idx = idx;
        loop {
            result.push_wstr(&input_str[..next_dollar_idx]);

            input_str = &input_str[next_dollar_idx..];
            if input_str.is_empty() {
                break;
            }

            fn find_dollar(s: &Wstr) -> usize {
                s.find('$').unwrap_or(s.len())
            }
            let mut lookup = |var_name: &Wstr| {
                let var_name = match var_name.as_str() {
                    Some(var_name) => var_name,
                    // No non-UTF-8 variables can exist
                    None => return Ok(None),
                };
                context(var_name)
            };

            let mut next_chars = input_str[1..].chars_approx();
            let next_char = next_chars.next();
            if next_char == Some('{') {
                match input_str.find('}') {
                    Some(closing_brace_idx) => {
                        let mut default_value = None;

                        // Search for the default split
                        let var_name_end_idx = match input_str[..closing_brace_idx].find(":-") {
                            // Only match if there's a variable name, ie. this is not valid ${:-value}
                            Some(default_split_idx) if default_split_idx != 2 => {
                                default_value =
                                    Some(&input_str[default_split_idx + 2..closing_brace_idx]);
                                default_split_idx
                            }
                            _ => closing_brace_idx,
                        };

                        let var_name = &input_str[2..var_name_end_idx];
                        match lookup(var_name) {
                            // if we have the variable set to some value
                            Ok(Some(var_value)) => {
                                result.push_xstr(var_value.as_ref());
                                input_str = &input_str[closing_brace_idx + 1..];
                                next_dollar_idx = find_dollar(input_str);
                            }

                            // if the variable is set and empty or unset
                            not_found_or_empty => {
                                let value = match (not_found_or_empty, default_value) {
                                    // return an error if we don't have a default and the variable is unset
                                    (Err(err), None) => {
                                        return Err(LookupError {
                                            var_name: var_name.to_ostring(),
                                            cause: err,
                                        });
                                    }
                                    // use the default value if set
                                    (_, Some(default)) => default,
                                    // leave the variable as it is if the environment is empty
                                    (_, None) => &input_str[..closing_brace_idx + 1],
                                };

                                result.push_wstr(value);
                                input_str = &input_str[closing_brace_idx + 1..];
                                next_dollar_idx = find_dollar(input_str);
                            }
                        }
                    }
                    // unbalanced braces
                    None => {
                        result.push_wstr(&input_str[..2]);
                        input_str = &input_str[2..];
                        next_dollar_idx = find_dollar(input_str);
                    }
                }
            } else if next_char.map(is_valid_var_name_char) == Some(true) {
                let mut end_idx;
                loop {
                    // Subtract the bytes length of the remainder from the length, and that's where we are
                    end_idx = input_str.len() - next_chars.wstr_len();
                    match next_chars.next() {
                        Some(c) if is_valid_var_name_char(c) => {},
                        _ => break,
                    }
                }

                let var_name = &input_str[1..end_idx];
                match try_lookup!(var_name, lookup(var_name)) {
                    Some(var_value) => {
                        result.push_xstr(var_value.as_ref());
                        input_str = &input_str[end_idx..];
                        next_dollar_idx = find_dollar(input_str);
                    }
                    None => {
                        result.push_wstr(&input_str[..end_idx]);
                        input_str = &input_str[end_idx..];
                        next_dollar_idx = find_dollar(input_str);
                    }
                }
            } else {
                result.push_str("$");
                input_str = if next_char == Some('$') {
                    &input_str[2..] // skip the next dollar for escaping
                } else {
                    &input_str[1..]
                };
                next_dollar_idx = find_dollar(input_str);
            };
        }
        Ok(result.into_ocow())
    } else {
        Ok(input.into_ocow())
    }
}

/// Same as [`env_with_context()`], but forbids the variable lookup function to return errors.
///
/// This function also performs environment expansion, but it requires context function of type
/// `FnMut(&str) -> Option<CO>` instead of `FnMut(&str) -> Result<Option<CO>, E>`. This simplifies
/// the API when you know in advance that the context lookups may not fail.
///
/// Because of the above, instead of [`Result<Cow<str>, LookupError<E>>`] this function returns just
/// [`Cow<str>`].
///
/// Note that if the context function returns [`None`], the behavior remains the same as that of
/// [`env_with_context()`]: the variable reference will remain in the output string unexpanded.
///
/// # Examples
///
/// ```
/// fn context(s: &str) -> Option<&'static str> {
///     match s {
///         "A" => Some("a value"),
///         "B" => Some("b value"),
///         _ => None
///     }
/// }
///
/// // Known variables are expanded
/// assert_eq!(
///     shellexpand::env_with_context_no_errors("begin/$A/${B}s/end", context),
///     "begin/a value/b values/end"
/// );
///
/// // Unknown variables are left as is
/// assert_eq!(
///     shellexpand::env_with_context_no_errors("begin/$U/end", context),
///     "begin/$U/end"
/// );
/// ```
#[inline]
pub fn env_with_context_no_errors<SI: ?Sized, CO, C>(input: &SI, mut context: C) -> Cow<Xstr>
where
    SI: AsRef<Xstr>,
    CO: AsRef<Xstr>,
    C: FnMut(&str) -> Option<CO>,
{
    match env_with_context(input, move |s| Ok::<Option<CO>, ()>(context(s))) {
        Ok(value) => value,
        Err(_) => unreachable!(),
    }
}

/// Performs the environment expansion using the default system context.
///
/// This function delegates to [`env_with_context()`], using the default system source for
/// environment variables, namely the [`std::env::var()`] function.
///
/// Note that variable lookup of unknown variables will fail with an error instead of, for example,
/// replacing the offending variables with an empty string. The author thinks that such behavior is
/// more useful than the other ones. If you need something else, use [`env_with_context()`] or
/// [`env_with_context_no_errors()`] with an appropriate context function.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // make sure that some environment variables are set
/// env::set_var("X", "x value");
/// env::set_var("Y", "y value");
///
/// // Known variables are expanded
/// assert_eq!(
///     shellexpand::env("begin/$X/${Y}s/end").unwrap(),
///     "begin/x value/y values/end"
/// );
///
/// // Unknown variables result in an error
/// assert_eq!(
///     shellexpand::env("begin/$Z/end"),
///     Err(shellexpand::LookupError {
///         var_name: "Z".into(),
///         cause: env::VarError::NotPresent
///     })
/// );
/// ```
#[inline]
pub fn env<SI: ?Sized>(input: &SI) -> Result<Cow<Xstr>, LookupError<VarError>>
where
    SI: AsRef<Xstr>,
{
    env_with_context(input, |s| std::env::var(s).map(Some))
}

/// Performs the tilde expansion using the provided context.
///
/// This function expands tilde (`~`) character in the beginning of the input string into contents
/// of the path returned by `home_dir` function. If the input string does not contain a tilde, or
/// if it is not followed either by a slash (`/`) or by the end of string, then it is also left as
/// is. This means, in particular, that expansions like `~anotheruser/directory` are not supported.
/// The context function may also return a `None`, in that case even if the tilde is present in the
/// input in the correct place, it won't be replaced (there is nothing to replace it with, after
/// all).
///
/// This function has three generic type parameters: `SI` represents the input string, `P` is the
/// output of a context lookup, and `HD` is the context closure. `SI` must be a type, a reference
/// to which can be converted to a string slice via [`AsRef<str>`], and `P` must be a type, a
/// reference to which can be converted to a `str` via [`AsRef<str>`].
/// Home directories which are available only as a `Path` are not supported here,
/// because they cannot be represented in the output string.
/// If you wish to support home directories which are not valid Unicode,
/// use the [`path`](crate::path) module.
///
/// If you need to expand the tilde into the actual user home directory, you can use [`tilde()`] or
/// [`full()`] functions.
///
/// # Examples
///
/// ```
/// use std::path::{PathBuf, Path};
///
/// fn home_dir() -> Option<String> { Some("/home/user".into()) }
///
/// assert_eq!(
///    shellexpand::tilde_with_context("~/some/dir", home_dir),
///    "/home/user/some/dir"
/// );
/// ```
pub fn tilde_with_context<SI: ?Sized, P, HD>(input: &SI, home_dir: HD) -> Cow<Xstr>
where
    SI: AsRef<Xstr>,
    P: AsRef<Xstr>,
    HD: FnOnce() -> Option<P>,
{
    let input_str = input.into_winput();
    if let Some(input_after_tilde) = input_str.strip_prefix('~') {
        if input_after_tilde.is_empty()
            || input_after_tilde.starts_with('/')
            || (cfg!(windows) && input_after_tilde.starts_with('\\'))
        {
            if let Some(hd) = home_dir() {
                let hd = hd.into_winput();
                let mut result = OString::with_capacity(hd.len() + input_after_tilde.len());
                result.push_wstr(hd.as_wstr());
                result.push_wstr(input_after_tilde);
                result.into_ocow()
            } else {
                // home dir is not available
                input.into_ocow()
            }
        } else {
            // we cannot handle `~otheruser/` paths yet
            input.into_ocow()
        }
    } else {
        // input doesn't start with tilde
        input.into_ocow()
    }
}

/// Performs the tilde expansion using the default system context.
///
/// This function delegates to [`tilde_with_context()`], using the default system source of home
/// directory path, namely [`dirs::home_dir()`] function.
///
/// # Examples
///
/// ```
/// let hds = dirs::home_dir()
///     .map(|p| p.display().to_string())
///     .unwrap_or_else(|| "~".to_owned());
///
/// assert_eq!(
///     shellexpand::tilde("~/some/dir"),
///     format!("{}/some/dir", hds)
/// );
/// ```
#[cfg(feature = "tilde")]
#[inline]
pub fn tilde<SI: ?Sized>(input: &SI) -> Cow<Xstr>
where
    SI: AsRef<Xstr>,
{
    tilde_with_context(input, home_dir)
}
