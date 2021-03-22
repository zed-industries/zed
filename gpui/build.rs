use std::{
    env,
    path::PathBuf,
    process::{self, Command},
};

fn main() {
    generate_dispatch_bindings();
    compile_context_predicate_parser();
    compile_metal_shaders();
    generate_shader_bindings();
}

fn generate_dispatch_bindings() {
    println!("cargo:rustc-link-lib=framework=System");
    println!("cargo:rerun-if-changed=src/platform/mac/dispatch.h");

    let bindings = bindgen::Builder::default()
        .header("src/platform/mac/dispatch.h")
        .whitelist_var("_dispatch_main_q")
        .whitelist_function("dispatch_async_f")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dispatch_sys.rs"))
        .expect("couldn't write dispatch bindings");
}

fn compile_context_predicate_parser() {
    let dir = PathBuf::from("./grammars/context-predicate/src");
    let parser_c = dir.join("parser.c");

    println!("cargo:rerun-if-changed={}", &parser_c.to_str().unwrap());
    cc::Build::new()
        .include(&dir)
        .file(parser_c)
        .compile("tree_sitter_context_predicate");
}

const SHADER_HEADER_PATH: &'static str = "./src/platform/mac/shaders/shaders.h";

fn compile_metal_shaders() {
    let shader_path = "./src/platform/mac/shaders/shaders.metal";
    let air_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.air");
    let metallib_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.metallib");

    println!("cargo:rerun-if-changed={}", SHADER_HEADER_PATH);
    println!("cargo:rerun-if-changed={}", shader_path);

    let output = Command::new("xcrun")
        .args(&["-sdk", "macosx", "metal", "-c", shader_path, "-o"])
        .arg(&air_output_path)
        .output()
        .unwrap();

    if !output.status.success() {
        eprintln!(
            "metal shader compilation failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        process::exit(1);
    }

    let output = Command::new("xcrun")
        .args(&["-sdk", "macosx", "metallib"])
        .arg(air_output_path)
        .arg("-o")
        .arg(metallib_output_path)
        .output()
        .unwrap();

    if !output.status.success() {
        eprintln!(
            "metallib compilation failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        process::exit(1);
    }
}

fn generate_shader_bindings() {
    let bindings = bindgen::Builder::default()
        .header(SHADER_HEADER_PATH)
        .whitelist_type("GPUIUniforms")
        .whitelist_type("GPUIQuadInputIndex")
        .whitelist_type("GPUIQuad")
        .whitelist_type("GPUIShadowInputIndex")
        .whitelist_type("GPUIShadow")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("shaders.rs"))
        .expect("couldn't write shader bindings");
}
