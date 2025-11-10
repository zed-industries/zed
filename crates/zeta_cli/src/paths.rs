use std::{env, path::PathBuf, sync::LazyLock};

static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(|| env::current_dir().unwrap().join("target"));
pub static CACHE_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| TARGET_DIR.join("zeta-llm-response-cache"));
pub static REPOS_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_DIR.join("zeta-repos"));
pub static WORKTREES_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_DIR.join("zeta-worktrees"));
pub static LOGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_DIR.join("zeta-logs"));
