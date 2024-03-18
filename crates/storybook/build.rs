fn main() {
    // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle.
    // TODO: We shouldn't depend on WebRTC in editor
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");

    if std::env::var("CARGO_CFG_TARGET_ENV").ok() == Some("msvc".to_string()) {
        println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);

        let manifest = std::path::Path::new("../zed/resources/windows/manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");

        println!(
            "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
            manifest.canonicalize().unwrap().display()
        );
    }
}
