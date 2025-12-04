use std::{env, path::PathBuf, sync::LazyLock};

pub static TARGET_ZETA_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| env::current_dir().unwrap().join("target/zeta"));
pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_ZETA_DIR.join("cache"));
pub static REPOS_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_ZETA_DIR.join("repos"));
pub static WORKTREES_DIR: LazyLock<PathBuf> = LazyLock::new(|| TARGET_ZETA_DIR.join("worktrees"));
pub static RUN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    TARGET_ZETA_DIR
        .join("runs")
        .join(chrono::Local::now().format("%d-%m-%y-%H_%M_%S").to_string())
});
pub static LATEST_EXAMPLE_RUN_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| TARGET_ZETA_DIR.join("latest"));

pub fn print_run_data_dir(deep: bool, use_color: bool) {
    println!("\n## Run Data\n");
    let mut files = Vec::new();

    let current_dir = std::env::current_dir().unwrap();
    for file in std::fs::read_dir(&*RUN_DIR).unwrap() {
        let file = file.unwrap();
        if file.file_type().unwrap().is_dir() && deep {
            for file in std::fs::read_dir(file.path()).unwrap() {
                let path = file.unwrap().path();
                let path = path.strip_prefix(&current_dir).unwrap_or(&path);
                files.push(format!(
                    "- {}/{}{}{}",
                    path.parent().unwrap().display(),
                    if use_color { "\x1b[34m" } else { "" },
                    path.file_name().unwrap().display(),
                    if use_color { "\x1b[0m" } else { "" },
                ));
            }
        } else {
            let path = file.path();
            let path = path.strip_prefix(&current_dir).unwrap_or(&path);
            files.push(format!(
                "- {}/{}{}{}",
                path.parent().unwrap().display(),
                if use_color { "\x1b[34m" } else { "" },
                path.file_name().unwrap().display(),
                if use_color { "\x1b[0m" } else { "" }
            ));
        }
    }
    files.sort();

    for file in files {
        println!("{}", file);
    }

    println!(
        "\n💡 Tip of the day: {} always points to the latest run\n",
        LATEST_EXAMPLE_RUN_DIR.display()
    );
}
