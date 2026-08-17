#![allow(clippy::too_many_arguments)]
// We are writing a very specific, well defined format, so it makes it easier to
// see exactly what is being written if we explicitly write out `\n` instead of
// hoping somebody notices the `writeln!` instead of `write!`.
#![allow(clippy::write_with_newline)]

mod alignment;
mod archive;
mod archive_writer;
mod coff;
mod coff_import_file;
mod mangler;
mod math_extras;
mod object_reader;

pub use archive::ArchiveKind;
pub use archive_writer::{NewArchiveMember, write_archive_to_stream};
pub use coff::MachineTypes;
pub use coff_import_file::{COFFShortExport, write_import_library};

pub type GetSymbolsFn =
    fn(buf: &[u8], f: &mut dyn FnMut(&[u8]) -> std::io::Result<()>) -> std::io::Result<bool>;
pub type Is64BitObjectFileFn = fn(buf: &[u8]) -> bool;
pub type IsECObjectFileFn = fn(buf: &[u8]) -> bool;
pub type IsAnyArm64CoffFn = fn(buf: &[u8]) -> bool;
pub type GetXCoffMemberAlignmentFn = fn(buf: &[u8]) -> u32;

/// Helper struct to query object file information from members.
pub struct ObjectReader {
    /// Iterates over the symbols in the object file.
    pub get_symbols: GetSymbolsFn,
    /// Returns true if the object file is 64-bit.
    /// Note that this should match LLVM's `SymbolicFile::is64Bit`, which
    /// considers all COFF files to be 32-bit.
    pub is_64_bit_object_file: Is64BitObjectFileFn,
    /// Returns true if the object file is an EC (that is, an Arm64EC or x64)
    /// object file
    pub is_ec_object_file: IsECObjectFileFn,
    /// Returns true if the object file is any Arm64 (Native Arm64, Arm64EC or
    /// Arm64X) COFF file.
    pub is_any_arm64_coff: IsAnyArm64CoffFn,
    /// Returns the member alignment of an XCoff object file.
    pub get_xcoff_member_alignment: GetXCoffMemberAlignmentFn,
}

/// Default implementation of [ObjectReader] that uses the `object` crate.
pub const DEFAULT_OBJECT_READER: ObjectReader = ObjectReader {
    get_symbols: object_reader::get_native_object_symbols,
    is_64_bit_object_file: object_reader::is_64_bit_symbolic_file,
    is_ec_object_file: object_reader::is_ec_object,
    is_any_arm64_coff: object_reader::is_any_arm64_coff,
    get_xcoff_member_alignment: object_reader::get_member_alignment,
};
