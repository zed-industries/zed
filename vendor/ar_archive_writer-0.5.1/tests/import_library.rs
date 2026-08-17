use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;

use ar_archive_writer::{ArchiveKind, COFFShortExport, MachineTypes};
use common::{create_archive_with_ar_archive_writer, create_archive_with_llvm_ar};
use object::coff::CoffFile;
use object::pe::ImageFileHeader;
use object::read::archive::ArchiveFile;
use object::{Architecture, Object, ObjectSection, ObjectSymbol, SubArchitecture, bytes_of};
use pretty_assertions::assert_eq;

mod common;

const DEFAULT_EXPORT: COFFShortExport = COFFShortExport {
    name: String::new(),
    ext_name: None,
    symbol_name: None,
    export_as: None,
    import_name: None,
    ordinal: 0,
    noname: false,
    data: false,
    private: false,
    constant: false,
};

fn get_members(machine_type: MachineTypes) -> Vec<COFFShortExport> {
    let prefix = match machine_type {
        MachineTypes::I386 => "_",
        _ => "",
    };
    // This must match import_library.def.
    vec![
        COFFShortExport {
            name: format!("{prefix}NormalFunc"),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}NormalData"),
            data: true,
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}NormalConstant"),
            constant: true,
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}PrivateFunc"),
            private: true,
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}FuncWithOrdinal"),
            ordinal: 1,
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}FuncWithNoName"),
            ordinal: 2,
            noname: true,
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}InternalName"),
            ext_name: Some(format!("{prefix}RenamedFunc")),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}OtherModule.OtherName"),
            ext_name: Some(format!("{prefix}ReexportedFunc")),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}OtherModule.#42"),
            ext_name: Some(format!("{prefix}ReexportedViaOrd")),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: format!("{prefix}FuncWithImportName"),
            import_name: Some("ImportName".to_string()),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "?foo@@$$hYAHXZ".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "?GetValue@?$Wrapper@UA@@@@$$hQEBAHXZ".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "??2@$$hYAPEAX_K@Z".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "?f@Base@D@C@B@A@@$$hUEAAHXZ".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "??0Base@D@C@B@A@@$$hQEAA@XZ".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "?f@Derived@@$$hUEAAHXZ".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "??0Derived@@$$hQEAA@XZ".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "?MakeObj@@$$hYAPEAUBase@D@C@B@A@@XZ".to_string(),
            ..DEFAULT_EXPORT
        },
        COFFShortExport {
            name: "?GetValue@?$Wrapper@UA@@@@$$hQEBAHUZ@?$WW@UA@@@@@Z".to_string(),
            ..DEFAULT_EXPORT
        },
    ]
}

fn create_import_library_with_ar_archive_writer(
    temp_dir: &Path,
    machine_type: MachineTypes,
    mingw: bool,
    comdat: bool,
) -> Vec<u8> {
    let mut output_bytes = Cursor::new(Vec::new());
    ar_archive_writer::write_import_library(
        &mut output_bytes,
        "MyLibrary.dll",
        &get_members(machine_type),
        machine_type,
        mingw,
        comdat,
        &[],
    )
    .unwrap();

    let output_archive_bytes = output_bytes.into_inner();
    let ar_archive_writer_file_path = temp_dir.join("output_ar_archive_writer.a");
    fs::write(ar_archive_writer_file_path, &output_archive_bytes).unwrap();

    output_archive_bytes
}

#[test]
fn compare_to_lib() {
    for machine_type in [
        MachineTypes::I386,
        MachineTypes::AMD64,
        MachineTypes::ARMNT,
        MachineTypes::ARM64,
        MachineTypes::ARM64EC,
    ] {
        let temp_dir = common::create_tmp_dir("import_library_compare_to_lib");

        let archive_writer_bytes =
            create_import_library_with_ar_archive_writer(&temp_dir, machine_type, false, false);

        let llvm_lib_bytes = {
            let machine_arg = match machine_type {
                MachineTypes::I386 => "X86",
                MachineTypes::AMD64 => "X64",
                MachineTypes::ARMNT => "ARM",
                MachineTypes::ARM64 => "ARM64",
                MachineTypes::ARM64EC => "ARM64EC",
                _ => panic!("Unsupported machine type"),
            };

            let llvm_lib_tool_path = common::create_llvm_lib_tool(&temp_dir);
            let output_library_path = temp_dir.join("output_llvm_lib.a");
            let def_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/import_library.def");
            let output = Command::new(llvm_lib_tool_path)
                .arg(format!("/MACHINE:{machine_arg}"))
                .arg(format!("/DEF:{}", def_path.to_string_lossy()))
                .arg(format!("/OUT:{}", output_library_path.to_string_lossy()))
                .output()
                .unwrap();

            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                "",
                "llvm-lib failed. archive: {output_library_path:?}"
            );
            fs::read(output_library_path).unwrap()
        };

        assert_eq!(
            llvm_lib_bytes, archive_writer_bytes,
            "Import library differs. Machine type: {machine_type:?}",
        );

        compare_comdat(
            &create_import_library_with_ar_archive_writer(&temp_dir, machine_type, false, true),
            &llvm_lib_bytes,
        );
    }
}

#[test]
fn compare_to_dlltool() {
    for machine_type in [
        MachineTypes::I386,
        MachineTypes::AMD64,
        MachineTypes::ARMNT,
        MachineTypes::ARM64,
    ] {
        let temp_dir = common::create_tmp_dir("import_library_compare_to_dlltool");

        let archive_writer_bytes =
            create_import_library_with_ar_archive_writer(&temp_dir, machine_type, true, false);

        let llvm_lib_bytes = {
            let machine_arg = match machine_type {
                MachineTypes::I386 => "i386",
                MachineTypes::AMD64 => "i386:x86-64",
                MachineTypes::ARMNT => "arm",
                MachineTypes::ARM64 => "arm64",
                _ => panic!("Unsupported machine type"),
            };

            let llvm_lib_tool_path = common::create_llvm_dlltool_tool(&temp_dir);
            let output_library_path = temp_dir.join("output_llvm_lib.a");
            let def_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/import_library.def");
            let output = Command::new(llvm_lib_tool_path)
                .arg("--machine")
                .arg(machine_arg)
                .arg("--input-def")
                .arg(def_path)
                .arg("--output-lib")
                .arg(&output_library_path)
                .output()
                .unwrap();

            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                "",
                "llvm-lib failed. archive: {output_library_path:?}"
            );
            fs::read(output_library_path).unwrap()
        };

        assert_eq!(
            llvm_lib_bytes, archive_writer_bytes,
            "Import library differs. Machine type: {machine_type:?}",
        );

        compare_comdat(
            &create_import_library_with_ar_archive_writer(&temp_dir, machine_type, true, true),
            &llvm_lib_bytes,
        );
    }
}

/// Creates an import library and then wraps that in an archive.
#[test]
fn wrap_in_archive() {
    for (architecture, subarch, machine_type) in [
        (Architecture::I386, None, MachineTypes::I386),
        (Architecture::X86_64, None, MachineTypes::AMD64),
        (Architecture::Arm, None, MachineTypes::ARMNT),
        (Architecture::Aarch64, None, MachineTypes::ARM64),
        (
            Architecture::Aarch64,
            Some(SubArchitecture::Arm64EC),
            MachineTypes::ARM64EC,
        ),
    ] {
        let temp_dir = common::create_tmp_dir("import_library_wrap_in_archive");

        let mut import_lib_bytes = Cursor::new(Vec::new());
        ar_archive_writer::write_import_library(
            &mut import_lib_bytes,
            &temp_dir.join("MyLibrary.dll").to_string_lossy(),
            &get_members(machine_type),
            machine_type,
            false,
            false,
            &[],
        )
        .unwrap();
        let import_lib_bytes = import_lib_bytes.into_inner();

        let is_ec = subarch == Some(SubArchitecture::Arm64EC);
        let llvm_ar_archive = create_archive_with_llvm_ar(
            &temp_dir,
            ArchiveKind::Coff,
            [("MyLibrary.dll.lib", import_lib_bytes.as_slice())],
            false,
            is_ec,
        );
        // When a archive is passed into lib.exe, it is opened and the individual members are included
        // in the new output library. Also, for whatever reason, the members are reversed.
        let archive = ArchiveFile::parse(import_lib_bytes.as_slice()).unwrap();
        let mut members = archive
            .members()
            .map(|m| {
                let member = m.unwrap();
                (
                    String::from_utf8(member.name().to_vec()).unwrap(),
                    member.data(import_lib_bytes.as_slice()).unwrap(),
                )
            })
            .collect::<Vec<_>>();
        members.reverse();
        let ar_archive_writer_archive = create_archive_with_ar_archive_writer(
            &temp_dir,
            ArchiveKind::Coff,
            members.iter().map(|(name, data)| (name.as_str(), *data)),
            false,
            is_ec,
        );

        assert_eq!(
            llvm_ar_archive, ar_archive_writer_archive,
            "Archives differ for architecture: {architecture:?}, subarch: {subarch:?}, machine type: {machine_type:?}",
        );
    }
}

fn compare_comdat(archive_writer_bytes: &[u8], llvm_bytes: &[u8]) {
    let archive_writer = ArchiveFile::parse(archive_writer_bytes).unwrap();
    let llvm = ArchiveFile::parse(llvm_bytes).unwrap();

    for (archive_member, llvm_member) in archive_writer.members().zip(llvm.members()) {
        let archive_member = archive_member.unwrap();
        let llvm_member = llvm_member.unwrap();

        // skip the EC symbols table.
        if archive_member.name() == b"/<ECSYMBOLS>/" {
            continue;
        }

        if archive_member.size() != llvm_member.size() {
            // Ensure that the member header is the same except for the file size.
            let mut llvm_file_header = *llvm_member.header().unwrap();
            llvm_file_header.size = archive_member.header().unwrap().size;
            assert_eq!(
                bytes_of(archive_member.header().unwrap()),
                bytes_of(&llvm_file_header)
            );

            // Make sure they are both COFF files with the same sections and symbols,
            // except for the different naming for the null import descriptor.
            let archive_data = archive_member.data(archive_writer_bytes).unwrap();
            let llvm_data = llvm_member.data(llvm_bytes).unwrap();
            let archive_file = CoffFile::<_, ImageFileHeader>::parse(archive_data).unwrap();
            let llvm_file = CoffFile::<_, ImageFileHeader>::parse(llvm_data).unwrap();

            for (archive_section, llvm_section) in archive_file.sections().zip(llvm_file.sections())
            {
                assert_eq!(archive_section.data(), llvm_section.data());
            }
            for (archive_symbol, llvm_symbol) in archive_file.symbols().zip(llvm_file.symbols()) {
                if llvm_symbol.name().unwrap() == "__NULL_IMPORT_DESCRIPTOR" {
                    assert!(
                        archive_symbol
                            .name()
                            .unwrap()
                            .starts_with("__NULL_IMPORT_DESCRIPTOR_")
                    );
                } else {
                    assert_eq!(archive_symbol.name(), llvm_symbol.name());
                }
                let archive_coff_symbol = archive_symbol.coff_symbol();
                let mut llvm_coff_symbol = *llvm_symbol.coff_symbol();
                llvm_coff_symbol.name = archive_coff_symbol.name;
                assert_eq!(bytes_of(archive_coff_symbol), bytes_of(&llvm_coff_symbol));
            }
        } else {
            assert_eq!(
                bytes_of(archive_member.header().unwrap()),
                bytes_of(llvm_member.header().unwrap())
            );
            assert_eq!(
                archive_member.data(archive_writer_bytes).unwrap(),
                llvm_member.data(llvm_bytes).unwrap()
            );
        }
    }
}
