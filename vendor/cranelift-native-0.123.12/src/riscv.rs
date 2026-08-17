use cranelift_codegen::settings::Configurable;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn hwcap_detect(isa_builder: &mut dyn Configurable) -> Result<(), &'static str> {
    let v = unsafe { libc::getauxval(libc::AT_HWCAP) };

    const HWCAP_RISCV_EXT_A: libc::c_ulong = 1 << (b'a' - b'a');
    const HWCAP_RISCV_EXT_C: libc::c_ulong = 1 << (b'c' - b'a');
    const HWCAP_RISCV_EXT_D: libc::c_ulong = 1 << (b'd' - b'a');
    const HWCAP_RISCV_EXT_F: libc::c_ulong = 1 << (b'f' - b'a');
    const HWCAP_RISCV_EXT_M: libc::c_ulong = 1 << (b'm' - b'a');
    const HWCAP_RISCV_EXT_V: libc::c_ulong = 1 << (b'v' - b'a');

    if (v & HWCAP_RISCV_EXT_A) != 0 {
        isa_builder.enable("has_a").unwrap();
    }

    if (v & HWCAP_RISCV_EXT_C) != 0 {
        isa_builder.enable("has_c").unwrap();
    }

    if (v & HWCAP_RISCV_EXT_D) != 0 {
        isa_builder.enable("has_d").unwrap();
    }

    if (v & HWCAP_RISCV_EXT_F) != 0 {
        isa_builder.enable("has_f").unwrap();

        // TODO: There doesn't seem to be a bit associated with this extension
        // rust enables it with the `f` extension:
        // https://github.com/rust-lang/stdarch/blob/790411f93c4b5eada3c23abb4c9a063fb0b24d99/crates/std_detect/src/detect/os/linux/riscv.rs#L43
        isa_builder.enable("has_zicsr").unwrap();
    }

    if (v & HWCAP_RISCV_EXT_M) != 0 {
        isa_builder.enable("has_m").unwrap();
    }

    if (v & HWCAP_RISCV_EXT_V) != 0 {
        isa_builder.enable("has_v").unwrap();
    }

    // In general extensions that are longer than one letter
    // won't have a bit associated with them. The Linux kernel
    // is currently working on a new way to query the extensions.
    Ok(())
}

/// Read the /proc/cpuinfo file and detect the extensions.
///
/// We are looking for the isa line string, which contains the extensions.
/// The format for this string is specified in the linux user space ABI for RISC-V:
/// https://github.com/torvalds/linux/blob/09a9639e56c01c7a00d6c0ca63f4c7c41abe075d/Documentation/riscv/uabi.rst
///
/// The format is fairly similar to the one specified in the RISC-V ISA manual, but
/// all lower case.
///
/// An example ISA string is: rv64imafdcvh_zawrs_zba_zbb_zicbom_zicboz_zicsr_zifencei_zihintpause
pub fn cpuinfo_detect(isa_builder: &mut dyn Configurable) -> Result<(), &'static str> {
    let file = File::open("/proc/cpuinfo").map_err(|_| "failed to open /proc/cpuinfo")?;

    let isa_string = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .find_map(|line| {
            if let Some((k, v)) = line.split_once(':') {
                if k.trim_end() == "isa" {
                    return Some(v.trim().to_string());
                }
            }
            None
        })
        .ok_or("failed to find isa line in /proc/cpuinfo")?;

    for ext in isa_string_extensions(&isa_string) {
        // Try enabling all the extensions that are parsed.
        // Cranelift won't recognize all of them, but that's okay we just ignore them.
        // Extensions flags in the RISC-V backend have the format of `has_x` for the `x` extension.
        let _ = isa_builder.enable(&format!("has_{ext}"));
    }

    Ok(())
}

/// Parses an ISA string and returns an iterator over the extensions.
fn isa_string_extensions(isa: &str) -> Vec<&str> {
    let mut parts = isa.split('_');
    let mut extensions = Vec::new();
    // The first entry has the form `rv64imafdcvh`, we need to skip the architecture ("rv64").
    // Each of the letters after the cpu architecture is an extension, so return them
    // individually.
    if let Some(letters) = parts.next().unwrap().strip_prefix("rv64") {
        extensions.extend(letters.matches(|_| true));
        extensions.extend(parts);
    }
    extensions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_isa() {
        let isa_string = "rv64imafdcvh_zawrs_zba_zbb_zicbom_zicboz_zicsr_zifencei_zihintpause";
        let extensions = vec![
            "i",
            "m",
            "a",
            "f",
            "d",
            "c",
            "v",
            "h",
            "zawrs",
            "zba",
            "zbb",
            "zicbom",
            "zicboz",
            "zicsr",
            "zifencei",
            "zihintpause",
        ];

        assert_eq!(isa_string_extensions(isa_string), extensions,);
    }
}
