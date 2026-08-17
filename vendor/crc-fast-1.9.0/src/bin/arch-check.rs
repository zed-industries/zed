// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This is a simple program that checks if the target architecture supports certain features.

#[cfg(target_arch = "aarch64")]
use std::arch::is_aarch64_feature_detected;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::arch::is_x86_feature_detected;

use crc_fast::get_calculator_target;
use crc_fast::CrcAlgorithm::{Crc32Iscsi, Crc32IsoHdlc, Crc64Nvme};

fn main() {
    // Check the target architecture and call the appropriate function
    #[cfg(target_arch = "aarch64")]
    aarch64_features();

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    x86_features();

    print_targets();

    print_cpu_info();
}

#[cfg(target_arch = "aarch64")]
fn aarch64_features() {
    let checkmark: char = '✓';

    println!("[AArch64] Checking for features...");

    if is_aarch64_feature_detected!("neon") {
        println!("  {checkmark} NEON",);
    } else {
        println!("  x NEON");
    }

    if is_aarch64_feature_detected!("crc") {
        println!("  {checkmark} CRC",);
    } else {
        println!("  x CRC");
    }

    if is_aarch64_feature_detected!("sha3") {
        println!("  {checkmark} SHA3\n",);
    } else {
        println!("  x SHA3\n");
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn x86_features() {
    let checkmark: char = '✓';

    println!("[X86] Checking for features...");

    if is_x86_feature_detected!("sse2") {
        println!("  {checkmark} SSE2",);
    } else {
        println!("  x SSE2");
    }

    if is_x86_feature_detected!("sse4.1") {
        println!("  {checkmark} SSE4.1",);
    } else {
        println!("  x SSE4.1");
    }

    if is_x86_feature_detected!("pclmulqdq") {
        println!("  {checkmark} PCLMULQDQ",);
    } else {
        println!("  x PCLMULQDQ");
    }

    if is_x86_feature_detected!("avx2") {
        println!("  {checkmark} AVX2",);
    } else {
        println!("  x AVX2");
    }

    if is_x86_feature_detected!("vpclmulqdq") {
        println!("  {checkmark} VPCLMULQDQ",);
    } else {
        println!("  x VPCLMULQDQ");
    }

    if is_x86_feature_detected!("avx512f") {
        println!("  {checkmark} AVX512F",);
    } else {
        println!("  x AVX512F");
    }

    if is_x86_feature_detected!("avx512vl") {
        println!("  {checkmark} AVX512VL\n",);
    } else {
        println!("  x AVX512VL\n");
    }
}

/// Print the acceleration targets
fn print_targets() {
    let checkmark: char = '✓';

    println!("[Acceleration targets]");

    println!(
        "  {} CRC-32/ISCSI target: {}",
        checkmark,
        get_calculator_target(Crc32Iscsi)
    );
    println!(
        "  {} CRC-32/ISO-HDLC target: {}",
        checkmark,
        get_calculator_target(Crc32IsoHdlc)
    );
    println!(
        "  {} CRC-64/NVME target: {}\n",
        checkmark,
        get_calculator_target(Crc64Nvme)
    );
}

/// Print the first entry of /proc/cpuinfo if it's available
fn print_cpu_info() {
    println!("\n[CPU Info]");
    if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
        // Split the content by double newlines and take the first entry
        if let Some(first_cpu) = cpuinfo.split("\n\n").next() {
            println!("{first_cpu}",);
        } else {
            println!("No CPU information found.");
        }
    } else {
        println!("Failed to read /proc/cpuinfo. This may not be available on your platform.\n");
    }
}
