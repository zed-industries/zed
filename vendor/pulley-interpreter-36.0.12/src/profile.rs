//! Low-level support for profiling pulley.
//!
//! This is used in conjunction with the `profiler-html.rs` example with Pulley
//! and the `pulley.rs` ProfilingAgent in Wasmtime.

use anyhow::{Context, Result, anyhow, bail};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::vec::Vec;

// Header markers for sections in the binary `*.data` file.

/// Section of the `*.data` file which looks like:
///
/// ```text
/// * byte: ID_FUNCTION
/// * addr: 8-byte little-endian address that this body was located at
/// * name_len: 4-byte little-endian byte length of `name`
/// * name: contents of the name of the function
/// * body_len: 4-byte little-endian byte length of `body`
/// * body: contents of the body of the function
/// ```
const ID_FUNCTION: u8 = 1;

/// Section of the `*.data` file which looks like:
///
/// ```text
/// * byte: ID_SAMPLES
/// * sample_len: 4-byte little-endian element count of `samples`
/// * samples: sequence of 8-byte little endian addresses
/// ```
const ID_SAMPLES: u8 = 2;

/// Representation of a currently executing program counter of an interpreter.
///
/// Stores an `Arc` internally that is safe to clone/read from other threads.
#[derive(Default, Clone)]
pub struct ExecutingPc(Arc<ExecutingPcState>);

#[derive(Default)]
struct ExecutingPcState {
    current_pc: AtomicUsize,
    done: AtomicBool,
}

impl ExecutingPc {
    pub(crate) fn as_ref(&self) -> ExecutingPcRef<'_> {
        ExecutingPcRef(&self.0.current_pc)
    }

    /// Loads the currently executing program counter, if the interpreter is
    /// running.
    pub fn get(&self) -> Option<usize> {
        match self.0.current_pc.load(Relaxed) {
            0 => None,
            n => Some(n),
        }
    }

    /// Returns whether the interpreter has been destroyed and will no longer
    /// execute any code.
    pub fn is_done(&self) -> bool {
        self.0.done.load(Relaxed)
    }

    pub(crate) fn set_done(&self) {
        self.0.done.store(true, Relaxed)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct ExecutingPcRef<'a>(&'a AtomicUsize);

impl ExecutingPcRef<'_> {
    pub(crate) fn record(&self, pc: usize) {
        self.0.store(pc, Relaxed);
    }
}

/// Utility to record profiling information to a file.
pub struct Recorder {
    /// The buffered writer used to write profiling data. Note that this is
    /// buffered to amortize the cost of writing out information to the
    /// filesystem to help avoid profiling overhead.
    file: BufWriter<File>,
}

impl Recorder {
    /// Creates a new recorder which will write to the specified filename.
    pub fn new(filename: &str) -> Result<Recorder> {
        Ok(Recorder {
            file: BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(filename)
                    .with_context(|| format!("failed to open `{filename}` for writing"))?,
            ),
        })
    }

    /// Adds a new function that may be sampled in the future.
    ///
    /// This must be given `code` where it resides and will be executed in the
    /// host address space.
    pub fn add_function(&mut self, name: &str, code: &[u8]) -> Result<()> {
        self.file.write_all(&[ID_FUNCTION])?;
        self.file
            .write_all(&u64::try_from(code.as_ptr() as usize)?.to_le_bytes())?;
        self.file
            .write_all(&u32::try_from(name.len())?.to_le_bytes())?;
        self.file.write_all(name.as_bytes())?;
        self.file
            .write_all(&u32::try_from(code.len())?.to_le_bytes())?;
        self.file.write_all(code)?;
        Ok(())
    }

    /// Adds a new set of samples to this recorded.
    pub fn add_samples(&mut self, samples: &mut Samples) -> Result<()> {
        self.file.write_all(&[ID_SAMPLES])?;

        samples.finalize();
        self.file.write_all(&samples.data)?;
        samples.reset();
        Ok(())
    }

    /// Flushes out all pending data to the filesystem.
    pub fn flush(&mut self) -> Result<()> {
        self.file.flush()?;
        Ok(())
    }
}

/// A set of samples of program counters that have been collected over time.
pub struct Samples {
    data: Vec<u8>,
    samples: u32,
}

impl Samples {
    /// Adds a new program counter to this sample.
    pub fn append(&mut self, sample: usize) {
        self.data.extend_from_slice(&(sample as u64).to_le_bytes());
        self.samples += 1;
    }

    /// Returns the number of samples that have been collected.
    pub fn num_samples(&self) -> u32 {
        self.samples
    }

    fn finalize(&mut self) {
        self.data[..4].copy_from_slice(&self.samples.to_le_bytes());
    }

    fn reset(&mut self) {
        self.data.truncate(0);
        self.data.extend_from_slice(&[0; 4]);
        self.samples = 0;
    }
}

impl Default for Samples {
    fn default() -> Samples {
        let mut samples = Samples {
            data: Vec::new(),
            samples: 0,
        };
        samples.reset();
        samples
    }
}

/// Sections that can be parsed from a `*.data` file.
///
/// This is the reverse of [`Recorder`] above.
pub enum Event<'a> {
    /// A named function was loaded at the specified address with the specified
    /// contents.
    Function(u64, &'a str, &'a [u8]),
    /// A set of samples were taken.
    Samples(&'a [SamplePc]),
}

/// A small wrapper around `u64` to reduce its alignment to 1.
#[repr(packed)]
pub struct SamplePc(pub u64);

/// Decodes a `*.data` file presented in its entirety as `bytes` into a sequence
/// of `Event`s.
pub fn decode(mut bytes: &[u8]) -> impl Iterator<Item = Result<Event<'_>>> + use<'_> {
    std::iter::from_fn(move || {
        if bytes.is_empty() {
            None
        } else {
            Some(decode_one(&mut bytes))
        }
    })
}

fn decode_one<'a>(bytes: &mut &'a [u8]) -> Result<Event<'a>> {
    match bytes.split_first().unwrap() {
        (&ID_FUNCTION, rest) => {
            let (addr, rest) = rest
                .split_first_chunk()
                .ok_or_else(|| anyhow!("invalid addr"))?;
            let addr = u64::from_le_bytes(*addr);

            let (name_len, rest) = rest
                .split_first_chunk()
                .ok_or_else(|| anyhow!("invalid name byte len"))?;
            let name_len = u32::from_le_bytes(*name_len);
            let (name, rest) = rest
                .split_at_checked(name_len as usize)
                .ok_or_else(|| anyhow!("invalid name contents"))?;
            let name = std::str::from_utf8(name)?;

            let (body_len, rest) = rest
                .split_first_chunk()
                .ok_or_else(|| anyhow!("invalid body byte len"))?;
            let body_len = u32::from_le_bytes(*body_len);
            let (body, rest) = rest
                .split_at_checked(body_len as usize)
                .ok_or_else(|| anyhow!("invalid body contents"))?;

            *bytes = rest;
            Ok(Event::Function(addr, name, body))
        }

        (&ID_SAMPLES, rest) => {
            let (samples, rest) = rest
                .split_first_chunk()
                .ok_or_else(|| anyhow!("invalid sample count"))?;
            let samples = u32::from_le_bytes(*samples);
            let (samples, rest) = rest
                .split_at_checked(samples as usize * 8)
                .ok_or_else(|| anyhow!("invalid sample data"))?;
            *bytes = rest;

            let (before, mid, after) = unsafe { samples.align_to::<SamplePc>() };
            if !before.is_empty() || !after.is_empty() {
                bail!("invalid sample data contents");
            }
            Ok(Event::Samples(mid))
        }

        _ => bail!("unknown ID in profile"),
    }
}
