fn main() {
    println!("cargo:rerun-if-changed=proto");
    let mut build = prost_build::Config::new();
    build
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("ProjectPath", "#[derive(Hash, Eq)]")
        .type_attribute("Anchor", "#[derive(Hash, Eq)]")
        .compile_protos(&["proto/zed.proto"], &["proto"])
        .unwrap();
}
