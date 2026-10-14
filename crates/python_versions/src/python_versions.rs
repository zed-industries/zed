//! Built-in Python version management.
//!
//! The IDE installs, lists, selects and removes Python versions itself, so a
//! machine with no Python can still create a virtual environment and run a
//! script. The versions are self-contained CPython builds from the
//! `python-build-standalone` project, unpacked into a folder of the IDE's own
//! (the *store*): nothing is installed on the machine and its `PATH` is left
//! alone.
//!
//! Unlike the built-in Node.js, whose manager is compiled into the IDE, every
//! version here is a download of some tens of megabytes from a third party.
//! So nothing is ever fetched as a side effect: [`list_available`] only reads
//! two small index files, and the download itself happens in [`install`],
//! which takes the [`Download`] the caller has shown the user and had
//! confirmed (what, how big, from where).
//!
//! One version is *active*, marked by the store's `current` link. A process
//! runs under it by having that link first on its `PATH` ([`live_bin_dirs`]),
//! so a terminal that is already open follows a switch. A project's own
//! virtual environment is a separate matter and takes precedence wherever the
//! caller says it does; this crate only knows about base interpreters.
//!
//! No GPUI or app-state dependency. The functions that touch the disk block,
//! and the `async` ones do so too between their network waits, so call them
//! all from a background thread.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use futures::AsyncReadExt as _;
use futures::io::Cursor;
use http_client::{AsyncBody, HttpClient, HttpRequestExt as _, Method, RedirectPolicy, Request};
use sha2::{Digest as _, Sha256};

/// Names the newest `python-build-standalone` release. A small file kept for
/// exactly this, so finding it costs no GitHub API call (and no rate limit).
const LATEST_RELEASE_URL: &str = "https://raw.githubusercontent.com/astral-sh/python-build-standalone/latest-release/latest-release.json";

/// Where the builds come from, as shown to the user before a download.
pub const SOURCE: &str = "github.com/astral-sh/python-build-standalone";

/// The kind of build installed: the interpreter and standard library only,
/// with debug symbols stripped. The smallest one that is a complete Python.
const FLAVOR: &str = "install_only_stripped.tar.gz";

/// One Python version the built-in manager has installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonVersion {
    /// The exact version, without a prefix: `3.13.16`.
    pub version: String,
    /// Whether this is the active version.
    pub current: bool,
    /// The folder it was unpacked into.
    pub dir: PathBuf,
    /// Its interpreter.
    pub python: PathBuf,
}

/// One version that can be installed on this machine, and everything needed
/// to ask the user about it and then fetch it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Download {
    /// The exact version: `3.13.16`.
    pub version: String,
    /// The archive's file name, which says what build it is.
    pub file_name: String,
    pub url: String,
    /// The archive's published SHA-256, checked after downloading.
    pub sha256: String,
    /// The download's size in bytes, once [`with_size`] has asked for it.
    pub size: Option<u64>,
}

/// Where the built-in manager keeps its Python versions: a `python_versions`
/// folder in the IDE's data directory.
pub fn store_dir() -> PathBuf {
    paths::data_dir().join("python_versions")
}

/// Whether `interpreter` is one of the built-in versions, as opposed to a
/// virtual environment's or the machine's own.
///
/// Such an interpreter is best named as plain `python` in a command the IDE
/// runs: the IDE puts the active version first on that command's `PATH`, and
/// the store sits under the user's profile, whose path can hold spaces that a
/// command line would have to quote.
pub fn is_built_in(store: &Path, interpreter: &Path) -> bool {
    let canonical = |path: &Path| std::fs::canonicalize(path).ok();
    match (canonical(interpreter), canonical(store)) {
        (Some(interpreter), Some(store)) => interpreter.starts_with(store),
        _ => interpreter.starts_with(store),
    }
}

fn versions_dir(store: &Path) -> PathBuf {
    store.join("versions")
}

/// The link that marks the active version: present and pointing at its
/// folder, or absent on the system Python.
fn current_link(store: &Path) -> PathBuf {
    store.join("current")
}

/// The interpreter inside an unpacked build. A Windows build has it at the
/// top; the others keep it in `bin`.
fn python_in(dir: &Path) -> PathBuf {
    if cfg!(windows) {
        dir.join("python.exe")
    } else {
        dir.join("bin").join("python3")
    }
}

/// Every installed version, newest first.
pub fn list(store: &Path) -> Result<Vec<PythonVersion>, String> {
    let entries = match std::fs::read_dir(versions_dir(store)) {
        Ok(entries) => entries,
        // Nothing has been installed yet.
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(format!("Couldn't read the Python versions: {error}")),
    };
    let active = std::fs::canonicalize(current_link(store)).ok();
    let mut versions: Vec<PythonVersion> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let dir = entry.path();
            let python = python_in(&dir);
            if !python.is_file() {
                return None;
            }
            let version = entry.file_name().to_str()?.to_string();
            let current = active.is_some() && std::fs::canonicalize(&dir).ok() == active;
            Some(PythonVersion {
                version,
                current,
                dir,
                python,
            })
        })
        .collect();
    versions.sort_by(|a, b| version_key(&b.version).cmp(&version_key(&a.version)));
    Ok(versions)
}

/// The active version, or `None` when the system Python is in use.
pub fn active(store: &Path) -> Option<PythonVersion> {
    list(store).ok()?.into_iter().find(|version| version.current)
}

/// Makes `version` the active one.
pub fn use_version(store: &Path, version: &str) -> Result<String, String> {
    let dir = versions_dir(store).join(version);
    if !python_in(&dir).is_file() {
        return Err(format!("Python {version} isn't installed"));
    }
    let link = current_link(store);
    remove_link(&link).map_err(|error| format!("Couldn't switch Python version: {error}"))?;
    create_link(&dir, &link).map_err(|error| format!("Couldn't switch Python version: {error}"))?;
    Ok(format!("Now using Python {version}"))
}

/// Makes no built-in version active, handing the choice back to whatever
/// `python` is on the `PATH`. The installed versions stay installed.
pub fn use_system(store: &Path) -> Result<String, String> {
    let link = current_link(store);
    if link.symlink_metadata().is_err() {
        return Ok("Already using the system Python".to_string());
    }
    remove_link(&link).map_err(|error| format!("Couldn't switch to the system Python: {error}"))?;
    Ok("Now using the system Python".to_string())
}

/// Removes `version`. Removing the active version leaves none active, which
/// falls back to the system Python.
///
/// A virtual environment created from this version stops working: it holds
/// the path of the interpreter it was made from. Callers should say so
/// before removing.
pub fn uninstall(store: &Path, version: &str) -> Result<String, String> {
    let dir = versions_dir(store).join(version);
    if !dir.is_dir() {
        return Err(format!("Python {version} isn't installed"));
    }
    let is_active = active(store).is_some_and(|active| active.version == version);
    if is_active {
        remove_link(&current_link(store))
            .map_err(|error| format!("Couldn't remove Python {version}: {error}"))?;
    }
    std::fs::remove_dir_all(&dir)
        .map_err(|error| format!("Couldn't remove Python {version}: {error}"))?;
    Ok(format!("Removed Python {version}"))
}

/// The folders to put first on a process's `PATH` for it to run the active
/// version: inside the store's `current` link, not the version's own folder.
///
/// The link is re-pointed on every switch, so a process that has these on
/// its `PATH` (a terminal that is already open) follows the switch. On the
/// system Python the link doesn't exist; entries that lead nowhere are
/// skipped, and the `python` further along the `PATH` is found.
///
/// Windows needs two: the interpreter is at the top of a build, and what
/// `pip install` adds as commands goes in `Scripts`. Elsewhere both are in
/// `bin`.
pub fn live_bin_dirs(store: &Path) -> Vec<PathBuf> {
    let link = current_link(store);
    if cfg!(windows) {
        vec![link.clone(), link.join("Scripts")]
    } else {
        vec![link.join("bin")]
    }
}

fn remove_link(link: &Path) -> std::io::Result<()> {
    if link.symlink_metadata().is_err() {
        return Ok(());
    }
    // A directory junction on Windows and a symlink elsewhere, which are
    // removed differently.
    if cfg!(windows) {
        std::fs::remove_dir(link)
    } else {
        std::fs::remove_file(link)
    }
}

#[cfg(windows)]
fn create_link(target: &Path, link: &Path) -> std::io::Result<()> {
    junction::create(target, link)
}

#[cfg(not(windows))]
fn create_link(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

/// The `python-build-standalone` name for the machine this is running on, or
/// `None` where there are no builds for it.
fn target_triple() -> Option<&'static str> {
    use std::env::consts::{ARCH, OS};
    Some(match (OS, ARCH) {
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        ("windows", "aarch64") => "aarch64-pc-windows-msvc",
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        _ => return None,
    })
}

/// Every version that can be installed on this machine, newest first: the
/// current patch release of each Python line in the newest release.
///
/// Reads two small index files and downloads no Python.
pub async fn list_available(http: Arc<dyn HttpClient>) -> Result<Vec<Download>, String> {
    let triple = target_triple()
        .ok_or_else(|| "There are no built-in Python builds for this kind of machine".to_string())?;
    let latest = get_bytes(&http, LATEST_RELEASE_URL).await?;
    let prefix = asset_url_prefix(&latest)?;
    let sums = get_bytes(&http, &format!("{prefix}/SHA256SUMS")).await?;
    let sums = String::from_utf8(sums)
        .map_err(|_| "The list of Python builds isn't readable".to_string())?;
    let downloads = parse_downloads(&sums, &prefix, triple);
    if downloads.is_empty() {
        return Err("The newest release has no Python builds for this machine".to_string());
    }
    Ok(downloads)
}

/// The folder a release's files are downloaded from, out of the
/// `latest-release.json` that names it.
fn asset_url_prefix(latest_release: &[u8]) -> Result<String, String> {
    let unreadable = || "The newest Python release couldn't be worked out".to_string();
    let json: serde_json::Value = serde_json::from_slice(latest_release).map_err(|_| unreadable())?;
    json.get("asset_url_prefix")
        .and_then(|prefix| prefix.as_str())
        .map(|prefix| prefix.trim_end_matches('/').to_string())
        .ok_or_else(unreadable)
}

/// The builds for `triple` in a release's `SHA256SUMS`, newest first.
///
/// Each line is a checksum and a file name such as
/// `cpython-3.13.16+20261009-x86_64-pc-windows-msvc-install_only_stripped.tar.gz`.
/// The suffix has to match exactly, which leaves out the other kinds of build
/// for the same machine (free-threaded, debug, the full source layout).
fn parse_downloads(sums: &str, prefix: &str, triple: &str) -> Vec<Download> {
    let suffix = format!("-{triple}-{FLAVOR}");
    let mut downloads: Vec<Download> = sums
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let sha256 = fields.next()?;
            let file_name = fields.next()?.trim_start_matches('*');
            let build = file_name.strip_prefix("cpython-")?.strip_suffix(&suffix)?;
            let (version, _release) = build.split_once('+')?;
            Some(Download {
                version: version.to_string(),
                file_name: file_name.to_string(),
                // `+` in a URL path is read as a space by some servers.
                url: format!("{prefix}/{}", file_name.replace('+', "%2B")),
                sha256: sha256.to_string(),
                size: None,
            })
        })
        .collect();
    downloads.sort_by(|a, b| version_key(&b.version).cmp(&version_key(&a.version)));
    downloads
}

/// Whether `version` is a pre-release of Python (`3.16.0a1`, `3.15.0rc2`):
/// published for testing, not for projects to depend on.
pub fn is_prerelease(version: &str) -> bool {
    !version_key(version).3
}

/// Orders versions: by their three numbers, and a pre-release (`3.15.0rc1`)
/// before the release it leads to.
fn version_key(version: &str) -> (u32, u32, u32, bool) {
    let mut numbers = [0u32; 3];
    let mut is_final = true;
    for (slot, part) in numbers.iter_mut().zip(version.split('.')) {
        let digits: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        *slot = digits.parse().unwrap_or(0);
        if digits.len() != part.len() {
            is_final = false;
        }
    }
    (numbers[0], numbers[1], numbers[2], is_final)
}

/// `download` with its size filled in, for the question put to the user
/// before fetching it. Asks the server for the file's headers only. When
/// that fails the size stays unknown, which the question can say.
pub async fn with_size(http: Arc<dyn HttpClient>, mut download: Download) -> Download {
    let request = Request::builder()
        .method(Method::HEAD)
        .uri(download.url.as_str())
        .follow_redirects(RedirectPolicy::FollowAll)
        .body(AsyncBody::default());
    let Ok(request) = request else {
        return download;
    };
    if let Ok(response) = http.send(request).await {
        download.size = response
            .headers()
            .get("content-length")
            .and_then(|length| length.to_str().ok())
            .and_then(|length| length.parse().ok())
            .filter(|_| response.status().is_success());
    }
    download
}

/// Downloads, checks and unpacks `download`. The first version installed
/// becomes the active one.
///
/// This is the only function here that fetches a Python. Call it once the
/// user has agreed to this particular download.
pub async fn install(
    store: &Path,
    download: &Download,
    http: Arc<dyn HttpClient>,
) -> Result<PythonVersion, String> {
    let version = &download.version;
    let target = versions_dir(store).join(version);
    if target.exists() {
        return Err(format!("Python {version} is already installed"));
    }

    log::info!("Downloading Python {version} from {}", download.url);
    let archive = get_bytes(&http, &download.url).await?;
    let checksum = format!("{:x}", Sha256::digest(&archive));
    if !checksum.eq_ignore_ascii_case(&download.sha256) {
        return Err(format!(
            "The download of Python {version} doesn't match its published checksum, so it wasn't installed"
        ));
    }

    // Unpacked beside the versions and moved into place whole, so a version
    // folder is never a half-unpacked one.
    let staging = store.join(".downloads").join(version);
    let _ = std::fs::remove_dir_all(&staging);
    let failed = |error: std::io::Error| format!("Couldn't install Python {version}: {error}");
    std::fs::create_dir_all(&staging).map_err(failed)?;
    let unpacked = unpack(archive, &staging).await.and_then(|()| {
        // Every build unpacks to a single `python` folder.
        let python_dir = staging.join("python");
        if !python_in(&python_dir).is_file() {
            return Err(std::io::Error::other("the download didn't hold a Python"));
        }
        std::fs::create_dir_all(versions_dir(store))?;
        std::fs::rename(&python_dir, &target)
    });
    let _ = std::fs::remove_dir_all(&staging);
    unpacked.map_err(failed)?;

    if active(store).is_none() {
        use_version(store, version)?;
    }
    list(store)?
        .into_iter()
        .find(|installed| &installed.version == version)
        .ok_or_else(|| format!("Python {version} was unpacked but can't be found"))
}

async fn unpack(archive: Vec<u8>, into: &Path) -> std::io::Result<()> {
    Archive::new(GzipDecoder::new(Cursor::new(archive)))
        .unpack(into)
        .await
}

async fn get_bytes(http: &Arc<dyn HttpClient>, url: &str) -> Result<Vec<u8>, String> {
    let mut response = http
        .get(url, AsyncBody::default(), true)
        .await
        .map_err(|error| format!("Couldn't reach {SOURCE}: {error}"))?;
    if !response.status().is_success() {
        return Err(format!("{SOURCE} answered {} for {url}", response.status()));
    }
    let mut bytes = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut bytes)
        .await
        .map_err(|error| format!("The download from {SOURCE} was cut short: {error}"))?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_client::{FakeHttpClient, Response};

    const PREFIX: &str = "https://example.test/releases/download/20261009";

    /// A gzip-compressed tar holding `python/python.exe`, `python/bin/python3`
    /// and an empty `python/Scripts/.empty`: the shape of a real build on
    /// every platform, in 156 bytes.
    const ARCHIVE: &[u8] = &[
        0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xed, 0xd4,
        0x41, 0x0a, 0xc2, 0x30, 0x10, 0x40, 0xd1, 0x59, 0x7b, 0x0a, 0x4f, 0xd0,
        0x24, 0xc6, 0xe8, 0x41, 0x3c, 0x81, 0x95, 0x80, 0x5d, 0xa8, 0xa1, 0x99,
        0x82, 0xbd, 0xbd, 0xa1, 0x45, 0x90, 0xae, 0x5c, 0x48, 0x10, 0xfa, 0xdf,
        0x66, 0x86, 0xd9, 0x64, 0xf5, 0x93, 0x46, 0xbd, 0x3e, 0xee, 0x26, 0x4d,
        0xa3, 0x89, 0xcf, 0x28, 0xbf, 0x67, 0x8b, 0x63, 0x08, 0xd3, 0x2c, 0x96,
        0xb3, 0x08, 0x1f, 0x7b, 0xb9, 0x3b, 0xe7, 0x9d, 0x95, 0xad, 0x95, 0x0a,
        0x86, 0xac, 0xe7, 0xbe, 0x3c, 0x2f, 0xeb, 0x94, 0x75, 0x68, 0x37, 0x82,
        0xb5, 0x9a, 0xc3, 0x37, 0x6d, 0xf7, 0xfe, 0x03, 0xfc, 0x7f, 0xf4, 0x7f,
        0xf0, 0xf4, 0x4f, 0xff, 0xa8, 0xd4, 0xff, 0xe9, 0xd2, 0x77, 0x49, 0xb3,
        0x69, 0xe2, 0x2d, 0xe9, 0x58, 0xbd, 0xff, 0xc5, 0xee, 0x76, 0x76, 0x1f,
        0xe8, 0x1f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, 0x6f, 0xbd, 0x00, 0x29, 0xd8, 0xba, 0x7c, 0x00, 0x28, 0x00, 0x00,
    ];
    const ARCHIVE_SHA256: &str = "19474d22f21bd2fb860fc18ed8c65ff5bca90416a0f42283c665006e9827b1f9";

    /// A store holding `versions`, each as a folder with just an interpreter.
    fn store(versions: &[&str]) -> tempfile::TempDir {
        let store = tempfile::tempdir().unwrap();
        for version in versions {
            let python = python_in(&versions_dir(store.path()).join(version));
            std::fs::create_dir_all(python.parent().unwrap()).unwrap();
            std::fs::write(python, "stub").unwrap();
        }
        store
    }

    fn names(versions: &[PythonVersion]) -> Vec<&str> {
        versions.iter().map(|v| v.version.as_str()).collect()
    }

    fn download(version: &str, sha256: &str) -> Download {
        Download {
            version: version.to_string(),
            file_name: format!("cpython-{version}.tar.gz"),
            url: format!("{PREFIX}/cpython-{version}.tar.gz"),
            sha256: sha256.to_string(),
            size: None,
        }
    }

    /// Serves the archive for any URL, and counts the requests made.
    fn serving_the_archive() -> (Arc<dyn HttpClient>, Arc<std::sync::atomic::AtomicUsize>) {
        let requests = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter = requests.clone();
        let http = FakeHttpClient::create(move |_request| {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async move {
                Ok(Response::builder()
                    .status(200)
                    .body(AsyncBody::from(ARCHIVE.to_vec()))
                    .unwrap())
            }
        });
        (http, requests)
    }

    #[test]
    fn the_builds_for_this_machine_are_picked_out_newest_first() {
        let sums = "\
aaa  cpython-3.12.15+20261009-x86_64-pc-windows-msvc-install_only_stripped.tar.gz
bbb  cpython-3.13.16+20261009-x86_64-pc-windows-msvc-install_only_stripped.tar.gz
ccc  cpython-3.13.16+20261009-x86_64-pc-windows-msvc-install_only.tar.gz
ddd  cpython-3.13.16+20261009-x86_64-pc-windows-msvc-freethreaded-install_only_stripped.tar.gz
eee  cpython-3.13.16+20261009-x86_64-unknown-linux-gnu-install_only_stripped.tar.gz
fff  cpython-3.15.0rc2+20261009-x86_64-pc-windows-msvc-install_only_stripped.tar.gz
ggg  cpython-3.9.25+20261009-x86_64-pc-windows-msvc-install_only_stripped.tar.gz
hhh  SHA256SUMS
";
        let downloads = parse_downloads(sums, PREFIX, "x86_64-pc-windows-msvc");
        let versions: Vec<&str> = downloads.iter().map(|d| d.version.as_str()).collect();
        assert_eq!(versions, vec!["3.15.0rc2", "3.13.16", "3.12.15", "3.9.25"]);

        let python_313 = &downloads[1];
        assert_eq!(python_313.sha256, "bbb");
        assert_eq!(
            python_313.url,
            format!(
                "{PREFIX}/cpython-3.13.16%2B20261009-x86_64-pc-windows-msvc-install_only_stripped.tar.gz"
            )
        );
        assert_eq!(python_313.size, None);

        // Another machine gets its own build, and one with none gets nothing.
        let linux = parse_downloads(sums, PREFIX, "x86_64-unknown-linux-gnu");
        assert_eq!(linux.len(), 1);
        assert_eq!(linux[0].sha256, "eee");
        assert!(parse_downloads(sums, PREFIX, "riscv64-unknown-linux-gnu").is_empty());
    }

    #[test]
    fn versions_order_by_number_with_a_prerelease_before_its_release() {
        let mut versions = vec!["3.9.25", "3.15.0", "3.10.2", "3.15.0rc2", "3.13.16"];
        versions.sort_by_key(|version| version_key(version));
        assert_eq!(versions, vec!["3.9.25", "3.10.2", "3.13.16", "3.15.0rc2", "3.15.0"]);

        for prerelease in ["3.16.0a1", "3.15.0b3", "3.15.0rc2"] {
            assert!(is_prerelease(prerelease), "{prerelease}");
        }
        for release in ["3.13.16", "3.15.0", "3.9.25"] {
            assert!(!is_prerelease(release), "{release}");
        }
    }

    #[test]
    fn the_release_folder_is_read_from_the_latest_release_file() {
        let json = br#"{"version": 1, "tag": "20261009", "asset_url_prefix": "https://example.test/releases/download/20261009/"}"#;
        assert_eq!(asset_url_prefix(json).unwrap(), PREFIX);
        assert!(asset_url_prefix(b"{}").is_err());
        assert!(asset_url_prefix(b"not json").is_err());
    }

    #[test]
    fn lists_newest_first_and_marks_the_active_version() {
        let store = store(&["3.9.25", "3.13.16", "3.12.15"]);
        let versions = list(store.path()).unwrap();
        assert_eq!(names(&versions), vec!["3.13.16", "3.12.15", "3.9.25"]);
        assert!(versions.iter().all(|v| !v.current));
        assert_eq!(active(store.path()), None);

        use_version(store.path(), "3.12.15").unwrap();
        let versions = list(store.path()).unwrap();
        assert!(versions[1].current);
        assert_eq!(active(store.path()).unwrap().version, "3.12.15");

        assert!(use_version(store.path(), "3.11.0").is_err());
        assert_eq!(active(store.path()).unwrap().version, "3.12.15");
    }

    #[test]
    fn a_built_in_interpreter_is_told_apart_from_others() {
        let store = store(&["3.13.16"]);
        use_version(store.path(), "3.13.16").unwrap();
        let installed = active(store.path()).unwrap();
        assert!(is_built_in(store.path(), &installed.python));
        // Reached through the active-version link, it is still built-in.
        assert!(is_built_in(store.path(), &python_in(&current_link(store.path()))));

        let elsewhere = tempfile::tempdir().unwrap();
        let venv_python = python_in(&elsewhere.path().join("venv"));
        std::fs::create_dir_all(venv_python.parent().unwrap()).unwrap();
        std::fs::write(&venv_python, "stub").unwrap();
        assert!(!is_built_in(store.path(), &venv_python));
        assert!(!is_built_in(store.path(), Path::new("python")));
    }

    #[test]
    fn the_store_is_in_the_ides_data_folder() {
        assert!(store_dir().starts_with(paths::data_dir()));
    }

    #[test]
    fn an_empty_store_lists_nothing() {
        let store = tempfile::tempdir().unwrap();
        assert!(list(store.path()).unwrap().is_empty());
        assert_eq!(active(store.path()), None);
        use_system(store.path()).unwrap();
    }

    /// The folders on a process's `PATH` are the same ones whichever version
    /// is active, and lead to that version: that is what lets a terminal
    /// that is already open follow a switch.
    #[test]
    fn the_path_entries_follow_the_active_version() {
        let store = store(&["3.12.15", "3.13.16"]);
        let entries = live_bin_dirs(store.path());
        let link = current_link(store.path());
        assert!(entries.iter().all(|entry| entry.starts_with(&link)));
        // On the system Python they lead nowhere, and are skipped.
        assert!(!link.exists());

        use_version(store.path(), "3.12.15").unwrap();
        assert_eq!(live_bin_dirs(store.path()), entries);
        assert!(python_in(&link).is_file());
        let target = std::fs::canonicalize(&link).unwrap();
        assert!(target.ends_with("3.12.15"));

        use_version(store.path(), "3.13.16").unwrap();
        let target = std::fs::canonicalize(&link).unwrap();
        assert!(target.ends_with("3.13.16"));

        use_system(store.path()).unwrap();
        assert!(!link.exists());
        assert_eq!(live_bin_dirs(store.path()), entries);
        // Both versions are still installed.
        assert_eq!(names(&list(store.path()).unwrap()), vec!["3.13.16", "3.12.15"]);
    }

    #[test]
    fn uninstalling_the_active_version_falls_back_to_the_system_python() {
        let store = store(&["3.12.15", "3.13.16"]);
        use_version(store.path(), "3.13.16").unwrap();

        uninstall(store.path(), "3.12.15").unwrap();
        assert_eq!(active(store.path()).unwrap().version, "3.13.16");

        uninstall(store.path(), "3.13.16").unwrap();
        assert!(list(store.path()).unwrap().is_empty());
        assert_eq!(active(store.path()), None);
        assert!(!current_link(store.path()).exists());
        assert!(uninstall(store.path(), "3.13.16").is_err());
    }

    #[test]
    fn installing_unpacks_a_checked_download_and_makes_the_first_one_active() {
        let store = tempfile::tempdir().unwrap();
        let (http, requests) = serving_the_archive();

        let installed = smol::block_on(install(
            store.path(),
            &download("3.13.16", ARCHIVE_SHA256),
            http.clone(),
        ))
        .unwrap();
        assert_eq!(installed.version, "3.13.16");
        assert!(installed.current);
        assert!(installed.python.is_file());
        assert_eq!(installed.dir, versions_dir(store.path()).join("3.13.16"));
        // Nothing is left behind from unpacking.
        assert!(!store.path().join(".downloads").join("3.13.16").exists());
        assert_eq!(requests.load(std::sync::atomic::Ordering::SeqCst), 1);

        // A second version is installed beside it and doesn't take over.
        let second = smol::block_on(install(
            store.path(),
            &download("3.12.15", ARCHIVE_SHA256),
            http.clone(),
        ))
        .unwrap();
        assert!(!second.current);
        assert_eq!(active(store.path()).unwrap().version, "3.13.16");

        // Installing what is already there is refused before any download.
        let again = smol::block_on(install(
            store.path(),
            &download("3.13.16", ARCHIVE_SHA256),
            http,
        ));
        assert!(again.is_err());
        assert_eq!(requests.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[test]
    fn a_download_that_fails_its_checksum_is_not_installed() {
        let store = tempfile::tempdir().unwrap();
        let (http, _requests) = serving_the_archive();
        let wrong = "0".repeat(64);

        let error = smol::block_on(install(store.path(), &download("3.13.16", &wrong), http))
            .unwrap_err();
        assert!(error.contains("checksum"), "{error}");
        assert!(list(store.path()).unwrap().is_empty());
        assert!(!versions_dir(store.path()).join("3.13.16").exists());
    }

    #[test]
    fn asking_for_the_size_downloads_nothing() {
        let http = FakeHttpClient::create(|request| async move {
            assert_eq!(request.method(), Method::HEAD);
            Ok(Response::builder()
                .status(200)
                .header("content-length", "21987654")
                .body(AsyncBody::default())
                .unwrap())
        });
        let sized = smol::block_on(with_size(http, download("3.13.16", ARCHIVE_SHA256)));
        assert_eq!(sized.size, Some(21_987_654));

        // A server that won't say leaves the size unknown, not wrong.
        let http = FakeHttpClient::with_404_response();
        let unsized_ = smol::block_on(with_size(http, download("3.13.16", ARCHIVE_SHA256)));
        assert_eq!(unsized_.size, None);
    }
}
