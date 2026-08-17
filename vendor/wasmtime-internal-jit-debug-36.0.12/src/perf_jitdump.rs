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

use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;
use std::ptr;
use std::string::String;
use std::vec::Vec;
use std::{mem, process};

/// Defines jitdump record types
#[repr(u32)]
pub enum RecordId {
    /// Value 0: JIT_CODE_LOAD: record describing a jitted function
    JitCodeLoad = 0,
    /// Value 1: JIT_CODE_MOVE: record describing an already jitted function which is moved
    _JitCodeMove = 1,
    /// Value 2: JIT_CODE_DEBUG_INFO: record describing the debug information for a jitted function
    JitCodeDebugInfo = 2,
    /// Value 3: JIT_CODE_CLOSE: record marking the end of the jit runtime (optional)
    _JitCodeClose = 3,
    /// Value 4: JIT_CODE_UNWINDING_INFO: record describing a function unwinding information
    _JitCodeUnwindingInfo = 4,
}

/// Each record starts with this fixed size record header which describes the record that follows
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct RecordHeader {
    /// uint32_t id: a value identifying the record type (see below)
    pub id: u32,
    /// uint32_t total_size: the size in bytes of the record including the header.
    pub record_size: u32,
    /// uint64_t timestamp: a timestamp of when the record was created.
    pub timestamp: u64,
}

unsafe impl object::Pod for RecordHeader {}

/// The CodeLoadRecord is used for describing jitted functions
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct CodeLoadRecord {
    /// Fixed sized header that describes this record
    pub header: RecordHeader,
    /// `uint32_t pid`: OS process id of the runtime generating the jitted code
    pub pid: u32,
    /// `uint32_t tid`: OS thread identification of the runtime thread generating the jitted code
    pub tid: u32,
    /// `uint64_t vma`: virtual address of jitted code start
    pub virtual_address: u64,
    /// `uint64_t code_addr`: code start address for the jitted code. By default vma = code_addr
    pub address: u64,
    /// `uint64_t code_size`: size in bytes of the generated jitted code
    pub size: u64,
    /// `uint64_t code_index`: unique identifier for the jitted code (see below)
    pub index: u64,
}

unsafe impl object::Pod for CodeLoadRecord {}

/// Describes source line information for a jitted function
#[derive(Debug, Default)]
#[repr(C)]
pub struct DebugEntry {
    /// `uint64_t code_addr`: address of function for which the debug information is generated
    pub address: u64,
    /// `uint32_t line`: source file line number (starting at 1)
    pub line: u32,
    /// `uint32_t discrim`: column discriminator, 0 is default
    pub discriminator: u32,
    /// `char name[n]`: source file name in ASCII, including null termination
    pub filename: String,
}

/// Describes debug information for a jitted function. An array of debug entries are
/// appended to this record during writing. Note, this record must precede the code
/// load record that describes the same jitted function.
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct DebugInfoRecord {
    /// Fixed sized header that describes this record
    pub header: RecordHeader,
    /// `uint64_t code_addr`: address of function for which the debug information is generated
    pub address: u64,
    /// `uint64_t nr_entry`: number of debug entries for the function appended to this record
    pub count: u64,
}

unsafe impl object::Pod for DebugInfoRecord {}

/// Fixed-sized header for each jitdump file
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct FileHeader {
    /// `uint32_t magic`: a magic number tagging the file type. The value is 4-byte long and represents the
    /// string "JiTD" in ASCII form. It is 0x4A695444 or 0x4454694a depending on the endianness. The field can
    /// be used to detect the endianness of the file
    pub magic: u32,
    /// `uint32_t version`: a 4-byte value representing the format version. It is currently set to 2
    pub version: u32,
    /// `uint32_t total_size`: size in bytes of file header
    pub size: u32,
    /// `uint32_t elf_mach`: ELF architecture encoding (ELF e_machine value as specified in /usr/include/elf.h)
    pub e_machine: u32,
    /// `uint32_t pad1`: padding. Reserved for future use
    pub pad1: u32,
    /// `uint32_t pid`: JIT runtime process identification (OS specific)
    pub pid: u32,
    /// `uint64_t timestamp`: timestamp of when the file was created
    pub timestamp: u64,
    /// `uint64_t flags`: a bitmask of flags
    pub flags: u64,
}

unsafe impl object::Pod for FileHeader {}

/// Interface for driving the creation of jitdump files
pub struct JitDumpFile {
    /// File instance for the jit dump file
    jitdump_file: File,

    map_addr: usize,
    map_len: usize,

    /// Unique identifier for jitted code
    code_index: u64,

    e_machine: u32,
}

impl JitDumpFile {
    /// Initialize a JitDumpAgent and write out the header
    pub fn new(filename: impl AsRef<Path>, e_machine: u32) -> io::Result<Self> {
        let jitdump_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename.as_ref())?;

        // After we make our `*.dump` file we execute an `mmap` syscall,
        // specifically with executable permissions, to map it into our address
        // space. This is required so `perf inject` will work later. The `perf
        // inject` command will see that an mmap syscall happened, and it'll see
        // the filename we mapped, and that'll trigger it to actually read and
        // parse the file.
        //
        // To match what some perf examples are doing we keep this `mmap` alive
        // until this agent goes away.
        let map_len = 1024;
        let map_addr = unsafe {
            let ptr = rustix::mm::mmap(
                ptr::null_mut(),
                map_len,
                rustix::mm::ProtFlags::EXEC | rustix::mm::ProtFlags::READ,
                rustix::mm::MapFlags::PRIVATE,
                &jitdump_file,
                0,
            )?;
            ptr as usize
        };
        let mut state = JitDumpFile {
            jitdump_file,
            map_addr,
            map_len,
            code_index: 0,
            e_machine,
        };
        state.write_file_header()?;
        Ok(state)
    }
}

impl JitDumpFile {
    /// Returns timestamp from a single source
    pub fn get_time_stamp(&self) -> u64 {
        // We need to use `CLOCK_MONOTONIC` on Linux which is what `Instant`
        // conveniently also uses, but `Instant` doesn't allow us to get access
        // to nanoseconds as an internal detail, so we calculate the nanoseconds
        // ourselves here.
        let ts = rustix::time::clock_gettime(rustix::time::ClockId::Monotonic);
        // TODO: What does it mean for either sec or nsec to be negative?
        (ts.tv_sec * 1_000_000_000 + ts.tv_nsec) as u64
    }

    /// Returns the next code index
    pub fn next_code_index(&mut self) -> u64 {
        let code_index = self.code_index;
        self.code_index += 1;
        code_index
    }

    pub fn write_file_header(&mut self) -> io::Result<()> {
        let header = FileHeader {
            timestamp: self.get_time_stamp(),
            e_machine: self.e_machine,
            magic: 0x4A695444,
            version: 1,
            size: mem::size_of::<FileHeader>() as u32,
            pad1: 0,
            pid: process::id(),
            flags: 0,
        };

        self.jitdump_file.write_all(object::bytes_of(&header))?;
        Ok(())
    }

    pub fn write_code_load_record(
        &mut self,
        record_name: &str,
        cl_record: CodeLoadRecord,
        code_buffer: &[u8],
    ) -> io::Result<()> {
        self.jitdump_file.write_all(object::bytes_of(&cl_record))?;
        self.jitdump_file.write_all(record_name.as_bytes())?;
        self.jitdump_file.write_all(b"\0")?;
        self.jitdump_file.write_all(code_buffer)?;
        Ok(())
    }

    /// Write DebugInfoRecord to open jit dump file.
    /// Must be written before the corresponding CodeLoadRecord.
    pub fn write_debug_info_record(&mut self, dir_record: DebugInfoRecord) -> io::Result<()> {
        self.jitdump_file.write_all(object::bytes_of(&dir_record))?;
        Ok(())
    }

    /// Write DebugInfoRecord to open jit dump file.
    /// Must be written before the corresponding CodeLoadRecord.
    pub fn write_debug_info_entries(&mut self, die_entries: Vec<DebugEntry>) -> io::Result<()> {
        for entry in die_entries.iter() {
            self.jitdump_file
                .write_all(object::bytes_of(&entry.address))?;
            self.jitdump_file.write_all(object::bytes_of(&entry.line))?;
            self.jitdump_file
                .write_all(object::bytes_of(&entry.discriminator))?;
            self.jitdump_file.write_all(entry.filename.as_bytes())?;
            self.jitdump_file.write_all(b"\0")?;
        }
        Ok(())
    }

    pub fn dump_code_load_record(
        &mut self,
        method_name: &str,
        code: &[u8],
        timestamp: u64,
        pid: u32,
        tid: u32,
    ) -> io::Result<()> {
        let name_len = method_name.len() + 1;
        let size_limit = mem::size_of::<CodeLoadRecord>();

        let rh = RecordHeader {
            id: RecordId::JitCodeLoad as u32,
            record_size: size_limit as u32 + name_len as u32 + code.len() as u32,
            timestamp,
        };

        let clr = CodeLoadRecord {
            header: rh,
            pid,
            tid,
            virtual_address: code.as_ptr() as u64,
            address: code.as_ptr() as u64,
            size: code.len() as u64,
            index: self.next_code_index(),
        };

        self.write_code_load_record(method_name, clr, code)
    }
}

impl Drop for JitDumpFile {
    fn drop(&mut self) {
        unsafe {
            rustix::mm::munmap(self.map_addr as *mut _, self.map_len).unwrap();
        }
    }
}
