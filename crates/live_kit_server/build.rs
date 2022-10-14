fn main() {
    prost_build::Config::new()
        .compile_protos(&["protocol/livekit_room.proto"], &["protocol"])
        .unwrap();
}
