use crate::debug::Compilation;
pub use crate::debug::transform::transform_dwarf;
use cranelift_codegen::ir::Endianness;
use cranelift_codegen::isa::{
    TargetIsa,
    unwind::{CfaUnwindInfo, UnwindInfo},
};
use gimli::write::{Address, Dwarf, EndianVec, FrameTable, Result, Sections, Writer};
use gimli::{RunTimeEndian, SectionId};

pub struct DwarfSection {
    pub name: &'static str,
    pub body: Vec<u8>,
    pub relocs: Vec<DwarfSectionReloc>,
}

#[derive(Clone)]
pub struct DwarfSectionReloc {
    pub target: DwarfSectionRelocTarget,
    pub offset: u32,
    pub addend: i32,
    pub size: u8,
}

#[derive(Clone)]
pub enum DwarfSectionRelocTarget {
    Func(usize),
    Section(&'static str),
}

fn emit_dwarf_sections(
    isa: &dyn TargetIsa,
    mut dwarf: Dwarf,
    frames: Option<FrameTable>,
) -> anyhow::Result<Vec<DwarfSection>> {
    let endian = match isa.endianness() {
        Endianness::Little => RunTimeEndian::Little,
        Endianness::Big => RunTimeEndian::Big,
    };
    let writer = WriterRelocate {
        relocs: Vec::new(),
        writer: EndianVec::new(endian),
    };
    let mut sections = Sections::new(writer);
    dwarf.write(&mut sections)?;
    if let Some(frames) = frames {
        frames.write_debug_frame(&mut sections.debug_frame)?;
    }

    let mut result = Vec::new();
    sections.for_each_mut(|id, s| -> anyhow::Result<()> {
        let name = id.name();
        let body = s.writer.take();
        if body.is_empty() {
            return Ok(());
        }
        let mut relocs = vec![];
        ::std::mem::swap(&mut relocs, &mut s.relocs);
        result.push(DwarfSection { name, body, relocs });
        Ok(())
    })?;

    Ok(result)
}

#[derive(Clone)]
pub struct WriterRelocate {
    relocs: Vec<DwarfSectionReloc>,
    writer: EndianVec<RunTimeEndian>,
}

impl Writer for WriterRelocate {
    type Endian = RunTimeEndian;

    fn endian(&self) -> Self::Endian {
        self.writer.endian()
    }

    fn len(&self) -> usize {
        self.writer.len()
    }

    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer.write(bytes)
    }

    fn write_at(&mut self, offset: usize, bytes: &[u8]) -> Result<()> {
        self.writer.write_at(offset, bytes)
    }

    fn write_address(&mut self, address: Address, size: u8) -> Result<()> {
        match address {
            Address::Constant(val) => self.write_udata(val, size),
            Address::Symbol { symbol, addend } => {
                let offset = self.len() as u32;
                self.relocs.push(DwarfSectionReloc {
                    target: DwarfSectionRelocTarget::Func(symbol),
                    offset,
                    size,
                    addend: addend as i32,
                });
                self.write_udata(addend as u64, size)
            }
        }
    }

    fn write_offset(&mut self, val: usize, section: SectionId, size: u8) -> Result<()> {
        let offset = self.len() as u32;
        let target = DwarfSectionRelocTarget::Section(section.name());
        self.relocs.push(DwarfSectionReloc {
            target,
            offset,
            size,
            addend: val as i32,
        });
        self.write_udata(val as u64, size)
    }

    fn write_offset_at(
        &mut self,
        offset: usize,
        val: usize,
        section: SectionId,
        size: u8,
    ) -> Result<()> {
        let target = DwarfSectionRelocTarget::Section(section.name());
        self.relocs.push(DwarfSectionReloc {
            target,
            offset: offset as u32,
            size,
            addend: val as i32,
        });
        self.write_udata_at(offset, val as u64, size)
    }
}

fn create_frame_table(
    isa: &dyn TargetIsa,
    compilation: &mut Compilation<'_>,
) -> Option<FrameTable> {
    let mut table = FrameTable::default();

    let cie_id = table.add_cie(isa.create_systemv_cie()?);

    for (_, symbol, metadata) in compilation.functions() {
        // The CFA-based unwind info will either be natively present, or we
        // have generated it and placed into the "cfa_unwind_info" auxiliary
        // field. We shouldn't emit both, though, it'd be wasteful.
        let mut unwind_info: Option<&CfaUnwindInfo> = None;
        if let Some(UnwindInfo::SystemV(info)) = &metadata.unwind_info {
            debug_assert!(metadata.cfa_unwind_info.is_none());
            unwind_info = Some(info);
        } else if let Some(info) = &metadata.cfa_unwind_info {
            unwind_info = Some(info);
        }

        if let Some(info) = unwind_info {
            table.add_fde(cie_id, info.to_fde(Address::Symbol { symbol, addend: 0 }));
        }
    }

    Some(table)
}

pub fn emit_dwarf(
    isa: &dyn TargetIsa,
    compilation: &mut Compilation<'_>,
) -> anyhow::Result<Vec<DwarfSection>> {
    let dwarf = transform_dwarf(isa, compilation)?;
    let frame_table = create_frame_table(isa, compilation);
    let sections = emit_dwarf_sections(isa, dwarf, frame_table)?;
    Ok(sections)
}
