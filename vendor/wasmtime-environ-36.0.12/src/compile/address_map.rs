//! Data structures to provide transformation of the source

use crate::InstructionAddressMap;
use crate::obj::ELF_WASMTIME_ADDRMAP;
use crate::prelude::*;
use object::write::{Object, StandardSegment};
use object::{LittleEndian, SectionKind, U32Bytes};
use std::ops::Range;

/// Builder for the address map section of a wasmtime compilation image.
///
/// This builder is used to conveniently built the `ELF_WASMTIME_ADDRMAP`
/// section by compilers, and provides utilities to directly insert the results
/// into an `Object`.
#[derive(Default)]
pub struct AddressMapSection {
    offsets: Vec<U32Bytes<LittleEndian>>,
    positions: Vec<U32Bytes<LittleEndian>>,
    last_offset: u32,
}

impl AddressMapSection {
    /// Pushes a new set of instruction mapping information for a function added
    /// in the executable.
    ///
    /// The `func` argument here is the range of the function, relative to the
    /// start of the text section in the executable. The `instrs` provided are
    /// the descriptors for instructions in the function and their various
    /// mappings back to original source positions.
    ///
    /// This is required to be called for `func` values that are strictly
    /// increasing in addresses (e.g. as the object is built). Additionally the
    /// `instrs` map must be sorted based on code offset in the native text
    /// section.
    pub fn push(&mut self, func: Range<u64>, instrs: &[InstructionAddressMap]) {
        // NB: for now this only supports <=4GB text sections in object files.
        // Alternative schemes will need to be created for >32-bit offsets to
        // avoid making this section overly large.
        let func_start = u32::try_from(func.start).unwrap();
        let func_end = u32::try_from(func.end).unwrap();

        self.offsets.reserve(instrs.len());
        self.positions.reserve(instrs.len());
        let mut last_srcloc = None;
        for map in instrs {
            // Sanity-check to ensure that functions are pushed in-order, otherwise
            // the `offsets` array won't be sorted which is our goal.
            let pos = func_start + map.code_offset;
            assert!(pos >= self.last_offset);
            self.last_offset = pos;

            // Drop duplicate instruction mappings that match what was
            // previously pushed into the array since the representation used
            // here will naturally cover `pos` with the previous entry.
            let srcloc = map.srcloc.file_offset().unwrap_or(u32::MAX);
            if Some(srcloc) == last_srcloc {
                continue;
            }
            last_srcloc = Some(srcloc);

            self.offsets.push(U32Bytes::new(LittleEndian, pos));
            self.positions.push(U32Bytes::new(LittleEndian, srcloc));
        }
        self.last_offset = func_end;
    }

    /// Finishes encoding this section into the `Object` provided.
    pub fn append_to(self, obj: &mut Object) {
        let section = obj.add_section(
            obj.segment_name(StandardSegment::Data).to_vec(),
            ELF_WASMTIME_ADDRMAP.as_bytes().to_vec(),
            SectionKind::ReadOnlyData,
        );

        // NB: this matches the encoding expected by `lookup` below.
        let amt = u32::try_from(self.offsets.len()).unwrap();
        obj.append_section_data(section, &amt.to_le_bytes(), 1);
        obj.append_section_data(section, object::bytes_of_slice(&self.offsets), 1);
        obj.append_section_data(section, object::bytes_of_slice(&self.positions), 1);
    }
}
