// Derived from object's round_trip.rs:
// https://github.com/gimli-rs/object/blob/0.35.0/tests/round_trip/mod.rs

use ar_archive_writer::ArchiveKind;
use object::{
    Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationKind, SymbolFlags,
    SymbolKind, SymbolScope,
};
use object::{RelocationFlags, SubArchitecture, read, write};
use pretty_assertions::assert_eq;

mod common;

fn round_trip_and_diff(
    test_name: &str,
    object: write::Object<'_>,
    archive_kind: ArchiveKind,
    trim_output_bytes: usize,
) {
    let tmpdir = common::create_tmp_dir(test_name);
    let input_bytes = object.write().unwrap();
    let is_ec = object.sub_architecture() == Some(SubArchitecture::Arm64EC);

    // Create a new archive using ar_archive_writer.
    let output_archive_bytes = common::create_archive_with_ar_archive_writer(
        &tmpdir,
        archive_kind,
        [("input.o", input_bytes.as_slice())],
        false,
        is_ec,
    );

    // Use llvm-ar to create the archive and diff with ar_archive_writer.
    let output_llvm_ar_bytes = common::create_archive_with_llvm_ar(
        &tmpdir,
        archive_kind,
        [("input.o", input_bytes.as_slice())],
        false,
        is_ec,
    );
    assert_eq!(
        output_archive_bytes, output_llvm_ar_bytes,
        "Comparing ar_archive_writer to llvm-ar. Test case: build {:?} for {:?}",
        archive_kind, object
    );

    // Read the archive and member using object and diff with original data.
    {
        let output_archive =
            read::archive::ArchiveFile::parse(output_archive_bytes.as_slice()).unwrap();
        let mut members = output_archive.members();
        let output_member = members.next().unwrap().unwrap();
        if archive_kind != ArchiveKind::Coff {
            assert_eq!(output_member.name(), b"input.o");
        }
        let output_bytes = output_member.data(output_archive_bytes.as_slice()).unwrap();

        // Apply fixup if required.
        let output_bytes = &output_bytes[..output_bytes.len() - trim_output_bytes];

        assert_eq!(
            &input_bytes, output_bytes,
            "Comparing object after round-trip. Test case: build {:?} for {:?}",
            archive_kind, object
        );
    }
}

#[test]
fn coff_any() {
    for (arch, sub_arch) in [
        (Architecture::Aarch64, None),
        (Architecture::Aarch64, Some(SubArchitecture::Arm64EC)),
        (Architecture::Arm, None),
        (Architecture::I386, None),
        (Architecture::X86_64, None),
    ]
    .iter()
    .copied()
    {
        for archive_kind in [ArchiveKind::Gnu, ArchiveKind::Coff] {
            let mut object = write::Object::new(BinaryFormat::Coff, arch, Endianness::Little);
            object.set_sub_architecture(sub_arch);

            object.add_file_symbol(b"file.c".to_vec());

            let text = object.section_id(write::StandardSection::Text);
            object.append_section_data(text, &[1; 30], 4);

            let func1_offset = object.append_section_data(text, &[1; 30], 4);
            assert_eq!(func1_offset, 32);
            let func1_symbol = object.add_symbol(write::Symbol {
                name: b"func1".to_vec(),
                value: func1_offset,
                size: 32,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                section: write::SymbolSection::Section(text),
                flags: SymbolFlags::None,
            });
            let func2_offset = object.append_section_data(text, &[1; 30], 4);
            assert_eq!(func2_offset, 64);
            object.add_symbol(write::Symbol {
                name: b"func2_long".to_vec(),
                value: func2_offset,
                size: 32,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                section: write::SymbolSection::Section(text),
                flags: SymbolFlags::None,
            });
            object
                .add_relocation(
                    text,
                    write::Relocation {
                        offset: 8,
                        symbol: func1_symbol,
                        addend: 0,
                        flags: RelocationFlags::Generic {
                            kind: RelocationKind::Absolute,
                            encoding: RelocationEncoding::Generic,
                            size: arch.address_size().unwrap().bytes() * 8,
                        },
                    },
                )
                .unwrap();

            round_trip_and_diff("coff_any", object, archive_kind, 0);
        }
    }
}
#[test]
fn elf_x86_64() {
    let mut object =
        write::Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);

    object.add_file_symbol(b"file.c".to_vec());

    let text = object.section_id(write::StandardSection::Text);
    object.append_section_data(text, &[1; 30], 4);

    let func1_offset = object.append_section_data(text, &[1; 30], 4);
    assert_eq!(func1_offset, 32);
    let func1_symbol = object.add_symbol(write::Symbol {
        name: b"func1".to_vec(),
        value: func1_offset,
        size: 32,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: write::SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    object
        .add_relocation(
            text,
            write::Relocation {
                offset: 8,
                symbol: func1_symbol,
                addend: 0,
                flags: RelocationFlags::Generic {
                    kind: RelocationKind::Absolute,
                    encoding: RelocationEncoding::Generic,
                    size: 64,
                },
            },
        )
        .unwrap();

    round_trip_and_diff("elf_x86_64", object, ArchiveKind::Gnu, 0);
}

#[test]
fn elf_any() {
    for (arch, endian, archive_kinds) in [
        (
            Architecture::Aarch64,
            Endianness::Little,
            &[ArchiveKind::Gnu][..],
        ),
        (
            Architecture::Aarch64_Ilp32,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (Architecture::Arm, Endianness::Little, &[ArchiveKind::Gnu]),
        (Architecture::Avr, Endianness::Little, &[ArchiveKind::Gnu]),
        (Architecture::Bpf, Endianness::Little, &[ArchiveKind::Gnu]),
        (Architecture::Csky, Endianness::Little, &[ArchiveKind::Gnu]),
        (Architecture::I386, Endianness::Little, &[ArchiveKind::Gnu]),
        (
            Architecture::X86_64,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (
            Architecture::X86_64_X32,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (
            Architecture::Hexagon,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (
            Architecture::LoongArch64,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (Architecture::Mips, Endianness::Little, &[ArchiveKind::Gnu]),
        (
            Architecture::Mips64,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (
            Architecture::Msp430,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (Architecture::PowerPc, Endianness::Big, &[ArchiveKind::Gnu]),
        (
            Architecture::PowerPc64,
            Endianness::Big,
            &[ArchiveKind::Gnu, ArchiveKind::AixBig],
        ),
        (
            Architecture::Riscv32,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (
            Architecture::Riscv64,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
        (Architecture::S390x, Endianness::Big, &[ArchiveKind::Gnu]),
        (Architecture::Sbf, Endianness::Little, &[ArchiveKind::Gnu]),
        (Architecture::Sparc64, Endianness::Big, &[ArchiveKind::Gnu]),
        (
            Architecture::Xtensa,
            Endianness::Little,
            &[ArchiveKind::Gnu],
        ),
    ]
    .iter()
    .copied()
    {
        for archive_kind in archive_kinds {
            let mut object = write::Object::new(BinaryFormat::Elf, arch, endian);

            let section = object.section_id(write::StandardSection::Data);
            object.append_section_data(section, &[1; 30], 4);
            let symbol = object.section_symbol(section);

            object
                .add_relocation(
                    section,
                    write::Relocation {
                        offset: 8,
                        symbol,
                        addend: 0,
                        flags: RelocationFlags::Generic {
                            kind: RelocationKind::Absolute,
                            encoding: RelocationEncoding::Generic,
                            size: 32,
                        },
                    },
                )
                .unwrap();
            if arch.address_size().unwrap().bytes() >= 8 {
                object
                    .add_relocation(
                        section,
                        write::Relocation {
                            offset: 16,
                            symbol,
                            addend: 0,
                            flags: RelocationFlags::Generic {
                                kind: RelocationKind::Absolute,
                                encoding: RelocationEncoding::Generic,
                                size: 64,
                            },
                        },
                    )
                    .unwrap();
            }

            round_trip_and_diff("elf_any", object, *archive_kind, 0);
        }
    }
}

#[test]
fn macho_x86_64() {
    let mut object = write::Object::new(
        BinaryFormat::MachO,
        Architecture::X86_64,
        Endianness::Little,
    );

    object.add_file_symbol(b"file.c".to_vec());

    let text = object.section_id(write::StandardSection::Text);
    object.append_section_data(text, &[1; 30], 4);

    let func1_offset = object.append_section_data(text, &[1; 30], 4);
    assert_eq!(func1_offset, 32);
    let func1_symbol = object.add_symbol(write::Symbol {
        name: b"func1".to_vec(),
        value: func1_offset,
        size: 32,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: write::SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    object
        .add_relocation(
            text,
            write::Relocation {
                offset: 8,
                symbol: func1_symbol,
                addend: 0,
                flags: RelocationFlags::Generic {
                    kind: RelocationKind::Absolute,
                    encoding: RelocationEncoding::Generic,
                    size: 64,
                },
            },
        )
        .unwrap();
    object
        .add_relocation(
            text,
            write::Relocation {
                offset: 16,
                symbol: func1_symbol,
                addend: -4,
                flags: RelocationFlags::Generic {
                    kind: RelocationKind::Relative,
                    encoding: RelocationEncoding::Generic,
                    size: 32,
                },
            },
        )
        .unwrap();

    round_trip_and_diff("macho_x86_64", object, ArchiveKind::Darwin, 0);
}

#[test]
fn macho_any() {
    // 32-bit object files get additional padding after the round-trip:
    // https://github.com/llvm/llvm-project/blob/3d3ef9d073e1e27ea57480b371b7f5a9f5642ed2/llvm/lib/Object/ArchiveWriter.cpp#L560-L565
    for (arch, subarch, endian, trim_output_bytes) in [
        (Architecture::Aarch64, None, Endianness::Little, 0),
        (
            Architecture::Aarch64,
            Some(SubArchitecture::Arm64E),
            Endianness::Little,
            0,
        ),
        (Architecture::Aarch64_Ilp32, None, Endianness::Little, 4),
        /* TODO:
        (Architecture::Arm, None, Endianness::Little),
        */
        (Architecture::I386, None, Endianness::Little, 4),
        (Architecture::X86_64, None, Endianness::Little, 0),
        /* TODO:
        (Architecture::PowerPc, None, Endianness::Big),
        (Architecture::PowerPc64, None, Endianness::Big),
        */
    ]
    .iter()
    .copied()
    {
        let mut object = write::Object::new(BinaryFormat::MachO, arch, endian);
        object.set_sub_architecture(subarch);

        let section = object.section_id(write::StandardSection::Data);
        object.append_section_data(section, &[1; 30], 4);
        let symbol = object.section_symbol(section);

        object
            .add_relocation(
                section,
                write::Relocation {
                    offset: 8,
                    symbol,
                    addend: 0,
                    flags: RelocationFlags::Generic {
                        kind: RelocationKind::Absolute,
                        encoding: RelocationEncoding::Generic,
                        size: 32,
                    },
                },
            )
            .unwrap();
        if arch.address_size().unwrap().bytes() >= 8 {
            object
                .add_relocation(
                    section,
                    write::Relocation {
                        offset: 16,
                        symbol,
                        addend: 0,
                        flags: RelocationFlags::Generic {
                            kind: RelocationKind::Absolute,
                            encoding: RelocationEncoding::Generic,
                            size: 64,
                        },
                    },
                )
                .unwrap();
        }

        round_trip_and_diff("macho_any", object, ArchiveKind::Darwin, trim_output_bytes);
    }
}

#[test]
fn xcoff_powerpc() {
    for arch in [Architecture::PowerPc, Architecture::PowerPc64] {
        let mut object = write::Object::new(BinaryFormat::Xcoff, arch, Endianness::Big);

        object.add_file_symbol(b"file.c".to_vec());

        let text = object.section_id(write::StandardSection::Text);
        object.append_section_data(text, &[1; 30], 4);

        let func1_offset = object.append_section_data(text, &[1; 30], 4);
        assert_eq!(func1_offset, 32);
        let func1_symbol = object.add_symbol(write::Symbol {
            name: b"func1".to_vec(),
            value: func1_offset,
            size: 32,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: write::SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });

        object
            .add_relocation(
                text,
                write::Relocation {
                    offset: 8,
                    symbol: func1_symbol,
                    addend: 0,
                    flags: RelocationFlags::Generic {
                        kind: RelocationKind::Absolute,
                        encoding: RelocationEncoding::Generic,
                        size: 64,
                    },
                },
            )
            .unwrap();

        round_trip_and_diff("xcoff_powerpc", object, ArchiveKind::Gnu, 0);
    }
}
