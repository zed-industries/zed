#![allow(dead_code)]

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;

use ar_archive_writer::{ArchiveKind, NewArchiveMember};
use object::write::{self, Object};
use object::{
    Architecture, BinaryFormat, Endianness, SubArchitecture, SymbolFlags, SymbolKind, SymbolScope,
};
use pretty_assertions::assert_eq;

/// Creates the temporary directory for a test.
pub fn create_tmp_dir(test_name: &str) -> PathBuf {
    let tmpdir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(test_name);
    match fs::remove_dir_all(&tmpdir) {
        Ok(_) => {}
        Err(err) => {
            if err.kind() != std::io::ErrorKind::NotFound {
                panic!("Failed to delete directory: {tmpdir:?}");
            }
        }
    }
    fs::create_dir_all(&tmpdir).unwrap();
    tmpdir
}

/// Creates a symlink to `llvm-ar` so that it acts like `llvm-lib`.
pub fn create_llvm_lib_tool(tmp_dir: &Path) -> PathBuf {
    let ar_path = cargo_binutils::Tool::Ar.path().unwrap();
    let lib_path = tmp_dir.join("llvm-lib");
    #[cfg(unix)]
    std::os::unix::fs::symlink(ar_path, &lib_path).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(ar_path, &lib_path).unwrap();
    lib_path
}

/// Creates a symlink to `llvm-ar` so that it acts like `llvm-dlltool`.
pub fn create_llvm_dlltool_tool(tmp_dir: &Path) -> PathBuf {
    let ar_path = cargo_binutils::Tool::Ar.path().unwrap();
    let lib_path = tmp_dir.join("llvm-dlltool");
    #[cfg(unix)]
    std::os::unix::fs::symlink(ar_path, &lib_path).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(ar_path, &lib_path).unwrap();
    lib_path
}

fn run_llvm_ar(
    object_paths: &[PathBuf],
    archive_path: &Path,
    archive_kind: ArchiveKind,
    thin: bool,
    is_ec: bool,
) {
    // FIXME: LLVM 19 adds support for "coff" as a format argument, so in the
    // meantime, we'll instruct llvm-ar to pretend to be llvm-lib.
    let output = if archive_kind == ArchiveKind::Coff {
        let lib_path = create_llvm_lib_tool(archive_path.parent().unwrap());
        let mut command = Command::new(lib_path);

        if is_ec {
            command.arg("/machine:arm64ec");
        }

        // llvm-lib reverses the order of the files versus llvm-ar.
        let mut object_paths = Vec::from(object_paths);
        object_paths.reverse();
        command
            .arg("/OUT:".to_string() + archive_path.to_str().unwrap())
            .args(object_paths)
            .output()
            .unwrap()
    } else {
        let ar_path = cargo_binutils::Tool::Ar.path().unwrap();
        let mut command = Command::new(ar_path);

        let format_arg = match archive_kind {
            ArchiveKind::AixBig => "bigarchive",
            ArchiveKind::Darwin => "darwin",
            ArchiveKind::Gnu => "gnu",
            _ => panic!("unsupported archive kind: {archive_kind:?}"),
        };
        command.arg(format!("--format={format_arg}"));

        if thin {
            command.arg("--thin");
        }

        command
            .arg("rcs")
            .arg(archive_path)
            .args(object_paths)
            .output()
            .unwrap()
    };

    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "",
        "llvm-ar failed. archive: {archive_path:?}"
    );
}

/// Creates an archive with the given objects using `llvm-ar`.
/// The generated archive is written to disk as `output_llvm_ar.a`.
pub fn create_archive_with_llvm_ar<'name, 'data>(
    tmpdir: &Path,
    archive_kind: ArchiveKind,
    input_objects: impl IntoIterator<Item = (&'name str, &'data [u8])>,
    thin: bool,
    is_ec: bool,
) -> Vec<u8> {
    let archive_file_path = tmpdir.join("output_llvm_ar.a");

    let input_file_paths = input_objects
        .into_iter()
        .map(|(name, bytes)| {
            let input_file_path = tmpdir.join(name);
            if name.contains('/') {
                fs::create_dir_all(input_file_path.parent().unwrap()).unwrap();
            }
            fs::write(&input_file_path, bytes).unwrap();
            input_file_path
        })
        .collect::<Vec<_>>();

    run_llvm_ar(
        &input_file_paths,
        &archive_file_path,
        archive_kind,
        thin,
        is_ec,
    );
    fs::read(archive_file_path).unwrap()
}

/// Creates an archive with the given objects using `ar_archive_writer`.
/// The generated archive is written to disk as `output_ar_archive_writer.a`.
pub fn create_archive_with_ar_archive_writer<'name, 'data>(
    tmpdir: &Path,
    archive_kind: ArchiveKind,
    input_objects: impl IntoIterator<Item = (&'name str, &'data [u8])>,
    thin: bool,
    is_ec: bool,
) -> Vec<u8> {
    let members = input_objects
        .into_iter()
        .map(|(name, bytes)| {
            let member_name = if thin {
                // Thin archives use the full path to the object file.
                tmpdir
                    .join(name)
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/")
            } else if archive_kind == ArchiveKind::Coff {
                // For COFF archives, we are running llvm-ar as lib.exe, which
                // uses the full path to the object file.
                tmpdir.join(name).to_string_lossy().to_string()
            } else {
                // Non-thin archives use the file name only.
                name.rsplit_once('/')
                    .map_or(name, |(_, filename)| filename)
                    .to_string()
            };

            NewArchiveMember::new(
                bytes,
                &ar_archive_writer::DEFAULT_OBJECT_READER,
                member_name,
            )
        })
        .collect::<Vec<_>>();
    let mut output_bytes = Cursor::new(Vec::new());
    ar_archive_writer::write_archive_to_stream(
        &mut output_bytes,
        &members,
        archive_kind,
        thin,
        Some(is_ec),
    )
    .unwrap();

    let output_archive_bytes = output_bytes.into_inner();
    let ar_archive_writer_file_path = tmpdir.join("output_ar_archive_writer.a");
    fs::write(ar_archive_writer_file_path, &output_archive_bytes).unwrap();
    output_archive_bytes
}

/// Helper for comparing archives generated by `llvm-ar` and `ar_archive_writer`
/// across a variety of archive kinds and their relevant object formats.
pub fn generate_archive_and_compare<F>(test_name: &str, generate_objects: F)
where
    F: Fn(
        Architecture,
        Option<SubArchitecture>,
        Endianness,
        BinaryFormat,
    ) -> Vec<(&'static str, Vec<u8>)>,
{
    for (architecture, subarch, endianness, binary_format, archive_kind, thin) in [
        // Elf + GNU + non-thin
        (
            Architecture::X86_64,
            None,
            Endianness::Little,
            BinaryFormat::Elf,
            ArchiveKind::Gnu,
            false,
        ),
        (
            Architecture::I386,
            None,
            Endianness::Little,
            BinaryFormat::Elf,
            ArchiveKind::Gnu,
            false,
        ),
        (
            Architecture::Aarch64,
            None,
            Endianness::Little,
            BinaryFormat::Elf,
            ArchiveKind::Gnu,
            false,
        ),
        // Elf + GNU + thin
        (
            Architecture::X86_64,
            None,
            Endianness::Little,
            BinaryFormat::Elf,
            ArchiveKind::Gnu,
            true,
        ),
        (
            Architecture::I386,
            None,
            Endianness::Little,
            BinaryFormat::Elf,
            ArchiveKind::Gnu,
            true,
        ),
        (
            Architecture::Aarch64,
            None,
            Endianness::Little,
            BinaryFormat::Elf,
            ArchiveKind::Gnu,
            true,
        ),
        // AIX Big
        (
            Architecture::PowerPc64,
            None,
            Endianness::Big,
            BinaryFormat::Elf,
            ArchiveKind::AixBig,
            false,
        ),
        // PE + GNU
        (
            Architecture::X86_64,
            None,
            Endianness::Little,
            BinaryFormat::Coff,
            ArchiveKind::Gnu,
            false,
        ),
        (
            Architecture::I386,
            None,
            Endianness::Little,
            BinaryFormat::Coff,
            ArchiveKind::Gnu,
            false,
        ),
        // PE + Coff
        (
            Architecture::X86_64,
            None,
            Endianness::Little,
            BinaryFormat::Coff,
            ArchiveKind::Coff,
            false,
        ),
        (
            Architecture::I386,
            None,
            Endianness::Little,
            BinaryFormat::Coff,
            ArchiveKind::Coff,
            false,
        ),
        (
            Architecture::Aarch64,
            None,
            Endianness::Little,
            BinaryFormat::Coff,
            ArchiveKind::Coff,
            false,
        ),
        (
            Architecture::Aarch64,
            Some(SubArchitecture::Arm64EC),
            Endianness::Little,
            BinaryFormat::Coff,
            ArchiveKind::Coff,
            false,
        ),
        // MachO
        (
            Architecture::X86_64,
            None,
            Endianness::Little,
            BinaryFormat::MachO,
            ArchiveKind::Darwin,
            false,
        ),
        (
            Architecture::Aarch64,
            None,
            Endianness::Little,
            BinaryFormat::MachO,
            ArchiveKind::Darwin,
            false,
        ),
        (
            Architecture::Aarch64,
            Some(SubArchitecture::Arm64E),
            Endianness::Little,
            BinaryFormat::MachO,
            ArchiveKind::Darwin,
            false,
        ),
    ] {
        let is_ec = subarch == Some(SubArchitecture::Arm64EC);
        let tmpdir = create_tmp_dir(test_name);
        let input_objects = generate_objects(architecture, subarch, endianness, binary_format);
        let llvm_ar_archive = create_archive_with_llvm_ar(
            &tmpdir,
            archive_kind,
            input_objects
                .iter()
                .map(|(name, bytes)| (*name, bytes.as_slice())),
            thin,
            is_ec,
        );
        let ar_archive_writer_archive = create_archive_with_ar_archive_writer(
            &tmpdir,
            archive_kind,
            input_objects
                .iter()
                .map(|(name, bytes)| (*name, bytes.as_slice())),
            thin,
            is_ec,
        );

        assert_eq!(
            llvm_ar_archive, ar_archive_writer_archive,
            "Archives differ for architecture: {architecture:?}, binary_format: {binary_format:?}, archive_kind: {archive_kind:?}, thin: {thin}",
        );
    }
}

pub fn add_file_with_functions_to_object(
    object: &mut Object<'_>,
    file_name: &[u8],
    func_names: &[&[u8]],
) {
    object.add_file_symbol(file_name.to_vec());

    let text = object.section_id(write::StandardSection::Text);
    object.append_section_data(text, &[1; 30], 4);

    for func_name in func_names {
        let offset = object.append_section_data(text, &[1; 30], 4);

        object.add_symbol(write::Symbol {
            name: func_name.to_vec(),
            value: offset,
            size: 32,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: write::SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
}
