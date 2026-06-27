fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=proto");
    // Compile the descriptor set with protox (a pure-Rust protobuf compiler) so
    // building Zed doesn't require a system `protoc` binary.
    let file_descriptors = protox::compile(["proto/zed.proto"], ["proto"])
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("ProjectPath", "#[derive(Hash, Eq)]")
        .type_attribute("Anchor", "#[derive(Hash, Eq)]")
        .compile_fds(file_descriptors)
}
