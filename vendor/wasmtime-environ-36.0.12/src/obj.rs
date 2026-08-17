//! Utilities for working with object files that operate as Wasmtime's
//! serialization and intermediate format for compiled modules.

use core::fmt;

/// Filler for the `os_abi` field of the ELF header.
///
/// This is just a constant that seems reasonable in the sense it's unlikely to
/// clash with others.
pub const ELFOSABI_WASMTIME: u8 = 200;

/// Flag for the `e_flags` field in the ELF header indicating a compiled
/// module.
pub const EF_WASMTIME_MODULE: u32 = 1 << 0;

/// Flag for the `e_flags` field in the ELF header indicating a compiled
/// component.
pub const EF_WASMTIME_COMPONENT: u32 = 1 << 1;

/// Flag for the `e_flags` field in the ELF header indicating compiled code for
/// pulley32
pub const EF_WASMTIME_PULLEY32: u32 = 1 << 2;

/// Flag for the `e_flags` field in the ELF header indicating compiled code for
/// pulley64
pub const EF_WASMTIME_PULLEY64: u32 = 1 << 3;

/// Flag for the `sh_flags` field in the ELF text section that indicates that
/// the text section does not itself need to be executable. This is used for the
/// Pulley target, for example, to indicate that it does not need to be made
/// natively executable as it does not contain actual native code.
pub const SH_WASMTIME_NOT_EXECUTED: u64 = 1 << 0;

/// A custom Wasmtime-specific section of our compilation image which stores
/// mapping data from offsets in the image to offset in the original wasm
/// binary.
///
/// This section has a custom binary encoding. Currently its encoding is:
///
/// * The section starts with a 32-bit little-endian integer. This integer is
///   how many entries are in the following two arrays.
/// * Next is an array with the previous count number of 32-bit little-endian
///   integers. This array is a sorted list of relative offsets within the text
///   section. This is intended to be a lookup array to perform a binary search
///   on an offset within the text section on this array.
/// * Finally there is another array, with the same count as before, also of
///   32-bit little-endian integers. These integers map 1:1 with the previous
///   array of offsets, and correspond to what the original offset was in the
///   wasm file.
///
/// Decoding this section is intentionally simple, it only requires loading a
/// 32-bit little-endian integer plus some bounds checks. Reading this section
/// is done with the `lookup_file_pos` function below. Reading involves
/// performing a binary search on the first array using the index found for the
/// native code offset to index into the second array and find the wasm code
/// offset.
///
/// At this time this section has an alignment of 1, which means all reads of it
/// are unaligned. Additionally at this time the 32-bit encodings chosen here
/// mean that >=4gb text sections are not supported.
pub const ELF_WASMTIME_ADDRMAP: &str = ".wasmtime.addrmap";

/// A custom Wasmtime-specific section of compilation which store information
/// about live gc references at various locations in the text section (stack
/// maps).
///
/// This section has a custom binary encoding described in `stack_maps.rs` which
/// is used to implement the single query we want to satisfy of: where are the
/// live GC references at this pc? Like the addrmap section this has an
/// alignment of 1 with unaligned reads, and it additionally doesn't support
/// >=4gb text sections.
pub const ELF_WASMTIME_STACK_MAP: &str = ".wasmtime.stackmap";

/// A custom binary-encoded section of wasmtime compilation artifacts which
/// encodes the ability to map an offset in the text section to the trap code
/// that it corresponds to.
///
/// This section is used at runtime to determine what flavor of trap happened to
/// ensure that embedders and debuggers know the reason for the wasm trap. The
/// encoding of this section is custom to Wasmtime and managed with helpers in
/// the `object` crate:
///
/// * First the section has a 32-bit little endian integer indicating how many
///   trap entries are in the section.
/// * Next is an array, of the same length as read before, of 32-bit
///   little-endian integers. These integers are offsets into the text section
///   of the compilation image.
/// * Finally is the same count number of bytes. Each of these bytes corresponds
///   to a trap code.
///
/// This section is decoded by `lookup_trap_code` below which will read the
/// section count, slice some bytes to get the various arrays, and then perform
/// a binary search on the offsets array to find the index corresponding to
/// the pc being looked up. If found the same index in the trap array (the array
/// of bytes) is the trap code for that offset.
///
/// Note that at this time this section has an alignment of 1. Additionally due
/// to the 32-bit encodings for offsets this doesn't support images >=4gb.
pub const ELF_WASMTIME_TRAPS: &str = ".wasmtime.traps";

/// A custom section which consists of just 1 byte which is either 0 or 1 as to
/// whether BTI is enabled.
pub const ELF_WASM_BTI: &str = ".wasmtime.bti";

/// A bincode-encoded section containing engine-specific metadata used to
/// double-check that an artifact can be loaded into the current host.
pub const ELF_WASM_ENGINE: &str = ".wasmtime.engine";

/// This is the name of the section in the final ELF image which contains
/// concatenated data segments from the original wasm module.
///
/// This section is simply a list of bytes and ranges into this section are
/// stored within a `Module` for each data segment. Memory initialization and
/// passive segment management all index data directly located in this section.
///
/// Note that this implementation does not afford any method of leveraging the
/// `data.drop` instruction to actually release the data back to the OS. The
/// data section is simply always present in the ELF image. If we wanted to
/// release the data it's probably best to figure out what the best
/// implementation is for it at the time given a particular set of constraints.
pub const ELF_WASM_DATA: &'static str = ".rodata.wasm";

/// This is the name of the section in the final ELF image which contains a
/// `bincode`-encoded `CompiledModuleInfo`.
///
/// This section is optionally decoded in `CompiledModule::from_artifacts`
/// depending on whether or not a `CompiledModuleInfo` is already available. In
/// cases like `Module::new` where compilation directly leads into consumption,
/// it's available. In cases like `Module::deserialize` this section must be
/// decoded to get all the relevant information.
pub const ELF_WASMTIME_INFO: &'static str = ".wasmtime.info";

/// This is the name of the section in the final ELF image which contains a
/// concatenated list of all function names.
///
/// This section is optionally included in the final artifact depending on
/// whether the wasm module has any name data at all (or in the future if we add
/// an option to not preserve name data). This section is a concatenated list of
/// strings where `CompiledModuleInfo::func_names` stores offsets/lengths into
/// this section.
///
/// Note that the goal of this section is to avoid having to decode names at
/// module-load time if we can. Names are typically only used for debugging or
/// things like backtraces so there's no need to eagerly load all of them. By
/// storing the data in a separate section the hope is that the data, which is
/// sometimes quite large (3MB seen for spidermonkey-compiled-to-wasm), can be
/// paged in lazily from an mmap and is never paged in if we never reference it.
pub const ELF_NAME_DATA: &'static str = ".name.wasm";

/// This is the name of the section in the final ELF image that contains the
/// concatenation of all the native DWARF information found in the original wasm
/// files.
///
/// This concatenation is not intended to be read by external tools at this time
/// and is instead indexed directly by relative indices stored in compilation
/// metadata.
pub const ELF_WASMTIME_DWARF: &str = ".wasmtime.dwarf";

/// Workaround to implement `core::error::Error` until
/// gimli-rs/object#747 is settled.
pub struct ObjectCrateErrorWrapper(pub object::Error);

impl fmt::Debug for ObjectCrateErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for ObjectCrateErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl core::error::Error for ObjectCrateErrorWrapper {}
