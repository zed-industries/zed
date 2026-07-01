fn main() {
    println!("cargo:rerun-if-changed=proto");
    // Compile the descriptor set with protox (a pure-Rust protobuf compiler) so
    // building Zed doesn't require a system `protoc` binary.
    let file_descriptors = protox::compile(["proto/zed.proto"], ["proto"]).unwrap();
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("ProjectPath", "#[derive(Hash, Eq)]")
        .type_attribute("Anchor", "#[derive(Hash, Eq)]")
        .compile_fds(file_descriptors)
        .unwrap();
}
