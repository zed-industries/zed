use {super::*, crate::minidump_format::*};

/// Retrieve the OS version information.
///
/// Note that this only works on 10.13.4+, but that release is over 4 years old
/// and 1 version behind the latest unsupported release at the time of this writing
///
/// Note that Breakpad/Crashpad use a private API in CoreFoundation to do this
/// via _CFCopySystemVersionDictionary->_kCFSystemVersionProductVersionKey
fn os_version() -> (u32, u32, u32) {
    let vers = mach::sysctl_string(b"kern.osproductversion\0");

    let inner = || {
        let mut it = vers.split('.');

        let major: u32 = it.next()?.parse().ok()?;
        let minor: u32 = it.next()?.parse().ok()?;
        let patch: u32 = it.next().and_then(|p| p.parse().ok()).unwrap_or_default();

        Some((major, minor, patch))
    };

    inner().unwrap_or_default()
}

/// Retrieves the OS build version.
///
/// Note that Breakpad/Crashpad use a private API in CoreFoundation to do this
/// via _CFCopySystemVersionDictionary->_kCFSystemVersionBuildVersionKey. I have
/// no idea how long this has been the case, but the same information can be
/// retrieved via `sysctlbyname` via the `kern.osversion` key as seen by comparing
/// its value versus the output of the `sw_vers -buildVersion` command
#[inline]
fn build_version() -> String {
    mach::sysctl_string(b"kern.osversion\0")
}

/// Retrieves more detailed information on the cpu.
///
/// Note that this function is only implemented on `x86_64` as Apple doesn't
/// expose similar info on `aarch64` (or at least, not via the same mechanisms)
fn read_cpu_info(cpu: &mut format::CPU_INFORMATION) {
    if !cfg!(target_arch = "x86_64") {
        return;
    }

    let mut md_feats: u64 = 1 << 2 /*PF_COMPARE_EXCHANGE_DOUBLE*/;
    let features: u64 = mach::sysctl_by_name(b"machdep.cpu.feature_bits\0");

    // Map the cpuid feature to its equivalent minidump cpu feature.
    // See https://en.wikipedia.org/wiki/CPUID for where the values for the
    // various cpuid bits come from, and
    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocessorfeaturepresent
    // for where the bits for the the minidump come from
    macro_rules! map_feature {
        ($set:expr, $cpuid_bit:expr, $md_bit:expr) => {
            if $set & (1 << $cpuid_bit) != 0 {
                md_feats |= 1 << $md_bit;
            }
        };
    }

    map_feature!(
        features, 4, /*TSC*/
        8  /* PF_RDTSC_INSTRUCTION_AVAILABLE */
    );
    map_feature!(features, 6 /*PAE*/, 9 /* PF_PAE_ENABLED */);
    map_feature!(
        features, 23, /*MMX*/
        3   /* PF_MMX_INSTRUCTIONS_AVAILABLE */
    );
    map_feature!(
        features, 25, /*SSE*/
        6   /* PF_XMMI_INSTRUCTIONS_AVAILABLE */
    );
    map_feature!(
        features, 26, /*SSE2*/
        10  /* PF_XMMI64_INSTRUCTIONS_AVAILABLE */
    );
    map_feature!(
        features, 32, /*SSE3*/
        13  /* PF_SSE3_INSTRUCTIONS_AVAILABLE */
    );
    map_feature!(
        features, 45, /*CX16*/
        14  /* PF_COMPARE_EXCHANGE128 */
    );
    map_feature!(features, 58 /*XSAVE*/, 17 /* PF_XSAVE_ENABLED */);
    map_feature!(
        features, 62, /*RDRAND*/
        28  /* PF_RDRAND_INSTRUCTION_AVAILABLE */
    );

    let ext_features: u64 = mach::sysctl_by_name(b"machdep.cpu.extfeature_bits\0");

    map_feature!(
        ext_features,
        27, /* RDTSCP */
        32  /* PF_RDTSCP_INSTRUCTION_AVAILABLE */
    );
    map_feature!(
        ext_features,
        31, /* 3DNOW */
        7   /* PF_3DNOW_INSTRUCTIONS_AVAILABLE */
    );

    let leaf_features: u32 = mach::sysctl_by_name(b"machdep.cpu.leaf7_feature_bits\0");
    map_feature!(
        leaf_features,
        0,  /* F7_FSGSBASE */
        22  /* PF_RDWRFSGSBASE_AVAILABLE */
    );

    // In newer production kernels, NX is always enabled.
    // See 10.15.0 xnu-6153.11.26/osfmk/x86_64/pmap.c nx_enabled.
    md_feats |= 1 << 12 /* PF_NX_ENABLED */;

    // All CPUs that Apple is known to have shipped should support DAZ.
    md_feats |= 1 << 11 /* PF_SSE_DAZ_MODE_AVAILABLE */;

    // minidump_common::format::OtherCpuInfo is just 2 adjacent u64's, we only
    // set the first, so just do a direct write to the bytes
    cpu.data[..std::mem::size_of::<u64>()].copy_from_slice(&md_feats.to_ne_bytes());
}

impl MinidumpWriter {
    /// Writes the [`MDStreamType::SystemInfoStream`] stream.
    ///
    /// On MacOS we includes basic CPU information, though some of it is not
    /// available on `aarch64` at the time of this writing, as well as kernel
    /// version information.
    pub(crate) fn write_system_info(
        &mut self,
        buffer: &mut DumpBuf,
        _dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        let mut info_section = MemoryWriter::<MDRawSystemInfo>::alloc(buffer)?;
        let dirent = MDRawDirectory {
            stream_type: MDStreamType::SystemInfoStream as u32,
            location: info_section.location(),
        };

        let number_of_processors: u8 = mach::int_sysctl_by_name(b"hw.ncpu\0");
        // SAFETY: POD buffer
        let mut cpu: format::CPU_INFORMATION = unsafe { std::mem::zeroed() };
        read_cpu_info(&mut cpu);

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                let processor_architecture = MDCPUArchitecture::PROCESSOR_ARCHITECTURE_AMD64;

                // machdep.cpu.family and machdep.cpu.model already take the extended family
                // and model IDs into account. See 10.9.2 xnu-2422.90.20/osfmk/i386/cpuid.c
                // cpuid_set_generic_info().
                let processor_level: u16 = mach::int_sysctl_by_name(b"machdep.cpu.family\0");
                let model: u8 = mach::int_sysctl_by_name(b"machdep.cpu.model\0");
                let stepping: u8 = mach::int_sysctl_by_name(b"machdep.cpu.stepping\0");

                let processor_revision = ((model as u16) << 8) | stepping as u16;
            } else if #[cfg(target_arch = "aarch64")] {
                let processor_architecture = MDCPUArchitecture::PROCESSOR_ARCHITECTURE_ARM64_OLD;

                let family: u32 = mach::sysctl_by_name(b"hw.cpufamily\0");

                let processor_level = (family & 0xffff0000 >> 16) as u16;
                let processor_revision = (family & 0x0000ffff)  as u16;
            } else {
                compile_error!("unsupported target architecture");
            }
        }

        let (major_version, minor_version, build_number) = os_version();
        let os_version_loc = write_string_to_location(buffer, &build_version())?;

        let info = MDRawSystemInfo {
            // CPU
            processor_architecture: processor_architecture as u16,
            processor_level,
            processor_revision,
            number_of_processors,
            cpu,

            // OS
            platform_id: PlatformId::MacOs as u32,
            product_type: 1, // VER_NT_WORKSTATION, could also be VER_NT_SERVER but...seriously?
            major_version,
            minor_version,
            build_number,
            csd_version_rva: os_version_loc.rva,

            suite_mask: 0,
            reserved2: 0,
        };

        info_section.set_value(buffer, info)?;

        Ok(dirent)
    }
}
