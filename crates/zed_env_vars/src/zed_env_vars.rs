use std::sync::LazyLock;

/// Whether Zed is running in stateless mode.
/// When true, Zed will use in-memory databases instead of persistent storage.
pub static ZED_STATELESS: LazyLock<bool> =
    LazyLock::new(|| std::env::var("ZED_STATELESS").is_ok_and(|v| !v.is_empty()));
