fn main() {
    println!("cargo:rerun-if-changed=proto");

    // `prost-build` shells out to `protoc`.
    // Prefer an explicit `PROTOC` override, otherwise try to find `protoc` on PATH.
    // If neither exists, fall back to a vendored `protoc` (works well for local dev / CI).
    let protoc_from_env = std::env::var_os("PROTOC").filter(|v| !v.is_empty());
    if protoc_from_env.is_none() {
        if let Ok(path) = which::which("protoc") {
            // Rust 2024 marks env mutation as `unsafe`.
            unsafe { std::env::set_var("PROTOC", path) };
        } else if let Ok(path) = protoc_bin_vendored::protoc_bin_path() {
            unsafe { std::env::set_var("PROTOC", path) };
        }
    }

    let mut build = prost_build::Config::new();
    build
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("ProjectPath", "#[derive(Hash, Eq)]")
        .type_attribute("Anchor", "#[derive(Hash, Eq)]")
        .compile_protos(&["proto/zed.proto"], &["proto"])
        .unwrap_or_else(|err| {
            let protoc = std::env::var("PROTOC").unwrap_or_else(|_| "<not set>".to_string());
            panic!(
                "failed to compile protos via prost-build (PROTOC={protoc}).\n\n{err}\n\nFix: install protoc (e.g. `brew install protobuf`) or ensure it is on PATH, or set PROTOC=/path/to/protoc."
            );
        });
}
