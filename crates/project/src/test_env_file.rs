//! Loader for the per-language `test_env_file` setting.
//!
//! Resolves the configured path(s) against the buffer's worktree root, reads
//! each file via the project's `Fs`, and returns a key/value map ready to be
//! merged into a `TaskContext.project_env`. Missing files are silently
//! skipped; per-line parse errors are logged but do not abort the load.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use collections::HashMap;
use fs::Fs;
use settings::TestEnvFilePaths;
use task::parse_env_file;

pub async fn load_test_env_files(
    paths: &TestEnvFilePaths,
    worktree_root: &Path,
    fs: &Arc<dyn Fs>,
) -> HashMap<String, String> {
    let mut result = HashMap::default();
    for raw in paths.as_ref() {
        let resolved = resolve_path(raw, worktree_root);
        if !fs.is_file(&resolved).await {
            continue;
        }
        match fs.open_sync(&resolved).await {
            Ok(reader) => {
                let (env, warnings) = parse_env_file(reader);
                for warning in &warnings {
                    log::warn!("test_env_file {}: {warning}", resolved.display());
                }
                result.extend(env);
            }
            Err(err) => {
                log::warn!(
                    "test_env_file {}: failed to read: {err}",
                    resolved.display()
                );
            }
        }
    }
    result
}

fn resolve_path(raw: &str, worktree_root: &Path) -> PathBuf {
    let worktree_str = worktree_root.to_string_lossy();
    let expanded = raw
        .replace("${ZED_WORKTREE_ROOT}", &worktree_str)
        .replace("$ZED_WORKTREE_ROOT", &worktree_str);
    let expanded = shellexpand::tilde(&expanded).into_owned();
    let path = PathBuf::from(expanded);
    if path.is_absolute() {
        path
    } else {
        worktree_root.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;

    fn paths_from(values: &[&str]) -> TestEnvFilePaths {
        if values.len() == 1 {
            TestEnvFilePaths::Single(values[0].to_string())
        } else {
            TestEnvFilePaths::Multiple(values.iter().map(|s| s.to_string()).collect())
        }
    }

    #[gpui::test]
    async fn loads_env_from_single_file(cx: &mut TestAppContext) {
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        fs.as_fake()
            .insert_tree(
                "/project",
                json!({ ".env": "FOO=bar\nBAZ=qux\n" }),
            )
            .await;

        let env = load_test_env_files(&paths_from(&[".env"]), Path::new("/project"), &fs).await;

        assert_eq!(env.get("FOO").map(String::as_str), Some("bar"));
        assert_eq!(env.get("BAZ").map(String::as_str), Some("qux"));
    }

    #[gpui::test]
    async fn missing_file_returns_empty(cx: &mut TestAppContext) {
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        fs.as_fake()
            .insert_tree("/project", json!({}))
            .await;

        let env = load_test_env_files(&paths_from(&[".env"]), Path::new("/project"), &fs).await;

        assert!(env.is_empty());
    }

    #[gpui::test]
    async fn layered_files_later_overrides_earlier(cx: &mut TestAppContext) {
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        fs.as_fake()
            .insert_tree(
                "/project",
                json!({
                    ".env": "SHARED=base\nOVERRIDDEN=base\n",
                    ".env.local": "OVERRIDDEN=local\nLOCAL_ONLY=yes\n",
                }),
            )
            .await;

        let env = load_test_env_files(
            &paths_from(&[".env", ".env.local"]),
            Path::new("/project"),
            &fs,
        )
        .await;

        assert_eq!(env.get("SHARED").map(String::as_str), Some("base"));
        assert_eq!(env.get("OVERRIDDEN").map(String::as_str), Some("local"));
        assert_eq!(env.get("LOCAL_ONLY").map(String::as_str), Some("yes"));
    }

    #[gpui::test]
    async fn resolves_zed_worktree_root_variable(cx: &mut TestAppContext) {
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        fs.as_fake()
            .insert_tree(
                "/project",
                json!({ ".env": "FOO=bar\n" }),
            )
            .await;

        let env = load_test_env_files(
            &paths_from(&["$ZED_WORKTREE_ROOT/.env"]),
            Path::new("/project"),
            &fs,
        )
        .await;

        assert_eq!(env.get("FOO").map(String::as_str), Some("bar"));
    }

    #[gpui::test]
    async fn absolute_path_used_as_is(cx: &mut TestAppContext) {
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        fs.as_fake()
            .insert_tree(
                "/elsewhere",
                json!({ "shared.env": "FOO=bar\n" }),
            )
            .await;

        let env = load_test_env_files(
            &paths_from(&["/elsewhere/shared.env"]),
            Path::new("/project"),
            &fs,
        )
        .await;

        assert_eq!(env.get("FOO").map(String::as_str), Some("bar"));
    }
}
