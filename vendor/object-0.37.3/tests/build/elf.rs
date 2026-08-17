use object::{build, elf};

// Test that offset 0 is supported for SHT_NOBITS sections.
#[test]
fn test_nobits_offset() {
    let mut builder = build::elf::Builder::new(object::Endianness::Little, true);
    builder.header.e_type = elf::ET_EXEC;
    builder.header.e_phoff = 0x40;

    let section = builder.sections.add();
    section.name = b".shstrtab"[..].into();
    section.sh_type = elf::SHT_STRTAB;
    section.data = build::elf::SectionData::SectionString;

    let section = builder.sections.add();
    section.name = b".bss"[..].into();
    section.sh_type = elf::SHT_NOBITS;
    section.sh_flags = (elf::SHF_ALLOC | elf::SHF_WRITE) as u64;
    section.sh_addr = 0x1000;
    section.sh_offset = 0;
    section.sh_size = 0x1000;
    section.sh_addralign = 16;
    section.data = build::elf::SectionData::UninitializedData(0x1000);
    let section_id = section.id();

    let segment = builder.segments.add();
    segment.p_type = elf::PT_LOAD;
    segment.p_flags = elf::PF_R | elf::PF_W;
    segment.p_offset = 0x1000;
    segment.p_vaddr = 0x1000;
    segment.p_paddr = 0x1000;
    segment.p_filesz = 0;
    segment.p_memsz = 0x1000;
    segment.p_align = 16;
    segment.sections.push(section_id);

    let mut buf = Vec::new();
    builder.write(&mut buf).unwrap();
}

// Test that we can read and write a file with no dynamic string table.
#[test]
fn test_no_dynstr() {
    let mut builder = build::elf::Builder::new(object::Endianness::Little, true);
    builder.header.e_type = elf::ET_EXEC;
    builder.header.e_machine = elf::EM_X86_64;
    builder.header.e_phoff = 0x40;

    let section = builder.sections.add();
    section.name = b".shstrtab"[..].into();
    section.sh_type = elf::SHT_STRTAB;
    section.data = build::elf::SectionData::SectionString;

    let section = builder.sections.add();
    section.name = b".dynsym"[..].into();
    section.sh_type = elf::SHT_DYNSYM;
    section.sh_flags = elf::SHF_ALLOC as u64;
    section.sh_addralign = 8;
    section.data = build::elf::SectionData::DynamicSymbol;
    let dynsym_id = section.id();

    let section = builder.sections.add();
    section.name = b".rela.dyn"[..].into();
    section.sh_type = elf::SHT_RELA;
    section.sh_flags = elf::SHF_ALLOC as u64;
    section.sh_addralign = 8;
    section.data =
        build::elf::SectionData::DynamicRelocation(vec![build::elf::DynamicRelocation {
            r_offset: 0x1000,
            symbol: None,
            r_type: elf::R_X86_64_64,
            r_addend: 0x300,
        }]);
    let rela_id = section.id();

    builder.set_section_sizes();

    let segment = builder.segments.add();
    segment.p_type = elf::PT_LOAD;
    segment.p_flags = elf::PF_R;
    segment.p_filesz = 0x1000;
    segment.p_memsz = 0x1000;
    segment.p_align = 8;
    segment.append_section(builder.sections.get_mut(dynsym_id));
    segment.append_section(builder.sections.get_mut(rela_id));

    let mut buf = Vec::new();
    builder.write(&mut buf).unwrap();

    let builder = build::elf::Builder::read(&*buf).unwrap();
    assert_eq!(builder.sections.count(), 3);
    assert_eq!(builder.segments.count(), 1);
    for section in &builder.sections {
        match &section.data {
            build::elf::SectionData::DynamicSymbol => {
                assert_eq!(section.sh_offset, 0x1000);
            }
            build::elf::SectionData::DynamicRelocation(rela) => {
                assert_eq!(section.sh_offset, 0x1018);
                assert_eq!(rela.len(), 1);
            }
            _ => {}
        }
    }
}

#[test]
fn test_attribute() {
    let mut builder = build::elf::Builder::new(object::Endianness::Little, true);
    builder.header.e_type = elf::ET_EXEC;
    builder.header.e_machine = elf::EM_X86_64;
    builder.header.e_phoff = 0x40;

    let section = builder.sections.add();
    section.name = b".shstrtab"[..].into();
    section.sh_type = elf::SHT_STRTAB;
    section.data = build::elf::SectionData::SectionString;

    let attributes = build::elf::AttributesSection {
        subsections: vec![build::elf::AttributesSubsection {
            vendor: b"GNU"[..].into(),
            subsubsections: vec![
                (build::elf::AttributesSubsubsection {
                    tag: build::elf::AttributeTag::File,
                    data: b"123"[..].into(),
                }),
            ],
        }],
    };
    let section = builder.sections.add();
    section.name = b".gnu.attributes"[..].into();
    section.sh_type = elf::SHT_GNU_ATTRIBUTES;
    section.sh_addralign = 8;
    section.data = build::elf::SectionData::Attributes(attributes);

    let mut buf = Vec::new();
    builder.write(&mut buf).unwrap();

    let builder = build::elf::Builder::read(&*buf).unwrap();
    assert_eq!(builder.sections.count(), 2);
    for section in &builder.sections {
        if let build::elf::SectionData::Attributes(attributes) = &section.data {
            assert_eq!(attributes.subsections.len(), 1);
            assert_eq!(attributes.subsections[0].vendor.as_slice(), b"GNU");
            assert_eq!(attributes.subsections[0].subsubsections.len(), 1);
            assert_eq!(
                attributes.subsections[0].subsubsections[0].tag,
                build::elf::AttributeTag::File
            );
            assert_eq!(
                attributes.subsections[0].subsubsections[0].data.as_slice(),
                b"123"
            );
        }
    }
}

#[test]
fn test_dynsym() {
    let mut builder = build::elf::Builder::new(object::Endianness::Little, true);
    builder.header.e_type = elf::ET_EXEC;
    builder.header.e_machine = elf::EM_X86_64;
    builder.header.e_phoff = 0x40;

    let section = builder.sections.add();
    section.name = b".shstrtab"[..].into();
    section.sh_type = elf::SHT_STRTAB;
    section.data = build::elf::SectionData::SectionString;

    let section = builder.sections.add();
    section.name = b".text"[..].into();
    section.sh_type = elf::SHT_PROGBITS;
    section.sh_flags = (elf::SHF_ALLOC | elf::SHF_EXECINSTR) as u64;
    section.sh_addralign = 16;
    section.data = build::elf::SectionData::Data(vec![0xcc; 100].into());
    let text_id = section.id();

    let section = builder.sections.add();
    section.name = b".dynsym"[..].into();
    section.sh_type = elf::SHT_DYNSYM;
    section.sh_flags = elf::SHF_ALLOC as u64;
    section.sh_addralign = 8;
    section.data = build::elf::SectionData::DynamicSymbol;
    let dynsym_id = section.id();

    let section = builder.sections.add();
    section.name = b".dynstr"[..].into();
    section.sh_type = elf::SHT_STRTAB;
    section.sh_flags = elf::SHF_ALLOC as u64;
    section.sh_addralign = 1;
    section.data = build::elf::SectionData::DynamicString;
    let dynstr_id = section.id();

    let section = builder.sections.add();
    section.name = b".gnu.hash"[..].into();
    section.sh_type = elf::SHT_GNU_HASH;
    section.sh_flags = elf::SHF_ALLOC as u64;
    section.sh_addralign = 8;
    section.data = build::elf::SectionData::GnuHash;
    let gnu_hash_id = section.id();
    builder.gnu_hash_bloom_shift = 1;
    builder.gnu_hash_bloom_count = 1;
    builder.gnu_hash_bucket_count = 1;

    let symbol = builder.dynamic_symbols.add();
    symbol.name = b"global"[..].into();
    symbol.set_st_info(elf::STB_GLOBAL, elf::STT_FUNC);
    symbol.section = Some(text_id);

    let symbol = builder.dynamic_symbols.add();
    symbol.name = b"undefined"[..].into();
    symbol.set_st_info(elf::STB_GLOBAL, elf::STT_NOTYPE);

    let symbol = builder.dynamic_symbols.add();
    symbol.name = b"local"[..].into();
    symbol.set_st_info(elf::STB_LOCAL, elf::STT_FUNC);
    symbol.section = Some(text_id);

    builder.set_section_sizes();

    let segment = builder.segments.add();
    segment.p_type = elf::PT_LOAD;
    segment.p_flags = elf::PF_R;
    segment.p_filesz = 0x1000;
    segment.p_memsz = 0x1000;
    segment.p_align = 8;
    segment.append_section(builder.sections.get_mut(text_id));
    segment.append_section(builder.sections.get_mut(dynsym_id));
    segment.append_section(builder.sections.get_mut(dynstr_id));
    segment.append_section(builder.sections.get_mut(gnu_hash_id));

    let mut buf = Vec::new();
    builder.write(&mut buf).unwrap();

    let builder = build::elf::Builder::read(&*buf).unwrap();
    assert_eq!(builder.sections.count(), 5);
    assert_eq!(builder.dynamic_symbols.count(), 3);
    // Check that the dynamic symbol table sorting handles
    // local and undefined symbols correctly.
    assert_eq!(
        builder
            .dynamic_symbols
            .iter()
            .map(|s| s.name.as_slice())
            .collect::<Vec<_>>(),
        vec![&b"local"[..], &b"undefined"[..], &b"global"[..]]
    );
    for section in &builder.sections {
        if let build::elf::SectionData::DynamicSymbol = &section.data {
            // Check that sh_info includes the number of local symbols.
            assert_eq!(section.sh_info, 2);
        }
    }
}
