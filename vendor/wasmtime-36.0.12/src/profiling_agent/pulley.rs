//! Basic support for emitting a `*.data` file which contains samples of pulley
//! bytecode.
//!
//! Pulley is Wasmtime's interpreter and native profilers are not good at
//! profiling bytecode interpreters because they show hot bytecode instructions
//! but we're instead often interested in the shape of the bytecode itself
//! around the hot instruction, for example to identify new macro-instructions
//! to add to Pulley. This module serves as a means of collecting data from
//! Pulley being executed in-process and serializing it to a file.
//!
//! The file collected here is populated by a sampling thread in-process. This
//! sampling thread only collects the current program counter of any interpreters
//! in the process. This does not collect stack traces at all. That means that
//! this profiler is only suitable for looking at "self time" and is not
//! suitable for getting a broader picture of what's going on (e.g. why
//! something was called in the first place).
//!
//! The general design of this profiler is:
//!
//! * Support for this all requires a `pulley-profile` feature at compile-time
//!   as it's generally a perf hit to the interpreter loop.
//! * Each Pulley interpreter updates an `AtomicUsize` before all instructions
//!   with the current PC that it's executing.
//! * This module spawns a "sampling thread" which will, at some frequency,
//!   collect all the PCs of all interpreters in the process.
//! * Once enough samples have been collected they're flushed out to a data file
//!   on a second thread, the "recording thread".
//!
//! The hope is that the sampling thread stays as steady as possible in its
//! sampling rate while not hitting OOM conditions in the process or anything
//! like that. The `*.data` file that's emitted is intended to be processed by
//! example code in the `pulley-interpreter` crate or `pulley/examples/*.rs` in
//! the Wasmtime repository.

use crate::prelude::*;
use crate::profiling_agent::ProfilingAgent;
use crate::vm::Interpreter;
use pulley_interpreter::profile::{ExecutingPc, Recorder, Samples};
use std::mem;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// Implementation of `ProfilingAgent` from the Wasmtime crate.
struct PulleyAgent {
    state: Arc<State>,

    /// Handle to the thread performing periodic sampling. This is joined on
    /// `Drop` of this structure so it's not a daemon thread permanently.
    sampling_thread: Option<JoinHandle<()>>,

    /// Same as the sampling thread above, but for recording data to the
    /// filesystem.
    recording_thread: Option<JoinHandle<()>>,
}

struct State {
    /// Protected state about the recorder, or the file being created. This is
    /// accessed both from the "recording thread" as well as `Engine` threads to
    /// register new pulley bytecode.
    recorder: Mutex<Recorder>,

    /// Protected state about sampling interpreters. This is accessed both from
    /// the "sampling thread" primarily but is additionally accessed from
    /// `Engine` threads to register new interpreters coming online.
    sampling: Mutex<SamplingState>,

    /// Condition variable which is signaled when sampling should cease and
    /// exit. This is coupled with `Drop for PulleyAgent`.
    sampling_done: Condvar,

    /// The frequency at which samples are collected. Defaults to 1000 but can
    /// be configured with the `PULLEY_SAMPLING_FREQ` environment variable.
    sampling_freq: u32,

    /// Number of samples to buffer before flushing them to a file. Defaults to
    /// 20000 but can be configured with the `PULLEY_SAMPLING_FLUSH_AMT`
    /// environment variable.
    sampling_flush_amt: u32,
}

/// State protected by a mutex in `State` above related to sampling.
#[derive(Default)]
struct SamplingState {
    /// All interpreters known to be executing. This is a list of
    /// pointers-to-the-current-PC which is updated whenever the interpreter
    /// executes an instruction.
    interpreters: Vec<ExecutingPc>,

    /// Current list of samples that have been collected.
    samples: Samples,
}

pub fn new() -> Result<Box<dyn ProfilingAgent>> {
    let pid = std::process::id();
    let filename = format!("./pulley-{pid}.data");
    let mut agent = PulleyAgent {
        state: Arc::new(State {
            recorder: Mutex::new(Recorder::new(&filename)?),
            sampling: Default::default(),
            sampling_done: Condvar::new(),
            sampling_freq: std::env::var("PULLEY_SAMPLING_FREQ")
                .ok()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(1_000),
            sampling_flush_amt: std::env::var("PULLEY_SAMPLING_FLUSH_AMT")
                .ok()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(20_000),
        }),
        sampling_thread: None,
        recording_thread: None,
    };

    let (tx, rx) = mpsc::channel();
    let state = agent.state.clone();
    agent.sampling_thread = Some(thread::spawn(move || sampling_thread(&state, tx)));
    let state = agent.state.clone();
    agent.recording_thread = Some(thread::spawn(move || recording_thread(&state, rx)));

    Ok(Box::new(agent))
}

impl ProfilingAgent for PulleyAgent {
    /// New functions are registered with `Recorder` to record the exact
    /// bytecode so disassembly is available during profile analysis.
    ///
    /// Note that this also provides the native address that code is loaded at
    /// so samples know what code it's within.
    fn register_function(&self, name: &str, code: &[u8]) {
        self.state
            .recorder
            .lock()
            .unwrap()
            .add_function(name, code)
            .expect("failed to register pulley function");
    }

    /// Registers a new interpreter coming online. Interpreters, with
    /// `pulley-profile` enabled, store a shadow program counter updated on each
    /// instruction which we can read from a different thread.
    fn register_interpreter(&self, interpreter: &Interpreter) {
        let pc = interpreter.pulley().executing_pc();
        self.state
            .sampling
            .lock()
            .unwrap()
            .interpreters
            .push(pc.clone());
    }
}

/// Execution of the thread responsible for sampling interpreters.
///
/// This thread has a few tasks:
///
/// * Needs to sample, at `state.sampling_freq`, the state of all known
///   interpreters. Ideally this sampling is as steady as possible.
/// * Needs to clean up interpreters which have been destroyed as there's
///   otherwise no hook for doing so.
/// * Needs to send batches of samples to the recording thread to get written to
///   the filesystem.
fn sampling_thread(state: &State, to_record: mpsc::Sender<Samples>) {
    // Calculate the `Duration` between each sample which will be in
    // nanoseconds. This duration is then used to create an `Instant` in time
    // where we'll be collecting the next sample.
    let between_ticks = Duration::new(0, 1_000_000_000 / state.sampling_freq);
    let start = Instant::now();
    let mut next_sample = start + between_ticks;

    // Helper closure to send off a batch of samples to the recording thread.
    // Note that recording is done off-thread to ensure that the filesystem I/O
    // interferes as little as possible with the sampling rate here.
    let record = |sampling: &mut SamplingState| {
        if sampling.samples.num_samples() == 0 {
            return;
        }
        let samples = mem::take(&mut sampling.samples);
        to_record.send(samples).unwrap();
    };

    let mut sampling = state.sampling.lock().unwrap();

    loop {
        // Calculate the duration, from this current moment in time, to when the
        // next sample is supposed to be taken. If the next sampling time is in
        // the past then this won't sleep but will still check the condvar.
        let dur = next_sample
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::new(0, 0));

        // Wait on `state.sampling_done`, but with the timeout we've calculated.
        // If this times out that means that the next sample can proceed.
        // Otherwise if this did not time out then it means that sampling should
        // cease as the profiler is being destroyed.
        let (guard, result) = state.sampling_done.wait_timeout(sampling, dur).unwrap();
        sampling = guard;
        if !result.timed_out() {
            break;
        }

        // Now that we've decided to take a sample increment the next sample
        // time by our interval. Once we're done sampling below we'll then sleep
        // again up to this time.
        next_sample += between_ticks;

        // Sample the state of all interpreters known. This first starts by
        // discarding any interpreters that are offline. Samples without a PC
        // are additionally discarded as it means the interpreter is inactive.
        //
        // Once enough samples have been collected they're flushed to the
        // recording thread.
        let SamplingState {
            interpreters,
            samples,
        } = &mut *sampling;
        interpreters.retain(|a| !a.is_done());
        for interpreter in interpreters.iter() {
            if let Some(pc) = interpreter.get() {
                samples.append(pc);
            }
        }
        if samples.num_samples() > state.sampling_flush_amt {
            record(&mut sampling);
        }
    }

    // Send any final samples to the recording thread after the loop has exited.
    record(&mut sampling);
}

/// Helper thread responsible for writing samples to the filesystem.
///
/// This receives samples over `to_record` and then performs the filesystem I/O
/// necessary to write them out. This thread completes once `to_record` is
/// closed, or when the sampling thread completes. At that time all data in the
/// recorder is flushed out as well.
fn recording_thread(state: &State, to_record: mpsc::Receiver<Samples>) {
    for mut samples in to_record {
        state
            .recorder
            .lock()
            .unwrap()
            .add_samples(&mut samples)
            .expect("failed to write samples");
    }

    state.recorder.lock().unwrap().flush().unwrap();
}

impl Drop for PulleyAgent {
    fn drop(&mut self) {
        // First notify the sampling thread that it's time to shut down and
        // wait for it to exit.
        self.state.sampling_done.notify_one();
        self.sampling_thread.take().unwrap().join().unwrap();

        // Wait on the recording thread as well which should terminate once
        // `sampling_thread` has terminated as well.
        self.recording_thread.take().unwrap().join().unwrap();
    }
}
