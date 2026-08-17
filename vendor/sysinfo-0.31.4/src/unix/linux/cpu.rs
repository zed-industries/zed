// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::too_many_arguments)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::Instant;

use crate::sys::utils::to_u64;
use crate::{Cpu, CpuRefreshKind};

macro_rules! to_str {
    ($e:expr) => {
        unsafe { std::str::from_utf8_unchecked($e) }
    };
}

pub(crate) struct CpusWrapper {
    pub(crate) global_cpu: CpuUsage,
    pub(crate) cpus: Vec<Cpu>,
    /// Field set to `false` in `update_cpus` and to `true` in `refresh_processes_specifics`.
    ///
    /// The reason behind this is to avoid calling the `update_cpus` more than necessary.
    /// For example when running `refresh_all` or `refresh_specifics`.
    need_cpus_update: bool,
    got_cpu_frequency: bool,
    /// This field is needed to prevent updating when not enough time passed since last update.
    last_update: Option<Instant>,
}

impl CpusWrapper {
    pub(crate) fn new() -> Self {
        Self {
            global_cpu: CpuUsage::default(),
            cpus: Vec::with_capacity(4),
            need_cpus_update: true,
            got_cpu_frequency: false,
            last_update: None,
        }
    }

    pub(crate) fn refresh_if_needed(
        &mut self,
        only_update_global_cpu: bool,
        refresh_kind: CpuRefreshKind,
    ) {
        if self.need_cpus_update {
            self.refresh(only_update_global_cpu, refresh_kind);
        }
    }

    pub(crate) fn refresh(&mut self, only_update_global_cpu: bool, refresh_kind: CpuRefreshKind) {
        let need_cpu_usage_update = self
            .last_update
            .map(|last_update| last_update.elapsed() > crate::MINIMUM_CPU_UPDATE_INTERVAL)
            .unwrap_or(true);

        let first = self.cpus.is_empty();
        let mut vendors_brands = if first {
            get_vendor_id_and_brand()
        } else {
            HashMap::new()
        };

        // If the last CPU usage update is too close (less than `MINIMUM_CPU_UPDATE_INTERVAL`),
        // we don't want to update CPUs times.
        if need_cpu_usage_update {
            self.last_update = Some(Instant::now());
            let f = match File::open("/proc/stat") {
                Ok(f) => f,
                Err(_e) => {
                    sysinfo_debug!("failed to retrieve CPU information: {:?}", _e);
                    return;
                }
            };
            let buf = BufReader::new(f);

            self.need_cpus_update = false;
            let mut i: usize = 0;
            let mut it = buf.split(b'\n');

            if first || refresh_kind.cpu_usage() {
                if let Some(Ok(line)) = it.next() {
                    if &line[..4] != b"cpu " {
                        return;
                    }
                    let mut parts = line.split(|x| *x == b' ').filter(|s| !s.is_empty()).skip(1);
                    self.global_cpu.set(
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                        parts.next().map(to_u64).unwrap_or(0),
                    );
                }
                if first || !only_update_global_cpu {
                    while let Some(Ok(line)) = it.next() {
                        if &line[..3] != b"cpu" {
                            break;
                        }

                        let mut parts = line.split(|x| *x == b' ').filter(|s| !s.is_empty());
                        if first {
                            let (vendor_id, brand) = match vendors_brands.remove(&i) {
                                Some((vendor_id, brand)) => (vendor_id, brand),
                                None => (String::new(), String::new()),
                            };
                            self.cpus.push(Cpu {
                                inner: CpuInner::new_with_values(
                                    to_str!(parts.next().unwrap_or(&[])),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    parts.next().map(to_u64).unwrap_or(0),
                                    0,
                                    vendor_id,
                                    brand,
                                ),
                            });
                        } else {
                            parts.next(); // we don't want the name again
                            self.cpus[i].inner.set(
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                                parts.next().map(to_u64).unwrap_or(0),
                            );
                        }

                        i += 1;
                    }
                }
            }
        }

        if refresh_kind.frequency() {
            #[cfg(feature = "multithread")]
            use rayon::iter::{
                IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
            };

            #[cfg(feature = "multithread")]
            // This function is voluntarily made generic in case we want to generalize it.
            fn iter_mut<'a, T>(
                val: &'a mut T,
            ) -> <&'a mut T as rayon::iter::IntoParallelIterator>::Iter
            where
                &'a mut T: rayon::iter::IntoParallelIterator,
            {
                val.par_iter_mut()
            }

            #[cfg(not(feature = "multithread"))]
            fn iter_mut<'a>(val: &'a mut Vec<Cpu>) -> std::slice::IterMut<'a, Cpu> {
                val.iter_mut()
            }

            // `get_cpu_frequency` is very slow, so better run it in parallel.
            iter_mut(&mut self.cpus)
                .enumerate()
                .for_each(|(pos, proc_)| proc_.inner.frequency = get_cpu_frequency(pos));

            self.got_cpu_frequency = true;
        }
    }

    pub(crate) fn get_global_raw_times(&self) -> (u64, u64) {
        (self.global_cpu.total_time, self.global_cpu.old_total_time)
    }

    pub(crate) fn len(&self) -> usize {
        self.cpus.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cpus.is_empty()
    }

    pub(crate) fn set_need_cpus_update(&mut self) {
        self.need_cpus_update = true;
    }
}

/// Struct containing values to compute a CPU usage.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CpuValues {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

impl CpuValues {
    /// Sets the given argument to the corresponding fields.
    pub fn set(
        &mut self,
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
    ) {
        // `guest` is already accounted in `user`.
        self.user = user.saturating_sub(guest);
        // `guest_nice` is already accounted in `nice`.
        self.nice = nice.saturating_sub(guest_nice);
        self.system = system;
        self.idle = idle;
        self.iowait = iowait;
        self.irq = irq;
        self.softirq = softirq;
        self.steal = steal;
        self.guest = guest;
        self.guest_nice = guest_nice;
    }

    /// Returns work time.
    pub fn work_time(&self) -> u64 {
        self.user
            .saturating_add(self.nice)
            .saturating_add(self.system)
            .saturating_add(self.irq)
            .saturating_add(self.softirq)
    }

    /// Returns total time.
    pub fn total_time(&self) -> u64 {
        self.work_time()
            .saturating_add(self.idle)
            .saturating_add(self.iowait)
            // `steal`, `guest` and `guest_nice` are only used if we want to account the "guest"
            // into the computation.
            .saturating_add(self.guest)
            .saturating_add(self.guest_nice)
            .saturating_add(self.steal)
    }
}

#[derive(Default)]
pub(crate) struct CpuUsage {
    percent: f32,
    old_values: CpuValues,
    new_values: CpuValues,
    total_time: u64,
    old_total_time: u64,
}

impl CpuUsage {
    pub(crate) fn new_with_values(
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
    ) -> Self {
        let mut new_values = CpuValues::default();
        new_values.set(
            user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice,
        );
        Self {
            old_values: CpuValues::default(),
            new_values,
            percent: 0f32,
            total_time: 0,
            old_total_time: 0,
        }
    }

    pub(crate) fn set(
        &mut self,
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
    ) {
        macro_rules! min {
            ($a:expr, $b:expr, $def:expr) => {
                if $a > $b {
                    ($a - $b) as f32
                } else {
                    $def
                }
            };
        }
        self.old_values = self.new_values;
        self.new_values.set(
            user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice,
        );
        self.total_time = self.new_values.total_time();
        self.old_total_time = self.old_values.total_time();
        self.percent = min!(self.new_values.work_time(), self.old_values.work_time(), 0.)
            / min!(self.total_time, self.old_total_time, 1.)
            * 100.;
        if self.percent > 100. {
            self.percent = 100.; // to prevent the percentage to go above 100%
        }
    }

    pub(crate) fn usage(&self) -> f32 {
        self.percent
    }
}

pub(crate) struct CpuInner {
    usage: CpuUsage,
    pub(crate) name: String,
    pub(crate) frequency: u64,
    pub(crate) vendor_id: String,
    pub(crate) brand: String,
}

impl CpuInner {
    pub(crate) fn new_with_values(
        name: &str,
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
        frequency: u64,
        vendor_id: String,
        brand: String,
    ) -> Self {
        Self {
            usage: CpuUsage::new_with_values(
                user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice,
            ),
            name: name.to_owned(),
            frequency,
            vendor_id,
            brand,
        }
    }

    pub(crate) fn set(
        &mut self,
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
    ) {
        self.usage.set(
            user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice,
        );
    }

    pub(crate) fn cpu_usage(&self) -> f32 {
        self.usage.percent
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Returns the CPU frequency in MHz.
    pub(crate) fn frequency(&self) -> u64 {
        self.frequency
    }

    pub(crate) fn vendor_id(&self) -> &str {
        &self.vendor_id
    }

    pub(crate) fn brand(&self) -> &str {
        &self.brand
    }
}

pub(crate) fn get_cpu_frequency(cpu_core_index: usize) -> u64 {
    let mut s = String::new();
    if File::open(format!(
        "/sys/devices/system/cpu/cpu{cpu_core_index}/cpufreq/scaling_cur_freq",
    ))
    .and_then(|mut f| f.read_to_string(&mut s))
    .is_ok()
    {
        let freq_option = s.trim().split('\n').next();
        if let Some(freq_string) = freq_option {
            if let Ok(freq) = freq_string.parse::<u64>() {
                return freq / 1000;
            }
        }
    }
    s.clear();
    if File::open("/proc/cpuinfo")
        .and_then(|mut f| f.read_to_string(&mut s))
        .is_err()
    {
        return 0;
    }
    let find_cpu_mhz = s.split('\n').find(|line| {
        line.starts_with("cpu MHz\t")
            || line.starts_with("BogoMIPS")
            || line.starts_with("clock\t")
            || line.starts_with("bogomips per cpu")
    });
    find_cpu_mhz
        .and_then(|line| line.split(':').last())
        .and_then(|val| val.replace("MHz", "").trim().parse::<f64>().ok())
        .map(|speed| speed as u64)
        .unwrap_or_default()
}

#[allow(unused_assignments)]
pub(crate) fn get_physical_core_count() -> Option<usize> {
    let mut s = String::new();
    if let Err(_e) = File::open("/proc/cpuinfo").and_then(|mut f| f.read_to_string(&mut s)) {
        sysinfo_debug!("Cannot read `/proc/cpuinfo` file: {:?}", _e);
        return None;
    }

    macro_rules! add_core {
        ($core_ids_and_physical_ids:ident, $core_id:ident, $physical_id:ident, $cpu:ident) => {{
            if !$core_id.is_empty() && !$physical_id.is_empty() {
                $core_ids_and_physical_ids.insert(format!("{} {}", $core_id, $physical_id));
            } else if !$cpu.is_empty() {
                // On systems with only physical cores like raspberry, there is no "core id" or
                // "physical id" fields. So if one of them is missing, we simply use the "CPU"
                // info and count it as a physical core.
                $core_ids_and_physical_ids.insert($cpu.to_owned());
            }
            $core_id = "";
            $physical_id = "";
            $cpu = "";
        }};
    }

    let mut core_ids_and_physical_ids: HashSet<String> = HashSet::new();
    let mut core_id = "";
    let mut physical_id = "";
    let mut cpu = "";

    for line in s.lines() {
        if line.is_empty() {
            add_core!(core_ids_and_physical_ids, core_id, physical_id, cpu);
        } else if line.starts_with("processor") {
            cpu = line
                .splitn(2, ':')
                .last()
                .map(|x| x.trim())
                .unwrap_or_default();
        } else if line.starts_with("core id") {
            core_id = line
                .splitn(2, ':')
                .last()
                .map(|x| x.trim())
                .unwrap_or_default();
        } else if line.starts_with("physical id") {
            physical_id = line
                .splitn(2, ':')
                .last()
                .map(|x| x.trim())
                .unwrap_or_default();
        }
    }
    add_core!(core_ids_and_physical_ids, core_id, physical_id, cpu);

    Some(core_ids_and_physical_ids.len())
}

/// Obtain the implementer of this CPU core.
///
/// This has been obtained from util-linux's lscpu implementation, see
/// https://github.com/util-linux/util-linux/blob/7076703b529d255600631306419cca1b48ab850a/sys-utils/lscpu-arm.c#L240
///
/// This list will have to be updated every time a new vendor appears, please keep it synchronized
/// with util-linux and update the link above with the commit you have used.
fn get_arm_implementer(implementer: u32) -> Option<&'static str> {
    Some(match implementer {
        0x41 => "ARM",
        0x42 => "Broadcom",
        0x43 => "Cavium",
        0x44 => "DEC",
        0x46 => "FUJITSU",
        0x48 => "HiSilicon",
        0x49 => "Infineon",
        0x4d => "Motorola/Freescale",
        0x4e => "NVIDIA",
        0x50 => "APM",
        0x51 => "Qualcomm",
        0x53 => "Samsung",
        0x56 => "Marvell",
        0x61 => "Apple",
        0x66 => "Faraday",
        0x69 => "Intel",
        0x70 => "Phytium",
        0xc0 => "Ampere",
        _ => return None,
    })
}

/// Obtain the part of this CPU core.
///
/// This has been obtained from util-linux's lscpu implementation, see
/// https://github.com/util-linux/util-linux/blob/7076703b529d255600631306419cca1b48ab850a/sys-utils/lscpu-arm.c#L34
///
/// This list will have to be updated every time a new core appears, please keep it synchronized
/// with util-linux and update the link above with the commit you have used.
fn get_arm_part(implementer: u32, part: u32) -> Option<&'static str> {
    Some(match (implementer, part) {
        // ARM
        (0x41, 0x810) => "ARM810",
        (0x41, 0x920) => "ARM920",
        (0x41, 0x922) => "ARM922",
        (0x41, 0x926) => "ARM926",
        (0x41, 0x940) => "ARM940",
        (0x41, 0x946) => "ARM946",
        (0x41, 0x966) => "ARM966",
        (0x41, 0xa20) => "ARM1020",
        (0x41, 0xa22) => "ARM1022",
        (0x41, 0xa26) => "ARM1026",
        (0x41, 0xb02) => "ARM11 MPCore",
        (0x41, 0xb36) => "ARM1136",
        (0x41, 0xb56) => "ARM1156",
        (0x41, 0xb76) => "ARM1176",
        (0x41, 0xc05) => "Cortex-A5",
        (0x41, 0xc07) => "Cortex-A7",
        (0x41, 0xc08) => "Cortex-A8",
        (0x41, 0xc09) => "Cortex-A9",
        (0x41, 0xc0d) => "Cortex-A17", // Originally A12
        (0x41, 0xc0f) => "Cortex-A15",
        (0x41, 0xc0e) => "Cortex-A17",
        (0x41, 0xc14) => "Cortex-R4",
        (0x41, 0xc15) => "Cortex-R5",
        (0x41, 0xc17) => "Cortex-R7",
        (0x41, 0xc18) => "Cortex-R8",
        (0x41, 0xc20) => "Cortex-M0",
        (0x41, 0xc21) => "Cortex-M1",
        (0x41, 0xc23) => "Cortex-M3",
        (0x41, 0xc24) => "Cortex-M4",
        (0x41, 0xc27) => "Cortex-M7",
        (0x41, 0xc60) => "Cortex-M0+",
        (0x41, 0xd01) => "Cortex-A32",
        (0x41, 0xd02) => "Cortex-A34",
        (0x41, 0xd03) => "Cortex-A53",
        (0x41, 0xd04) => "Cortex-A35",
        (0x41, 0xd05) => "Cortex-A55",
        (0x41, 0xd06) => "Cortex-A65",
        (0x41, 0xd07) => "Cortex-A57",
        (0x41, 0xd08) => "Cortex-A72",
        (0x41, 0xd09) => "Cortex-A73",
        (0x41, 0xd0a) => "Cortex-A75",
        (0x41, 0xd0b) => "Cortex-A76",
        (0x41, 0xd0c) => "Neoverse-N1",
        (0x41, 0xd0d) => "Cortex-A77",
        (0x41, 0xd0e) => "Cortex-A76AE",
        (0x41, 0xd13) => "Cortex-R52",
        (0x41, 0xd20) => "Cortex-M23",
        (0x41, 0xd21) => "Cortex-M33",
        (0x41, 0xd40) => "Neoverse-V1",
        (0x41, 0xd41) => "Cortex-A78",
        (0x41, 0xd42) => "Cortex-A78AE",
        (0x41, 0xd43) => "Cortex-A65AE",
        (0x41, 0xd44) => "Cortex-X1",
        (0x41, 0xd46) => "Cortex-A510",
        (0x41, 0xd47) => "Cortex-A710",
        (0x41, 0xd48) => "Cortex-X2",
        (0x41, 0xd49) => "Neoverse-N2",
        (0x41, 0xd4a) => "Neoverse-E1",
        (0x41, 0xd4b) => "Cortex-A78C",
        (0x41, 0xd4c) => "Cortex-X1C",
        (0x41, 0xd4d) => "Cortex-A715",
        (0x41, 0xd4e) => "Cortex-X3",

        // Broadcom
        (0x42, 0x00f) => "Brahma-B15",
        (0x42, 0x100) => "Brahma-B53",
        (0x42, 0x516) => "ThunderX2",

        // Cavium
        (0x43, 0x0a0) => "ThunderX",
        (0x43, 0x0a1) => "ThunderX-88XX",
        (0x43, 0x0a2) => "ThunderX-81XX",
        (0x43, 0x0a3) => "ThunderX-83XX",
        (0x43, 0x0af) => "ThunderX2-99xx",

        // DEC
        (0x44, 0xa10) => "SA110",
        (0x44, 0xa11) => "SA1100",

        // Fujitsu
        (0x46, 0x001) => "A64FX",

        // HiSilicon
        (0x48, 0xd01) => "Kunpeng-920", // aka tsv110

        // NVIDIA
        (0x4e, 0x000) => "Denver",
        (0x4e, 0x003) => "Denver 2",
        (0x4e, 0x004) => "Carmel",

        // APM
        (0x50, 0x000) => "X-Gene",

        // Qualcomm
        (0x51, 0x00f) => "Scorpion",
        (0x51, 0x02d) => "Scorpion",
        (0x51, 0x04d) => "Krait",
        (0x51, 0x06f) => "Krait",
        (0x51, 0x201) => "Kryo",
        (0x51, 0x205) => "Kryo",
        (0x51, 0x211) => "Kryo",
        (0x51, 0x800) => "Falkor-V1/Kryo",
        (0x51, 0x801) => "Kryo-V2",
        (0x51, 0x802) => "Kryo-3XX-Gold",
        (0x51, 0x803) => "Kryo-3XX-Silver",
        (0x51, 0x804) => "Kryo-4XX-Gold",
        (0x51, 0x805) => "Kryo-4XX-Silver",
        (0x51, 0xc00) => "Falkor",
        (0x51, 0xc01) => "Saphira",

        // Samsung
        (0x53, 0x001) => "exynos-m1",

        // Marvell
        (0x56, 0x131) => "Feroceon-88FR131",
        (0x56, 0x581) => "PJ4/PJ4b",
        (0x56, 0x584) => "PJ4B-MP",

        // Apple
        (0x61, 0x020) => "Icestorm-A14",
        (0x61, 0x021) => "Firestorm-A14",
        (0x61, 0x022) => "Icestorm-M1",
        (0x61, 0x023) => "Firestorm-M1",
        (0x61, 0x024) => "Icestorm-M1-Pro",
        (0x61, 0x025) => "Firestorm-M1-Pro",
        (0x61, 0x028) => "Icestorm-M1-Max",
        (0x61, 0x029) => "Firestorm-M1-Max",
        (0x61, 0x030) => "Blizzard-A15",
        (0x61, 0x031) => "Avalanche-A15",
        (0x61, 0x032) => "Blizzard-M2",
        (0x61, 0x033) => "Avalanche-M2",

        // Faraday
        (0x66, 0x526) => "FA526",
        (0x66, 0x626) => "FA626",

        // Intel
        (0x69, 0x200) => "i80200",
        (0x69, 0x210) => "PXA250A",
        (0x69, 0x212) => "PXA210A",
        (0x69, 0x242) => "i80321-400",
        (0x69, 0x243) => "i80321-600",
        (0x69, 0x290) => "PXA250B/PXA26x",
        (0x69, 0x292) => "PXA210B",
        (0x69, 0x2c2) => "i80321-400-B0",
        (0x69, 0x2c3) => "i80321-600-B0",
        (0x69, 0x2d0) => "PXA250C/PXA255/PXA26x",
        (0x69, 0x2d2) => "PXA210C",
        (0x69, 0x411) => "PXA27x",
        (0x69, 0x41c) => "IPX425-533",
        (0x69, 0x41d) => "IPX425-400",
        (0x69, 0x41f) => "IPX425-266",
        (0x69, 0x682) => "PXA32x",
        (0x69, 0x683) => "PXA930/PXA935",
        (0x69, 0x688) => "PXA30x",
        (0x69, 0x689) => "PXA31x",
        (0x69, 0xb11) => "SA1110",
        (0x69, 0xc12) => "IPX1200",

        // Phytium
        (0x70, 0x660) => "FTC660",
        (0x70, 0x661) => "FTC661",
        (0x70, 0x662) => "FTC662",
        (0x70, 0x663) => "FTC663",

        _ => return None,
    })
}

/// Returns the brand/vendor string for the first CPU (which should be the same for all CPUs).
pub(crate) fn get_vendor_id_and_brand() -> HashMap<usize, (String, String)> {
    let mut s = String::new();
    if File::open("/proc/cpuinfo")
        .and_then(|mut f| f.read_to_string(&mut s))
        .is_err()
    {
        return HashMap::new();
    }

    fn get_value(s: &str) -> String {
        s.split(':')
            .last()
            .map(|x| x.trim().to_owned())
            .unwrap_or_default()
    }

    fn get_hex_value(s: &str) -> u32 {
        s.split(':')
            .last()
            .map(|x| x.trim())
            .filter(|x| x.starts_with("0x"))
            .map(|x| u32::from_str_radix(&x[2..], 16).unwrap())
            .unwrap_or_default()
    }

    #[inline]
    fn is_new_processor(line: &str) -> bool {
        line.starts_with("processor\t")
    }

    #[derive(Default)]
    struct CpuInfo {
        index: usize,
        vendor_id: Option<String>,
        brand: Option<String>,
        implementer: Option<u32>,
        part: Option<u32>,
    }

    impl CpuInfo {
        fn has_all_info(&self) -> bool {
            (self.brand.is_some() && self.vendor_id.is_some())
                || (self.implementer.is_some() && self.part.is_some())
        }

        fn convert(mut self) -> (usize, String, String) {
            let (vendor_id, brand) = if let (Some(implementer), Some(part)) =
                (self.implementer.take(), self.part.take())
            {
                let vendor_id = get_arm_implementer(implementer).map(String::from);
                // It's possible to "model name" even with an ARM CPU, so just in case we can't retrieve
                // the brand from "CPU part", we will then use the value from "model name".
                //
                // Example from raspberry pi 3B+:
                //
                // ```
                // model name      : ARMv7 Processor rev 4 (v7l)
                // CPU implementer : 0x41
                // CPU part        : 0xd03
                // ```
                let brand = get_arm_part(implementer, part)
                    .map(String::from)
                    .or_else(|| self.brand.take());
                (vendor_id, brand)
            } else {
                (self.vendor_id.take(), self.brand.take())
            };
            (
                self.index,
                vendor_id.unwrap_or_default(),
                brand.unwrap_or_default(),
            )
        }
    }

    let mut cpus: HashMap<usize, (String, String)> = HashMap::new();
    let mut lines = s.split('\n');
    while let Some(line) = lines.next() {
        if is_new_processor(line) {
            let index = match line
                .split(':')
                .nth(1)
                .and_then(|i| i.trim().parse::<usize>().ok())
            {
                Some(index) => index,
                None => {
                    sysinfo_debug!("Couldn't get processor ID from {line:?}, ignoring this core");
                    continue;
                }
            };

            let mut info = CpuInfo {
                index,
                ..Default::default()
            };

            #[allow(clippy::while_let_on_iterator)]
            while let Some(line) = lines.next() {
                if line.starts_with("vendor_id\t") {
                    info.vendor_id = Some(get_value(line));
                } else if line.starts_with("model name\t") {
                    info.brand = Some(get_value(line));
                } else if line.starts_with("CPU implementer\t") {
                    info.implementer = Some(get_hex_value(line));
                } else if line.starts_with("CPU part\t") {
                    info.part = Some(get_hex_value(line));
                } else if info.has_all_info() || is_new_processor(line) {
                    break;
                }
            }
            let (index, vendor_id, brand) = info.convert();
            cpus.insert(index, (vendor_id, brand));
        }
    }
    cpus
}
