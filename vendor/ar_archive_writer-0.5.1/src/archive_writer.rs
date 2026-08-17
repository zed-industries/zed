#![allow(rustc::default_hash_types, rustc::potential_query_instability)]

// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::collections::{BTreeMap, HashMap};
use std::io::{self, Cursor, Seek, Write};
use std::mem::size_of;

use crate::ObjectReader;
use crate::alignment::*;
use crate::archive::*;
use crate::coff_import_file;
use crate::math_extras::align_to_power_of2;

const BIG_AR_MEM_HDR_SIZE: u64 = {
    // `try_into` is not const, so check the size manually.
    assert!(
        std::mem::size_of::<usize>() <= std::mem::size_of::<u64>()
            || std::mem::size_of::<big_archive::BigArMemHdrType>() < u64::MAX as usize
    );
    std::mem::size_of::<big_archive::BigArMemHdrType>() as u64
};

#[derive(Default)]
struct SymMap {
    use_ec_map: bool,
    map: BTreeMap<Box<[u8]>, u16>,
    ec_map: BTreeMap<Box<[u8]>, u16>,
}

pub struct NewArchiveMember<'a> {
    pub buf: Box<dyn AsRef<[u8]> + 'a>,
    pub object_reader: &'static ObjectReader,
    pub member_name: String,
    pub mtime: u64,
    pub uid: u32,
    pub gid: u32,
    pub perms: u32,
}

impl<'a> NewArchiveMember<'a> {
    pub fn new<T: AsRef<[u8]> + 'a>(
        buf: T,
        object_reader: &'static ObjectReader,
        member_name: String,
    ) -> Self {
        Self {
            buf: Box::new(buf),
            object_reader,
            member_name,
            mtime: 0,
            uid: 0,
            gid: 0,
            perms: 0o644,
        }
    }
}

fn is_darwin(kind: ArchiveKind) -> bool {
    matches!(kind, ArchiveKind::Darwin | ArchiveKind::Darwin64)
}

fn is_aix_big_archive(kind: ArchiveKind) -> bool {
    kind == ArchiveKind::AixBig
}

fn is_coff_archive(kind: ArchiveKind) -> bool {
    kind == ArchiveKind::Coff
}

fn is_bsd_like(kind: ArchiveKind) -> bool {
    match kind {
        ArchiveKind::Gnu | ArchiveKind::Gnu64 | ArchiveKind::AixBig | ArchiveKind::Coff => false,
        ArchiveKind::Bsd | ArchiveKind::Darwin | ArchiveKind::Darwin64 => true,
    }
}

fn print_rest_of_member_header<W: Write>(
    w: &mut W,
    mtime: u64,
    uid: u32,
    gid: u32,
    perms: u32,
    size: u64,
) -> io::Result<()> {
    // The format has only 6 chars for uid and gid. Truncate if the provided
    // values don't fit.
    write!(
        w,
        "{:<12}{:<6}{:<6}{:<8o}{:<10}`\n",
        mtime,
        uid % 1000000,
        gid % 1000000,
        perms,
        size
    )
}

fn print_gnu_small_member_header<W: Write>(
    w: &mut W,
    name: String,
    mtime: u64,
    uid: u32,
    gid: u32,
    perms: u32,
    size: u64,
) -> io::Result<()> {
    write!(w, "{:<16}", name + "/")?;
    print_rest_of_member_header(w, mtime, uid, gid, perms, size)
}

fn print_bsd_member_header<W: Write>(
    w: &mut W,
    pos: u64,
    name: &str,
    mtime: u64,
    uid: u32,
    gid: u32,
    perms: u32,
    size: u64,
) -> io::Result<()> {
    let pos_after_header = pos + 60 + u64::try_from(name.len()).unwrap();
    // Pad so that even 64 bit object files are aligned.
    let pad = offset_to_alignment(pos_after_header, 8);
    let name_with_padding = u64::try_from(name.len()).unwrap() + pad;
    write!(w, "#1/{name_with_padding:<13}")?;
    print_rest_of_member_header(w, mtime, uid, gid, perms, name_with_padding + size)?;
    write!(w, "{name}")?;
    write!(
        w,
        "{nil:\0<pad$}",
        nil = "",
        pad = usize::try_from(pad).unwrap()
    )
}

fn print_big_archive_member_header<W: Write>(
    w: &mut W,
    name: &str,
    mtime: u64,
    uid: u32,
    gid: u32,
    perms: u32,
    size: u64,
    prev_offset: u64,
    next_offset: u64,
) -> io::Result<()> {
    write!(
        w,
        "{:<20}{:<20}{:<20}{:<12}{:<12}{:<12}{:<12o}{:<4}",
        size,
        next_offset,
        prev_offset,
        mtime,
        u64::from(uid) % 1000000000000u64,
        u64::from(gid) % 1000000000000u64,
        perms,
        name.len(),
    )?;

    if !name.is_empty() {
        write!(w, "{name}")?;

        if name.len() % 2 != 0 {
            write!(w, "\0")?;
        }
    }

    write!(w, "`\n")?;

    Ok(())
}

fn use_string_table(thin: bool, name: &str) -> bool {
    thin || name.len() >= 16 || name.contains('/')
}

fn is_64bit_kind(kind: ArchiveKind) -> bool {
    match kind {
        ArchiveKind::Gnu | ArchiveKind::Bsd | ArchiveKind::Darwin | ArchiveKind::Coff => false,
        ArchiveKind::AixBig | ArchiveKind::Darwin64 | ArchiveKind::Gnu64 => true,
    }
}

fn print_member_header<'m, W: Write, T: Write + Seek>(
    w: &mut W,
    pos: u64,
    string_table: &mut T,
    member_names: &mut HashMap<&'m str, u64>,
    kind: ArchiveKind,
    thin: bool,
    m: &'m NewArchiveMember<'m>,
    mtime: u64,
    size: u64,
) -> io::Result<()> {
    if is_bsd_like(kind) {
        return print_bsd_member_header(w, pos, &m.member_name, mtime, m.uid, m.gid, m.perms, size);
    }

    if !use_string_table(thin, &m.member_name) {
        return print_gnu_small_member_header(
            w,
            m.member_name.clone(),
            mtime,
            m.uid,
            m.gid,
            m.perms,
            size,
        );
    }

    write!(w, "/")?;
    let name_pos;
    if thin {
        name_pos = string_table.stream_position()?;
        write!(string_table, "{}/\n", m.member_name)?;
    } else if let Some(&pos) = member_names.get(&*m.member_name) {
        name_pos = pos;
    } else {
        name_pos = string_table.stream_position()?;
        member_names.insert(&m.member_name, name_pos);
        write!(string_table, "{}", m.member_name)?;
        if is_coff_archive(kind) {
            write!(string_table, "\0")?;
        } else {
            write!(string_table, "/\n")?;
        }
    }
    write!(w, "{name_pos:<15}")?;
    print_rest_of_member_header(w, mtime, m.uid, m.gid, m.perms, size)
}

struct MemberData<'a> {
    symbols: Vec<u64>,
    header: Vec<u8>,
    data: &'a [u8],
    padding: &'static [u8],
    pre_head_pad_size: u64,
    object_reader: &'static ObjectReader,
}

fn compute_string_table(names: &[u8]) -> MemberData<'_> {
    let size = u64::try_from(names.len()).unwrap();
    let pad = offset_to_alignment(size, 2);
    let mut header = Vec::new();
    write!(header, "{:<48}", "//").unwrap();
    write!(header, "{:<10}", size + pad).unwrap();
    write!(header, "`\n").unwrap();
    MemberData {
        symbols: vec![],
        header,
        data: names,
        padding: if pad != 0 { b"\n" } else { b"" },
        pre_head_pad_size: 0,
        object_reader: &crate::DEFAULT_OBJECT_READER,
    }
}

const fn now() -> u64 {
    0
}

// NOTE: isArchiveSymbol was moved to object_reader.rs

fn print_n_bits<W: Write>(w: &mut W, kind: ArchiveKind, val: u64) -> io::Result<()> {
    if is_64bit_kind(kind) {
        w.write_all(&if is_bsd_like(kind) {
            u64::to_le_bytes(val)
        } else {
            u64::to_be_bytes(val)
        })
    } else {
        w.write_all(&if is_bsd_like(kind) {
            u32::to_le_bytes(u32::try_from(val).unwrap())
        } else {
            u32::to_be_bytes(u32::try_from(val).unwrap())
        })
    }
}

fn compute_symbol_table_size_and_pad(
    kind: ArchiveKind,
    num_syms: u64,
    offset_size: u64,
    string_table_size: u64,
) -> (u64, u64) {
    assert!(
        offset_size == 4 || offset_size == 8,
        "Unsupported offset_size"
    );
    let mut size = offset_size; // Number of entries
    if is_bsd_like(kind) {
        size += num_syms * offset_size * 2; // Table
    } else {
        size += num_syms * offset_size; // Table
    }
    if is_bsd_like(kind) {
        size += offset_size; // byte count;
    }
    size += string_table_size;
    // ld64 expects the members to be 8-byte aligned for 64-bit content and at
    // least 4-byte aligned for 32-bit content.  Opt for the larger encoding
    // uniformly.
    // We do this for all bsd formats because it simplifies aligning members.
    let pad = if is_aix_big_archive(kind) {
        0
    } else {
        offset_to_alignment(size, if is_bsd_like(kind) { 8 } else { 2 })
    };
    size += pad;
    (size, pad)
}

fn compute_symbol_map_size_and_pad(num_obj: usize, sym_map: &SymMap) -> (u64, u64) {
    let mut size = size_of::<u32>() * 2; // Number of symbols and objects entries
    size += num_obj * size_of::<u32>(); // Offset table

    for name in sym_map.map.keys() {
        size += size_of::<u16>() + name.len() + 1;
    }

    let mut size = u64::try_from(size).unwrap();
    let pad = offset_to_alignment(size, 2);
    size += pad;
    (size, pad)
}

fn compute_ec_symbols_size_and_pad(sym_map: &SymMap) -> (u64, u64) {
    let mut size = size_of::<u32>(); // Number of symbols

    for name in sym_map.ec_map.keys() {
        size += size_of::<u16>() + name.len() + 1;
    }

    let mut size = u64::try_from(size).unwrap();
    let pad = offset_to_alignment(size, 2);
    size += pad;
    (size, pad)
}

fn write_symbol_table_header<W: Write + Seek>(
    w: &mut W,
    kind: ArchiveKind,
    size: u64,
    prev_member_offset: u64,
    next_member_offset: u64,
) -> io::Result<()> {
    if is_bsd_like(kind) {
        let name = if is_64bit_kind(kind) {
            "__.SYMDEF_64"
        } else {
            "__.SYMDEF"
        };
        let pos = w.stream_position()?;
        print_bsd_member_header(w, pos, name, now(), 0, 0, 0, size)
    } else if is_aix_big_archive(kind) {
        print_big_archive_member_header(
            w,
            "",
            now(),
            0,
            0,
            0,
            size,
            prev_member_offset,
            next_member_offset,
        )
    } else {
        let name = if is_64bit_kind(kind) { "/SYM64" } else { "" };
        print_gnu_small_member_header(w, name.to_string(), now(), 0, 0, 0, size)
    }
}

fn compute_headers_size(
    kind: ArchiveKind,
    num_members: usize,
    string_member_size: u64,
    num_syms: u64,
    sym_names_size: u64,
    sym_map: Option<&SymMap>,
) -> io::Result<u64> {
    let offset_size = if is_64bit_kind(kind) { 8 } else { 4 };
    let (symtab_size, _) =
        compute_symbol_table_size_and_pad(kind, num_syms, offset_size, sym_names_size);
    let compute_symbol_table_header_size = || -> io::Result<u64> {
        let mut tmp = Cursor::new(Vec::new());
        write_symbol_table_header(&mut tmp, kind, symtab_size, 0, 0)?;
        Ok(tmp.into_inner().len().try_into().unwrap())
    };
    let header_size = compute_symbol_table_header_size()?;
    let mut size = u64::try_from("!<arch>\n".len()).unwrap() + header_size + symtab_size;

    if let Some(sym_map) = sym_map {
        size += header_size + compute_symbol_map_size_and_pad(num_members, sym_map).0;
        if !sym_map.ec_map.is_empty() {
            size += header_size + compute_ec_symbols_size_and_pad(sym_map).0;
        }
    }

    Ok(size + string_member_size)
}

// NOTE: is64BitSymbolicFile, getAuxMaxAlignment and getMemberAlignment were
// moved to object_reader.rs

fn write_symbol_table<W: Write + Seek>(
    w: &mut W,
    kind: ArchiveKind,
    members: &[MemberData<'_>],
    string_table: &[u8],
    members_offset: u64,
    num_syms: u64,
    prev_member_offset: u64,
    next_member_offset: u64,
    is_64_bit: bool,
) -> io::Result<()> {
    // We don't write a symbol table on an archive with no members -- except on
    // Darwin, where the linker will abort unless the archive has a symbol table.
    if string_table.is_empty() && !is_darwin(kind) && !is_coff_archive(kind) {
        return Ok(());
    }

    let offset_size = if is_64bit_kind(kind) { 8 } else { 4 };
    let (size, pad) = compute_symbol_table_size_and_pad(
        kind,
        num_syms,
        offset_size,
        string_table.len().try_into().unwrap(),
    );
    write_symbol_table_header(w, kind, size, prev_member_offset, next_member_offset)?;

    if is_bsd_like(kind) {
        print_n_bits(w, kind, num_syms * 2 * offset_size)?;
    } else {
        print_n_bits(w, kind, num_syms)?;
    }

    let mut pos = members_offset;
    for m in members {
        if is_aix_big_archive(kind) {
            pos += m.pre_head_pad_size;
            if (m.object_reader.is_64_bit_object_file)(m.data) != is_64_bit {
                pos += u64::try_from(m.header.len() + m.data.len() + m.padding.len()).unwrap();
                continue;
            }
        }

        for &string_offset in &m.symbols {
            if is_bsd_like(kind) {
                print_n_bits(w, kind, string_offset)?;
            }
            print_n_bits(w, kind, pos)?; // member offset
        }
        pos += u64::try_from(m.header.len() + m.data.len() + m.padding.len()).unwrap();
    }

    if is_bsd_like(kind) {
        // byte count of the string table
        print_n_bits(w, kind, u64::try_from(string_table.len()).unwrap())?;
    }

    w.write_all(string_table)?;

    write!(
        w,
        "{nil:\0<pad$}",
        nil = "",
        pad = usize::try_from(pad).unwrap()
    )
}

fn write_symbol_map<W: Write + Seek>(
    w: &mut W,
    kind: ArchiveKind,
    members: &[MemberData<'_>],
    sym_map: &SymMap,
    members_offset: u64,
) -> io::Result<()> {
    let (size, pad) = compute_symbol_map_size_and_pad(members.len(), sym_map);
    write_symbol_table_header(w, kind, size, 0, 0)?;

    let mut pos: u32 = members_offset.try_into().unwrap();

    w.write_all(&u32::try_from(members.len()).unwrap().to_le_bytes())?;
    for m in members {
        w.write_all(&pos.to_le_bytes())?; // member offset
        pos = pos
            .checked_add(
                (m.header.len() + m.data.len() + m.padding.len())
                    .try_into()
                    .unwrap(),
            )
            .unwrap();
    }

    w.write_all(&u32::try_from(sym_map.map.len()).unwrap().to_le_bytes())?;

    for s in sym_map.map.values() {
        w.write_all(&s.to_le_bytes())?;
    }
    for s in sym_map.map.keys() {
        w.write_all(s)?;
        w.write_all(&[0])?;
    }

    write!(
        w,
        "{nil:\0<pad$}",
        nil = "",
        pad = usize::try_from(pad).unwrap()
    )?;
    Ok(())
}

fn write_ec_symbols<W: Write + Seek>(w: &mut W, sym_map: &SymMap) -> io::Result<()> {
    let (size, pad) = compute_ec_symbols_size_and_pad(sym_map);
    print_gnu_small_member_header(w, "/<ECSYMBOLS>".to_string(), now(), 0, 0, 0, size)?;

    w.write_all(&u32::try_from(sym_map.ec_map.len()).unwrap().to_le_bytes())?;

    for s in sym_map.ec_map.values() {
        w.write_all(&s.to_le_bytes())?;
    }
    for s in sym_map.ec_map.keys() {
        w.write_all(s)?;
        w.write_all(&[0])?;
    }

    write!(
        w,
        "{nil:\0<pad$}",
        nil = "",
        pad = usize::try_from(pad).unwrap()
    )?;
    Ok(())
}

// NOTE: isECObject and isAnyArm64COFF was moved to object_reader.rs

fn is_import_descriptor(name: &[u8]) -> bool {
    name.starts_with(coff_import_file::IMPORT_DESCRIPTOR_PREFIX)
        || name.starts_with(coff_import_file::NULL_IMPORT_DESCRIPTOR_SYMBOL_NAME)
        || (name.starts_with(coff_import_file::NULL_THUNK_DATA_PREFIX)
            && name.ends_with(coff_import_file::NULL_THUNK_DATA_SUFFIX))
}

// NOTE: LLVM calls this getSymbols and has the get_native_object_symbols
// function (moved to object_reader.rs) inlined.
fn write_symbols(
    obj: &[u8],
    index: u16,
    sym_names: &mut Cursor<Vec<u8>>,
    sym_map: &mut Option<&mut SymMap>,
    object_reader: &ObjectReader,
) -> io::Result<Vec<u64>> {
    let mut ret = vec![];

    let mut is_using_map = false;
    let (mut map, mut ec_map) = if let Some(sym_map) = sym_map {
        if sym_map.use_ec_map && (object_reader.is_ec_object_file)(obj) {
            (Some(&mut sym_map.ec_map), None)
        } else {
            is_using_map = true;
            (
                Some(&mut sym_map.map),
                sym_map.use_ec_map.then_some(&mut sym_map.ec_map),
            )
        }
    } else {
        (None, None)
    };

    (object_reader.get_symbols)(obj, &mut |name| {
        if let Some(map) = &mut map {
            let entry = map.entry(name.to_vec().into_boxed_slice());
            if matches!(entry, std::collections::btree_map::Entry::Occupied(_)) {
                return Ok(()); // ignore duplicated symbol
            }
            entry.or_insert(index);

            if is_using_map {
                ret.push(sym_names.stream_position()?);
                sym_names.write_all(name)?;
                sym_names.write_all(&[0])?;

                // If EC is enabled, then the import descriptors are NOT put into EC
                // objects so we need to copy them to the EC map manually.
                if let Some(ec_map) = &mut ec_map
                    && is_import_descriptor(name)
                {
                    ec_map.insert(name.to_vec().into_boxed_slice(), index);
                }
            }
        } else {
            ret.push(sym_names.stream_position()?);
            sym_names.write_all(name)?;
            sym_names.write_all(&[0])?;
        }
        Ok(())
    })?;
    Ok(ret)
}

fn compute_member_data<'a, S: Write + Seek>(
    string_table: &mut S,
    sym_names: &mut Cursor<Vec<u8>>,
    kind: ArchiveKind,
    thin: bool,
    sym_map: &mut Option<&mut SymMap>,
    new_members: &'a [NewArchiveMember<'a>],
    is_ec: Option<bool>,
) -> io::Result<Vec<MemberData<'a>>> {
    const PADDING_DATA: &[u8; 8] = &[b'\n'; 8];

    let mut mem_head_pad_size = 0;
    // This ignores the symbol table, but we only need the value mod 8 and the
    // symbol table is aligned to be a multiple of 8 bytes
    let mut pos = if is_aix_big_archive(kind) {
        u64::try_from(std::mem::size_of::<big_archive::FixLenHdr>()).unwrap()
    } else {
        0
    };

    let mut ret = vec![];
    let mut has_object = false;

    // Deduplicate long member names in the string table and reuse earlier name
    // offsets. This especially saves space for COFF Import libraries where all
    // members have the same name.
    let mut member_names = HashMap::<&str, u64>::new();

    // UniqueTimestamps is a special case to improve debugging on Darwin:
    //
    // The Darwin linker does not link debug info into the final
    // binary. Instead, it emits entries of type N_OSO in the output
    // binary's symbol table, containing references to the linked-in
    // object files. Using that reference, the debugger can read the
    // debug data directly from the object files. Alternatively, an
    // invocation of 'dsymutil' will link the debug data from the object
    // files into a dSYM bundle, which can be loaded by the debugger,
    // instead of the object files.
    //
    // For an object file, the N_OSO entries contain the absolute path
    // path to the file, and the file's timestamp. For an object
    // included in an archive, the path is formatted like
    // "/absolute/path/to/archive.a(member.o)", and the timestamp is the
    // archive member's timestamp, rather than the archive's timestamp.
    //
    // However, this doesn't always uniquely identify an object within
    // an archive -- an archive file can have multiple entries with the
    // same filename. (This will happen commonly if the original object
    // files started in different directories.) The only way they get
    // distinguished, then, is via the timestamp. But this process is
    // unable to find the correct object file in the archive when there
    // are two files of the same name and timestamp.
    //
    // Additionally, timestamp==0 is treated specially, and causes the
    // timestamp to be ignored as a match criteria.
    //
    // That will "usually" work out okay when creating an archive not in
    // deterministic timestamp mode, because the objects will probably
    // have been created at different timestamps.
    //
    // To ameliorate this problem, in deterministic archive mode (which
    // is the default), on Darwin we will emit a unique non-zero
    // timestamp for each entry with a duplicated name. This is still
    // deterministic: the only thing affecting that timestamp is the
    // order of the files in the resultant archive.
    //
    // See also the functions that handle the lookup:
    // in lldb: ObjectContainerBSDArchive::Archive::FindObject()
    // in llvm/tools/dsymutil: BinaryHolder::GetArchiveMemberBuffers().
    let unique_timestamps = is_darwin(kind);
    let mut filename_count = HashMap::new();
    if unique_timestamps {
        for m in new_members {
            *filename_count.entry(&*m.member_name).or_insert(0) += 1;
        }
        for (_name, count) in filename_count.iter_mut() {
            *count = if *count > 1 { 1 } else { 0 };
        }
    }

    if let Some(sym_map) = sym_map {
        if let Some(is_ec) = is_ec {
            sym_map.use_ec_map = is_ec;
        } else {
            // When IsEC is not specified by the caller, use it when we have both
            // any ARM64 object (ARM64 or ARM64EC) and any EC object (ARM64EC or
            // AMD64). This may be a single ARM64EC object, but may also be separate
            // ARM64 and AMD64 objects.
            let mut have_arm64 = false;
            let mut have_ec = false;
            for m in new_members {
                if !have_arm64 {
                    have_arm64 = (m.object_reader.is_any_arm64_coff)(m.buf.as_ref().as_ref());
                }
                if !have_ec {
                    have_ec = (m.object_reader.is_ec_object_file)(m.buf.as_ref().as_ref());
                }
                if have_arm64 && have_ec {
                    sym_map.use_ec_map = true;
                    break;
                }
            }
        }
    }

    // The big archive format needs to know the offset of the previous member
    // header.
    let mut prev_offset = 0;
    let mut next_mem_head_pad_size = 0;
    let mut index = 0;
    for m in new_members {
        let mut header = Vec::new();

        let buf = m.buf.as_ref().as_ref();
        let data = if thin { &[][..] } else { buf };

        index += 1;

        // ld64 expects the members to be 8-byte aligned for 64-bit content and at
        // least 4-byte aligned for 32-bit content.  Opt for the larger encoding
        // uniformly.  This matches the behaviour with cctools and ensures that ld64
        // is happy with archives that we generate.
        let member_padding = if is_darwin(kind) {
            offset_to_alignment(u64::try_from(data.len()).unwrap(), 8)
        } else {
            0
        };
        let tail_padding =
            offset_to_alignment(u64::try_from(data.len()).unwrap() + member_padding, 2);
        let padding = &PADDING_DATA[..usize::try_from(member_padding + tail_padding).unwrap()];

        let mtime = if unique_timestamps {
            // Increment timestamp for each file of a given name.
            *filename_count.get_mut(&*m.member_name).unwrap() += 1;
            filename_count[&*m.member_name] - 1
        } else {
            m.mtime
        };

        let size = u64::try_from(buf.len()).unwrap() + member_padding;
        if size > MAX_MEMBER_SIZE {
            return Err(io::Error::other(format!(
                "Archive member {} is too big",
                m.member_name
            )));
        }

        // In the big archive file format, we need to calculate and include the next
        // member offset and previous member offset in the file member header.
        if is_aix_big_archive(kind) {
            let offset_to_mem_data =
                pos + BIG_AR_MEM_HDR_SIZE + align_to(m.member_name.len().try_into().unwrap(), 2);

            if index == 1 {
                next_mem_head_pad_size = align_to_power_of2(
                    offset_to_mem_data,
                    (m.object_reader.get_xcoff_member_alignment)(buf).into(),
                ) - offset_to_mem_data;
            }

            mem_head_pad_size = next_mem_head_pad_size;
            pos += mem_head_pad_size;
            let mut next_offset = pos
                + BIG_AR_MEM_HDR_SIZE
                + align_to(u64::try_from(m.member_name.len()).unwrap(), 2)
                + align_to(size, 2);

            // If there is another member file after this, we need to calculate the
            // padding before the header.
            if index != new_members.len() {
                let offset_to_next_mem_data = next_offset
                    + BIG_AR_MEM_HDR_SIZE
                    + align_to(new_members[index].member_name.len().try_into().unwrap(), 2);
                next_mem_head_pad_size = align_to_power_of2(
                    offset_to_next_mem_data,
                    (m.object_reader.get_xcoff_member_alignment)(
                        new_members[index].buf.as_ref().as_ref(),
                    )
                    .into(),
                ) - offset_to_next_mem_data;
                next_offset += next_mem_head_pad_size;
            }

            print_big_archive_member_header(
                &mut header,
                &m.member_name,
                mtime,
                m.uid,
                m.gid,
                m.perms,
                size,
                prev_offset,
                next_offset,
            )?;
            prev_offset = pos;
        } else {
            print_member_header(
                &mut header,
                pos,
                string_table,
                &mut member_names,
                kind,
                thin,
                m,
                mtime,
                size,
            )?;
        }

        let symbols = write_symbols(
            buf,
            index.try_into().unwrap(),
            sym_names,
            sym_map,
            m.object_reader,
        )?;
        has_object = true;

        pos += u64::try_from(header.len() + data.len() + padding.len()).unwrap();
        ret.push(MemberData {
            symbols,
            header,
            data,
            padding,
            pre_head_pad_size: mem_head_pad_size,
            object_reader: m.object_reader,
        })
    }

    // If there are no symbols, emit an empty symbol table, to satisfy Solaris
    // tools, older versions of which expect a symbol table in a non-empty
    // archive, regardless of whether there are any symbols in it.
    if has_object && sym_names.stream_position()? == 0 && !is_coff_archive(kind) {
        write!(sym_names, "\0\0\0")?;
    }

    Ok(ret)
}

pub fn write_archive_to_stream<'a, W: Write + Seek>(
    w: &mut W,
    new_members: &'a [NewArchiveMember<'a>],
    mut kind: ArchiveKind,
    thin: bool,
    is_ec: Option<bool>,
) -> io::Result<()> {
    assert!(
        !thin || !is_bsd_like(kind),
        "Only the gnu format has a thin mode"
    );

    let mut sym_names = Cursor::new(Vec::new());
    let mut string_table = Cursor::new(Vec::new());
    let mut sym_map = SymMap::default();

    // COFF symbol map uses 16-bit indexes, so we can't use it if there are too
    // many members.
    if is_coff_archive(kind) && new_members.len() > 0xfffe {
        kind = ArchiveKind::Gnu;
    }

    let data = compute_member_data(
        &mut string_table,
        &mut sym_names,
        kind,
        thin,
        &mut is_coff_archive(kind).then_some(&mut sym_map),
        new_members,
        is_ec,
    )?;

    let sym_names = sym_names.into_inner();

    let mut string_table_size: u64 = 0;
    let mut string_table_member = None;
    let string_table = string_table.into_inner();
    if !string_table.is_empty() && !is_aix_big_archive(kind) {
        let string_table_temp = compute_string_table(&string_table);
        string_table_size = (string_table_temp.header.len()
            + string_table_temp.data.len()
            + string_table_temp.padding.len())
        .try_into()
        .unwrap();
        string_table_member = Some(string_table_temp);
    }

    // We would like to detect if we need to switch to a 64-bit symbol table.
    let mut last_member_end_offset = 0;
    let mut last_member_header_offset = 0;
    let mut num_syms = 0;
    let mut num_syms32: u64 = 0; // Store symbol number of 32-bit member files.

    for m in &data {
        // Record the start of the member's offset
        last_member_end_offset += m.pre_head_pad_size;
        last_member_header_offset = last_member_end_offset;
        // Account for the size of each part associated with the member.
        last_member_end_offset +=
            u64::try_from(m.header.len() + m.data.len() + m.padding.len()).unwrap();
        num_syms += u64::try_from(m.symbols.len()).unwrap();

        // AIX big archive files may contain two global symbol tables. The
        // first global symbol table locates 32-bit file members that define global
        // symbols; the second global symbol table does the same for 64-bit file
        // members. As a big archive can have both 32-bit and 64-bit file members,
        // we need to know the number of symbols in each symbol table individually.
        if is_aix_big_archive(kind) && !(m.object_reader.is_64_bit_object_file)(m.data) {
            num_syms32 = num_syms32
                .checked_add(m.symbols.len().try_into().unwrap())
                .unwrap();
        }
    }

    let mut maybe_headers_size = None;

    // The symbol table is put at the end of the big archive file. The symbol
    // table is at the start of the archive file for other archive formats.
    if !is_64bit_kind(kind) {
        // We assume 32-bit offsets to see if 32-bit symbols are possible or not.
        maybe_headers_size = Some(compute_headers_size(
            kind,
            data.len(),
            string_table_size,
            num_syms,
            sym_names.len().try_into().unwrap(),
            is_coff_archive(kind).then_some(&sym_map),
        )?);

        // The SYM64 format is used when an archive's member offsets are larger than
        // 32-bits can hold. The need for this shift in format is detected by
        // writeArchive. To test this we need to generate a file with a member that
        // has an offset larger than 32-bits but this demands a very slow test. To
        // speed the test up we use this environment variable to pretend like the
        // cutoff happens before 32-bits and instead happens at some much smaller
        // value.
        // FIXME allow lowering the threshold for tests
        const SYM64_THRESHOLD: u64 = 1 << 32;

        // If LastMemberHeaderOffset isn't going to fit in a 32-bit varible we need
        // to switch to 64-bit. Note that the file can be larger than 4GB as long as
        // the last member starts before the 4GB offset.
        if maybe_headers_size.unwrap() + last_member_header_offset >= SYM64_THRESHOLD {
            if kind == ArchiveKind::Darwin {
                kind = ArchiveKind::Darwin64;
            } else {
                kind = ArchiveKind::Gnu64;
            }
            maybe_headers_size = None;
        }
    }

    if thin {
        write!(w, "!<thin>\n")?;
    } else if is_aix_big_archive(kind) {
        write!(w, "<bigaf>\n")?;
    } else {
        write!(w, "!<arch>\n")?;
    }

    let headers_size;
    if !is_aix_big_archive(kind) {
        headers_size = if let Some(headers_size) = maybe_headers_size {
            headers_size
        } else {
            compute_headers_size(
                kind,
                data.len(),
                string_table_size,
                num_syms,
                sym_names.len().try_into().unwrap(),
                is_coff_archive(kind).then_some(&sym_map),
            )?
        };
        write_symbol_table(
            w,
            kind,
            &data,
            &sym_names,
            headers_size,
            num_syms,
            0,
            0,
            false,
        )?;

        if is_coff_archive(kind) {
            write_symbol_map(w, kind, &data, &sym_map, headers_size)?;
        }

        if string_table_size > 0 {
            let string_table_member = string_table_member.unwrap();
            w.write_all(&string_table_member.header)?;
            w.write_all(string_table_member.data)?;
            w.write_all(string_table_member.padding)?;
        }

        if !sym_map.ec_map.is_empty() {
            write_ec_symbols(w, &sym_map)?;
        }

        for m in data {
            w.write_all(&m.header)?;
            w.write_all(m.data)?;
            w.write_all(m.padding)?;
        }
    } else {
        headers_size = u64::try_from(std::mem::size_of::<big_archive::FixLenHdr>()).unwrap();
        last_member_end_offset += headers_size;
        last_member_header_offset += headers_size;

        // For the big archive (AIX) format, compute a table of member names and
        // offsets, used in the member table.
        let mut member_table_name_str_tbl_size = 0;
        let mut member_offsets = vec![];
        let mut member_names = vec![];

        // Loop across object to find offset and names.
        let mut member_end_offset =
            u64::try_from(std::mem::size_of::<big_archive::FixLenHdr>()).unwrap();
        for i in 0..new_members.len() {
            let member = &new_members[i];
            member_table_name_str_tbl_size += member.member_name.len() + 1;
            member_end_offset += data[i].pre_head_pad_size;
            member_offsets.push(member_end_offset);
            member_names.push(&member.member_name);
            // File member name ended with "`\n". The length is included in
            // BigArMemHdrType.
            member_end_offset += BIG_AR_MEM_HDR_SIZE
                + align_to(u64::try_from(data[i].data.len()).unwrap(), 2)
                + align_to(u64::try_from(member.member_name.len()).unwrap(), 2);
        }

        // AIX member table size.
        let member_table_size =
            u64::try_from(20 + 20 * member_offsets.len() + member_table_name_str_tbl_size).unwrap();

        let mut sym_names32 = Cursor::new(Vec::new());
        let mut sym_names64 = Cursor::new(Vec::new());

        if num_syms > 0 {
            // Generate the symbol names for the members.
            for m in &data {
                write_symbols(
                    m.data,
                    0,
                    if (m.object_reader.is_64_bit_object_file)(m.data) {
                        &mut sym_names64
                    } else {
                        &mut sym_names32
                    },
                    &mut None,
                    m.object_reader,
                )?;
            }
        }

        let member_table_end_offset =
            last_member_end_offset + align_to(BIG_AR_MEM_HDR_SIZE + member_table_size, 2);

        // In AIX OS, The 'GlobSymOffset' field in the fixed-length header contains
        // the offset to the 32-bit global symbol table, and the 'GlobSym64Offset'
        // contains the offset to the 64-bit global symbol table.
        let global_symbol_offset = if num_syms32 > 0 {
            member_table_end_offset
        } else {
            0
        };

        let mut global_symbol_offset64 = 0;
        let num_syms64 = num_syms - num_syms32;
        if num_syms64 > 0 {
            if global_symbol_offset == 0 {
                global_symbol_offset64 = member_table_end_offset;
            } else {
                // If there is a global symbol table for 32-bit members,
                // the 64-bit global symbol table is after the 32-bit one.
                global_symbol_offset64 = global_symbol_offset
                    + BIG_AR_MEM_HDR_SIZE
                    + (num_syms32 + 1) * 8
                    + align_to(sym_names32.get_ref().len().try_into().unwrap(), 2);
            }
        }

        // Fixed Sized Header.
        // Offset to member table
        write!(
            w,
            "{:<20}",
            if !new_members.is_empty() {
                last_member_end_offset
            } else {
                0
            }
        )?;
        // If there are no file members in the archive, there will be no global
        // symbol table.
        write!(w, "{global_symbol_offset:<20}")?;
        write!(w, "{global_symbol_offset64:<20}")?;
        // Offset to first archive member
        write!(
            w,
            "{:<20}",
            if !new_members.is_empty() {
                u64::try_from(std::mem::size_of::<big_archive::FixLenHdr>()).unwrap()
                    + data[0].pre_head_pad_size
            } else {
                0
            }
        )?;
        // Offset to last archive member
        write!(
            w,
            "{:<20}",
            if !new_members.is_empty() {
                last_member_header_offset
            } else {
                0
            }
        )?;
        // Offset to first member of free list - Not supported yet
        write!(w, "{:<20}", 0)?;

        for m in &data {
            write!(
                w,
                "{nil:\0<pad$}",
                nil = "",
                pad = usize::try_from(m.pre_head_pad_size).unwrap()
            )?;
            w.write_all(&m.header)?;
            w.write_all(m.data)?;
            if m.data.len() % 2 != 0 {
                w.write_all(&[0])?;
            }
        }

        if !new_members.is_empty() {
            // Member table.
            print_big_archive_member_header(
                w,
                "",
                0,
                0,
                0,
                0,
                member_table_size,
                last_member_header_offset,
                if global_symbol_offset != 0 {
                    global_symbol_offset
                } else {
                    global_symbol_offset64
                },
            )?;
            write!(w, "{:<20}", member_offsets.len())?; // Number of members
            for member_offset in member_offsets {
                write!(w, "{member_offset:<20}")?;
            }
            for member_name in member_names {
                w.write_all(member_name.as_bytes())?;
                w.write_all(&[0])?;
            }

            if member_table_name_str_tbl_size % 2 != 0 {
                // Name table must be tail padded to an even number of
                // bytes.
                w.write_all(&[0])?;
            }

            // Write global symbol table for 32-bit file members.
            if global_symbol_offset != 0 {
                write_symbol_table(
                    w,
                    kind,
                    &data,
                    sym_names32.get_ref(),
                    headers_size,
                    num_syms32,
                    last_member_end_offset,
                    global_symbol_offset64,
                    false,
                )?;
                // Add padding between the symbol tables, if needed.
                if global_symbol_offset64 != 0 && (sym_names32.get_ref().len() % 2) != 0 {
                    w.write_all(&[0])?;
                }
            }

            // Write global symbol table for 64-bit file members.
            if global_symbol_offset64 != 0 {
                write_symbol_table(
                    w,
                    kind,
                    &data,
                    sym_names64.get_ref(),
                    headers_size,
                    num_syms64,
                    if global_symbol_offset != 0 {
                        global_symbol_offset
                    } else {
                        last_member_end_offset
                    },
                    0,
                    true,
                )?;
            }
        }
    }

    w.flush()
}
