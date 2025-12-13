fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = which::which("protoc").inspect_err(|e| {
        eprintln!(
            "Protoc not found: {}. Please install protoc or ensure it's in your PATH.",
            e
        );
    })?;

    prost_build::Config::new()
        .type_attribute("SendDataResponse", "#[allow(clippy::empty_docs)]")
        .protoc_arg(format!("--proto_path={}", protoc_path.display()))
        .compile_protos(
            &["vendored/protocol/livekit_room.proto"],
            &["vendored/protocol"],
        )?; // Use '?' to propagate errors
    Ok(())
}
