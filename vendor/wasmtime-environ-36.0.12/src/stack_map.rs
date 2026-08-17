use cranelift_bitset::ScalarBitSet;
use object::{Bytes, LittleEndian, U32Bytes};

struct StackMapSection<'a> {
    pcs: &'a [U32Bytes<LittleEndian>],
    pointers_to_stack_map: &'a [U32Bytes<LittleEndian>],
    stack_map_data: &'a [U32Bytes<LittleEndian>],
}

impl<'a> StackMapSection<'a> {
    fn parse(section: &'a [u8]) -> Option<StackMapSection<'a>> {
        let mut section = Bytes(section);
        // NB: this matches the encoding written by `append_to` in the
        // `compile::stack_map` module.
        let pc_count = section.read::<U32Bytes<LittleEndian>>().ok()?;
        let pc_count = usize::try_from(pc_count.get(LittleEndian)).ok()?;
        let (pcs, section) =
            object::slice_from_bytes::<U32Bytes<LittleEndian>>(section.0, pc_count).ok()?;
        let (pointers_to_stack_map, section) =
            object::slice_from_bytes::<U32Bytes<LittleEndian>>(section, pc_count).ok()?;
        let stack_map_data =
            object::slice_from_all_bytes::<U32Bytes<LittleEndian>>(section).ok()?;
        Some(StackMapSection {
            pcs,
            pointers_to_stack_map,
            stack_map_data,
        })
    }

    fn lookup(&self, pc: u32) -> Option<StackMap<'a>> {
        let pc_index = self
            .pcs
            .binary_search_by_key(&pc, |v| v.get(LittleEndian))
            .ok()?;
        self.get(pc_index)
    }

    fn into_iter(self) -> impl Iterator<Item = (u32, StackMap<'a>)> + 'a {
        self.pcs
            .iter()
            .enumerate()
            .map(move |(i, pc)| (pc.get(LittleEndian), self.get(i).unwrap()))
    }

    /// Returns the stack map corresponding to the `i`th pc.
    fn get(&self, i: usize) -> Option<StackMap<'a>> {
        let pointer_to_stack_map = self.pointers_to_stack_map[i].get(LittleEndian) as usize;
        let data = self.stack_map_data.get(pointer_to_stack_map..)?;

        let (frame_size, data) = data.split_first()?;
        let (count, data) = data.split_first()?;
        let data = data.get(..count.get(LittleEndian) as usize)?;

        Some(StackMap {
            frame_size: frame_size.get(LittleEndian),
            data,
        })
    }
}

/// A map for determining where live GC references live in a stack frame.
///
/// Note that this is currently primarily documented as cranelift's
/// `binemit::StackMap`, so for detailed documentation about this please read
/// the docs over there.
pub struct StackMap<'a> {
    frame_size: u32,
    data: &'a [U32Bytes<LittleEndian>],
}

impl<'a> StackMap<'a> {
    /// Looks up a stack map for `pc` within the `section` provided.
    ///
    /// The `section` should be produced by `StackMapSection` in the
    /// `compile::stack_map` module. The `pc` should be relative to the start
    /// of the `.text` section in the final executable.
    pub fn lookup(pc: u32, section: &'a [u8]) -> Option<StackMap<'a>> {
        StackMapSection::parse(section)?.lookup(pc)
    }

    /// Iterate over the stack maps contained in the given stack map section.
    ///
    /// This function takes a `section` as its first argument which must have
    /// been created with `StackMapSection` builder. This is intended to be the
    /// raw `ELF_WASMTIME_STACK_MAP` section from the compilation artifact.
    ///
    /// The yielded offsets are relative to the start of the text section for
    /// this map's code object.
    pub fn iter(section: &'a [u8]) -> Option<impl Iterator<Item = (u32, StackMap<'a>)> + 'a> {
        Some(StackMapSection::parse(section)?.into_iter())
    }

    /// Returns the byte size of this stack map's frame.
    pub fn frame_size(&self) -> u32 {
        self.frame_size
    }

    /// Given a frame pointer, get the stack pointer.
    ///
    /// # Safety
    ///
    /// The `fp` must be the frame pointer at the code offset that this stack
    /// map is associated with.
    pub unsafe fn sp(&self, fp: *mut usize) -> *mut usize {
        let frame_size = usize::try_from(self.frame_size).unwrap();
        unsafe { fp.byte_sub(frame_size) }
    }

    /// Given the stack pointer, get a reference to each live GC reference in
    /// the stack frame.
    ///
    /// # Safety
    ///
    /// The `sp` must be the stack pointer at the code offset that this stack
    /// map is associated with.
    pub unsafe fn live_gc_refs(&self, sp: *mut usize) -> impl Iterator<Item = *mut u32> + '_ {
        self.offsets().map(move |i| {
            log::trace!("Live GC ref in frame at frame offset {i:#x}");
            let i = usize::try_from(i).unwrap();
            let ptr_to_gc_ref = unsafe { sp.byte_add(i) };

            // Assert that the pointer is inside this stack map's frame.
            assert!({
                let delta = ptr_to_gc_ref as usize - sp as usize;
                let frame_size = usize::try_from(self.frame_size).unwrap();
                delta < frame_size
            });

            ptr_to_gc_ref.cast::<u32>()
        })
    }

    /// Returns the offsets that this stack map registers GC references at.
    pub fn offsets(&self) -> impl Iterator<Item = u32> + '_ {
        // Here `self.data` is a bit set of offsets divided by 4, so iterate
        // over all the bits in `self.data` and multiply their position by 4.
        let bit_positions = self.data.iter().enumerate().flat_map(|(i, word)| {
            ScalarBitSet(word.get(LittleEndian))
                .iter()
                .map(move |bit| (i as u32) * 32 + u32::from(bit))
        });

        bit_positions.map(|pos| pos * 4)
    }
}
