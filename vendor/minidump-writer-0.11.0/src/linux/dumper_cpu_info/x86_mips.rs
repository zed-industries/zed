use {
    super::CpuInfoError,
    crate::minidump_format::*,
    failspot::failspot,
    std::{
        io::{BufRead, BufReader},
        path,
    },
};

type Result<T> = std::result::Result<T, CpuInfoError>;

struct CpuInfoEntry {
    info_name: &'static str,
    value: i32,
    found: bool,
}

impl CpuInfoEntry {
    fn new(info_name: &'static str, value: i32, found: bool) -> Self {
        CpuInfoEntry {
            info_name,
            value,
            found,
        }
    }
}

pub fn write_cpu_information(sys_info: &mut MDRawSystemInfo) -> Result<()> {
    let vendor_id_name = "vendor_id";
    let mut cpu_info_table = [
        CpuInfoEntry::new("processor", -1, false),
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        CpuInfoEntry::new("model", 0, false),
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        CpuInfoEntry::new("stepping", 0, false),
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        CpuInfoEntry::new("cpu family", 0, false),
    ];

    // processor_architecture should always be set, do this first
    sys_info.processor_architecture = if cfg!(target_arch = "mips") {
        MDCPUArchitecture::PROCESSOR_ARCHITECTURE_MIPS
    } else if cfg!(target_arch = "mips64") {
        MDCPUArchitecture::PROCESSOR_ARCHITECTURE_MIPS64
    } else if cfg!(target_arch = "x86") {
        MDCPUArchitecture::PROCESSOR_ARCHITECTURE_INTEL
    } else {
        MDCPUArchitecture::PROCESSOR_ARCHITECTURE_AMD64
    } as u16;

    failspot!(
        CpuInfoFileOpen
        bail(std::io::Error::other("test requested cpuinfo file failure"))
    );

    let cpuinfo_file = std::fs::File::open(path::PathBuf::from("/proc/cpuinfo"))?;

    let mut vendor_id = String::new();
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

        let mut liter = line.split(':').map(|x| x.trim());
        let field = liter.next().unwrap(); // guaranteed to have at least one item
        let value = if let Some(val) = liter.next() {
            val
        } else {
            continue;
        };

        let mut is_first_entry = true;
        for entry in cpu_info_table.iter_mut() {
            if !is_first_entry && entry.found {
                // except for the 'processor' field, ignore repeated values.
                continue;
            }
            is_first_entry = false;
            if field == entry.info_name {
                if let Ok(v) = value.parse() {
                    entry.value = v;
                    entry.found = true;
                } else {
                    continue;
                }
            }

            // special case for vendor_id
            if field == vendor_id_name && !value.is_empty() {
                vendor_id = value.into();
            }
        }
    }
    // make sure we got everything we wanted
    if !cpu_info_table.iter().all(|x| x.found) {
        return Err(CpuInfoError::NotAllProcEntriesFound);
    }
    // cpu_info_table[0] holds the last cpu id listed in /proc/cpuinfo,
    // assuming this is the highest id, change it to the number of CPUs
    // by adding one.
    cpu_info_table[0].value += 1;

    sys_info.number_of_processors = cpu_info_table[0].value as u8; // TODO: might not work on special machines with LOTS of CPUs
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        sys_info.processor_level = cpu_info_table[3].value as u16;
        sys_info.processor_revision =
            ((cpu_info_table[1].value << 8) | cpu_info_table[2].value) as u16;
    }
    if !vendor_id.is_empty() {
        let vendor_id = vendor_id.as_bytes();
        // The vendor_id is the first 12 (3 * size_of::<u32>()) bytes
        let vendor_len = std::cmp::min(3 * std::mem::size_of::<u32>(), vendor_id.len());
        sys_info.cpu.data[..vendor_len].copy_from_slice(&vendor_id[..vendor_len]);
    }

    Ok(())
}
