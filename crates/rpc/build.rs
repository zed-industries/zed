fn main() {
    let mut build = prost_build::Config::new();

    build
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .compile_protos(&["proto/zed.proto"], &["proto"])
        .unwrap();

    println!("cargo:rerun-if-changed=src/json.rs");
}
