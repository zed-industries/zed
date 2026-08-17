
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch="aarch64"))]
macro_rules! compile_env_matches_or_is_empty {
    ($envvar:tt, $feature_name:tt) => {
        (match option_env!($envvar) {
            None => true,
            Some(v) => v == $feature_name
        })
    }
}

