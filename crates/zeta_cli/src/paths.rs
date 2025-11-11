use std::{env, path::PathBuf, sync::LazyLock};

pub static TARGET_ZETA_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| env::current_dir().unwrap().join("target/zeta"));
pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_ZETA_DIR.join("cache"));
pub static REPOS_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_ZETA_DIR.join("repos"));
pub static WORKTREES_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_ZETA_DIR.join("worktrees"));
pub static RUN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    TARGET_ZETA_DIR
        .join("runs")
        .join(chrono::Local::now().format("%Y%m%d%H%M%S").to_string())
});
pub static LATEST_EXAMPLE_RUN_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| TARGET_ZETA_DIR.join("latest"));
// todo! remove
pub static LOGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_ZETA_DIR.join("zeta-logs"));
pub static LOGS_SEARCH_PROMPT: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("search_prompt.md"));
pub static LOGS_SEARCH_QUERIES: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("search_queries.json"));
pub static LOGS_PREDICTION_PROMPT: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("prediction_prompt.md"));
pub static LOGS_PREDICTION_RESPONSE: LazyLock<PathBuf> =
    LazyLock::new(|| LOGS_DIR.join("prediction_response.md"));
