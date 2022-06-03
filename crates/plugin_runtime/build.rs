use std::env;
use std::process::Command;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    let plugin_workspace = cwd.join("plugin").join("Cargo.toml");
    Command::new("cargo")
        .args(&["clean", "--manifest-path"])
        .arg(&plugin_workspace)
        .status()
        .unwrap();
    Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--manifest-path",
        ])
        .arg(&plugin_workspace)
        .status()
        .unwrap();
    println!("cargo:warning=recompiling plugins")
}
