/// Pure command-construction functions for AVR toolchain invocations.
/// No subprocess spawning (that's Task 6) — only data construction and validation.
/// Mirrors prototypes/0002-arduino-core/build.sh's exact flags and invocation order.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
        assert!(spec.args.contains(&"-std=gnu11".to_string()));
        assert!(spec.args.contains(&"-Os".to_string()));
        assert!(spec.args.contains(&"-w".to_string()));
        assert!(spec.args.contains(&"-ffunction-sections".to_string()));
        assert!(spec.args.contains(&"-fdata-sections".to_string()));
        assert!(spec.args.contains(&"-DF_CPU=16000000L".to_string()));
        assert!(spec.args.contains(&"-DARDUINO=10808".to_string()));
        assert!(spec.args.contains(&"-DARDUINO_AVR_UNO".to_string()));
        assert!(spec.args.contains(&"-DARDUINO_ARCH_AVR".to_string()));
        assert!(spec.args.contains(&"-Ivendor/cores/arduino".to_string()));
        assert!(spec.args.contains(&"-Ivendor/variants/standard".to_string()));
        assert!(spec.args.contains(&"src/main.c".to_string()));
        assert!(spec.args.contains(&"build/main.c.o".to_string()));
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
        assert!(spec.args.contains(&"-std=gnu++11".to_string()));
        assert!(spec.args.contains(&"-fpermissive".to_string()));
        assert!(spec.args.contains(&"-fno-exceptions".to_string()));
        assert!(spec.args.contains(&"-fno-threadsafe-statics".to_string()));
        assert!(spec.args.contains(&"-Os".to_string()));
        assert!(spec.args.contains(&"-w".to_string()));
        assert!(spec.args.contains(&"-ffunction-sections".to_string()));
        assert!(spec.args.contains(&"-fdata-sections".to_string()));
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
        assert!(spec.args.contains(&"-x".to_string()));
        assert!(spec.args.contains(&"assembler-with-cpp".to_string()));
        assert!(spec.args.contains(&"-w".to_string()));
        assert!(spec.args.contains(&"-ffunction-sections".to_string()));
        assert!(spec.args.contains(&"-fdata-sections".to_string()));
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
        assert_eq!(spec.args[0], "rcs");
        assert!(spec.args.contains(&"build/core.a".to_string()));
        assert!(spec.args.contains(&"build/core/a.o".to_string()));
        assert!(spec.args.contains(&"build/core/b.o".to_string()));
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
        assert!(spec.args.contains(&"-std=gnu++11".to_string()));
        assert!(spec.args.contains(&"-fpermissive".to_string()));
        assert!(spec.args.contains(&"-fno-exceptions".to_string()));
        assert!(spec.args.contains(&"-fno-threadsafe-statics".to_string()));
        assert!(spec.args.contains(&"-Os".to_string()));
        assert!(spec.args.contains(&"-ffunction-sections".to_string()));
        assert!(spec.args.contains(&"-fdata-sections".to_string()));
        assert!(spec.args.contains(&"-DF_CPU=16000000L".to_string()));
        assert!(spec.args.contains(&"-DARDUINO=10808".to_string()));
        assert!(spec.args.contains(&"-DARDUINO_AVR_UNO".to_string()));
        assert!(spec.args.contains(&"-DARDUINO_ARCH_AVR".to_string()));
        assert!(spec.args.contains(&"cpp/sketch.cpp".to_string()));
        assert!(spec.args.contains(&"build/sketch.o".to_string()));
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
        assert!(spec.args.contains(&"build".to_string()));
        assert!(spec.args.contains(&"--release".to_string()));
        assert!(spec.args.contains(&"-Z".to_string()));
        assert!(spec.args.contains(&"build-std=core".to_string()));
        assert!(spec.args.contains(&"--target".to_string()));
        assert!(spec.args.contains(&"avr-none".to_string()));
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
        assert!(spec.args.contains(&"-mmcu=atmega328p".to_string()));
        assert!(spec.args.contains(&"-Os".to_string()));
        assert!(spec.args.contains(&"-Wl,--gc-sections".to_string()));
        assert!(spec.args.contains(&"build/sketch.o".to_string()));
        assert!(spec.args.contains(&"build/core.a".to_string()));
        assert!(spec
            .args
            .contains(&"-Lrust/target/avr-none/release".to_string()));
        assert!(spec.args.contains(&"-lcitadel_logic".to_string()));
        assert!(spec.args.contains(&"build/firmware.elf".to_string()));
    }

    #[test]
    fn test_objcopy_args() {
        let spec = objcopy_args(Path::new("build/firmware.elf"), Path::new("build/firmware.hex"));

        assert_eq!(spec.program, "avr-objcopy");
        assert_eq!(spec.args[0], "-O");
        assert_eq!(spec.args[1], "ihex");
        assert_eq!(spec.args[2], "-R");
        assert_eq!(spec.args[3], ".eeprom");
        assert!(spec.args.contains(&"build/firmware.elf".to_string()));
        assert!(spec.args.contains(&"build/firmware.hex".to_string()));
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
        assert!(spec.args.contains(&"-c".to_string()));
        assert!(spec.args.contains(&"arduino".to_string()));
        assert!(spec.args.contains(&"-p".to_string()));
        assert!(spec.args.contains(&"atmega328p".to_string()));
        assert!(spec.args.contains(&"-P".to_string()));
        assert!(spec.args.contains(&"/dev/ttyUSB0".to_string()));
        assert!(spec.args.contains(&"-b".to_string()));
        assert!(spec.args.contains(&"115200".to_string()));
        assert!(spec.args.contains(&"-U".to_string()));
        assert!(spec
            .args
            .contains(&"flash:w:build/firmware.hex:i".to_string()));
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
