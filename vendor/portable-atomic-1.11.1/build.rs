// SPDX-License-Identifier: Apache-2.0 OR MIT

// The rustc-cfg emitted by the build script are *not* public API.

#![allow(clippy::match_same_arms)] // https://github.com/rust-lang/rust-clippy/issues/12044

#[path = "version.rs"]
mod version;
use self::version::{Version, rustc_version};

#[path = "src/gen/build.rs"]
mod generated;

use std::{env, str};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/gen/build.rs");
    println!("cargo:rerun-if-changed=version.rs");

    #[cfg(feature = "unsafe-assume-single-core")]
    println!("cargo:rustc-cfg=portable_atomic_unsafe_assume_single_core");
    #[cfg(feature = "s-mode")]
    println!("cargo:rustc-cfg=portable_atomic_s_mode");
    #[cfg(feature = "force-amo")]
    println!("cargo:rustc-cfg=portable_atomic_force_amo");
    #[cfg(feature = "disable-fiq")]
    println!("cargo:rustc-cfg=portable_atomic_disable_fiq");

    let target = &*env::var("TARGET").expect("TARGET not set");
    let target_arch = &*env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");
    let target_os = &*env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");

    let version = match rustc_version() {
        Some(version) => version,
        None => {
            if env::var_os("PORTABLE_ATOMIC_DENY_WARNINGS").is_some() {
                panic!("unable to determine rustc version")
            }
            println!(
                "cargo:warning={}: unable to determine rustc version; assuming latest stable rustc (1.{})",
                env!("CARGO_PKG_NAME"),
                Version::LATEST.minor
            );
            Version::LATEST
        }
    };

    if version.minor >= 80 {
        println!(
            r#"cargo:rustc-check-cfg=cfg(target_feature,values("lsfe","fast-serialization","load-store-on-cond","distinct-ops","miscellaneous-extensions-3"))"#
        );

        // Custom cfgs set by build script. Not public API.
        // grep -F 'cargo:rustc-cfg=' build.rs | grep -Ev '^ *//' | sed -E 's/^.*cargo:rustc-cfg=//; s/(=\\)?".*$//' | LC_ALL=C sort -u | tr '\n' ',' | sed -E 's/,$/\n/'
        println!(
            "cargo:rustc-check-cfg=cfg(portable_atomic_disable_fiq,portable_atomic_force_amo,portable_atomic_ll_sc_rmw,portable_atomic_atomic_intrinsics,portable_atomic_no_asm,portable_atomic_no_asm_maybe_uninit,portable_atomic_no_atomic_64,portable_atomic_no_atomic_cas,portable_atomic_no_atomic_load_store,portable_atomic_no_atomic_min_max,portable_atomic_no_cfg_target_has_atomic,portable_atomic_no_cmpxchg16b_intrinsic,portable_atomic_no_cmpxchg16b_target_feature,portable_atomic_no_const_mut_refs,portable_atomic_no_const_raw_ptr_deref,portable_atomic_no_const_transmute,portable_atomic_no_core_unwind_safe,portable_atomic_no_diagnostic_namespace,portable_atomic_no_offset_of,portable_atomic_no_strict_provenance,portable_atomic_no_stronger_failure_ordering,portable_atomic_no_track_caller,portable_atomic_no_unsafe_op_in_unsafe_fn,portable_atomic_pre_llvm_15,portable_atomic_pre_llvm_16,portable_atomic_pre_llvm_18,portable_atomic_pre_llvm_20,portable_atomic_s_mode,portable_atomic_sanitize_thread,portable_atomic_target_feature,portable_atomic_unsafe_assume_single_core,portable_atomic_unstable_asm,portable_atomic_unstable_asm_experimental_arch,portable_atomic_unstable_cfg_target_has_atomic,portable_atomic_unstable_isa_attribute)"
        );
        // TODO: handle multi-line target_feature_fallback
        // grep -F 'target_feature_fallback("' build.rs | grep -Ev '^ *//' | sed -E 's/^.*target_feature_fallback\(//; s/",.*$/"/' | LC_ALL=C sort -u | tr '\n' ',' | sed -E 's/,$/\n/'
        println!(
            r#"cargo:rustc-check-cfg=cfg(portable_atomic_target_feature,values("cmpxchg16b","distinct-ops","fast-serialization","load-store-on-cond","lse","lse128","lse2","lsfe","mclass","miscellaneous-extensions-3","quadword-atomics","rcpc3","v6","zaamo","zabha","zacas"))"#
        );
    }

    // https://github.com/rust-lang/rust/pull/123745 (includes https://github.com/rust-lang/cargo/pull/13560) merged in Rust 1.79 (nightly-2024-04-11).
    if !version.probe(79, 2024, 4, 10) {
        // HACK: If --target is specified, rustflags is not applied to the build
        // script itself, so the build script will not be recompiled when rustflags
        // is changed. That in itself is not a problem, but the old Cargo does
        // not rerun the build script as well, which can be problematic.
        // https://github.com/rust-lang/cargo/issues/13003
        // This problem has been fixed in 1.79 so only older versions need a workaround.
        println!("cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
        println!("cargo:rerun-if-env-changed=RUSTFLAGS");
        println!("cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS");
        let mut target_upper = target.replace(|c: char| c == '-' || c == '.', "_");
        target_upper.make_ascii_uppercase();
        println!("cargo:rerun-if-env-changed=CARGO_TARGET_{}_RUSTFLAGS", target_upper);
    }

    // Note that cfgs are `no_`*, not `has_*`. This allows treating as the latest
    // stable rustc is used when the build script doesn't run. This is useful
    // for non-cargo build systems that don't run the build script.

    // atomic_min_max stabilized in Rust 1.45 (nightly-2020-05-30): https://github.com/rust-lang/rust/pull/72324
    if !version.probe(45, 2020, 5, 29) {
        println!("cargo:rustc-cfg=portable_atomic_no_atomic_min_max");
    }
    // track_caller stabilized in Rust 1.46 (nightly-2020-07-02): https://github.com/rust-lang/rust/pull/72445
    if !version.probe(46, 2020, 7, 1) {
        println!("cargo:rustc-cfg=portable_atomic_no_track_caller");
    }
    // unsafe_op_in_unsafe_fn stabilized in Rust 1.52 (nightly-2021-03-11): https://github.com/rust-lang/rust/pull/79208
    if !version.probe(52, 2021, 3, 10) {
        println!("cargo:rustc-cfg=portable_atomic_no_unsafe_op_in_unsafe_fn");
    }
    // const_transmute stabilized in Rust 1.56 (nightly-2021-07-29): https://github.com/rust-lang/rust/pull/85769
    if !version.probe(56, 2021, 7, 28) {
        println!("cargo:rustc-cfg=portable_atomic_no_const_transmute");
    }
    // https://github.com/rust-lang/rust/pull/84662 merged in Rust 1.56 (nightly-2021-08-02).
    if !version.probe(56, 2021, 8, 1) {
        println!("cargo:rustc-cfg=portable_atomic_no_core_unwind_safe");
    }
    // const_raw_ptr_deref stabilized in Rust 1.58 (nightly-2021-11-15): https://github.com/rust-lang/rust/pull/89551
    if !version.probe(58, 2021, 11, 14) {
        println!("cargo:rustc-cfg=portable_atomic_no_const_raw_ptr_deref");
    }
    // https://github.com/rust-lang/rust/pull/98383 merged in Rust 1.64 (nightly-2022-07-19).
    if !version.probe(64, 2022, 7, 18) {
        println!("cargo:rustc-cfg=portable_atomic_no_stronger_failure_ordering");
    }
    // https://github.com/rust-lang/rust/pull/114790 merged in nightly-2023-08-24
    if !version.probe(74, 2023, 8, 23) {
        println!("cargo:rustc-cfg=portable_atomic_no_asm_maybe_uninit");
    }
    // For test
    if version.minor < 77 {
        println!("cargo:rustc-cfg=portable_atomic_no_offset_of");
    }
    // #[diagnostic] stabilized in Rust 1.78 (nightly-2024-03-09): https://github.com/rust-lang/rust/pull/119888
    if !version.probe(78, 2024, 3, 8) {
        println!("cargo:rustc-cfg=portable_atomic_no_diagnostic_namespace");
    }
    // const_mut_refs/const_refs_to_cell stabilized in Rust 1.83 (nightly-2024-09-16): https://github.com/rust-lang/rust/pull/129195
    if !version.probe(83, 2024, 9, 15) {
        println!("cargo:rustc-cfg=portable_atomic_no_const_mut_refs");
    }
    // strict_provenance/exposed_provenance APIs stabilized in Rust 1.84 (nightly-2024-10-22): https://github.com/rust-lang/rust/pull/130350
    if !version.probe(84, 2024, 10, 21) {
        println!("cargo:rustc-cfg=portable_atomic_no_strict_provenance");
    }

    // For Miri and ThreadSanitizer. (aarch64, arm64ec, s390x, powerpc64)
    // https://github.com/rust-lang/rust/pull/97423 merged in Rust 1.64 (nightly-2022-06-30).
    // https://github.com/rust-lang/rust/pull/141507 merged in Rust 1.89 (nightly-2025-05-31).
    if version.nightly
        && version.probe(64, 2022, 6, 29)
        && !version.probe(89, 2025, 5, 30)
        && (target_arch != "powerpc64" || version.llvm >= 15)
    {
        println!("cargo:rustc-cfg=portable_atomic_atomic_intrinsics");
    }

    // asm! on AArch64, Arm, RISC-V, x86, and x86_64 stabilized in Rust 1.59 (nightly-2021-12-16): https://github.com/rust-lang/rust/pull/91728
    let no_asm = !version.probe(59, 2021, 12, 15);
    if no_asm {
        if version.nightly
            && version.probe(46, 2020, 6, 20)
            && ((target_arch != "x86" && target_arch != "x86_64") || version.llvm >= 10)
            && is_allowed_feature("asm")
        {
            // This feature was added in Rust 1.45 (nightly-2020-05-20), but
            // concat! in asm! requires Rust 1.46 (nightly-2020-06-21).
            // x86 intel syntax requires LLVM 10 (since Rust 1.53, the minimum
            // external LLVM version is 10+: https://github.com/rust-lang/rust/pull/83387).
            // The part of this feature we use has not been changed since nightly-2020-06-21
            // until it was stabilized, so it can safely be enabled in nightly for that period.
            println!("cargo:rustc-cfg=portable_atomic_unstable_asm");
        }
        println!("cargo:rustc-cfg=portable_atomic_no_asm");
    } else {
        match target_arch {
            "arm64ec" | "s390x" => {
                // asm! on Arm64EC and s390x stabilized in Rust 1.84 (nightly-2024-11-11): https://github.com/rust-lang/rust/pull/131781, https://github.com/rust-lang/rust/pull/131258
                if !version.probe(84, 2024, 11, 10) {
                    if version.nightly
                        && (target_arch != "s390x" || version.probe(71, 2023, 5, 8))
                        && is_allowed_feature("asm_experimental_arch")
                    {
                        // https://github.com/rust-lang/rust/pull/111331 merged in Rust 1.71 (nightly-2023-05-09).
                        // The part of this feature we use has not been changed since nightly-2023-05-09
                        // until it was stabilized, so it can safely be enabled in nightly for that period.
                        println!("cargo:rustc-cfg=portable_atomic_unstable_asm_experimental_arch");
                    } else {
                        println!("cargo:rustc-cfg=portable_atomic_no_asm");
                    }
                }
            }
            "powerpc64" => {
                // https://github.com/rust-lang/rust/pull/93868 merged in Rust 1.60 (nightly-2022-02-13).
                if version.nightly
                    && version.probe(60, 2022, 2, 12)
                    && is_allowed_feature("asm_experimental_arch")
                {
                    println!("cargo:rustc-cfg=portable_atomic_unstable_asm_experimental_arch");
                }
            }
            _ => {}
        }
    }

    // feature(cfg_target_has_atomic) stabilized in Rust 1.60 (nightly-2022-02-11): https://github.com/rust-lang/rust/pull/93824
    if !version.probe(60, 2022, 2, 10) {
        if version.nightly
            && version.probe(40, 2019, 10, 13)
            && is_allowed_feature("cfg_target_has_atomic")
        {
            // The part of this feature we use has not been changed since nightly-2019-10-14
            // until it was stabilized, so it can safely be enabled in nightly for that period.
            println!("cargo:rustc-cfg=portable_atomic_unstable_cfg_target_has_atomic");
        } else {
            println!("cargo:rustc-cfg=portable_atomic_no_cfg_target_has_atomic");
            let target = &*convert_custom_linux_target(target);
            if generated::NO_ATOMIC_CAS.contains(&target) {
                println!("cargo:rustc-cfg=portable_atomic_no_atomic_cas");
            }
            if generated::NO_ATOMIC_64.contains(&target) {
                println!("cargo:rustc-cfg=portable_atomic_no_atomic_64");
            } else {
                // Otherwise, assuming `"max-atomic-width" == 64` or `"max-atomic-width" == 128`.
            }
        }
    }
    // We don't need to use convert_custom_linux_target here because all linux targets have atomics.
    if generated::NO_ATOMIC.contains(&target) {
        println!("cargo:rustc-cfg=portable_atomic_no_atomic_load_store");
    }

    if version.llvm < 20 {
        println!("cargo:rustc-cfg=portable_atomic_pre_llvm_20");
        if version.llvm < 18 {
            println!("cargo:rustc-cfg=portable_atomic_pre_llvm_18");
            if version.llvm < 16 {
                println!("cargo:rustc-cfg=portable_atomic_pre_llvm_16");
                if version.llvm < 15 {
                    println!("cargo:rustc-cfg=portable_atomic_pre_llvm_15");
                }
            }
        }
    }

    if version.nightly {
        // `cfg(sanitize = "..")` is not stabilized.
        let sanitize = env::var("CARGO_CFG_SANITIZE").unwrap_or_default();
        if sanitize.contains("thread") {
            // Most kinds of sanitizers are not compatible with asm
            // (https://github.com/google/sanitizers/issues/192),
            // but it seems that ThreadSanitizer is the only one that can cause
            // false positives in our code.
            println!("cargo:rustc-cfg=portable_atomic_sanitize_thread");
        }
    }

    match target_arch {
        "x86_64" => {
            // cmpxchg16b_target_feature stabilized in Rust 1.69 (nightly-2023-03-01): https://github.com/rust-lang/rust/pull/106774
            if !version.probe(69, 2023, 2, 28) {
                println!("cargo:rustc-cfg=portable_atomic_no_cmpxchg16b_target_feature");
            }
            // For Miri and ThreadSanitizer.
            // https://github.com/rust-lang/rust/pull/109359 (includes https://github.com/rust-lang/stdarch/pull/1358) merged in Rust 1.70 (nightly-2023-03-24).
            if version.nightly && !version.probe(70, 2023, 3, 23) {
                println!("cargo:rustc-cfg=portable_atomic_no_cmpxchg16b_intrinsic");
            }

            // cmpxchg16b_target_feature stabilized in Rust 1.69.
            if needs_target_feature_fallback(&version, Some(69)) {
                // x86_64 Apple targets always support CMPXCHG16B:
                // https://github.com/rust-lang/rust/blob/1.68.0/compiler/rustc_target/src/spec/x86_64_apple_darwin.rs#L8
                // https://github.com/rust-lang/rust/blob/1.68.0/compiler/rustc_target/src/spec/apple_base.rs#L69-L70
                // (Windows (except Windows 7, since Rust 1.78) and Fuchsia (since Rust 1.87) targets
                // also enable CMPXCHG16B, but this branch is only used on pre-1.69 that
                // cmpxchg16b_target_feature is unstable.)
                // Script to get builtin targets that support CMPXCHG16B by default:
                // $ (for target in $(rustc -Z unstable-options --print all-target-specs-json | jq -r '. | to_entries[] | if .value.arch == "x86_64" then .key else empty end'); do rustc --print cfg --target "${target}" | grep -Fq '"cmpxchg16b"' && printf '%s\n' "${target}"; done)
                let is_apple = env::var("CARGO_CFG_TARGET_VENDOR").unwrap_or_default() == "apple";
                let cmpxchg16b = is_apple;
                // LLVM recognizes this also as cx16 target feature: https://godbolt.org/z/KM3jz616j
                // However, it is unlikely that rustc will support that name, so we ignore it.
                target_feature_fallback("cmpxchg16b", cmpxchg16b);
            }
        }
        "aarch64" | "arm64ec" => {
            // target_feature "lse2"/"lse128"/"rcpc3" is unstable and available on rustc side since nightly-2024-08-30: https://github.com/rust-lang/rust/pull/128192
            if !version.probe(82, 2024, 8, 29) || needs_target_feature_fallback(&version, None) {
                // FEAT_LSE2 doesn't imply FEAT_LSE. FEAT_LSE128 implies FEAT_LSE but not FEAT_LSE2.
                // AArch64 macOS always supports FEAT_LSE and FEAT_LSE2 because M1 is Armv8.4 with all features of Armv8.5 except FEAT_BTI:
                // https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/AArch64/AArch64Processors.td#L1230
                // https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/AArch64/AArch64Processors.td#L891
                // Script to get builtin targets that support FEAT_LSE/FEAT_LSE2 by default:
                // $ (for target in $(rustc -Z unstable-options --print all-target-specs-json | jq -r '. | to_entries[] | if .value.arch == "aarch64" or .value.arch == "arm64ec" then .key else empty end'); do rustc --print cfg --target "${target}" | grep -Fq '"lse"' && printf '%s\n' "${target}"; done)
                // $ (for target in $(rustc -Z unstable-options --print all-target-specs-json | jq -r '. | to_entries[] | if .value.arch == "aarch64" or .value.arch == "arm64ec" then .key else empty end'); do rustc --print cfg --target "${target}" | grep -Fq '"lse2"' && printf '%s\n' "${target}"; done)
                let is_macos = target_os == "macos";
                let mut lse = is_macos;
                target_feature_fallback("lse2", is_macos);
                lse |= target_feature_fallback("lse128", false);
                target_feature_fallback("rcpc3", false);
                // aarch64_target_feature stabilized in Rust 1.61.
                if needs_target_feature_fallback(&version, Some(61)) {
                    target_feature_fallback("lse", lse);
                }
            }
            // As of rustc 1.85, target_feature "lsfe" is not available on rustc side:
            // https://github.com/rust-lang/rust/blob/1.85.0/compiler/rustc_target/src/target_features.rs
            target_feature_fallback("lsfe", false);

            // As of Apple M1/M1 Pro, on Apple hardware, CAS-loop-based RMW is much slower than
            // LL/SC-loop-based RMW: https://github.com/taiki-e/portable-atomic/pull/89
            let is_apple = env::var("CARGO_CFG_TARGET_VENDOR").unwrap_or_default() == "apple";
            if is_apple || target_cpu().map_or(false, |cpu| cpu.starts_with("apple-")) {
                println!("cargo:rustc-cfg=portable_atomic_ll_sc_rmw");
            }
        }
        "arm" => {
            // For non-Linux/Android pre-v6 Arm (tier 3) with unsafe_assume_single_core enabled.
            // feature(isa_attribute) stabilized in Rust 1.67 (nightly-2022-11-06): https://github.com/rust-lang/rust/pull/102458
            if version.nightly && !version.probe(67, 2022, 11, 5) {
                println!("cargo:rustc-cfg=portable_atomic_unstable_isa_attribute");
            }

            if needs_target_feature_fallback(&version, None) {
                // #[cfg(target_feature = "v7")] and others don't work on stable.
                // armv7-unknown-linux-gnueabihf
                //    ^^
                let mut subarch =
                    strip_prefix(target, "arm").or_else(|| strip_prefix(target, "thumb")).unwrap();
                subarch = strip_prefix(subarch, "eb").unwrap_or(subarch); // ignore endianness
                subarch = subarch.split('-').next().unwrap(); // ignore vender/os/env
                subarch = subarch.split('.').next().unwrap(); // ignore .base/.main suffix
                let mut known = true;
                // See https://github.com/taiki-e/atomic-maybe-uninit/blob/HEAD/build.rs for details
                let mut mclass = false;
                match subarch {
                    "v7" | "v7a" | "v7neon" | "v7s" | "v7k" | "v8" | "v8a" | "v9" | "v9a" => {} // aclass
                    "v7r" | "v8r" | "v9r" => {} // rclass
                    "v6m" | "v7em" | "v7m" | "v8m" => mclass = true,
                    // arm-linux-androideabi is v5te
                    // https://github.com/rust-lang/rust/blob/1.84.0/compiler/rustc_target/src/spec/targets/arm_linux_androideabi.rs#L18
                    _ if target == "arm-linux-androideabi" => subarch = "v5te",
                    // armeb-unknown-linux-gnueabi is v8 & aclass
                    // https://github.com/rust-lang/rust/blob/1.84.0/compiler/rustc_target/src/spec/targets/armeb_unknown_linux_gnueabi.rs#L18
                    _ if target == "armeb-unknown-linux-gnueabi" => subarch = "v8",
                    // Legacy Arm architectures (pre-v7 except v6m) don't have *class target feature.
                    "" => subarch = "v6",
                    "v4t" | "v5te" | "v6" | "v6k" => {}
                    _ => {
                        known = false;
                        if env::var_os("PORTABLE_ATOMIC_DENY_WARNINGS").is_some() {
                            panic!("unrecognized Arm subarch: {}", target)
                        }
                        println!(
                            "cargo:warning={}: unrecognized Arm subarch: {}",
                            env!("CARGO_PKG_NAME"),
                            target
                        );
                    }
                }
                let v6 = known
                    && (subarch.starts_with("v6")
                        || subarch.starts_with("v7")
                        || subarch.starts_with("v8")
                        || subarch.starts_with("v9"));
                target_feature_fallback("v6", v6);
                target_feature_fallback("mclass", mclass);
            }
        }
        "riscv32" | "riscv64" => {
            // zabha and zacas imply zaamo in GCC, LLVM 20+, and Rust, but do not in LLVM 19.
            // However, enabling them without zaamo or a is not allowed in LLVM 19, so we can assume
            // zaamo is available when zabha is enabled).
            // https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/RISCV/RISCVFeatures.td#L233-L247
            // https://github.com/llvm/llvm-project/commit/956361ca080a689a96b6552d28681aaf0ad2f494
            // https://github.com/gcc-mirror/gcc/blob/08693e29ec186fd7941d0b73d4d466388971fe2f/gcc/config/riscv/arch-canonicalize#L45-L46
            // https://github.com/rust-lang/rust/pull/130877
            let mut zaamo = false;
            // target_feature "zacas" is unstable and available on rustc side since nightly-2025-02-26: https://github.com/rust-lang/rust/pull/137417
            if !version.probe(87, 2025, 2, 25) || needs_target_feature_fallback(&version, None) {
                // amocas.{w,d,q} (and amocas.{b,h} if zabha is also available)
                // available as experimental since LLVM 17 https://github.com/llvm/llvm-project/commit/29f630a1ddcbb03caa31b5002f0cbc105ff3a869
                // available non-experimental since LLVM 20 https://github.com/llvm/llvm-project/commit/614aeda93b2225c6eb42b00ba189ba7ca2585c60
                zaamo |= target_feature_fallback("zacas", false);
            }
            // target_feature "zaamo"/"zabha" is unstable and available on rustc side since nightly-2024-10-02: https://github.com/rust-lang/rust/pull/130877
            if !version.probe(83, 2024, 10, 1) || needs_target_feature_fallback(&version, None) {
                if version.llvm >= 19 {
                    // amo*.{b,h}
                    // available since LLVM 19 https://github.com/llvm/llvm-project/commit/89f87c387627150d342722b79c78cea2311cddf7 / https://github.com/llvm/llvm-project/commit/6b7444964a8d028989beee554a1f5c61d16a1cac
                    zaamo |= target_feature_fallback("zabha", false);
                }
                // amo*.{w,d}
                target_feature_fallback("zaamo", zaamo);
            }
        }
        "powerpc64" => {
            // target_feature "quadword-atomics" is unstable and available on rustc side since nightly-2024-09-28: https://github.com/rust-lang/rust/pull/130873
            if !version.probe(83, 2024, 9, 27) || needs_target_feature_fallback(&version, None) {
                let mut pwr8_features = false;
                if let Some(cpu) = target_cpu() {
                    if let Some(mut cpu_version) = strip_prefix(&cpu, "pwr") {
                        cpu_version = strip_suffix(cpu_version, "x").unwrap_or(cpu_version); // for pwr5x and pwr6x
                        if let Ok(cpu_version) = cpu_version.parse::<u32>() {
                            pwr8_features = cpu_version >= 8;
                        }
                    } else {
                        // https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/PowerPC/PPC.td#L714
                        // https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/PowerPC/PPC.td#L483
                        // On the minimum external LLVM version of the oldest rustc version which we can use asm_experimental_arch
                        // on this target (see CI config for more), "future" is based on pwr10 features.
                        // https://github.com/llvm/llvm-project/blob/llvmorg-12.0.0/llvm/lib/Target/PowerPC/PPC.td#L370
                        pwr8_features = cpu == "future" || cpu == "ppc64le";
                    }
                } else {
                    // powerpc64le is pwr8 by default https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/PowerPC/PPC.td#L714
                    // See also https://github.com/rust-lang/rust/issues/59932
                    pwr8_features = env::var("CARGO_CFG_TARGET_ENDIAN")
                        .expect("CARGO_CFG_TARGET_ENDIAN not set")
                        == "little";
                }
                // power8 features: https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/PowerPC/PPC.td#L409
                // lqarx and stqcx.
                target_feature_fallback("quadword-atomics", pwr8_features);
            }
        }
        "s390x" => {
            let mut arch9_features = false; // z196+
            let mut arch13_features = false; // z15+
            if let Some(cpu) = target_cpu() {
                // LLVM and GCC recognize the same names:
                // https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/SystemZ/SystemZProcessors.td
                // https://github.com/gcc-mirror/gcc/blob/releases/gcc-14.2.0/gcc/config/s390/s390.opt#L58-L125
                if let Some(arch_version) = strip_prefix(&cpu, "arch") {
                    if let Ok(arch_version) = arch_version.parse::<u32>() {
                        arch9_features = arch_version >= 9;
                        arch13_features = arch_version >= 13;
                    }
                } else {
                    match &*cpu {
                        "z196" | "zEC12" | "z13" | "z14" => arch9_features = true,
                        "z15" | "z16" => {
                            arch9_features = true;
                            arch13_features = true;
                        }
                        _ => {}
                    }
                }
            }
            // As of rustc 1.84, target_feature "fast-serialization"/"load-store-on-cond"/"distinct-ops"/"miscellaneous-extensions-3" is not available on rustc side:
            // https://github.com/rust-lang/rust/blob/1.84.0/compiler/rustc_target/src/target_features.rs#L547
            // arch9 features: https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/SystemZ/SystemZFeatures.td#L103
            // bcr 14,0
            target_feature_fallback("fast-serialization", arch9_features);
            // {l,st}oc{,g}{,r}
            target_feature_fallback("load-store-on-cond", arch9_features);
            // {al,sl,n,o,x}{,g}rk
            target_feature_fallback("distinct-ops", arch9_features);
            // arch13 features: https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/SystemZ/SystemZFeatures.td#L301
            // nand (nnr{,g}k), select (sel{,g}r), etc.
            target_feature_fallback("miscellaneous-extensions-3", arch13_features);
        }
        _ => {}
    }
}

// HACK: Currently, it seems that the only way to handle unstable target
// features on the stable is to parse the `-C target-feature` in RUSTFLAGS.
//
// - #[cfg(target_feature = "unstable_target_feature")] doesn't work on stable.
// - CARGO_CFG_TARGET_FEATURE excludes unstable target features on stable.
//
// As mentioned in the [RFC2045], unstable target features are also passed to LLVM
// (e.g., https://godbolt.org/z/4rr7rMcfG), so this hack works properly on stable.
//
// [RFC2045]: https://rust-lang.github.io/rfcs/2045-target-feature.html#backend-compilation-options
fn needs_target_feature_fallback(version: &Version, stable: Option<u32>) -> bool {
    match stable {
        // In these cases, cfg(target_feature = "...") would work, so skip emitting our own fallback target_feature cfg.
        _ if version.nightly => false,
        Some(stabilized) if version.minor >= stabilized => false,
        _ => true,
    }
}
fn target_feature_fallback(name: &str, mut has_target_feature: bool) -> bool {
    if let Some(rustflags) = env::var_os("CARGO_ENCODED_RUSTFLAGS") {
        for mut flag in rustflags.to_string_lossy().split('\x1f') {
            flag = strip_prefix(flag, "-C").unwrap_or(flag);
            if let Some(flag) = strip_prefix(flag, "target-feature=") {
                for s in flag.split(',') {
                    // TODO: Handles cases where a specific target feature
                    // implicitly enables another target feature.
                    match (s.as_bytes().first(), s.as_bytes().get(1..)) {
                        (Some(b'+'), Some(f)) if f == name.as_bytes() => has_target_feature = true,
                        (Some(b'-'), Some(f)) if f == name.as_bytes() => has_target_feature = false,
                        _ => {}
                    }
                }
            }
        }
    }
    if has_target_feature {
        println!("cargo:rustc-cfg=portable_atomic_target_feature=\"{}\"", name);
    }
    has_target_feature
}

fn target_cpu() -> Option<String> {
    let rustflags = env::var_os("CARGO_ENCODED_RUSTFLAGS")?;
    let rustflags = rustflags.to_string_lossy();
    let mut cpu = None;
    for mut flag in rustflags.split('\x1f') {
        flag = strip_prefix(flag, "-C").unwrap_or(flag);
        if let Some(flag) = strip_prefix(flag, "target-cpu=") {
            cpu = Some(flag);
        }
    }
    cpu.map(str::to_owned)
}

fn is_allowed_feature(name: &str) -> bool {
    // https://github.com/dtolnay/thiserror/pull/248
    if env::var_os("RUSTC_STAGE").is_some() {
        return false;
    }

    // allowed by default
    let mut allowed = true;
    if let Some(rustflags) = env::var_os("CARGO_ENCODED_RUSTFLAGS") {
        for mut flag in rustflags.to_string_lossy().split('\x1f') {
            flag = strip_prefix(flag, "-Z").unwrap_or(flag);
            if let Some(flag) = strip_prefix(flag, "allow-features=") {
                // If it is specified multiple times, the last value will be preferred.
                allowed = flag.split(',').any(|allowed| allowed == name);
            }
        }
    }
    allowed
}

// Adapted from https://github.com/crossbeam-rs/crossbeam/blob/crossbeam-utils-0.8.14/build-common.rs.
//
// The target triplets have the form of 'arch-vendor-system'.
//
// When building for Linux (e.g. the 'system' part is
// 'linux-something'), replace the vendor with 'unknown'
// so that mapping to rust standard targets happens correctly.
fn convert_custom_linux_target(target: &str) -> String {
    let mut parts: Vec<&str> = target.split('-').collect();
    let system = parts.get(2);
    if system == Some(&"linux") {
        parts[1] = "unknown";
    }
    parts.join("-")
}

// str::strip_prefix requires Rust 1.45
#[must_use]
fn strip_prefix<'a>(s: &'a str, pat: &str) -> Option<&'a str> {
    if s.starts_with(pat) { Some(&s[pat.len()..]) } else { None }
}
// str::strip_suffix requires Rust 1.45
#[must_use]
fn strip_suffix<'a>(s: &'a str, pat: &str) -> Option<&'a str> {
    if s.ends_with(pat) { Some(&s[..s.len() - pat.len()]) } else { None }
}
