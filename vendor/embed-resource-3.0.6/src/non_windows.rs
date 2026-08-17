use self::super::{ParameterBundle, apply_parameters};
use std::process::{Command, Stdio};
use std::path::{PathBuf, Path};
use std::{env, fs, mem};
use std::borrow::Cow;
use std::ffi::OsStr;
use memchr::memmem;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceCompiler {
    compiler: Result<Compiler, Cow<'static, str>>,
}

impl ResourceCompiler {
    pub fn new() -> ResourceCompiler {
        ResourceCompiler { compiler: Compiler::probe() }
    }

    #[inline]
    pub fn is_supported(&mut self) -> Option<Cow<'static, str>> {
        match mem::replace(&mut self.compiler, Err("".into())) {
            Ok(c) => {
                self.compiler = Ok(c);
                None
            }
            Err(e) => Some(e),
        }
    }

    pub fn compile_resource<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>>(
        &self, out_dir: &str, prefix: &str, resource: &str, parameters: ParameterBundle<Ms, Mi, Is, Ii>)
        -> Result<String, Cow<'static, str>> {
        self.compiler.as_ref().expect("Not supported but we got to compile_resource()?").compile(out_dir, prefix, resource, parameters)
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CompilerType {
    /// LLVM-RC
    ///
    /// Requires a separate C preprocessor step on the source RC file
    LlvmRc { has_no_preprocess: bool, },
    /// MinGW windres
    WindRes,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Compiler {
    tp: CompilerType,
    executable: Cow<'static, str>,
}

impl Compiler {
    pub fn probe() -> Result<Compiler, Cow<'static, str>> {
        let target = env::var("TARGET").map_err(|_| Cow::from("no $TARGET"))?;

        if let Ok(rc) = env::var(&format!("RC_{}", target))
            .or_else(|_| env::var(&format!("RC_{}", target.replace('-', "_"))))
            .or_else(|_| env::var("RC")) {
            return guess_compiler_variant(&rc);
        }

        if target.ends_with("-windows-gnu") || target.ends_with("-windows-gnullvm") {
            let executable = format!("{}-w64-mingw32-windres", &target[0..target.find('-').unwrap_or_default()]);
            if is_runnable(&executable) {
                return Ok(Compiler {
                    tp: CompilerType::WindRes,
                    executable: executable.into(),
                });
            } else {
                return Err(executable.into());
            }
        } else if target.ends_with("-windows-msvc") {
            if is_runnable("llvm-rc") {
                return Ok(Compiler {
                    tp: CompilerType::LlvmRc {
                        has_no_preprocess: Command::new("llvm-rc")
                            .arg("/?")
                            .output()
                            .ok()
                            .map(|out| memmem::find(&out.stdout, b"no-preprocess").is_some())
                            .unwrap_or(false),
                    },
                    executable: "llvm-rc".into(),
                });
            } else {
                return Err("llvm-rc".into());
            }
        }

        Err("".into())
    }

    pub fn compile<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>>(&self, out_dir: &str, prefix: &str,
                                                                                                                 resource: &str,
                                                                                                                 parameters: ParameterBundle<Ms, Mi, Is, Ii>)
                                                                                                                 -> Result<String, Cow<'static, str>> {
        let out_file = format!("{}/{}.lib", out_dir, prefix);
        match self.tp {
            CompilerType::LlvmRc { has_no_preprocess } => {
                let preprocessed_path = format!("{}/{}-preprocessed.rc", out_dir, prefix);
                fs::write(&preprocessed_path,
                          cc_xc(apply_parameters_cc(cc::Build::new().define("RC_INVOKED", None), parameters))
                              .file(resource)
                              .cargo_metadata(false)
                              .include(out_dir)
                              .expand()).map_err(|e| e.to_string())?;

                try_command(Command::new(&self.executable[..])
                                .args(&["/fo", &out_file])
                                .args(&["/C", "65001"]) // UTF-8, cf. https://github.com/nabijaczleweli/rust-embed-resource/pull/73
                                .args(if has_no_preprocess {
                                    // We already preprocessed using CC. llvm-rc preprocessing
                                    // requires having clang in PATH, which more exotic toolchains
                                    // may not necessarily have.
                                    &["/no-preprocess"][..]
                                } else {
                                    &[][..]
                                })
                                .args(&["--", &preprocessed_path])
                                .stdin(Stdio::piped())
                                .current_dir(or_curdir(Path::new(resource).parent().expect("Resource parent nonexistent?"))),
                            Path::new(&self.executable[..]),
                            "compile",
                            &preprocessed_path,
                            &out_file)?;
            }
            CompilerType::WindRes => {
                try_command(apply_parameters(Command::new(&self.executable[..])
                                                 .args(&["--input", resource, "--output-format=coff", "--output", &out_file, "--include-dir", out_dir]),
                                             "-D",
                                             "--include-dir",
                                             parameters),
                            Path::new(&self.executable[..]),
                            "compile",
                            resource,
                            &out_file)?;
            }
        }
        Ok(out_file)
    }
}

fn apply_parameters_cc<'t, Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>>(to: &'t mut cc::Build,
                                                                                                                         parameters: ParameterBundle<Ms,
                                                                                                                                                     Mi,
                                                                                                                                                     Is,
                                                                                                                                                     Ii>)
                                                                                                                         -> &'t mut cc::Build {
    for m in parameters.macros {
        let mut m = m.as_ref().to_str().expect("macros must be UTF-8 in this configuration").splitn(2, '=');
        to.define(m.next().unwrap(), m.next());
    }
    for id in parameters.include_dirs {
        to.include(id.as_ref());
    }
    to
}

fn cc_xc(to: &mut cc::Build) -> &mut cc::Build {
    if to.get_compiler().is_like_msvc() {
        // clang-cl
        to.flag("-Xclang");
    }
    to.flag("-xc");
    to
}

fn try_command(cmd: &mut Command, exec: &Path, action: &str, whom: &str, whre: &str) -> Result<(), Cow<'static, str>> {
    match cmd.status() {
        Ok(stat) if stat.success() => Ok(()),
        Ok(stat) => Err(format!("{} failed to {} \"{}\" into \"{}\" with {}", exec.display(), action, whom, whre, stat).into()),
        Err(e) => Err(format!("Couldn't execute {} to {} \"{}\" into \"{}\": {}", exec.display(), action, whom, whre, e).into()),
    }
}

fn or_curdir(directory: &Path) -> &Path {
    if directory == Path::new("") {
        Path::new(".")
    } else {
        directory
    }
}

/// -V will print the version in windres.
/// /? will print the help in LLVM-RC and Microsoft RC.EXE.
/// If combined, /? takes precedence over -V.
fn guess_compiler_variant(s: &str) -> Result<Compiler, Cow<'static, str>> {
    match Command::new(s).args(&["-V", "/?"]).output() {
        Ok(out) => {
            if out.stdout.starts_with(b"GNU windres") {
                Ok(Compiler {
                    executable: s.to_string().into(),
                    tp: CompilerType::WindRes,
                })
            } else if out.stdout.starts_with(b"OVERVIEW: Resource Converter") || out.stdout.starts_with(b"OVERVIEW: LLVM Resource Converter") {
                Ok(Compiler {
                    executable: s.to_string().into(),
                    tp: CompilerType::LlvmRc { has_no_preprocess: memmem::find(&out.stdout, b"no-preprocess").is_some() },
                })
            } else {
                Err(format!("Unknown RC compiler variant: {}", s).into())
            }
        }
        Err(err) => Err(format!("Couldn't execute {}: {}", s, err).into()),
    }
}


fn is_runnable(s: &str) -> bool {
    Command::new(s).spawn().map(|mut c| c.kill()).is_ok()
}

pub fn find_windows_sdk_tool_impl(_: &str) -> Option<PathBuf> {
    None
}
