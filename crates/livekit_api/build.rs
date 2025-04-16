fn main() {
    prost_build::Config::new()
        .type_attribute("SendDataResponse", "#[allow(clippy::empty_docs)]")
        .compile_protos(
            &["vendored/protocol/livekit_room.proto"],
            &["vendored/protocol"],
        )
        .unwrap();
}
