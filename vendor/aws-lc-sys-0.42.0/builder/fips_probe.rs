// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::system_library::ResolvedLib;
use crate::{
    cargo_env, compiler_is_cl_like, emit_warning, execute_command, is_lto_flag, out_dir, target,
    target_os, OutputLibType,
};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Joins a literal flag prefix (e.g. `-I`, `-L`, `-Wl,-rpath,`) with a path
/// without forcing UTF-8.
fn prefixed_os_arg(prefix: &str, value: &OsStr) -> std::ffi::OsString {
    let mut s = std::ffi::OsString::from(prefix);
    s.push(value);
    s
}

/// Runs `program` with owned `OsString` args.
fn execute_os(program: &OsStr, args: &[std::ffi::OsString]) -> crate::CommandResult {
    let refs: Vec<&OsStr> = args.iter().map(std::ffi::OsString::as_os_str).collect();
    execute_command(program, &refs)
}

/// Extra libraries the FIPS link probe must name when linking a *static*
/// libcrypto. The probe is bare C and so lacks the std-provided pthread/dl/c
/// linkage a real Rust binary would have. Each entry must be linkable on the
/// target (Apple has no standalone `-ldl`) and is a no-op where folded into
/// libc (e.g. glibc >= 2.34).
fn fips_probe_static_deps() -> &'static [&'static str] {
    match target_os().as_str() {
        "linux" | "android" => &["-pthread", "-ldl", "-lm"],
        // *BSD: threads via `-pthread`; dl/m symbols live in libc.
        "freebsd" | "openbsd" | "netbsd" | "dragonfly" => &["-pthread"],
        // Apple (libSystem) and anything else (incl. windows-gnu): nothing
        // portable to add; a genuine missing dep still surfaces as a link
        // error whose diagnostic names transitive dependencies.
        _ => &[],
    }
}

/// Returns cc-rs's target compiler flags, minus LTO flags.
fn inherited_probe_args(compiler: &cc::Tool) -> Vec<std::ffi::OsString> {
    compiler
        .args()
        .iter()
        .filter(|arg| !arg.to_str().is_some_and(is_lto_flag))
        .cloned()
        .collect()
}

/// Appends the probe source and link arguments to the inherited compiler flags.
fn append_probe_link_args(
    args: &mut Vec<std::ffi::OsString>,
    is_cl_like: bool,
    include_dir: &Path,
    crypto_lib: &ResolvedLib,
    lib_dir: &Path,
    probe_src: &Path,
    exec_path: &Path,
) {
    if is_cl_like {
        let obj_path = out_dir().join("aws_lc_fips_link_probe.obj");
        args.extend([
            prefixed_os_arg("/I", include_dir.as_os_str()),
            probe_src.as_os_str().to_os_string(),
            prefixed_os_arg("/Fe:", exec_path.as_os_str()),
            prefixed_os_arg("/Fo:", obj_path.as_os_str()),
            "/link".into(),
            crypto_lib.path.as_os_str().to_os_string(),
        ]);
    } else {
        args.extend([
            prefixed_os_arg("-I", include_dir.as_os_str()),
            probe_src.as_os_str().to_os_string(),
            prefixed_os_arg("-L", lib_dir.as_os_str()),
            prefixed_os_arg("-Wl,-rpath,", lib_dir.as_os_str()),
        ]);
        if matches!(crypto_lib.lib_type, OutputLibType::Static) {
            args.push(crypto_lib.path.as_os_str().to_os_string());
            args.extend(fips_probe_static_deps().iter().copied().map(Into::into));
        } else {
            args.push(format!("-l{}", crypto_lib.name).into());
        }
        args.extend(["-o".into(), exec_path.as_os_str().to_os_string()]);
    }
}

/// Compiles and links the FIPS probe against the install's `libcrypto`,
/// returning the linked executable's path on success.
pub(crate) fn compile_fips_probe(
    manifest_dir: &Path,
    include_dir: &Path,
    crypto_lib: &ResolvedLib,
    lib_dir: &Path,
) -> Result<PathBuf, String> {
    let probe_src = manifest_dir.join("builder").join("fips_link_probe.c");
    if !probe_src.is_file() {
        return Err(format!(
            "FIPS probe source missing: {}",
            probe_src.display()
        ));
    }
    println!("cargo:rerun-if-changed={}", probe_src.display());

    let exec_path = out_dir().join(if target_os() == "windows" {
        "aws_lc_fips_link_probe.exe"
    } else {
        "aws_lc_fips_link_probe"
    });

    let cc_build = cc::Build::new();
    let compiler = cc_build.get_compiler();
    // Keyed on the compiler family (not the target ABI); unsupported families error below.
    if !(compiler.is_like_clang() || compiler.is_like_gnu() || compiler.is_like_msvc()) {
        return Err(format!(
            "FIPS verification requires a Clang-, GCC-, or MSVC-compatible compiler; \
             {} is not supported. Set CC to a supported compiler.",
            compiler.path().display(),
        ));
    }

    let mut args = inherited_probe_args(&compiler);
    append_probe_link_args(
        &mut args,
        // Probe link-flag syntax follows the compiler driver mode, not the ABI.
        compiler_is_cl_like(&compiler),
        include_dir,
        crypto_lib,
        lib_dir,
        &probe_src,
        &exec_path,
    );

    let result = execute_os(compiler.path().as_os_str(), &args);
    if !result.executed {
        return Err(format!(
            "FIPS verification failed: could not execute compiler {compiler}.\n\
             Spawn error: {spawn_error}",
            compiler = compiler.path().display(),
            spawn_error = result.spawn_error.as_deref().unwrap_or("unknown error"),
        ));
    }
    if !result.status {
        return Err(format!(
            "FIPS verification failed: linker rejected the probe against {lib}.\n\
             The library is likely not an AWS-LC FIPS build (BORINGSSL_integrity_test \
             is exported only by FIPS, non-ASAN builds); when cross-compiling it can \
             also indicate an architecture or CRT model (/MD vs /MT) mismatch.\n\
             Stderr: {stderr}",
            lib = crypto_lib.path.display(),
            stderr = result.stderr,
        ));
    }

    Ok(exec_path)
}

/// Verifies that the supplied system `libcrypto` is a usable AWS-LC FIPS
/// library by linking the FIPS-only probe and, when possible, running it.
pub(crate) fn verify_fips_install(
    manifest_dir: &Path,
    include_dir: &Path,
    crypto_lib: &ResolvedLib,
    lib_dir: &Path,
) -> Result<(), String> {
    let probe = compile_fips_probe(manifest_dir, include_dir, crypto_lib, lib_dir)?;
    run_fips_probe(&probe, crypto_lib)
}

const FIPS_MODE_OFF_EXIT_CODE: i32 = 42;

/// Runs the probe when the host can launch a target binary.
pub(crate) fn run_fips_probe(exec_path: &Path, crypto_lib: &ResolvedLib) -> Result<(), String> {
    let invocation: Option<(std::ffi::OsString, Vec<std::ffi::OsString>)> = if cargo_env("HOST")
        == target()
    {
        Some((exec_path.as_os_str().to_os_string(), Vec::new()))
    } else if let Some(mut runner) = target_runner() {
        // runner = [program, args...]; append the probe path last.
        let program = runner.remove(0);
        let mut run_args: Vec<std::ffi::OsString> = runner.into_iter().map(Into::into).collect();
        run_args.push(exec_path.as_os_str().to_os_string());
        Some((program.into(), run_args))
    } else {
        None
    };

    let Some((program, run_args)) = invocation else {
        let _ = std::fs::remove_file(exec_path);
        emit_warning(
            "Cross-compile detected with no target runner: skipping runtime FIPS_mode() check. \
             Build-time link probe passed; the runtime constructor in fips_runtime_check.c \
             will assert FIPS mode at process startup.",
        );
        return Ok(());
    };

    let runtime_result = execute_os(program.as_os_str(), &run_args);
    let _ = std::fs::remove_file(exec_path);

    if !runtime_result.executed {
        emit_warning(format!(
            "FIPS runtime check skipped: probe linked but could not be executed via {program}. \
             Relying on the build-time link probe and the runtime constructor. \
             Spawn error: {spawn_error}. Stderr: {stderr}",
            program = program.to_string_lossy(),
            spawn_error = runtime_result
                .spawn_error
                .as_deref()
                .unwrap_or("unknown error"),
            stderr = runtime_result.stderr,
        ));
        return Ok(());
    }
    if runtime_result.status {
        emit_warning(
            "FIPS verification: build-time link probe and runtime FIPS_mode() check passed.",
        );
        return Ok(());
    }
    if runtime_result.exit_code == Some(FIPS_MODE_OFF_EXIT_CODE) {
        return Err(format!(
            "FIPS verification failed: probe linked but FIPS_mode() returned 0 at \
             runtime against {lib}. The library exposes FIPS-only symbols but is \
             not running in FIPS mode (e.g. a FIPS+ASAN build, which the AWS-LC \
             FIPS module disables at runtime).\n\
             Stderr: {stderr}",
            lib = crypto_lib.path.display(),
            stderr = runtime_result.stderr,
        ));
    }
    emit_warning(format!(
        "FIPS runtime check skipped: probe ran via {program} but exited unexpectedly. \
         Relying on the build-time link probe and the runtime constructor. \
         Exit code: {exit_code:?}. Stderr: {stderr}",
        program = program.to_string_lossy(),
        exit_code = runtime_result.exit_code,
        stderr = runtime_result.stderr,
    ));

    Ok(())
}

/// Returns `CARGO_TARGET_<TRIPLE>_RUNNER` as `[program, args...]`, if set.
fn target_runner() -> Option<Vec<String>> {
    let key = format!(
        "CARGO_TARGET_{}_RUNNER",
        target().replace(['-', '.'], "_").to_uppercase()
    );
    println!("cargo:rerun-if-env-changed={key}");
    let value = std::env::var(&key).ok()?;
    match parse_shell_words(&value) {
        Ok(parts) if parts.is_empty() => None,
        Ok(parts) => Some(parts),
        Err(err) => {
            emit_warning(format!("Ignoring invalid {key}: {err}"));
            None
        }
    }
}

fn parse_shell_words(value: &str) -> Result<Vec<String>, String> {
    #[derive(Clone, Copy)]
    enum Quote {
        Single,
        Double,
    }

    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escape = false;

    for ch in value.chars() {
        if escape {
            current.push(ch);
            escape = false;
            continue;
        }

        match quote {
            Some(Quote::Single) => {
                if ch == '\'' {
                    quote = None;
                } else {
                    current.push(ch);
                }
            }
            Some(Quote::Double) => match ch {
                '"' => quote = None,
                '\\' => escape = true,
                _ => current.push(ch),
            },
            None => match ch {
                '\'' => quote = Some(Quote::Single),
                '"' => quote = Some(Quote::Double),
                '\\' => escape = true,
                _ if ch.is_whitespace() => {
                    if !current.is_empty() {
                        parts.push(std::mem::take(&mut current));
                    }
                }
                _ => current.push(ch),
            },
        }
    }

    if escape {
        return Err("runner ends with an unfinished escape".to_string());
    }
    if quote.is_some() {
        return Err("runner contains an unmatched quote".to_string());
    }
    if !current.is_empty() {
        parts.push(current);
    }

    Ok(parts)
}

#[cfg(test)]
#[path = "fips_probe_tests.rs"]
mod tests;
