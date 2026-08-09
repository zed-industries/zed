/// Pure command-construction functions for AVR toolchain invocations.
/// No subprocess spawning (that's Task 6) — only data construction and validation.
/// Mirrors prototypes/0002-arduino-core/build.sh's exact flags and invocation order.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use util::command::new_command;

/// Command specification for spawning a subprocess.
/// Pure data structure — no I/O, no execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub program: &'static str,
    pub args: Vec<String>,
    pub current_dir: Option<PathBuf>,
    pub env: HashMap<String, String>,
}

impl CommandSpec {
    pub fn new(program: &'static str) -> Self {
        CommandSpec {
            program,
            args: Vec::new(),
            current_dir: None,
            env: HashMap::new(),
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_current_dir(mut self, dir: PathBuf) -> Self {
        self.current_dir = Some(dir);
        self
    }

    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }
}

/// Arguments for compiling a single AVR source file (`.c`, `.cpp`, or `.S`).
/// Language and flags chosen by file extension.
/// Mirrors build.sh lines 20-35.
pub fn core_object_compile_args(
    source_path: &Path,
    output_path: &Path,
    mcu: &str,
    core_dir: &Path,
    variant_dir: &Path,
) -> anyhow::Result<CommandSpec> {
    let source_ext = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("source file has no extension"))?;

    let (program, mut args) = match source_ext {
        "c" => (
            "avr-gcc",
            vec![
                format!("-mmcu={}", mcu),
                "-Os".to_string(),
                "-w".to_string(),
                "-std=gnu11".to_string(),
                "-ffunction-sections".to_string(),
                "-fdata-sections".to_string(),
            ],
        ),
        "cpp" => (
            "avr-g++",
            vec![
                format!("-mmcu={}", mcu),
                "-Os".to_string(),
                "-w".to_string(),
                "-std=gnu++11".to_string(),
                "-fpermissive".to_string(),
                "-fno-exceptions".to_string(),
                "-fno-threadsafe-statics".to_string(),
                "-ffunction-sections".to_string(),
                "-fdata-sections".to_string(),
            ],
        ),
        "S" => (
            "avr-gcc",
            vec![
                format!("-mmcu={}", mcu),
                "-x".to_string(),
                "assembler-with-cpp".to_string(),
                "-w".to_string(),
                "-ffunction-sections".to_string(),
                "-fdata-sections".to_string(),
            ],
        ),
        _ => {
            return Err(anyhow::anyhow!(
                "unsupported source file extension: {}",
                source_ext
            ))
        }
    };

    // Add defines (from build.sh line 13)
    args.push("-DF_CPU=16000000L".to_string());
    args.push("-DARDUINO=10808".to_string());
    args.push("-DARDUINO_AVR_UNO".to_string());
    args.push("-DARDUINO_ARCH_AVR".to_string());

    // Add includes
    args.push(format!("-I{}", core_dir.display()));
    args.push(format!("-I{}", variant_dir.display()));

    // Add compilation flags
    args.push("-c".to_string());
    args.push(source_path.display().to_string());
    args.push("-o".to_string());
    args.push(output_path.display().to_string());

    Ok(CommandSpec {
        program,
        args,
        current_dir: None,
        env: HashMap::new(),
    })
}

/// Arguments for archiving compiled core objects.
/// Mirrors build.sh line 38: `avr-ar rcs <archive> <objects...>`
pub fn core_archive_args(archive_path: &Path, object_paths: &[PathBuf]) -> CommandSpec {
    let mut args = vec!["rcs".to_string(), archive_path.display().to_string()];
    for obj in object_paths {
        args.push(obj.display().to_string());
    }

    CommandSpec {
        program: "avr-ar",
        args,
        current_dir: None,
        env: HashMap::new(),
    }
}

/// Arguments for compiling the sketch C++ file.
/// Mirrors build.sh lines 41-43.
pub fn sketch_compile_args(
    sketch_path: &Path,
    output_path: &Path,
    mcu: &str,
    core_dir: &Path,
    variant_dir: &Path,
) -> CommandSpec {
    let args = vec![
        format!("-mmcu={}", mcu),
        "-Os".to_string(),
        "-std=gnu++11".to_string(),
        "-fpermissive".to_string(),
        "-fno-exceptions".to_string(),
        "-fno-threadsafe-statics".to_string(),
        "-ffunction-sections".to_string(),
        "-fdata-sections".to_string(),
        "-DF_CPU=16000000L".to_string(),
        "-DARDUINO=10808".to_string(),
        "-DARDUINO_AVR_UNO".to_string(),
        "-DARDUINO_ARCH_AVR".to_string(),
        format!("-I{}", core_dir.display()),
        format!("-I{}", variant_dir.display()),
        "-c".to_string(),
        sketch_path.display().to_string(),
        "-o".to_string(),
        output_path.display().to_string(),
    ];

    CommandSpec {
        program: "avr-g++",
        args,
        current_dir: None,
        env: HashMap::new(),
    }
}

/// Command to build the Rust firmware logic crate.
/// Mirrors build.sh lines 45-49.
pub fn rust_build_command(rust_dir: &Path, mcu: &str) -> CommandSpec {
    let mut env = HashMap::new();
    env.insert(
        "RUSTFLAGS".to_string(),
        format!("-C target-cpu={}", mcu),
    );

    CommandSpec {
        program: "cargo",
        args: vec![
            "build".to_string(),
            "--release".to_string(),
            "-Z".to_string(),
            "build-std=core".to_string(),
            "--target".to_string(),
            "avr-none".to_string(),
        ],
        current_dir: Some(rust_dir.to_path_buf()),
        env,
    }
}

/// Arguments for linking the final ELF.
/// Mirrors build.sh lines 52-55.
pub fn link_args(
    output_elf: &Path,
    sketch_obj: &Path,
    core_archive: &Path,
    rust_lib_dir: &Path,
    rust_crate_name: &str,
    mcu: &str,
) -> CommandSpec {
    let args = vec![
        format!("-mmcu={}", mcu),
        "-Os".to_string(),
        "-Wl,--gc-sections".to_string(),
        "-o".to_string(),
        output_elf.display().to_string(),
        sketch_obj.display().to_string(),
        core_archive.display().to_string(),
        format!("-L{}", rust_lib_dir.display()),
        format!("-l{}", rust_crate_name),
    ];

    CommandSpec {
        program: "avr-g++",
        args,
        current_dir: None,
        env: HashMap::new(),
    }
}

/// Arguments for converting ELF to Intel HEX.
/// Mirrors build.sh line 57: `avr-objcopy -O ihex -R .eeprom <elf> <hex>`
pub fn objcopy_args(elf_path: &Path, hex_path: &Path) -> CommandSpec {
    let args = vec![
        "-O".to_string(),
        "ihex".to_string(),
        "-R".to_string(),
        ".eeprom".to_string(),
        elf_path.display().to_string(),
        hex_path.display().to_string(),
    ];

    CommandSpec {
        program: "avr-objcopy",
        args,
        current_dir: None,
        env: HashMap::new(),
    }
}

/// Arguments for flashing the hex file to the device.
/// Matches brief spec: `-c <programmer> -p <mmcu> -P <port> -b <baud> -U flash:w:<hex>:i`
pub fn avrdude_flash_args(
    programmer: &str,
    mcu: &str,
    port: &str,
    baud: u32,
    hex_path: &Path,
) -> CommandSpec {
    let args = vec![
        "-c".to_string(),
        programmer.to_string(),
        "-p".to_string(),
        mcu.to_string(),
        "-P".to_string(),
        port.to_string(),
        "-b".to_string(),
        baud.to_string(),
        "-U".to_string(),
        format!("flash:w:{}:i", hex_path.display()),
    ];

    CommandSpec {
        program: "avrdude",
        args,
        current_dir: None,
        env: HashMap::new(),
    }
}

/// Parse `[package] name` from a Cargo.toml string.
/// Returns error if there's no `[package]` table.
pub fn parse_cargo_package_name(cargo_toml_content: &str) -> anyhow::Result<String> {
    #[derive(serde::Deserialize)]
    struct CargoToml {
        package: CargoPackage,
    }

    #[derive(serde::Deserialize)]
    struct CargoPackage {
        name: String,
    }

    let parsed: CargoToml = toml::from_str(cargo_toml_content)?;
    Ok(parsed.package.name)
}

/// One stage of the build/flash pipeline. Identifies which `CommandSpec`
/// failed when reporting a `BuildError`, so a toast can name the failing
/// step instead of just dumping raw stderr.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStep {
    Preflight,
    CoreCompile,
    CoreArchive,
    SketchCompile,
    RustBuild,
    ParseCrateName,
    Link,
    Objcopy,
    Flash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildError {
    pub step: BuildStep,
    pub message: String,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} failed: {}", self.step, self.message)
    }
}

impl std::error::Error for BuildError {}

/// Spawns `spec` as a subprocess and maps a non-zero exit (or a spawn
/// failure) to a `BuildError` tagged with `step`, capturing stderr so the
/// caller can surface a meaningful message instead of just "build failed".
async fn run(step: BuildStep, spec: &CommandSpec) -> Result<(), BuildError> {
    let mut command = new_command(spec.program);
    command.args(&spec.args);
    // Citadel's own process is itself launched under a pinned rustup
    // toolchain (its own rust-toolchain.toml), which sets RUSTUP_TOOLCHAIN
    // in this process's environment. That variable takes precedence over
    // rustup's file-based `rust-toolchain.toml` discovery, so without
    // clearing it here, spawning `cargo` for the *user's* rust/ crate would
    // silently build with Citadel's own toolchain instead of the project's
    // pinned one (surfacing as "the -Z flag is only accepted on nightly").
    // Harmless to remove for the non-cargo tools in this pipeline (avr-gcc,
    // avr-g++, avr-ar, avr-objcopy, avrdude), which don't read it.
    command.env_remove("RUSTUP_TOOLCHAIN");
    command.envs(&spec.env);
    if let Some(current_dir) = &spec.current_dir {
        command.current_dir(current_dir);
    }

    let output = command.output().await.map_err(|error| BuildError {
        step,
        message: error.to_string(),
    })?;

    if !output.status.success() {
        return Err(BuildError {
            step,
            message: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    Ok(())
}

/// Checks that every binary the pipeline shells out to is on `PATH`, so a
/// missing toolchain fails fast with the names of what's missing instead of
/// surfacing as an opaque "command not found" from whichever step happens to
/// run first.
pub async fn check_toolchain_available() -> Result<(), Vec<&'static str>> {
    const REQUIRED: &[&str] = &["avr-gcc", "avr-g++", "avr-ar", "avr-objcopy", "avrdude", "cargo"];

    let missing: Vec<&'static str> = REQUIRED
        .iter()
        .copied()
        .filter(|binary| which::which(binary).is_err())
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

/// Compiles every `.c`/`.cpp`/`.S` file under `<core_source_dir>/cores/arduino`
/// into `<cache_dir>/<mmcu>/core.a` and returns the archive path. If that
/// archive already exists, compilation is skipped entirely and the existing
/// path is returned — ArduinoCore-avr is paid for once per `mmcu`, not once
/// per build.
pub async fn ensure_core_archive(
    core_source_dir: &Path,
    cache_dir: &Path,
    mmcu: &str,
) -> Result<PathBuf, BuildError> {
    let archive_path = cache_dir.join(mmcu).join("core.a");
    if archive_path.exists() {
        return Ok(archive_path);
    }

    let core_dir = core_source_dir.join("cores").join("arduino");
    let variant_dir = core_source_dir.join("variants").join("standard");
    let object_dir = cache_dir.join(mmcu).join("core");

    std::fs::create_dir_all(&object_dir).map_err(|error| BuildError {
        step: BuildStep::CoreCompile,
        message: format!(
            "failed to create core object directory {}: {error}",
            object_dir.display()
        ),
    })?;

    let entries = std::fs::read_dir(&core_dir).map_err(|error| BuildError {
        step: BuildStep::CoreCompile,
        message: format!(
            "failed to read core source directory {}: {error}",
            core_dir.display()
        ),
    })?;

    let mut object_paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| BuildError {
            step: BuildStep::CoreCompile,
            message: error.to_string(),
        })?;
        let source_path = entry.path();
        let is_compilable_source = matches!(
            source_path.extension().and_then(|ext| ext.to_str()),
            Some("c") | Some("cpp") | Some("S")
        );
        if !is_compilable_source {
            continue;
        }

        let file_name = source_path.file_name().ok_or_else(|| BuildError {
            step: BuildStep::CoreCompile,
            message: format!("core source file has no file name: {}", source_path.display()),
        })?;
        let object_path = object_dir.join(format!("{}.o", file_name.to_string_lossy()));

        let spec = core_object_compile_args(&source_path, &object_path, mmcu, &core_dir, &variant_dir)
            .map_err(|error| BuildError {
                step: BuildStep::CoreCompile,
                message: error.to_string(),
            })?;
        run(BuildStep::CoreCompile, &spec).await?;

        object_paths.push(object_path);
    }

    if object_paths.is_empty() {
        return Err(BuildError {
            step: BuildStep::CoreCompile,
            message: format!(
                "no compilable core source files found under {}",
                core_dir.display()
            ),
        });
    }

    let archive_spec = core_archive_args(&archive_path, &object_paths);
    run(BuildStep::CoreArchive, &archive_spec).await?;

    Ok(archive_path)
}

/// Everything the pipeline needs to build and flash one project onto one
/// connected board.
pub struct BuildTarget {
    pub project_root: PathBuf,
    pub core_source_dir: PathBuf,
    pub core_cache_dir: PathBuf,
    pub mmcu: String,
    pub port_name: String,
    pub avrdude_programmer: String,
    pub avrdude_baud: u32,
}

/// Runs the full pipeline: preflight toolchain check, core cache (compiles
/// ArduinoCore-avr once per `mmcu` and reuses it after), sketch compile,
/// Rust firmware build, crate-name parsing, link, objcopy, and avrdude
/// flash. Returns the produced `.hex` path on success.
pub async fn build_and_flash(target: BuildTarget) -> Result<PathBuf, BuildError> {
    check_toolchain_available()
        .await
        .map_err(|missing_binaries| BuildError {
            step: BuildStep::Preflight,
            message: format!(
                "missing required toolchain binaries on PATH: {}",
                missing_binaries.join(", ")
            ),
        })?;

    let core_archive = ensure_core_archive(&target.core_source_dir, &target.core_cache_dir, &target.mmcu).await?;

    let core_dir = target.core_source_dir.join("cores").join("arduino");
    let variant_dir = target.core_source_dir.join("variants").join("standard");

    let build_dir = target.project_root.join("build");
    std::fs::create_dir_all(&build_dir).map_err(|error| BuildError {
        step: BuildStep::SketchCompile,
        message: format!("failed to create build directory {}: {error}", build_dir.display()),
    })?;

    let sketch_path = target.project_root.join("cpp").join("io.cpp");
    let sketch_object = build_dir.join("sketch.o");
    let sketch_spec = sketch_compile_args(&sketch_path, &sketch_object, &target.mmcu, &core_dir, &variant_dir);
    run(BuildStep::SketchCompile, &sketch_spec).await?;

    let rust_dir = target.project_root.join("rust");
    let rust_spec = rust_build_command(&rust_dir, &target.mmcu);
    run(BuildStep::RustBuild, &rust_spec).await?;

    let cargo_toml_path = rust_dir.join("Cargo.toml");
    let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path).map_err(|error| BuildError {
        step: BuildStep::ParseCrateName,
        message: format!("failed to read {}: {error}", cargo_toml_path.display()),
    })?;
    let crate_name =
        parse_cargo_package_name(&cargo_toml_content).map_err(|error| BuildError {
            step: BuildStep::ParseCrateName,
            message: error.to_string(),
        })?;

    let rust_lib_dir = rust_dir.join("target").join("avr-none").join("release");
    let firmware_elf = build_dir.join("firmware.elf");
    let link_spec = link_args(
        &firmware_elf,
        &sketch_object,
        &core_archive,
        &rust_lib_dir,
        &crate_name,
        &target.mmcu,
    );
    run(BuildStep::Link, &link_spec).await?;

    let firmware_hex = build_dir.join("firmware.hex");
    let objcopy_spec = objcopy_args(&firmware_elf, &firmware_hex);
    run(BuildStep::Objcopy, &objcopy_spec).await?;

    let flash_spec = avrdude_flash_args(
        &target.avrdude_programmer,
        &target.mmcu,
        &target.port_name,
        target.avrdude_baud,
        &firmware_hex,
    );
    run(BuildStep::Flash, &flash_spec).await?;

    Ok(firmware_hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_object_compile_args_c_file() {
        let spec = core_object_compile_args(
            Path::new("src/main.c"),
            Path::new("build/main.c.o"),
            "atmega328p",
            Path::new("vendor/cores/arduino"),
            Path::new("vendor/variants/standard"),
        )
        .expect("should compile");

        assert_eq!(spec.program, "avr-gcc");
        assert_eq!(
            spec.args,
            vec![
                "-mmcu=atmega328p".to_string(),
                "-Os".to_string(),
                "-w".to_string(),
                "-std=gnu11".to_string(),
                "-ffunction-sections".to_string(),
                "-fdata-sections".to_string(),
                "-DF_CPU=16000000L".to_string(),
                "-DARDUINO=10808".to_string(),
                "-DARDUINO_AVR_UNO".to_string(),
                "-DARDUINO_ARCH_AVR".to_string(),
                "-Ivendor/cores/arduino".to_string(),
                "-Ivendor/variants/standard".to_string(),
                "-c".to_string(),
                "src/main.c".to_string(),
                "-o".to_string(),
                "build/main.c.o".to_string(),
            ]
        );
    }

    #[test]
    fn test_core_object_compile_args_cpp_file() {
        let spec = core_object_compile_args(
            Path::new("src/main.cpp"),
            Path::new("build/main.cpp.o"),
            "atmega328p",
            Path::new("vendor/cores/arduino"),
            Path::new("vendor/variants/standard"),
        )
        .expect("should compile");

        assert_eq!(spec.program, "avr-g++");
        assert_eq!(
            spec.args,
            vec![
                "-mmcu=atmega328p".to_string(),
                "-Os".to_string(),
                "-w".to_string(),
                "-std=gnu++11".to_string(),
                "-fpermissive".to_string(),
                "-fno-exceptions".to_string(),
                "-fno-threadsafe-statics".to_string(),
                "-ffunction-sections".to_string(),
                "-fdata-sections".to_string(),
                "-DF_CPU=16000000L".to_string(),
                "-DARDUINO=10808".to_string(),
                "-DARDUINO_AVR_UNO".to_string(),
                "-DARDUINO_ARCH_AVR".to_string(),
                "-Ivendor/cores/arduino".to_string(),
                "-Ivendor/variants/standard".to_string(),
                "-c".to_string(),
                "src/main.cpp".to_string(),
                "-o".to_string(),
                "build/main.cpp.o".to_string(),
            ]
        );
    }

    #[test]
    fn test_core_object_compile_args_asm_file() {
        let spec = core_object_compile_args(
            Path::new("src/main.S"),
            Path::new("build/main.S.o"),
            "atmega328p",
            Path::new("vendor/cores/arduino"),
            Path::new("vendor/variants/standard"),
        )
        .expect("should compile");

        assert_eq!(spec.program, "avr-gcc");
        assert_eq!(
            spec.args,
            vec![
                "-mmcu=atmega328p".to_string(),
                "-x".to_string(),
                "assembler-with-cpp".to_string(),
                "-w".to_string(),
                "-ffunction-sections".to_string(),
                "-fdata-sections".to_string(),
                "-DF_CPU=16000000L".to_string(),
                "-DARDUINO=10808".to_string(),
                "-DARDUINO_AVR_UNO".to_string(),
                "-DARDUINO_ARCH_AVR".to_string(),
                "-Ivendor/cores/arduino".to_string(),
                "-Ivendor/variants/standard".to_string(),
                "-c".to_string(),
                "src/main.S".to_string(),
                "-o".to_string(),
                "build/main.S.o".to_string(),
            ]
        );
    }

    #[test]
    fn test_core_object_compile_args_invalid_extension() {
        let result = core_object_compile_args(
            Path::new("src/main.invalid"),
            Path::new("build/main.o"),
            "atmega328p",
            Path::new("vendor/cores/arduino"),
            Path::new("vendor/variants/standard"),
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsupported source file extension"));
    }

    #[test]
    fn test_core_archive_args() {
        let archive = Path::new("build/core.a");
        let objects = vec![
            PathBuf::from("build/core/a.o"),
            PathBuf::from("build/core/b.o"),
        ];

        let spec = core_archive_args(archive, &objects);

        assert_eq!(spec.program, "avr-ar");
        assert_eq!(
            spec.args,
            vec![
                "rcs".to_string(),
                "build/core.a".to_string(),
                "build/core/a.o".to_string(),
                "build/core/b.o".to_string(),
            ]
        );
    }

    #[test]
    fn test_sketch_compile_args() {
        let spec = sketch_compile_args(
            Path::new("cpp/sketch.cpp"),
            Path::new("build/sketch.o"),
            "atmega328p",
            Path::new("vendor/cores/arduino"),
            Path::new("vendor/variants/standard"),
        );

        assert_eq!(spec.program, "avr-g++");
        assert_eq!(
            spec.args,
            vec![
                "-mmcu=atmega328p".to_string(),
                "-Os".to_string(),
                "-std=gnu++11".to_string(),
                "-fpermissive".to_string(),
                "-fno-exceptions".to_string(),
                "-fno-threadsafe-statics".to_string(),
                "-ffunction-sections".to_string(),
                "-fdata-sections".to_string(),
                "-DF_CPU=16000000L".to_string(),
                "-DARDUINO=10808".to_string(),
                "-DARDUINO_AVR_UNO".to_string(),
                "-DARDUINO_ARCH_AVR".to_string(),
                "-Ivendor/cores/arduino".to_string(),
                "-Ivendor/variants/standard".to_string(),
                "-c".to_string(),
                "cpp/sketch.cpp".to_string(),
                "-o".to_string(),
                "build/sketch.o".to_string(),
            ]
        );
    }

    #[test]
    fn test_rust_build_command() {
        let spec = rust_build_command(Path::new("rust"), "atmega328p");

        assert_eq!(spec.program, "cargo");
        assert_eq!(spec.current_dir, Some(PathBuf::from("rust")));
        assert_eq!(
            spec.env.get("RUSTFLAGS"),
            Some(&"-C target-cpu=atmega328p".to_string())
        );
        assert_eq!(
            spec.args,
            vec![
                "build".to_string(),
                "--release".to_string(),
                "-Z".to_string(),
                "build-std=core".to_string(),
                "--target".to_string(),
                "avr-none".to_string(),
            ]
        );
    }

    #[test]
    fn test_link_args() {
        let spec = link_args(
            Path::new("build/firmware.elf"),
            Path::new("build/sketch.o"),
            Path::new("build/core.a"),
            Path::new("rust/target/avr-none/release"),
            "citadel_logic",
            "atmega328p",
        );

        assert_eq!(spec.program, "avr-g++");
        assert_eq!(
            spec.args,
            vec![
                "-mmcu=atmega328p".to_string(),
                "-Os".to_string(),
                "-Wl,--gc-sections".to_string(),
                "-o".to_string(),
                "build/firmware.elf".to_string(),
                "build/sketch.o".to_string(),
                "build/core.a".to_string(),
                "-Lrust/target/avr-none/release".to_string(),
                "-lcitadel_logic".to_string(),
            ]
        );
    }

    #[test]
    fn test_objcopy_args() {
        let spec = objcopy_args(Path::new("build/firmware.elf"), Path::new("build/firmware.hex"));

        assert_eq!(spec.program, "avr-objcopy");
        assert_eq!(
            spec.args,
            vec![
                "-O".to_string(),
                "ihex".to_string(),
                "-R".to_string(),
                ".eeprom".to_string(),
                "build/firmware.elf".to_string(),
                "build/firmware.hex".to_string(),
            ]
        );
    }

    #[test]
    fn test_avrdude_flash_args() {
        let spec = avrdude_flash_args(
            "arduino",
            "atmega328p",
            "/dev/ttyUSB0",
            115200,
            Path::new("build/firmware.hex"),
        );

        assert_eq!(spec.program, "avrdude");
        assert_eq!(
            spec.args,
            vec![
                "-c".to_string(),
                "arduino".to_string(),
                "-p".to_string(),
                "atmega328p".to_string(),
                "-P".to_string(),
                "/dev/ttyUSB0".to_string(),
                "-b".to_string(),
                "115200".to_string(),
                "-U".to_string(),
                "flash:w:build/firmware.hex:i".to_string(),
            ]
        );
    }

    #[test]
    fn test_parse_cargo_package_name() {
        let cargo_toml = r#"
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
"#;

        let name = parse_cargo_package_name(cargo_toml).expect("should parse");
        assert_eq!(name, "my_project");
    }

    #[test]
    fn test_parse_cargo_package_name_missing_package() {
        let cargo_toml = r#"
[dependencies]
serde = "1.0"
"#;

        let result = parse_cargo_package_name(cargo_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_cargo_package_name_missing_name() {
        let cargo_toml = r#"
[package]
version = "0.1.0"
"#;

        let result = parse_cargo_package_name(cargo_toml);
        assert!(result.is_err());
    }
}
