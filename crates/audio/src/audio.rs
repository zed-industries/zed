#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
mod audio_pipeline;

#[cfg(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd"))]
mod fake_pipeline {
}
