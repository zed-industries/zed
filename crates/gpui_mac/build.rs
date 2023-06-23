use std::{
    env,
    path::PathBuf,
    process::{self, Command},
};

fn main() {
    generate_dispatch_bindings();
    compile_metal_shaders();
    generate_shader_bindings();
}

fn generate_dispatch_bindings() {
    println!("cargo:rustc-link-lib=framework=System");
    println!("cargo:rerun-if-changed=src/dispatch.h");

    let bindings = bindgen::Builder::default()
        .header("src/dispatch.h")
        .allowlist_var("_dispatch_main_q")
        .allowlist_function("dispatch_async_f")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dispatch_sys.rs"))
        .expect("couldn't write dispatch bindings");
}

const SHADER_HEADER_PATH: &str = "./src/shaders/shaders.h";

fn compile_metal_shaders() {
    let shader_path = "./src/shaders/shaders.metal";
    let air_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.air");
    let metallib_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.metallib");

    println!("cargo:rerun-if-changed={}", SHADER_HEADER_PATH);
    println!("cargo:rerun-if-changed={}", shader_path);

    let output = Command::new("xcrun")
        .args([
            "-sdk",
            "macosx",
            "metal",
            "-gline-tables-only",
            "-mmacosx-version-min=10.15.7",
            "-MO",
            "-c",
            shader_path,
            "-o",
        ])
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
        .args(["-sdk", "macosx", "metallib"])
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
        .allowlist_type("GPUI.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("shaders.rs"))
        .expect("couldn't write shader bindings");
}
