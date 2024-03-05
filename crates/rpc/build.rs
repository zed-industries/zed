fn main() {
    let mut build = prost_build::Config::new();
    build
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .type_attribute(".", "#[allow(clippy::large_enum_variant)]")
        .compile_protos(&["proto/zed.proto"], &["proto"])
        .unwrap();
}
