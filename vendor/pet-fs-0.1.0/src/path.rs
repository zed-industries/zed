// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::path::MAIN_SEPARATOR;

#[cfg(windows)]
use std::{collections::HashMap, sync::RwLock};

#[cfg(windows)]
use lazy_static::lazy_static;

#[cfg(windows)]
lazy_static! {
    /// Per-process memoization cache for `norm_case` results on Windows.
    ///
    /// Each `norm_case` call invokes `GetLongPathNameW`, which is a real
    /// filesystem syscall (Defender-intercepted on Windows). Many hot paths
    /// — the user home directory, common parents, every `PATH` entry, every
    /// registry `env_path`/`executable` — get normalized repeatedly per
    /// refresh. Memoizing eliminates the repeat syscalls.
    ///
    /// We cache only successful normalizations: a failure (`GetLongPathNameW`
    /// returning 0, typically because the path doesn't exist yet) is *not*
    /// cached so a path that gets created later can still be normalized.
    static ref NORM_CASE_CACHE: RwLock<HashMap<PathBuf, PathBuf>> =
        RwLock::new(HashMap::new());
}

/// Strips trailing path separators from a path, preserving root paths.
///
/// This function removes trailing `/` or `\` from paths while ensuring that root paths
/// like `/` on Unix or `C:\` on Windows are preserved.
///
/// # Examples
/// - `/home/user/` → `/home/user`
/// - `C:\Users\` → `C:\Users`
/// - `/` → `/` (preserved)
/// - `C:\` → `C:\` (preserved)
///
/// # Use Cases
/// Use this when path data comes from external sources that may include trailing separators:
/// - Windows Registry entries (e.g., `C:\...\x64\`)
/// - Configuration files (environments.txt, .condarc)
/// - Environment variables
///
/// # Related
/// - `norm_case()` - Full path normalization (includes trailing separator stripping on Windows)
pub fn strip_trailing_separator<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_str = path.as_ref().to_string_lossy();

    #[cfg(windows)]
    {
        // On Windows, preserve root paths (e.g. "C:\", "\\server\", "\\?\C:\")
        let mut result = path_str.to_string();
        while (result.ends_with('\\') || result.ends_with('/'))
            && Path::new(&result).parent().is_some()
        {
            result.pop();
        }
        PathBuf::from(result)
    }

    #[cfg(unix)]
    {
        // On Unix, preserve the root "/"
        let mut result = path_str.to_string();
        while result.len() > 1 && result.ends_with(MAIN_SEPARATOR) {
            result.pop();
        }
        PathBuf::from(result)
    }
}

/// Normalizes path case on Windows without resolving symlinks/junctions.
///
/// # Behavior by Platform
///
/// ## Windows
/// - Normalizes path case to match the actual filesystem casing
/// - Converts relative paths to absolute paths
/// - Converts forward slashes to backslashes
/// - Strips trailing path separators (except for root paths like `C:\`)
/// - Removes UNC prefix (`\\?\`) if the original path didn't have it
/// - **Does NOT resolve symlinks or junctions** (uses `GetLongPathNameW`)
/// - For non-existent paths, returns the absolute path without case normalization
///
/// ## Unix
/// - Returns the path unchanged (no-op)
/// - Path case is significant on Unix, so no normalization is performed
///
/// # Use Cases
///
/// This function is typically used for:
///
/// 1. **Path Comparison/Hashing**: Ensures consistent path representation for cache keys
///    and hash generation (e.g., Poetry environment name hashing, fs_cache)
///
/// 2. **Sanitizing External Path Sources**: Normalizes paths from external sources like:
///    - Windows Registry entries (may have trailing slashes)
///    - Configuration files (environments.txt, .condarc)
///    - Environment variables (VIRTUAL_ENV, WORKON_HOME)
///
/// 3. **Storing/Displaying Paths**: Ensures paths are in a canonical form for storage
///    and display (e.g., `PythonEnvironment.executable`, `PythonEnvironment.prefix`)
///
/// # Important Notes
///
/// - On Windows, this function uses `GetLongPathNameW` which **preserves junction paths**
///   unlike `fs::canonicalize` which would resolve them to their target.
/// - For symlink resolution, use `resolve_symlink()` instead.
/// - On Windows, results are memoized in a per-process cache. The key is
///   the computed *absolute* path with separators canonicalized (forward
///   → backslash) and trailing separators stripped, so the same logical
///   path is cached only once even when callers vary in style. Only
///   successful normalizations are cached, so paths that do not yet exist
///   on disk can still be normalized later. The cache has a generous soft
///   cap (`NORM_CASE_CACHE_MAX_ENTRIES`) and is cleared when exceeded; it
///   is otherwise never invalidated for the lifetime of the process.
///
/// # Related
/// - `strip_trailing_separator()` - Just removes trailing separators
/// - `resolve_symlink()` - Resolves symlinks to their target
/// - `expand_path()` - Expands `~` and environment variables
///
/// See: <https://github.com/microsoft/python-environment-tools/issues/186>
/// See: <https://github.com/microsoft/python-environment-tools/issues/278>
pub fn norm_case<P: AsRef<Path>>(path: P) -> PathBuf {
    // On unix do not use canonicalize, results in weird issues with homebrew paths
    // Even readlink does the same thing
    // Running readlink for a path thats not a symlink ends up returning relative paths for some reason.
    // A better solution is to first check if a path is a symlink and then resolve it.
    #[cfg(unix)]
    return path.as_ref().to_path_buf();

    #[cfg(windows)]
    {
        // First, convert to absolute path if relative, without resolving symlinks/junctions.
        // We key the cache on this *absolute* path rather than the caller's input so
        // a relative input like "./foo" stays correct if the process CWD changes
        // between calls. If `current_dir()` itself fails (extremely rare), we
        // proceed with the original path but skip caching — see below.
        let (absolute_path, abs_ok) = if path.as_ref().is_absolute() {
            (path.as_ref().to_path_buf(), true)
        } else if let Ok(abs) = std::env::current_dir() {
            (abs.join(path.as_ref()), true)
        } else {
            (path.as_ref().to_path_buf(), false)
        };

        // Apply a cheap, deterministic pre-normalization (slash style + trailing
        // separator) to the cache key so the same logical path is not cached under
        // multiple keys (e.g. `C:\Windows`, `C:\Windows\`, and `C:/Windows`).
        // This is purely an in-memory transform — no syscalls.
        let cache_key = cache_key_for(&absolute_path);

        // Fast path: cache hit. Read lock is held only while we copy the
        // already-normalized PathBuf out. If the lock is poisoned we recover
        // the inner map via `into_inner()` so the cache keeps working for
        // the rest of the process — silently disabling the cache after a
        // single bad caller would reintroduce the syscall storm we're
        // trying to avoid.
        let cache_read = NORM_CASE_CACHE
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(cached) = cache_read.get(&cache_key) {
            return cached.clone();
        }
        // Explicitly drop the read guard before we try to acquire the
        // write guard below: `std::sync::RwLock` does not guarantee
        // safe same-thread read → write upgrades, and holding both at
        // once can deadlock on some platforms.
        drop(cache_read);

        // Use GetLongPathNameW to normalize case without resolving junctions.
        // If normalization fails, fall back to the computed absolute path to keep behavior consistent.
        match normalize_case_windows(&absolute_path) {
            Some(normalized) => {
                // Only cache successful normalizations whose key is a real
                // absolute path:
                //  * Failures (None) are skipped because the path may be
                //    created later in the process and should normalize then.
                //  * !abs_ok means we never resolved CWD, so the key is a
                //    relative path — caching it could become wrong if CWD
                //    changes later. This branch is essentially unreachable
                //    in practice (current_dir() failing) but we'd rather
                //    pay the syscall again than poison the cache.
                if abs_ok {
                    let mut cache = NORM_CASE_CACHE
                        .write()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                    // Soft cap: if the cache grows unreasonably large
                    // (e.g. a long-lived JSONRPC server scanning many
                    // workspaces), reset rather than tracking a true LRU.
                    // The working set per-session is small, so re-warming
                    // is cheap.
                    if cache.len() >= NORM_CASE_CACHE_MAX_ENTRIES {
                        cache.clear();
                    }
                    cache.insert(cache_key, normalized.clone());
                }
                normalized
            }
            None => absolute_path,
        }
    }
}

/// Soft upper bound on the number of entries `NORM_CASE_CACHE` will hold
/// before being cleared. The working set of paths a single PET refresh
/// touches is at most low hundreds (PATH entries, registry installs,
/// conda envs, virtualenv prefixes, etc.). 4096 leaves ample headroom
/// for unusual workspaces while keeping the worst-case memory footprint
/// bounded for long-lived JSONRPC server processes.
#[cfg(windows)]
const NORM_CASE_CACHE_MAX_ENTRIES: usize = 4096;

/// Builds a deterministic cache key for `norm_case` from an absolute path.
///
/// We canonicalize separators (forward → backslash) and strip trailing
/// separators (preserving roots) so that, for example, `C:\Windows`,
/// `C:\Windows\`, and `C:/Windows` all collapse to the same key. This is
/// purely an in-memory transform — no syscalls — and exists only to
/// improve cache hit rate, not to change the result of `norm_case`.
///
/// We operate on the UTF-16 (`encode_wide`) representation rather than
/// going through `to_string_lossy()` so the transform is lossless for
/// all `Path` values, including the rare paths that contain non-Unicode
/// bytes (unpaired surrogates) which `to_string_lossy()` would map to
/// `U+FFFD` and so could collapse distinct paths to the same key.
#[cfg(windows)]
fn cache_key_for(absolute_path: &Path) -> PathBuf {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};

    const FWD: u16 = b'/' as u16;
    const BACK: u16 = b'\\' as u16;

    // Replace forward slashes with backslashes in-place on the wide form.
    let mut wide: Vec<u16> = absolute_path
        .as_os_str()
        .encode_wide()
        .map(|c| if c == FWD { BACK } else { c })
        .collect();

    // Strip trailing separators while preserving root paths (drive roots,
    // UNC roots, network paths, extended-length roots). We rebuild a
    // `PathBuf` only as a probe for `Path::parent()`, which already
    // handles all the Windows root shapes correctly.
    while wide.last() == Some(&BACK) {
        let probe = PathBuf::from(OsString::from_wide(&wide));
        if probe.parent().is_some() {
            wide.pop();
        } else {
            break;
        }
    }

    PathBuf::from(OsString::from_wide(&wide))
}

/// Windows-specific path case normalization using GetLongPathNameW.
/// This normalizes the case of path components but does NOT resolve junctions or symlinks.
/// Note: GetLongPathNameW requires the path to exist on the filesystem to normalize it.
/// For non-existent paths, it will fail and this function returns None.
/// Also note: Converting paths to strings via to_string_lossy() may lose information
/// for paths with invalid UTF-8 sequences (replaced with U+FFFD), though Windows paths
/// are typically valid Unicode.
#[cfg(windows)]
fn normalize_case_windows(path: &Path) -> Option<PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use windows_sys::Win32::Storage::FileSystem::GetLongPathNameW;

    // Check if original path has UNC prefix before normalization
    let original_path_str = path.to_string_lossy();
    let original_has_unc = original_path_str.starts_with(r"\\?\");

    // Normalize forward slashes to backslashes (canonicalize did this)
    let path_str = original_path_str.replace('/', "\\");
    let normalized_path = PathBuf::from(&path_str);

    // Convert path to wide string (UTF-16) with null terminator
    let wide_path: Vec<u16> = normalized_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // First call to get required buffer size
    let required_len = unsafe { GetLongPathNameW(wide_path.as_ptr(), std::ptr::null_mut(), 0) };

    if required_len == 0 {
        // GetLongPathNameW failed (path likely doesn't exist), return None
        return None;
    }

    // Allocate buffer and get the normalized path
    let mut buffer: Vec<u16> = vec![0; required_len as usize];
    let actual_len =
        unsafe { GetLongPathNameW(wide_path.as_ptr(), buffer.as_mut_ptr(), required_len) };

    if actual_len == 0 || actual_len > required_len {
        // Call failed or buffer too small
        return None;
    }

    // Truncate buffer to actual length (excluding null terminator)
    buffer.truncate(actual_len as usize);

    // Convert back to PathBuf
    let os_string = OsString::from_wide(&buffer);
    let mut result_str = os_string.to_string_lossy().to_string();

    // Remove UNC prefix if original path didn't have it
    // GetLongPathNameW may add \\?\ prefix in some cases
    if result_str.starts_with(r"\\?\") && !original_has_unc {
        result_str = result_str.trim_start_matches(r"\\?\").to_string();
    }

    // Strip trailing path separators to match canonicalize behavior,
    // but avoid stripping them from root paths (drive roots, UNC roots, network paths).
    // We use Path::parent() to detect root paths robustly.
    let mut current_path = PathBuf::from(&result_str);
    while current_path.parent().is_some() {
        let s = current_path.to_string_lossy();
        if s.ends_with('\\') || s.ends_with('/') {
            result_str.pop();
            current_path = PathBuf::from(&result_str);
        } else {
            break;
        }
    }

    Some(PathBuf::from(result_str))
}

/// Resolves any symlink to its real file path without filtering.
///
/// Returns `None` if the path is not a symlink or cannot be resolved.
/// If the real file equals the input, returns `None` (the path is not a symlink).
///
/// # Use Cases
/// - Resolving Homebrew symlinks for tools like Poetry: `/opt/homebrew/bin/poetry` → Cellar path
/// - Generic symlink resolution where no filename filtering is needed
///
/// # Related
/// - `resolve_symlink()` - Filtered version for Python/Conda executables only
pub fn resolve_any_symlink<T: AsRef<Path>>(path: &T) -> Option<PathBuf> {
    let metadata = std::fs::symlink_metadata(path).ok()?;
    if metadata.is_file() || !metadata.file_type().is_symlink() {
        return None;
    }
    if let Ok(readlink) = std::fs::canonicalize(path) {
        if readlink == path.as_ref().to_path_buf() {
            None
        } else {
            Some(readlink)
        }
    } else {
        None
    }
}

/// Resolves a symlink to its real file path.
///
/// Returns `None` if the path is not a symlink or cannot be resolved.
/// If the real file equals the input, returns `None` (the path is not a symlink).
///
/// # Filtering
/// This function only resolves symlinks for Python and Conda related executables:
/// - Files starting with `python` or `conda` (e.g., `python3.12`, `conda`)
/// - Excludes files ending with `-config` or `-build` (e.g., `python3-config`)
///
/// # Use Cases
/// - Identifying the actual Python executable behind symbolic links
/// - Homebrew Python symlinks: `/opt/homebrew/bin/python3.12` → actual Cellar path
/// - Tracking all symlink variants of a Python installation
///
/// # Related
/// - `norm_case()` - Normalizes path case without resolving symlinks
/// - `resolve_any_symlink()` - Unfiltered version for any symlink
pub fn resolve_symlink<T: AsRef<Path>>(exe: &T) -> Option<PathBuf> {
    let name = exe.as_ref().file_name()?.to_string_lossy();
    // In bin directory of homebrew, we have files like python-build, python-config, python3-config
    if name.ends_with("-config") || name.ends_with("-build") {
        return None;
    }
    // We support resolving conda symlinks.
    if !name.starts_with("python") && !name.starts_with("conda") {
        return None;
    }

    // Running readlink for a path thats not a symlink ends up returning relative paths for some reason.
    // A better solution is to first check if a path is a symlink and then resolve it.
    let metadata = std::fs::symlink_metadata(exe).ok()?;
    if metadata.is_file() || !metadata.file_type().is_symlink() {
        return None;
    }
    if let Ok(readlink) = std::fs::canonicalize(exe) {
        if readlink == exe.as_ref().to_path_buf() {
            None
        } else {
            Some(readlink)
        }
    } else {
        None
    }
}

/// Expands `~` (home directory) and environment variables in a path.
///
/// This function handles:
/// - `~` prefix: Expands to the user's home directory
/// - `${USERNAME}`: Expands to the current username
/// - `${HOME}`: Expands to the home directory
///
/// # Examples
/// - `~/envs` → `/home/user/envs`
/// - `${HOME}/.conda` → `/home/user/.conda`
///
/// # Environment Variables
/// - On Unix: Uses `$HOME` for home directory, `$USER` for username
/// - On Windows: Uses `%USERPROFILE%` for home directory, `%USERNAME%` for username
///
/// # Use Cases
/// Used primarily for expanding paths from conda rc files which support
/// [environment variable expansion](https://docs.conda.io/projects/conda/en/23.1.x/user-guide/configuration/use-condarc.html#expansion-of-environment-variables).
///
/// # Related
/// - `norm_case()` - Normalizes path case
/// - `strip_trailing_separator()` - Removes trailing path separators
pub fn expand_path(path: PathBuf) -> PathBuf {
    if path.starts_with("~") {
        if let Some(ref home) = get_user_home() {
            if let Ok(path) = path.strip_prefix("~") {
                return home.join(path);
            } else {
                return path;
            }
        }
    }

    // Specifically for https://docs.conda.io/projects/conda/en/23.1.x/user-guide/configuration/use-condarc.html#expansion-of-environment-variables
    if path.to_str().unwrap_or_default().contains("${USERNAME}")
        || path.to_str().unwrap_or_default().contains("${HOME}")
    {
        let username = env::var("USERNAME")
            .or(env::var("USER"))
            .unwrap_or_default();
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or_default();
        return PathBuf::from(
            path.to_str()
                .unwrap()
                .replace("${USERNAME}", &username)
                .replace("${HOME}", &home),
        );
    }
    path
}

fn get_user_home() -> Option<PathBuf> {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));
    match home {
        Ok(home) => Some(norm_case(PathBuf::from(home))),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== strip_trailing_separator tests ====================

    #[test]
    fn test_strip_trailing_separator_no_trailing() {
        // Paths without trailing separators should remain unchanged
        assert_eq!(
            strip_trailing_separator("/home/user"),
            PathBuf::from("/home/user")
        );
        assert_eq!(
            strip_trailing_separator("/home/user/envs"),
            PathBuf::from("/home/user/envs")
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_strip_trailing_separator_unix() {
        // Strip trailing slash
        assert_eq!(
            strip_trailing_separator("/home/user/"),
            PathBuf::from("/home/user")
        );
        // Multiple trailing slashes
        assert_eq!(
            strip_trailing_separator("/home/user///"),
            PathBuf::from("/home/user")
        );
        // Root path should be preserved
        assert_eq!(strip_trailing_separator("/"), PathBuf::from("/"));
    }

    #[test]
    #[cfg(windows)]
    fn test_strip_trailing_separator_windows() {
        // Strip trailing backslash
        assert_eq!(
            strip_trailing_separator("C:\\Users\\"),
            PathBuf::from("C:\\Users")
        );
        // Strip trailing forward slash (also valid on Windows)
        assert_eq!(
            strip_trailing_separator("C:\\Users/"),
            PathBuf::from("C:\\Users")
        );
        // Root path should be preserved
        assert_eq!(strip_trailing_separator("C:\\"), PathBuf::from("C:\\"));
    }

    #[test]
    #[cfg(windows)]
    fn test_strip_trailing_separator_windows_unc_paths() {
        // UNC path with trailing separator - should strip it
        assert_eq!(
            strip_trailing_separator("\\\\server\\share\\folder\\"),
            PathBuf::from("\\\\server\\share\\folder")
        );
        // UNC root path should be preserved
        assert_eq!(
            strip_trailing_separator("\\\\server\\share\\"),
            PathBuf::from("\\\\server\\share\\")
        );
        // Extended-length path root should be preserved
        assert_eq!(
            strip_trailing_separator("\\\\?\\C:\\"),
            PathBuf::from("\\\\?\\C:\\")
        );
        // Extended-length path with subfolder - should strip trailing separator
        assert_eq!(
            strip_trailing_separator("\\\\?\\C:\\Users\\"),
            PathBuf::from("\\\\?\\C:\\Users")
        );
    }

    // ==================== norm_case tests ====================

    #[test]
    #[cfg(unix)]
    fn test_norm_case_returns_path_for_nonexistent_unix() {
        // On Unix, norm_case returns the path unchanged (noop)
        let nonexistent = PathBuf::from("/this/path/does/not/exist/anywhere");
        let result = norm_case(&nonexistent);
        assert_eq!(result, nonexistent);
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_returns_absolute_for_nonexistent_windows() {
        // On Windows, norm_case returns an absolute path even for non-existent paths
        // (falls back to absolute_path when GetLongPathNameW fails)
        let nonexistent = PathBuf::from("C:\\this\\path\\does\\not\\exist\\anywhere");
        let result = norm_case(&nonexistent);
        assert!(result.is_absolute(), "Result should be absolute path");
        // The path should be preserved (just made absolute if it wasn't)
        assert!(
            result
                .to_string_lossy()
                .to_lowercase()
                .contains("does\\not\\exist"),
            "Path components should be preserved"
        );
    }

    #[test]
    fn test_norm_case_existing_path() {
        // norm_case should work on existing paths
        let temp_dir = std::env::temp_dir();
        let result = norm_case(&temp_dir);
        // On unix, should return unchanged; on Windows, should normalize case
        assert!(result.exists());
    }

    #[test]
    #[cfg(unix)]
    fn test_norm_case_unix_noop() {
        // On unix, norm_case should return the path unchanged
        let path = PathBuf::from("/Some/Path/With/Mixed/Case");
        let result = norm_case(&path);
        assert_eq!(result, path);
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_case_normalization() {
        // On Windows, norm_case should normalize the case of existing paths
        // Use the Windows directory which always exists
        let path = PathBuf::from("c:\\windows\\system32");
        let result = norm_case(&path);
        // The result should have proper casing (C:\Windows\System32)
        assert!(result.to_string_lossy().contains("Windows"));
        assert!(result.to_string_lossy().contains("System32"));
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_preserves_junction() {
        // This is the key test for issue #186:
        // norm_case should NOT resolve junctions to their target
        use std::fs;
        use std::process::Command;

        let temp_dir = std::env::temp_dir();
        let target_dir = temp_dir.join("pet_test_junction_target");
        let junction_dir = temp_dir.join("pet_test_junction_link");

        // Clean up any existing test directories
        let _ = fs::remove_dir_all(&target_dir);
        let _ = fs::remove_dir_all(&junction_dir);

        // Create target directory
        fs::create_dir_all(&target_dir).expect("Failed to create target directory");

        // Create a junction using mklink /J (requires no special privileges)
        let output = Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                &junction_dir.to_string_lossy(),
                &target_dir.to_string_lossy(),
            ])
            .output()
            .expect("Failed to create junction");

        if !output.status.success() {
            // Clean up and skip test if junction creation failed
            let _ = fs::remove_dir_all(&target_dir);
            eprintln!(
                "Skipping junction test - failed to create junction: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }

        // Verify junction was created
        assert!(junction_dir.exists(), "Junction should exist");

        // The key assertion: norm_case should return the junction path, NOT the target path
        let result = norm_case(&junction_dir);

        // The result should still be the junction path, not resolved to target
        // Compare the path names (case-insensitive on Windows)
        assert!(
            result
                .to_string_lossy()
                .to_lowercase()
                .contains("pet_test_junction_link"),
            "norm_case should preserve junction path, got: {:?}",
            result
        );
        assert!(
            !result
                .to_string_lossy()
                .to_lowercase()
                .contains("pet_test_junction_target"),
            "norm_case should NOT resolve to target path, got: {:?}",
            result
        );

        // Clean up
        // Remove junction first (using rmdir, not remove_dir_all, to not follow the junction)
        let _ = Command::new("cmd")
            .args(["/C", "rmdir", &junction_dir.to_string_lossy()])
            .output();
        let _ = fs::remove_dir_all(&target_dir);
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_relative_path() {
        // Test that relative paths are converted to absolute
        let relative = PathBuf::from(".");
        let result = norm_case(&relative);
        assert!(result.is_absolute(), "Result should be absolute path");
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_no_unc_prefix_added() {
        // Ensure we don't add UNC prefix to paths that didn't have it
        let path = PathBuf::from("C:\\Windows");
        let result = norm_case(&path);
        assert!(
            !result.to_string_lossy().starts_with(r"\\?\"),
            "Should not add UNC prefix"
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_strips_trailing_slash() {
        // norm_case should strip trailing slashes to match canonicalize behavior
        let path_with_slash = PathBuf::from("C:\\Windows\\");
        let result = norm_case(&path_with_slash);
        assert!(
            !result.to_string_lossy().ends_with('\\'),
            "Should strip trailing backslash, got: {:?}",
            result
        );

        // But root paths like C:\ should keep their slash
        let root_path = PathBuf::from("C:\\");
        let root_result = norm_case(&root_path);
        assert!(
            root_result.to_string_lossy().ends_with('\\'),
            "Root path should keep trailing backslash, got: {:?}",
            root_result
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_normalizes_slashes() {
        // norm_case should convert forward slashes to backslashes (like canonicalize did)
        let path_with_forward_slashes = PathBuf::from("C:/Windows/System32");
        let result = norm_case(&path_with_forward_slashes);
        assert!(
            !result.to_string_lossy().contains('/'),
            "Should convert forward slashes to backslashes, got: {:?}",
            result
        );
        assert!(
            result.to_string_lossy().contains('\\'),
            "Should have backslashes, got: {:?}",
            result
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_preserves_unc_prefix() {
        // If the original path has a UNC prefix, it should be preserved
        let unc_path = PathBuf::from(r"\\?\C:\Windows");
        let result = norm_case(&unc_path);
        assert!(
            result.to_string_lossy().starts_with(r"\\?\"),
            "Should preserve UNC prefix when present in original, got: {:?}",
            result
        );
    }

    /// Helper: returns an existing Windows directory derived from
    /// `SystemRoot` (e.g. `C:\Windows`) so tests don't hardcode a drive
    /// letter or the `\System32` subfolder. Both can vary on custom
    /// installs / CI setups.
    #[cfg(windows)]
    fn system_root_dir() -> PathBuf {
        let root = std::env::var("SystemRoot")
            .or_else(|_| std::env::var("WINDIR"))
            .expect("SystemRoot or WINDIR must be set on Windows");
        PathBuf::from(root)
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_memoizes_existing_path() {
        // Successful normalizations should be cached so subsequent calls
        // for the same input do not re-issue GetLongPathNameW.
        // The cache is keyed on the canonicalized absolute path, so we
        // pass an already-canonical absolute path here to make the
        // assertion deterministic.
        let path = system_root_dir();
        let key = cache_key_for(&path);
        // Tests run in parallel and share NORM_CASE_CACHE; clear the key
        // first so this test really exercises insertion-on-miss.
        NORM_CASE_CACHE
            .write()
            .expect("norm_case cache poisoned")
            .remove(&key);

        let first = norm_case(&path);
        // Cache must contain the canonicalized form of the input.
        let cached = NORM_CASE_CACHE
            .read()
            .expect("norm_case cache poisoned")
            .get(&key)
            .cloned();
        assert_eq!(
            cached.as_ref(),
            Some(&first),
            "expected norm_case to insert the result into the cache"
        );
        // Second call returns the same result (and goes through the cache).
        let second = norm_case(&path);
        assert_eq!(first, second);
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_does_not_cache_failures() {
        // GetLongPathNameW fails for non-existent paths. We must NOT cache
        // those, otherwise a path that gets created later in the process
        // would forever return its non-normalized fallback form.
        //
        // Build the test path inside `std::env::temp_dir()` so it is
        // anchored to a real, writable location regardless of which drive
        // Windows is installed on, and use a unique name with a low
        // collision rate. Then make sure it does not exist on disk so
        // GetLongPathNameW deterministically fails.
        let nonexistent = std::env::temp_dir().join(format!(
            "pet_test_norm_case_no_cache_{}_{}",
            std::process::id(),
            // Wall-clock nanoseconds — process id alone isn't enough on
            // hosts that recycle PIDs quickly.
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        assert!(
            !nonexistent.exists(),
            "test setup expected a non-existent path"
        );

        let key = cache_key_for(&nonexistent);
        // Make sure we start from a clean slate for this key.
        NORM_CASE_CACHE
            .write()
            .expect("norm_case cache poisoned")
            .remove(&key);

        let _ = norm_case(&nonexistent);

        let has_entry = NORM_CASE_CACHE
            .read()
            .expect("norm_case cache poisoned")
            .contains_key(&key);
        assert!(
            !has_entry,
            "failed normalizations must not be inserted into the cache"
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_cache_key_is_absolute_path() {
        // The cache is keyed on the computed absolute path, not the
        // caller-supplied input. Calling norm_case on a relative path
        // must NOT insert an entry under that relative key; the entry
        // must instead live under the resolved absolute path.
        //
        // We avoid mutating the process CWD here because cargo runs tests
        // in parallel and a global chdir would race with other tests.
        let relative = PathBuf::from(".");
        let result = norm_case(&relative);

        assert!(result.is_absolute(), "result must be absolute");
        let cache = NORM_CASE_CACHE.read().expect("norm_case cache poisoned");
        assert!(
            !cache.contains_key(&relative),
            "cache must not be keyed on the caller-supplied relative path"
        );
        // The canonicalized absolute form must be present (assuming the
        // CWD exists, which it always does for a running test process).
        let abs = std::env::current_dir()
            .expect("cwd must exist")
            .join(&relative);
        let key = cache_key_for(&abs);
        assert_eq!(
            cache.get(&key).cloned(),
            Some(result),
            "cache must be keyed on the canonicalized absolute path"
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_norm_case_windows_cache_key_canonicalizes_separators_and_trailing_slash() {
        // Variant inputs (mixed separators, trailing slash) should all share
        // a single cache entry once one of them populates it.
        // Use SystemRoot rather than hardcoding a drive letter, since
        // Windows can be installed elsewhere.
        let canonical = system_root_dir();
        let canonical_str = canonical
            .to_str()
            .expect("SystemRoot is expected to be valid UTF-8");
        let with_forward_slashes = PathBuf::from(canonical_str.replace('\\', "/"));
        let with_trailing_slash = {
            let mut s = canonical_str.to_string();
            s.push('\\');
            PathBuf::from(s)
        };

        // Pre-populate by calling norm_case on the canonical form.
        let expected = norm_case(&canonical);

        // All variants compute to the same cache key.
        let key_canonical = cache_key_for(&canonical);
        let key_forward = cache_key_for(&with_forward_slashes);
        let key_trailing = cache_key_for(&with_trailing_slash);
        assert_eq!(key_canonical, key_forward);
        assert_eq!(key_canonical, key_trailing);

        // And subsequent calls for the variants return the same cached result.
        assert_eq!(norm_case(&with_forward_slashes), expected);
        assert_eq!(norm_case(&with_trailing_slash), expected);
    }

    // ==================== resolve_any_symlink tests ====================

    #[test]
    fn test_resolve_any_symlink_nonexistent_path() {
        // Non-existent paths should return None
        let nonexistent = PathBuf::from("/this/path/does/not/exist/anywhere");
        assert_eq!(resolve_any_symlink(&nonexistent), None);
    }

    #[test]
    fn test_resolve_any_symlink_regular_file() {
        // Regular files (not symlinks) should return None
        use std::io::Write;
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("pet_test_regular_file.txt");

        // Create a regular file
        let mut file = std::fs::File::create(&test_file).expect("Failed to create test file");
        file.write_all(b"test").expect("Failed to write test file");

        // resolve_any_symlink should return None for regular files
        assert_eq!(resolve_any_symlink(&test_file), None);

        // Clean up
        let _ = std::fs::remove_file(&test_file);
    }

    #[test]
    fn test_resolve_any_symlink_directory() {
        // Directories (not symlinks) should return None
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join("pet_test_regular_dir");

        // Create a regular directory
        let _ = std::fs::create_dir(&test_dir);

        // resolve_any_symlink should return None for regular directories
        assert_eq!(resolve_any_symlink(&test_dir), None);

        // Clean up
        let _ = std::fs::remove_dir(&test_dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_resolve_any_symlink_unix_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = std::env::temp_dir();
        let target_file = temp_dir.join("pet_test_symlink_target.txt");
        let symlink_path = temp_dir.join("pet_test_symlink.txt");

        // Clean up any existing test files
        let _ = std::fs::remove_file(&target_file);
        let _ = std::fs::remove_file(&symlink_path);

        // Create target file
        std::fs::write(&target_file, "test").expect("Failed to create target file");

        // Create symlink
        symlink(&target_file, &symlink_path).expect("Failed to create symlink");

        // resolve_any_symlink should return the target path
        let result = resolve_any_symlink(&symlink_path);
        assert!(result.is_some(), "Should resolve symlink");

        let resolved = result.unwrap();
        // The resolved path should be canonicalized, so compare canonical forms
        let expected = std::fs::canonicalize(&target_file).unwrap();
        assert_eq!(resolved, expected);

        // Clean up
        let _ = std::fs::remove_file(&symlink_path);
        let _ = std::fs::remove_file(&target_file);
    }
}
