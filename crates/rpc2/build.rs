fn main() {
    let mut build = prost_build::Config::new();
    // build.protoc_arg("--experimental_allow_proto3_optional");
    build
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .compile_protos(&["proto/zed.proto"], &["proto"])
        .unwrap();
}
