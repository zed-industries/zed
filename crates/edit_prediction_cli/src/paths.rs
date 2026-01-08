use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = dirs::home_dir().unwrap().join(".zed_ep");
    ensure_dir(&dir)
});
pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| ensure_dir(&DATA_DIR.join("cache")));
pub static REPOS_DIR: LazyLock<PathBuf> = LazyLock::new(|| ensure_dir(&DATA_DIR.join("repos")));
pub static WORKTREES_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| ensure_dir(&DATA_DIR.join("worktrees")));
pub static RUN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    DATA_DIR
        .join("runs")
        .join(chrono::Local::now().format("%d-%m-%y-%H_%M_%S").to_string())
});
pub static LATEST_EXAMPLE_RUN_DIR: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("latest"));
pub static LATEST_FAILED_EXAMPLES_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("latest_failed"));
pub static LLM_CACHE_DB: LazyLock<PathBuf> = LazyLock::new(|| CACHE_DIR.join("llm_cache.sqlite"));
pub static SYNTHESIZE_STATE_FILE: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("synthesize_state.json"));
pub static FAILED_EXAMPLES_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| ensure_dir(&RUN_DIR.join("failed")));

fn ensure_dir(path: &Path) -> PathBuf {
    std::fs::create_dir_all(path).expect("Failed to create directory");
    path.to_path_buf()
}
