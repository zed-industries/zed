use crate::obj::ELF_WASMTIME_STACK_MAP;
use crate::prelude::*;
use cranelift_bitset::CompoundBitSet;
use object::write::{Object, StandardSegment};
use object::{LittleEndian, SectionKind, U32Bytes};

/// Builder for the `ELF_WASMTIME_STACK_MAP` section in compiled executables.
///
/// This format is parsed by `crate::stack_map`.
///
/// The current layout of the format is:
///
/// ```text
/// ┌─────────────────────┬───── 0x00 (relative, not necessarily aligned)
/// │ count: 4-byte LE    │
/// ├─────────────────────┼───── 0x04
/// │ pc1: 4-byte LE      │
/// │ pc2: 4-byte LE      │
/// │ ...                 │
/// │ pcN: 4-byte LE      │
/// ├─────────────────────┼───── 0x04 + 4 * count
/// │ offset1: 4-byte LE  │
/// │ offset1: 4-byte LE  │
/// │ ...                 │
/// │ offsetN: 4-byte LE  │
/// ├─────────────────────┼───── 0x04 + 8 * count
/// │ data[0]: 4-byte LE  │
/// │ data[1]: 4-byte LE  │
/// │ ...                 │
/// │ data[M]: 4-byte LE  │
/// └─────────────────────┴───── 0x04 + 8 * count + 4 * M
/// ```
///
/// Here `count` is the size of the `pcN` and `offsetN` arrays. The two arrays
/// are the same size and have corresponding entries in one another. When
/// looking up a stack map for a particular program counter:
///
/// * A binary search is performed on the `pcN` array.
/// * The corresponding `offsetM` value is looked up once the `pcM` entry,
///   matching the lookup pc, is found.
/// * The `offsetM` value is used to access `data[offsetM]` which is an array of
///   4-byte entries located after the `offset*` array. This stack map is then
///   encoded as below.
///
/// This encoding scheme is chosen so parsing this data structure effectively
/// isn't required. It's usable at-rest from a compiled artifact in a section of
/// an executable. Notably having offsets into the data array means that a stack
/// map is just a slice into the data array, and the entire data structure can
/// be "parsed" by reading `count` and otherwise just making sure various
/// offsets are in-bounds.
///
/// A stack map located at `data[offsetM]` is encoded as:
///
/// ```text
/// ┌───────────────────────────────────────────────────────┐
/// │ data[offsetM + 0]: frame_size: 4-byte LE              │
/// ├───────────────────────────────────────────────────────┤
/// │ data[offsetM + 1]: count: 4-byte LE                   │
/// ├───────────────────────────────────────────────────────┤
/// │ data[offsetM + 2 + 0]: bitmap: 4-byte LE              │
/// │ data[offsetM + 2 + 1]: bitmap: 4-byte LE              │
/// │ ...                                                   │
/// │ data[offsetM + 2 + count - 1]: bitmap: 4-byte LE      │
/// └───────────────────────────────────────────────────────┘
/// ```
///
/// Here `frame_size` and `count` are always greater than 0. Entries in the bit
/// map represent `stack_slot / 4` so must be multiplied by 4 to get the actual
/// stack offset entry. This is because all stack slots are aligned at 4 bytes
/// so by dividing them all by 4 we're able to compress the bit map that much
/// more.
#[derive(Default)]
pub struct StackMapSection {
    pcs: Vec<U32Bytes<LittleEndian>>,
    pointers_to_stack_map: Vec<U32Bytes<LittleEndian>>,
    stack_map_data: Vec<U32Bytes<LittleEndian>>,
    last_offset: u32,
}

impl StackMapSection {
    /// Appends stack map information for `code_offset` which has the specified
    /// `frame_size` and `frame_offsets` are the active GC references.
    pub fn push(
        &mut self,
        code_offset: u64,
        frame_size: u32,
        frame_offsets: impl ExactSizeIterator<Item = u32>,
    ) {
        // NB: for now this only supports <=4GB text sections in object files.
        // Alternative schemes will need to be created for >32-bit offsets to
        // avoid making this section overly large.
        let code_offset = u32::try_from(code_offset).unwrap();

        // Sanity-check to ensure that functions are pushed in-order, otherwise
        // the `pcs` array won't be sorted which is our goal.
        assert!(code_offset >= self.last_offset);
        self.last_offset = code_offset;

        // Skip encoding information for this code offset if there's not
        // actually anything in the stack map.
        if frame_offsets.len() == 0 {
            return;
        }

        // Record parallel entries in `pcs`/`pointers_to_stack_map`.
        self.pcs.push(U32Bytes::new(LittleEndian, code_offset));
        self.pointers_to_stack_map.push(U32Bytes::new(
            LittleEndian,
            u32::try_from(self.stack_map_data.len()).unwrap(),
        ));

        // The frame data starts with the frame size and is then followed by
        // `offsets` represented as a bit set.
        self.stack_map_data
            .push(U32Bytes::new(LittleEndian, frame_size));

        let mut bits = CompoundBitSet::<u32>::default();
        for offset in frame_offsets {
            assert!(offset % 4 == 0);
            bits.insert((offset / 4) as usize);
        }
        let count = bits.iter_scalars().count();
        self.stack_map_data
            .push(U32Bytes::new(LittleEndian, count as u32));
        for scalar in bits.iter_scalars() {
            self.stack_map_data
                .push(U32Bytes::new(LittleEndian, scalar.0));
        }
    }

    /// Finishes encoding this section into the `Object` provided.
    pub fn append_to(self, obj: &mut Object) {
        // Don't append anything for this section if there weren't any actual
        // stack maps present, no need to waste space!
        if self.pcs.is_empty() {
            return;
        }
        let section = obj.add_section(
            obj.segment_name(StandardSegment::Data).to_vec(),
            ELF_WASMTIME_STACK_MAP.as_bytes().to_vec(),
            SectionKind::ReadOnlyData,
        );

        // NB: this matches the encoding expected by `lookup` in the
        // `crate::stack_maps` module.
        let amt = u32::try_from(self.pcs.len()).unwrap();
        obj.append_section_data(section, &amt.to_le_bytes(), 1);
        obj.append_section_data(section, object::bytes_of_slice(&self.pcs), 1);
        obj.append_section_data(
            section,
            object::bytes_of_slice(&self.pointers_to_stack_map),
            1,
        );
        obj.append_section_data(section, object::bytes_of_slice(&self.stack_map_data), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack_map::StackMap;
    use object::{Object, ObjectSection};

    fn roundtrip(maps: &[(u64, u32, &[u32])]) {
        let mut section = StackMapSection::default();
        for (pc, frame, offsets) in maps {
            println!("append {pc}");
            section.push(*pc, *frame, offsets.iter().copied());
        }
        let mut object = object::write::Object::new(
            object::BinaryFormat::Elf,
            object::Architecture::X86_64,
            object::Endianness::Little,
        );
        section.append_to(&mut object);
        let elf = object.write().unwrap();

        let image = object::File::parse(&elf[..]).unwrap();
        let data = image
            .sections()
            .find(|s| s.name().ok() == Some(ELF_WASMTIME_STACK_MAP))
            .unwrap()
            .data()
            .unwrap();

        for (pc, frame, offsets) in maps {
            println!("lookup {pc}");
            let map = match StackMap::lookup(*pc as u32, data) {
                Some(map) => map,
                None => {
                    assert!(offsets.is_empty());
                    continue;
                }
            };
            assert_eq!(map.frame_size(), *frame);

            let map_offsets = map.offsets().collect::<Vec<_>>();
            assert_eq!(map_offsets, *offsets);
        }

        let mut expected = maps.iter();
        'outer: for (pc, map) in StackMap::iter(data).unwrap() {
            while let Some((expected_pc, expected_frame, expected_offsets)) = expected.next() {
                if expected_offsets.is_empty() {
                    continue;
                }
                assert_eq!(*expected_pc, u64::from(pc));
                assert_eq!(*expected_frame, map.frame_size());
                let offsets = map.offsets().collect::<Vec<_>>();
                assert_eq!(offsets, *expected_offsets);
                continue 'outer;
            }
            panic!("didn't find {pc:#x} in expected list");
        }
        assert!(expected.next().is_none());
    }

    #[test]
    fn roundtrip_many() {
        roundtrip(&[(0, 4, &[0])]);
        roundtrip(&[
            (0, 4, &[0]),
            (4, 200, &[0, 4, 20, 180]),
            (200, 20, &[12]),
            (600, 0, &[]),
            (800, 20, &[0, 4, 8, 12, 16]),
            (1200, 2000, &[1800, 1804, 1808, 1900]),
        ]);
    }
}
