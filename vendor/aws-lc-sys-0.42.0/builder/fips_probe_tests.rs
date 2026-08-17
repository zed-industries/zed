// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use super::*;
use crate::system_library::ResolvedLib;
use crate::{EnvGuard, ENV_MUTEX};
use std::path::{Path, PathBuf};

fn setup_test_env_with_target(os: &str, env: &str) -> Vec<EnvGuard> {
    let arch = std::env::consts::ARCH;
    let vars: &[(&str, &str)] = &[
        ("CARGO_CFG_TARGET_OS", os),
        ("CARGO_CFG_TARGET_ENV", env),
        ("CARGO_CFG_TARGET_FEATURE", ""),
        ("CARGO_CFG_TARGET_ARCH", arch),
        ("CARGO_CFG_TARGET_POINTER_WIDTH", "64"),
        ("TARGET", arch),
        ("CARGO_PKG_NAME", "aws-lc-sys"),
    ];

    vars.iter()
        .map(|(key, val)| EnvGuard::new(key, *val))
        .collect()
}

fn os_args_to_strings(args: &[std::ffi::OsString]) -> Vec<String> {
    args.iter()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn test_target_runner_parses_program_and_args() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let _target = EnvGuard::new("TARGET", "aarch64-unknown-linux-gnu");
    let _runner = EnvGuard::new(
        "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER",
        "qemu-aarch64 -L /usr/aarch64-linux-gnu",
    );
    assert_eq!(
        target_runner(),
        Some(vec![
            "qemu-aarch64".to_string(),
            "-L".to_string(),
            "/usr/aarch64-linux-gnu".to_string(),
        ])
    );
}

#[test]
fn test_target_runner_preserves_quoted_path_and_arg() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let _target = EnvGuard::new("TARGET", "aarch64-unknown-linux-gnu");
    let _runner = EnvGuard::new(
        "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER",
        "\"/opt/Program Files/qemu-aarch64\" -L \"/sdk root\"",
    );
    assert_eq!(
        target_runner(),
        Some(vec![
            "/opt/Program Files/qemu-aarch64".to_string(),
            "-L".to_string(),
            "/sdk root".to_string(),
        ])
    );
}

#[test]
fn test_target_runner_absent_is_none() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let _target = EnvGuard::new("TARGET", "aws-lc-fips-sys-no-runner-triple");
    assert_eq!(target_runner(), None);
}

#[test]
fn test_target_runner_blank_is_none() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let _target = EnvGuard::new("TARGET", "x86_64-pc-windows-gnu");
    let _runner = EnvGuard::new("CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER", "   ");
    assert_eq!(target_runner(), None);
}

#[test]
fn test_probe_link_args_unix_static_orders_deps_after_crypto() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let _env = setup_test_env_with_target("linux", "");
    let crypto = ResolvedLib {
        lib_type: OutputLibType::Static,
        name: "crypto".to_string(),
        path: PathBuf::from("/install/lib/libcrypto.a"),
    };

    let mut args = Vec::new();
    append_probe_link_args(
        &mut args,
        false,
        Path::new("/install/include"),
        &crypto,
        Path::new("/install/lib"),
        Path::new("/repo/builder/fips_link_probe.c"),
        Path::new("/out/aws_lc_fips_link_probe"),
    );
    let args = os_args_to_strings(&args);

    assert!(args.contains(&"-I/install/include".to_string()), "{args:?}");
    assert!(args.contains(&"-L/install/lib".to_string()), "{args:?}");

    let crypto_idx = args
        .iter()
        .position(|a| a == "/install/lib/libcrypto.a")
        .expect("libcrypto.a by path");
    let dl_idx = args.iter().position(|a| a == "-ldl").expect("-ldl");
    assert!(
        crypto_idx < dl_idx,
        "libcrypto.a must come before -ldl: {args:?}"
    );
    assert_eq!(
        args.last().map(String::as_str),
        Some("/out/aws_lc_fips_link_probe")
    );
    assert_eq!(args[args.len() - 2], "-o");
    assert!(!args.iter().any(|a| a == "-lcrypto"), "{args:?}");
}

#[test]
fn test_probe_link_args_unix_dynamic_omits_static_deps() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let _env = setup_test_env_with_target("linux", "");
    let crypto = ResolvedLib {
        lib_type: OutputLibType::Dynamic,
        name: "crypto-awslc".to_string(),
        path: PathBuf::from("/install/lib/libcrypto-awslc.so"),
    };

    let mut args = Vec::new();
    append_probe_link_args(
        &mut args,
        false,
        Path::new("/install/include"),
        &crypto,
        Path::new("/install/lib"),
        Path::new("/repo/builder/fips_link_probe.c"),
        Path::new("/out/aws_lc_fips_link_probe"),
    );
    let args = os_args_to_strings(&args);

    assert!(args.contains(&"-lcrypto-awslc".to_string()), "{args:?}");
    assert!(
        !args.iter().any(|a| a == "-ldl" || a == "-pthread"),
        "{args:?}"
    );
}

#[test]
fn test_probe_link_args_msvc_passes_lib_by_path_after_link() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let _env = setup_test_env_with_target("windows", "msvc");
    let temp_out = tempfile::tempdir().unwrap();
    let _out_dir = EnvGuard::new("OUT_DIR", temp_out.path());
    let crypto = ResolvedLib {
        lib_type: OutputLibType::Static,
        name: "crypto".to_string(),
        path: PathBuf::from("C:/install/lib/crypto.lib"),
    };

    let mut args = Vec::new();
    append_probe_link_args(
        &mut args,
        true,
        Path::new("C:/install/include"),
        &crypto,
        Path::new("C:/install/lib"),
        Path::new("C:/repo/builder/fips_link_probe.c"),
        Path::new("C:/out/aws_lc_fips_link_probe.exe"),
    );
    let args = os_args_to_strings(&args);

    let link_idx = args.iter().position(|a| a == "/link").expect("/link");
    let lib_idx = args
        .iter()
        .position(|a| a == "C:/install/lib/crypto.lib")
        .expect("crypto.lib by path");
    assert!(link_idx < lib_idx, "crypto.lib must follow /link: {args:?}");
    assert!(!args.iter().any(|a| a == "/nologo"), "{args:?}");
}
