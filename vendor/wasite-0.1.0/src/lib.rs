//! Abstraction over Wasite

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/wasite"
)]
#![forbid(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

use std::{
    env::{self, VarError},
    io::{self, Error, ErrorKind, Result, Write},
    num::NonZeroU16,
    str::FromStr,
};

fn parse<T: FromStr>(name: &str) -> Result<T>
where
    T::Err: Send + Sync + std::error::Error + 'static,
{
    env::var(name)
        .map_err(|e| {
            let kind = match e {
                VarError::NotPresent => ErrorKind::NotFound,
                VarError::NotUnicode(_) => ErrorKind::InvalidData,
            };
            Error::new(kind, e)
        })?
        .parse()
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
}

/// Get terminal width in columns
pub fn width() -> Result<NonZeroU16> {
    parse("COLUMNS")
}

/// Get terminal height in lines
pub fn height() -> Result<NonZeroU16> {
    parse("LINES")
}

/// Get the cursor's column number
pub fn column() -> Result<NonZeroU16> {
    parse("COLUMN")
}

/// Get the cursor's line number
pub fn line() -> Result<NonZeroU16> {
    parse("LINE")
}

/// Get the current screen number
pub fn screen() -> Result<u8> {
    parse("SCREEN")
}

/// Set the current screen number
pub fn set_screen(value: u8) {
    env::set_var("SCREEN", value.to_string())
}

/// Get the current window title
pub fn title() -> Result<String> {
    parse("TITLE")
}

/// Set the current window title
pub fn set_title(value: impl Into<String>) {
    env::set_var("TITLE", value.into())
}

/// Get the current user's username
pub fn user() -> Result<String> {
    parse("USER")
}

/// Set the current user's username
pub fn set_user(value: impl Into<String>) {
    env::set_var("USER", value.into())
}

/// Get the device's hostname
pub fn hostname() -> Result<String> {
    parse("HOSTNAME")
}

/// Set the device's hostname
pub fn set_hostname(value: impl Into<String>) {
    env::set_var("HOSTNAME", value.into())
}

/// Get the device's pretty/display name
pub fn name() -> Result<String> {
    parse("NAME")
}

/// Set the device's pretty/display name
pub fn set_name(value: impl Into<String>) {
    env::set_var("NAME", value.into())
}

/// Get the current timezone (using the IANA TZDB identifier)
pub fn tz() -> Result<String> {
    parse("TZ")
}

/// Set the current timezone (using the IANA TZDB identifier)
pub fn set_tz(value: impl Into<String>) {
    env::set_var("TZ", value.into())
}

/// Get the user's preferred languages, separated by semicolons
pub fn langs() -> Result<String> {
    parse("LANGS")
}

/// Set the user's preferred languages, separated by semicolons
pub fn set_langs(value: impl Into<String>) {
    env::set_var("LANGS", value.into())
}

/// Return true if raw mode is enabled
pub fn raw() -> Result<bool> {
    match parse("RAW")? {
        0u8 => Ok(false),
        1u8 => Ok(true),
        e => Err(Error::new(ErrorKind::InvalidInput, e.to_string())),
    }
}

/// Turn raw mode on or off
pub fn set_raw(value: bool) {
    let int: u8 = value.into();

    env::set_var("RAW", int.to_string())
}

/// Terminal commands
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    /// Clear the screen
    Clear,
    /// Flash the screen
    Alert,
}

/// Execute terminal commands
pub fn execute(commands: &[Command]) -> Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for command in commands {
        match command {
            Command::Clear => stdout.write_all(b"\x00")?,
            Command::Alert => stdout.write_all(b"\x07")?,
        }
    }

    stdout.flush()
}
