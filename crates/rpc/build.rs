fn main() {
    let mut build = prost_build::Config::new();

    build
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .field_attribute(
            "ChatCompletionTool.FunctionObject.parameters",
            "#[serde(serialize_with = \"crate::json::serialize_prost_struct_to_json_object\")]",
        )
        .compile_protos(&["proto/zed.proto"], &["proto"])
        .unwrap();

    println!("cargo:rerun-if-changed=src/json.rs");
}
