pub mod arc_cow;
pub mod command;
pub mod fs;
pub mod markdown;
pub mod paths;
pub mod serde;
pub mod size;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
pub mod time;

use anyhow::Result;
use futures::Future;
use itertools::Either;
use regex::Regex;
use std::sync::{LazyLock, OnceLock};
use std::{
    borrow::Cow,
    cmp::{self, Ordering},
    env,
    ops::{AddAssign, Range, RangeInclusive},
    panic::Location,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use unicase::UniCase;

#[cfg(unix)]
use anyhow::{Context as _, anyhow};

pub use take_until::*;
#[cfg(any(test, feature = "test-support"))]
pub use util_macros::{line_endings, separator, uri};

#[macro_export]
macro_rules! debug_panic {
    ( $($fmt_arg:tt)* ) => {
        if cfg!(debug_assertions) {
            panic!( $($fmt_arg)* );
        } else {
            let backtrace = std::backtrace::Backtrace::capture();
            log::error!("{}\n{:?}", format_args!($($fmt_arg)*), backtrace);
        }
    };
}

/// A macro to add "C:" to the beginning of a path literal on Windows, and replace all
/// the separator from `/` to `\`.
/// But on non-Windows platforms, it will return the path literal as is.
///
/// # Examples
/// ```rust
/// use util::path;
///
/// let path = path!("/Users/user/file.txt");
/// #[cfg(target_os = "windows")]
/// assert_eq!(path, "C:\\Users\\user\\file.txt");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(path, "/Users/user/file.txt");
/// ```
#[cfg(all(any(test, feature = "test-support"), target_os = "windows"))]
#[macro_export]
macro_rules! path {
    ($path:literal) => {
        concat!("C:", util::separator!($path))
    };
}

/// A macro to add "C:" to the beginning of a path literal on Windows, and replace all
/// the separator from `/` to `\`.
/// But on non-Windows platforms, it will return the path literal as is.
///
/// # Examples
/// ```rust
/// use util::path;
///
/// let path = path!("/Users/user/file.txt");
/// #[cfg(target_os = "windows")]
/// assert_eq!(path, "C:\\Users\\user\\file.txt");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(path, "/Users/user/file.txt");
/// ```
#[cfg(all(any(test, feature = "test-support"), not(target_os = "windows")))]
#[macro_export]
macro_rules! path {
    ($path:literal) => {
        $path
    };
}

pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

/// Removes characters from the end of the string if its length is greater than `max_chars` and
/// appends "..." to the string. Returns string unchanged if its length is smaller than max_chars.
pub fn truncate_and_trailoff(s: &str, max_chars: usize) -> String {
    debug_assert!(max_chars >= 5);

    // If the string's byte length is <= max_chars, walking the string can be skipped since the
    // number of chars is <= the number of bytes.
    if s.len() <= max_chars {
        return s.to_string();
    }
    let truncation_ix = s.char_indices().map(|(i, _)| i).nth(max_chars);
    match truncation_ix {
        Some(index) => s[..index].to_string() + "…",
        _ => s.to_string(),
    }
}

/// Removes characters from the front of the string if its length is greater than `max_chars` and
/// prepends the string with "...". Returns string unchanged if its length is smaller than max_chars.
pub fn truncate_and_remove_front(s: &str, max_chars: usize) -> String {
    debug_assert!(max_chars >= 5);

    // If the string's byte length is <= max_chars, walking the string can be skipped since the
    // number of chars is <= the number of bytes.
    if s.len() <= max_chars {
        return s.to_string();
    }
    let suffix_char_length = max_chars.saturating_sub(1);
    let truncation_ix = s
        .char_indices()
        .map(|(i, _)| i)
        .nth_back(suffix_char_length);
    match truncation_ix {
        Some(index) if index > 0 => "…".to_string() + &s[index..],
        _ => s.to_string(),
    }
}

/// Takes only `max_lines` from the string and, if there were more than `max_lines-1`, appends a
/// a newline and "..." to the string, so that `max_lines` are returned.
/// Returns string unchanged if its length is smaller than max_lines.
pub fn truncate_lines_and_trailoff(s: &str, max_lines: usize) -> String {
    let mut lines = s.lines().take(max_lines).collect::<Vec<_>>();
    if lines.len() > max_lines - 1 {
        lines.pop();
        lines.join("\n") + "\n…"
    } else {
        lines.join("\n")
    }
}

/// Truncates the string at a character boundary, such that the result is less than `max_bytes` in
/// length.
pub fn truncate_to_byte_limit(s: &str, max_bytes: usize) -> &str {
    if s.len() < max_bytes {
        return s;
    }

    for i in (0..max_bytes).rev() {
        if s.is_char_boundary(i) {
            return &s[..i];
        }
    }

    ""
}

/// Takes a prefix of complete lines which fit within the byte limit. If the first line is longer
/// than the limit, truncates at a character boundary.
pub fn truncate_lines_to_byte_limit(s: &str, max_bytes: usize) -> &str {
    if s.len() < max_bytes {
        return s;
    }

    for i in (0..max_bytes).rev() {
        if s.is_char_boundary(i) {
            if s.as_bytes()[i] == b'\n' {
                // Since the i-th character is \n, valid to slice at i + 1.
                return &s[..i + 1];
            }
        }
    }

    truncate_to_byte_limit(s, max_bytes)
}

#[test]
fn test_truncate_lines_to_byte_limit() {
    let text = "Line 1\nLine 2\nLine 3\nLine 4";

    // Limit that includes all lines
    assert_eq!(truncate_lines_to_byte_limit(text, 100), text);

    // Exactly the first line
    assert_eq!(truncate_lines_to_byte_limit(text, 7), "Line 1\n");

    // Limit between lines
    assert_eq!(truncate_lines_to_byte_limit(text, 13), "Line 1\n");
    assert_eq!(truncate_lines_to_byte_limit(text, 20), "Line 1\nLine 2\n");

    // Limit before first newline
    assert_eq!(truncate_lines_to_byte_limit(text, 6), "Line ");

    // Test with non-ASCII characters
    let text_utf8 = "Line 1\nLíne 2\nLine 3";
    assert_eq!(
        truncate_lines_to_byte_limit(text_utf8, 15),
        "Line 1\nLíne 2\n"
    );
}

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

/// Extend a sorted vector with a sorted sequence of items, maintaining the vector's sort order and
/// enforcing a maximum length. This also de-duplicates items. Sort the items according to the given callback. Before calling this,
/// both `vec` and `new_items` should already be sorted according to the `cmp` comparator.
pub fn extend_sorted<T, I, F>(vec: &mut Vec<T>, new_items: I, limit: usize, mut cmp: F)
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut start_index = 0;
    for new_item in new_items {
        if let Err(i) = vec[start_index..].binary_search_by(|m| cmp(m, &new_item)) {
            let index = start_index + i;
            if vec.len() < limit {
                vec.insert(index, new_item);
            } else if index < vec.len() {
                vec.pop();
                vec.insert(index, new_item);
            }
            start_index = index;
        }
    }
}

pub fn truncate_to_bottom_n_sorted_by<T, F>(items: &mut Vec<T>, limit: usize, compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if limit == 0 {
        items.truncate(0);
    }
    if items.len() <= limit {
        items.sort_by(compare);
        return;
    }
    // When limit is near to items.len() it may be more efficient to sort the whole list and
    // truncate, rather than always doing selection first as is done below. It's hard to analyze
    // where the threshold for this should be since the quickselect style algorithm used by
    // `select_nth_unstable_by` makes the prefix partially sorted, and so its work is not wasted -
    // the expected number of comparisons needed by `sort_by` is less than it is for some arbitrary
    // unsorted input.
    items.select_nth_unstable_by(limit, compare);
    items.truncate(limit);
    items.sort_by(compare);
}

#[cfg(unix)]
fn load_shell_from_passwd() -> Result<()> {
    let buflen = match unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) } {
        n if n < 0 => 1024,
        n => n as usize,
    };
    let mut buffer = Vec::with_capacity(buflen);

    let mut pwd: std::mem::MaybeUninit<libc::passwd> = std::mem::MaybeUninit::uninit();
    let mut result: *mut libc::passwd = std::ptr::null_mut();

    let uid = unsafe { libc::getuid() };
    let status = unsafe {
        libc::getpwuid_r(
            uid,
            pwd.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut libc::c_char,
            buflen,
            &mut result,
        )
    };
    let entry = unsafe { pwd.assume_init() };

    anyhow::ensure!(
        status == 0,
        "call to getpwuid_r failed. uid: {}, status: {}",
        uid,
        status
    );
    anyhow::ensure!(!result.is_null(), "passwd entry for uid {} not found", uid);
    anyhow::ensure!(
        entry.pw_uid == uid,
        "passwd entry has different uid ({}) than getuid ({}) returned",
        entry.pw_uid,
        uid,
    );

    let shell = unsafe { std::ffi::CStr::from_ptr(entry.pw_shell).to_str().unwrap() };
    if env::var("SHELL").map_or(true, |shell_env| shell_env != shell) {
        log::info!(
            "updating SHELL environment variable to value from passwd entry: {:?}",
            shell,
        );
        unsafe { env::set_var("SHELL", shell) };
    }

    Ok(())
}

#[cfg(unix)]
pub fn load_login_shell_environment() -> Result<()> {
    load_shell_from_passwd().log_err();

    let marker = "ZED_LOGIN_SHELL_START";
    let shell = env::var("SHELL").context(
        "SHELL environment variable is not assigned so we can't source login environment variables",
    )?;

    // If possible, we want to `cd` in the user's `$HOME` to trigger programs
    // such as direnv, asdf, mise, ... to adjust the PATH. These tools often hook
    // into shell's `cd` command (and hooks) to manipulate env.
    // We do this so that we get the env a user would have when spawning a shell
    // in home directory.
    let shell_cmd_prefix = std::env::var_os("HOME")
        .and_then(|home| home.into_string().ok())
        .map(|home| format!("cd '{home}';"));

    let shell_cmd = format!(
        "{}printf '%s' {marker}; /usr/bin/env;",
        shell_cmd_prefix.as_deref().unwrap_or("")
    );

    let output = set_pre_exec_to_start_new_session(
        std::process::Command::new(&shell).args(["-l", "-i", "-c", &shell_cmd]),
    )
    .output()
    .context("failed to spawn login shell to source login environment variables")?;
    if !output.status.success() {
        Err(anyhow!("login shell exited with error"))?;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if let Some(env_output_start) = stdout.find(marker) {
        let env_output = &stdout[env_output_start + marker.len()..];

        parse_env_output(env_output, |key, value| unsafe { env::set_var(key, value) });

        log::info!(
            "set environment variables from shell:{}, path:{}",
            shell,
            env::var("PATH").unwrap_or_default(),
        );
    }

    Ok(())
}

/// Configures the process to start a new session, to prevent interactive shells from taking control
/// of the terminal.
///
/// For more details: https://registerspill.thorstenball.com/p/how-to-lose-control-of-your-shell
pub fn set_pre_exec_to_start_new_session(
    command: &mut std::process::Command,
) -> &mut std::process::Command {
    // safety: code in pre_exec should be signal safe.
    // https://man7.org/linux/man-pages/man7/signal-safety.7.html
    #[cfg(not(target_os = "windows"))]
    unsafe {
        use std::os::unix::process::CommandExt;
        command.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    };
    command
}

/// Parse the result of calling `usr/bin/env` with no arguments
pub fn parse_env_output(env: &str, mut f: impl FnMut(String, String)) {
    let mut current_key: Option<String> = None;
    let mut current_value: Option<String> = None;

    for line in env.split_terminator('\n') {
        if let Some(separator_index) = line.find('=') {
            if !line[..separator_index].is_empty() {
                if let Some((key, value)) = Option::zip(current_key.take(), current_value.take()) {
                    f(key, value)
                }
                current_key = Some(line[..separator_index].to_string());
                current_value = Some(line[separator_index + 1..].to_string());
                continue;
            };
        }
        if let Some(value) = current_value.as_mut() {
            value.push('\n');
            value.push_str(line);
        }
    }
    if let Some((key, value)) = Option::zip(current_key.take(), current_value.take()) {
        f(key, value)
    }
}

pub fn merge_json_value_into(source: serde_json::Value, target: &mut serde_json::Value) {
    use serde_json::Value;

    match (source, target) {
        (Value::Object(source), Value::Object(target)) => {
            for (key, value) in source {
                if let Some(target) = target.get_mut(&key) {
                    merge_json_value_into(value, target);
                } else {
                    target.insert(key, value);
                }
            }
        }

        (Value::Array(source), Value::Array(target)) => {
            for value in source {
                target.push(value);
            }
        }

        (source, target) => *target = source,
    }
}

pub fn merge_non_null_json_value_into(source: serde_json::Value, target: &mut serde_json::Value) {
    use serde_json::Value;
    if let Value::Object(source_object) = source {
        let target_object = if let Value::Object(target) = target {
            target
        } else {
            *target = Value::Object(Default::default());
            target.as_object_mut().unwrap()
        };
        for (key, value) in source_object {
            if let Some(target) = target_object.get_mut(&key) {
                merge_non_null_json_value_into(value, target);
            } else if !value.is_null() {
                target_object.insert(key, value);
            }
        }
    } else if !source.is_null() {
        *target = source
    }
}

pub fn measure<R>(label: &str, f: impl FnOnce() -> R) -> R {
    static ZED_MEASUREMENTS: OnceLock<bool> = OnceLock::new();
    let zed_measurements = ZED_MEASUREMENTS.get_or_init(|| {
        env::var("ZED_MEASUREMENTS")
            .map(|measurements| measurements == "1" || measurements == "true")
            .unwrap_or(false)
    });

    if *zed_measurements {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        eprintln!("{}: {:?}", label, elapsed);
        result
    } else {
        f()
    }
}

pub fn iterate_expanded_and_wrapped_usize_range(
    range: Range<usize>,
    additional_before: usize,
    additional_after: usize,
    wrap_length: usize,
) -> impl Iterator<Item = usize> {
    let start_wraps = range.start < additional_before;
    let end_wraps = wrap_length < range.end + additional_after;
    if start_wraps && end_wraps {
        Either::Left(0..wrap_length)
    } else if start_wraps {
        let wrapped_start = (range.start + wrap_length).saturating_sub(additional_before);
        if wrapped_start <= range.end {
            Either::Left(0..wrap_length)
        } else {
            Either::Right((0..range.end + additional_after).chain(wrapped_start..wrap_length))
        }
    } else if end_wraps {
        let wrapped_end = range.end + additional_after - wrap_length;
        if range.start <= wrapped_end {
            Either::Left(0..wrap_length)
        } else {
            Either::Right((0..wrapped_end).chain(range.start - additional_before..wrap_length))
        }
    } else {
        Either::Left((range.start - additional_before)..(range.end + additional_after))
    }
}

#[cfg(target_os = "windows")]
pub fn get_windows_system_shell() -> String {
    use std::path::PathBuf;

    fn find_pwsh_in_programfiles(find_alternate: bool, find_preview: bool) -> Option<PathBuf> {
        #[cfg(target_pointer_width = "64")]
        let env_var = if find_alternate {
            "ProgramFiles(x86)"
        } else {
            "ProgramFiles"
        };

        #[cfg(target_pointer_width = "32")]
        let env_var = if find_alternate {
            "ProgramW6432"
        } else {
            "ProgramFiles"
        };

        let install_base_dir = PathBuf::from(std::env::var_os(env_var)?).join("PowerShell");
        install_base_dir
            .read_dir()
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| matches!(entry.file_type(), Ok(ft) if ft.is_dir()))
            .filter_map(|entry| {
                let dir_name = entry.file_name();
                let dir_name = dir_name.to_string_lossy();

                let version = if find_preview {
                    let dash_index = dir_name.find('-')?;
                    if &dir_name[dash_index + 1..] != "preview" {
                        return None;
                    };
                    dir_name[..dash_index].parse::<u32>().ok()?
                } else {
                    dir_name.parse::<u32>().ok()?
                };

                let exe_path = entry.path().join("pwsh.exe");
                if exe_path.exists() {
                    Some((version, exe_path))
                } else {
                    None
                }
            })
            .max_by_key(|(version, _)| *version)
            .map(|(_, path)| path)
    }

    fn find_pwsh_in_msix(find_preview: bool) -> Option<PathBuf> {
        let msix_app_dir =
            PathBuf::from(std::env::var_os("LOCALAPPDATA")?).join("Microsoft\\WindowsApps");
        if !msix_app_dir.exists() {
            return None;
        }

        let prefix = if find_preview {
            "Microsoft.PowerShellPreview_"
        } else {
            "Microsoft.PowerShell_"
        };
        msix_app_dir
            .read_dir()
            .ok()?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if !matches!(entry.file_type(), Ok(ft) if ft.is_dir()) {
                    return None;
                }

                if !entry.file_name().to_string_lossy().starts_with(prefix) {
                    return None;
                }

                let exe_path = entry.path().join("pwsh.exe");
                exe_path.exists().then_some(exe_path)
            })
            .next()
    }

    fn find_pwsh_in_scoop() -> Option<PathBuf> {
        let pwsh_exe =
            PathBuf::from(std::env::var_os("USERPROFILE")?).join("scoop\\shims\\pwsh.exe");
        pwsh_exe.exists().then_some(pwsh_exe)
    }

    static SYSTEM_SHELL: LazyLock<String> = LazyLock::new(|| {
        find_pwsh_in_programfiles(false, false)
            .or_else(|| find_pwsh_in_programfiles(true, false))
            .or_else(|| find_pwsh_in_msix(false))
            .or_else(|| find_pwsh_in_programfiles(false, true))
            .or_else(|| find_pwsh_in_msix(true))
            .or_else(|| find_pwsh_in_programfiles(true, true))
            .or_else(find_pwsh_in_scoop)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or("powershell.exe".to_string())
    });

    (*SYSTEM_SHELL).clone()
}

pub trait ResultExt<E> {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
    /// Assert that this result should never be an error in development or tests.
    fn debug_assert_ok(self, reason: &str) -> Self;
    fn warn_on_err(self) -> Option<Self::Ok>;
    fn log_with_level(self, level: log::Level) -> Option<Self::Ok>;
    fn anyhow(self) -> anyhow::Result<Self::Ok>
    where
        E: Into<anyhow::Error>;
}

impl<T, E> ResultExt<E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    #[track_caller]
    fn log_err(self) -> Option<T> {
        self.log_with_level(log::Level::Error)
    }

    #[track_caller]
    fn debug_assert_ok(self, reason: &str) -> Self {
        if let Err(error) = &self {
            debug_panic!("{reason} - {error:?}");
        }
        self
    }

    #[track_caller]
    fn warn_on_err(self) -> Option<T> {
        self.log_with_level(log::Level::Warn)
    }

    #[track_caller]
    fn log_with_level(self, level: log::Level) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log_error_with_caller(*Location::caller(), error, level);
                None
            }
        }
    }

    fn anyhow(self) -> anyhow::Result<T>
    where
        E: Into<anyhow::Error>,
    {
        self.map_err(Into::into)
    }
}

fn log_error_with_caller<E>(caller: core::panic::Location<'_>, error: E, level: log::Level)
where
    E: std::fmt::Debug,
{
    #[cfg(not(target_os = "windows"))]
    let file = caller.file();
    #[cfg(target_os = "windows")]
    let file = caller.file().replace('\\', "/");
    // In this codebase, the first segment of the file path is
    // the 'crates' folder, followed by the crate name.
    let target = file.split('/').nth(1);

    log::logger().log(
        &log::Record::builder()
            .target(target.unwrap_or(""))
            .module_path(target)
            .args(format_args!("{:?}", error))
            .file(Some(caller.file()))
            .line(Some(caller.line()))
            .level(level)
            .build(),
    );
}

pub fn log_err<E: std::fmt::Debug>(error: &E) {
    log_error_with_caller(*Location::caller(), error, log::Level::Warn);
}

pub trait TryFutureExt {
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;
    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized;
}

impl<F, T, E> TryFutureExt for F
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    #[track_caller]
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, log::Level::Error, *location)
    }

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        LogErrorFuture(self, log::Level::Error, location)
    }

    #[track_caller]
    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, log::Level::Warn, *location)
    }

    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized,
    {
        UnwrapFuture(self)
    }
}

#[must_use]
pub struct LogErrorFuture<F>(F, log::Level, core::panic::Location<'static>);

impl<F, T, E> Future for LogErrorFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let level = self.1;
        let location = self.2;
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(output) => Poll::Ready(match output {
                Ok(output) => Some(output),
                Err(error) => {
                    log_error_with_caller(location, error, level);
                    None
                }
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct UnwrapFuture<F>(F);

impl<F, T, E> Future for UnwrapFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(result) => Poll::Ready(result.unwrap()),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Deferred<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Deferred<F> {
    /// Drop without running the deferred function.
    pub fn abort(mut self) {
        self.0.take();
    }
}

impl<F: FnOnce()> Drop for Deferred<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

/// Run the given function when the returned value is dropped (unless it's cancelled).
#[must_use]
pub fn defer<F: FnOnce()>(f: F) -> Deferred<F> {
    Deferred(Some(f))
}

#[cfg(any(test, feature = "test-support"))]
mod rng {
    use rand::{Rng, seq::SliceRandom};
    pub struct RandomCharIter<T: Rng> {
        rng: T,
        simple_text: bool,
    }

    impl<T: Rng> RandomCharIter<T> {
        pub fn new(rng: T) -> Self {
            Self {
                rng,
                simple_text: std::env::var("SIMPLE_TEXT").map_or(false, |v| !v.is_empty()),
            }
        }

        pub fn with_simple_text(mut self) -> Self {
            self.simple_text = true;
            self
        }
    }

    impl<T: Rng> Iterator for RandomCharIter<T> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            if self.simple_text {
                return if self.rng.gen_range(0..100) < 5 {
                    Some('\n')
                } else {
                    Some(self.rng.gen_range(b'a'..b'z' + 1).into())
                };
            }

            match self.rng.gen_range(0..100) {
                // whitespace
                0..=19 => [' ', '\n', '\r', '\t'].choose(&mut self.rng).copied(),
                // two-byte greek letters
                20..=32 => char::from_u32(self.rng.gen_range(('α' as u32)..('ω' as u32 + 1))),
                // // three-byte characters
                33..=45 => ['✋', '✅', '❌', '❎', '⭐']
                    .choose(&mut self.rng)
                    .copied(),
                // // four-byte characters
                46..=58 => ['🍐', '🏀', '🍗', '🎉'].choose(&mut self.rng).copied(),
                // ascii letters
                _ => Some(self.rng.gen_range(b'a'..b'z' + 1).into()),
            }
        }
    }
}
#[cfg(any(test, feature = "test-support"))]
pub use rng::RandomCharIter;
/// Get an embedded file as a string.
pub fn asset_str<A: rust_embed::RustEmbed>(path: &str) -> Cow<'static, str> {
    match A::get(path).expect(path).data {
        Cow::Borrowed(bytes) => Cow::Borrowed(std::str::from_utf8(bytes).unwrap()),
        Cow::Owned(bytes) => Cow::Owned(String::from_utf8(bytes).unwrap()),
    }
}

/// Expands to an immediately-invoked function expression. Good for using the ? operator
/// in functions which do not return an Option or Result.
///
/// Accepts a normal block, an async block, or an async move block.
#[macro_export]
macro_rules! maybe {
    ($block:block) => {
        (|| $block)()
    };
    (async $block:block) => {
        (|| async $block)()
    };
    (async move $block:block) => {
        (|| async move $block)()
    };
}

pub trait RangeExt<T> {
    fn sorted(&self) -> Self;
    fn to_inclusive(&self) -> RangeInclusive<T>;
    fn overlaps(&self, other: &Range<T>) -> bool;
    fn contains_inclusive(&self, other: &Range<T>) -> bool;
}

impl<T: Ord + Clone> RangeExt<T> for Range<T> {
    fn sorted(&self) -> Self {
        cmp::min(&self.start, &self.end).clone()..cmp::max(&self.start, &self.end).clone()
    }

    fn to_inclusive(&self) -> RangeInclusive<T> {
        self.start.clone()..=self.end.clone()
    }

    fn overlaps(&self, other: &Range<T>) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn contains_inclusive(&self, other: &Range<T>) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

impl<T: Ord + Clone> RangeExt<T> for RangeInclusive<T> {
    fn sorted(&self) -> Self {
        cmp::min(self.start(), self.end()).clone()..=cmp::max(self.start(), self.end()).clone()
    }

    fn to_inclusive(&self) -> RangeInclusive<T> {
        self.clone()
    }

    fn overlaps(&self, other: &Range<T>) -> bool {
        self.start() < &other.end && &other.start <= self.end()
    }

    fn contains_inclusive(&self, other: &Range<T>) -> bool {
        self.start() <= &other.start && &other.end <= self.end()
    }
}

/// A way to sort strings with starting numbers numerically first, falling back to alphanumeric one,
/// case-insensitive.
///
/// This is useful for turning regular alphanumerically sorted sequences as `1-abc, 10, 11-def, .., 2, 21-abc`
/// into `1-abc, 2, 10, 11-def, .., 21-abc`
#[derive(Debug, PartialEq, Eq)]
pub struct NumericPrefixWithSuffix<'a>(Option<u64>, &'a str);

impl<'a> NumericPrefixWithSuffix<'a> {
    pub fn from_numeric_prefixed_str(str: &'a str) -> Self {
        let i = str.chars().take_while(|c| c.is_ascii_digit()).count();
        let (prefix, remainder) = str.split_at(i);

        let prefix = prefix.parse().ok();
        Self(prefix, remainder)
    }
}

/// When dealing with equality, we need to consider the case of the strings to achieve strict equality
/// to handle cases like "a" < "A" instead of "a" == "A".
impl Ord for NumericPrefixWithSuffix<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (None, None) => UniCase::new(self.1)
                .cmp(&UniCase::new(other.1))
                .then_with(|| self.1.cmp(other.1).reverse()),
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(a), Some(b)) => a.cmp(&b).then_with(|| {
                UniCase::new(self.1)
                    .cmp(&UniCase::new(other.1))
                    .then_with(|| self.1.cmp(other.1).reverse())
            }),
        }
    }
}

impl PartialOrd for NumericPrefixWithSuffix<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Capitalizes the first character of a string.
///
/// This function takes a string slice as input and returns a new `String` with the first character
/// capitalized.
///
/// # Examples
///
/// ```
/// use util::capitalize;
///
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize("WORLD"), "WORLD");
/// assert_eq!(capitalize(""), "");
/// ```
pub fn capitalize(str: &str) -> String {
    let mut chars = str.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn emoji_regex() -> &'static Regex {
    static EMOJI_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("(\\p{Emoji}|\u{200D})").unwrap());
    &EMOJI_REGEX
}

/// Returns true if the given string consists of emojis only.
/// E.g. "👨‍👩‍👧‍👧👋" will return true, but "👋!" will return false.
pub fn word_consists_of_emojis(s: &str) -> bool {
    let mut prev_end = 0;
    for capture in emoji_regex().find_iter(s) {
        if capture.start() != prev_end {
            return false;
        }
        prev_end = capture.end();
    }
    prev_end == s.len()
}

pub fn default<D: Default>() -> D {
    Default::default()
}

pub fn get_system_shell() -> String {
    #[cfg(target_os = "windows")]
    {
        get_windows_system_shell()
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("SHELL").unwrap_or("/bin/sh".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_sorted() {
        let mut vec = vec![];

        extend_sorted(&mut vec, vec![21, 17, 13, 8, 1, 0], 5, |a, b| b.cmp(a));
        assert_eq!(vec, &[21, 17, 13, 8, 1]);

        extend_sorted(&mut vec, vec![101, 19, 17, 8, 2], 8, |a, b| b.cmp(a));
        assert_eq!(vec, &[101, 21, 19, 17, 13, 8, 2, 1]);

        extend_sorted(&mut vec, vec![1000, 19, 17, 9, 5], 8, |a, b| b.cmp(a));
        assert_eq!(vec, &[1000, 101, 21, 19, 17, 13, 9, 8]);
    }

    #[test]
    fn test_truncate_to_bottom_n_sorted_by() {
        let mut vec: Vec<u32> = vec![5, 2, 3, 4, 1];
        truncate_to_bottom_n_sorted_by(&mut vec, 10, &u32::cmp);
        assert_eq!(vec, &[1, 2, 3, 4, 5]);

        vec = vec![5, 2, 3, 4, 1];
        truncate_to_bottom_n_sorted_by(&mut vec, 5, &u32::cmp);
        assert_eq!(vec, &[1, 2, 3, 4, 5]);

        vec = vec![5, 2, 3, 4, 1];
        truncate_to_bottom_n_sorted_by(&mut vec, 4, &u32::cmp);
        assert_eq!(vec, &[1, 2, 3, 4]);

        vec = vec![5, 2, 3, 4, 1];
        truncate_to_bottom_n_sorted_by(&mut vec, 1, &u32::cmp);
        assert_eq!(vec, &[1]);

        vec = vec![5, 2, 3, 4, 1];
        truncate_to_bottom_n_sorted_by(&mut vec, 0, &u32::cmp);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_iife() {
        fn option_returning_function() -> Option<()> {
            None
        }

        let foo = maybe!({
            option_returning_function()?;
            Some(())
        });

        assert_eq!(foo, None);
    }

    #[test]
    fn test_truncate_and_trailoff() {
        assert_eq!(truncate_and_trailoff("", 5), "");
        assert_eq!(truncate_and_trailoff("aaaaaa", 7), "aaaaaa");
        assert_eq!(truncate_and_trailoff("aaaaaa", 6), "aaaaaa");
        assert_eq!(truncate_and_trailoff("aaaaaa", 5), "aaaaa…");
        assert_eq!(truncate_and_trailoff("èèèèèè", 7), "èèèèèè");
        assert_eq!(truncate_and_trailoff("èèèèèè", 6), "èèèèèè");
        assert_eq!(truncate_and_trailoff("èèèèèè", 5), "èèèèè…");
    }

    #[test]
    fn test_truncate_and_remove_front() {
        assert_eq!(truncate_and_remove_front("", 5), "");
        assert_eq!(truncate_and_remove_front("aaaaaa", 7), "aaaaaa");
        assert_eq!(truncate_and_remove_front("aaaaaa", 6), "aaaaaa");
        assert_eq!(truncate_and_remove_front("aaaaaa", 5), "…aaaaa");
        assert_eq!(truncate_and_remove_front("èèèèèè", 7), "èèèèèè");
        assert_eq!(truncate_and_remove_front("èèèèèè", 6), "èèèèèè");
        assert_eq!(truncate_and_remove_front("èèèèèè", 5), "…èèèèè");
    }

    #[test]
    fn test_numeric_prefix_str_method() {
        let target = "1a";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(1), "a")
        );

        let target = "12ab";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(12), "ab")
        );

        let target = "12_ab";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(12), "_ab")
        );

        let target = "1_2ab";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(1), "_2ab")
        );

        let target = "1.2";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(1), ".2")
        );

        let target = "1.2_a";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(1), ".2_a")
        );

        let target = "12.2_a";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(12), ".2_a")
        );

        let target = "12a.2_a";
        assert_eq!(
            NumericPrefixWithSuffix::from_numeric_prefixed_str(target),
            NumericPrefixWithSuffix(Some(12), "a.2_a")
        );
    }

    #[test]
    fn test_numeric_prefix_with_suffix() {
        let mut sorted = vec!["1-abc", "10", "11def", "2", "21-abc"];
        sorted.sort_by_key(|s| NumericPrefixWithSuffix::from_numeric_prefixed_str(s));
        assert_eq!(sorted, ["1-abc", "2", "10", "11def", "21-abc"]);

        for numeric_prefix_less in ["numeric_prefix_less", "aaa", "~™£"] {
            assert_eq!(
                NumericPrefixWithSuffix::from_numeric_prefixed_str(numeric_prefix_less),
                NumericPrefixWithSuffix(None, numeric_prefix_less),
                "String without numeric prefix `{numeric_prefix_less}` should not be converted into NumericPrefixWithSuffix"
            )
        }
    }

    #[test]
    fn test_word_consists_of_emojis() {
        let words_to_test = vec![
            ("👨‍👩‍👧‍👧👋🥒", true),
            ("👋", true),
            ("!👋", false),
            ("👋!", false),
            ("👋 ", false),
            (" 👋", false),
            ("Test", false),
        ];

        for (text, expected_result) in words_to_test {
            assert_eq!(word_consists_of_emojis(text), expected_result);
        }
    }

    #[test]
    fn test_truncate_lines_and_trailoff() {
        let text = r#"Line 1
Line 2
Line 3"#;

        assert_eq!(
            truncate_lines_and_trailoff(text, 2),
            r#"Line 1
…"#
        );

        assert_eq!(
            truncate_lines_and_trailoff(text, 3),
            r#"Line 1
Line 2
…"#
        );

        assert_eq!(
            truncate_lines_and_trailoff(text, 4),
            r#"Line 1
Line 2
Line 3"#
        );
    }

    #[test]
    fn test_iterate_expanded_and_wrapped_usize_range() {
        // Neither wrap
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(2..4, 1, 1, 8).collect::<Vec<usize>>(),
            (1..5).collect::<Vec<usize>>()
        );
        // Start wraps
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(2..4, 3, 1, 8).collect::<Vec<usize>>(),
            ((0..5).chain(7..8)).collect::<Vec<usize>>()
        );
        // Start wraps all the way around
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(2..4, 5, 1, 8).collect::<Vec<usize>>(),
            (0..8).collect::<Vec<usize>>()
        );
        // Start wraps all the way around and past 0
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(2..4, 10, 1, 8).collect::<Vec<usize>>(),
            (0..8).collect::<Vec<usize>>()
        );
        // End wraps
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(3..5, 1, 4, 8).collect::<Vec<usize>>(),
            (0..1).chain(2..8).collect::<Vec<usize>>()
        );
        // End wraps all the way around
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(3..5, 1, 5, 8).collect::<Vec<usize>>(),
            (0..8).collect::<Vec<usize>>()
        );
        // End wraps all the way around and past the end
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(3..5, 1, 10, 8).collect::<Vec<usize>>(),
            (0..8).collect::<Vec<usize>>()
        );
        // Both start and end wrap
        assert_eq!(
            iterate_expanded_and_wrapped_usize_range(3..5, 4, 4, 8).collect::<Vec<usize>>(),
            (0..8).collect::<Vec<usize>>()
        );
    }
}
