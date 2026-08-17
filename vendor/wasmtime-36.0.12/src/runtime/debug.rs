use crate::prelude::*;
use core::mem::size_of;
use object::elf::*;
use object::endian::{BigEndian, Endian, Endianness, LittleEndian};
use object::read::elf::{FileHeader, SectionHeader};
use object::{
    File, NativeEndian as NE, Object, ObjectSection, ObjectSymbol, RelocationEncoding,
    RelocationKind, RelocationTarget, U64Bytes,
};
use wasmtime_environ::obj;

pub(crate) fn create_gdbjit_image(
    mut bytes: Vec<u8>,
    code_region: (*const u8, usize),
) -> Result<Vec<u8>, Error> {
    let e = ensure_supported_elf_format(&bytes)?;

    // patch relocs
    relocate_dwarf_sections(&mut bytes, code_region)?;

    // elf is still missing details...
    match e {
        Endianness::Little => {
            convert_object_elf_to_loadable_file::<LittleEndian>(&mut bytes, code_region)
        }
        Endianness::Big => {
            convert_object_elf_to_loadable_file::<BigEndian>(&mut bytes, code_region)
        }
    }

    Ok(bytes)
}

fn relocate_dwarf_sections(bytes: &mut [u8], code_region: (*const u8, usize)) -> Result<(), Error> {
    let mut relocations = Vec::new();
    let obj = File::parse(&bytes[..]).map_err(obj::ObjectCrateErrorWrapper)?;
    for section in obj.sections() {
        let section_start = match section.file_range() {
            Some((start, _)) => start,
            None => continue,
        };
        for (off, r) in section.relocations() {
            if r.kind() != RelocationKind::Absolute
                || r.encoding() != RelocationEncoding::Generic
                || r.size() != 64
            {
                continue;
            }

            let sym = match r.target() {
                RelocationTarget::Symbol(index) => match obj.symbol_by_index(index) {
                    Ok(sym) => sym,
                    Err(_) => continue,
                },
                _ => continue,
            };
            relocations.push((
                section_start + off,
                (code_region.0 as u64)
                    .wrapping_add(sym.address())
                    .wrapping_add(r.addend() as u64),
            ));
        }
    }

    for (offset, value) in relocations {
        let (loc, _) = offset
            .try_into()
            .ok()
            .and_then(|offset| object::from_bytes_mut::<U64Bytes<NE>>(&mut bytes[offset..]).ok())
            .ok_or_else(|| anyhow!("invalid dwarf relocations"))?;
        loc.set(NE, value);
    }
    Ok(())
}

fn ensure_supported_elf_format(bytes: &[u8]) -> Result<Endianness, Error> {
    use object::elf::*;
    use object::read::elf::*;

    let kind = match object::FileKind::parse(bytes) {
        Ok(file) => file,
        Err(err) => {
            bail!("Failed to parse file: {}", err);
        }
    };
    let header = match kind {
        object::FileKind::Elf64 => match object::elf::FileHeader64::<Endianness>::parse(bytes) {
            Ok(header) => header,
            Err(err) => {
                bail!("Unsupported ELF file: {}", err);
            }
        },
        _ => {
            bail!("only 64-bit ELF files currently supported")
        }
    };
    let e = header.endian().unwrap();

    match header.e_machine.get(e) {
        EM_AARCH64 => (),
        EM_X86_64 => (),
        EM_S390 => (),
        EM_RISCV => (),
        machine => {
            bail!("Unsupported ELF target machine: {:x}", machine);
        }
    }
    ensure!(
        header.e_phoff.get(e) == 0 && header.e_phnum.get(e) == 0,
        "program header table is empty"
    );
    let e_shentsize = header.e_shentsize.get(e);
    let req_shentsize = match e {
        Endianness::Little => size_of::<SectionHeader64<LittleEndian>>(),
        Endianness::Big => size_of::<SectionHeader64<BigEndian>>(),
    };
    ensure!(e_shentsize as usize == req_shentsize, "size of sh");
    Ok(e)
}

fn convert_object_elf_to_loadable_file<E: Endian>(
    bytes: &mut Vec<u8>,
    code_region: (*const u8, usize),
) {
    let e = E::default();

    let header = FileHeader64::<E>::parse(&bytes[..]).unwrap();
    let sections = header.sections(e, &bytes[..]).unwrap();
    let text_range = match sections.section_by_name(e, b".text") {
        Some((i, text)) => {
            let range = text.file_range(e);
            let e_shoff = usize::try_from(header.e_shoff.get(e)).unwrap();
            let off = e_shoff + i.0 * header.e_shentsize.get(e) as usize;

            let section: &mut SectionHeader64<E> =
                object::from_bytes_mut(&mut bytes[off..]).unwrap().0;
            // Patch vaddr, and save file location and its size.
            section.sh_addr.set(e, code_region.0 as u64);
            range
        }
        None => None,
    };

    // LLDB wants segment with virtual address set, placing them at the end of ELF.
    let ph_off = bytes.len();
    let e_phentsize = size_of::<ProgramHeader64<E>>();
    let e_phnum = 1;
    bytes.resize(ph_off + e_phentsize * e_phnum, 0);
    if let Some((sh_offset, sh_size)) = text_range {
        let (v_offset, size) = code_region;
        let program: &mut ProgramHeader64<E> =
            object::from_bytes_mut(&mut bytes[ph_off..]).unwrap().0;
        program.p_type.set(e, PT_LOAD);
        program.p_offset.set(e, sh_offset);
        program.p_vaddr.set(e, v_offset as u64);
        program.p_paddr.set(e, v_offset as u64);
        program.p_filesz.set(e, sh_size);
        program.p_memsz.set(e, size as u64);
    } else {
        unreachable!();
    }

    // It is somewhat loadable ELF file at this moment.
    let header: &mut FileHeader64<E> = object::from_bytes_mut(bytes).unwrap().0;
    header.e_type.set(e, ET_DYN);
    header.e_phoff.set(e, ph_off as u64);
    header
        .e_phentsize
        .set(e, u16::try_from(e_phentsize).unwrap());
    header.e_phnum.set(e, u16::try_from(e_phnum).unwrap());
}
