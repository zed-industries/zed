use {
    super::{auxv::AuxvType, module_reader::ModuleReaderError, serializers::*},
    crate::serializers::*,
    byteorder::{NativeEndian, ReadBytesExt},
    goblin::elf,
    memmap2::{Mmap, MmapOptions},
    procfs_core::process::{MMPermissions, MMapPath, MemoryMaps},
    std::{
        ffi::{OsStr, OsString},
        fs::File,
        mem::size_of,
        os::unix::ffi::{OsStrExt, OsStringExt},
        path::PathBuf,
    },
};

pub const LINUX_GATE_LIBRARY_NAME: &str = "linux-gate.so";
pub const DELETED_SUFFIX: &[u8] = b" (deleted)";

type Result<T> = std::result::Result<T, MapsReaderError>;

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub struct SystemMappingInfo {
    pub start_address: usize,
    pub end_address: usize,
}

// One of these is produced for each mapping in the process (i.e. line in
// /proc/$x/maps).
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub struct MappingInfo {
    // On Android, relocation packing can mean that the reported start
    // address of the mapping must be adjusted by a bias in order to
    // compensate for the compression of the relocation section. The
    // following two members hold (after LateInit) the adjusted mapping
    // range. See crbug.com/606972 for more information.
    pub start_address: usize,
    pub size: usize,
    // When Android relocation packing causes |start_addr| and |size| to
    // be modified with a load bias, we need to remember the unbiased
    // address range. The following structure holds the original mapping
    // address range as reported by the operating system.
    pub system_mapping_info: SystemMappingInfo,
    pub offset: usize,              // offset into the backed file.
    pub permissions: MMPermissions, // read, write and execute permissions.
    pub name: Option<OsString>,
    // pub elf_obj: Option<elf::Elf>,
}

#[derive(Debug)]
pub struct MappingEntry {
    pub mapping: MappingInfo,
    pub identifier: Vec<u8>,
}

// A list of <MappingInfo, GUID>
pub type MappingList = Vec<MappingEntry>;

#[derive(thiserror::Error, Debug, serde::Serialize)]
pub enum MapsReaderError {
    #[error("Couldn't parse as ELF file")]
    ELFParsingFailed(
        #[from]
        #[serde(serialize_with = "serialize_goblin_error")]
        goblin::error::Error,
    ),
    #[error("No soname found (filename: {})", .0.to_string_lossy())]
    NoSoName(OsString, #[source] ModuleReaderError),

    // parse_from_line()
    #[error("Map entry malformed: No {0} found")]
    MapEntryMalformed(&'static str),
    #[error("Couldn't parse address")]
    UnparsableInteger(
        #[from]
        #[serde(skip)]
        std::num::ParseIntError,
    ),
    #[error("Linux gate location doesn't fit in the required integer type")]
    LinuxGateNotConvertable(
        #[from]
        #[serde(skip)]
        std::num::TryFromIntError,
    ),

    // get_mmap()
    #[error("Not safe to open mapping {}", .0.to_string_lossy())]
    NotSafeToOpenMapping(OsString),
    #[error("IO Error")]
    FileError(
        #[from]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[error("Mmapped file empty or not an ELF file")]
    MmapSanityCheckFailed,
    #[error("Symlink does not match ({0} vs. {1})")]
    SymlinkError(std::path::PathBuf, std::path::PathBuf),
}

fn is_mapping_a_path(pathname: Option<&OsStr>) -> bool {
    match pathname {
        Some(x) => x.as_bytes().contains(&b'/'),
        None => false,
    }
}

/// Sanitize mapped paths.
///
/// This removes a ` (deleted)` suffix, if present.
fn sanitize_path(pathname: OsString) -> OsString {
    if let Some(bytes) = pathname.as_bytes().strip_suffix(DELETED_SUFFIX) {
        OsString::from_vec(bytes.to_owned())
    } else {
        pathname
    }
}

impl MappingInfo {
    /// Return whether the `name` field is a path (contains a `/`).
    pub fn name_is_path(&self) -> bool {
        is_mapping_a_path(self.name.as_deref())
    }

    pub fn is_empty_page(&self) -> bool {
        (self.offset == 0) && (self.permissions == MMPermissions::PRIVATE) && self.name.is_none()
    }

    pub fn end_address(&self) -> usize {
        self.start_address + self.size
    }

    pub fn aggregate(
        memory_maps: MemoryMaps,
        linux_gate_loc: Option<AuxvType>,
    ) -> Result<Vec<Self>> {
        let mut infos = Vec::<Self>::new();

        for mm in memory_maps {
            let start_address: usize = mm.address.0.try_into()?;
            let end_address: usize = mm.address.1.try_into()?;
            let mut offset: usize = mm.offset.try_into()?;

            let mut pathname: Option<OsString> = match mm.pathname {
                MMapPath::Path(p) => Some(sanitize_path(p.into())),
                MMapPath::Heap => Some("[heap]".into()),
                MMapPath::Stack => Some("[stack]".into()),
                MMapPath::TStack(i) => Some(format!("[stack:{i}]").into()),
                MMapPath::Vdso => Some("[vdso]".into()),
                MMapPath::Vvar => Some("[vvar]".into()),
                MMapPath::Vsyscall => Some("[vsyscall]".into()),
                MMapPath::Rollup => Some("[rollup]".into()),
                MMapPath::Vsys(i) => Some(format!("/SYSV{i:x}").into()),
                MMapPath::Other(n) => Some(format!("[{n}]").into()),
                MMapPath::Anonymous => None,
            };

            let is_path = is_mapping_a_path(pathname.as_deref());

            if let Some(linux_gate_loc) = linux_gate_loc.map(|u| usize::try_from(u).unwrap()) {
                if !is_path && start_address == linux_gate_loc {
                    pathname = Some(LINUX_GATE_LIBRARY_NAME.into());
                    offset = 0;
                }
            }

            if let Some(prev_module) = infos.last_mut() {
                if (start_address == prev_module.end_address())
                    && pathname.is_some()
                    && (pathname == prev_module.name)
                {
                    // Merge adjacent mappings into one module, assuming they're a single
                    // library mapped by the dynamic linker.
                    prev_module.system_mapping_info.end_address = end_address;
                    prev_module.size = end_address - prev_module.start_address;
                    prev_module.permissions |= mm.perms;
                    continue;
                } else if (start_address == prev_module.end_address())
                    && prev_module.is_executable()
                    && prev_module.name_is_path()
                    && ((offset == 0) || (offset == prev_module.end_address()))
                    && (mm.perms == MMPermissions::PRIVATE)
                {
                    // Also merge mappings that result from address ranges that the
                    // linker reserved but which a loaded library did not use. These
                    // appear as an anonymous private mapping with no access flags set
                    // and which directly follow an executable mapping.
                    prev_module.size = end_address - prev_module.start_address;
                    continue;
                }
            }

            // Sometimes the unused ranges reserved but the linker appear within the library.
            // If we detect an empty page that is adjacent to two mappings of the same library
            // we fold the three mappings together.
            if let Some(previous_modules) = infos.rchunks_exact_mut(2).next() {
                let empty_page = if let Some(prev_module) = previous_modules.last() {
                    let prev_prev_module = previous_modules.first().unwrap();
                    prev_prev_module.name_is_path()
                        && (prev_prev_module.end_address() == prev_module.start_address)
                        && prev_module.is_empty_page()
                        && (prev_module.end_address() == start_address)
                } else {
                    false
                };

                if empty_page {
                    let prev_prev_module = previous_modules.first_mut().unwrap();

                    if pathname == prev_prev_module.name {
                        prev_prev_module.system_mapping_info.end_address = end_address;
                        prev_prev_module.size = end_address - prev_prev_module.start_address;
                        prev_prev_module.permissions |= mm.perms;
                        infos.pop();
                        continue;
                    }
                }
            }

            infos.push(MappingInfo {
                start_address,
                size: end_address - start_address,
                system_mapping_info: SystemMappingInfo {
                    start_address,
                    end_address,
                },
                offset,
                permissions: mm.perms,
                name: pathname,
            });
        }
        Ok(infos)
    }

    pub fn get_mmap(name: &Option<OsString>, offset: usize) -> Result<Mmap> {
        if !MappingInfo::is_mapped_file_safe_to_open(name) {
            return Err(MapsReaderError::NotSafeToOpenMapping(
                name.clone().unwrap_or_default(),
            ));
        }

        // Not doing this as root_prefix is always "" at the moment
        //   if (!dumper.GetMappingAbsolutePath(mapping, filename))
        let filename = name.clone().unwrap_or_default();
        let mapped_file = unsafe {
            MmapOptions::new()
                .offset(offset.try_into()?) // try_into() to work for both 32 and 64 bit
                .map(&File::open(filename)?)?
        };

        if mapped_file.is_empty() || mapped_file.len() < elf::header::SELFMAG {
            return Err(MapsReaderError::MmapSanityCheckFailed);
        }
        Ok(mapped_file)
    }

    pub fn stack_has_pointer_to_mapping(&self, stack_copy: &[u8], sp_offset: usize) -> bool {
        // Loop over all stack words that would have been on the stack in
        // the target process (i.e. are word aligned, and at addresses >=
        // the stack pointer).  Regardless of the alignment of |stack_copy|,
        // the memory starting at |stack_copy| + |offset| represents an
        // aligned word in the target process.
        let low_addr = self.system_mapping_info.start_address;
        let high_addr = self.system_mapping_info.end_address;
        let mut offset = (sp_offset + size_of::<usize>() - 1) & !(size_of::<usize>() - 1);
        while offset <= stack_copy.len() - size_of::<usize>() {
            let addr = match std::mem::size_of::<usize>() {
                4 => stack_copy[offset..]
                    .as_ref()
                    .read_u32::<NativeEndian>()
                    .map(|u| u as usize),
                8 => stack_copy[offset..]
                    .as_ref()
                    .read_u64::<NativeEndian>()
                    .map(|u| u as usize),
                x => panic!("Unexpected type width: {x}"),
            };
            if let Ok(addr) = addr {
                if low_addr <= addr && addr <= high_addr {
                    return true;
                }
                offset += size_of::<usize>();
            } else {
                break;
            }
        }
        false
    }

    pub fn is_mapped_file_safe_to_open(name: &Option<OsString>) -> bool {
        // It is unsafe to attempt to open a mapped file that lives under /dev,
        // because the semantics of the open may be driver-specific so we'd risk
        // hanging the crash dumper. And a file in /dev/ almost certainly has no
        // ELF file identifier anyways.
        if let Some(name) = name {
            if name.as_bytes().starts_with(b"/dev/") {
                return false;
            }
        }
        true
    }

    /// Find the shared object name (SONAME) by examining the ELF information
    /// for the mapping.
    fn so_name(&self) -> Result<String> {
        use super::module_reader::{ReadFromModule, SoName};

        let mapped_file = MappingInfo::get_mmap(&self.name, self.offset)?;
        Ok(SoName::read_from_module((&*mapped_file).into())
            .map_err(|e| MapsReaderError::NoSoName(self.name.clone().unwrap_or_default(), e))?
            .0
            .to_string())
    }

    #[inline]
    fn so_version(&self) -> Option<SoVersion> {
        SoVersion::parse(self.name.as_deref()?)
    }

    pub fn get_mapping_effective_path_name_and_version(
        &self,
        soname: Option<String>,
    ) -> Result<(PathBuf, String, Option<SoVersion>)> {
        let mut file_path = PathBuf::from(self.name.clone().unwrap_or_default());

        // Tools such as minidump_stackwalk use the name of the module to look up
        // symbols produced by dump_syms. dump_syms will prefer to use a module's
        // DT_SONAME as the module name, if one exists, and will fall back to the
        // filesystem name of the module.

        // Just use the filesystem name if no SONAME is present.
        let Some(file_name) = soname.or_else(|| self.so_name().ok()) else {
            //   file_path := /path/to/libname.so
            //   file_name := libname.so
            let file_name = file_path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();

            return Ok((file_path, file_name, self.so_version()));
        };

        if self.is_executable() && self.offset != 0 {
            // If an executable is mapped from a non-zero offset, this is likely because
            // the executable was loaded directly from inside an archive file (e.g., an
            // apk on Android).
            // In this case, we append the file_name to the mapped archive path:
            //   file_name := libname.so
            //   file_path := /path/to/ARCHIVE.APK/libname.so
            file_path.push(&file_name);
        } else {
            // Otherwise, replace the basename with the SONAME.
            file_path.set_file_name(&file_name);
        }

        Ok((file_path, file_name, self.so_version()))
    }

    pub fn is_contained_in(&self, user_mapping_list: &MappingList) -> bool {
        for user in user_mapping_list {
            // Ignore any mappings that are wholly contained within
            // mappings in the mapping_info_ list.
            if self.start_address >= user.mapping.start_address
                && (self.start_address + self.size)
                    <= (user.mapping.start_address + user.mapping.size)
            {
                return true;
            }
        }
        false
    }

    pub fn is_interesting(&self) -> bool {
        // only want modules with filenames.
        self.name.is_some() &&
        // Only want to include one mapping per shared lib.
        // Avoid filtering executable mappings.
        (self.offset == 0 || self.is_executable()) &&
        // big enough to get a signature for.
        self.size >= 4096
    }

    pub fn contains_address(&self, address: usize) -> bool {
        self.system_mapping_info.start_address <= address
            && address < self.system_mapping_info.end_address
    }

    pub fn is_executable(&self) -> bool {
        self.permissions.contains(MMPermissions::EXECUTE)
    }

    pub fn is_readable(&self) -> bool {
        self.permissions.contains(MMPermissions::READ)
    }

    pub fn is_writable(&self) -> bool {
        self.permissions.contains(MMPermissions::WRITE)
    }
}

/// Version metadata retrieved from an .so filename
///
/// There is no standard for .so version numbers so this implementation just
/// does a best effort to pull as much data as it can based on real .so schemes
/// seen
///
/// That being said, the [libtool](https://www.gnu.org/software/libtool/manual/html_node/Libtool-versioning.html)
/// versioning scheme is fairly common
#[cfg_attr(test, derive(Debug))]
pub struct SoVersion {
    /// Might be non-zero if there is at least one non-zero numeric component after .so.
    ///
    /// Equivalent to `current` in libtool versions
    pub major: u32,
    /// The numeric component after the major version, if any
    ///
    /// Equivalent to `revision` in libtool versions
    pub minor: u32,
    /// The numeric component after the minor version, if any
    ///
    /// Equivalent to `age` in libtool versions
    pub patch: u32,
    /// The patch component may contain additional non-numeric metadata similar
    /// to a semver prelease, this is any numeric data that suffixes that prerelease
    /// string
    pub prerelease: u32,
}

impl SoVersion {
    /// Attempts to retrieve the .so version of the elf path via its filename
    fn parse(so_path: &OsStr) -> Option<Self> {
        let filename = std::path::Path::new(so_path).file_name()?;

        // Avoid an allocation unless the string contains non-utf8
        let filename = filename.to_string_lossy();

        let (_, version) = filename.split_once(".so.")?;

        let mut sov = Self {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: 0,
        };

        let comps = [
            &mut sov.major,
            &mut sov.minor,
            &mut sov.patch,
            &mut sov.prerelease,
        ];

        for (i, comp) in version.split('.').enumerate() {
            if i <= 1 {
                *comps[i] = comp.parse().unwrap_or_default();
            } else if i >= 4 {
                break;
            } else {
                // In some cases the release/patch version is alphanumeric (eg. '2rc5'),
                // so try to parse either a single or two numbers
                if let Some(pend) = comp.find(|c: char| !c.is_ascii_digit()) {
                    if let Ok(patch) = comp[..pend].parse() {
                        *comps[i] = patch;
                    }

                    if i >= comps.len() - 1 {
                        break;
                    }
                    if let Some(pre) = comp.rfind(|c: char| !c.is_ascii_digit()) {
                        if let Ok(pre) = comp[pre + 1..].parse() {
                            *comps[i + 1] = pre;
                            break;
                        }
                    }
                } else {
                    *comps[i] = comp.parse().unwrap_or_default();
                }
            }
        }

        Some(sov)
    }
}

#[cfg(test)]
impl PartialEq<(u32, u32, u32, u32)> for SoVersion {
    fn eq(&self, o: &(u32, u32, u32, u32)) -> bool {
        self.major == o.0 && self.minor == o.1 && self.patch == o.2 && self.prerelease == o.3
    }
}

#[cfg(test)]
#[cfg(target_pointer_width = "64")] // All addresses are 64 bit and I'm currently too lazy to adjust it to work for both
mod tests {
    use super::*;
    use procfs_core::FromRead;

    fn get_mappings_for(map: &str, linux_gate_loc: u64) -> Vec<MappingInfo> {
        MappingInfo::aggregate(
            MemoryMaps::from_read(map.as_bytes()).expect("failed to read mapping info"),
            Some(linux_gate_loc),
        )
        .unwrap_or_default()
    }

    const LINES: &str = "\
5597483fc000-5597483fe000 r--p 00000000 00:31 4750073                    /usr/bin/cat
5597483fe000-559748402000 r-xp 00002000 00:31 4750073                    /usr/bin/cat
559748402000-559748404000 r--p 00006000 00:31 4750073                    /usr/bin/cat
559748404000-559748405000 r--p 00007000 00:31 4750073                    /usr/bin/cat
559748405000-559748406000 rw-p 00008000 00:31 4750073                    /usr/bin/cat
559749b0e000-559749b2f000 rw-p 00000000 00:00 0                          [heap]
7efd968d3000-7efd968f5000 rw-p 00000000 00:00 0 
7efd968f5000-7efd9694a000 r--p 00000000 00:31 5004638                    /usr/lib/locale/en_US.utf8/LC_CTYPE
7efd9694a000-7efd96bc2000 r--p 00000000 00:31 5004373                    /usr/lib/locale/en_US.utf8/LC_COLLATE
7efd96bc2000-7efd96bc4000 rw-p 00000000 00:00 0 
7efd96bc4000-7efd96bea000 r--p 00000000 00:31 4996104                    /lib64/libc-2.32.so
7efd96bea000-7efd96d39000 r-xp 00026000 00:31 4996104                    /lib64/libc-2.32.so
7efd96d39000-7efd96d85000 r--p 00175000 00:31 4996104                    /lib64/libc-2.32.so
7efd96d85000-7efd96d86000 ---p 001c1000 00:31 4996104                    /lib64/libc-2.32.so
7efd96d86000-7efd96d89000 r--p 001c1000 00:31 4996104                    /lib64/libc-2.32.so
7efd96d89000-7efd96d8c000 rw-p 001c4000 00:31 4996104                    /lib64/libc-2.32.so
7efd96d8c000-7efd96d92000 ---p 00000000 00:00 0 
7efd96da0000-7efd96da1000 r--p 00000000 00:31 5004379                    /usr/lib/locale/en_US.utf8/LC_NUMERIC
7efd96da1000-7efd96da2000 r--p 00000000 00:31 5004382                    /usr/lib/locale/en_US.utf8/LC_TIME
7efd96da2000-7efd96da3000 r--p 00000000 00:31 5004377                    /usr/lib/locale/en_US.utf8/LC_MONETARY
7efd96da3000-7efd96da4000 r--p 00000000 00:31 5004376                    /usr/lib/locale/en_US.utf8/LC_MESSAGES/SYS_LC_MESSAGES
7efd96da4000-7efd96da5000 r--p 00000000 00:31 5004380                    /usr/lib/locale/en_US.utf8/LC_PAPER
7efd96da5000-7efd96da6000 r--p 00000000 00:31 5004378                    /usr/lib/locale/en_US.utf8/LC_NAME
7efd96da6000-7efd96da7000 r--p 00000000 00:31 5004372                    /usr/lib/locale/en_US.utf8/LC_ADDRESS
7efd96da7000-7efd96da8000 r--p 00000000 00:31 5004381                    /usr/lib/locale/en_US.utf8/LC_TELEPHONE
7efd96da8000-7efd96da9000 r--p 00000000 00:31 5004375                    /usr/lib/locale/en_US.utf8/LC_MEASUREMENT
7efd96da9000-7efd96db0000 r--s 00000000 00:31 5004639                    /usr/lib64/gconv/gconv-modules.cache
7efd96db0000-7efd96db1000 r--p 00000000 00:31 5004374                    /usr/lib/locale/en_US.utf8/LC_IDENTIFICATION
7efd96db1000-7efd96db2000 r--p 00000000 00:31 4996100                    /lib64/ld-2.32.so
7efd96db2000-7efd96dd3000 r-xp 00001000 00:31 4996100                    /lib64/ld-2.32.so
7efd96dd3000-7efd96ddc000 r--p 00022000 00:31 4996100                    /lib64/ld-2.32.so
7efd96ddc000-7efd96ddd000 r--p 0002a000 00:31 4996100                    /lib64/ld-2.32.so
7efd96ddd000-7efd96ddf000 rw-p 0002b000 00:31 4996100                    /lib64/ld-2.32.so
7ffc6dfda000-7ffc6dffb000 rw-p 00000000 00:00 0                          [stack]
7ffc6e0f3000-7ffc6e0f7000 r--p 00000000 00:00 0                          [vvar]
7ffc6e0f7000-7ffc6e0f9000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]";
    const LINUX_GATE_LOC: u64 = 0x7ffc6e0f7000;

    fn get_all_mappings() -> Vec<MappingInfo> {
        get_mappings_for(LINES, LINUX_GATE_LOC)
    }

    #[test]
    fn test_merged() {
        // Only /usr/bin/cat and [heap]
        let mappings = get_mappings_for(
            "\
5597483fc000-5597483fe000 r--p 00000000 00:31 4750073                    /usr/bin/cat
5597483fe000-559748402000 r-xp 00002000 00:31 4750073                    /usr/bin/cat
559748402000-559748404000 r--p 00006000 00:31 4750073                    /usr/bin/cat
559748404000-559748405000 r--p 00007000 00:31 4750073                    /usr/bin/cat
559748405000-559748406000 rw-p 00008000 00:31 4750073                    /usr/bin/cat
559749b0e000-559749b2f000 rw-p 00000000 00:00 0                          [heap]
7efd968d3000-7efd968f5000 rw-p 00000000 00:00 0 ",
            0x7ffc6e0f7000,
        );

        assert_eq!(mappings.len(), 3);
        let cat_map = MappingInfo {
            start_address: 0x5597483fc000,
            size: 40960,
            system_mapping_info: SystemMappingInfo {
                start_address: 0x5597483fc000,
                end_address: 0x559748406000,
            },
            offset: 0,
            permissions: MMPermissions::READ
                | MMPermissions::WRITE
                | MMPermissions::EXECUTE
                | MMPermissions::PRIVATE,
            name: Some("/usr/bin/cat".into()),
        };

        assert_eq!(mappings[0], cat_map);

        let heap_map = MappingInfo {
            start_address: 0x559749b0e000,
            size: 135168,
            system_mapping_info: SystemMappingInfo {
                start_address: 0x559749b0e000,
                end_address: 0x559749b2f000,
            },
            offset: 0,
            permissions: MMPermissions::READ | MMPermissions::WRITE | MMPermissions::PRIVATE,
            name: Some("[heap]".into()),
        };

        assert_eq!(mappings[1], heap_map);

        let empty_map = MappingInfo {
            start_address: 0x7efd968d3000,
            size: 139264,
            system_mapping_info: SystemMappingInfo {
                start_address: 0x7efd968d3000,
                end_address: 0x7efd968f5000,
            },
            offset: 0,
            permissions: MMPermissions::READ | MMPermissions::WRITE | MMPermissions::PRIVATE,
            name: None,
        };

        assert_eq!(mappings[2], empty_map);
    }

    #[test]
    fn test_linux_gate_parsing() {
        let mappings = get_all_mappings();

        let gate_map = MappingInfo {
            start_address: 0x7ffc6e0f7000,
            size: 8192,
            system_mapping_info: SystemMappingInfo {
                start_address: 0x7ffc6e0f7000,
                end_address: 0x7ffc6e0f9000,
            },
            offset: 0,
            permissions: MMPermissions::READ | MMPermissions::EXECUTE | MMPermissions::PRIVATE,
            name: Some("linux-gate.so".into()),
        };

        assert_eq!(mappings[21], gate_map);
    }

    #[test]
    fn test_reading_all() {
        let mappings = get_all_mappings();

        let found_items: Vec<Option<OsString>> = vec![
            Some("/usr/bin/cat".into()),
            Some("[heap]".into()),
            None,
            Some("/usr/lib/locale/en_US.utf8/LC_CTYPE".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_COLLATE".into()),
            None,
            Some("/lib64/libc-2.32.so".into()),
            // The original shows a None here, but this is an address ranges that the
            // linker reserved but which a loaded library did not use. These
            // appear as an anonymous private mapping with no access flags set
            // and which directly follow an executable mapping.
            Some("/usr/lib/locale/en_US.utf8/LC_NUMERIC".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_TIME".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_MONETARY".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_MESSAGES/SYS_LC_MESSAGES".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_PAPER".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_NAME".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_ADDRESS".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_TELEPHONE".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_MEASUREMENT".into()),
            Some("/usr/lib64/gconv/gconv-modules.cache".into()),
            Some("/usr/lib/locale/en_US.utf8/LC_IDENTIFICATION".into()),
            Some("/lib64/ld-2.32.so".into()),
            Some("[stack]".into()),
            Some("[vvar]".into()),
            // This is rewritten from [vdso] to linux-gate.so
            Some("linux-gate.so".into()),
            Some("[vsyscall]".into()),
        ];

        assert_eq!(
            mappings.iter().map(|x| x.name.clone()).collect::<Vec<_>>(),
            found_items
        );
    }

    #[test]
    fn test_merged_reserved_mappings() {
        let mappings = get_all_mappings();

        let gate_map = MappingInfo {
            start_address: 0x7efd96bc4000,
            size: 1892352, // Merged the anonymous area after in this mapping, so its bigger..
            system_mapping_info: SystemMappingInfo {
                start_address: 0x7efd96bc4000,
                end_address: 0x7efd96d8c000, // ..but this is not visible here
            },
            offset: 0,
            permissions: MMPermissions::READ
                | MMPermissions::WRITE
                | MMPermissions::EXECUTE
                | MMPermissions::PRIVATE,
            name: Some("/lib64/libc-2.32.so".into()),
        };

        assert_eq!(mappings[6], gate_map);
    }

    #[test]
    fn test_merged_reserved_mappings_within_module() {
        let mappings = get_mappings_for(
            "\
9b4a0000-9b931000 r--p 00000000 08:12 393449     /data/app/org.mozilla.firefox-1/lib/x86/libxul.so
9b931000-9bcae000 ---p 00000000 00:00 0 
9bcae000-a116b000 r-xp 00490000 08:12 393449     /data/app/org.mozilla.firefox-1/lib/x86/libxul.so
a116b000-a4562000 r--p 0594d000 08:12 393449     /data/app/org.mozilla.firefox-1/lib/x86/libxul.so
a4562000-a4563000 ---p 00000000 00:00 0 
a4563000-a4840000 r--p 08d44000 08:12 393449     /data/app/org.mozilla.firefox-1/lib/x86/libxul.so
a4840000-a4873000 rw-p 09021000 08:12 393449     /data/app/org.mozilla.firefox-1/lib/x86/libxul.so",
            0xa4876000,
        );

        let gate_map = MappingInfo {
            start_address: 0x9b4a0000,
            size: 155004928, // Merged the anonymous area after in this mapping, so its bigger..
            system_mapping_info: SystemMappingInfo {
                start_address: 0x9b4a0000,
                end_address: 0xa4873000,
            },
            offset: 0,
            permissions: MMPermissions::READ
                | MMPermissions::WRITE
                | MMPermissions::EXECUTE
                | MMPermissions::PRIVATE,
            name: Some("/data/app/org.mozilla.firefox-1/lib/x86/libxul.so".into()),
        };

        assert_eq!(mappings[0], gate_map);
    }

    #[test]
    fn test_get_mapping_effective_name() {
        let mappings = get_mappings_for(
            "\
7f0b97b6f000-7f0b97b70000 r--p 00000000 00:3e 27136458                   /home/martin/Documents/mozilla/devel/mozilla-central/obj/widget/gtk/mozgtk/gtk3/libmozgtk.so
7f0b97b70000-7f0b97b71000 r-xp 00000000 00:3e 27136458                   /home/martin/Documents/mozilla/devel/mozilla-central/obj/widget/gtk/mozgtk/gtk3/libmozgtk.so
7f0b97b71000-7f0b97b73000 r--p 00000000 00:3e 27136458                   /home/martin/Documents/mozilla/devel/mozilla-central/obj/widget/gtk/mozgtk/gtk3/libmozgtk.so
7f0b97b73000-7f0b97b74000 rw-p 00001000 00:3e 27136458                   /home/martin/Documents/mozilla/devel/mozilla-central/obj/widget/gtk/mozgtk/gtk3/libmozgtk.so",
            0x7ffe091bf000,
        );
        assert_eq!(mappings.len(), 1);

        let (file_path, file_name, _version) = mappings[0]
            .get_mapping_effective_path_name_and_version(None)
            .expect("Couldn't get effective name for mapping");
        assert_eq!(file_name, "libmozgtk.so");
        assert_eq!(file_path, PathBuf::from("/home/martin/Documents/mozilla/devel/mozilla-central/obj/widget/gtk/mozgtk/gtk3/libmozgtk.so"));
    }

    #[test]
    fn test_elf_file_so_version() {
        #[rustfmt::skip]
        let test_cases = [
            ("/usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32", (6, 0, 32, 0)),
            ("/usr/lib/x86_64-linux-gnu/libcairo-gobject.so.2.11800.0", (2, 11800, 0, 0)),
            ("/usr/lib/x86_64-linux-gnu/libm.so.6", (6, 0, 0, 0)),
            ("/usr/lib/x86_64-linux-gnu/libpthread.so.0", (0, 0, 0, 0)),
            ("/usr/lib/x86_64-linux-gnu/libgmodule-2.0.so.0.7800.0", (0, 7800, 0, 0)),
            ("/usr/lib/x86_64-linux-gnu/libabsl_time_zone.so.20220623.0.0", (20220623, 0, 0, 0)),
            ("/usr/lib/x86_64-linux-gnu/libdbus-1.so.3.34.2rc5", (3, 34, 2, 5)),
            ("/usr/lib/x86_64-linux-gnu/libdbus-1.so.3.34.2rc", (3, 34, 2, 0)),
            ("/usr/lib/x86_64-linux-gnu/libdbus-1.so.3.34.rc5", (3, 34, 0, 5)),
            ("/usr/lib/x86_64-linux-gnu/libtoto.so.AAA", (0, 0, 0, 0)),
            ("/usr/lib/x86_64-linux-gnu/libsemver-1.so.1.2.alpha.1", (1, 2, 0, 1)),
            ("/usr/lib/x86_64-linux-gnu/libboop.so.1.2.3.4.5", (1, 2, 3, 4)),
            ("/usr/lib/x86_64-linux-gnu/libboop.so.1.2.3pre4.5", (1, 2, 3, 4)),
        ];

        assert!(SoVersion::parse(OsStr::new("/home/alex/bin/firefox/libmozsandbox.so")).is_none());

        for (path, expected) in test_cases {
            let actual = SoVersion::parse(OsStr::new(path)).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_whitespaces_in_name() {
        let mappings = get_mappings_for(
            "\
10000000-20000000 r--p 00000000 00:3e 27136458                   libmoz    gtk.so
30000000-40000000 r--p 00000000 00:3e 27136458                   \"libmoz     gtk.so (deleted)\"
30000000-40000000 r--p 00000000 00:3e 27136458                   ",
            0x7ffe091bf000,
        );

        assert_eq!(mappings.len(), 3);
        assert_eq!(mappings[0].name, Some("libmoz    gtk.so".into()));
        assert_eq!(
            mappings[1].name,
            Some("\"libmoz     gtk.so (deleted)\"".into())
        );
        assert_eq!(mappings[2].name, None);
    }
}
