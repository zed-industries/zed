#![cfg(unix)]

use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::{AsyncReadExt as _, AsyncWriteExt as _};
use remote::command::{
    RemoteCommand, home_relative_path, home_stdio_launcher_command, stdio_launcher_command,
};
use smol::process::{Command, Stdio};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::fs::PermissionsExt as _;
use util::shell::{Shell, ShellKind};
use util::shell_builder::ShellBuilder;

#[test]
fn test_remote_command_preserves_agent_stdin() -> Result<()> {
    smol::block_on(async {
        let descriptor = RemoteCommand {
            program: String::from("/bin/cat"),
            args: Vec::new(),
            env: HashMap::default(),
            working_dir: None,
        };
        let mut child = launcher().spawn()?;
        let mut stdin = child.stdin.take().context("missing stdin")?;
        let mut stdout = child.stdout.take().context("missing stdout")?;
        let message = b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\"}\n\0\xff";
        let mut input = descriptor.encode()?;
        input.extend_from_slice(message);
        stdin.write_all(&input).await?;
        stdin.flush().await?;
        let mut output = vec![0; message.len()];
        stdout.read_exact(&mut output).await?;
        assert_eq!(output, message);
        drop(stdin);
        let output = child.output().await?;
        assert!(output.status.success());
        assert_eq!(output.stderr, b"");
        Ok(())
    })
}

#[test]
fn test_remote_command_preserves_environment() -> Result<()> {
    smol::block_on(async {
        let directory = tempfile::tempdir()?;
        let marker = directory.path().join("injected");
        let environment = HashMap::from_iter([
            (
                String::from("TOKEN"),
                String::from("'\"\n\r\t\\$()=secret\n"),
            ),
            (String::from("EMPTY"), String::new()),
            (String::from("UNICODE"), String::from("钥匙🔑")),
            (String::from("LARGE"), "s".repeat(32 * 1024)),
            (
                format!("KEY$(touch {})", marker.display()),
                String::from("literal"),
            ),
            (String::from("KEY-WITH-DASH"), String::from("preserved")),
        ]);
        for (key, value) in &environment {
            let descriptor = RemoteCommand {
                program: String::from("/usr/bin/printenv"),
                args: vec![key.clone()],
                env: environment.clone(),
                working_dir: None,
            };
            let mut child = launcher().spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert!(output.status.success(), "{key}: {:?}", output.stderr);
            assert_eq!(output.stdout, format!("{value}\n").as_bytes());
            assert_eq!(output.stderr, b"");
        }
        assert!(!marker.exists());
        Ok(())
    })
}

#[test]
fn test_remote_command_working_directory_and_exit_status() -> Result<()> {
    smol::block_on(async {
        let home = tempfile::tempdir()?;
        let project = home.path().join("project 'with spaces'");
        std::fs::create_dir(&project)?;
        for (env, expected_pwd) in [
            (HashMap::default(), project.display().to_string()),
            (
                HashMap::from_iter([(String::from("PWD"), String::from("explicit"))]),
                String::from("explicit"),
            ),
        ] {
            let descriptor = RemoteCommand {
                program: String::from("/usr/bin/printenv"),
                args: vec![String::from("PWD")],
                env,
                working_dir: Some(String::from("~/project 'with spaces'")),
            };
            let mut child = launcher().env("HOME", home.path()).spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert!(output.status.success(), "{:?}", output.stderr);
            assert_eq!(output.stdout, format!("{expected_pwd}\n").as_bytes());
        }
        let descriptor = RemoteCommand {
            program: String::from("/bin/sh"),
            args: vec![String::from("-c"), String::from("pwd -P; exit 37")],
            env: HashMap::from_iter([(
                String::from("HOME"),
                String::from("/not-the-bootstrap-home"),
            )]),
            working_dir: Some(String::from("~/project 'with spaces'")),
        };
        let mut child = launcher().env("HOME", home.path()).spawn()?;
        child
            .stdin
            .take()
            .context("missing stdin")?
            .write_all(&descriptor.encode()?)
            .await?;
        let output = child.output().await?;
        assert_eq!(output.status.code(), Some(37));
        assert_eq!(
            output.stdout,
            format!("{}\n", project.canonicalize()?.display()).as_bytes()
        );
        assert_eq!(output.stderr, b"");
        Ok(())
    })
}

#[test]
fn test_remote_command_inherits_shell_environment() -> Result<()> {
    smol::block_on(async {
        let home = tempfile::tempdir()?;
        let shell_only_binary = home.path().join("shell-only-printenv");
        std::os::unix::fs::symlink("/usr/bin/printenv", &shell_only_binary)?;
        let home_path = home.path().display().to_string();
        let quoted_home = ShellKind::Posix
            .try_quote(&home_path)
            .context("shell quoting")?;
        std::fs::write(
            home.path().join(".bashrc"),
            format!(
                "export PATH={quoted_home}:/usr/bin:/bin\nexport SHELL_SECRET=only-in-shell-startup\n"
            ),
        )?;
        for (program, env, expected) in [
            (
                "shell-only-printenv",
                HashMap::default(),
                "only-in-shell-startup",
            ),
            (
                "/usr/bin/printenv",
                HashMap::from_iter([(String::from("SHELL_SECRET"), String::from("from-payload"))]),
                "from-payload",
            ),
        ] {
            let descriptor = RemoteCommand {
                program: String::from(program),
                args: vec![String::from("SHELL_SECRET")],
                env,
                working_dir: None,
            };
            let exec =
                stdio_launcher_command(ShellKind::Posix, env!("CARGO_BIN_EXE_remote_server"))?;
            let mut child = shell_launcher(home.path(), &exec).spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert!(output.status.success(), "{program}: {:?}", output.stderr);
            assert_eq!(output.stdout, format!("{expected}\n").as_bytes());
        }
        Ok(())
    })
}

#[test]
fn test_remote_command_inherits_default_shell_startup_environment() -> Result<()> {
    smol::block_on(async {
        let home = tempfile::tempdir()?;
        let helper = install_helper(home.path())?;
        let startup_script = home.path().join("startup env");
        std::fs::write(
            &startup_script,
            "export STARTUP_TOKEN=only-in-default-shell-startup\n",
        )?;
        let descriptor = RemoteCommand {
            program: String::from("/usr/bin/printenv"),
            args: vec![String::from("STARTUP_TOKEN")],
            env: HashMap::default(),
            working_dir: None,
        };
        let (inner_shell, inner_args) =
            ShellBuilder::new(&Shell::Program(String::from("/bin/bash")), false).build(
                Some(stdio_launcher_command(ShellKind::Posix, &helper)?),
                &[],
            );
        let default_shell_command = std::iter::once(inner_shell)
            .chain(inner_args)
            .map(|argument| {
                ShellKind::Posix
                    .try_quote(&argument)
                    .map(Cow::into_owned)
                    .context("shell quoting")
            })
            .collect::<Result<Vec<_>>>()?
            .join(" ");
        let mut child = Command::new("/bin/bash")
            .args(["-c", &default_shell_command])
            .env_clear()
            .env("HOME", home.path())
            .env("PATH", "/usr/bin:/bin")
            .env("BASH_ENV", &startup_script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        child
            .stdin
            .take()
            .context("missing stdin")?
            .write_all(&descriptor.encode()?)
            .await?;
        let output = child.output().await?;
        assert!(output.status.success(), "{:?}", output.stderr);
        assert_eq!(output.stdout, b"only-in-default-shell-startup\n");
        Ok(())
    })
}

#[test]
fn test_remote_command_survives_shell_startup_changing_directory() -> Result<()> {
    smol::block_on(async {
        let home = tempfile::tempdir()?;
        let project = home.path().join("project");
        std::fs::create_dir(&project)?;
        let helper = install_helper(home.path())?;
        std::fs::write(
            home.path().join(".bashrc"),
            "cd /tmp\nexport HOME=/not-the-real-home\n",
        )?;
        let descriptor = RemoteCommand {
            program: String::from("/bin/pwd"),
            args: vec![String::from("-P")],
            env: HashMap::default(),
            working_dir: Some(project.display().to_string()),
        };
        let exec = stdio_launcher_command(ShellKind::Posix, &helper)?;
        let mut child = shell_launcher(home.path(), &exec)
            .current_dir(&project)
            .spawn()?;
        child
            .stdin
            .take()
            .context("missing stdin")?
            .write_all(&descriptor.encode()?)
            .await?;
        let output = child.output().await?;
        assert!(output.status.success(), "{:?}", output.stderr);
        assert_eq!(
            output.stdout,
            format!("{}\n", project.canonicalize()?.display()).as_bytes()
        );
        Ok(())
    })
}

#[test]
fn test_remote_command_runs_shell_directory_hooks() -> Result<()> {
    smol::block_on(async {
        let home = tempfile::tempdir()?;
        let project = home.path().join("project 'with spaces'");
        let application = project.join("application");
        std::fs::create_dir_all(&application)?;
        let helper = install_helper(home.path())?;
        let project_path = project.display().to_string();
        let quoted_project = ShellKind::Posix
            .try_quote(&project_path)
            .context("shell quoting")?;
        std::fs::write(
            home.path().join(".bashrc"),
            format!(
                "cd() {{ builtin cd \"$@\" || return; if [ \"$PWD\" = {quoted_project} ]; then export PROJECT_SECRET=project-only; builtin cd application; else unset PROJECT_SECRET; fi; }}\n"
            ),
        )?;
        for (working_dir, expected) in [
            (
                Some(project_path.clone()),
                format!("project-only\n{}\n", application.canonicalize()?.display()),
            ),
            (
                None,
                format!("\n{}\n", home.path().canonicalize()?.display()),
            ),
        ] {
            let descriptor = RemoteCommand {
                program: String::from("/bin/sh"),
                args: vec![
                    String::from("-c"),
                    String::from("printf '%s\\n%s\\n' \"$PROJECT_SECRET\" \"$(pwd -P)\""),
                ],
                env: HashMap::default(),
                working_dir: None,
            };
            let prefix = match &working_dir {
                Some(working_dir) => format!(
                    "cd {} && ",
                    ShellKind::Posix
                        .try_quote(working_dir)
                        .context("shell quoting")?
                ),
                None => String::from("cd && "),
            };
            let exec = format!(
                "{prefix}{}",
                stdio_launcher_command(ShellKind::Posix, &helper)?
            );
            let mut child = shell_launcher(home.path(), &exec)
                .current_dir(home.path())
                .spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert!(
                output.status.success(),
                "{working_dir:?}: {:?}",
                output.stderr
            );
            assert_eq!(output.stdout, expected.as_bytes(), "{working_dir:?}");
        }
        Ok(())
    })
}

#[test]
fn test_remote_command_runs_scripts_without_shebang() -> Result<()> {
    smol::block_on(async {
        let directory = tempfile::tempdir()?;
        let binary_directory = directory.path().join("bin");
        std::fs::create_dir(&binary_directory)?;
        let script = binary_directory.join("agent-wrapper");
        for script in [&script, &binary_directory.join("-a")] {
            std::fs::write(
                script,
                "printf '%s|%s|%s\\n' \"$WRAPPER_TOKEN\" \"$1\" \"$(pwd -P)\"\n",
            )?;
            std::fs::set_permissions(script, std::fs::Permissions::from_mode(0o755))?;
        }
        let stale_directory = directory.path().join("stale");
        std::fs::create_dir(&stale_directory)?;
        let stale_script = stale_directory.join("agent-wrapper");
        std::fs::write(
            &stale_script,
            "#!/nonexistent/interpreter\nprintf 'stale\\n'\n",
        )?;
        std::fs::set_permissions(&stale_script, std::fs::Permissions::from_mode(0o755))?;
        let expected = format!(
            "wrapper-secret|--acp|{}\n",
            directory.path().canonicalize()?.display()
        );
        for program in [
            script.display().to_string(),
            String::from("agent-wrapper"),
            String::from("-a"),
        ] {
            let descriptor = RemoteCommand {
                program,
                args: vec![String::from("--acp")],
                env: HashMap::from_iter([
                    (
                        String::from("WRAPPER_TOKEN"),
                        String::from("wrapper-secret"),
                    ),
                    (
                        String::from("PATH"),
                        format!(
                            "{}:{}:/usr/bin:/bin",
                            stale_directory.display(),
                            binary_directory.display()
                        ),
                    ),
                ]),
                working_dir: Some(directory.path().display().to_string()),
            };
            let mut child = launcher().env("PATH", "/usr/bin:/bin").spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert!(
                output.status.success(),
                "{}: {:?}",
                descriptor.program,
                output.stderr
            );
            assert_eq!(output.stdout, expected.as_bytes(), "{}", descriptor.program);
        }
        Ok(())
    })
}

#[test]
fn test_remote_command_matches_native_path_lookup() -> Result<()> {
    smol::block_on(async {
        let directory = tempfile::tempdir()?;
        let binary_directory = directory.path().join("bin");
        std::fs::create_dir(&binary_directory)?;
        let agent = binary_directory.join("agent");
        std::fs::write(&agent, "#!/bin/sh\nprintf 'found\\n'\n")?;
        std::fs::set_permissions(&agent, std::fs::Permissions::from_mode(0o755))?;
        let loop_directory = directory.path().join("loop");
        std::fs::create_dir(&loop_directory)?;
        std::os::unix::fs::symlink("agent", loop_directory.join("agent"))?;
        let binary_directory = binary_directory.as_os_str().as_bytes();
        let overlong_directory = [
            directory.path().as_os_str().as_bytes(),
            b"/",
            &[b'x'; 300][..],
        ]
        .concat();
        for (label, path, must_succeed) in [
            (
                "non-UTF-8 entry",
                [&b"/not-present-\xff:"[..], binary_directory].concat(),
                true,
            ),
            (
                "symlink loop",
                [
                    loop_directory.as_os_str().as_bytes(),
                    b":",
                    binary_directory,
                ]
                .concat(),
                false,
            ),
            (
                "overlong entry",
                [&overlong_directory[..], b":", binary_directory].concat(),
                false,
            ),
        ] {
            let path = OsStr::from_bytes(&path);
            let native = Command::new("/usr/bin/env")
                .arg("agent")
                .env_clear()
                .env("PATH", path)
                .output()
                .await?;
            let descriptor = RemoteCommand {
                program: String::from("agent"),
                args: Vec::new(),
                env: HashMap::default(),
                working_dir: None,
            };
            let mut child = launcher().env_clear().env("PATH", path).spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert_eq!(
                (output.status.success(), output.stdout.clone()),
                (native.status.success(), native.stdout),
                "{label}: {:?}",
                output.stderr
            );
            if must_succeed {
                assert_eq!(output.stdout, b"found\n", "{label}");
            }
        }
        Ok(())
    })
}

#[test]
fn test_remote_command_launcher_expands_home_in_shell() -> Result<()> {
    smol::block_on(async {
        let home = tempfile::tempdir()?;
        install_helper(home.path())?;
        std::fs::write(home.path().join(".bashrc"), "cd /tmp\n")?;
        let descriptor = RemoteCommand {
            program: String::from("/usr/bin/printenv"),
            args: vec![String::from("LINE\nBREAK")],
            env: HashMap::from_iter([(String::from("LINE\nBREAK"), String::from("kept"))]),
            working_dir: None,
        };
        let exec = format!(
            "cd && {}",
            home_stdio_launcher_command(ShellKind::Posix, None, ".local/share/zed/remote server")?
        );
        assert_eq!(
            exec,
            "cd && exec \"$HOME\"/'.local/share/zed/remote server' exec"
        );
        let mut child = shell_launcher(home.path(), &exec).spawn()?;
        child
            .stdin
            .take()
            .context("missing stdin")?
            .write_all(&descriptor.encode()?)
            .await?;
        let output = child.output().await?;
        assert!(output.status.success(), "{:?}", output.stderr);
        assert_eq!(output.stdout, b"kept\n");
        Ok(())
    })
}

#[test]
fn test_remote_command_launcher_quoting_in_tcsh() -> Result<()> {
    let Some(tcsh) = ["/bin/tcsh", "/usr/bin/tcsh"]
        .into_iter()
        .find(|tcsh| std::path::Path::new(tcsh).exists())
    else {
        return Ok(());
    };
    smol::block_on(async {
        let directory = tempfile::tempdir()?;
        let home = directory.path().join("bang!home");
        let project = home.join("O'Brien !1 project\\'; /usr/bin/printf INJECTED; true \\'");
        std::fs::create_dir_all(&project)?;
        let helper = install_helper(&home)?;
        let descriptor = RemoteCommand {
            program: String::from("/bin/sh"),
            args: vec![
                String::from("-c"),
                String::from("printf '%s\\n%s\\n' \"$AGENT_TOKEN\" \"$(pwd -P)\""),
            ],
            env: HashMap::from_iter([(String::from("AGENT_TOKEN"), String::from("secret!"))]),
            working_dir: None,
        };
        let expected = format!("secret!\n{}\n", project.canonicalize()?.display());
        let project_path = project.display().to_string();
        for setting in [
            "",
            "set backslash_quote\n",
            "set histchars = ''\n",
            "set backslash_quote\nset histchars = ''\n",
        ] {
            let exec = format!(
                "{setting}cd {} && {}",
                ShellKind::Tcsh
                    .try_quote(&project_path)
                    .context("shell quoting")?,
                stdio_launcher_command(ShellKind::Tcsh, &helper)?
            );
            let remote_shell = [String::from("-f"), String::from("-c"), exec.clone()];
            let direct = {
                let mut command = Command::new(tcsh);
                command.args(&remote_shell);
                command
            };
            let through_local_shell = Command::from(
                ShellBuilder::new(&Shell::Program(String::from(tcsh)), false)
                    .non_interactive()
                    .build_std_command(Some(String::from(tcsh)), &remote_shell),
            );
            for mut command in [direct, through_local_shell] {
                let mut child = command
                    .env_clear()
                    .env("HOME", &home)
                    .env("PATH", "/usr/bin:/bin")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                child
                    .stdin
                    .take()
                    .context("missing stdin")?
                    .write_all(&descriptor.encode()?)
                    .await?;
                let output = child.output().await?;
                assert!(output.status.success(), "{exec}: {:?}", output.stderr);
                assert_eq!(String::from_utf8_lossy(&output.stdout), expected, "{exec}");
                assert_eq!(output.stderr, b"");
            }
        }
        Ok(())
    })
}

#[test]
fn test_remote_command_launcher_expands_home_in_nushell() -> Result<()> {
    let Some(nu) = ["/opt/homebrew/bin/nu", "/usr/local/bin/nu", "/usr/bin/nu"]
        .into_iter()
        .find(|nu| std::path::Path::new(nu).exists())
    else {
        return Ok(());
    };
    smol::block_on(async {
        let directory = tempfile::tempdir()?;
        let home = directory.path().join("home a^b");
        let project = home.join("O'Brien project");
        std::fs::create_dir_all(&project)?;
        install_helper(&home)?;
        let descriptor = RemoteCommand {
            program: String::from("/bin/sh"),
            args: vec![
                String::from("-c"),
                String::from("printf '%s\\n%s\\n' \"$AGENT_TOKEN\" \"$(pwd -P)\""),
            ],
            env: HashMap::from_iter([(String::from("AGENT_TOKEN"), String::from("secret!"))]),
            working_dir: None,
        };
        let expected = format!("secret!\n{}\n", project.canonicalize()?.display());
        let project_path = project.display().to_string();
        for (working_dir, home_dir) in [
            (Some("~/O'Brien project"), None),
            (Some(project_path.as_str()), None),
            (Some("~/O'Brien project"), Some(home.display().to_string())),
        ] {
            let exec = format!(
                "cd {} {} {}",
                match working_dir.and_then(|working_dir| working_dir.strip_prefix("~/")) {
                    Some(remainder) => home_relative_path(ShellKind::Nushell, Some(remainder))?,
                    None => ShellKind::Nushell
                        .try_quote(working_dir.context("working dir")?)
                        .context("shell quoting")?
                        .into_owned(),
                },
                ShellKind::Nushell.sequential_and_commands_separator(),
                home_stdio_launcher_command(
                    ShellKind::Nushell,
                    home_dir.as_deref(),
                    ".local/share/zed/remote server"
                )?
            );
            let mut child = Command::new(nu)
                .args(["--no-config-file", "-c", &exec])
                .env_clear()
                .env("HOME", &home)
                .env("PATH", "/usr/bin:/bin")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert!(output.status.success(), "{exec}: {:?}", output.stderr);
            assert_eq!(String::from_utf8_lossy(&output.stdout), expected, "{exec}");
            assert_eq!(output.stderr, b"");
        }
        Ok(())
    })
}

#[test]
fn test_remote_command_launcher_survives_local_nushell() -> Result<()> {
    let Some(nu) = ["/opt/homebrew/bin/nu", "/usr/local/bin/nu", "/usr/bin/nu"]
        .into_iter()
        .find(|nu| std::path::Path::new(nu).exists())
    else {
        return Ok(());
    };
    smol::block_on(async {
        let directory = tempfile::tempdir()?;
        let home = directory.path().join("home$REMOTE_PART");
        let project = home.join("project");
        std::fs::create_dir_all(&project)?;
        install_helper(&home)?;
        let descriptor = RemoteCommand {
            program: String::from("/bin/sh"),
            args: vec![
                String::from("-c"),
                String::from("printf '%s\\n%s\\n' \"$AGENT_TOKEN\" \"$(pwd -P)\""),
            ],
            env: HashMap::from_iter([(String::from("AGENT_TOKEN"), String::from("secret"))]),
            working_dir: None,
        };
        let expected = format!("secret\n{}\n", project.canonicalize()?.display());
        for home_dir in [None, Some(home.display().to_string())] {
            let exec = format!(
                "cd {} && {}",
                home_relative_path(ShellKind::Posix, Some("project"))?,
                home_stdio_launcher_command(
                    ShellKind::Posix,
                    home_dir.as_deref(),
                    ".local/share/zed/remote server"
                )?
            );
            let remote_shell = [String::from("-c"), exec.clone()];
            let mut child = Command::from(
                ShellBuilder::new(&Shell::Program(String::from(nu)), false)
                    .non_interactive()
                    .literal_args()
                    .build_std_command(Some(String::from("/bin/bash")), &remote_shell),
            )
            .env_clear()
            .env("HOME", &home)
            .env("PATH", "/usr/bin:/bin")
            .current_dir(directory.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
            child
                .stdin
                .take()
                .context("missing stdin")?
                .write_all(&descriptor.encode()?)
                .await?;
            let output = child.output().await?;
            assert!(output.status.success(), "{exec}: {:?}", output.stderr);
            assert_eq!(String::from_utf8_lossy(&output.stdout), expected, "{exec}");
            assert_eq!(output.stderr, b"");
        }
        Ok(())
    })
}

fn launcher() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_remote_server"));
    command
        .arg("exec")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

fn install_helper(home: &std::path::Path) -> Result<String> {
    let helper_directory = home.join(".local/share/zed");
    std::fs::create_dir_all(&helper_directory)?;
    let helper = helper_directory.join("remote server");
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_remote_server"), &helper)?;
    Ok(helper.display().to_string())
}

fn shell_launcher(home: &std::path::Path, exec: &str) -> Command {
    let mut command = Command::new("/bin/bash");
    command
        .args(["-i", "-c", exec])
        .env_clear()
        .env("HOME", home)
        .env("PATH", "/usr/bin:/bin")
        .env("TERM", "dumb")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}
