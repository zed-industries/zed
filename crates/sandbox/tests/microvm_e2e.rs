//! Live end-to-end test of the `microvm` sandbox backend against a real smol
//! engine. Ignored by default; run with
//! `ZED_SANDBOX_MICROVM_E2E=1 cargo test -p sandbox --test microvm_e2e -- --ignored`.
//! Needs a hypervisor and either `SMOLVM=<engine binary>` or network access
//! for the SDK to fetch its engine. Creates one machine named
//! `zed-sandbox-<hash>` labelled `zed.sandbox=1` and deletes it afterwards.
#![cfg(target_os = "linux")]
#![allow(
    clippy::disallowed_methods,
    reason = "the test spawns the wrapped command exactly as a blocking harness would"
)]

use std::{collections::HashMap, fs, path::Path, process::Command};

use sandbox::{
    CommandAndArgs, HostFilesystemLocation, Sandbox, SandboxBackend, SandboxFsPolicy,
    SandboxNetPolicy, SandboxPolicy,
};

fn enabled() -> bool {
    std::env::var("ZED_SANDBOX_MICROVM_E2E").is_ok_and(|value| value == "1")
}

fn policy(project: &Path, git_dir: &Path, network: SandboxNetPolicy) -> SandboxPolicy {
    SandboxPolicy {
        fs: SandboxFsPolicy::Restricted {
            writable_paths: vec![HostFilesystemLocation::capture(project).unwrap()],
            protected_paths: vec![HostFilesystemLocation::capture(git_dir).unwrap()],
        },
        network,
    }
}

/// Run `script` through the backend the way Zed does: wrap the host shell's
/// `-c <script>` invocation and spawn the returned argv.
fn run(sandbox: &mut Sandbox, project: &Path, script: &str) -> (i32, String, String) {
    let command = CommandAndArgs {
        program: "/bin/sh".to_string(),
        args: vec![
            "-i".to_string(),
            "-c".to_string(),
            format!("exec </dev/null; {script}"),
        ],
        env: HashMap::from([("ZED_E2E_MARKER".to_string(), "present".to_string())]),
        cwd: Some(project.to_path_buf()),
    };
    let wrapped = futures::executor::block_on(sandbox.wrap(&command)).expect("wrap");
    let output = Command::new(&wrapped.program)
        .args(&wrapped.args)
        .envs(&wrapped.env)
        .output()
        .expect("spawn wrapped command");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// The engine's sandbox machines, found through the SDK by label rather than
/// by re-deriving the backend's name hash here.
fn machine_names() -> Vec<String> {
    smolmachines::Machine::list(
        &smolmachines::ConnectOptions::local(),
        &[("zed.sandbox", "1")],
    )
    .expect("list machines")
    .into_iter()
    .map(|machine| machine.name)
    .collect()
}

#[test]
#[ignore = "needs a smol engine and a hypervisor; set ZED_SANDBOX_MICROVM_E2E=1"]
fn microvm_backend_runs_commands_in_a_project_machine() {
    if !enabled() {
        eprintln!("ZED_SANDBOX_MICROVM_E2E is not set; skipping");
        return;
    }
    let root = tempfile::tempdir().expect("tempdir");
    let project = root.path().join("project");
    let git_dir = project.join(".git");
    fs::create_dir_all(git_dir.join("refs")).unwrap();
    fs::write(project.join("hello.txt"), "hello from host\n").unwrap();
    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n").unwrap();
    let before = machine_names();

    let blocked = policy(&project, &git_dir, SandboxNetPolicy::Blocked);
    assert_eq!(
        Sandbox::can_create_with(&blocked, SandboxBackend::Microvm),
        Ok(())
    );
    let mut sandbox = Sandbox::with_backend(blocked, SandboxBackend::Microvm).unwrap();

    // 1. Zed's exact spawn shape (`sh -i -c "exec </dev/null; …"`) exits 0 and
    //    prints only the command's output, inside the guest's own session.
    let (code, stdout, stderr) = run(&mut sandbox, &project, "printf 'works\\n'");
    assert_eq!(code, 0, "stderr: {stderr}");
    assert_eq!(stdout.trim_end(), "works");
    assert!(!stderr.contains("process group"), "stderr: {stderr}");

    // 2. The project is the same path inside; writes land on the host; the
    //    protected Git metadata is read-only; the guest env carries the
    //    command's variables and the guest is a different kernel.
    let (code, stdout, _) = run(
        &mut sandbox,
        &project,
        &format!(
            "cat {p}/hello.txt && pwd && echo \"marker=$ZED_E2E_MARKER\" && echo guest > {p}/from-guest.txt && (echo x > {p}/.git/blocked 2>&1 || echo 'git metadata is read-only') && uname -r",
            p = project.display()
        ),
    );
    assert_eq!(code, 0, "stdout: {stdout}");
    assert!(stdout.contains("hello from host"), "{stdout}");
    assert!(stdout.contains(&project.display().to_string()), "{stdout}");
    assert!(stdout.contains("marker=present"), "{stdout}");
    assert!(stdout.contains("git metadata is read-only"), "{stdout}");
    assert_eq!(
        fs::read_to_string(project.join("from-guest.txt")).unwrap(),
        "guest\n"
    );
    assert!(!git_dir.join("blocked").exists());
    let host_kernel =
        String::from_utf8(Command::new("uname").arg("-r").output().unwrap().stdout).unwrap();
    assert!(
        !stdout.contains(host_kernel.trim()),
        "guest reports the host kernel: {stdout}"
    );

    // 3. Tool installs need no host grant and leave the host untouched.
    let (code, stdout, _) = run(
        &mut sandbox,
        &project,
        "mkdir -p /usr/local/lib/zed-e2e && echo installed > /usr/local/lib/zed-e2e/tool && cat /usr/local/lib/zed-e2e/tool",
    );
    assert_eq!(code, 0);
    assert!(stdout.contains("installed"));
    assert!(!Path::new("/usr/local/lib/zed-e2e").exists());

    // 4. network: Blocked means no egress at all.
    let (_, stdout, _) = run(
        &mut sandbox,
        &project,
        "if getent hosts deb.debian.org >/dev/null 2>&1; then echo REACHABLE; else echo blocked; fi",
    );
    assert!(stdout.contains("blocked"), "{stdout}");

    // 5. A command with open network on the same project reuses the machine
    //    with the policy switched, and egress works.
    let open = policy(&project, &git_dir, SandboxNetPolicy::Unrestricted);
    let mut sandbox_open = Sandbox::with_backend(open, SandboxBackend::Microvm).unwrap();
    let (_, stdout, _) = run(
        &mut sandbox_open,
        &project,
        "if getent hosts deb.debian.org >/dev/null 2>&1; then echo reachable; else echo BLOCKED; fi",
    );
    assert!(stdout.contains("reachable"), "{stdout}");

    // Exactly one machine was created for the project, and it is cleaned up.
    let after = machine_names();
    let created: Vec<&String> = after.iter().filter(|name| !before.contains(name)).collect();
    assert_eq!(created.len(), 1, "machines after: {after:?}");
    smolmachines::Machine::attach(created[0].as_str())
        .expect("attach machine")
        .delete()
        .expect("delete machine");
}
