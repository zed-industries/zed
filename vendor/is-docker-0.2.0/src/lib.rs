extern crate once_cell;

use std::fs;
use once_cell::sync::OnceCell;

fn has_docker_env_file() -> bool {
    fs::metadata("/.dockerenv").is_ok()
}

fn has_docker_in_cgroup() -> bool {
    match fs::read_to_string("/proc/self/cgroup") {
        Ok(file_contents) => file_contents.contains("docker"),
        Err(_error) => false,
    }
}

pub fn is_docker() -> bool {
    static CACHED_RESULT: OnceCell<bool> = OnceCell::new();

    *CACHED_RESULT.get_or_init(|| {
        has_docker_env_file() || has_docker_in_cgroup()
    })   
}
