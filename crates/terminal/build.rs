fn main() {
    // Get ZED version from crates/zed/Cargo.toml for TERM_PROGRAM_VERSION.
    const ZED_MANIFEST_PATH: &str = "../zed/Cargo.toml";
    println!("cargo::rerun-if-changed={}", ZED_MANIFEST_PATH);
    let manifest =
        std::fs::read_to_string(ZED_MANIFEST_PATH).expect("Read zed's Cargo.toml failed.");
    let manifest: toml::Value = toml::from_str(&manifest).expect("Parse zed's Cargo.toml failed");
    let version = manifest["package"]["version"].as_str().unwrap();
    println!("cargo:rustc-env=ZED_VERSION={version}");
    println!("cargo:warning=Info: using '{version}' for ZED_VERSION env var");
}
