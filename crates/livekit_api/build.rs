fn main() {
    println!("cargo:rerun-if-changed=vendored/protocol");
    // Compile the descriptor set with protox (a pure-Rust protobuf compiler) so
    // building Zed doesn't require a system `protoc` binary.
    let file_descriptors = protox::compile(
        ["vendored/protocol/livekit_room.proto"],
        ["vendored/protocol"],
    )
    .unwrap();
    prost_build::Config::new()
        .type_attribute("SendDataResponse", "#[allow(clippy::empty_docs)]")
        .compile_fds(file_descriptors)
        .unwrap();
}
