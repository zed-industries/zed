//! Functionality for reading a remote process's memory

use super::{minidump_writer::MinidumpWriter, serializers::*, Pid};

enum Style {
    /// Uses [`process_vm_readv`](https://linux.die.net/man/2/process_vm_readv)
    /// to read the memory.
    ///
    /// This is not available on old <3.2 (really, ancient) kernels, and requires
    /// the same permissions as ptrace
    VirtualMem,
    /// Reads the memory from `/proc/<pid>/mem`
    ///
    /// Available on basically all versions of Linux, but could fail if the process
    /// has insufficient privileges, ie ptrace
    File(std::fs::File),
    /// Reads the memory with [ptrace (`PTRACE_PEEKDATA`)](https://man7.org/linux/man-pages/man2/ptrace.2.html)
    ///
    /// Reads data one word at a time, so slow, but fairly reliable, as long as
    /// the process can be ptraced
    Ptrace,
    /// No methods succeeded, generally there isn't a case where failing a syscall
    /// will work if called again
    Unavailable {
        vmem: nix::Error,
        file: nix::Error,
        ptrace: nix::Error,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error("Copy from process {child} failed (source {src}, offset: {offset}, length: {length})")]
pub struct CopyFromProcessError {
    pub child: Pid,
    pub src: usize,
    pub offset: usize,
    pub length: usize,
    #[serde(serialize_with = "serialize_nix_error")]
    pub source: nix::Error,
}

pub struct MemReader {
    /// The pid of the child to read
    pid: nix::unistd::Pid,
    style: Option<Style>,
}

impl std::fmt::Debug for MemReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.style {
            Some(Style::VirtualMem) => "process_vm_readv",
            Some(Style::File(_)) => "/proc/<pid>/mem",
            Some(Style::Ptrace) => "PTRACE_PEEKDATA",
            Some(Style::Unavailable { vmem, file, ptrace }) => {
                return write!(
                    f,
                    "process_vm_readv: {vmem}, /proc/<pid>/mem: {file}, PTRACE_PEEKDATA: {ptrace}"
                );
            }
            None => "unknown",
        };

        f.write_str(s)
    }
}

impl MemReader {
    /// Creates a [`Self`] for the specified process id, the method used will
    /// be probed for on the first access
    #[inline]
    pub fn new(pid: i32) -> Self {
        Self {
            pid: nix::unistd::Pid::from_raw(pid),
            style: None,
        }
    }

    #[inline]
    #[doc(hidden)]
    pub fn for_virtual_mem(pid: i32) -> Self {
        Self {
            pid: nix::unistd::Pid::from_raw(pid),
            style: Some(Style::VirtualMem),
        }
    }

    #[inline]
    #[doc(hidden)]
    pub fn for_file(pid: i32) -> std::io::Result<Self> {
        let file = std::fs::File::open(format!("/proc/{pid}/mem"))?;

        Ok(Self {
            pid: nix::unistd::Pid::from_raw(pid),
            style: Some(Style::File(file)),
        })
    }

    #[inline]
    #[doc(hidden)]
    pub fn for_ptrace(pid: i32) -> Self {
        Self {
            pid: nix::unistd::Pid::from_raw(pid),
            style: Some(Style::Ptrace),
        }
    }

    #[inline]
    pub fn read_to_vec(
        &mut self,
        src: usize,
        length: std::num::NonZeroUsize,
    ) -> Result<Vec<u8>, CopyFromProcessError> {
        let mut output = vec![0u8; length.into()];
        let bytes_read = self.read(src, &mut output)?;
        output.truncate(bytes_read);
        Ok(output)
    }

    pub fn read(&mut self, src: usize, dst: &mut [u8]) -> Result<usize, CopyFromProcessError> {
        if let Some(rs) = &mut self.style {
            let res = match rs {
                Style::VirtualMem => Self::vmem(self.pid, src, dst).map_err(|s| (s, 0)),
                Style::File(file) => Self::file(file, src, dst).map_err(|s| (s, 0)),
                Style::Ptrace => Self::ptrace(self.pid, src, dst),
                Style::Unavailable { ptrace, .. } => Err((*ptrace, 0)),
            };

            return res.map_err(|(source, offset)| CopyFromProcessError {
                child: self.pid.as_raw(),
                src,
                offset,
                length: dst.len(),
                source,
            });
        }

        // Attempt to read in order of speed
        let vmem = match Self::vmem(self.pid, src, dst) {
            Ok(len) => {
                self.style = Some(Style::VirtualMem);
                return Ok(len);
            }
            Err(err) => err,
        };

        let file = match std::fs::File::open(format!("/proc/{}/mem", self.pid)) {
            Ok(mut file) => match Self::file(&mut file, src, dst) {
                Ok(len) => {
                    self.style = Some(Style::File(file));
                    return Ok(len);
                }
                Err(err) => err,
            },
            Err(err) => nix::Error::from_raw(err.raw_os_error().expect(
                "failed to open /proc/<pid>/mem and the I/O error doesn't have an OS code",
            )),
        };

        let ptrace = match Self::ptrace(self.pid, src, dst) {
            Ok(len) => {
                self.style = Some(Style::Ptrace);
                return Ok(len);
            }
            Err((err, _)) => err,
        };

        self.style = Some(Style::Unavailable { vmem, file, ptrace });
        Err(CopyFromProcessError {
            child: self.pid.as_raw(),
            src,
            offset: 0,
            length: dst.len(),
            source: ptrace,
        })
    }

    #[inline]
    fn vmem(pid: nix::unistd::Pid, src: usize, dst: &mut [u8]) -> Result<usize, nix::Error> {
        let remote = &[nix::sys::uio::RemoteIoVec {
            base: src,
            len: dst.len(),
        }];
        nix::sys::uio::process_vm_readv(pid, &mut [std::io::IoSliceMut::new(dst)], remote)
    }

    #[inline]
    fn file(file: &mut std::fs::File, src: usize, dst: &mut [u8]) -> Result<usize, nix::Error> {
        use std::os::unix::fs::FileExt;

        file.read_exact_at(dst, src as u64).map_err(|err| {
            if let Some(os) = err.raw_os_error() {
                nix::Error::from_raw(os)
            } else {
                nix::Error::E2BIG /* EOF */
            }
        })?;

        Ok(dst.len())
    }

    #[inline]
    fn ptrace(
        pid: nix::unistd::Pid,
        src: usize,
        dst: &mut [u8],
    ) -> Result<usize, (nix::Error, usize)> {
        let mut offset = 0;
        let mut chunks = dst.chunks_exact_mut(std::mem::size_of::<usize>());

        for chunk in chunks.by_ref() {
            let word = nix::sys::ptrace::read(pid, (src + offset) as *mut std::ffi::c_void)
                .map_err(|err| (err, offset))?;
            chunk.copy_from_slice(&word.to_ne_bytes());
            offset += std::mem::size_of::<usize>();
        }

        // I don't think there would ever be a case where we would not read on word boundaries, but just in case...
        let last = chunks.into_remainder();
        if !last.is_empty() {
            let word = nix::sys::ptrace::read(pid, (src + offset) as *mut std::ffi::c_void)
                .map_err(|err| (err, offset))?;
            last.copy_from_slice(&word.to_ne_bytes()[..last.len()]);
        }

        Ok(dst.len())
    }
}

impl MinidumpWriter {
    /// Copies a block of bytes from the target process, returning the heap
    /// allocated copy
    #[inline]
    pub fn copy_from_process(
        pid: Pid,
        src: usize,
        length: usize,
    ) -> Result<Vec<u8>, CopyFromProcessError> {
        let length = std::num::NonZeroUsize::new(length).ok_or(CopyFromProcessError {
            src,
            child: pid,
            offset: 0,
            length,
            // TODO: We should make copy_from_process also take a NonZero,
            // as EINVAL could also come from the syscalls that actually read
            // memory as well which could be confusing
            source: nix::errno::Errno::EINVAL,
        })?;

        let mut mem = MemReader::new(pid);
        mem.read_to_vec(src, length)
    }
}
