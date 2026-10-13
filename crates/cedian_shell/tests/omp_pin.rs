//! ADR-0057 decision 4, plus U13's bundle: a set `CEDIAN_OMP_BINARY` runs as
//! given; then the OMP inside `cedian.app` (`Contents/Resources/omp`), which
//! the bundle shipped; else the first of PATH's `omp` and `~/.local/bin/omp`
//! whose `--version` is the pin, else the first one found. Off the pin, a
//! warning names both versions. One test: it sets the process environment.

use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

fn fake(dir: &Path, version: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let path = dir.join("omp");
    std::fs::write(&path, format!("#!/bin/sh\necho omp/{version}\n")).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

#[test]
fn the_pinned_omp_is_found_first_else_the_first_one_warns() {
    let root = std::env::temp_dir().join(format!("cedian-omp-pin-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let on_path = fake(&root.join("bin"), "18.7.0");
    let home = root.join("home");
    let local = fake(&home.join(".local/bin"), "18.8.6");
    let env_omp = fake(&root.join("env"), "18.8.6");
    let set = |path: &Path, home: &Path, env: Option<&Path>| unsafe {
        // SAFETY: the only test in this binary; nothing else reads the env.
        std::env::set_var("PATH", path);
        std::env::set_var("HOME", home);
        match env {
            Some(env) => std::env::set_var("CEDIAN_OMP_BINARY", env),
            None => std::env::remove_var("CEDIAN_OMP_BINARY"),
        }
    };

    set(&root.join("bin"), &home, None);
    let choice = cedian_shell::launch::omp_binary().unwrap();
    assert_eq!(
        choice.binary, local,
        "PATH's 18.7.0 is skipped for ~/.local/bin's pin"
    );
    assert_eq!(choice.warning, None);

    let off_pin = fake(&root.join("env-off"), "18.9.0");
    set(&root.join("bin"), &home, Some(&off_pin));
    let choice = cedian_shell::launch::omp_binary().unwrap();
    assert_eq!(
        choice.binary, off_pin,
        "a set CEDIAN_OMP_BINARY wins over ~/.local/bin's pin"
    );
    let warning = choice.warning.expect("an off-pin CEDIAN_OMP_BINARY warns");
    assert!(
        warning.contains("18.9.0") && warning.contains("18.8.6"),
        "{warning}"
    );

    set(&home.join(".local/bin"), &home, Some(&env_omp));
    assert_eq!(
        cedian_shell::launch::omp_binary().unwrap(),
        cedian_shell::launch::OmpChoice {
            binary: env_omp,
            warning: None
        },
        "PATH already holds the pin, and the set CEDIAN_OMP_BINARY still wins"
    );

    let nobody = root.join("nobody");

    set(&root.join("bin"), &nobody, None);
    let choice = cedian_shell::launch::omp_binary().unwrap();
    assert_eq!(choice.binary, on_path, "none pinned: the first found runs");
    let warning = choice.warning.expect("a warning when no omp is the pin");
    assert!(
        warning.contains("18.7.0") && warning.contains("18.8.6"),
        "{warning}"
    );

    // U13's bundle: the OMP inside `cedian.app` is what the app runs, ahead of
    // anything on PATH (`script/cedian-build-omp` puts the pin there).
    let app = root.join("bundle/cedian.app");
    let exe = app.join("Contents/MacOS/cedian");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, "the app binary").unwrap();
    let sidecar = fake(&app.join("Contents/Resources"), "18.8.6");

    set(&home.join(".local/bin"), &home, None);
    let choice = cedian_shell::launch::omp_binary_for(Some(&exe)).unwrap();
    assert_eq!(
        choice.binary, sidecar,
        "the bundle's own OMP wins over ~/.local/bin's pin"
    );
    assert_eq!(choice.warning, None);

    // The bundle's OMP runs as the bundle shipped it, pin or not: an off-pin
    // sidecar is still chosen over PATH's pin, with the warning.
    let odd = fake(&root.join("odd"), "19.0.0");
    std::fs::copy(&odd, &sidecar).unwrap();
    let choice = cedian_shell::launch::omp_binary_for(Some(&exe)).unwrap();
    assert_eq!(choice.binary, sidecar, "an off-pin sidecar still runs");
    let warning = choice.warning.expect("an off-pin sidecar warns");
    assert!(
        warning.contains("19.0.0") && warning.contains("18.8.6"),
        "{warning}"
    );
    // Only a `Contents/MacOS/<binary>` layout is a bundle: an executable next
    // to a `Resources/` directory is not one.
    let plain = root.join("plain/MacOS/cedian");
    std::fs::create_dir_all(plain.parent().unwrap()).unwrap();
    std::fs::write(&plain, "a binary").unwrap();
    fake(&root.join("plain/Resources"), "19.1.0");
    set(&home.join(".local/bin"), &home, None);
    assert_eq!(
        cedian_shell::launch::omp_binary_for(Some(&plain))
            .unwrap()
            .binary,
        local,
        "no bundle layout: the PATH scan still finds ~/.local/bin's pin"
    );

    // The bundle's sidecar outranks PATH even when PATH holds the pin.
    set(&home.join(".local/bin"), &home, None);
    std::fs::copy(&odd, &sidecar).unwrap();
    let choice = cedian_shell::launch::omp_binary_for(Some(&exe)).unwrap();
    assert_eq!(choice.binary, sidecar, "the sidecar still wins, off-pin");
    assert_eq!(
        choice.warning.as_deref().map(|w| w.contains("19.0.0")),
        Some(true)
    );

    set(&root.join("empty"), &nobody, None);
    assert!(
        cedian_shell::launch::omp_binary().is_err(),
        "no omp anywhere"
    );
    let _ = std::fs::remove_dir_all(&root);
}
