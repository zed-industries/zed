//! Bindings for WASIp1 aka Preview 1 aka `wasi_snapshot_preview1`.
//!
//! This module contains runtime support for configuring and executing
//! WASIp1-using core WebAssembly modules. Support for WASIp1 is built on top of
//! support for WASIp2 available at [the crate root](crate), but that's just an
//! internal implementation detail.
//!
//! Unlike the crate root, support for WASIp1 centers around two APIs:
//!
//! * [`WasiP1Ctx`]
//! * [`add_to_linker_sync`] (or [`add_to_linker_async`])
//!
//! First a [`WasiCtxBuilder`] will be used and finalized with the [`build_p1`]
//! method to create a [`WasiCtx`]. Next a [`wasmtime::Linker`] is configured
//! with WASI imports by using the `add_to_linker_*` desired (sync or async
//! depending on [`Config::async_support`]).
//!
//! Note that WASIp1 is not as extensible or configurable as WASIp2 so the
//! support in this module is enough to run wasm modules but any customization
//! beyond that [`WasiCtxBuilder`] already supports is not possible yet.
//!
//! [`WasiCtxBuilder`]: crate::p2::WasiCtxBuilder
//! [`build_p1`]: crate::p2::WasiCtxBuilder::build_p1
//! [`Config::async_support`]: wasmtime::Config::async_support
//!
//! # Components vs Modules
//!
//! Note that WASIp1 does not work for components at this time, only core wasm
//! modules. That means this module is only for users of [`wasmtime::Module`]
//! and [`wasmtime::Linker`], for example. If you're using
//! [`wasmtime::component::Component`] or [`wasmtime::component::Linker`] you'll
//! want the WASIp2 [support this crate has](crate) instead.
//!
//! # Examples
//!
//! ```no_run
//! use wasmtime::{Result, Engine, Linker, Module, Store};
//! use wasmtime_wasi::preview1::{self, WasiP1Ctx};
//! use wasmtime_wasi::WasiCtxBuilder;
//!
//! // An example of executing a WASIp1 "command"
//! fn main() -> Result<()> {
//!     let args = std::env::args().skip(1).collect::<Vec<_>>();
//!     let engine = Engine::default();
//!     let module = Module::from_file(&engine, &args[0])?;
//!
//!     let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
//!     preview1::add_to_linker_async(&mut linker, |t| t)?;
//!     let pre = linker.instantiate_pre(&module)?;
//!
//!     let wasi_ctx = WasiCtxBuilder::new()
//!         .inherit_stdio()
//!         .inherit_env()
//!         .args(&args)
//!         .build_p1();
//!
//!     let mut store = Store::new(&engine, wasi_ctx);
//!     let instance = pre.instantiate(&mut store)?;
//!     let func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
//!     func.call(&mut store, ())?;
//!
//!     Ok(())
//! }
//! ```

use crate::p2::bindings::{
    cli::{
        stderr::Host as _, stdin::Host as _, stdout::Host as _, terminal_input, terminal_output,
        terminal_stderr::Host as _, terminal_stdin::Host as _, terminal_stdout::Host as _,
    },
    clocks::{monotonic_clock, wall_clock},
    filesystem::{preopens::Host as _, types as filesystem},
};
use crate::p2::{FsError, IsATTY};
use crate::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
use anyhow::{Context, bail};
use std::collections::{BTreeMap, BTreeSet, HashSet, btree_map};
use std::mem::{self, size_of, size_of_val};
use std::slice;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use system_interface::fs::FileIoExt;
use wasmtime::component::Resource;
use wasmtime_wasi_io::{
    bindings::wasi::io::streams,
    streams::{StreamError, StreamResult},
};
use wiggle::tracing::instrument;
use wiggle::{GuestError, GuestMemory, GuestPtr, GuestType};

// Bring all WASI traits in scope that this implementation builds on.
use crate::p2::bindings::cli::environment::Host as _;
use crate::p2::bindings::filesystem::types::HostDescriptor as _;
use crate::p2::bindings::random::random::Host as _;
use wasmtime_wasi_io::bindings::wasi::io::poll::Host as _;

/// Structure containing state for WASIp1.
///
/// This structure is created through [`WasiCtxBuilder::build_p1`] and is
/// configured through the various methods of [`WasiCtxBuilder`]. This structure
/// itself implements generated traits for WASIp1 as well as [`WasiView`] to
/// have access to WASIp2.
///
/// Instances of [`WasiP1Ctx`] are typically stored within the `T` of
/// [`Store<T>`](wasmtime::Store).
///
/// [`WasiCtxBuilder::build_p1`]: crate::p2::WasiCtxBuilder::build_p1
/// [`WasiCtxBuilder`]: crate::p2::WasiCtxBuilder
///
/// # Examples
///
/// ```no_run
/// use wasmtime::{Result, Linker};
/// use wasmtime_wasi::preview1::{self, WasiP1Ctx};
/// use wasmtime_wasi::WasiCtxBuilder;
///
/// struct MyState {
///     // ... custom state as necessary ...
///
///     wasi: WasiP1Ctx,
/// }
///
/// impl MyState {
///     fn new() -> MyState {
///         MyState {
///             // .. initialize custom state if needed ..
///
///             wasi: WasiCtxBuilder::new()
///                 .arg("./foo.wasm")
///                 // .. more customization if necesssary ..
///                 .build_p1(),
///         }
///     }
/// }
///
/// fn add_to_linker(linker: &mut Linker<MyState>) -> Result<()> {
///     preview1::add_to_linker_sync(linker, |my_state| &mut my_state.wasi)?;
///     Ok(())
/// }
/// ```
pub struct WasiP1Ctx {
    table: ResourceTable,
    wasi: WasiCtx,
    adapter: WasiPreview1Adapter,
    hostcall_fuel: usize,
}

impl WasiP1Ctx {
    pub(crate) fn new(wasi: WasiCtx) -> Self {
        Self {
            table: ResourceTable::new(),
            wasi,
            adapter: WasiPreview1Adapter::new(),
            hostcall_fuel: 0,
        }
    }

    fn as_wasi_impl(&mut self) -> WasiCtxView<'_> {
        WasiView::ctx(self)
    }

    fn as_io_impl(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn consume_fuel(&mut self, fuel: usize) -> Result<()> {
        if fuel > self.hostcall_fuel {
            return Err(types::Errno::Nomem.into());
        }
        self.hostcall_fuel -= fuel;
        Ok(())
    }

    /// Assumes the host is going to copy all of `array` in which case a
    /// corresponding amount of fuel is consumed to ensure it's not too large.
    fn consume_fuel_for_array<T>(&mut self, array: wiggle::GuestPtr<[T]>) -> Result<()> {
        let byte_size = usize::try_from(array.len())?
            .checked_mul(size_of::<T>())
            .ok_or(types::Errno::Overflow)?;
        self.consume_fuel(byte_size)
    }

    /// Returns the first non-empty buffer in `ciovs` or a single empty buffer
    /// if they're all empty.
    ///
    /// Additionally consumes a corresponding amount of fuel appropriate to the
    /// size of `ciovs` and the first non-empty array.
    fn first_non_empty_ciovec(
        &mut self,
        memory: &GuestMemory<'_>,
        ciovs: types::CiovecArray,
    ) -> Result<GuestPtr<[u8]>> {
        self.consume_fuel_for_array(ciovs)?;
        for iov in ciovs.iter() {
            let iov = memory.read(iov?)?;
            if iov.buf_len == 0 {
                continue;
            }
            let ret = iov.buf.as_array(iov.buf_len);
            self.consume_fuel_for_array(ret)?;
            return Ok(ret);
        }
        Ok(GuestPtr::new((0, 0)))
    }

    /// Returns the first non-empty buffer in `iovs` or a single empty buffer if
    /// they're all empty.
    ///
    /// Additionally consumes a corresponding amount of fuel appropriate to the
    /// size of `ciovs` and the first non-empty array.
    fn first_non_empty_iovec(
        &mut self,
        memory: &GuestMemory<'_>,
        iovs: types::IovecArray,
    ) -> Result<GuestPtr<[u8]>> {
        self.consume_fuel_for_array(iovs)?;
        for iov in iovs.iter() {
            let iov = memory.read(iov?)?;
            if iov.buf_len == 0 {
                continue;
            }
            let ret = iov.buf.as_array(iov.buf_len);
            self.consume_fuel_for_array(ret)?;
            return Ok(ret);
        }
        Ok(GuestPtr::new((0, 0)))
    }

    /// Copies the guest string `ptr` into the host.
    ///
    /// Consumes a corresponding amount of fuel for the byte size of `ptr` and
    /// fails if it's too large.
    fn read_string(&mut self, memory: &GuestMemory<'_>, ptr: GuestPtr<str>) -> Result<String> {
        self.consume_fuel(usize::try_from(ptr.len())?)?;
        Ok(memory.as_cow_str(ptr)?.into_owned())
    }
}

impl WasiView for WasiP1Ctx {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

#[derive(Debug)]
struct File {
    /// The handle to the preview2 descriptor of type [`crate::p2::filesystem::Descriptor::File`].
    fd: Resource<filesystem::Descriptor>,

    /// The current-position pointer.
    position: Arc<AtomicU64>,

    /// In append mode, all writes append to the file.
    append: bool,

    /// When blocking, read and write calls dispatch to blocking_read and
    /// blocking_check_write on the underlying streams. When false, read and write
    /// dispatch to stream's plain read and check_write.
    blocking_mode: BlockingMode,
}

/// NB: preview1 files always use blocking writes regardless of what
/// they're configured to use since OSes don't have nonblocking
/// reads/writes anyway. This behavior originated in the first
/// implementation of WASIp1 where flags were propagated to the
/// OS and the OS ignored the nonblocking flag for files
/// generally.
#[derive(Clone, Copy, Debug)]
enum BlockingMode {
    Blocking,
    NonBlocking,
}
impl BlockingMode {
    fn from_fdflags(flags: &types::Fdflags) -> Self {
        if flags.contains(types::Fdflags::NONBLOCK) {
            BlockingMode::NonBlocking
        } else {
            BlockingMode::Blocking
        }
    }
    async fn read(
        &self,
        host: &mut impl streams::HostInputStream,
        input_stream: Resource<streams::InputStream>,
        max_size: usize,
    ) -> Result<Vec<u8>, types::Error> {
        let max_size = max_size.try_into().unwrap_or(u64::MAX);
        match streams::HostInputStream::blocking_read(host, input_stream, max_size).await {
            Ok(r) if r.is_empty() => Err(types::Errno::Intr.into()),
            Ok(r) => Ok(r),
            Err(StreamError::Closed) => Ok(Vec::new()),
            Err(e) => Err(e.into()),
        }
    }
    async fn write(
        &self,
        memory: &mut GuestMemory<'_>,
        host: &mut impl streams::HostOutputStream,
        output_stream: Resource<streams::OutputStream>,
        bytes: GuestPtr<[u8]>,
    ) -> StreamResult<usize> {
        use streams::HostOutputStream as Streams;

        let bytes = memory
            .as_cow(bytes)
            .map_err(|e| StreamError::Trap(e.into()))?;
        let mut bytes = &bytes[..];

        let total = bytes.len();
        while !bytes.is_empty() {
            // NOTE: blocking_write_and_flush takes at most one 4k buffer.
            let len = bytes.len().min(4096);
            let (chunk, rest) = bytes.split_at(len);
            bytes = rest;

            Streams::blocking_write_and_flush(host, output_stream.borrowed(), Vec::from(chunk))
                .await?
        }

        Ok(total)
    }
}

#[derive(Debug)]
enum Descriptor {
    Stdin {
        stream: Resource<streams::InputStream>,
        isatty: IsATTY,
    },
    Stdout {
        stream: Resource<streams::OutputStream>,
        isatty: IsATTY,
    },
    Stderr {
        stream: Resource<streams::OutputStream>,
        isatty: IsATTY,
    },
    /// A fd of type [`crate::p2::filesystem::Descriptor::Dir`]
    Directory {
        fd: Resource<filesystem::Descriptor>,
        /// The path this directory was preopened as.
        /// `None` means this directory was opened using `open-at`.
        preopen_path: Option<String>,
    },
    /// A fd of type [`crate::p2::filesystem::Descriptor::File`]
    File(File),
}

#[derive(Debug, Default)]
struct WasiPreview1Adapter {
    descriptors: Option<Descriptors>,
}

#[derive(Debug, Default)]
struct Descriptors {
    used: BTreeMap<u32, Descriptor>,
    free: BTreeSet<u32>,
}

impl Descriptors {
    /// Initializes [Self] using `preopens`
    fn new(mut host: WasiCtxView<'_>) -> Result<Self, types::Error> {
        let mut descriptors = Self::default();
        descriptors.push(Descriptor::Stdin {
            stream: host
                .get_stdin()
                .context("failed to call `get-stdin`")
                .map_err(types::Error::trap)?,
            isatty: if let Some(term_in) = host
                .get_terminal_stdin()
                .context("failed to call `get-terminal-stdin`")
                .map_err(types::Error::trap)?
            {
                terminal_input::HostTerminalInput::drop(&mut host, term_in)
                    .context("failed to call `drop-terminal-input`")
                    .map_err(types::Error::trap)?;
                IsATTY::Yes
            } else {
                IsATTY::No
            },
        })?;
        descriptors.push(Descriptor::Stdout {
            stream: host
                .get_stdout()
                .context("failed to call `get-stdout`")
                .map_err(types::Error::trap)?,
            isatty: if let Some(term_out) = host
                .get_terminal_stdout()
                .context("failed to call `get-terminal-stdout`")
                .map_err(types::Error::trap)?
            {
                terminal_output::HostTerminalOutput::drop(&mut host, term_out)
                    .context("failed to call `drop-terminal-output`")
                    .map_err(types::Error::trap)?;
                IsATTY::Yes
            } else {
                IsATTY::No
            },
        })?;
        descriptors.push(Descriptor::Stderr {
            stream: host
                .get_stderr()
                .context("failed to call `get-stderr`")
                .map_err(types::Error::trap)?,
            isatty: if let Some(term_out) = host
                .get_terminal_stderr()
                .context("failed to call `get-terminal-stderr`")
                .map_err(types::Error::trap)?
            {
                terminal_output::HostTerminalOutput::drop(&mut host, term_out)
                    .context("failed to call `drop-terminal-output`")
                    .map_err(types::Error::trap)?;
                IsATTY::Yes
            } else {
                IsATTY::No
            },
        })?;

        for dir in host
            .get_directories()
            .context("failed to call `get-directories`")
            .map_err(types::Error::trap)?
        {
            descriptors.push(Descriptor::Directory {
                fd: dir.0,
                preopen_path: Some(dir.1),
            })?;
        }
        Ok(descriptors)
    }

    /// Returns next descriptor number, which was never assigned
    fn unused(&self) -> Result<u32> {
        match self.used.last_key_value() {
            Some((fd, _)) => {
                if let Some(fd) = fd.checked_add(1) {
                    return Ok(fd);
                }
                if self.used.len() == u32::MAX as usize {
                    return Err(types::Errno::Loop.into());
                }
                // TODO: Optimize
                Ok((0..u32::MAX)
                    .rev()
                    .find(|fd| !self.used.contains_key(fd))
                    .expect("failed to find an unused file descriptor"))
            }
            None => Ok(0),
        }
    }

    /// Pushes the [Descriptor] returning corresponding number.
    /// This operation will try to reuse numbers previously removed via [`Self::remove`]
    /// and rely on [`Self::unused`] if no free numbers are recorded
    fn push(&mut self, desc: Descriptor) -> Result<u32> {
        let fd = if let Some(fd) = self.free.pop_last() {
            fd
        } else {
            self.unused()?
        };
        assert!(self.used.insert(fd, desc).is_none());
        Ok(fd)
    }
}

impl WasiPreview1Adapter {
    fn new() -> Self {
        Self::default()
    }
}

/// A mutably-borrowed [`WasiPreview1View`] implementation, which provides access to the stored
/// state. It can be thought of as an in-flight [`WasiPreview1Adapter`] transaction, all
/// changes will be recorded in the underlying [`WasiPreview1Adapter`] returned by
/// [`WasiPreview1View::adapter_mut`] on [`Drop`] of this struct.
// NOTE: This exists for the most part just due to the fact that `bindgen` generates methods with
// `&mut self` receivers and so this struct lets us extend the lifetime of the `&mut self` borrow
// of the [`WasiPreview1View`] to provide means to return mutably and immutably borrowed [`Descriptors`]
// without having to rely on something like `Arc<Mutex<Descriptors>>`, while also being able to
// call methods like [`Descriptor::is_file`] and hiding complexity from preview1 method implementations.
struct Transaction<'a> {
    view: &'a mut WasiP1Ctx,
    descriptors: Descriptors,
}

impl Drop for Transaction<'_> {
    /// Record changes in the [`WasiPreview1Adapter`] returned by [`WasiPreview1View::adapter_mut`]
    fn drop(&mut self) {
        let descriptors = mem::take(&mut self.descriptors);
        self.view.adapter.descriptors = Some(descriptors);
    }
}

impl Transaction<'_> {
    /// Borrows [`Descriptor`] corresponding to `fd`.
    ///
    /// # Errors
    ///
    /// Returns [`types::Errno::Badf`] if no [`Descriptor`] is found
    fn get_descriptor(&self, fd: types::Fd) -> Result<&Descriptor> {
        let fd = fd.into();
        let desc = self.descriptors.used.get(&fd).ok_or(types::Errno::Badf)?;
        Ok(desc)
    }

    /// Borrows [`File`] corresponding to `fd`
    /// if it describes a [`Descriptor::File`]
    fn get_file(&self, fd: types::Fd) -> Result<&File> {
        let fd = fd.into();
        match self.descriptors.used.get(&fd) {
            Some(Descriptor::File(file)) => Ok(file),
            _ => Err(types::Errno::Badf.into()),
        }
    }

    /// Mutably borrows [`File`] corresponding to `fd`
    /// if it describes a [`Descriptor::File`]
    fn get_file_mut(&mut self, fd: types::Fd) -> Result<&mut File> {
        let fd = fd.into();
        match self.descriptors.used.get_mut(&fd) {
            Some(Descriptor::File(file)) => Ok(file),
            _ => Err(types::Errno::Badf.into()),
        }
    }

    /// Borrows [`File`] corresponding to `fd`
    /// if it describes a [`Descriptor::File`]
    ///
    /// # Errors
    ///
    /// Returns [`types::Errno::Spipe`] if the descriptor corresponds to stdio
    fn get_seekable(&self, fd: types::Fd) -> Result<&File> {
        let fd = fd.into();
        match self.descriptors.used.get(&fd) {
            Some(Descriptor::File(file)) => Ok(file),
            Some(
                Descriptor::Stdin { .. } | Descriptor::Stdout { .. } | Descriptor::Stderr { .. },
            ) => {
                // NOTE: legacy implementation returns SPIPE here
                Err(types::Errno::Spipe.into())
            }
            _ => Err(types::Errno::Badf.into()),
        }
    }

    /// Returns [`filesystem::Descriptor`] corresponding to `fd`
    fn get_fd(&self, fd: types::Fd) -> Result<Resource<filesystem::Descriptor>> {
        match self.get_descriptor(fd)? {
            Descriptor::File(File { fd, .. }) => Ok(fd.borrowed()),
            Descriptor::Directory { fd, .. } => Ok(fd.borrowed()),
            Descriptor::Stdin { .. } | Descriptor::Stdout { .. } | Descriptor::Stderr { .. } => {
                Err(types::Errno::Badf.into())
            }
        }
    }

    /// Returns [`filesystem::Descriptor`] corresponding to `fd`
    /// if it describes a [`Descriptor::File`]
    fn get_file_fd(&self, fd: types::Fd) -> Result<Resource<filesystem::Descriptor>> {
        self.get_file(fd).map(|File { fd, .. }| fd.borrowed())
    }

    /// Returns [`filesystem::Descriptor`] corresponding to `fd`
    /// if it describes a [`Descriptor::Directory`]
    fn get_dir_fd(&self, fd: types::Fd) -> Result<Resource<filesystem::Descriptor>> {
        let fd = fd.into();
        match self.descriptors.used.get(&fd) {
            Some(Descriptor::Directory { fd, .. }) => Ok(fd.borrowed()),
            _ => Err(types::Errno::Badf.into()),
        }
    }
}

impl WasiP1Ctx {
    /// Lazily initializes [`WasiPreview1Adapter`] returned by [`WasiPreview1View::adapter_mut`]
    /// and returns [`Transaction`] on success
    fn transact(&mut self) -> Result<Transaction<'_>, types::Error> {
        let descriptors = if let Some(descriptors) = self.adapter.descriptors.take() {
            descriptors
        } else {
            Descriptors::new(self.as_wasi_impl())?
        };
        Ok(Transaction {
            view: self,
            descriptors,
        })
    }

    /// Lazily initializes [`WasiPreview1Adapter`] returned by [`WasiPreview1View::adapter_mut`]
    /// and returns [`filesystem::Descriptor`] corresponding to `fd`
    fn get_fd(&mut self, fd: types::Fd) -> Result<Resource<filesystem::Descriptor>, types::Error> {
        let st = self.transact()?;
        let fd = st.get_fd(fd)?;
        Ok(fd)
    }

    /// Lazily initializes [`WasiPreview1Adapter`] returned by [`WasiPreview1View::adapter_mut`]
    /// and returns [`filesystem::Descriptor`] corresponding to `fd`
    /// if it describes a [`Descriptor::File`] of [`crate::p2::filesystem::File`] type
    fn get_file_fd(
        &mut self,
        fd: types::Fd,
    ) -> Result<Resource<filesystem::Descriptor>, types::Error> {
        let st = self.transact()?;
        let fd = st.get_file_fd(fd)?;
        Ok(fd)
    }

    /// Lazily initializes [`WasiPreview1Adapter`] returned by [`WasiPreview1View::adapter_mut`]
    /// and returns [`filesystem::Descriptor`] corresponding to `fd`
    /// if it describes a [`Descriptor::File`] or [`Descriptor::PreopenDirectory`]
    /// of [`crate::p2::filesystem::Dir`] type
    fn get_dir_fd(
        &mut self,
        fd: types::Fd,
    ) -> Result<Resource<filesystem::Descriptor>, types::Error> {
        let st = self.transact()?;
        let fd = st.get_dir_fd(fd)?;
        Ok(fd)
    }

    /// Shared implementation of `fd_write` and `fd_pwrite`.
    async fn fd_write_impl(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        ciovs: types::CiovecArray,
        write: FdWrite,
    ) -> Result<types::Size, types::Error> {
        let buf = self.first_non_empty_ciovec(memory, ciovs)?;
        let t = self.transact()?;
        let desc = t.get_descriptor(fd)?;
        match desc {
            Descriptor::File(File {
                fd,
                append,
                position,
                // NB: files always use blocking writes regardless of what
                // they're configured to use since OSes don't have nonblocking
                // reads/writes anyway. This behavior originated in the first
                // implementation of WASIp1 where flags were propagated to the
                // OS and the OS ignored the nonblocking flag for files
                // generally.
                blocking_mode: _,
            }) => {
                let fd = fd.borrowed();
                let position = position.clone();
                let pos = position.load(Ordering::Relaxed);
                let append = *append;
                drop(t);
                let f = self.table.get(&fd)?.file()?;

                let do_write = move |f: &cap_std::fs::File, buf: &[u8]| match (append, write) {
                    // Note that this is implementing Linux semantics of
                    // `pwrite` where the offset is ignored if the file was
                    // opened in append mode.
                    (true, _) => f.append(&buf),
                    (false, FdWrite::At(pos)) => f.write_at(&buf, pos),
                    (false, FdWrite::AtCur) => f.write_at(&buf, pos),
                };

                let nwritten = match f.as_blocking_file() {
                    // If we can block then skip the copy out of wasm memory and
                    // write directly to `f`.
                    Some(f) => do_write(f, &memory.as_cow(buf)?),
                    // ... otherwise copy out of wasm memory and use
                    // `spawn_blocking` to do this write in a thread that can
                    // block.
                    None => {
                        let buf = memory.to_vec(buf)?;
                        f.run_blocking(move |f| do_write(f, &buf)).await
                    }
                };

                let nwritten = nwritten.map_err(|e| StreamError::LastOperationFailed(e.into()))?;

                // If this was a write at the current position then update the
                // current position with the result, otherwise the current
                // position is left unmodified.
                if let FdWrite::AtCur = write {
                    if append {
                        let len = self.as_wasi_impl().stat(fd).await?;
                        position.store(len.size, Ordering::Relaxed);
                    } else {
                        let pos = pos
                            .checked_add(nwritten as u64)
                            .ok_or(types::Errno::Overflow)?;
                        position.store(pos, Ordering::Relaxed);
                    }
                }
                Ok(nwritten.try_into()?)
            }
            Descriptor::Stdout { stream, .. } | Descriptor::Stderr { stream, .. } => {
                match write {
                    // Reject calls to `fd_pwrite` on stdio descriptors...
                    FdWrite::At(_) => return Err(types::Errno::Spipe.into()),
                    // ... but allow calls to `fd_write`
                    FdWrite::AtCur => {}
                }
                let stream = stream.borrowed();
                drop(t);
                let n = BlockingMode::Blocking
                    .write(memory, &mut self.as_io_impl(), stream, buf)
                    .await?
                    .try_into()?;
                Ok(n)
            }
            _ => Err(types::Errno::Badf.into()),
        }
    }
}

#[derive(Copy, Clone)]
enum FdWrite {
    At(u64),
    AtCur,
}

/// Adds asynchronous versions of all WASIp1 functions to the
/// [`wasmtime::Linker`] provided.
///
/// This method will add WASIp1 functions to `linker`. Access to [`WasiP1Ctx`]
/// is provided with `f` by projecting from the store-local state of `T` to
/// [`WasiP1Ctx`]. The closure `f` is invoked every time a WASIp1 function is
/// called to get access to [`WasiP1Ctx`] from `T`. The returned [`WasiP1Ctx`] is
/// used to implement I/O and controls what each function will return.
///
/// It's recommended that [`WasiP1Ctx`] is stored as a field in `T` or that `T =
/// WasiP1Ctx` itself. The closure `f` should be a small projection (e.g. `&mut
/// arg.field`) or something otherwise "small" as it will be executed every time
/// a WASI call is made.
///
/// Note that this function is intended for use with
/// [`Config::async_support(true)`]. If you're looking for a synchronous version
/// see [`add_to_linker_sync`].
///
/// [`Config::async_support(true)`]: wasmtime::Config::async_support
///
/// # Examples
///
/// If the `T` in `Linker<T>` is just `WasiP1Ctx`:
///
/// ```no_run
/// use wasmtime::{Result, Linker, Engine, Config};
/// use wasmtime_wasi::preview1::{self, WasiP1Ctx};
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
///     preview1::add_to_linker_async(&mut linker, |cx| cx)?;
///
///     // ... continue to add more to `linker` as necessary and use it ...
///
///     Ok(())
/// }
/// ```
///
/// If the `T` in `Linker<T>` is custom state:
///
/// ```no_run
/// use wasmtime::{Result, Linker, Engine, Config};
/// use wasmtime_wasi::preview1::{self, WasiP1Ctx};
///
/// struct MyState {
///     // .. other custom state here ..
///
///     wasi: WasiP1Ctx,
/// }
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker: Linker<MyState> = Linker::new(&engine);
///     preview1::add_to_linker_async(&mut linker, |cx| &mut cx.wasi)?;
///
///     // ... continue to add more to `linker` as necessary and use it ...
///
///     Ok(())
/// }
/// ```
pub fn add_to_linker_async<T: Send + 'static>(
    linker: &mut wasmtime::Linker<T>,
    f: impl Fn(&mut T) -> &mut WasiP1Ctx + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::preview1::wasi_snapshot_preview1::add_to_linker(linker, f)
}

/// Adds synchronous versions of all WASIp1 functions to the
/// [`wasmtime::Linker`] provided.
///
/// This method will add WASIp1 functions to `linker`. Access to [`WasiP1Ctx`]
/// is provided with `f` by projecting from the store-local state of `T` to
/// [`WasiP1Ctx`]. The closure `f` is invoked every time a WASIp1 function is
/// called to get access to [`WasiP1Ctx`] from `T`. The returned [`WasiP1Ctx`] is
/// used to implement I/O and controls what each function will return.
///
/// It's recommended that [`WasiP1Ctx`] is stored as a field in `T` or that `T =
/// WasiP1Ctx` itself. The closure `f` should be a small projection (e.g. `&mut
/// arg.field`) or something otherwise "small" as it will be executed every time
/// a WASI call is made.
///
/// Note that this function is intended for use with
/// [`Config::async_support(false)`]. If you're looking for a synchronous version
/// see [`add_to_linker_async`].
///
/// [`Config::async_support(false)`]: wasmtime::Config::async_support
///
/// # Examples
///
/// If the `T` in `Linker<T>` is just `WasiP1Ctx`:
///
/// ```no_run
/// use wasmtime::{Result, Linker, Engine, Config};
/// use wasmtime_wasi::preview1::{self, WasiP1Ctx};
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
///     preview1::add_to_linker_async(&mut linker, |cx| cx)?;
///
///     // ... continue to add more to `linker` as necessary and use it ...
///
///     Ok(())
/// }
/// ```
///
/// If the `T` in `Linker<T>` is custom state:
///
/// ```no_run
/// use wasmtime::{Result, Linker, Engine, Config};
/// use wasmtime_wasi::preview1::{self, WasiP1Ctx};
///
/// struct MyState {
///     // .. other custom state here ..
///
///     wasi: WasiP1Ctx,
/// }
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker: Linker<MyState> = Linker::new(&engine);
///     preview1::add_to_linker_async(&mut linker, |cx| &mut cx.wasi)?;
///
///     // ... continue to add more to `linker` as necessary and use it ...
///
///     Ok(())
/// }
/// ```
pub fn add_to_linker_sync<T: Send + 'static>(
    linker: &mut wasmtime::Linker<T>,
    f: impl Fn(&mut T) -> &mut WasiP1Ctx + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    sync::add_wasi_snapshot_preview1_to_linker(linker, f)
}

// Generate the wasi_snapshot_preview1::WasiSnapshotPreview1 trait,
// and the module types.
// None of the generated modules, traits, or types should be used externally
// to this module.
wiggle::from_witx!({
    witx: ["witx/preview1/wasi_snapshot_preview1.witx"],
    async: {
        wasi_snapshot_preview1::{
            fd_advise, fd_close, fd_datasync, fd_fdstat_get, fd_filestat_get, fd_filestat_set_size,
            fd_filestat_set_times, fd_read, fd_pread, fd_seek, fd_sync, fd_readdir, fd_write,
            fd_pwrite, poll_oneoff, path_create_directory, path_filestat_get,
            path_filestat_set_times, path_link, path_open, path_readlink, path_remove_directory,
            path_rename, path_symlink, path_unlink_file, fd_renumber
        }
    },
    errors: { errno => trappable Error },
});

pub(crate) mod sync {
    use anyhow::Result;
    use std::future::Future;

    wiggle::wasmtime_integration!({
        witx: ["witx/preview1/wasi_snapshot_preview1.witx"],
        target: super,
        block_on[in_tokio]: {
            wasi_snapshot_preview1::{
                fd_advise, fd_close, fd_datasync, fd_fdstat_get, fd_filestat_get, fd_filestat_set_size,
                fd_filestat_set_times, fd_read, fd_pread, fd_seek, fd_sync, fd_readdir, fd_write,
                fd_pwrite, poll_oneoff, path_create_directory, path_filestat_get,
                path_filestat_set_times, path_link, path_open, path_readlink, path_remove_directory,
                path_rename, path_symlink, path_unlink_file, fd_renumber
            }
        },
        errors: { errno => trappable Error },
    });

    // Small wrapper around `in_tokio` to add a `Result` layer which is always
    // `Ok`
    fn in_tokio<F: Future>(future: F) -> Result<F::Output> {
        Ok(crate::runtime::in_tokio(future))
    }
}

impl wiggle::GuestErrorType for types::Errno {
    fn success() -> Self {
        Self::Success
    }
}

impl From<StreamError> for types::Error {
    fn from(err: StreamError) -> Self {
        match err {
            StreamError::Closed => types::Errno::Io.into(),
            StreamError::LastOperationFailed(e) => match e.downcast::<std::io::Error>() {
                Ok(err) => filesystem::ErrorCode::from(err).into(),
                Err(e) => {
                    tracing::debug!("dropping error {e:?}");
                    types::Errno::Io.into()
                }
            },
            StreamError::Trap(e) => types::Error::trap(e),
        }
    }
}

impl From<FsError> for types::Error {
    fn from(err: FsError) -> Self {
        match err.downcast() {
            Ok(code) => code.into(),
            Err(e) => types::Error::trap(e),
        }
    }
}

fn systimespec(set: bool, ts: types::Timestamp, now: bool) -> Result<filesystem::NewTimestamp> {
    if set && now {
        Err(types::Errno::Inval.into())
    } else if set {
        Ok(filesystem::NewTimestamp::Timestamp(filesystem::Datetime {
            seconds: ts / 1_000_000_000,
            nanoseconds: (ts % 1_000_000_000) as _,
        }))
    } else if now {
        Ok(filesystem::NewTimestamp::Now)
    } else {
        Ok(filesystem::NewTimestamp::NoChange)
    }
}

impl TryFrom<wall_clock::Datetime> for types::Timestamp {
    type Error = types::Errno;

    fn try_from(
        wall_clock::Datetime {
            seconds,
            nanoseconds,
        }: wall_clock::Datetime,
    ) -> Result<Self, Self::Error> {
        types::Timestamp::from(seconds)
            .checked_mul(1_000_000_000)
            .and_then(|ns| ns.checked_add(nanoseconds.into()))
            .ok_or(types::Errno::Overflow)
    }
}

impl From<types::Lookupflags> for filesystem::PathFlags {
    fn from(flags: types::Lookupflags) -> Self {
        if flags.contains(types::Lookupflags::SYMLINK_FOLLOW) {
            filesystem::PathFlags::SYMLINK_FOLLOW
        } else {
            filesystem::PathFlags::empty()
        }
    }
}

impl From<types::Oflags> for filesystem::OpenFlags {
    fn from(flags: types::Oflags) -> Self {
        let mut out = filesystem::OpenFlags::empty();
        if flags.contains(types::Oflags::CREAT) {
            out |= filesystem::OpenFlags::CREATE;
        }
        if flags.contains(types::Oflags::DIRECTORY) {
            out |= filesystem::OpenFlags::DIRECTORY;
        }
        if flags.contains(types::Oflags::EXCL) {
            out |= filesystem::OpenFlags::EXCLUSIVE;
        }
        if flags.contains(types::Oflags::TRUNC) {
            out |= filesystem::OpenFlags::TRUNCATE;
        }
        out
    }
}

impl From<types::Advice> for filesystem::Advice {
    fn from(advice: types::Advice) -> Self {
        match advice {
            types::Advice::Normal => filesystem::Advice::Normal,
            types::Advice::Sequential => filesystem::Advice::Sequential,
            types::Advice::Random => filesystem::Advice::Random,
            types::Advice::Willneed => filesystem::Advice::WillNeed,
            types::Advice::Dontneed => filesystem::Advice::DontNeed,
            types::Advice::Noreuse => filesystem::Advice::NoReuse,
        }
    }
}

impl TryFrom<filesystem::DescriptorType> for types::Filetype {
    type Error = anyhow::Error;

    fn try_from(ty: filesystem::DescriptorType) -> Result<Self, Self::Error> {
        match ty {
            filesystem::DescriptorType::RegularFile => Ok(types::Filetype::RegularFile),
            filesystem::DescriptorType::Directory => Ok(types::Filetype::Directory),
            filesystem::DescriptorType::BlockDevice => Ok(types::Filetype::BlockDevice),
            filesystem::DescriptorType::CharacterDevice => Ok(types::Filetype::CharacterDevice),
            // preview1 never had a FIFO code.
            filesystem::DescriptorType::Fifo => Ok(types::Filetype::Unknown),
            // TODO: Add a way to disginguish between FILETYPE_SOCKET_STREAM and
            // FILETYPE_SOCKET_DGRAM.
            filesystem::DescriptorType::Socket => {
                bail!("sockets are not currently supported")
            }
            filesystem::DescriptorType::SymbolicLink => Ok(types::Filetype::SymbolicLink),
            filesystem::DescriptorType::Unknown => Ok(types::Filetype::Unknown),
        }
    }
}

impl From<IsATTY> for types::Filetype {
    fn from(isatty: IsATTY) -> Self {
        match isatty {
            IsATTY::Yes => types::Filetype::CharacterDevice,
            IsATTY::No => types::Filetype::Unknown,
        }
    }
}

impl From<filesystem::ErrorCode> for types::Errno {
    fn from(code: filesystem::ErrorCode) -> Self {
        match code {
            filesystem::ErrorCode::Access => types::Errno::Acces,
            filesystem::ErrorCode::WouldBlock => types::Errno::Again,
            filesystem::ErrorCode::Already => types::Errno::Already,
            filesystem::ErrorCode::BadDescriptor => types::Errno::Badf,
            filesystem::ErrorCode::Busy => types::Errno::Busy,
            filesystem::ErrorCode::Deadlock => types::Errno::Deadlk,
            filesystem::ErrorCode::Quota => types::Errno::Dquot,
            filesystem::ErrorCode::Exist => types::Errno::Exist,
            filesystem::ErrorCode::FileTooLarge => types::Errno::Fbig,
            filesystem::ErrorCode::IllegalByteSequence => types::Errno::Ilseq,
            filesystem::ErrorCode::InProgress => types::Errno::Inprogress,
            filesystem::ErrorCode::Interrupted => types::Errno::Intr,
            filesystem::ErrorCode::Invalid => types::Errno::Inval,
            filesystem::ErrorCode::Io => types::Errno::Io,
            filesystem::ErrorCode::IsDirectory => types::Errno::Isdir,
            filesystem::ErrorCode::Loop => types::Errno::Loop,
            filesystem::ErrorCode::TooManyLinks => types::Errno::Mlink,
            filesystem::ErrorCode::MessageSize => types::Errno::Msgsize,
            filesystem::ErrorCode::NameTooLong => types::Errno::Nametoolong,
            filesystem::ErrorCode::NoDevice => types::Errno::Nodev,
            filesystem::ErrorCode::NoEntry => types::Errno::Noent,
            filesystem::ErrorCode::NoLock => types::Errno::Nolck,
            filesystem::ErrorCode::InsufficientMemory => types::Errno::Nomem,
            filesystem::ErrorCode::InsufficientSpace => types::Errno::Nospc,
            filesystem::ErrorCode::Unsupported => types::Errno::Notsup,
            filesystem::ErrorCode::NotDirectory => types::Errno::Notdir,
            filesystem::ErrorCode::NotEmpty => types::Errno::Notempty,
            filesystem::ErrorCode::NotRecoverable => types::Errno::Notrecoverable,
            filesystem::ErrorCode::NoTty => types::Errno::Notty,
            filesystem::ErrorCode::NoSuchDevice => types::Errno::Nxio,
            filesystem::ErrorCode::Overflow => types::Errno::Overflow,
            filesystem::ErrorCode::NotPermitted => types::Errno::Perm,
            filesystem::ErrorCode::Pipe => types::Errno::Pipe,
            filesystem::ErrorCode::ReadOnly => types::Errno::Rofs,
            filesystem::ErrorCode::InvalidSeek => types::Errno::Spipe,
            filesystem::ErrorCode::TextFileBusy => types::Errno::Txtbsy,
            filesystem::ErrorCode::CrossDevice => types::Errno::Xdev,
        }
    }
}

impl From<std::num::TryFromIntError> for types::Error {
    fn from(_: std::num::TryFromIntError) -> Self {
        types::Errno::Overflow.into()
    }
}

impl From<GuestError> for types::Error {
    fn from(err: GuestError) -> Self {
        use wiggle::GuestError::*;
        match err {
            InvalidFlagValue { .. } => types::Errno::Inval.into(),
            InvalidEnumValue { .. } => types::Errno::Inval.into(),
            // As per
            // https://github.com/WebAssembly/wasi/blob/main/legacy/tools/witx-docs.md#pointers
            //
            // > If a misaligned pointer is passed to a function, the function
            // > shall trap.
            // >
            // > If an out-of-bounds pointer is passed to a function and the
            // > function needs to dereference it, the function shall trap.
            //
            // so this turns OOB and misalignment errors into traps.
            PtrOverflow { .. } | PtrOutOfBounds { .. } | PtrNotAligned { .. } => {
                types::Error::trap(err.into())
            }
            InvalidUtf8 { .. } => types::Errno::Ilseq.into(),
            TryFromIntError { .. } => types::Errno::Overflow.into(),
            SliceLengthsDiffer { .. } => types::Errno::Fault.into(),
            InFunc { err, .. } => types::Error::from(*err),
        }
    }
}

impl From<filesystem::ErrorCode> for types::Error {
    fn from(code: filesystem::ErrorCode) -> Self {
        types::Errno::from(code).into()
    }
}

impl From<wasmtime::component::ResourceTableError> for types::Error {
    fn from(err: wasmtime::component::ResourceTableError) -> Self {
        types::Error::trap(err.into())
    }
}

type Result<T, E = types::Error> = std::result::Result<T, E>;

fn write_bytes(
    memory: &mut GuestMemory<'_>,
    ptr: GuestPtr<u8>,
    buf: &[u8],
) -> Result<GuestPtr<u8>, types::Error> {
    // NOTE: legacy implementation always returns Inval errno

    let len = u32::try_from(buf.len())?;

    memory.copy_from_slice(buf, ptr.as_array(len))?;
    let next = ptr.add(len)?;
    Ok(next)
}

fn write_byte(memory: &mut GuestMemory<'_>, ptr: GuestPtr<u8>, byte: u8) -> Result<GuestPtr<u8>> {
    memory.write(ptr, byte)?;
    let next = ptr.add(1)?;
    Ok(next)
}

#[async_trait::async_trait]
// Implement the WasiSnapshotPreview1 trait using only the traits that are
// required for T, i.e., in terms of the preview 2 wit interface, and state
// stored in the WasiPreview1Adapter struct.
impl wasi_snapshot_preview1::WasiSnapshotPreview1 for WasiP1Ctx {
    fn set_hostcall_fuel(&mut self, fuel: usize) {
        self.hostcall_fuel = fuel;
    }

    #[instrument(skip(self, memory))]
    fn args_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        argv: GuestPtr<GuestPtr<u8>>,
        argv_buf: GuestPtr<u8>,
    ) -> Result<(), types::Error> {
        self.as_wasi_impl()
            .get_arguments()
            .context("failed to call `get-arguments`")
            .map_err(types::Error::trap)?
            .into_iter()
            .try_fold((argv, argv_buf), |(argv, argv_buf), arg| -> Result<_> {
                memory.write(argv, argv_buf)?;
                let argv = argv.add(1)?;

                let argv_buf = write_bytes(memory, argv_buf, arg.as_bytes())?;
                let argv_buf = write_byte(memory, argv_buf, 0)?;

                Ok((argv, argv_buf))
            })?;
        Ok(())
    }

    #[instrument(skip(self, _memory))]
    fn args_sizes_get(
        &mut self,
        _memory: &mut GuestMemory<'_>,
    ) -> Result<(types::Size, types::Size), types::Error> {
        let args = self
            .as_wasi_impl()
            .get_arguments()
            .context("failed to call `get-arguments`")
            .map_err(types::Error::trap)?;
        let num = args.len().try_into().map_err(|_| types::Errno::Overflow)?;
        let len = args
            .iter()
            .map(|buf| buf.len() + 1) // Each argument is expected to be `\0` terminated.
            .sum::<usize>()
            .try_into()
            .map_err(|_| types::Errno::Overflow)?;
        Ok((num, len))
    }

    #[instrument(skip(self, memory))]
    fn environ_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        environ: GuestPtr<GuestPtr<u8>>,
        environ_buf: GuestPtr<u8>,
    ) -> Result<(), types::Error> {
        self.as_wasi_impl()
            .get_environment()
            .context("failed to call `get-environment`")
            .map_err(types::Error::trap)?
            .into_iter()
            .try_fold(
                (environ, environ_buf),
                |(environ, environ_buf), (k, v)| -> Result<_, types::Error> {
                    memory.write(environ, environ_buf)?;
                    let environ = environ.add(1)?;

                    let environ_buf = write_bytes(memory, environ_buf, k.as_bytes())?;
                    let environ_buf = write_byte(memory, environ_buf, b'=')?;
                    let environ_buf = write_bytes(memory, environ_buf, v.as_bytes())?;
                    let environ_buf = write_byte(memory, environ_buf, 0)?;

                    Ok((environ, environ_buf))
                },
            )?;
        Ok(())
    }

    #[instrument(skip(self, _memory))]
    fn environ_sizes_get(
        &mut self,
        _memory: &mut GuestMemory<'_>,
    ) -> Result<(types::Size, types::Size), types::Error> {
        let environ = self
            .as_wasi_impl()
            .get_environment()
            .context("failed to call `get-environment`")
            .map_err(types::Error::trap)?;
        let num = environ.len().try_into()?;
        let len = environ
            .iter()
            .map(|(k, v)| k.len() + 1 + v.len() + 1) // Key/value pairs are expected to be joined with `=`s, and terminated with `\0`s.
            .sum::<usize>()
            .try_into()?;
        Ok((num, len))
    }

    #[instrument(skip(self, _memory))]
    fn clock_res_get(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        id: types::Clockid,
    ) -> Result<types::Timestamp, types::Error> {
        let res = match id {
            types::Clockid::Realtime => wall_clock::Host::resolution(&mut self.as_wasi_impl())
                .context("failed to call `wall_clock::resolution`")
                .map_err(types::Error::trap)?
                .try_into()?,
            types::Clockid::Monotonic => {
                monotonic_clock::Host::resolution(&mut self.as_wasi_impl())
                    .context("failed to call `monotonic_clock::resolution`")
                    .map_err(types::Error::trap)?
            }
            types::Clockid::ProcessCputimeId | types::Clockid::ThreadCputimeId => {
                return Err(types::Errno::Badf.into());
            }
        };
        Ok(res)
    }

    #[instrument(skip(self, _memory))]
    fn clock_time_get(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        id: types::Clockid,
        _precision: types::Timestamp,
    ) -> Result<types::Timestamp, types::Error> {
        let now = match id {
            types::Clockid::Realtime => wall_clock::Host::now(&mut self.as_wasi_impl())
                .context("failed to call `wall_clock::now`")
                .map_err(types::Error::trap)?
                .try_into()?,
            types::Clockid::Monotonic => monotonic_clock::Host::now(&mut self.as_wasi_impl())
                .context("failed to call `monotonic_clock::now`")
                .map_err(types::Error::trap)?,
            types::Clockid::ProcessCputimeId | types::Clockid::ThreadCputimeId => {
                return Err(types::Errno::Badf.into());
            }
        };
        Ok(now)
    }

    #[instrument(skip(self, _memory))]
    async fn fd_advise(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
        advice: types::Advice,
    ) -> Result<(), types::Error> {
        let fd = self.get_file_fd(fd)?;
        self.as_wasi_impl()
            .advise(fd, offset, len, advice.into())
            .await?;
        Ok(())
    }

    /// Force the allocation of space in a file.
    /// NOTE: This is similar to `posix_fallocate` in POSIX.
    #[instrument(skip(self, _memory))]
    fn fd_allocate(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        _offset: types::Filesize,
        _len: types::Filesize,
    ) -> Result<(), types::Error> {
        self.get_file_fd(fd)?;
        Err(types::Errno::Notsup.into())
    }

    /// Close a file descriptor.
    /// NOTE: This is similar to `close` in POSIX.
    #[instrument(skip(self, _memory))]
    async fn fd_close(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<(), types::Error> {
        let desc = {
            let fd = fd.into();
            let mut st = self.transact()?;
            let desc = st.descriptors.used.remove(&fd).ok_or(types::Errno::Badf)?;
            st.descriptors.free.insert(fd);
            desc
        };
        match desc {
            Descriptor::Stdin { stream, .. } => {
                streams::HostInputStream::drop(&mut self.as_io_impl(), stream)
                    .await
                    .context("failed to call `drop` on `input-stream`")
            }
            Descriptor::Stdout { stream, .. } | Descriptor::Stderr { stream, .. } => {
                streams::HostOutputStream::drop(&mut self.as_io_impl(), stream)
                    .await
                    .context("failed to call `drop` on `output-stream`")
            }
            Descriptor::File(File { fd, .. }) | Descriptor::Directory { fd, .. } => {
                filesystem::HostDescriptor::drop(&mut self.as_wasi_impl(), fd)
                    .context("failed to call `drop`")
            }
        }
        .map_err(types::Error::trap)
    }

    /// Synchronize the data of a file to disk.
    /// NOTE: This is similar to `fdatasync` in POSIX.
    #[instrument(skip(self, _memory))]
    async fn fd_datasync(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<(), types::Error> {
        let fd = self.get_file_fd(fd)?;
        self.as_wasi_impl().sync_data(fd).await?;
        Ok(())
    }

    /// Get the attributes of a file descriptor.
    /// NOTE: This returns similar flags to `fsync(fd, F_GETFL)` in POSIX, as well as additional fields.
    #[instrument(skip(self, _memory))]
    async fn fd_fdstat_get(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Fdstat, types::Error> {
        let (fd, blocking, append) = match self.transact()?.get_descriptor(fd)? {
            Descriptor::Stdin { isatty, .. } => {
                let fs_rights_base = types::Rights::FD_READ;
                return Ok(types::Fdstat {
                    fs_filetype: (*isatty).into(),
                    fs_flags: types::Fdflags::empty(),
                    fs_rights_base,
                    fs_rights_inheriting: fs_rights_base,
                });
            }
            Descriptor::Stdout { isatty, .. } | Descriptor::Stderr { isatty, .. } => {
                let fs_rights_base = types::Rights::FD_WRITE;
                return Ok(types::Fdstat {
                    fs_filetype: (*isatty).into(),
                    fs_flags: types::Fdflags::empty(),
                    fs_rights_base,
                    fs_rights_inheriting: fs_rights_base,
                });
            }
            Descriptor::Directory {
                preopen_path: Some(_),
                ..
            } => {
                // Hard-coded set or rights expected by many userlands:
                let fs_rights_base = types::Rights::PATH_CREATE_DIRECTORY
                    | types::Rights::PATH_CREATE_FILE
                    | types::Rights::PATH_LINK_SOURCE
                    | types::Rights::PATH_LINK_TARGET
                    | types::Rights::PATH_OPEN
                    | types::Rights::FD_READDIR
                    | types::Rights::PATH_READLINK
                    | types::Rights::PATH_RENAME_SOURCE
                    | types::Rights::PATH_RENAME_TARGET
                    | types::Rights::PATH_SYMLINK
                    | types::Rights::PATH_REMOVE_DIRECTORY
                    | types::Rights::PATH_UNLINK_FILE
                    | types::Rights::PATH_FILESTAT_GET
                    | types::Rights::PATH_FILESTAT_SET_TIMES
                    | types::Rights::FD_FILESTAT_GET
                    | types::Rights::FD_FILESTAT_SET_TIMES;

                let fs_rights_inheriting = fs_rights_base
                    | types::Rights::FD_DATASYNC
                    | types::Rights::FD_READ
                    | types::Rights::FD_SEEK
                    | types::Rights::FD_FDSTAT_SET_FLAGS
                    | types::Rights::FD_SYNC
                    | types::Rights::FD_TELL
                    | types::Rights::FD_WRITE
                    | types::Rights::FD_ADVISE
                    | types::Rights::FD_ALLOCATE
                    | types::Rights::FD_FILESTAT_GET
                    | types::Rights::FD_FILESTAT_SET_SIZE
                    | types::Rights::FD_FILESTAT_SET_TIMES
                    | types::Rights::POLL_FD_READWRITE;

                return Ok(types::Fdstat {
                    fs_filetype: types::Filetype::Directory,
                    fs_flags: types::Fdflags::empty(),
                    fs_rights_base,
                    fs_rights_inheriting,
                });
            }
            Descriptor::Directory { fd, .. } => (fd.borrowed(), BlockingMode::Blocking, false),
            Descriptor::File(File {
                fd,
                blocking_mode,
                append,
                ..
            }) => (fd.borrowed(), *blocking_mode, *append),
        };
        let flags = self.as_wasi_impl().get_flags(fd.borrowed()).await?;
        let fs_filetype = self
            .as_wasi_impl()
            .get_type(fd.borrowed())
            .await?
            .try_into()
            .map_err(types::Error::trap)?;
        let mut fs_flags = types::Fdflags::empty();
        let mut fs_rights_base = types::Rights::all();
        if let types::Filetype::Directory = fs_filetype {
            fs_rights_base &= !types::Rights::FD_SEEK;
            fs_rights_base &= !types::Rights::FD_FILESTAT_SET_SIZE;
        }
        if !flags.contains(filesystem::DescriptorFlags::READ) {
            fs_rights_base &= !types::Rights::FD_READ;
            fs_rights_base &= !types::Rights::FD_READDIR;
        }
        if !flags.contains(filesystem::DescriptorFlags::WRITE) {
            fs_rights_base &= !types::Rights::FD_WRITE;
        }
        if flags.contains(filesystem::DescriptorFlags::DATA_INTEGRITY_SYNC) {
            fs_flags |= types::Fdflags::DSYNC;
        }
        if flags.contains(filesystem::DescriptorFlags::REQUESTED_WRITE_SYNC) {
            fs_flags |= types::Fdflags::RSYNC;
        }
        if flags.contains(filesystem::DescriptorFlags::FILE_INTEGRITY_SYNC) {
            fs_flags |= types::Fdflags::SYNC;
        }
        if append {
            fs_flags |= types::Fdflags::APPEND;
        }
        if matches!(blocking, BlockingMode::NonBlocking) {
            fs_flags |= types::Fdflags::NONBLOCK;
        }
        Ok(types::Fdstat {
            fs_filetype,
            fs_flags,
            fs_rights_base,
            fs_rights_inheriting: fs_rights_base,
        })
    }

    /// Adjust the flags associated with a file descriptor.
    /// NOTE: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
    #[instrument(skip(self, _memory))]
    fn fd_fdstat_set_flags(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        flags: types::Fdflags,
    ) -> Result<(), types::Error> {
        let mut st = self.transact()?;
        let File {
            append,
            blocking_mode,
            ..
        } = st.get_file_mut(fd)?;

        // Only support changing the NONBLOCK or APPEND flags.
        if flags.contains(types::Fdflags::DSYNC)
            || flags.contains(types::Fdflags::SYNC)
            || flags.contains(types::Fdflags::RSYNC)
        {
            return Err(types::Errno::Inval.into());
        }
        *append = flags.contains(types::Fdflags::APPEND);
        *blocking_mode = BlockingMode::from_fdflags(&flags);
        Ok(())
    }

    /// Does not do anything if `fd` corresponds to a valid descriptor and returns `[types::Errno::Badf]` error otherwise.
    #[instrument(skip(self, _memory))]
    fn fd_fdstat_set_rights(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        _fs_rights_base: types::Rights,
        _fs_rights_inheriting: types::Rights,
    ) -> Result<(), types::Error> {
        self.get_fd(fd)?;
        Ok(())
    }

    /// Return the attributes of an open file.
    #[instrument(skip(self, _memory))]
    async fn fd_filestat_get(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Filestat, types::Error> {
        let t = self.transact()?;
        let desc = t.get_descriptor(fd)?;
        match desc {
            Descriptor::Stdin { isatty, .. }
            | Descriptor::Stdout { isatty, .. }
            | Descriptor::Stderr { isatty, .. } => Ok(types::Filestat {
                dev: 0,
                ino: 0,
                filetype: (*isatty).into(),
                nlink: 0,
                size: 0,
                atim: 0,
                mtim: 0,
                ctim: 0,
            }),
            Descriptor::Directory { fd, .. } | Descriptor::File(File { fd, .. }) => {
                let fd = fd.borrowed();
                drop(t);
                let filesystem::DescriptorStat {
                    type_,
                    link_count: nlink,
                    size,
                    data_access_timestamp,
                    data_modification_timestamp,
                    status_change_timestamp,
                } = self.as_wasi_impl().stat(fd.borrowed()).await?;
                let metadata_hash = self.as_wasi_impl().metadata_hash(fd).await?;
                let filetype = type_.try_into().map_err(types::Error::trap)?;
                let zero = wall_clock::Datetime {
                    seconds: 0,
                    nanoseconds: 0,
                };
                let atim = data_access_timestamp.unwrap_or(zero).try_into()?;
                let mtim = data_modification_timestamp.unwrap_or(zero).try_into()?;
                let ctim = status_change_timestamp.unwrap_or(zero).try_into()?;
                Ok(types::Filestat {
                    dev: 1,
                    ino: metadata_hash.lower,
                    filetype,
                    nlink,
                    size,
                    atim,
                    mtim,
                    ctim,
                })
            }
        }
    }

    /// Adjust the size of an open file. If this increases the file's size, the extra bytes are filled with zeros.
    /// NOTE: This is similar to `ftruncate` in POSIX.
    #[instrument(skip(self, _memory))]
    async fn fd_filestat_set_size(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        size: types::Filesize,
    ) -> Result<(), types::Error> {
        let fd = self.get_file_fd(fd)?;
        self.as_wasi_impl().set_size(fd, size).await?;
        Ok(())
    }

    /// Adjust the timestamps of an open file or directory.
    /// NOTE: This is similar to `futimens` in POSIX.
    #[instrument(skip(self, _memory))]
    async fn fd_filestat_set_times(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), types::Error> {
        let atim = systimespec(
            fst_flags.contains(types::Fstflags::ATIM),
            atim,
            fst_flags.contains(types::Fstflags::ATIM_NOW),
        )?;
        let mtim = systimespec(
            fst_flags.contains(types::Fstflags::MTIM),
            mtim,
            fst_flags.contains(types::Fstflags::MTIM_NOW),
        )?;

        let fd = self.get_fd(fd)?;
        self.as_wasi_impl().set_times(fd, atim, mtim).await?;
        Ok(())
    }

    /// Read from a file descriptor.
    /// NOTE: This is similar to `readv` in POSIX.
    #[instrument(skip(self, memory))]
    async fn fd_read(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        iovs: types::IovecArray,
    ) -> Result<types::Size, types::Error> {
        let iov = self.first_non_empty_iovec(memory, iovs)?;
        let t = self.transact()?;
        let desc = t.get_descriptor(fd)?;
        match desc {
            Descriptor::File(File {
                fd,
                position,
                // NB: the nonblocking flag is intentionally ignored here and
                // blocking reads/writes are always performed.
                blocking_mode: _,
                ..
            }) => {
                let fd = fd.borrowed();
                let position = position.clone();
                drop(t);
                let pos = position.load(Ordering::Relaxed);
                let file = self.table.get(&fd)?.file()?;
                let bytes_read = match (file.as_blocking_file(), memory.as_slice_mut(iov)?) {
                    // Try to read directly into wasm memory where possible
                    // when the current thread can block and additionally wasm
                    // memory isn't shared.
                    (Some(file), Some(mut buf)) => file
                        .read_at(&mut buf, pos)
                        .map_err(|e| StreamError::LastOperationFailed(e.into()))?,
                    // ... otherwise fall back to performing the read on a
                    // blocking thread and which copies the data back into wasm
                    // memory.
                    (_, buf) => {
                        drop(buf);
                        let mut buf = vec![0; iov.len() as usize];
                        let buf = file
                            .run_blocking(move |file| -> Result<_, types::Error> {
                                let bytes_read = file
                                    .read_at(&mut buf, pos)
                                    .map_err(|e| StreamError::LastOperationFailed(e.into()))?;
                                buf.truncate(bytes_read);
                                Ok(buf)
                            })
                            .await?;
                        let iov = iov.get_range(0..u32::try_from(buf.len())?).unwrap();
                        memory.copy_from_slice(&buf, iov)?;
                        buf.len()
                    }
                };

                let pos = pos
                    .checked_add(bytes_read.try_into()?)
                    .ok_or(types::Errno::Overflow)?;
                position.store(pos, Ordering::Relaxed);

                Ok(bytes_read.try_into()?)
            }
            Descriptor::Stdin { stream, .. } => {
                let stream = stream.borrowed();
                drop(t);
                let read = BlockingMode::Blocking
                    .read(&mut self.as_io_impl(), stream, iov.len().try_into()?)
                    .await?;
                if read.len() > iov.len().try_into()? {
                    return Err(types::Errno::Range.into());
                }
                let iov = iov.get_range(0..u32::try_from(read.len())?).unwrap();
                memory.copy_from_slice(&read, iov)?;
                let n = read.len().try_into()?;
                Ok(n)
            }
            _ => return Err(types::Errno::Badf.into()),
        }
    }

    /// Read from a file descriptor, without using and updating the file descriptor's offset.
    /// NOTE: This is similar to `preadv` in POSIX.
    #[instrument(skip(self, memory))]
    async fn fd_pread(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        iovs: types::IovecArray,
        offset: types::Filesize,
    ) -> Result<types::Size, types::Error> {
        let buf = self.first_non_empty_iovec(memory, iovs)?;
        let t = self.transact()?;
        let desc = t.get_descriptor(fd)?;
        let (buf, read) = match desc {
            Descriptor::File(File {
                fd, blocking_mode, ..
            }) => {
                let fd = fd.borrowed();
                let blocking_mode = *blocking_mode;
                drop(t);

                let stream = self.as_wasi_impl().read_via_stream(fd, offset)?;
                let read = blocking_mode
                    .read(
                        &mut self.as_io_impl(),
                        stream.borrowed(),
                        buf.len().try_into()?,
                    )
                    .await;
                streams::HostInputStream::drop(&mut self.as_io_impl(), stream)
                    .await
                    .map_err(|e| types::Error::trap(e))?;
                (buf, read?)
            }
            Descriptor::Stdin { .. } => {
                // NOTE: legacy implementation returns SPIPE here
                return Err(types::Errno::Spipe.into());
            }
            _ => return Err(types::Errno::Badf.into()),
        };
        if read.len() > buf.len().try_into()? {
            return Err(types::Errno::Range.into());
        }
        let buf = buf.get_range(0..u32::try_from(read.len())?).unwrap();
        memory.copy_from_slice(&read, buf)?;
        let n = read.len().try_into()?;
        Ok(n)
    }

    /// Write to a file descriptor.
    /// NOTE: This is similar to `writev` in POSIX.
    #[instrument(skip(self, memory))]
    async fn fd_write(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        ciovs: types::CiovecArray,
    ) -> Result<types::Size, types::Error> {
        self.fd_write_impl(memory, fd, ciovs, FdWrite::AtCur).await
    }

    /// Write to a file descriptor, without using and updating the file descriptor's offset.
    /// NOTE: This is similar to `pwritev` in POSIX.
    #[instrument(skip(self, memory))]
    async fn fd_pwrite(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        ciovs: types::CiovecArray,
        offset: types::Filesize,
    ) -> Result<types::Size, types::Error> {
        self.fd_write_impl(memory, fd, ciovs, FdWrite::At(offset))
            .await
    }

    /// Return a description of the given preopened file descriptor.
    #[instrument(skip(self, _memory))]
    fn fd_prestat_get(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Prestat, types::Error> {
        if let Descriptor::Directory {
            preopen_path: Some(p),
            ..
        } = self.transact()?.get_descriptor(fd)?
        {
            let pr_name_len = p.len().try_into()?;
            return Ok(types::Prestat::Dir(types::PrestatDir { pr_name_len }));
        }
        Err(types::Errno::Badf.into()) // NOTE: legacy implementation returns BADF here
    }

    /// Return a description of the given preopened file descriptor.
    #[instrument(skip(self, memory))]
    fn fd_prestat_dir_name(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        path: GuestPtr<u8>,
        path_max_len: types::Size,
    ) -> Result<(), types::Error> {
        let path_max_len = path_max_len.try_into()?;
        if let Descriptor::Directory {
            preopen_path: Some(p),
            ..
        } = self.transact()?.get_descriptor(fd)?
        {
            if p.len() > path_max_len {
                return Err(types::Errno::Nametoolong.into());
            }
            write_bytes(memory, path, p.as_bytes())?;
            return Ok(());
        }
        Err(types::Errno::Notdir.into()) // NOTE: legacy implementation returns NOTDIR here
    }

    /// Atomically replace a file descriptor by renumbering another file descriptor.
    #[instrument(skip(self, memory))]
    async fn fd_renumber(
        &mut self,
        memory: &mut GuestMemory<'_>,
        from_fd: types::Fd,
        to_fd: types::Fd,
    ) -> Result<(), types::Error> {
        let from = from_fd.into();
        let to = to_fd.into();
        {
            let st = self.transact()?;
            if !st.descriptors.used.contains_key(&to) || !st.descriptors.used.contains_key(&from) {
                return Err(types::Errno::Badf.into());
            }
            if from == to {
                return Ok(());
            }
        }
        self.fd_close(memory, to_fd).await?;
        let mut st = self.transact()?;
        let btree_map::Entry::Occupied(desc) = st.descriptors.used.entry(from) else {
            return Err(types::Errno::Badf.into());
        };
        let desc = desc.remove();
        st.descriptors.free.insert(from);
        st.descriptors.free.remove(&to);
        st.descriptors.used.insert(to, desc);
        Ok(())
    }

    /// Move the offset of a file descriptor.
    /// NOTE: This is similar to `lseek` in POSIX.
    #[instrument(skip(self, _memory))]
    async fn fd_seek(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        offset: types::Filedelta,
        whence: types::Whence,
    ) -> Result<types::Filesize, types::Error> {
        let t = self.transact()?;
        let File { fd, position, .. } = t.get_seekable(fd)?;
        let fd = fd.borrowed();
        let position = position.clone();
        drop(t);
        let pos = match whence {
            types::Whence::Set if offset >= 0 => {
                offset.try_into().map_err(|_| types::Errno::Inval)?
            }
            types::Whence::Cur => position
                .load(Ordering::Relaxed)
                .checked_add_signed(offset)
                .ok_or(types::Errno::Inval)?,
            types::Whence::End => {
                let filesystem::DescriptorStat { size, .. } = self.as_wasi_impl().stat(fd).await?;
                size.checked_add_signed(offset).ok_or(types::Errno::Inval)?
            }
            _ => return Err(types::Errno::Inval.into()),
        };
        position.store(pos, Ordering::Relaxed);
        Ok(pos)
    }

    /// Synchronize the data and metadata of a file to disk.
    /// NOTE: This is similar to `fsync` in POSIX.
    #[instrument(skip(self, _memory))]
    async fn fd_sync(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<(), types::Error> {
        let fd = self.get_file_fd(fd)?;
        self.as_wasi_impl().sync(fd).await?;
        Ok(())
    }

    /// Return the current offset of a file descriptor.
    /// NOTE: This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX.
    #[instrument(skip(self, _memory))]
    fn fd_tell(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Filesize, types::Error> {
        let pos = self
            .transact()?
            .get_seekable(fd)
            .map(|File { position, .. }| position.load(Ordering::Relaxed))?;
        Ok(pos)
    }

    #[instrument(skip(self, memory))]
    async fn fd_readdir(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        buf: GuestPtr<u8>,
        buf_len: types::Size,
        cookie: types::Dircookie,
    ) -> Result<types::Size, types::Error> {
        let fd = self.get_dir_fd(fd)?;
        let stream = self.as_wasi_impl().read_directory(fd.borrowed()).await?;
        let dir_metadata_hash = self.as_wasi_impl().metadata_hash(fd.borrowed()).await?;
        let cookie = cookie.try_into().map_err(|_| types::Errno::Overflow)?;

        let head = [
            (
                types::Dirent {
                    d_next: 1u64.to_le(),
                    d_ino: dir_metadata_hash.lower.to_le(),
                    d_type: types::Filetype::Directory,
                    d_namlen: 1u32.to_le(),
                },
                ".".into(),
            ),
            (
                types::Dirent {
                    d_next: 2u64.to_le(),
                    d_ino: dir_metadata_hash.lower.to_le(), // NOTE: incorrect, but legacy implementation returns `fd` inode here
                    d_type: types::Filetype::Directory,
                    d_namlen: 2u32.to_le(),
                },
                "..".into(),
            ),
        ];

        let mut dir = Vec::new();
        for (entry, d_next) in self
            .table
            // remove iterator from table and use it directly:
            .delete(stream)?
            .into_iter()
            .zip(3u64..)
        {
            let filesystem::DirectoryEntry { type_, name } = entry?;
            let metadata_hash = self
                .as_wasi_impl()
                .metadata_hash_at(fd.borrowed(), filesystem::PathFlags::empty(), name.clone())
                .await?;
            let d_type = type_.try_into().map_err(types::Error::trap)?;
            let d_namlen: u32 = name.len().try_into().map_err(|_| types::Errno::Overflow)?;
            dir.push((
                types::Dirent {
                    d_next: d_next.to_le(),
                    d_ino: metadata_hash.lower.to_le(),
                    d_type, // endian-invariant
                    d_namlen: d_namlen.to_le(),
                },
                name,
            ))
        }

        // assume that `types::Dirent` size always fits in `u32`
        const DIRENT_SIZE: u32 = size_of::<types::Dirent>() as _;
        assert_eq!(
            types::Dirent::guest_size(),
            DIRENT_SIZE,
            "Dirent guest repr and host repr should match"
        );
        let mut buf = buf;
        let mut cap = buf_len;
        for (ref entry, path) in head.into_iter().chain(dir.into_iter()).skip(cookie) {
            let mut path = path.into_bytes();
            assert_eq!(
                1,
                size_of_val(&entry.d_type),
                "Dirent member d_type should be endian-invariant"
            );
            let entry_len = cap.min(DIRENT_SIZE);
            let entry = entry as *const _ as _;
            let entry = unsafe { slice::from_raw_parts(entry, entry_len as _) };
            cap = cap.checked_sub(entry_len).unwrap();
            buf = write_bytes(memory, buf, entry)?;
            if cap == 0 {
                return Ok(buf_len);
            }

            if let Ok(cap) = cap.try_into() {
                // `path` cannot be longer than `usize`, only truncate if `cap` fits in `usize`
                path.truncate(cap);
            }
            cap = cap.checked_sub(path.len() as _).unwrap();
            buf = write_bytes(memory, buf, &path)?;
            if cap == 0 {
                return Ok(buf_len);
            }
        }
        Ok(buf_len.checked_sub(cap).unwrap())
    }

    #[instrument(skip(self, memory))]
    async fn path_create_directory(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
    ) -> Result<(), types::Error> {
        let dirfd = self.get_dir_fd(dirfd)?;
        let path = self.read_string(memory, path)?;
        self.as_wasi_impl()
            .create_directory_at(dirfd.borrowed(), path)
            .await?;
        Ok(())
    }

    /// Return the attributes of a file or directory.
    /// NOTE: This is similar to `stat` in POSIX.
    #[instrument(skip(self, memory))]
    async fn path_filestat_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: GuestPtr<str>,
    ) -> Result<types::Filestat, types::Error> {
        let dirfd = self.get_dir_fd(dirfd)?;
        let path = self.read_string(memory, path)?;
        let filesystem::DescriptorStat {
            type_,
            link_count: nlink,
            size,
            data_access_timestamp,
            data_modification_timestamp,
            status_change_timestamp,
        } = self
            .as_wasi_impl()
            .stat_at(dirfd.borrowed(), flags.into(), path.clone())
            .await?;
        let metadata_hash = self
            .as_wasi_impl()
            .metadata_hash_at(dirfd, flags.into(), path)
            .await?;
        let filetype = type_.try_into().map_err(types::Error::trap)?;
        let zero = wall_clock::Datetime {
            seconds: 0,
            nanoseconds: 0,
        };
        let atim = data_access_timestamp.unwrap_or(zero).try_into()?;
        let mtim = data_modification_timestamp.unwrap_or(zero).try_into()?;
        let ctim = status_change_timestamp.unwrap_or(zero).try_into()?;
        Ok(types::Filestat {
            dev: 1,
            ino: metadata_hash.lower,
            filetype,
            nlink,
            size,
            atim,
            mtim,
            ctim,
        })
    }

    /// Adjust the timestamps of a file or directory.
    /// NOTE: This is similar to `utimensat` in POSIX.
    #[instrument(skip(self, memory))]
    async fn path_filestat_set_times(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: GuestPtr<str>,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), types::Error> {
        let atim = systimespec(
            fst_flags.contains(types::Fstflags::ATIM),
            atim,
            fst_flags.contains(types::Fstflags::ATIM_NOW),
        )?;
        let mtim = systimespec(
            fst_flags.contains(types::Fstflags::MTIM),
            mtim,
            fst_flags.contains(types::Fstflags::MTIM_NOW),
        )?;

        let dirfd = self.get_dir_fd(dirfd)?;
        let path = self.read_string(memory, path)?;
        self.as_wasi_impl()
            .set_times_at(dirfd, flags.into(), path, atim, mtim)
            .await?;
        Ok(())
    }

    /// Create a hard link.
    /// NOTE: This is similar to `linkat` in POSIX.
    #[instrument(skip(self, memory))]
    async fn path_link(
        &mut self,
        memory: &mut GuestMemory<'_>,
        src_fd: types::Fd,
        src_flags: types::Lookupflags,
        src_path: GuestPtr<str>,
        target_fd: types::Fd,
        target_path: GuestPtr<str>,
    ) -> Result<(), types::Error> {
        let src_fd = self.get_dir_fd(src_fd)?;
        let target_fd = self.get_dir_fd(target_fd)?;
        let src_path = self.read_string(memory, src_path)?;
        let target_path = self.read_string(memory, target_path)?;
        self.as_wasi_impl()
            .link_at(src_fd, src_flags.into(), src_path, target_fd, target_path)
            .await?;
        Ok(())
    }

    /// Open a file or directory.
    /// NOTE: This is similar to `openat` in POSIX.
    #[instrument(skip(self, memory))]
    async fn path_open(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        dirflags: types::Lookupflags,
        path: GuestPtr<str>,
        oflags: types::Oflags,
        fs_rights_base: types::Rights,
        _fs_rights_inheriting: types::Rights,
        fdflags: types::Fdflags,
    ) -> Result<types::Fd, types::Error> {
        let path = self.read_string(memory, path)?;

        let mut flags = filesystem::DescriptorFlags::empty();
        if fs_rights_base.contains(types::Rights::FD_READ) {
            flags |= filesystem::DescriptorFlags::READ;
        }
        if fs_rights_base.contains(types::Rights::FD_WRITE) {
            flags |= filesystem::DescriptorFlags::WRITE;
        }
        if fdflags.contains(types::Fdflags::SYNC) {
            flags |= filesystem::DescriptorFlags::FILE_INTEGRITY_SYNC;
        }
        if fdflags.contains(types::Fdflags::DSYNC) {
            flags |= filesystem::DescriptorFlags::DATA_INTEGRITY_SYNC;
        }
        if fdflags.contains(types::Fdflags::RSYNC) {
            flags |= filesystem::DescriptorFlags::REQUESTED_WRITE_SYNC;
        }

        let t = self.transact()?;
        let dirfd = match t.get_descriptor(dirfd)? {
            Descriptor::Directory { fd, .. } => fd.borrowed(),
            Descriptor::File(_) => return Err(types::Errno::Notdir.into()),
            _ => return Err(types::Errno::Badf.into()),
        };
        drop(t);
        let fd = self
            .as_wasi_impl()
            .open_at(dirfd, dirflags.into(), path, oflags.into(), flags)
            .await?;
        let mut t = self.transact()?;
        let desc = match t.view.table.get(&fd)? {
            crate::p2::filesystem::Descriptor::Dir(_) => Descriptor::Directory {
                fd,
                preopen_path: None,
            },
            crate::p2::filesystem::Descriptor::File(_) => Descriptor::File(File {
                fd,
                position: Default::default(),
                append: fdflags.contains(types::Fdflags::APPEND),
                blocking_mode: BlockingMode::from_fdflags(&fdflags),
            }),
        };
        let fd = t.descriptors.push(desc)?;
        Ok(fd.into())
    }

    /// Read the contents of a symbolic link.
    /// NOTE: This is similar to `readlinkat` in POSIX.
    #[instrument(skip(self, memory))]
    async fn path_readlink(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
        buf: GuestPtr<u8>,
        buf_len: types::Size,
    ) -> Result<types::Size, types::Error> {
        let dirfd = self.get_dir_fd(dirfd)?;
        let path = self.read_string(memory, path)?;
        let mut path = self
            .as_wasi_impl()
            .readlink_at(dirfd, path)
            .await?
            .into_bytes();
        if let Ok(buf_len) = buf_len.try_into() {
            // `path` cannot be longer than `usize`, only truncate if `buf_len` fits in `usize`
            path.truncate(buf_len);
        }
        let n = path.len().try_into().map_err(|_| types::Errno::Overflow)?;
        write_bytes(memory, buf, &path)?;
        Ok(n)
    }

    #[instrument(skip(self, memory))]
    async fn path_remove_directory(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
    ) -> Result<(), types::Error> {
        let dirfd = self.get_dir_fd(dirfd)?;
        let path = self.read_string(memory, path)?;
        self.as_wasi_impl().remove_directory_at(dirfd, path).await?;
        Ok(())
    }

    /// Rename a file or directory.
    /// NOTE: This is similar to `renameat` in POSIX.
    #[instrument(skip(self, memory))]
    async fn path_rename(
        &mut self,
        memory: &mut GuestMemory<'_>,
        src_fd: types::Fd,
        src_path: GuestPtr<str>,
        dest_fd: types::Fd,
        dest_path: GuestPtr<str>,
    ) -> Result<(), types::Error> {
        let src_fd = self.get_dir_fd(src_fd)?;
        let dest_fd = self.get_dir_fd(dest_fd)?;
        let src_path = self.read_string(memory, src_path)?;
        let dest_path = self.read_string(memory, dest_path)?;
        self.as_wasi_impl()
            .rename_at(src_fd, src_path, dest_fd, dest_path)
            .await?;
        Ok(())
    }

    #[instrument(skip(self, memory))]
    async fn path_symlink(
        &mut self,
        memory: &mut GuestMemory<'_>,
        src_path: GuestPtr<str>,
        dirfd: types::Fd,
        dest_path: GuestPtr<str>,
    ) -> Result<(), types::Error> {
        let dirfd = self.get_dir_fd(dirfd)?;
        let src_path = self.read_string(memory, src_path)?;
        let dest_path = self.read_string(memory, dest_path)?;
        self.as_wasi_impl()
            .symlink_at(dirfd.borrowed(), src_path, dest_path)
            .await?;
        Ok(())
    }

    #[instrument(skip(self, memory))]
    async fn path_unlink_file(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
    ) -> Result<(), types::Error> {
        let dirfd = self.get_dir_fd(dirfd)?;
        let path = memory.as_cow_str(path)?.into_owned();
        self.as_wasi_impl()
            .unlink_file_at(dirfd.borrowed(), path)
            .await?;
        Ok(())
    }

    #[instrument(skip(self, memory))]
    async fn poll_oneoff(
        &mut self,
        memory: &mut GuestMemory<'_>,
        subs: GuestPtr<types::Subscription>,
        events: GuestPtr<types::Event>,
        nsubscriptions: types::Size,
    ) -> Result<types::Size, types::Error> {
        if nsubscriptions == 0 {
            // Indefinite sleeping is not supported in preview1.
            return Err(types::Errno::Inval.into());
        }

        // This is a special case where `poll_oneoff` is just sleeping
        // on a single relative timer event. This special case was added
        // after experimental observations showed that std::thread::sleep
        // results in more consistent sleep times. This design ensures that
        // wasmtime can handle real-time requirements more accurately.
        if nsubscriptions == 1 {
            let sub = memory.read(subs)?;
            if let types::SubscriptionU::Clock(clocksub) = sub.u {
                if !clocksub
                    .flags
                    .contains(types::Subclockflags::SUBSCRIPTION_CLOCK_ABSTIME)
                    && self.wasi.allow_blocking_current_thread
                {
                    std::thread::sleep(std::time::Duration::from_nanos(clocksub.timeout));
                    memory.write(
                        events,
                        types::Event {
                            userdata: sub.userdata,
                            error: types::Errno::Success,
                            type_: types::Eventtype::Clock,
                            fd_readwrite: types::EventFdReadwrite {
                                flags: types::Eventrwflags::empty(),
                                nbytes: 1,
                            },
                        },
                    )?;
                    return Ok(1);
                }
            }
        }

        let subs = subs.as_array(nsubscriptions);
        let events = events.as_array(nsubscriptions);
        let n = usize::try_from(nsubscriptions).unwrap_or(usize::MAX);

        self.consume_fuel_for_array(subs)?;
        self.consume_fuel_for_array(events)?;

        let mut temp = TempResources {
            ctx: self,
            pollables: Vec::with_capacity(n),
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        let mut borrowed_pollables = Vec::with_capacity(n);
        for sub in subs.iter() {
            let sub = memory.read(sub?)?;
            let p = match sub.u {
                types::SubscriptionU::Clock(types::SubscriptionClock {
                    id,
                    timeout,
                    flags,
                    ..
                }) => {
                    let absolute = flags.contains(types::Subclockflags::SUBSCRIPTION_CLOCK_ABSTIME);
                    let (timeout, absolute) = match id {
                        types::Clockid::Monotonic => (timeout, absolute),
                        types::Clockid::Realtime if !absolute => (timeout, false),
                        types::Clockid::Realtime => {
                            let now = wall_clock::Host::now(&mut temp.ctx.as_wasi_impl())
                                .context("failed to call `wall_clock::now`")
                                .map_err(types::Error::trap)?;

                            // Convert `timeout` to `Datetime` format.
                            let seconds = timeout / 1_000_000_000;
                            let nanoseconds = timeout % 1_000_000_000;

                            let timeout = if now.seconds < seconds
                                || now.seconds == seconds
                                    && u64::from(now.nanoseconds) < nanoseconds
                            {
                                // `now` is less than `timeout`, which is expressible as u64,
                                // subtract the nanosecond counts directly
                                now.seconds * 1_000_000_000 + u64::from(now.nanoseconds) - timeout
                            } else {
                                0
                            };
                            (timeout, false)
                        }
                        _ => return Err(types::Errno::Inval.into()),
                    };
                    if absolute {
                        monotonic_clock::Host::subscribe_instant(
                            &mut temp.ctx.as_wasi_impl(),
                            timeout,
                        )
                        .context("failed to call `monotonic_clock::subscribe_instant`")
                        .map_err(types::Error::trap)?
                    } else {
                        monotonic_clock::Host::subscribe_duration(
                            &mut temp.ctx.as_wasi_impl(),
                            timeout,
                        )
                        .context("failed to call `monotonic_clock::subscribe_duration`")
                        .map_err(types::Error::trap)?
                    }
                }
                types::SubscriptionU::FdRead(types::SubscriptionFdReadwrite {
                    file_descriptor,
                }) => {
                    let stream = {
                        let t = temp.ctx.transact()?;
                        let desc = t.get_descriptor(file_descriptor)?;
                        match desc {
                            Descriptor::Stdin { stream, .. } => stream.borrowed(),
                            Descriptor::File(File { fd, position, .. }) => {
                                let pos = position.load(Ordering::Relaxed);
                                let fd = fd.borrowed();
                                drop(t);
                                let r = temp.ctx.as_wasi_impl().read_via_stream(fd, pos)?;
                                let ret = r.borrowed();
                                temp.inputs.push(r);
                                ret
                            }
                            // TODO: Support sockets
                            _ => return Err(types::Errno::Badf.into()),
                        }
                    };
                    streams::HostInputStream::subscribe(&mut temp.ctx.as_io_impl(), stream)
                        .context("failed to call `subscribe` on `input-stream`")
                        .map_err(types::Error::trap)?
                }
                types::SubscriptionU::FdWrite(types::SubscriptionFdReadwrite {
                    file_descriptor,
                }) => {
                    let stream = {
                        let t = temp.ctx.transact()?;
                        let desc = t.get_descriptor(file_descriptor)?;
                        match desc {
                            Descriptor::Stdout { stream, .. }
                            | Descriptor::Stderr { stream, .. } => stream.borrowed(),
                            Descriptor::File(File {
                                fd,
                                position,
                                append,
                                ..
                            }) => {
                                let fd = fd.borrowed();
                                let position = position.clone();
                                let append = *append;
                                drop(t);
                                let r = if append {
                                    temp.ctx.as_wasi_impl().append_via_stream(fd)?
                                } else {
                                    let pos = position.load(Ordering::Relaxed);
                                    temp.ctx.as_wasi_impl().write_via_stream(fd, pos)?
                                };
                                let ret = r.borrowed();
                                temp.outputs.push(r);
                                ret
                            }
                            // TODO: Support sockets
                            _ => return Err(types::Errno::Badf.into()),
                        }
                    };
                    streams::HostOutputStream::subscribe(&mut temp.ctx.as_io_impl(), stream)
                        .context("failed to call `subscribe` on `output-stream`")
                        .map_err(types::Error::trap)?
                }
            };
            borrowed_pollables.push(p.borrowed());
            temp.pollables.push(p);
        }
        let ready: HashSet<_> = temp
            .ctx
            .as_io_impl()
            .poll(borrowed_pollables)
            .await
            .context("failed to call `poll-oneoff`")
            .map_err(types::Error::trap)?
            .into_iter()
            .collect();
        drop(temp);

        let mut count: types::Size = 0;
        for (sub, event) in (0..)
            .zip(subs.iter())
            .filter_map(|(idx, sub)| ready.contains(&idx).then_some(sub))
            .zip(events.iter())
        {
            let sub = memory.read(sub?)?;
            let event = event?;
            let e = match sub.u {
                types::SubscriptionU::Clock(..) => types::Event {
                    userdata: sub.userdata,
                    error: types::Errno::Success,
                    type_: types::Eventtype::Clock,
                    fd_readwrite: types::EventFdReadwrite {
                        flags: types::Eventrwflags::empty(),
                        nbytes: 0,
                    },
                },
                types::SubscriptionU::FdRead(types::SubscriptionFdReadwrite {
                    file_descriptor,
                }) => {
                    let t = self.transact()?;
                    let desc = t.get_descriptor(file_descriptor)?;
                    match desc {
                        Descriptor::Stdin { .. } => types::Event {
                            userdata: sub.userdata,
                            error: types::Errno::Success,
                            type_: types::Eventtype::FdRead,
                            fd_readwrite: types::EventFdReadwrite {
                                flags: types::Eventrwflags::empty(),
                                nbytes: 1,
                            },
                        },
                        Descriptor::File(File { fd, position, .. }) => {
                            let fd = fd.borrowed();
                            let position = position.clone();
                            drop(t);
                            match self.as_wasi_impl().stat(fd).await? {
                                filesystem::DescriptorStat { size, .. } => {
                                    let pos = position.load(Ordering::Relaxed);
                                    let nbytes = size.saturating_sub(pos);
                                    types::Event {
                                        userdata: sub.userdata,
                                        error: types::Errno::Success,
                                        type_: types::Eventtype::FdRead,
                                        fd_readwrite: types::EventFdReadwrite {
                                            flags: if nbytes == 0 {
                                                types::Eventrwflags::FD_READWRITE_HANGUP
                                            } else {
                                                types::Eventrwflags::empty()
                                            },
                                            nbytes: 1,
                                        },
                                    }
                                }
                            }
                        }
                        // TODO: Support sockets
                        _ => return Err(types::Errno::Badf.into()),
                    }
                }
                types::SubscriptionU::FdWrite(types::SubscriptionFdReadwrite {
                    file_descriptor,
                }) => {
                    let t = self.transact()?;
                    let desc = t.get_descriptor(file_descriptor)?;
                    match desc {
                        Descriptor::Stdout { .. } | Descriptor::Stderr { .. } => types::Event {
                            userdata: sub.userdata,
                            error: types::Errno::Success,
                            type_: types::Eventtype::FdWrite,
                            fd_readwrite: types::EventFdReadwrite {
                                flags: types::Eventrwflags::empty(),
                                nbytes: 1,
                            },
                        },
                        Descriptor::File(_) => types::Event {
                            userdata: sub.userdata,
                            error: types::Errno::Success,
                            type_: types::Eventtype::FdWrite,
                            fd_readwrite: types::EventFdReadwrite {
                                flags: types::Eventrwflags::empty(),
                                nbytes: 1,
                            },
                        },
                        // TODO: Support sockets
                        _ => return Err(types::Errno::Badf.into()),
                    }
                }
            };
            memory.write(event, e)?;
            count = count
                .checked_add(1)
                .ok_or_else(|| types::Error::from(types::Errno::Overflow))?
        }
        return Ok(count);

        struct TempResources<'a> {
            ctx: &'a mut WasiP1Ctx,
            pollables: Vec<Resource<crate::p2::bindings::io::streams::Pollable>>,
            inputs: Vec<Resource<crate::p2::bindings::io::streams::InputStream>>,
            outputs: Vec<Resource<crate::p2::bindings::io::streams::OutputStream>>,
        }

        impl Drop for TempResources<'_> {
            fn drop(&mut self) {
                for p in self.pollables.drain(..) {
                    use wasmtime_wasi_io::bindings::wasi::io::poll::HostPollable;
                    self.ctx.table.drop(p).unwrap();
                }
                for p in self.inputs.drain(..) {
                    assert!(p.owned());
                    self.ctx.table.delete(p).unwrap();
                }
                for p in self.outputs.drain(..) {
                    assert!(p.owned());
                    self.ctx.table.delete(p).unwrap();
                }
            }
        }
    }

    #[instrument(skip(self, _memory))]
    fn proc_exit(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        status: types::Exitcode,
    ) -> anyhow::Error {
        // Check that the status is within WASI's range.
        if status >= 126 {
            return anyhow::Error::msg("exit with invalid exit status outside of [0..126)");
        }
        crate::I32Exit(status as i32).into()
    }

    #[instrument(skip(self, _memory))]
    fn proc_raise(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        _sig: types::Signal,
    ) -> Result<(), types::Error> {
        Err(types::Errno::Notsup.into())
    }

    #[instrument(skip(self, _memory))]
    fn sched_yield(&mut self, _memory: &mut GuestMemory<'_>) -> Result<(), types::Error> {
        // No such thing in preview 2. Intentionally left empty.
        Ok(())
    }

    #[instrument(skip(self, memory))]
    fn random_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        buf: GuestPtr<u8>,
        buf_len: types::Size,
    ) -> Result<(), types::Error> {
        let rand = self
            .wasi
            .random
            .get_random_bytes(buf_len.into())
            .context("failed to call `get-random-bytes`")
            .map_err(types::Error::trap)?;
        write_bytes(memory, buf, &rand)?;
        Ok(())
    }

    #[instrument(skip(self, _memory))]
    fn sock_accept(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        flags: types::Fdflags,
    ) -> Result<types::Fd, types::Error> {
        tracing::warn!("preview1 sock_accept is not implemented");
        self.transact()?.get_descriptor(fd)?;
        Err(types::Errno::Notsock.into())
    }

    #[instrument(skip(self, _memory))]
    fn sock_recv(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        ri_data: types::IovecArray,
        ri_flags: types::Riflags,
    ) -> Result<(types::Size, types::Roflags), types::Error> {
        tracing::warn!("preview1 sock_recv is not implemented");
        self.transact()?.get_descriptor(fd)?;
        Err(types::Errno::Notsock.into())
    }

    #[instrument(skip(self, _memory))]
    fn sock_send(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        si_data: types::CiovecArray,
        _si_flags: types::Siflags,
    ) -> Result<types::Size, types::Error> {
        tracing::warn!("preview1 sock_send is not implemented");
        self.transact()?.get_descriptor(fd)?;
        Err(types::Errno::Notsock.into())
    }

    #[instrument(skip(self, _memory))]
    fn sock_shutdown(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        how: types::Sdflags,
    ) -> Result<(), types::Error> {
        tracing::warn!("preview1 sock_shutdown is not implemented");
        self.transact()?.get_descriptor(fd)?;
        Err(types::Errno::Notsock.into())
    }
}

trait ResourceExt<T> {
    fn borrowed(&self) -> Resource<T>;
}

impl<T: 'static> ResourceExt<T> for Resource<T> {
    fn borrowed(&self) -> Resource<T> {
        Resource::new_borrow(self.rep())
    }
}
