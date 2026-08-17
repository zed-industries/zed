// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "gen")]
fn generate_protobuf_binding_file() {
    protobuf_codegen::Codegen::new()
        .out_dir("proto")
        .inputs(["proto/proto_model.proto"])
        .includes(["proto"])
        .run()
        .expect("Protobuf codegen failed");
}

#[cfg(not(feature = "gen"))]
fn generate_protobuf_binding_file() {}

fn main() {
    generate_protobuf_binding_file()
}
