#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    apple_build::run();
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple_build {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    use cbindgen::Config;

    pub fn run() {
        let header_path = generate_shader_bindings();

        #[cfg(feature = "runtime_shaders")]
        emit_stitched_shaders(&header_path);
        #[cfg(not(feature = "runtime_shaders"))]
        compile_metal_shaders(&header_path);
    }

    fn generate_shader_bindings() -> PathBuf {
        let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("scene.h");
        let gpui_dir: PathBuf = gpui::GPUI_MANIFEST_DIR.into();

        let mut config = Config {
            include_guard: Some("SCENE_H".into()),
            language: cbindgen::Language::C,
            no_includes: true,
            ..Default::default()
        };
        config.export.include.extend([
            "Bounds".into(),
            "Corners".into(),
            "Edges".into(),
            "Size".into(),
            "Pixels".into(),
            "PointF".into(),
            "Hsla".into(),
            "ContentMask".into(),
            "Uniforms".into(),
            "AtlasTile".into(),
            "PathRasterizationInputIndex".into(),
            "PathVertex_ScaledPixels".into(),
            "PathRasterizationVertex".into(),
            "ShadowInputIndex".into(),
            "Shadow".into(),
            "QuadInputIndex".into(),
            "Underline".into(),
            "UnderlineInputIndex".into(),
            "Quad".into(),
            "BorderStyle".into(),
            "SpriteInputIndex".into(),
            "MonochromeSprite".into(),
            "PolychromeSprite".into(),
            "PathSprite".into(),
            "SurfaceInputIndex".into(),
            "SurfaceBounds".into(),
            "TransformationMatrix".into(),
        ]);
        config.enumeration.prefix_with_name = true;

        let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let gpui_src_paths = [
            gpui_dir.join("src/scene.rs"),
            gpui_dir.join("src/geometry.rs"),
            gpui_dir.join("src/color.rs"),
            gpui_dir.join("src/window.rs"),
            gpui_dir.join("src/platform.rs"),
        ];
        let local_src_paths = [crate_dir.join("src/metal_renderer.rs")];

        let mut builder = cbindgen::Builder::new();
        for src_path in gpui_src_paths.iter().chain(local_src_paths.iter()) {
            println!("cargo:rerun-if-changed={}", src_path.display());
            builder = builder.with_src(src_path);
        }

        builder
            .with_config(config)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(&output_path);

        output_path
    }

    #[cfg(feature = "runtime_shaders")]
    fn emit_stitched_shaders(header_path: &Path) {
        let header_contents = std::fs::read_to_string(header_path).unwrap();
        let shader_path = Path::new("./src/shaders.metal");
        let shader_contents = std::fs::read_to_string(shader_path).unwrap();
        let output_path =
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("stitched_shaders.metal");
        std::fs::write(output_path, format!("{header_contents}\n{shader_contents}")).unwrap();
        println!("cargo:rerun-if-changed={}", shader_path.display());
    }

    #[cfg(not(feature = "runtime_shaders"))]
    fn compile_metal_shaders(header_path: &Path) {
        use std::process::{self, Command};

        let target = env::var("TARGET").unwrap();
        let (sdk, minimum_version_argument) = if target.contains("apple-ios") {
            if target.ends_with("-sim") {
                ("iphonesimulator", "-mios-simulator-version-min=15.0")
            } else {
                ("iphoneos", "-mios-version-min=15.0")
            }
        } else {
            ("macosx", "-mmacosx-version-min=10.15.7")
        };

        let shader_path = "./src/shaders.metal";
        let air_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.air");
        let metallib_output_path =
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.metallib");
        println!("cargo:rerun-if-changed={shader_path}");

        let output = Command::new("xcrun")
            .args([
                "-sdk",
                sdk,
                "metal",
                "-gline-tables-only",
                minimum_version_argument,
                "-MO",
                "-c",
                shader_path,
                "-include",
                header_path.to_str().unwrap(),
                "-o",
            ])
            .arg(&air_output_path)
            .output()
            .unwrap();

        if !output.status.success() {
            println!(
                "cargo::error=metal shader compilation failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            process::exit(1);
        }

        let output = Command::new("xcrun")
            .args(["-sdk", sdk, "metallib"])
            .arg(air_output_path)
            .arg("-o")
            .arg(metallib_output_path)
            .output()
            .unwrap();

        if !output.status.success() {
            println!(
                "cargo::error=metallib compilation failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            process::exit(1);
        }
    }
}
