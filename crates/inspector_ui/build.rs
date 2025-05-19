fn main() {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let Some(zed_repo_dir) = cargo_manifest_dir.strip_suffix("crates/inspector_ui") else {
        panic!(
            "expected CARGO_MANIFEST_DIR to match crates/inspector_ui/, but got {cargo_manifest_dir}"
        );
    };
    println!("cargo:rustc-env=ZED_REPO_DIR={}", zed_repo_dir);
}
