//! Data structures to provide transformation of the source

use object::{Bytes, LittleEndian, U32Bytes};
use serde_derive::{Deserialize, Serialize};

/// Single source location to generated address mapping.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InstructionAddressMap {
    /// Where in the source wasm binary this instruction comes from, specified
    /// in an offset of bytes from the front of the file.
    pub srcloc: FilePos,

    /// Offset from the start of the function's compiled code to where this
    /// instruction is located, or the region where it starts.
    pub code_offset: u32,
}

/// A position within an original source file,
///
/// This structure is used as a newtype wrapper around a 32-bit integer which
/// represents an offset within a file where a wasm instruction or function is
/// to be originally found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilePos(u32);

impl FilePos {
    /// Create a new file position with the given offset.
    pub fn new(pos: u32) -> FilePos {
        assert!(pos != u32::MAX);
        FilePos(pos)
    }

    /// Returns the offset that this offset was created with.
    ///
    /// Note that the `Default` implementation will return `None` here, whereas
    /// positions created with `FilePos::new` will return `Some`.
    pub fn file_offset(self) -> Option<u32> {
        if self.0 == u32::MAX {
            None
        } else {
            Some(self.0)
        }
    }
}

impl Default for FilePos {
    fn default() -> FilePos {
        FilePos(u32::MAX)
    }
}

/// Parse an `ELF_WASMTIME_ADDRMAP` section, returning the slice of code offsets
/// and the slice of associated file positions for each offset.
fn parse_address_map(
    section: &[u8],
) -> Option<(&[U32Bytes<LittleEndian>], &[U32Bytes<LittleEndian>])> {
    let mut section = Bytes(section);
    // NB: this matches the encoding written by `append_to` in the
    // `compile::address_map` module.
    let count = section.read::<U32Bytes<LittleEndian>>().ok()?;
    let count = usize::try_from(count.get(LittleEndian)).ok()?;
    let (offsets, section) =
        object::slice_from_bytes::<U32Bytes<LittleEndian>>(section.0, count).ok()?;
    let (positions, section) =
        object::slice_from_bytes::<U32Bytes<LittleEndian>>(section, count).ok()?;
    debug_assert!(section.is_empty());
    Some((offsets, positions))
}

/// Lookup an `offset` within an encoded address map section, returning the
/// original `FilePos` that corresponds to the offset, if found.
///
/// This function takes a `section` as its first argument which must have been
/// created with `AddressMapSection` above. This is intended to be the raw
/// `ELF_WASMTIME_ADDRMAP` section from the compilation artifact.
///
/// The `offset` provided is a relative offset from the start of the text
/// section of the pc that is being looked up. If `offset` is out of range or
/// doesn't correspond to anything in this file then `None` is returned.
pub fn lookup_file_pos(section: &[u8], offset: usize) -> Option<FilePos> {
    let (offsets, positions) = parse_address_map(section)?;

    // First perform a binary search on the `offsets` array. This is a sorted
    // array of offsets within the text section, which is conveniently what our
    // `offset` also is. Note that we are somewhat unlikely to find a precise
    // match on the element in the array, so we're largely interested in which
    // "bucket" the `offset` falls into.
    let offset = u32::try_from(offset).ok()?;
    let index = match offsets.binary_search_by_key(&offset, |v| v.get(LittleEndian)) {
        // Exact hit!
        Ok(i) => i,

        // This *would* be at the first slot in the array, so no
        // instructions cover `pc`.
        Err(0) => return None,

        // This would be at the `nth` slot, so we're at the `n-1`th slot.
        Err(n) => n - 1,
    };

    // Using the `index` we found of which bucket `offset` corresponds to we can
    // lookup the actual `FilePos` value in the `positions` array.
    let pos = positions.get(index)?;
    Some(FilePos(pos.get(LittleEndian)))
}

/// Iterate over the address map contained in the given address map section.
///
/// This function takes a `section` as its first argument which must have been
/// created with `AddressMapSection` above. This is intended to be the raw
/// `ELF_WASMTIME_ADDRMAP` section from the compilation artifact.
///
/// The yielded offsets are relative to the start of the text section for this
/// map's code object.
pub fn iterate_address_map<'a>(
    section: &'a [u8],
) -> Option<impl Iterator<Item = (u32, FilePos)> + 'a> {
    let (offsets, positions) = parse_address_map(section)?;

    Some(
        offsets
            .iter()
            .map(|o| o.get(LittleEndian))
            .zip(positions.iter().map(|pos| FilePos(pos.get(LittleEndian)))),
    )
}
