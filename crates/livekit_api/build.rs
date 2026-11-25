fn main() {
    println!("cargo:rerun-if-changed=vendored/protocol");
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
