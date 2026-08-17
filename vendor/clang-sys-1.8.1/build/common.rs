// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use glob::{MatchOptions, Pattern};

//================================================
// Commands
//================================================

thread_local! {
    /// The errors encountered by the build script while executing commands.
    static COMMAND_ERRORS: RefCell<HashMap<String, Vec<String>>> = RefCell::default();
}

/// Adds an error encountered by the build script while executing a command.
fn add_command_error(name: &str, path: &str, arguments: &[&str], message: String) {
    COMMAND_ERRORS.with(|e| {
        e.borrow_mut()
            .entry(name.into())
            .or_default()
            .push(format!(
                "couldn't execute `{} {}` (path={}) ({})",
                name,
                arguments.join(" "),
                path,
                message,
            ))
    });
}

/// A struct that prints the errors encountered by the build script while
/// executing commands when dropped (unless explictly discarded).
///
/// This is handy because we only want to print these errors when the build
/// script fails to link to an instance of `libclang`. For example, if
/// `llvm-config` couldn't be executed but an instance of `libclang` was found
/// anyway we don't want to pollute the build output with irrelevant errors.
#[derive(Default)]
pub struct CommandErrorPrinter {
    discard: bool,
}

impl CommandErrorPrinter {
    pub fn discard(mut self) {
        self.discard = true;
    }
}

impl Drop for CommandErrorPrinter {
    fn drop(&mut self) {
        if self.discard {
            return;
        }

        let errors = COMMAND_ERRORS.with(|e| e.borrow().clone());

        if let Some(errors) = errors.get("llvm-config") {
            println!(
                "cargo:warning=could not execute `llvm-config` one or more \
                times, if the LLVM_CONFIG_PATH environment variable is set to \
                a full path to valid `llvm-config` executable it will be used \
                to try to find an instance of `libclang` on your system: {}",
                errors
                    .iter()
                    .map(|e| format!("\"{}\"", e))
                    .collect::<Vec<_>>()
                    .join("\n  "),
            )
        }

        if let Some(errors) = errors.get("xcode-select") {
            println!(
                "cargo:warning=could not execute `xcode-select` one or more \
                times, if a valid instance of this executable is on your PATH \
                it will be used to try to find an instance of `libclang` on \
                your system: {}",
                errors
                    .iter()
                    .map(|e| format!("\"{}\"", e))
                    .collect::<Vec<_>>()
                    .join("\n  "),
            )
        }
    }
}

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref RUN_COMMAND_MOCK: std::sync::Mutex<
        Option<Box<dyn Fn(&str, &str, &[&str]) -> Option<String> + Send + Sync + 'static>>,
    > = std::sync::Mutex::new(None);
}

/// Executes a command and returns the `stdout` output if the command was
/// successfully executed (errors are added to `COMMAND_ERRORS`).
fn run_command(name: &str, path: &str, arguments: &[&str]) -> Option<String> {
    #[cfg(test)]
    if let Some(command) = &*RUN_COMMAND_MOCK.lock().unwrap() {
        return command(name, path, arguments);
    }

    let output = match Command::new(path).args(arguments).output() {
        Ok(output) => output,
        Err(error) => {
            let message = format!("error: {}", error);
            add_command_error(name, path, arguments, message);
            return None;
        }
    };

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let message = format!("exit code: {}", output.status);
        add_command_error(name, path, arguments, message);
        None
    }
}

/// Executes the `llvm-config` command and returns the `stdout` output if the
/// command was successfully executed (errors are added to `COMMAND_ERRORS`).
pub fn run_llvm_config(arguments: &[&str]) -> Option<String> {
    let path = env::var("LLVM_CONFIG_PATH").unwrap_or_else(|_| "llvm-config".into());
    run_command("llvm-config", &path, arguments)
}

/// Executes the `xcode-select` command and returns the `stdout` output if the
/// command was successfully executed (errors are added to `COMMAND_ERRORS`).
pub fn run_xcode_select(arguments: &[&str]) -> Option<String> {
    run_command("xcode-select", "xcode-select", arguments)
}

//================================================
// Search Directories
//================================================
// These search directories are listed in order of
// preference, so if multiple `libclang` instances
// are found when searching matching directories,
// the `libclang` instances from earlier
// directories will be preferred (though version
// takes precedence over location).
//================================================

/// `libclang` directory patterns for Haiku.
const DIRECTORIES_HAIKU: &[&str] = &[
    "/boot/home/config/non-packaged/develop/lib",
    "/boot/home/config/non-packaged/lib",
    "/boot/system/non-packaged/develop/lib",
    "/boot/system/non-packaged/lib",
    "/boot/system/develop/lib",
    "/boot/system/lib",
];

/// `libclang` directory patterns for Linux (and FreeBSD).
const DIRECTORIES_LINUX: &[&str] = &[
    "/usr/local/llvm*/lib*",
    "/usr/local/lib*/*/*",
    "/usr/local/lib*/*",
    "/usr/local/lib*",
    "/usr/lib*/*/*",
    "/usr/lib*/*",
    "/usr/lib*",
];

/// `libclang` directory patterns for macOS.
const DIRECTORIES_MACOS: &[&str] = &[
    "/usr/local/opt/llvm*/lib/llvm*/lib",
    "/Library/Developer/CommandLineTools/usr/lib",
    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
    "/usr/local/opt/llvm*/lib",
];

/// `libclang` directory patterns for Windows.
///
/// The boolean indicates whether the directory pattern should be used when
/// compiling for an MSVC target environment.
const DIRECTORIES_WINDOWS: &[(&str, bool)] = &[
    // LLVM + Clang can be installed using Scoop (https://scoop.sh).
    // Other Windows package managers install LLVM + Clang to other listed
    // system-wide directories.
    ("C:\\Users\\*\\scoop\\apps\\llvm\\current\\lib", true),
    ("C:\\MSYS*\\MinGW*\\lib", false),
    ("C:\\Program Files*\\LLVM\\lib", true),
    ("C:\\LLVM\\lib", true),
    // LLVM + Clang can be installed as a component of Visual Studio.
    // https://github.com/KyleMayes/clang-sys/issues/121
    ("C:\\Program Files*\\Microsoft Visual Studio\\*\\VC\\Tools\\Llvm\\**\\lib", true),
];

/// `libclang` directory patterns for illumos
const DIRECTORIES_ILLUMOS: &[&str] = &[
    "/opt/ooce/llvm-*/lib",
    "/opt/ooce/clang-*/lib",
];

//================================================
// Searching
//================================================

/// Finds the files in a directory that match one or more filename glob patterns
/// and returns the paths to and filenames of those files.
fn search_directory(directory: &Path, filenames: &[String]) -> Vec<(PathBuf, String)> {
    // Escape the specified directory in case it contains characters that have
    // special meaning in glob patterns (e.g., `[` or `]`).
    let directory = Pattern::escape(directory.to_str().unwrap());
    let directory = Path::new(&directory);

    // Join the escaped directory to the filename glob patterns to obtain
    // complete glob patterns for the files being searched for.
    let paths = filenames
        .iter()
        .map(|f| directory.join(f).to_str().unwrap().to_owned());

    // Prevent wildcards from matching path separators to ensure that the search
    // is limited to the specified directory.
    let mut options = MatchOptions::new();
    options.require_literal_separator = true;

    paths
        .map(|p| glob::glob_with(&p, options))
        .filter_map(Result::ok)
        .flatten()
        .filter_map(|p| {
            let path = p.ok()?;
            let filename = path.file_name()?.to_str().unwrap();

            // The `libclang_shared` library has been renamed to `libclang-cpp`
            // in Clang 10. This can cause instances of this library (e.g.,
            // `libclang-cpp.so.10`) to be matched by patterns looking for
            // instances of `libclang`.
            if filename.contains("-cpp.") {
                return None;
            }

            Some((path.parent().unwrap().to_owned(), filename.into()))
        })
        .collect::<Vec<_>>()
}

/// Finds the files in a directory (and any relevant sibling directories) that
/// match one or more filename glob patterns and returns the paths to and
/// filenames of those files.
fn search_directories(directory: &Path, filenames: &[String]) -> Vec<(PathBuf, String)> {
    let mut results = search_directory(directory, filenames);

    // On Windows, `libclang.dll` is usually found in the LLVM `bin` directory
    // while `libclang.lib` is usually found in the LLVM `lib` directory. To
    // keep things consistent with other platforms, only LLVM `lib` directories
    // are included in the backup search directory globs so we need to search
    // the LLVM `bin` directory here.
    if target_os!("windows") && directory.ends_with("lib") {
        let sibling = directory.parent().unwrap().join("bin");
        results.extend(search_directory(&sibling, filenames));
    }

    results
}

/// Finds the `libclang` static or dynamic libraries matching one or more
/// filename glob patterns and returns the paths to and filenames of those files.
pub fn search_libclang_directories(filenames: &[String], variable: &str) -> Vec<(PathBuf, String)> {
    // Search only the path indicated by the relevant environment variable
    // (e.g., `LIBCLANG_PATH`) if it is set.
    if let Ok(path) = env::var(variable).map(|d| Path::new(&d).to_path_buf()) {
        // Check if the path is a matching file.
        if let Some(parent) = path.parent() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let libraries = search_directories(parent, filenames);
            if libraries.iter().any(|(_, f)| f == filename) {
                return vec![(parent.into(), filename.into())];
            }
        }

        // Check if the path is directory containing a matching file.
        return search_directories(&path, filenames);
    }

    let mut found = vec![];

    // Search the `bin` and `lib` directories in the directory returned by
    // `llvm-config --prefix`.
    if let Some(output) = run_llvm_config(&["--prefix"]) {
        let directory = Path::new(output.lines().next().unwrap()).to_path_buf();
        found.extend(search_directories(&directory.join("bin"), filenames));
        found.extend(search_directories(&directory.join("lib"), filenames));
        found.extend(search_directories(&directory.join("lib64"), filenames));
    }

    // Search the toolchain directory in the directory returned by
    // `xcode-select --print-path`.
    if target_os!("macos") {
        if let Some(output) = run_xcode_select(&["--print-path"]) {
            let directory = Path::new(output.lines().next().unwrap()).to_path_buf();
            let directory = directory.join("Toolchains/XcodeDefault.xctoolchain/usr/lib");
            found.extend(search_directories(&directory, filenames));
        }
    }

    // Search the directories in the `LD_LIBRARY_PATH` environment variable.
    if let Ok(path) = env::var("LD_LIBRARY_PATH") {
        for directory in env::split_paths(&path) {
            found.extend(search_directories(&directory, filenames));
        }
    }

    // Determine the `libclang` directory patterns.
    let directories: Vec<&str> = if target_os!("haiku") {
        DIRECTORIES_HAIKU.into()
    } else if target_os!("linux") || target_os!("freebsd") {
        DIRECTORIES_LINUX.into()
    } else if target_os!("macos") {
        DIRECTORIES_MACOS.into()
    } else if target_os!("windows") {
        let msvc = target_env!("msvc");
        DIRECTORIES_WINDOWS
            .iter()
            .filter(|d| d.1 || !msvc)
            .map(|d| d.0)
            .collect()
    } else if target_os!("illumos") {
        DIRECTORIES_ILLUMOS.into()
    } else {
        vec![]
    };

    // We use temporary directories when testing the build script so we'll
    // remove the prefixes that make the directories absolute.
    let directories = if test!() {
        directories
            .iter()
            .map(|d| d.strip_prefix('/').or_else(|| d.strip_prefix("C:\\")).unwrap_or(d))
            .collect::<Vec<_>>()
    } else {
        directories
    };

    // Search the directories provided by the `libclang` directory patterns.
    let mut options = MatchOptions::new();
    options.case_sensitive = false;
    options.require_literal_separator = true;
    for directory in directories.iter() {
        if let Ok(directories) = glob::glob_with(directory, options) {
            for directory in directories.filter_map(Result::ok).filter(|p| p.is_dir()) {
                found.extend(search_directories(&directory, filenames));
            }
        }
    }

    found
}
