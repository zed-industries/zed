//! The runtime.

use std::env;

use once_cell::sync::Lazy;

/// Dummy runtime struct.
pub struct Runtime {}

/// The global runtime.
pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    // Create an executor thread pool.

    let thread_name = env::var("ASYNC_STD_THREAD_NAME").unwrap_or_else(|_| "async-std/runtime".to_string());
    async_global_executor::init_with_config(async_global_executor::GlobalExecutorConfig::default().with_env_var("ASYNC_STD_THREAD_COUNT").with_thread_name_fn(move || thread_name.clone()));

    Runtime {}
});
