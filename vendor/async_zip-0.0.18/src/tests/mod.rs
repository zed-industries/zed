// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

pub(crate) mod combined;
pub(crate) mod read;
pub(crate) mod spec;
pub(crate) mod write;

use std::sync::Once;
static ENV_LOGGER: Once = Once::new();

/// Initialize the env logger for any tests that require it.
/// Safe to call multiple times.
fn init_logger() {
    ENV_LOGGER.call_once(|| env_logger::Builder::from_default_env().format_module_path(true).init());
}
