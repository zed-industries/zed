//! Adds support for profiling JIT-ed code using VTune. By default, VTune
//! support is built in to Wasmtime (configure with the `vtune` feature flag).
//! To enable it at runtime, use the `--profile=vtune` CLI flag.
//!
//! ### Profile
//!
//! ```ignore
//! vtune -run-pass-thru=--no-altstack -v -collect hotspots target/debug/wasmtime --profile=vtune test.wasm
//! ```
//!
//! Note: `vtune` is a command-line tool for VTune which must [be
//! installed](https://www.intel.com/content/www/us/en/developer/tools/oneapi/vtune-profiler.html#standalone)
//! for this to work.

use crate::prelude::*;
use crate::profiling_agent::ProfilingAgent;
use ittapi::jit::MethodLoadBuilder;
use std::sync::Mutex;

/// Interface for driving the ittapi for VTune support
struct VTuneAgent {
    // Note that we use a mutex internally to serialize state updates since multiple threads may be
    // sharing this agent.
    state: Mutex<State>,
}

/// Interface for driving vtune
#[derive(Default)]
struct State {
    vtune: ittapi::jit::Jit,
}

/// Initialize a VTuneAgent.
pub fn new() -> Result<Box<dyn ProfilingAgent>> {
    Ok(Box::new(VTuneAgent {
        state: Mutex::new(State {
            vtune: Default::default(),
        }),
    }))
}

impl Drop for VTuneAgent {
    fn drop(&mut self) {
        self.state.lock().unwrap().event_shutdown();
    }
}

impl State {
    /// Notify vtune about a newly tracked code region.
    fn notify_code(&mut self, module_name: &str, method_name: &str, code: &[u8]) {
        self.vtune
            .load_method(
                MethodLoadBuilder::new(method_name.to_owned(), code.as_ptr(), code.len())
                    .class_file_name(module_name.to_owned())
                    .source_file_name("<unknown wasm filename>".to_owned()),
            )
            .unwrap();
    }

    /// Shutdown module
    fn event_shutdown(&mut self) {
        // Ignore if something went wrong.
        let _ = self.vtune.shutdown();
    }
}

impl ProfilingAgent for VTuneAgent {
    fn register_function(&self, name: &str, code: &[u8]) {
        self.state.lock().unwrap().register_function(name, code);
    }
}

impl State {
    fn register_function(&mut self, name: &str, code: &[u8]) {
        self.notify_code("wasmtime", name, code);
    }
}
