//! Which executables are a runtime's, and what each of its processes is,
//! from its command line.

use std::path::Path;

/// `node`, and `nodejs` as some Linux distributions name it. Not anything
/// that merely contains "node".
pub(crate) fn is_node(stem: &str) -> bool {
    matches!(stem, "node" | "nodejs")
}

/// `python`, `python3`, `python3.12`, `pythonw` (Windows' windowless one),
/// a free-threaded `python3.13t`, and the Windows launchers `py` and `pyw`.
pub(crate) fn is_python(stem: &str) -> bool {
    if matches!(stem, "py" | "pyw") {
        return true;
    }
    let Some(rest) = stem.strip_prefix("python") else {
        return false;
    };
    let rest = rest.strip_prefix('w').unwrap_or(rest);
    let rest = rest.strip_suffix('t').unwrap_or(rest);
    rest.chars().all(|c| c.is_ascii_digit() || c == '.')
}

/// File names that say nothing about what a script is, so the package's own
/// name is used instead.
const GENERIC_SCRIPT_NAMES: &[&str] = &["index", "cli", "main", "bin", "start", "run", "npm-cli"];

/// What a Node process is, from its command line: the tool and up to two of
/// its subcommand words (`next dev`, `npm run dev`, `gulp build`), or the
/// script's file name for a project's own script (`server.js`). `name` (the
/// executable's) when there is no script at all, as for a bare `node` REPL.
pub(crate) fn node_label(command: &[String], name: &str) -> String {
    let mut args = command.iter().skip(1).peekable();
    // Node's own options come before the script.
    while args.next_if(|arg| arg.starts_with('-')).is_some() {}
    let Some(script) = args.next() else {
        return executable(name);
    };
    let words = words(args);

    let script = script.replace('\\', "/");
    let file = Path::new(&script);
    let stem = file
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();

    let tool = match package_of(&script) {
        // A package's script that names itself (`tsserver.js` in
        // `typescript`) says more than the package does.
        Some(package) if GENERIC_SCRIPT_NAMES.contains(&stem.as_str()) || stem == package => {
            package
        }
        Some(_) => stem,
        // A script of the project's own: its file name says what it is.
        None => file
            .file_name()
            .map(|file| file.to_string_lossy().into_owned())
            .unwrap_or_else(|| script.clone()),
    };
    join(&tool, words)
}

/// Python's own options that take a value as the next argument.
const PYTHON_OPTIONS_WITH_VALUE: &[&str] = &["-W", "-X", "--check-hash-based-pycs"];

/// What a Python process is, from its command line: the module and up to two
/// of its words for `-m` (`flask run`, `uvicorn app:app`), the script's file
/// name for a script (`manage.py runserver`), or a command that `pip`
/// installed (`.venv/Scripts/uvicorn.exe` runs as Python with that file as
/// its script) by its own name. `python -c` for inline code, and the
/// executable's name with no script at all.
pub(crate) fn python_label(command: &[String], name: &str) -> String {
    let mut args = command.iter().skip(1).peekable();
    while let Some(arg) = args.next_if(|arg| arg.starts_with('-')) {
        match arg.as_str() {
            "-m" => {
                let Some(module) = args.next() else {
                    return executable(name);
                };
                return join(module, words(args));
            }
            "-c" => return format!("{} -c", executable(name)),
            option if PYTHON_OPTIONS_WITH_VALUE.contains(&option) => {
                args.next();
            }
            _ => {}
        }
    }
    let Some(script) = args.next() else {
        return executable(name);
    };
    let words = words(args);

    let script = script.replace('\\', "/");
    let file = Path::new(&script);
    let in_commands_folder = file
        .parent()
        .and_then(Path::file_name)
        .is_some_and(|folder| folder == "Scripts" || folder == "bin");
    let part = if in_commands_folder {
        file.file_stem()
    } else {
        file.file_name()
    };
    let tool = part
        .map(|part| part.to_string_lossy().into_owned())
        .unwrap_or_else(|| script.clone());
    join(&tool, words)
}

/// Up to two subcommand words, stopping at the first option.
fn words<'a>(args: impl Iterator<Item = &'a String>) -> Vec<&'a str> {
    args.take_while(|arg| !arg.starts_with('-'))
        .take(2)
        .map(String::as_str)
        .collect()
}

fn join(tool: &str, words: Vec<&str>) -> String {
    let mut label = std::iter::once(tool)
        .chain(words)
        .collect::<Vec<_>>()
        .join(" ");
    const MAX: usize = 40;
    if label.chars().count() > MAX {
        label = label.chars().take(MAX - 1).collect::<String>() + "\u{2026}";
    }
    label
}

/// The executable's name without its extension: `node`, `python3`.
fn executable(name: &str) -> String {
    Path::new(name)
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_else(|| name.to_string())
}

/// The package a script belongs to: the folder after the last
/// `node_modules/`, with its scope (`@scope/name`) when it has one.
fn package_of(script: &str) -> Option<String> {
    let (_, rest) = script.rsplit_once("node_modules/")?;
    let mut parts = rest.split('/');
    let first = parts.next()?;
    if first.starts_with('@') {
        Some(format!("{first}/{}", parts.next()?))
    } else {
        Some(first.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| part.to_string()).collect()
    }

    #[test]
    fn only_the_runtimes_own_executables_count() {
        for stem in ["node", "nodejs"] {
            assert!(is_node(stem), "{stem}");
        }
        for stem in ["nodemanager", "node-red", "nodejs-helper", "pnode"] {
            assert!(!is_node(stem), "{stem}");
        }
        for stem in [
            "python",
            "python3",
            "python3.12",
            "pythonw",
            "python3.13t",
            "py",
            "pyw",
        ] {
            assert!(is_python(stem), "{stem}");
        }
        for stem in ["pythonservice", "ipython", "python-lsp", "pyright"] {
            assert!(!is_python(stem), "{stem}");
        }
    }

    #[test]
    fn node_labels_say_what_each_process_is() {
        let cases: &[(&[&str], &str)] = &[
            (
                &[
                    r"C:\nodejs\node.exe",
                    r"H:\app\node_modules\next\dist\bin\next",
                    "dev",
                ],
                "next dev",
            ),
            (
                &[
                    "node",
                    r"C:\nodejs\node_modules\npm\bin\npm-cli.js",
                    "run",
                    "dev",
                ],
                "npm run dev",
            ),
            (
                &[
                    "node",
                    "--max-old-space-size=3072",
                    "/opt/ide/node_modules/typescript/lib/tsserver.js",
                    "--serverMode",
                    "partialSemantic",
                ],
                "tsserver",
            ),
            (&["node", "/app/node_modules/vite/bin/vite.js"], "vite"),
            (
                &[
                    "node",
                    "/app/node_modules/gulp/bin/gulp.js",
                    "build",
                    "--ship",
                ],
                "gulp build",
            ),
            (
                &["node", "/app/node_modules/@scope/tool/index.js", "watch"],
                "@scope/tool watch",
            ),
            (&["node", "src/server.js", "--port", "3000"], "server.js"),
            (&[r"C:\nodejs\node.exe"], "node"),
        ];
        for (parts, expected) in cases {
            assert_eq!(
                node_label(&command(parts), "node.exe"),
                *expected,
                "{parts:?}"
            );
        }
    }

    #[test]
    fn python_labels_say_what_each_process_is() {
        let cases: &[(&[&str], &str)] = &[
            (&["python", "-m", "flask", "run", "--debug"], "flask run"),
            (
                &["python3", "-u", "-m", "uvicorn", "app:app"],
                "uvicorn app:app",
            ),
            (
                &["python", "manage.py", "runserver", "8000"],
                "manage.py runserver 8000",
            ),
            (
                &[
                    r"H:\app\.venv\Scripts\python.exe",
                    r"H:\app\.venv\Scripts\uvicorn.exe",
                    "main:app",
                ],
                "uvicorn main:app",
            ),
            (
                &["/app/.venv/bin/python", "/app/.venv/bin/celery", "worker"],
                "celery worker",
            ),
            (&["python", "-X", "dev", "src/train.py"], "train.py"),
            (&["python", "-c", "print(1)"], "python -c"),
            (&[r"C:\Python312\python.exe"], "python"),
        ];
        for (parts, expected) in cases {
            assert_eq!(
                python_label(&command(parts), "python.exe"),
                *expected,
                "{parts:?}"
            );
        }
    }

    #[test]
    fn a_long_label_is_shortened() {
        let long = "x".repeat(60);
        let label = node_label(&command(&["node", "/app/server.js", &long]), "node");
        assert_eq!(label.chars().count(), 40);
        assert!(label.ends_with('\u{2026}'));
    }

    #[test]
    fn package_names_keep_their_scope() {
        assert_eq!(
            package_of("/a/node_modules/next/dist/bin/next").as_deref(),
            Some("next")
        );
        assert_eq!(
            package_of("/a/node_modules/x/node_modules/@b/c/i.js").as_deref(),
            Some("@b/c")
        );
        assert_eq!(package_of("/a/src/server.js"), None);
    }
}
