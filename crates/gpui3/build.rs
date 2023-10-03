use std::{
    env,
    path::{Path, PathBuf},
    process::{self, Command},
};

use cbindgen::Config;

fn main() {
    generate_dispatch_bindings();
    let header_path = generate_shader_bindings();
    compile_metal_shaders(&header_path);
}

fn generate_dispatch_bindings() {
    println!("cargo:rustc-link-lib=framework=System");
    println!("cargo:rerun-if-changed=src/platform/mac/dispatch.h");

    let bindings = bindgen::Builder::default()
        .header("src/platform/mac/dispatch.h")
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

fn generate_shader_bindings() -> PathBuf {
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("scene.h");
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut config = Config::default();
    config.include_guard = Some("SCENE_H".into());
    config.language = cbindgen::Language::C;
    config.export.include.extend([
        "Bounds".into(),
        "Corners".into(),
        "Edges".into(),
        "Size".into(),
        "Pixels".into(),
        "PointF".into(),
        "Hsla".into(),
        "Quad".into(),
        "QuadInputIndex".into(),
        "QuadUniforms".into(),
        "AtlasTile".into(),
        "MonochromeSprite".into(),
    ]);
    config.no_includes = true;
    config.enumeration.prefix_with_name = true;
    cbindgen::Builder::new()
        .with_src(crate_dir.join("src/scene.rs"))
        .with_src(crate_dir.join("src/geometry.rs"))
        .with_src(crate_dir.join("src/color.rs"))
        .with_src(crate_dir.join("src/platform.rs"))
        .with_src(crate_dir.join("src/platform/mac/metal_renderer.rs"))
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&output_path);
    output_path
}

fn compile_metal_shaders(header_path: &Path) {
    let shader_path = "./src/platform/mac/shaders.metal";
    let air_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.air");
    let metallib_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.metallib");

    println!("cargo:rerun-if-changed={}", header_path.display());
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
            "-include",
            &header_path.to_str().unwrap(),
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
