fn main() {
    println!("cargo:rerun-if-changed=proto");
    let file_descriptors = protox::compile(["proto/zed.proto"], ["proto"]).unwrap();
    let mut build = prost_build::Config::new();
    build
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_fds(file_descriptors)
        .unwrap();
}
