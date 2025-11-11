use std::{env, path::PathBuf, sync::LazyLock};

static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(|| env::current_dir().unwrap().join("target"));
pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_DIR.join("zeta-eval-support"));
pub static REPOS_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_DIR.join("zeta-repos"));
pub static WORKTREES_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_DIR.join("zeta-worktrees"));
pub static LOGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_DIR.join("zeta-logs"));
pub static LOGS_SEARCH_PROMPT: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("search_prompt.md"));
pub static LOGS_SEARCH_QUERIES: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("search_queries.json"));
pub static LOGS_PREDICTION_PROMPT: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("prediction_prompt.md"));
pub static LOGS_PREDICTION_RESPONSE: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("prediction_response.md"));
