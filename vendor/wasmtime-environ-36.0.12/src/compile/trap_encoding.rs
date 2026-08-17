use crate::TrapInformation;
use crate::obj::ELF_WASMTIME_TRAPS;
use crate::prelude::*;
use object::write::{Object, StandardSegment};
use object::{LittleEndian, SectionKind, U32Bytes};
use std::ops::Range;

/// A helper structure to build the custom-encoded section of a wasmtime
/// compilation image which encodes trap information.
///
/// This structure is incrementally fed the results of compiling individual
/// functions and handles all the encoding internally, allowing usage of
/// `lookup_trap_code` below with the resulting section.
#[derive(Default)]
pub struct TrapEncodingBuilder {
    offsets: Vec<U32Bytes<LittleEndian>>,
    traps: Vec<u8>,
    last_offset: u32,
}

impl TrapEncodingBuilder {
    /// Appends trap information about a function into this section.
    ///
    /// This function is called to describe traps for the `func` range
    /// specified. The `func` offsets are specified relative to the text section
    /// itself, and the `traps` offsets are specified relative to the start of
    /// `func`.
    ///
    /// This is required to be called in-order for increasing ranges of `func`
    /// to ensure the final array is properly sorted. Additionally `traps` must
    /// be sorted.
    pub fn push(&mut self, func: Range<u64>, traps: &[TrapInformation]) {
        // NB: for now this only supports <=4GB text sections in object files.
        // Alternative schemes will need to be created for >32-bit offsets to
        // avoid making this section overly large.
        let func_start = u32::try_from(func.start).unwrap();
        let func_end = u32::try_from(func.end).unwrap();

        // Sanity-check to ensure that functions are pushed in-order, otherwise
        // the `offsets` array won't be sorted which is our goal.
        assert!(func_start >= self.last_offset);

        self.offsets.reserve(traps.len());
        self.traps.reserve(traps.len());
        for info in traps {
            let pos = func_start + info.code_offset;
            assert!(pos >= self.last_offset);
            self.offsets.push(U32Bytes::new(LittleEndian, pos));
            self.traps.push(info.trap_code as u8);
            self.last_offset = pos;
        }

        self.last_offset = func_end;
    }

    /// Encodes this section into the object provided.
    pub fn append_to(self, obj: &mut Object) {
        let section = obj.add_section(
            obj.segment_name(StandardSegment::Data).to_vec(),
            ELF_WASMTIME_TRAPS.as_bytes().to_vec(),
            SectionKind::ReadOnlyData,
        );

        // NB: this matches the encoding expected by `lookup` below.
        let amt = u32::try_from(self.traps.len()).unwrap();
        obj.append_section_data(section, &amt.to_le_bytes(), 1);
        obj.append_section_data(section, object::bytes_of_slice(&self.offsets), 1);
        obj.append_section_data(section, &self.traps, 1);
    }
}
