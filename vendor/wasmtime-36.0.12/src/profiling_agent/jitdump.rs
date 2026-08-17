//! Support for jitdump files which can be used by perf for profiling jitted code.
//! Spec definitions for the output format is as described here:
//! <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/perf/Documentation/jitdump-specification.txt>
//!
//! Usage Example:
//!     Record
//!         sudo perf record -k 1 -e instructions:u target/debug/wasmtime -g --profile=jitdump test.wasm
//!     Combine
//!         sudo perf inject -v -j -i perf.data -o perf.jit.data
//!     Report
//!         sudo perf report -i perf.jit.data -F+period,srcline
//! Note: For descriptive results, the WASM file being executed should contain dwarf debug data

use crate::prelude::*;
use crate::profiling_agent::ProfilingAgent;
use object::elf;
use std::process;
use std::sync::Mutex;
use target_lexicon::Architecture;
use wasmtime_environ::Unsigned;
use wasmtime_jit_debug::perf_jitdump::*;

/// Interface for driving the creation of jitdump files
struct JitDumpAgent {
    pid: u32,
}

/// Process-wide JIT dump file. Perf only accepts a unique file per process, in the injection step.
static JITDUMP_FILE: Mutex<Option<JitDumpFile>> = Mutex::new(None);

/// Initialize a JitDumpAgent and write out the header.
pub fn new() -> Result<Box<dyn ProfilingAgent>> {
    let mut jitdump_file = JITDUMP_FILE.lock().unwrap();

    if jitdump_file.is_none() {
        let filename = format!("./jit-{}.dump", process::id());
        let e_machine = match target_lexicon::HOST.architecture {
            Architecture::X86_64 => elf::EM_X86_64 as u32,
            Architecture::X86_32(_) => elf::EM_386 as u32,
            Architecture::Arm(_) => elf::EM_ARM as u32,
            Architecture::Aarch64(_) => elf::EM_AARCH64 as u32,
            Architecture::S390x => elf::EM_S390 as u32,
            _ => unimplemented!("unrecognized architecture"),
        };
        *jitdump_file = Some(JitDumpFile::new(filename, e_machine)?);
    }

    Ok(Box::new(JitDumpAgent {
        pid: std::process::id(),
    }))
}

impl ProfilingAgent for JitDumpAgent {
    fn register_function(&self, name: &str, code: &[u8]) {
        let mut jitdump_file = JITDUMP_FILE.lock().unwrap();
        let jitdump_file = jitdump_file.as_mut().unwrap();
        let timestamp = jitdump_file.get_time_stamp();
        let tid = rustix::thread::gettid().as_raw_nonzero().get().unsigned();
        if let Err(err) = jitdump_file.dump_code_load_record(&name, code, timestamp, self.pid, tid)
        {
            println!("Jitdump: write_code_load_failed_record failed: {err:?}\n");
        }
    }
}
