// `WriterError` is large and clippy doesn't like that, but not a huge deal atm
#![allow(clippy::result_large_err)]

#[cfg(target_os = "android")]
mod android;
pub mod app_memory;
pub(crate) mod auxv;
pub mod crash_context;
mod dso_debug;
mod dumper_cpu_info;
pub mod maps_reader;
pub mod mem_reader;
pub mod minidump_writer;
pub mod module_reader;
mod serializers;
pub mod thread_info;

pub use maps_reader::LINUX_GATE_LIBRARY_NAME;
pub type Pid = i32;
