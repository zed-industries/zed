fn main() {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = std::path::PathBuf::from(&cargo_manifest_dir);

    if path.file_name().as_ref().and_then(|name| name.to_str()) != Some("inspector_ui") {
        panic!(
            "expected CARGO_MANIFEST_DIR to end with crates/inspector_ui, but got {cargo_manifest_dir}"
        );
    }
    path.pop();

    if path.file_name().as_ref().and_then(|name| name.to_str()) != Some("crates") {
        panic!(
            "expected CARGO_MANIFEST_DIR to end with crates/inspector_ui, but got {cargo_manifest_dir}"
        );
    }
    path.pop();

    println!("cargo:rustc-env=ZED_REPO_DIR={}", path.display());
}
