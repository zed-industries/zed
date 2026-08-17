extern crate cbindgen;

use cbindgen::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::{env, fs, str};
use tempfile::NamedTempFile;

use pretty_assertions::assert_eq;

// Set automatically by cargo for integration tests
static CBINDGEN_PATH: &str = env!("CARGO_BIN_EXE_cbindgen");

fn style_str(style: Style) -> &'static str {
    match style {
        Style::Both => "both",
        Style::Tag => "tag",
        Style::Type => "type",
    }
}

struct CBindgenOutput {
    bindings_content: Vec<u8>,
    depfile_content: Option<String>,
    symfile_content: Option<String>,
}

fn run_cbindgen(
    path: &Path,
    output: Option<&Path>,
    language: Language,
    cpp_compat: bool,
    style: Option<Style>,
    generate_depfile: bool,
    package_version: bool,
    generate_symfile: bool,
) -> CBindgenOutput {
    assert!(
        !output.is_none() || !(generate_depfile || generate_symfile),
        "generating a depfile or symfile requires outputting to a path"
    );
    let program = Path::new(CBINDGEN_PATH);
    let mut command = Command::new(program);
    if let Some(output) = output {
        command.arg("--output").arg(output);
    }

    let depfile = if generate_depfile {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        command.arg("--depfile").arg(tmp.path());
        Some(tmp)
    } else {
        None
    };

    let symfile = if generate_symfile {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        command.arg("--symfile").arg(tmp.path());
        Some(tmp)
    } else {
        None
    };

    match language {
        Language::Cxx => {}
        Language::C => {
            command.arg("--lang").arg("c");

            if cpp_compat {
                command.arg("--cpp-compat");
            }
        }
        Language::Cython => {
            command.arg("--lang").arg("cython");
        }
    }

    if package_version {
        command.arg("--package-version");
    }

    if let Some(style) = style {
        command.arg("--style").arg(style_str(style));
    }

    let config = path.with_extension("toml");
    if config.exists() {
        command.arg("--config").arg(config);
    }

    command.arg(path);

    println!("Running: {:?}", command);
    let cbindgen_output = command.output().expect("failed to execute process");

    assert!(
        cbindgen_output.status.success(),
        "cbindgen failed: {:?} with error: {}",
        output,
        str::from_utf8(&cbindgen_output.stderr).unwrap_or_default()
    );

    let bindings_content = if let Some(output_path) = output {
        let mut bindings = Vec::new();
        // Ignore errors here, we have assertions on the expected output later.
        let _ = File::open(output_path).map(|mut file| {
            let _ = file.read_to_end(&mut bindings);
        });
        bindings
    } else {
        cbindgen_output.stdout
    };

    fn read_to_string(f: NamedTempFile) -> String {
        std::fs::read_to_string(&f).expect(&format!("Failed to read file as String: {:?}", f))
    }
    let depfile_content = depfile.map(read_to_string);
    let symfile_content = symfile.map(read_to_string);

    CBindgenOutput {
        bindings_content,
        depfile_content,
        symfile_content,
    }
}

fn compile(
    cbindgen_output: &Path,
    tests_path: &Path,
    tmp_dir: &Path,
    language: Language,
    style: Option<Style>,
    skip_warning_as_error: bool,
) {
    let cc = match language {
        Language::Cxx => env::var("CXX").unwrap_or_else(|_| "g++".to_owned()),
        Language::C => env::var("CC").unwrap_or_else(|_| "gcc".to_owned()),
        Language::Cython => env::var("CYTHON").unwrap_or_else(|_| "cython".to_owned()),
    };

    let file_name = cbindgen_output
        .file_name()
        .expect("cbindgen output should be a file");
    let mut object = tmp_dir.join(file_name);
    object.set_extension("o");

    let mut command = Command::new(cc);
    match language {
        Language::Cxx | Language::C => {
            command.arg("-D").arg("DEFINED");
            command.arg("-I").arg(tests_path);
            command.arg("-Wall");
            if !skip_warning_as_error {
                command.arg("-Werror");
            }
            // `swift_name` is not recognzied by gcc.
            command.arg("-Wno-attributes");
            // clang warns about unused const variables.
            command.arg("-Wno-unused-const-variable");
            // clang also warns about returning non-instantiated templates (they could
            // be specialized, but they're not so it's fine).
            command.arg("-Wno-return-type-c-linkage");
            // deprecated warnings should not be errors as it's intended
            command.arg("-Wno-deprecated-declarations");

            if let Language::Cxx = language {
                // enum class is a c++11 extension which makes g++ on macos 10.14 error out
                // inline variables are are a c++17 extension
                command.arg("-std=c++17");
                // Prevents warnings when compiling .c files as c++.
                command.arg("-x").arg("c++");
                if let Ok(extra_flags) = env::var("CXXFLAGS") {
                    command.args(extra_flags.split_whitespace());
                }
            } else if let Ok(extra_flags) = env::var("CFLAGS") {
                command.args(extra_flags.split_whitespace());
            }

            if let Some(style) = style {
                command.arg("-D");
                command.arg(format!(
                    "CBINDGEN_STYLE_{}",
                    style_str(style).to_uppercase()
                ));
            }

            command.arg("-o").arg(&object);
            command.arg("-c").arg(cbindgen_output);
        }
        Language::Cython => {
            command.arg("-Wextra");
            if !skip_warning_as_error {
                // Our tests contain code that is deprecated in Cython 3.0.
                // Allowing warnings buys a little time.
                // command.arg("-Werror");
            }
            command.arg("-3");
            command.arg("-o").arg(&object);
            command.arg(cbindgen_output);
        }
    }

    println!("Running: {:?}", command);
    let out = command.output().expect("failed to compile");
    assert!(out.status.success(), "Output failed to compile: {:?}", out);

    if object.exists() {
        fs::remove_file(object).unwrap();
    }
}

const SKIP_WARNING_AS_ERROR_SUFFIX: &str = ".skip_warning_as_error";

#[allow(clippy::too_many_arguments)]
fn run_compile_test(
    name: &'static str,
    path: &Path,
    tmp_dir: &Path,
    language: Language,
    cpp_compat: bool,
    style: Option<Style>,
    cbindgen_outputs: &mut HashSet<Vec<u8>>,
    package_version: bool,
    generate_symfile: bool,
) {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let tests_path = Path::new(&crate_dir).join("tests");
    let mut generated_file = tests_path.join("expectations");
    let mut generated_symfile = tests_path.join("expectations-symbols");
    fs::create_dir_all(&generated_file).unwrap();
    fs::create_dir_all(&generated_symfile).unwrap();

    let verify = env::var_os("CBINDGEN_TEST_VERIFY").is_some();
    let no_compile = env::var_os("CBINDGEN_TEST_NO_COMPILE").is_some();

    let style_ext = style
        // Cython is sensitive to dots, so we can't include any dots.
        .map(|style| match style {
            Style::Both => "_both",
            Style::Tag => "_tag",
            Style::Type => "",
        })
        .unwrap_or_default();
    let lang_ext = match language {
        Language::Cxx => ".cpp",
        Language::C if cpp_compat => ".compat.c",
        Language::C => ".c",
        // cbindgen is supposed to generate declaration files (`.pxd`), but `cython` compiler
        // is extension-sensitive and won't work on them, so we use implementation files (`.pyx`)
        // in the test suite.
        Language::Cython => ".pyx",
    };

    let skip_warning_as_error = name.rfind(SKIP_WARNING_AS_ERROR_SUFFIX).is_some();

    let source_file =
        format!("{}{}{}", name, style_ext, lang_ext).replace(SKIP_WARNING_AS_ERROR_SUFFIX, "");
    let symbols_file = format!("{source_file}.sym");

    generated_file.push(source_file);
    generated_symfile.push(symbols_file);

    let (output_file, generate_depfile, generate_symfile) = if verify {
        (None, false, false)
    } else {
        (
            Some(generated_file.as_path()),
            // --depfile does not work in combination with expanding yet, so we blacklist expanding tests.
            !(name.contains("expand") || name.contains("bitfield")),
            generate_symfile,
        )
    };

    let CBindgenOutput {
        bindings_content,
        depfile_content,
        symfile_content,
    } = run_cbindgen(
        path,
        output_file,
        language,
        cpp_compat,
        style,
        generate_depfile,
        package_version,
        generate_symfile,
    );
    if generate_depfile {
        let depfile = depfile_content.expect("No depfile generated");
        assert!(!depfile.is_empty());
        let mut rules = depfile.split(':');
        let target = rules.next().expect("No target found");
        assert_eq!(target, generated_file.as_os_str().to_str().unwrap());
        let sources = rules.next().unwrap();
        // All the tests here only have one sourcefile.
        assert!(
            sources.contains(path.to_str().unwrap()),
            "Path: {:?}, Depfile contents: {}",
            path,
            depfile
        );
        assert_eq!(rules.count(), 0, "More than 1 rule in the depfile");
    }

    if cbindgen_outputs.contains(&bindings_content) {
        // We already generated an identical file previously.
        if verify {
            assert!(!generated_file.exists());
        } else if generated_file.exists() {
            fs::remove_file(&generated_file).unwrap();
        }
    } else {
        if verify {
            // Compare cbindgen output to expected (existing on disk) output.
            let prev_cbindgen_bindings = fs::read(&generated_file).unwrap();
            assert_eq!(bindings_content, prev_cbindgen_bindings);
        } else {
            fs::write(&generated_file, &bindings_content)
                .expect("Failed to write generated bindings.");
            if generate_symfile {
                let symbols = symfile_content.expect("No symfile generated");
                fs::write(&generated_symfile, &symbols)
                    .expect("Failed to write generated symbols.");
            }
        }

        cbindgen_outputs.insert(bindings_content);

        if no_compile {
            return;
        }

        compile(
            &generated_file,
            &tests_path,
            tmp_dir,
            language,
            style,
            skip_warning_as_error,
        );

        if language == Language::C && cpp_compat {
            compile(
                &generated_file,
                &tests_path,
                tmp_dir,
                Language::Cxx,
                style,
                skip_warning_as_error,
            );
        }
    }
}

fn test_file(name: &'static str, filename: &'static str) {
    let test = Path::new(filename);
    let tmp_dir = tempfile::Builder::new()
        .prefix("cbindgen-test-output")
        .tempdir()
        .expect("Creating tmp dir failed");
    let tmp_dir = tmp_dir.path();
    // Run tests in deduplication priority order. C++ compatibility tests are run first,
    // otherwise we would lose the C++ compiler run if they were deduplicated.
    let mut cbindgen_outputs = HashSet::new();
    for cpp_compat in &[true, false] {
        for style in &[Style::Type, Style::Tag, Style::Both] {
            // We only need to generate the symfile once,
            // it should not change with the different options.
            let generate_symfile = !cpp_compat && *style == Style::Type;
            run_compile_test(
                name,
                test,
                tmp_dir,
                Language::C,
                *cpp_compat,
                Some(*style),
                &mut cbindgen_outputs,
                false,
                generate_symfile,
            );
        }
    }

    run_compile_test(
        name,
        test,
        tmp_dir,
        Language::Cxx,
        /* cpp_compat = */ false,
        None,
        &mut HashSet::new(),
        false,
        /* generate_symfile = */ false,
    );

    // `Style::Both` should be identical to `Style::Tag` for Cython.
    let mut cbindgen_outputs = HashSet::new();
    for style in &[Style::Type, Style::Tag] {
        run_compile_test(
            name,
            test,
            tmp_dir,
            Language::Cython,
            /* cpp_compat = */ false,
            Some(*style),
            &mut cbindgen_outputs,
            false,
            /* generate_symfile = */ false,
        );
    }
}

macro_rules! test_file {
    ($test_function_name:ident, $name:expr, $file:tt) => {
        #[test]
        fn $test_function_name() {
            test_file($name, $file);
        }
    };
}

// This file is generated by build.rs
include!(concat!(env!("OUT_DIR"), "/tests.rs"));
