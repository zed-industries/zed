use {
    super::CpuInfoError,
    crate::minidump_format::*,
    scroll::Pwrite,
    std::{
        collections::HashSet,
        fs::File,
        io::{BufRead, BufReader, Read},
        path,
    },
};

type Result<T> = std::result::Result<T, CpuInfoError>;

pub fn parse_cpus_from_sysfile(file: &mut File) -> Result<HashSet<u32>> {
    let mut res = HashSet::new();
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    // Expected format: comma-separated list of items, where each
    // item can be a decimal integer, or two decimal integers separated
    // by a dash.
    // E.g.:
    //       0
    //       0,1,2,3
    //       0-3
    //       1,10-23
    for items in content.split(',') {
        let items = items.trim();
        if items.is_empty() {
            continue;
        }
        let cores: std::result::Result<Vec<_>, _> =
            items.split('-').map(|x| x.parse::<u32>()).collect();
        let cores = cores?;
        match cores.as_slice() {
            [x] => {
                res.insert(*x);
            }
            [x, y] => {
                for core in *x..=*y {
                    res.insert(core);
                }
            }
            _ => {
                return Err(CpuInfoError::UnparsableCores(format!("{:?}", cores)));
            }
        }
    }
    Ok(res)
}

struct CpuInfoEntry {
    field: &'static str,
    format: char,
    bit_lshift: u8,
    bit_length: u8,
}

impl CpuInfoEntry {
    fn new(field: &'static str, format: char, bit_lshift: u8, bit_length: u8) -> Self {
        CpuInfoEntry {
            field,
            format,
            bit_lshift,
            bit_length,
        }
    }
}

/// Retrieves the hardware capabilities from the 'Features' field.
#[cfg(target_arch = "arm")]
fn parse_features(val: &str) -> u32 {
    struct CpuFeaturesEntry {
        tag: &'static str,
        hwcaps: u32,
    }

    impl CpuFeaturesEntry {
        fn new(tag: &'static str, hwcaps: u32) -> Self {
            CpuFeaturesEntry { tag, hwcaps }
        }
    }

    // The ELF hwcaps are listed in the "Features" entry as textual tags.
    // This table is used to rebuild them.
    let cpu_features_entries = [
        CpuFeaturesEntry::new("swp", MDCPUInformationARMElfHwCaps::HWCAP_SWP.bits()),
        CpuFeaturesEntry::new("half", MDCPUInformationARMElfHwCaps::HWCAP_HALF.bits()),
        CpuFeaturesEntry::new("thumb", MDCPUInformationARMElfHwCaps::HWCAP_THUMB.bits()),
        CpuFeaturesEntry::new("bit26", MDCPUInformationARMElfHwCaps::HWCAP_26BIT.bits()),
        CpuFeaturesEntry::new(
            "fastmult",
            MDCPUInformationARMElfHwCaps::HWCAP_FAST_MULT.bits(),
        ),
        CpuFeaturesEntry::new("fpa", MDCPUInformationARMElfHwCaps::HWCAP_FPA.bits()),
        CpuFeaturesEntry::new("vfp", MDCPUInformationARMElfHwCaps::HWCAP_VFP.bits()),
        CpuFeaturesEntry::new("edsp", MDCPUInformationARMElfHwCaps::HWCAP_EDSP.bits()),
        CpuFeaturesEntry::new("java", MDCPUInformationARMElfHwCaps::HWCAP_JAVA.bits()),
        CpuFeaturesEntry::new("iwmmxt", MDCPUInformationARMElfHwCaps::HWCAP_IWMMXT.bits()),
        CpuFeaturesEntry::new("crunch", MDCPUInformationARMElfHwCaps::HWCAP_CRUNCH.bits()),
        CpuFeaturesEntry::new(
            "thumbee",
            MDCPUInformationARMElfHwCaps::HWCAP_THUMBEE.bits(),
        ),
        CpuFeaturesEntry::new("neon", MDCPUInformationARMElfHwCaps::HWCAP_NEON.bits()),
        CpuFeaturesEntry::new("vfpv3", MDCPUInformationARMElfHwCaps::HWCAP_VFPv3.bits()),
        CpuFeaturesEntry::new(
            "vfpv3d16",
            MDCPUInformationARMElfHwCaps::HWCAP_VFPv3D16.bits(),
        ),
        CpuFeaturesEntry::new("tls", MDCPUInformationARMElfHwCaps::HWCAP_TLS.bits()),
        CpuFeaturesEntry::new("vfpv4", MDCPUInformationARMElfHwCaps::HWCAP_VFPv4.bits()),
        CpuFeaturesEntry::new("idiva", MDCPUInformationARMElfHwCaps::HWCAP_IDIVA.bits()),
        CpuFeaturesEntry::new("idivt", MDCPUInformationARMElfHwCaps::HWCAP_IDIVT.bits()),
        CpuFeaturesEntry::new("idiv", MDCPUInformationARMElfHwCaps::HWCAP_IDIV.bits()),
    ];

    let mut ehwc = 0;
    // Parse each space-separated tag.
    for tag in val.split_whitespace() {
        for entry in &cpu_features_entries {
            if entry.tag == tag {
                ehwc |= entry.hwcaps;
                break;
            }
        }
    }

    ehwc
}

/// Stub for aarch64, always 0
#[cfg(target_arch = "aarch64")]
fn parse_features(_val: &str) -> u32 {
    0
}

pub fn write_cpu_information(sys_info: &mut MDRawSystemInfo) -> Result<()> {
    // The CPUID value is broken up in several entries in /proc/cpuinfo.
    // This table is used to rebuild it from the entries.
    let cpu_id_entries = [
        CpuInfoEntry::new("CPU implementer", 'x', 24, 8),
        CpuInfoEntry::new("CPU variant", 'x', 20, 4),
        CpuInfoEntry::new("CPU part", 'x', 4, 12),
        CpuInfoEntry::new("CPU revision", 'd', 0, 4),
    ];

    // processor_architecture should always be set, do this first
    if cfg!(target_arch = "aarch64") {
        sys_info.processor_architecture =
            MDCPUArchitecture::PROCESSOR_ARCHITECTURE_ARM64_OLD as u16;
    } else {
        sys_info.processor_architecture = MDCPUArchitecture::PROCESSOR_ARCHITECTURE_ARM as u16;
    }

    // /proc/cpuinfo is not readable under various sandboxed environments
    // (e.g. Android services with the android:isolatedProcess attribute)
    // prepare for this by setting default values now, which will be
    // returned when this happens.
    //
    // Note: Bogus values are used to distinguish between failures (to
    //       read /sys and /proc files) and really badly configured kernels.
    sys_info.number_of_processors = 0;
    sys_info.processor_level = 1; // There is no ARMv1
    sys_info.processor_revision = 42;

    // Counting the number of CPUs involves parsing two sysfs files,
    // because the content of /proc/cpuinfo will only mirror the number
    // of 'online' cores, and thus will vary with time.
    // See http://www.kernel.org/doc/Documentation/cputopology.txt
    if let Ok(mut present_file) = File::open("/sys/devices/system/cpu/present") {
        // Ignore unparsable content
        let cpus_present = parse_cpus_from_sysfile(&mut present_file).unwrap_or_default();

        if let Ok(mut possible_file) = File::open("/sys/devices/system/cpu/possible") {
            // Ignore unparsable content
            let cpus_possible = parse_cpus_from_sysfile(&mut possible_file).unwrap_or_default();
            let intersection = cpus_present.intersection(&cpus_possible).count();
            let cpu_count = std::cmp::min(255, intersection) as u8;
            sys_info.number_of_processors = cpu_count;
        }
    }

    // Parse /proc/cpuinfo to reconstruct the CPUID value, as well
    // as the ELF hwcaps field. For the latter, it would be easier to
    // read /proc/self/auxv but unfortunately, this file is not always
    // readable from regular Android applications on later versions
    // (>= 4.1) of the Android platform.

    let cpuinfo_file = match File::open(path::PathBuf::from("/proc/cpuinfo")) {
        Ok(x) => x,
        Err(_) => {
            // Do not return Error here to allow the minidump generation
            // to happen properly.
            return Ok(());
        }
    };

    let mut cpuid = 0;
    let mut elf_hwcaps = 0;

    for line in BufReader::new(cpuinfo_file).lines() {
        let line = line?;
        // Expected format: <field-name> <space>+ ':' <space> <value>
        // Note that:
        //   - empty lines happen.
        //   - <field-name> can contain spaces.
        //   - some fields have an empty <value>
        if line.trim().is_empty() {
            continue;
        }

        let (field, value) = if let Some(ind) = line.find(':') {
            (&line[..ind], Some(&line[ind + 1..]))
        } else {
            (line.as_str(), None)
        };

        if let Some(val) = value {
            for entry in &cpu_id_entries {
                if field != entry.field {
                    continue;
                }

                let rr = if val.starts_with("0x") || entry.format == 'x' {
                    usize::from_str_radix(val.trim_start_matches("0x"), 16)
                } else {
                    val.parse()
                };

                if let Ok(mut result) = rr {
                    result &= (1 << entry.bit_length) - 1;
                    result <<= entry.bit_lshift;
                    cpuid |= result as u32;
                }
            }
        }

        if cfg!(target_arch = "arm") {
            // Get the architecture version from the "Processor" field.
            // Note that it is also available in the "CPU architecture" field,
            // however, some existing kernels are misconfigured and will report
            // invalid values here (e.g. 6, while the CPU is ARMv7-A based).
            // The "Processor" field doesn't have this issue.
            if field == "Processor" {
                // Expected format: <text> (v<level><endian>)
                // Where <text> is some text like "ARMv7 Processor rev 2"
                // and <level> is a decimal corresponding to the ARM
                // architecture number. <endian> is either 'l' or 'b'
                // and corresponds to the endianess, it is ignored here.
                if let Some(val) = value.and_then(|v| v.split_whitespace().last()) {
                    // val is now something like "(v7l)"
                    sys_info.processor_level = val[2..val.len() - 2].parse::<u16>().unwrap_or(5);
                }
            }
        } else {
            // aarch64
            // The aarch64 architecture does not provide the architecture level
            // in the Processor field, so we instead check the "CPU architecture"
            // field.
            if field == "CPU architecture" {
                sys_info.processor_level = match value.and_then(|v| v.parse::<u16>().ok()) {
                    Some(v) => v,
                    None => {
                        continue;
                    }
                };
            }
        }

        // Rebuild the ELF hwcaps from the 'Features' field.
        if field == "Features" {
            if let Some(val) = value {
                elf_hwcaps = parse_features(val);
            }
        }
    }

    // The sys_info.cpu field is just a byte array, but in arm's case it is
    // actually
    // minidump_common::format::ARMCpuInfo {
    //  pub cpuid: u32,
    //  pub elf_hwcaps: u32,
    // }
    sys_info
        .cpu
        .data
        .pwrite_with(cpuid, 0, scroll::Endian::Little)
        .expect("impossible");
    sys_info
        .cpu
        .data
        .pwrite_with(
            elf_hwcaps,
            std::mem::size_of::<u32>(),
            scroll::Endian::Little,
        )
        .expect("impossible");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // In tests we can have access to std
    extern crate std;
    use std::io::Write;

    fn new_file(content: &str) -> File {
        let mut file = tempfile::Builder::new()
            .prefix("cpu_sets")
            .tempfile()
            .unwrap();
        write!(file, "{}", content).unwrap();
        std::fs::File::open(file).unwrap()
    }

    #[test]
    fn test_empty_count() {
        let mut file = new_file("");
        let set = parse_cpus_from_sysfile(&mut file).expect("Failed to parse empty file");
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_one_cpu() {
        let mut file = new_file("10");
        let set = parse_cpus_from_sysfile(&mut file).expect("Failed to file");
        assert_eq!(set, [10,].iter().copied().collect());
    }

    #[test]
    fn test_one_cpu_newline() {
        let mut file = new_file("10\n");
        let set = parse_cpus_from_sysfile(&mut file).expect("Failed to file");
        assert_eq!(set, [10,].iter().copied().collect());
    }

    #[test]
    fn test_two_cpus() {
        let mut file = new_file("1,10\n");
        let set = parse_cpus_from_sysfile(&mut file).expect("Failed to file");
        assert_eq!(set, [1, 10].iter().copied().collect());
    }

    #[test]
    fn test_two_cpus_with_range() {
        let mut file = new_file("1-2\n");
        let set = parse_cpus_from_sysfile(&mut file).expect("Failed to file");
        assert_eq!(set, [1, 2].iter().copied().collect());
    }

    #[test]
    fn test_ten_cpus_with_range() {
        let mut file = new_file("9-18\n");
        let set = parse_cpus_from_sysfile(&mut file).expect("Failed to file");
        assert_eq!(set, (9..=18).collect());
    }

    #[test]
    fn test_multiple_items() {
        let mut file = new_file("0, 2-4, 128\n");
        let set = parse_cpus_from_sysfile(&mut file).expect("Failed to file");
        assert_eq!(set, [0, 2, 3, 4, 128].iter().copied().collect());
    }

    #[test]
    fn test_intersects_with() {
        let mut file1 = new_file("9-19\n");
        let mut set1 = parse_cpus_from_sysfile(&mut file1).expect("Failed to file");
        assert_eq!(set1, (9..=19).collect());

        let mut file2 = new_file("16-24\n");
        let set2 = parse_cpus_from_sysfile(&mut file2).expect("Failed to file");
        assert_eq!(set2, (16..=24).collect());

        set1 = set1.intersection(&set2).copied().collect();
        assert_eq!(set1, (16..=19).collect());
    }

    #[test]
    fn test_intersects_with_discontinuous() {
        let mut file1 = new_file("0, 2-4, 7, 10\n");
        let mut set1 = parse_cpus_from_sysfile(&mut file1).expect("Failed to file");
        assert_eq!(set1, [0, 2, 3, 4, 7, 10].iter().copied().collect());

        let mut file2 = new_file("0-2, 5, 8-10\n");
        let set2 = parse_cpus_from_sysfile(&mut file2).expect("Failed to file");
        assert_eq!(set2, [0, 1, 2, 5, 8, 9, 10].iter().copied().collect());

        set1 = set1.intersection(&set2).copied().collect();
        assert_eq!(set1, [0, 2, 10].iter().copied().collect());
    }

    #[test]
    fn test_bad_input() {
        let mut file = new_file("abc\n");
        let _set = parse_cpus_from_sysfile(&mut file).expect_err("Did not fail to parse");
    }

    #[test]
    fn test_bad_input_range() {
        let mut file = new_file("1-abc\n");
        let _set = parse_cpus_from_sysfile(&mut file).expect_err("Did not fail to parse");
    }
}
