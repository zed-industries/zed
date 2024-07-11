fn main() {
    let mut build = prost_build::Config::new();
    build
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/zed.proto"], &["proto"])
        .unwrap();
}
