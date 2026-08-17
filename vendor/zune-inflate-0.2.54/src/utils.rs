use core::cell::Cell;

/// make_decode_table_entry() creates a decode table entry for the given symbol
/// by combining the static part 'decode_results[sym]' with the dynamic part
/// 'len', which is the remaining codeword length (the codeword length for main
/// table entries, or the codeword length minus TABLEBITS for subtable entries).
///
/// In all cases, we add 'len' to each of the two low-order bytes to create the
/// appropriately-formatted decode table entry.  See the definitions of the
/// *_decode_results[] arrays below, where the entry format is described.
pub(crate) fn make_decode_table_entry(decode_results: &[u32], sym: usize, len: u32) -> u32
{
    decode_results[sym] + (len << 8) + len
}

/// A safe version of src.copy_within that helps me because I tend to always
/// confuse the arguments
pub fn fixed_copy_within<const SIZE: usize>(dest: &mut [u8], src_offset: usize, dest_offset: usize)
{
    // for debug builds ensure we don't go out of bounds
    debug_assert!(
        dest_offset + SIZE <= dest.len(),
        "[dst]: End position {} out of range for slice of length {}",
        dest_offset + SIZE,
        dest.len()
    );

    dest.copy_within(src_offset..src_offset + SIZE, dest_offset);
}

#[inline(always)]
pub fn copy_rep_matches(dest: &mut [u8], offset: usize, dest_offset: usize, length: usize)
{
    // This is a slightly complicated rep match copier that has
    // no bounds check.

    // The only invariant we need to uphold is dest[dest_offset] should
    // copy from dest[offset]
    // i.e in the first iteration, the first entry in the window will point
    // to dest[offset] and the
    // last entry will point to dest[dest_offset]
    // it's easy to prove dest[offset] since we take our slice
    // from offset.
    // but proving dest[dest_offset] is trickier
    // If we were at offset, to get to dest_offset, we could
    // 1. Get difference between dest_offset and offset
    // 2. Add that difference to offset.
    //

    let diff = dest_offset - offset + 1;

    // note
    for window in Cell::from_mut(&mut dest[offset..dest_offset + length + 2])
        .as_slice_of_cells()
        .windows(diff)
    {
        window.last().unwrap().set(window[0].get());
    }
}

/// Return the minimum of two usizes in a const context
#[rustfmt::skip]
pub const fn const_min_usize(a: usize, b: usize) -> usize
{
    if a < b { a } else { b }
}

/// Calculate the adler hash of a piece of data.
#[inline(never)]
#[cfg(feature = "zlib")]
pub fn calc_adler_hash(data: &[u8]) -> u32
{
    use simd_adler32::Adler32;
    let mut hasher = Adler32::new();

    hasher.write(data);

    hasher.finish()
}
