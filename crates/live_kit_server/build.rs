fn main() {
    prost_build::Config::new()
        .type_attribute("SendDataResponse", "#[allow(clippy::empty_docs)]")
        .compile_protos(&["protocol/livekit_room.proto"], &["protocol"])
        .unwrap();
}
