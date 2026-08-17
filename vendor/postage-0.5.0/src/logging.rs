#![allow(dead_code)]

#[cfg(feature = "debug")]
pub use debug_impl::*;

#[cfg(feature = "debug")]
mod debug_impl {
    use log::LevelFilter;
    use std::sync::Once;

    static LOG: Once = Once::new();

    pub fn enable_log() {
        LOG.call_once(|| {
            simple_logger::SimpleLogger::new()
                .with_level(LevelFilter::Info)
                .init()
                .unwrap();
        });
    }
}

#[cfg(not(feature = "debug"))]
pub fn enable_log() {}
