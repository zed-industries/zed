#![allow(
    clippy::disallowed_methods,
    reason = "We are not in an async environment, so std::process::Command is fine"
)]
#![cfg_attr(
    any(target_os = "linux", target_os = "freebsd", target_os = "windows"),
    allow(dead_code)
)]

mod completions;

use crate::completions::Shell;

use anyhow::{Context as _, Result};
use clap::{CommandFactory, Parser};
use cli::{AgentAction, CliRequest, CliResponse, IpcHandshake, ipc::IpcOneShotServer};
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::ExitStatus,
    sync::Arc,
    thread::{self, JoinHandle},
};
use tempfile::{NamedTempFile, TempDir};
use util::paths::PathWithPosition;
use walkdir::WalkDir;

use std::io::IsTerminal;

const URL_PREFIX: [&'static str; 5] = ["zed://", "http://", "https://", "file://", "ssh://"];

struct Detect;

trait InstalledApp {
    fn zed_version_string(&self) -> String;
    fn launch(&self, ipc_url: String, user_data_dir: Option<&str>) -> anyhow::Result<()>;
    fn run_foreground(
        &self,
        ipc_url: String,
        user_data_dir: Option<&str>,
    ) -> io::Result<ExitStatus>;
    fn path(&self) -> PathBuf;
}

#[derive(Parser, Debug)]
#[command(
    name = "zed",
    disable_version_flag = true,
    before_help = "The Zed CLI binary.
This CLI is a separate binary that invokes Zed.

Examples:
    `zed`
          Simply opens Zed
    `zed --foreground`
          Runs in foreground (shows all logs)
    `zed path-to-your-project`
          Open your project in Zed
    `zed -n path-to-file `
          Open file/folder in a new window",
    after_help = "To read from stdin, append '-', e.g. 'ps axf | zed -'"
)]
struct Args {
    /// Wait for all of the given paths to be opened/closed before exiting.
    ///
    /// When opening a directory, waits until the created window is closed.
    #[arg(short, long)]
    wait: bool,
    /// Add files to the currently open workspace
    #[arg(short, long, overrides_with_all = ["new", "reuse", "existing", "classic"])]
    add: bool,
    /// Create a new workspace
    #[arg(short, long, overrides_with_all = ["add", "reuse", "existing", "classic"])]
    new: bool,
    /// Reuse an existing window, replacing its workspace
    #[arg(short, long, overrides_with_all = ["add", "new", "existing", "classic"], hide = true)]
    reuse: bool,
    /// Open in existing Zed window
    #[arg(short = 'e', long = "existing", overrides_with_all = ["add", "new", "reuse", "classic"])]
    existing: bool,
    /// Use the classic open behavior: new window for directories, reuse for files
    #[arg(long, hide = true, overrides_with_all = ["add", "new", "reuse", "existing"])]
    classic: bool,
    /// Sets a custom directory for all user data (e.g., database, extensions, logs).
    /// This overrides the default platform-specific data directory location:
    #[cfg_attr(target_os = "macos", doc = "`~/Library/Application Support/Zed`.")]
    #[cfg_attr(target_os = "windows", doc = "`%LOCALAPPDATA%\\Zed`.")]
    #[cfg_attr(
        not(any(target_os = "windows", target_os = "macos")),
        doc = "`$XDG_DATA_HOME/zed`."
    )]
    #[arg(long, value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    user_data_dir: Option<String>,
    /// The paths to open in Zed (space-separated).
    ///
    /// Use `path:line:column` syntax to open a file at the given line and column.
    #[arg(trailing_var_arg = true, value_hint = clap::ValueHint::AnyPath)]
    paths_with_position: Vec<String>,
    /// Print Zed's version and the app path.
    #[arg(short, long)]
    version: bool,
    /// Run zed in the foreground (useful for debugging)
    #[arg(long)]
    foreground: bool,
    /// Custom path to Zed.app or the zed binary
    #[arg(long)]
    zed: Option<PathBuf>,
    /// Run zed in dev-server mode
    #[arg(long)]
    dev_server_token: Option<String>,
    /// The username and WSL distribution to use when opening paths. If not specified,
    /// Zed will attempt to open the paths directly.
    ///
    /// The username is optional, and if not specified, the default user for the distribution
    /// will be used.
    ///
    /// Example: `me@Ubuntu` or `Ubuntu`.
    ///
    /// WARN: You should not fill in this field by hand.
    #[cfg(target_os = "windows")]
    #[arg(long, value_name = "USER@DISTRO")]
    wsl: Option<String>,
    /// Not supported in Zed CLI, only supported on Zed binary
    /// Will attempt to give the correct command to run
    #[arg(long)]
    system_specs: bool,
    /// Open the project in a dev container.
    ///
    /// Automatically triggers "Reopen in Dev Container" if a `.devcontainer/`
    /// configuration is found in the project directory.
    #[arg(long)]
    dev_container: bool,
    /// Pairs of file paths to diff. Can be specified multiple times.
    /// When directories are provided, recurses into them and shows all changed files in a single multi-diff view.
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["OLD_PATH", "NEW_PATH"], value_hint = clap::ValueHint::AnyPath)]
    diff: Vec<String>,
    /// Send a prompt to the Zed agent in a running instance
    #[arg(long)]
    agent: bool,
    /// List agent threads instead of sending a prompt
    #[arg(long)]
    agent_list: bool,
    /// Target an existing thread by id, given either in full or as a unique
    /// leading fragment of one. Hyphens are optional, so `550e8400-e29b` and
    /// `550e8400e29b` select the same thread. Use `--agent-list` to see ids.
    #[arg(long, value_name = "ID")]
    agent_thread: Option<String>,
    /// Target an existing thread by ACP session id
    #[arg(long, value_name = "SESSION_ID")]
    agent_session: Option<String>,
    /// Scope the thread lookup to a project directory, defaulting to the working directory
    #[arg(long, value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    agent_project: Option<PathBuf>,
    /// Always start a new agent thread and print its id
    #[arg(long)]
    agent_new: bool,
    /// Wait for the agent turn to finish before exiting
    #[arg(long)]
    agent_wait: bool,
    /// Agent profile for a new thread, which decides the available tools.
    /// Requires `--agent-new`
    #[arg(long, value_name = "PROFILE")]
    agent_profile: Option<String>,
    /// Model for a new thread, given as `provider/model-id`. Requires
    /// `--agent-new`
    #[arg(long, value_name = "PROVIDER/MODEL")]
    agent_model: Option<String>,
    /// Output format for `--agent-list`
    #[arg(long, value_name = "FORMAT", default_value_t = AgentListFormat::Text)]
    agent_list_format: AgentListFormat,
    /// Generate shell completions for Zed
    #[arg(long, value_names = ["SHELL"])]
    completions: Option<Shell>,
    /// Uninstall Zed from user system
    #[cfg(all(
        any(target_os = "linux", target_os = "macos"),
        not(feature = "no-bundled-uninstall")
    ))]
    #[arg(long)]
    uninstall: bool,

    /// Used for SSH/Git password authentication, to remove the need for netcat as a dependency,
    /// by having Zed act like netcat communicating over a Unix socket.
    #[arg(long, hide = true)]
    askpass: Option<String>,
}

/// Output format for `--agent-list`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
#[non_exhaustive]
#[value(rename_all = "lower")]
enum AgentListFormat {
    /// One thread per line, for reading in a terminal.
    Text,
    /// A JSON array, for scripts that would otherwise parse columns.
    Json,
}

impl std::fmt::Display for AgentListFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Text => "text",
            Self::Json => "json",
        };
        formatter.write_str(name)
    }
}

/// Parses a path containing a position (e.g. `path:line:column`)
/// and returns its canonicalized string representation.
///
/// If a part of path doesn't exist, it will canonicalize the
/// existing part and append the non-existing part.
///
/// This method must return an absolute path, as many zed
/// crates assume absolute paths.
fn parse_path_with_position(argument_str: &str) -> anyhow::Result<String> {
    match Path::new(argument_str).canonicalize() {
        Ok(existing_path) => Ok(PathWithPosition::from_path(existing_path)),
        Err(_) => PathWithPosition::parse_str(argument_str).map_path(|mut path| {
            let curdir = env::current_dir().context("retrieving current directory")?;
            let mut children = Vec::new();
            let root;
            loop {
                // canonicalize handles './', and '/'.
                if let Ok(canonicalized) = fs::canonicalize(&path) {
                    root = canonicalized;
                    break;
                }
                // The comparison to `curdir` is just a shortcut
                // since we know it is canonical. The other one
                // is if `argument_str` is a string that starts
                // with a name (e.g. "foo/bar").
                if path == curdir || path == Path::new("") {
                    root = curdir;
                    break;
                }
                children.push(
                    path.file_name()
                        .with_context(|| format!("parsing as path with position {argument_str}"))?
                        .to_owned(),
                );
                if !path.pop() {
                    unreachable!("parsing as path with position {argument_str}");
                }
            }
            Ok(children.iter().rev().fold(root, |mut path, child| {
                path.push(child);
                path
            }))
        }),
    }
    .map(|path_with_pos| path_with_pos.to_string(&|path| path.to_string_lossy().into_owned()))
}

/// Returns whether a `--diff` argument refers to an existing path, allowing a
/// trailing `:line:column` suffix (parsed later by the Zed side, matching how
/// regular `zed path:line:column` arguments are handled).
fn diff_path_exists(diff_path: &str) -> bool {
    Path::new(diff_path).exists() || PathWithPosition::parse_str(diff_path).path.exists()
}

fn expand_directory_diff_pairs(
    diff_pairs: Vec<[String; 2]>,
) -> anyhow::Result<(Vec<[String; 2]>, Vec<TempDir>)> {
    let mut expanded = Vec::new();
    let mut temp_dirs = Vec::new();

    for pair in diff_pairs {
        let left = PathBuf::from(&pair[0]);
        let right = PathBuf::from(&pair[1]);

        if left.is_dir() && right.is_dir() {
            let (mut pairs, temp_dir) = expand_directory_pair(&left, &right)?;
            expanded.append(&mut pairs);
            if let Some(temp_dir) = temp_dir {
                temp_dirs.push(temp_dir);
            }
        } else {
            expanded.push(pair);
        }
    }

    Ok((expanded, temp_dirs))
}

fn expand_directory_pair(
    left: &Path,
    right: &Path,
) -> anyhow::Result<(Vec<[String; 2]>, Option<TempDir>)> {
    let left_files = collect_files(left)?;
    let right_files = collect_files(right)?;

    let mut rel_paths = BTreeSet::new();
    rel_paths.extend(left_files.keys().cloned());
    rel_paths.extend(right_files.keys().cloned());

    let mut temp_dir = TempDir::new()?;
    let mut temp_dir_used = false;
    let mut pairs = Vec::new();

    for rel in rel_paths {
        match (left_files.get(&rel), right_files.get(&rel)) {
            (Some(left_path), Some(right_path)) => {
                pairs.push([
                    left_path.to_string_lossy().into_owned(),
                    right_path.to_string_lossy().into_owned(),
                ]);
            }
            (Some(left_path), None) => {
                let stub = create_empty_stub(&mut temp_dir, &rel)?;
                temp_dir_used = true;
                pairs.push([
                    left_path.to_string_lossy().into_owned(),
                    stub.to_string_lossy().into_owned(),
                ]);
            }
            (None, Some(right_path)) => {
                let stub = create_empty_stub(&mut temp_dir, &rel)?;
                temp_dir_used = true;
                pairs.push([
                    stub.to_string_lossy().into_owned(),
                    right_path.to_string_lossy().into_owned(),
                ]);
            }
            (None, None) => {}
        }
    }

    let temp_dir = if temp_dir_used { Some(temp_dir) } else { None };
    Ok((pairs, temp_dir))
}

fn collect_files(root: &Path) -> anyhow::Result<BTreeMap<PathBuf, PathBuf>> {
    let mut files = BTreeMap::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let rel = entry
                .path()
                .strip_prefix(root)
                .context("stripping directory prefix")?
                .to_path_buf();
            files.insert(rel, entry.into_path());
        }
    }

    Ok(files)
}

fn create_empty_stub(temp_dir: &mut TempDir, rel: &Path) -> anyhow::Result<PathBuf> {
    let stub_path = temp_dir.path().join(rel);
    if let Some(parent) = stub_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(&stub_path)?;
    Ok(stub_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use util::path;
    use util::paths::SanitizedPath;
    use util::test::TempTree;

    macro_rules! assert_path_eq {
        ($left:expr, $right:expr) => {
            assert_eq!(
                SanitizedPath::new(Path::new(&$left)),
                SanitizedPath::new(Path::new(&$right))
            )
        };
    }

    fn cwd() -> PathBuf {
        env::current_dir().unwrap()
    }

    static CWD_LOCK: Mutex<()> = Mutex::new(());

    fn with_cwd<T>(path: &Path, f: impl FnOnce() -> anyhow::Result<T>) -> anyhow::Result<T> {
        let _lock = CWD_LOCK.lock();
        let old_cwd = cwd();
        env::set_current_dir(path)?;
        let result = f();
        env::set_current_dir(old_cwd)?;
        result
    }

    #[test]
    fn test_parse_non_existing_path() {
        // Absolute path
        let result = parse_path_with_position(path!("/non/existing/path.txt")).unwrap();
        assert_path_eq!(result, path!("/non/existing/path.txt"));

        // Absolute path in cwd
        let path = cwd().join(path!("non/existing/path.txt"));
        let expected = path.to_string_lossy().to_string();
        let result = parse_path_with_position(&expected).unwrap();
        assert_path_eq!(result, expected);

        // Relative path
        let result = parse_path_with_position(path!("non/existing/path.txt")).unwrap();
        assert_path_eq!(result, expected)
    }

    #[test]
    fn test_parse_existing_path() {
        let temp_tree = TempTree::new(json!({
            "file.txt": "",
        }));
        let file_path = temp_tree.path().join("file.txt");
        let expected = file_path.to_string_lossy().to_string();

        // Absolute path
        let result = parse_path_with_position(file_path.to_str().unwrap()).unwrap();
        assert_path_eq!(result, expected);

        // Relative path
        let result = with_cwd(temp_tree.path(), || parse_path_with_position("file.txt")).unwrap();
        assert_path_eq!(result, expected);
    }

    // NOTE:
    // While POSIX symbolic links are somewhat supported on Windows, they are an opt in by the user, and thus
    // we assume that they are not supported out of the box.
    #[cfg(not(windows))]
    #[test]
    fn test_parse_symlink_file() {
        let temp_tree = TempTree::new(json!({
            "target.txt": "",
        }));
        let target_path = temp_tree.path().join("target.txt");
        let symlink_path = temp_tree.path().join("symlink.txt");
        std::os::unix::fs::symlink(&target_path, &symlink_path).unwrap();

        // Absolute path
        let result = parse_path_with_position(symlink_path.to_str().unwrap()).unwrap();
        assert_eq!(result, target_path.to_string_lossy());

        // Relative path
        let result =
            with_cwd(temp_tree.path(), || parse_path_with_position("symlink.txt")).unwrap();
        assert_eq!(result, target_path.to_string_lossy());
    }

    #[cfg(not(windows))]
    #[test]
    fn test_parse_symlink_dir() {
        let temp_tree = TempTree::new(json!({
            "some": {
                "dir": { // symlink target
                    "ec": {
                        "tory": {
                            "file.txt": "",
        }}}}}));

        let target_file_path = temp_tree.path().join("some/dir/ec/tory/file.txt");
        let expected = target_file_path.to_string_lossy();

        let dir_path = temp_tree.path().join("some/dir");
        let symlink_path = temp_tree.path().join("symlink");
        std::os::unix::fs::symlink(&dir_path, &symlink_path).unwrap();

        // Absolute path
        let result =
            parse_path_with_position(symlink_path.join("ec/tory/file.txt").to_str().unwrap())
                .unwrap();
        assert_eq!(result, expected);

        // Relative path
        let result = with_cwd(temp_tree.path(), || {
            parse_path_with_position("symlink/ec/tory/file.txt")
        })
        .unwrap();
        assert_eq!(result, expected);
    }

    fn make_args() -> Args {
        Args {
            wait: false,
            add: false,
            new: false,
            reuse: false,
            existing: false,
            classic: false,
            user_data_dir: None,
            paths_with_position: vec![],
            version: false,
            foreground: false,
            zed: None,
            dev_server_token: None,
            #[cfg(target_os = "windows")]
            wsl: None,
            system_specs: false,
            dev_container: false,
            diff: vec![],
            agent: false,
            agent_list: false,
            agent_thread: None,
            agent_session: None,
            agent_project: None,
            agent_new: false,
            agent_wait: false,
            agent_list_format: AgentListFormat::Text,
            agent_profile: None,
            agent_model: None,
            completions: None,
            #[cfg(all(
                any(target_os = "linux", target_os = "macos"),
                not(feature = "no-bundled-uninstall")
            ))]
            uninstall: false,
            askpass: None,
        }
    }

    #[test]
    fn test_validate_agent_no_flags_is_ok() {
        let args = make_args();
        assert!(validate_agent_args(&args).is_ok());
    }

    #[test]
    fn test_validate_agent_thread_without_agent_is_error() {
        let args = Args {
            agent_thread: Some("abc123".to_string()),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_session_without_agent_is_error() {
        let args = Args {
            agent_session: Some("session-1".to_string()),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_new_without_agent_is_error() {
        let args = Args {
            agent_new: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_wait_without_agent_is_error() {
        let args = Args {
            agent_wait: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_list_with_prompt_is_error() {
        let args = Args {
            agent_list: true,
            paths_with_position: vec!["some prompt".to_string()],
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_list_with_new_is_error() {
        let args = Args {
            agent_list: true,
            agent_new: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_list_with_wait_is_error() {
        let args = Args {
            agent_list: true,
            agent_wait: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_list_with_thread_is_error() {
        let args = Args {
            agent_list: true,
            agent_thread: Some("abc".into()),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_list_with_session_is_error() {
        let args = Args {
            agent_list: true,
            agent_session: Some("sess1".into()),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_thread_and_session_together_is_error() {
        let args = Args {
            agent: true,
            paths_with_position: vec!["hello".into()],
            agent_thread: Some("abc".into()),
            agent_session: Some("sess1".into()),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_new_with_thread_is_error() {
        let args = Args {
            agent: true,
            paths_with_position: vec!["hello".into()],
            agent_new: true,
            agent_thread: Some("abc".into()),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_new_with_session_is_error() {
        let args = Args {
            agent: true,
            paths_with_position: vec!["hello".into()],
            agent_new: true,
            agent_session: Some("sess1".into()),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_list_format_without_agent_list_is_error() {
        let without_agent_flags = Args {
            agent_list_format: AgentListFormat::Json,
            ..make_args()
        };
        assert!(validate_agent_args(&without_agent_flags).is_err());

        let with_prompt = Args {
            agent: true,
            paths_with_position: vec!["hello".to_string()],
            agent_list_format: AgentListFormat::Json,
            ..make_args()
        };
        assert!(validate_agent_args(&with_prompt).is_err());
    }

    #[test]
    fn test_validate_agent_list_with_json_format_is_ok() {
        let args = Args {
            agent_list: true,
            agent_list_format: AgentListFormat::Json,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_ok());
    }

    #[test]
    fn test_validate_agent_profile_and_model_without_agent_or_list_are_errors() {
        for mutate in [
            (|args: &mut Args| args.agent_profile = Some("write".to_string())) as fn(&mut Args),
            |args: &mut Args| args.agent_model = Some("anthropic/claude".to_string()),
        ] {
            let mut args = make_args();
            mutate(&mut args);
            assert!(validate_agent_args(&args).is_err());

            let mut listing = Args {
                agent_list: true,
                ..make_args()
            };
            mutate(&mut listing);
            assert!(validate_agent_args(&listing).is_err());
        }
    }

    #[test]
    fn test_validate_agent_with_profile_and_model_requires_new_thread() {
        let existing_thread = Args {
            agent: true,
            paths_with_position: vec!["hello".to_string()],
            agent_profile: Some("write".to_string()),
            ..make_args()
        };
        assert!(validate_agent_args(&existing_thread).is_err());

        let new_thread = Args {
            agent: true,
            agent_new: true,
            paths_with_position: vec!["hello".to_string()],
            agent_profile: Some("write".to_string()),
            agent_model: Some("anthropic/claude-sonnet-4".to_string()),
            ..make_args()
        };
        assert!(validate_agent_args(&new_thread).is_ok());
    }

    #[test]
    fn test_validate_agent_project_without_agent_is_error() {
        let args = Args {
            agent_project: Some(PathBuf::from("/tmp")),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_with_open_behavior_flags_is_error() {
        for mutate in [
            (|args: &mut Args| args.wait = true) as fn(&mut Args),
            |args: &mut Args| args.add = true,
            |args: &mut Args| args.new = true,
            |args: &mut Args| args.reuse = true,
            |args: &mut Args| args.existing = true,
            |args: &mut Args| args.classic = true,
            |args: &mut Args| args.dev_container = true,
            |args: &mut Args| args.diff = vec!["a".to_string(), "b".to_string()],
        ] {
            let mut args = Args {
                agent: true,
                paths_with_position: vec!["hello".to_string()],
                ..make_args()
            };
            mutate(&mut args);
            assert!(validate_agent_args(&args).is_err());
        }
    }

    #[test]
    fn test_validate_agent_allows_foreground() {
        // `--foreground` is how a development build is driven.
        let args = Args {
            agent: true,
            foreground: true,
            paths_with_position: vec!["hello".to_string()],
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_ok());
    }

    #[test]
    fn test_validate_agent_and_agent_list_together_is_error() {
        let args = Args {
            agent: true,
            agent_list: true,
            paths_with_position: vec!["hello".to_string()],
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_whitespace_prompt_is_error() {
        let args = Args {
            agent: true,
            paths_with_position: vec!["   ".to_string()],
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_no_prompt_is_error() {
        let args = Args {
            agent: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_err());
    }

    #[test]
    fn test_validate_agent_with_prompt_is_ok() {
        let args = Args {
            agent: true,
            paths_with_position: vec!["hello".into()],
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_ok());
    }

    #[test]
    fn test_validate_agent_list_alone_is_ok() {
        let args = Args {
            agent_list: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_ok());
    }

    #[test]
    fn test_validate_agent_list_with_project_is_ok() {
        let args = Args {
            agent_list: true,
            agent_project: Some(PathBuf::from("/tmp")),
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_ok());
    }

    #[test]
    fn test_validate_agent_with_thread_and_project_is_ok() {
        let args = Args {
            agent: true,
            paths_with_position: vec!["fix this".into()],
            agent_thread: Some("abc123".into()),
            agent_project: Some(PathBuf::from("/tmp")),
            agent_wait: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_ok());
    }

    #[test]
    fn test_validate_agent_new_alone_is_ok() {
        let args = Args {
            agent: true,
            paths_with_position: vec!["fix this".into()],
            agent_new: true,
            ..make_args()
        };
        assert!(validate_agent_args(&args).is_ok());
    }
}

fn parse_path_in_wsl(source: &str, wsl: &str) -> Result<String> {
    let mut source = PathWithPosition::parse_str(source);

    let (user, distro_name) = if let Some((user, distro)) = wsl.split_once('@') {
        if user.is_empty() {
            anyhow::bail!("user is empty in wsl argument");
        }
        (Some(user), distro)
    } else {
        (None, wsl)
    };

    let mut args = vec!["--distribution", distro_name];
    if let Some(user) = user {
        args.push("--user");
        args.push(user);
    }

    let command = [
        OsStr::new("realpath"),
        OsStr::new("-s"),
        source.path.as_ref(),
    ];

    let output = util::command::new_std_command("wsl.exe")
        .args(&args)
        .arg("--exec")
        .args(&command)
        .output()?;
    let result = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        let fallback = util::command::new_std_command("wsl.exe")
            .args(&args)
            .arg("--")
            .args(&command)
            .output()?;
        String::from_utf8_lossy(&fallback.stdout).to_string()
    };

    source.path = Path::new(result.trim()).to_owned();

    Ok(source.to_string(&|path| path.to_string_lossy().into_owned()))
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn validate_agent_args(args: &Args) -> Result<()> {
    let is_agent_list = args.agent_list;
    let is_prompt = args.agent;

    if !is_prompt && !is_agent_list {
        // The modifier flags are meaningless on their own, and silently
        // ignoring them would mask a forgotten `--agent`.
        anyhow::ensure!(
            args.agent_thread.is_none(),
            "--agent-thread requires --agent"
        );
        anyhow::ensure!(
            args.agent_session.is_none(),
            "--agent-session requires --agent"
        );
        anyhow::ensure!(!args.agent_new, "--agent-new requires --agent");
        anyhow::ensure!(!args.agent_wait, "--agent-wait requires --agent");
        anyhow::ensure!(
            args.agent_project.is_none(),
            "--agent-project requires --agent or --agent-list"
        );
        anyhow::ensure!(
            args.agent_list_format == AgentListFormat::Text,
            "--agent-list-format requires --agent-list"
        );
        anyhow::ensure!(
            args.agent_profile.is_none(),
            "--agent-profile requires --agent"
        );
        anyhow::ensure!(args.agent_model.is_none(), "--agent-model requires --agent");
        return Ok(());
    }

    anyhow::ensure!(
        !(is_prompt && args.agent_list_format != AgentListFormat::Text),
        "--agent-list-format requires --agent-list"
    );

    anyhow::ensure!(
        !(is_prompt && is_agent_list),
        "--agent and --agent-list cannot be used together"
    );

    // Flags that steer file opening have no meaning for an agent request, and
    // silently dropping them would hide a mistyped command. `--foreground` is
    // deliberately still allowed: it is how a development build is driven.
    for (is_set, flag) in [
        (args.wait, "--wait"),
        (args.add, "--add"),
        (args.new, "--new"),
        (args.reuse, "--reuse"),
        (args.existing, "--existing"),
        (args.classic, "--classic"),
        (args.dev_container, "--dev-container"),
        (!args.diff.is_empty(), "--diff"),
        (args.askpass.is_some(), "--askpass"),
    ] {
        anyhow::ensure!(
            !is_set,
            "{flag} cannot be combined with --agent or --agent-list"
        );
    }

    anyhow::ensure!(
        !(is_agent_list && !args.paths_with_position.is_empty()),
        "--agent-list does not take a prompt"
    );

    // --agent-list with any prompt-only flags is invalid.
    if is_agent_list {
        anyhow::ensure!(
            !args.agent_new,
            "--agent-list cannot be combined with --agent-new"
        );
        anyhow::ensure!(
            !args.agent_wait,
            "--agent-list cannot be combined with --agent-wait"
        );
        anyhow::ensure!(
            args.agent_thread.is_none(),
            "--agent-list cannot be combined with --agent-thread"
        );
        anyhow::ensure!(
            args.agent_session.is_none(),
            "--agent-list cannot be combined with --agent-session"
        );
        anyhow::ensure!(
            args.agent_profile.is_none(),
            "--agent-list cannot be combined with --agent-profile"
        );
        anyhow::ensure!(
            args.agent_model.is_none(),
            "--agent-list cannot be combined with --agent-model"
        );
    }

    // --agent-thread together with --agent-session is invalid.
    anyhow::ensure!(
        !(args.agent_thread.is_some() && args.agent_session.is_some()),
        "--agent-thread and --agent-session cannot be used together"
    );

    // Profile and model are persisted on the thread and also cascade into the
    // model selection and any running subagents, so they are only accepted when
    // creating a thread rather than reconfiguring one the user already owns.
    if !args.agent_new {
        anyhow::ensure!(
            args.agent_profile.is_none(),
            "--agent-profile requires --agent-new"
        );
        anyhow::ensure!(
            args.agent_model.is_none(),
            "--agent-model requires --agent-new"
        );
    }

    // --agent-new together with --agent-thread or --agent-session is invalid.
    if args.agent_new {
        anyhow::ensure!(
            args.agent_thread.is_none(),
            "--agent-new cannot be combined with --agent-thread"
        );
        anyhow::ensure!(
            args.agent_session.is_none(),
            "--agent-new cannot be combined with --agent-session"
        );
    }

    // --agent requires a prompt (or stdin). A prompt made only of whitespace
    // would otherwise reach the agent as an empty message it cannot act on.
    if is_prompt && !is_agent_list {
        anyhow::ensure!(
            args.paths_with_position
                .iter()
                .any(|argument| !argument.trim().is_empty()),
            "no prompt provided: pass a prompt as positional arguments, or pipe one via stdin using '-'"
        );
    }

    Ok(())
}

fn run() -> Result<()> {
    #[cfg(unix)]
    util::prevent_root_execution();

    // Exit flatpak sandbox if needed
    #[cfg(target_os = "linux")]
    {
        flatpak::try_restart_to_host();
        flatpak::ld_extra_libs();
    }

    // Intercept version designators
    #[cfg(target_os = "macos")]
    if let Some(channel) = std::env::args().nth(1).filter(|arg| arg.starts_with("--")) {
        // When the first argument is a name of a release channel, we're going to spawn off the CLI of that version, with trailing args passed along.
        use std::str::FromStr as _;

        if let Ok(channel) = release_channel::ReleaseChannel::from_str(&channel[2..]) {
            return mac_os::spawn_channel_cli(channel, std::env::args().skip(2).collect());
        }
    }

    // Must happen before clap — SSH invokes cli.exe directly as SSH_ASKPASS
    // and passes the socket path via env var to avoid argument parsing.
    if let Ok(socket) = std::env::var("ZED_ASKPASS_SOCKET") {
        askpass::main_from_args(&socket, std::env::args().skip(1));
        return Ok(());
    }

    let args = Args::parse();

    // Validate agent arg combinations before doing any IPC work.
    validate_agent_args(&args)?;
    let is_agent_mode = args.agent || args.agent_list;

    // `zed --askpass` Makes zed operate in nc/netcat mode for use with askpass
    if let Some(socket) = &args.askpass {
        askpass::main(socket);
        return Ok(());
    }

    // Set custom data directory before any path operations
    let user_data_dir = args.user_data_dir.clone();
    if let Some(dir) = &user_data_dir {
        paths::set_custom_data_dir(dir);
    }

    #[cfg(target_os = "linux")]
    let args = flatpak::set_bin_if_no_escape(args);

    let app = Detect::detect(args.zed.as_deref()).context("Bundle detection")?;

    if let Some(shell) = &args.completions {
        let file_path = std::env::current_exe()?;
        let file_name = file_path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or("--completions expects a UTF-8 name for cli bin")
            .map_err(anyhow::Error::msg)?;
        let mut cmd = Args::command();
        cmd.set_bin_name(file_name);
        cmd.build();
        crate::completions::main(&cmd, shell);
        return Ok(());
    }

    if args.version {
        println!("{}", app.zed_version_string());
        return Ok(());
    }

    if args.system_specs {
        let path = app.path();
        let msg = [
            "The `--system-specs` argument is not supported in the Zed CLI, only on Zed binary.",
            "To retrieve the system specs on the command line, run the following command:",
            &format!("{} --system-specs", path.display()),
        ];
        anyhow::bail!(msg.join("\n"));
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "macos"),
        not(feature = "no-bundled-uninstall")
    ))]
    if args.uninstall {
        static UNINSTALL_SCRIPT: &[u8] = include_bytes!("../../../script/uninstall.sh");

        let tmp_dir = tempfile::tempdir()?;
        let script_path = tmp_dir.path().join("uninstall.sh");
        fs::write(&script_path, UNINSTALL_SCRIPT)?;

        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;

        let status = std::process::Command::new("sh")
            .arg(&script_path)
            .env("ZED_CHANNEL", &*release_channel::RELEASE_CHANNEL_NAME)
            .status()
            .context("Failed to execute uninstall script")?;

        std::process::exit(status.code().unwrap_or(1));
    }

    let (server, server_name) =
        IpcOneShotServer::<IpcHandshake>::new().context("Handshake before Zed spawn")?;
    let url = format!("zed-cli://{server_name}");

    let open_behavior = if args.new {
        cli::OpenBehavior::AlwaysNew
    } else if args.add {
        cli::OpenBehavior::Add
    } else if args.existing {
        cli::OpenBehavior::ExistingWindow
    } else if args.classic {
        cli::OpenBehavior::Classic
    } else if args.reuse {
        cli::OpenBehavior::Reuse
    } else {
        cli::OpenBehavior::Default
    };

    let env = {
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            use collections::HashMap;

            // On Linux, the desktop entry uses `cli` to spawn `zed`.
            // We need to handle env vars correctly since std::env::vars() may not contain
            // project-specific vars (e.g. those set by direnv).
            // By setting env to None here, the LSP will use worktree env vars instead,
            // which is what we want.
            if !std::io::stdout().is_terminal() {
                None
            } else {
                Some(std::env::vars().collect::<HashMap<_, _>>())
            }
        }

        #[cfg(target_os = "windows")]
        {
            // On Windows, by default, a child process inherits a copy of the environment block of the parent process.
            // So we don't need to pass env vars explicitly.
            None
        }

        #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "windows")))]
        {
            use collections::HashMap;

            Some(std::env::vars().collect::<HashMap<_, _>>())
        }
    };

    let exit_status = Arc::new(Mutex::new(None));

    let mut agent_prompt: Option<String> = None;
    let mut agent_project: Option<PathBuf> = None;

    if is_agent_mode {
        if !args.paths_with_position.is_empty() {
            let prompt_text = args.paths_with_position.join(" ");
            if prompt_text == "-" {
                // Read prompt from stdin. Do this synchronously before the
                // sender thread so we know the full prompt before we handshake.
                let mut stdin = std::io::stdin().lock();
                if !io::IsTerminal::is_terminal(&stdin) {
                    let mut buffer = String::new();
                    std::io::Read::read_to_string(&mut stdin, &mut buffer)?;
                    agent_prompt = Some(buffer);
                } else {
                    anyhow::bail!(
                        "stdin is a terminal, but '-' was given as the prompt; pipe input or pass a literal prompt"
                    );
                }
            } else {
                agent_prompt = Some(prompt_text);
            }
        }

        if args.agent {
            // An all-whitespace prompt would otherwise be dispatched as an
            // empty content block, which the agent cannot act on.
            let prompt = agent_prompt
                .as_deref()
                .map(str::trim)
                .filter(|prompt| !prompt.is_empty())
                .context("no prompt provided: pipe one via stdin, or pass it as arguments")?;
            agent_prompt = Some(prompt.to_string());
        }

        if let Some(project_path) = args.agent_project.as_deref() {
            // Without this the app silently falls back to whichever project
            // happens to be open, which looks like success but targets the
            // wrong thread.
            anyhow::ensure!(
                project_path.is_dir(),
                "--agent-project directory does not exist: \"{}\"",
                project_path.display()
            );
        }
        agent_project = args
            .agent_project
            .clone()
            .or_else(|| env::current_dir().ok());
        if let Some(ref project_path) = agent_project {
            if project_path.exists() {
                agent_project = Some(fs::canonicalize(project_path)?);
            }
        }
    }

    let mut paths = vec![];
    let mut urls = vec![];
    let mut diff_paths = vec![];
    let mut diff_all_mode = false;
    let mut stdin_tmp_file: Option<fs::File> = None;
    let mut anonymous_fd_tmp_files = vec![];
    let wsl_for_ipc: Option<String> = {
        #[cfg(target_os = "windows")]
        {
            if !is_agent_mode {
                args.wsl.clone()
            } else {
                None
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    };

    if !is_agent_mode {
        diff_all_mode = args
            .diff
            .chunks(2)
            .any(|pair| Path::new(&pair[0]).is_dir() || Path::new(&pair[1]).is_dir());

        for path in args.diff.chunks(2) {
            let left = parse_path_with_position(&path[0])?;
            let right = parse_path_with_position(&path[1])?;
            for diff_path in [&left, &right] {
                anyhow::ensure!(
                    diff_path_exists(diff_path),
                    "--diff path does not exist: {diff_path}"
                );
            }
            diff_paths.push([left, right]);
        }

        let (expanded_diff_paths, temp_dirs) = expand_directory_diff_pairs(diff_paths)?;
        diff_paths = expanded_diff_paths;
        for temp_dir in temp_dirs {
            let _ = temp_dir.keep();
        }

        #[cfg(target_os = "windows")]
        let wsl = args.wsl.as_ref();
        #[cfg(not(target_os = "windows"))]
        let wsl: Option<&String> = None;

        for path in args.paths_with_position.iter() {
            if URL_PREFIX.iter().any(|&prefix| path.starts_with(prefix)) {
                urls.push(path.to_string());
            } else if path == "-" && args.paths_with_position.len() == 1 {
                let file = NamedTempFile::new()?;
                paths.push(file.path().to_string_lossy().into_owned());
                let (file, _) = file.keep()?;
                stdin_tmp_file = Some(file);
            } else if let Some(file) = anonymous_fd(path) {
                let tmp_file = NamedTempFile::new()?;
                paths.push(tmp_file.path().to_string_lossy().into_owned());
                let (tmp_file, _) = tmp_file.keep()?;
                anonymous_fd_tmp_files.push((file, tmp_file));
            } else if let Some(wsl) = wsl {
                urls.push(format!("file://{}", parse_path_in_wsl(path, wsl)?));
            } else {
                paths.push(parse_path_with_position(path)?);
            }
        }

        anyhow::ensure!(
            args.dev_server_token.is_none(),
            "Dev servers were removed in v0.157.x please upgrade to SSH remoting: https://zed.dev/docs/remote-development"
        );
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .stack_size(10 * 1024 * 1024)
        .thread_name(|ix| format!("RayonWorker{}", ix))
        .build_global()
        .unwrap();

    let ipc_request = if is_agent_mode {
        let action = if args.agent_list {
            AgentAction::List {
                project: agent_project,
                json: args.agent_list_format == AgentListFormat::Json,
            }
        } else {
            AgentAction::Prompt {
                prompt: agent_prompt.unwrap_or_default(),
                thread: args.agent_thread.clone(),
                session: args.agent_session.clone(),
                project: agent_project,
                profile: args.agent_profile.clone(),
                model: args.agent_model.clone(),
                new_thread: args.agent_new,
                wait: args.agent_wait,
            }
        };
        CliRequest::Agent {
            action,
            cwd: env::current_dir().ok(),
        }
    } else {
        CliRequest::Open {
            paths,
            urls,
            diff_paths,
            diff_all: diff_all_mode,
            wsl: wsl_for_ipc,
            wait: args.wait,
            open_behavior,
            env,
            user_data_dir: user_data_dir.clone(),
            dev_container: args.dev_container,
            cwd: env::current_dir().ok(),
        }
    };

    let sender: JoinHandle<anyhow::Result<()>> = thread::Builder::new()
        .name("CliReceiver".to_string())
        .spawn({
            let exit_status = exit_status.clone();
            move || {
                let (_, handshake) = server.accept().context("Handshake after Zed spawn")?;
                let (tx, rx) = (handshake.requests, handshake.responses);

                tx.send(ipc_request)?;

                while let Ok(response) = rx.recv() {
                    match response {
                        CliResponse::Ping => {}
                        CliResponse::Stdout { message } => println!("{message}"),
                        CliResponse::Stderr { message } => eprintln!("{message}"),
                        CliResponse::Exit { status } => {
                            exit_status.lock().replace(status);
                            return Ok(());
                        }
                        CliResponse::PromptOpenBehavior => {
                            // Should not occur in agent mode, but handle gracefully.
                            if is_agent_mode {
                                eprintln!(
                                    "protocol error: received PromptOpenBehavior in agent mode"
                                );
                            } else {
                                let behavior = prompt_open_behavior()
                                    .unwrap_or(cli::CliBehaviorSetting::ExistingWindow);
                                tx.send(CliRequest::SetOpenBehavior { behavior })?;
                            }
                        }
                    }
                }

                // The channel closed without an `Exit`, which means Zed went
                // away mid-request. Reporting success here would let a webhook
                // treat a dropped request as a delivered one.
                if is_agent_mode && exit_status.lock().is_none() {
                    eprintln!("error: Zed closed the connection without responding");
                    exit_status.lock().replace(1);
                }

                Ok(())
            }
        })
        .unwrap();

    let stdin_pipe_handle: Option<JoinHandle<anyhow::Result<()>>> =
        stdin_tmp_file.map(|mut tmp_file| {
            thread::Builder::new()
                .name("CliStdin".to_string())
                .spawn(move || {
                    let mut stdin = std::io::stdin().lock();
                    if !io::IsTerminal::is_terminal(&stdin) {
                        io::copy(&mut stdin, &mut tmp_file)?;
                    }
                    Ok(())
                })
                .unwrap()
        });

    let anonymous_fd_pipe_handles: Vec<_> = anonymous_fd_tmp_files
        .into_iter()
        .map(|(mut file, mut tmp_file)| {
            thread::Builder::new()
                .name("CliAnonymousFd".to_string())
                .spawn(move || io::copy(&mut file, &mut tmp_file))
                .unwrap()
        })
        .collect();

    if args.foreground {
        app.run_foreground(url, user_data_dir.as_deref())?;
    } else {
        app.launch(url, user_data_dir.as_deref())?;
        sender.join().unwrap()?;
        if let Some(handle) = stdin_pipe_handle {
            handle.join().unwrap()?;
        }
        for handle in anonymous_fd_pipe_handles {
            handle.join().unwrap()?;
        }
    }

    if let Some(exit_status) = exit_status.lock().take() {
        std::process::exit(exit_status);
    }
    Ok(())
}

fn anonymous_fd(path: &str) -> Option<fs::File> {
    #[cfg(target_os = "linux")]
    {
        use std::os::fd::{self, FromRawFd};

        let fd_str = path.strip_prefix("/proc/self/fd/")?;

        let link = fs::read_link(path).ok()?;
        if !link.starts_with("memfd:") {
            return None;
        }

        let fd: fd::RawFd = fd_str.parse().ok()?;
        let file = unsafe { fs::File::from_raw_fd(fd) };
        Some(file)
    }
    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    {
        use std::os::{
            fd::{self, FromRawFd},
            unix::fs::FileTypeExt,
        };

        let fd_str = path.strip_prefix("/dev/fd/")?;

        let metadata = fs::metadata(path).ok()?;
        let file_type = metadata.file_type();
        if !file_type.is_fifo() && !file_type.is_socket() {
            return None;
        }
        let fd: fd::RawFd = fd_str.parse().ok()?;
        let file = unsafe { fs::File::from_raw_fd(fd) };
        Some(file)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        _ = path;
        // not implemented for bsd, windows. Could be, but isn't yet
        None
    }
}

/// Shows an interactive prompt asking the user to choose the default open
/// behavior for `zed <path>`. Returns `None` if the prompt cannot be shown
/// (e.g. stdin is not a terminal) or the user cancels.
fn prompt_open_behavior() -> Option<cli::CliBehaviorSetting> {
    if !std::io::stdin().is_terminal() {
        return None;
    }

    let blue = console::Style::new().blue();
    let items = [
        format!(
            "Add to existing Zed window ({})",
            blue.apply_to("zed --existing")
        ),
        format!("Open a new window ({})", blue.apply_to("zed --classic")),
    ];

    let prompt = format!(
        "Configure default behavior for {}\n{}",
        blue.apply_to("zed <path>"),
        console::style("You can change this later in Zed settings"),
    );

    let selection = dialoguer::Select::new()
        .with_prompt(&prompt)
        .items(&items)
        .default(0)
        .interact()
        .ok()?;

    Some(if selection == 0 {
        cli::CliBehaviorSetting::ExistingWindow
    } else {
        cli::CliBehaviorSetting::NewWindow
    })
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux {
    use std::{
        env,
        ffi::OsString,
        io,
        os::unix::net::{SocketAddr, UnixDatagram},
        path::{Path, PathBuf},
        process::{self, ExitStatus},
        thread,
        time::Duration,
    };

    use anyhow::{Context as _, anyhow};
    use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
    use fork::Fork;

    use crate::{Detect, InstalledApp};

    struct App(PathBuf);

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let path = if let Some(path) = path {
                path.to_path_buf().canonicalize()?
            } else {
                let cli = env::current_exe()?;
                let dir = cli.parent().context("no parent path for cli")?;

                // libexec is the standard, lib/zed is for Arch (and other non-libexec distros),
                // ./zed is for the target directory in development builds.
                let possible_locations =
                    ["../libexec/zed-editor", "../lib/zed/zed-editor", "./zed"];
                possible_locations
                    .iter()
                    .find_map(|p| dir.join(p).canonicalize().ok().filter(|path| path != &cli))
                    .with_context(|| {
                        format!("could not find any of: {}", possible_locations.join(", "))
                    })?
            };

            Ok(App(path))
        }
    }

    impl InstalledApp for App {
        fn zed_version_string(&self) -> String {
            format!(
                "Zed {}{}{} – {}",
                if *release_channel::RELEASE_CHANNEL_NAME == "stable" {
                    "".to_string()
                } else {
                    format!("{} ", *release_channel::RELEASE_CHANNEL_NAME)
                },
                option_env!("RELEASE_VERSION").unwrap_or_default(),
                match option_env!("ZED_COMMIT_SHA") {
                    Some(commit_sha) => format!(" {commit_sha} "),
                    None => "".to_string(),
                },
                self.0.display(),
            )
        }

        fn launch(&self, ipc_url: String, user_data_dir: Option<&str>) -> anyhow::Result<()> {
            let data_dir = user_data_dir
                .map(PathBuf::from)
                .unwrap_or_else(|| paths::data_dir().clone());

            let sock_path = data_dir.join(format!(
                "zed-{}.sock",
                *release_channel::RELEASE_CHANNEL_NAME
            ));
            let sock = UnixDatagram::unbound()?;
            if sock.connect(&sock_path).is_err() {
                self.boot_background(ipc_url, user_data_dir)?;
            } else {
                sock.send(ipc_url.as_bytes())?;
            }
            Ok(())
        }

        fn run_foreground(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> io::Result<ExitStatus> {
            let mut cmd = std::process::Command::new(self.0.clone());
            cmd.arg(ipc_url);
            if let Some(dir) = user_data_dir {
                cmd.arg("--user-data-dir").arg(dir);
            }
            cmd.status()
        }

        fn path(&self) -> PathBuf {
            self.0.clone()
        }
    }

    impl App {
        fn boot_background(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> anyhow::Result<()> {
            let path = &self.0;

            match fork::fork() {
                Ok(Fork::Parent(_)) => Ok(()),
                Ok(Fork::Child) => {
                    unsafe { std::env::set_var(FORCE_CLI_MODE_ENV_VAR_NAME, "") };
                    if fork::setsid().is_err() {
                        eprintln!("failed to setsid: {}", std::io::Error::last_os_error());
                        process::exit(1);
                    }
                    if fork::close_fd().is_err() {
                        eprintln!("failed to close_fd: {}", std::io::Error::last_os_error());
                    }
                    let mut args: Vec<OsString> =
                        vec![path.as_os_str().to_owned(), OsString::from(ipc_url)];
                    if let Some(dir) = user_data_dir {
                        args.push(OsString::from("--user-data-dir"));
                        args.push(OsString::from(dir));
                    }
                    let error = exec::execvp(path.clone(), &args);
                    // if exec succeeded, we never get here.
                    eprintln!("failed to exec {:?}: {}", path, error);
                    process::exit(1)
                }
                Err(_) => Err(anyhow!(io::Error::last_os_error())),
            }
        }

        fn wait_for_socket(
            &self,
            sock_addr: &SocketAddr,
            sock: &mut UnixDatagram,
        ) -> Result<(), std::io::Error> {
            for _ in 0..100 {
                thread::sleep(Duration::from_millis(10));
                if sock.connect_addr(sock_addr).is_ok() {
                    return Ok(());
                }
            }
            sock.connect_addr(sock_addr)
        }
    }
}

#[cfg(target_os = "linux")]
mod flatpak {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::process::Command;
    use std::{env, process};

    const EXTRA_LIB_ENV_NAME: &str = "ZED_FLATPAK_LIB_PATH";
    const NO_ESCAPE_ENV_NAME: &str = "ZED_FLATPAK_NO_ESCAPE";

    /// Adds bundled libraries to LD_LIBRARY_PATH if running under flatpak
    pub fn ld_extra_libs() {
        let mut paths = if let Ok(paths) = env::var("LD_LIBRARY_PATH") {
            env::split_paths(&paths).collect()
        } else {
            Vec::new()
        };

        if let Ok(extra_path) = env::var(EXTRA_LIB_ENV_NAME) {
            paths.push(extra_path.into());
        }

        unsafe { env::set_var("LD_LIBRARY_PATH", env::join_paths(paths).unwrap()) };
    }

    /// Restarts outside of the sandbox if currently running within it
    pub fn try_restart_to_host() {
        if let Some(flatpak_dir) = get_flatpak_dir() {
            let mut args = vec!["/usr/bin/flatpak-spawn".into(), "--host".into()];
            args.append(&mut get_xdg_env_args());
            args.push("--env=ZED_UPDATE_EXPLANATION=Please use flatpak to update zed".into());
            args.push(
                format!(
                    "--env={EXTRA_LIB_ENV_NAME}={}",
                    flatpak_dir.join("lib").to_str().unwrap()
                )
                .into(),
            );
            args.push(flatpak_dir.join("bin").join("zed").into());

            let mut is_app_location_set = false;
            for arg in &env::args_os().collect::<Vec<_>>()[1..] {
                args.push(arg.clone());
                is_app_location_set |= arg == "--zed";
            }

            if !is_app_location_set {
                args.push("--zed".into());
                args.push(flatpak_dir.join("libexec").join("zed-editor").into());
            }

            let error = exec::execvp("/usr/bin/flatpak-spawn", args);
            eprintln!("failed restart cli on host: {:?}", error);
            process::exit(1);
        }
    }

    pub fn set_bin_if_no_escape(mut args: super::Args) -> super::Args {
        if env::var(NO_ESCAPE_ENV_NAME).is_ok()
            && env::var("FLATPAK_ID").is_ok_and(|id| id.starts_with("dev.zed.Zed"))
            && args.zed.is_none()
        {
            args.zed = Some("/app/libexec/zed-editor".into());
            unsafe { env::set_var("ZED_UPDATE_EXPLANATION", "Please use flatpak to update zed") };
        }
        args
    }

    fn get_flatpak_dir() -> Option<PathBuf> {
        if env::var(NO_ESCAPE_ENV_NAME).is_ok() {
            return None;
        }

        if let Ok(flatpak_id) = env::var("FLATPAK_ID") {
            if !flatpak_id.starts_with("dev.zed.Zed") {
                return None;
            }

            let install_dir = Command::new("/usr/bin/flatpak-spawn")
                .arg("--host")
                .arg("flatpak")
                .arg("info")
                .arg("--show-location")
                .arg(flatpak_id)
                .output()
                .unwrap();
            let install_dir = PathBuf::from(String::from_utf8(install_dir.stdout).unwrap().trim());
            Some(install_dir.join("files"))
        } else {
            None
        }
    }

    fn get_xdg_env_args() -> Vec<OsString> {
        let xdg_keys = [
            "XDG_DATA_HOME",
            "XDG_CONFIG_HOME",
            "XDG_CACHE_HOME",
            "XDG_STATE_HOME",
        ];
        env::vars()
            .filter(|(key, _)| xdg_keys.contains(&key.as_str()))
            .map(|(key, val)| format!("--env=FLATPAK_{}={}", key, val).into())
            .collect()
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use anyhow::Context;
    use release_channel::app_identifier;
    use windows::{
        Win32::{
            Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GENERIC_WRITE, GetLastError},
            Storage::FileSystem::{
                CreateFileW, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, OPEN_EXISTING, WriteFile,
            },
            System::Threading::CreateMutexW,
        },
        core::HSTRING,
    };

    use crate::{Detect, InstalledApp};
    use std::io;
    use std::path::{Path, PathBuf};
    use std::process::{ExitStatus, Stdio};

    fn check_single_instance() -> bool {
        let mutex = unsafe {
            CreateMutexW(
                None,
                false,
                &HSTRING::from(format!("{}-Instance-Mutex", app_identifier())),
            )
            .expect("Unable to create instance sync event")
        };
        let last_err = unsafe { GetLastError() };
        let _ = unsafe { CloseHandle(mutex) };
        last_err != ERROR_ALREADY_EXISTS
    }

    struct App(PathBuf);

    impl InstalledApp for App {
        fn zed_version_string(&self) -> String {
            format!(
                "Zed {}{}{} – {}",
                if *release_channel::RELEASE_CHANNEL_NAME == "stable" {
                    "".to_string()
                } else {
                    format!("{} ", *release_channel::RELEASE_CHANNEL_NAME)
                },
                option_env!("RELEASE_VERSION").unwrap_or_default(),
                match option_env!("ZED_COMMIT_SHA") {
                    Some(commit_sha) => format!(" {commit_sha} "),
                    None => "".to_string(),
                },
                self.0.display(),
            )
        }

        fn launch(&self, ipc_url: String, user_data_dir: Option<&str>) -> anyhow::Result<()> {
            if check_single_instance() {
                let mut cmd = std::process::Command::new(self.0.clone());
                cmd.arg(ipc_url);
                if let Some(dir) = user_data_dir {
                    cmd.arg("--user-data-dir").arg(dir);
                }
                cmd.stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null());
                cmd.spawn()?;
            } else {
                unsafe {
                    let pipe = CreateFileW(
                        &HSTRING::from(format!("\\\\.\\pipe\\{}-Named-Pipe", app_identifier())),
                        GENERIC_WRITE.0,
                        FILE_SHARE_MODE::default(),
                        None,
                        OPEN_EXISTING,
                        FILE_FLAGS_AND_ATTRIBUTES::default(),
                        None,
                    )?;
                    let message = ipc_url.as_bytes();
                    let mut bytes_written = 0;
                    WriteFile(pipe, Some(message), Some(&mut bytes_written), None)?;
                    CloseHandle(pipe)?;
                }
            }
            Ok(())
        }

        fn run_foreground(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> io::Result<ExitStatus> {
            let mut cmd = std::process::Command::new(self.0.clone());
            cmd.arg(ipc_url).arg("--foreground");
            if let Some(dir) = user_data_dir {
                cmd.arg("--user-data-dir").arg(dir);
            }
            cmd.spawn()?.wait()
        }

        fn path(&self) -> PathBuf {
            self.0.clone()
        }
    }

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let path = if let Some(path) = path {
                path.to_path_buf().canonicalize()?
            } else {
                let cli = std::env::current_exe()?;
                let dir = cli.parent().context("no parent path for cli")?;

                // ../Zed.exe is the standard, lib/zed is for MSYS2, ./zed.exe is for the target
                // directory in development builds.
                let possible_locations = ["../Zed.exe", "../lib/zed/zed-editor.exe", "./zed.exe"];
                possible_locations
                    .iter()
                    .find_map(|p| dir.join(p).canonicalize().ok().filter(|path| path != &cli))
                    .context(format!(
                        "could not find any of: {}",
                        possible_locations.join(", ")
                    ))?
            };

            Ok(App(path))
        }
    }
}

#[cfg(target_os = "macos")]
mod mac_os {
    use anyhow::{Context as _, Result};
    use core_foundation::{
        array::{CFArray, CFIndex},
        base::TCFType as _,
        string::kCFStringEncodingUTF8,
        url::{CFURL, CFURLCreateWithBytes},
    };
    use core_services::{
        LSLaunchURLSpec, LSOpenFromURLSpec, kLSLaunchDefaults, kLSLaunchDontSwitch,
    };
    use serde::Deserialize;
    use std::{
        ffi::OsStr,
        fs, io,
        path::{Path, PathBuf},
        process::{Command, ExitStatus},
        ptr,
    };

    use cli::FORCE_CLI_MODE_ENV_VAR_NAME;

    use crate::{Detect, InstalledApp};

    #[derive(Debug, Deserialize)]
    struct InfoPlist {
        #[serde(rename = "CFBundleShortVersionString")]
        bundle_short_version_string: String,
    }

    enum Bundle {
        App {
            app_bundle: PathBuf,
            plist: InfoPlist,
        },
        LocalPath {
            executable: PathBuf,
        },
    }

    fn locate_bundle() -> Result<PathBuf> {
        let cli_path = std::env::current_exe()?.canonicalize()?;
        let mut app_path = cli_path.clone();
        while app_path.extension() != Some(OsStr::new("app")) {
            anyhow::ensure!(
                app_path.pop(),
                "cannot find app bundle containing {cli_path:?}"
            );
        }
        Ok(app_path)
    }

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let bundle_path = if let Some(bundle_path) = path {
                bundle_path
                    .canonicalize()
                    .with_context(|| format!("Args bundle path {bundle_path:?} canonicalization"))?
            } else {
                locate_bundle().context("bundle autodiscovery")?
            };

            match bundle_path.extension().and_then(|ext| ext.to_str()) {
                Some("app") => {
                    let plist_path = bundle_path.join("Contents/Info.plist");
                    let plist =
                        plist::from_file::<_, InfoPlist>(&plist_path).with_context(|| {
                            format!("Reading *.app bundle plist file at {plist_path:?}")
                        })?;
                    Ok(Bundle::App {
                        app_bundle: bundle_path,
                        plist,
                    })
                }
                _ => Ok(Bundle::LocalPath {
                    executable: bundle_path,
                }),
            }
        }
    }

    impl InstalledApp for Bundle {
        fn zed_version_string(&self) -> String {
            format!("Zed {} – {}", self.version(), self.path().display(),)
        }

        fn launch(&self, url: String, user_data_dir: Option<&str>) -> anyhow::Result<()> {
            match self {
                Self::App { app_bundle, .. } => {
                    let app_path = app_bundle;

                    let status = unsafe {
                        let app_url = CFURL::from_path(app_path, true)
                            .with_context(|| format!("invalid app path {app_path:?}"))?;
                        let url_to_open = CFURL::wrap_under_create_rule(CFURLCreateWithBytes(
                            ptr::null(),
                            url.as_ptr(),
                            url.len() as CFIndex,
                            kCFStringEncodingUTF8,
                            ptr::null(),
                        ));
                        // equivalent to: open zed-cli:... -a /Applications/Zed\ Preview.app
                        let urls_to_open =
                            CFArray::from_copyable(&[url_to_open.as_concrete_TypeRef()]);
                        LSOpenFromURLSpec(
                            &LSLaunchURLSpec {
                                appURL: app_url.as_concrete_TypeRef(),
                                itemURLs: urls_to_open.as_concrete_TypeRef(),
                                passThruParams: ptr::null(),
                                launchFlags: kLSLaunchDefaults | kLSLaunchDontSwitch,
                                asyncRefCon: ptr::null_mut(),
                            },
                            ptr::null_mut(),
                        )
                    };

                    anyhow::ensure!(
                        status == 0,
                        "cannot start app bundle {}",
                        self.zed_version_string()
                    );
                }

                Self::LocalPath { executable, .. } => {
                    let executable_parent = executable
                        .parent()
                        .with_context(|| format!("Executable {executable:?} path has no parent"))?;
                    let subprocess_stdout_file = fs::File::create(
                        executable_parent.join("zed_dev.log"),
                    )
                    .with_context(|| format!("Log file creation in {executable_parent:?}"))?;
                    let subprocess_stdin_file =
                        subprocess_stdout_file.try_clone().with_context(|| {
                            format!("Cloning descriptor for file {subprocess_stdout_file:?}")
                        })?;
                    let mut command = std::process::Command::new(executable);
                    command.env(FORCE_CLI_MODE_ENV_VAR_NAME, "");
                    if let Some(dir) = user_data_dir {
                        command.arg("--user-data-dir").arg(dir);
                    }
                    command
                        .stderr(subprocess_stdout_file)
                        .stdout(subprocess_stdin_file)
                        .arg(url);

                    command
                        .spawn()
                        .with_context(|| format!("Spawning {command:?}"))?;
                }
            }

            Ok(())
        }

        fn run_foreground(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> io::Result<ExitStatus> {
            let path = match self {
                Bundle::App { app_bundle, .. } => app_bundle.join("Contents/MacOS/zed"),
                Bundle::LocalPath { executable, .. } => executable.clone(),
            };

            let mut cmd = std::process::Command::new(path);
            cmd.arg(ipc_url);
            if let Some(dir) = user_data_dir {
                cmd.arg("--user-data-dir").arg(dir);
            }
            cmd.status()
        }

        fn path(&self) -> PathBuf {
            match self {
                Bundle::App { app_bundle, .. } => app_bundle.join("Contents/MacOS/zed"),
                Bundle::LocalPath { executable, .. } => executable.clone(),
            }
        }
    }

    impl Bundle {
        fn version(&self) -> String {
            match self {
                Self::App { plist, .. } => plist.bundle_short_version_string.clone(),
                Self::LocalPath { .. } => "<development>".to_string(),
            }
        }

        fn path(&self) -> &Path {
            match self {
                Self::App { app_bundle, .. } => app_bundle,
                Self::LocalPath { executable, .. } => executable,
            }
        }
    }

    pub(super) fn spawn_channel_cli(
        channel: release_channel::ReleaseChannel,
        leftover_args: Vec<String>,
    ) -> Result<()> {
        use anyhow::bail;

        let app_path_prompt = format!(
            "POSIX path of (path to application \"{}\")",
            channel.display_name()
        );
        let app_path_output = Command::new("osascript")
            .arg("-e")
            .arg(&app_path_prompt)
            .output()?;
        if !app_path_output.status.success() {
            bail!(
                "Could not determine app path for {}",
                channel.display_name()
            );
        }
        let app_path = String::from_utf8(app_path_output.stdout)?.trim().to_owned();
        let cli_path = format!("{app_path}/Contents/MacOS/cli");
        Command::new(cli_path).args(leftover_args).spawn()?;
        Ok(())
    }
}
