//! Performs autodetection of the host for the purposes of running
//! Cranelift to generate code to run on the same machine.

#![deny(missing_docs)]

use cranelift_codegen::isa;
use cranelift_codegen::settings::Configurable;
use target_lexicon::Triple;

#[cfg(all(target_arch = "riscv64", target_os = "linux"))]
mod riscv;

/// Return an `isa` builder configured for the current host
/// machine, or `Err(())` if the host machine is not supported
/// in the current configuration.
pub fn builder() -> Result<isa::Builder, &'static str> {
    builder_with_options(true)
}

/// Return an `isa` builder configured for the current host
/// machine, or `Err(())` if the host machine is not supported
/// in the current configuration.
///
/// Selects the given backend variant specifically; this is
/// useful when more than one backend exists for a given target
/// (e.g., on x86-64).
pub fn builder_with_options(infer_native_flags: bool) -> Result<isa::Builder, &'static str> {
    let mut isa_builder = isa::lookup(Triple::host()).map_err(|err| match err {
        isa::LookupError::SupportDisabled => "support for architecture disabled at compile time",
        isa::LookupError::Unsupported => "unsupported architecture",
    })?;
    if infer_native_flags {
        self::infer_native_flags(&mut isa_builder)?;
    }
    Ok(isa_builder)
}

/// Return an `isa` builder configured for the current host
/// machine, or `Err(())` if the host machine is not supported
/// in the current configuration.
///
/// Selects the given backend variant specifically; this is
/// useful when more than one backend exists for a given target
/// (e.g., on x86-64).
pub fn infer_native_flags(isa_builder: &mut dyn Configurable) -> Result<(), &'static str> {
    #[cfg(target_arch = "x86_64")]
    {
        if !std::is_x86_feature_detected!("sse2") {
            return Err("x86 support requires SSE2");
        }

        if std::is_x86_feature_detected!("cmpxchg16b") {
            isa_builder.enable("has_cmpxchg16b").unwrap();
        }
        if std::is_x86_feature_detected!("sse3") {
            isa_builder.enable("has_sse3").unwrap();
        }
        if std::is_x86_feature_detected!("ssse3") {
            isa_builder.enable("has_ssse3").unwrap();
        }
        if std::is_x86_feature_detected!("sse4.1") {
            isa_builder.enable("has_sse41").unwrap();
        }
        if std::is_x86_feature_detected!("sse4.2") {
            isa_builder.enable("has_sse42").unwrap();
        }
        if std::is_x86_feature_detected!("popcnt") {
            isa_builder.enable("has_popcnt").unwrap();
        }
        if std::is_x86_feature_detected!("avx") {
            isa_builder.enable("has_avx").unwrap();
        }
        if std::is_x86_feature_detected!("avx2") {
            isa_builder.enable("has_avx2").unwrap();
        }
        if std::is_x86_feature_detected!("fma") {
            isa_builder.enable("has_fma").unwrap();
        }
        if std::is_x86_feature_detected!("bmi1") {
            isa_builder.enable("has_bmi1").unwrap();
        }
        if std::is_x86_feature_detected!("bmi2") {
            isa_builder.enable("has_bmi2").unwrap();
        }
        if std::is_x86_feature_detected!("avx512bitalg") {
            isa_builder.enable("has_avx512bitalg").unwrap();
        }
        if std::is_x86_feature_detected!("avx512dq") {
            isa_builder.enable("has_avx512dq").unwrap();
        }
        if std::is_x86_feature_detected!("avx512f") {
            isa_builder.enable("has_avx512f").unwrap();
        }
        if std::is_x86_feature_detected!("avx512vl") {
            isa_builder.enable("has_avx512vl").unwrap();
        }
        if std::is_x86_feature_detected!("avx512vbmi") {
            isa_builder.enable("has_avx512vbmi").unwrap();
        }
        if std::is_x86_feature_detected!("lzcnt") {
            isa_builder.enable("has_lzcnt").unwrap();
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("lse") {
            isa_builder.enable("has_lse").unwrap();
        }

        if std::arch::is_aarch64_feature_detected!("paca") {
            isa_builder.enable("has_pauth").unwrap();
        }

        if std::arch::is_aarch64_feature_detected!("fp16") {
            isa_builder.enable("has_fp16").unwrap();
        }

        if cfg!(target_os = "macos") {
            // Pointer authentication is always available on Apple Silicon.
            isa_builder.enable("sign_return_address").unwrap();
            // macOS enforces the use of the B key for return addresses.
            isa_builder.enable("sign_return_address_with_bkey").unwrap();
        }
    }

    // `is_s390x_feature_detected` is nightly only for now, so use the
    // STORE FACILITY LIST EXTENDED instruction as a temporary measure.
    #[cfg(target_arch = "s390x")]
    {
        let mut facility_list: [u64; 4] = [0; 4];
        unsafe {
            core::arch::asm!(
                "stfle 0({})",
                in(reg_addr) facility_list.as_mut_ptr() ,
                inout("r0") facility_list.len() as u64 - 1 => _,
                options(nostack)
            );
        }
        let get_facility_bit = |n: usize| {
            // NOTE: bits are numbered from the left.
            facility_list[n / 64] & (1 << (63 - (n % 64))) != 0
        };

        if get_facility_bit(61) {
            isa_builder.enable("has_mie3").unwrap();
        }
        if get_facility_bit(84) {
            isa_builder.enable("has_mie4").unwrap();
        }
        if get_facility_bit(148) {
            isa_builder.enable("has_vxrs_ext2").unwrap();
        }
        if get_facility_bit(198) {
            isa_builder.enable("has_vxrs_ext3").unwrap();
        }
    }

    // `is_riscv_feature_detected` is nightly only for now, use
    // getauxval from the libc crate directly as a temporary measure.
    #[cfg(all(target_arch = "riscv64", target_os = "linux"))]
    {
        // Try both hwcap and cpuinfo
        // HWCAP only returns single letter extensions, cpuinfo returns all of
        // them but may not be available in some systems (QEMU < 8.1).
        riscv::hwcap_detect(isa_builder)?;

        // Ignore errors for cpuinfo. QEMU versions prior to 8.1 do not emulate
        // the cpuinfo interface, so we can't rely on it being present for now.
        let _ = riscv::cpuinfo_detect(isa_builder);
    }

    // On all other architectures (e.g. wasm32) we won't infer any native flags,
    // but still need to use the `isa_builder` to avoid compiler warnings.
    let _ = isa_builder;
    Ok(())
}

/// Version number of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::builder;
    use cranelift_codegen::isa::CallConv;
    use cranelift_codegen::settings;

    #[test]
    fn test() {
        if let Ok(isa_builder) = builder() {
            let flag_builder = settings::builder();
            let isa = isa_builder
                .finish(settings::Flags::new(flag_builder))
                .unwrap();

            if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
                assert_eq!(isa.default_call_conv(), CallConv::AppleAarch64);
            } else if cfg!(unix) {
                assert_eq!(isa.default_call_conv(), CallConv::SystemV);
            } else if cfg!(windows) {
                assert_eq!(isa.default_call_conv(), CallConv::WindowsFastcall);
            }

            if cfg!(target_pointer_width = "64") {
                assert_eq!(isa.pointer_bits(), 64);
            } else if cfg!(target_pointer_width = "32") {
                assert_eq!(isa.pointer_bits(), 32);
            } else if cfg!(target_pointer_width = "16") {
                assert_eq!(isa.pointer_bits(), 16);
            }
        }
    }
}
