#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
#![cfg_attr(any(not(target_os = "macos"), feature = "macos-blade"), allow(unused))]

//TODO: consider generating shader code for WGSL
//TODO: deprecate "runtime-shaders" and "macos-blade"

use std::env;

fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS");
    println!("cargo::rustc-check-cfg=cfg(gles)");

    #[cfg(any(
        not(any(target_os = "macos", target_os = "windows")),
        all(target_os = "macos", feature = "macos-blade")
    ))]
    check_wgsl_shaders();

    match target.as_deref() {
        Ok("macos") => {
            #[cfg(target_os = "macos")]
            macos::build();
        }
        Ok("windows") => {
            #[cfg(target_os = "windows")]
            windows::build();
        }
        _ => (),
    };
}

#[cfg(any(
    not(any(target_os = "macos", target_os = "windows")),
    all(target_os = "macos", feature = "macos-blade")
))]
fn check_wgsl_shaders() {
    use std::path::PathBuf;
    use std::process;
    use std::str::FromStr;

    let shader_source_path = "./src/platform/blade/shaders.wgsl";
    let shader_path = PathBuf::from_str(shader_source_path).unwrap();
    println!("cargo:rerun-if-changed={}", &shader_path.display());

    let shader_source = std::fs::read_to_string(&shader_path).unwrap();

    match naga::front::wgsl::parse_str(&shader_source) {
        Ok(_) => {
            // All clear
        }
        Err(e) => {
            println!("cargo::error=WGSL shader compilation failed:\n{}", e);
            process::exit(1);
        }
    }
}
#[cfg(target_os = "macos")]
mod macos {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    use cbindgen::Config;

    pub(super) fn build() {
        generate_dispatch_bindings();
        #[cfg(not(feature = "macos-blade"))]
        {
            let header_path = generate_shader_bindings();

            #[cfg(feature = "runtime_shaders")]
            emit_stitched_shaders(&header_path);
            #[cfg(not(feature = "runtime_shaders"))]
            compile_metal_shaders(&header_path);
        }
    }

    fn generate_dispatch_bindings() {
        println!("cargo:rustc-link-lib=framework=System");

        let bindings = bindgen::Builder::default()
            .header("src/platform/mac/dispatch.h")
            .allowlist_var("_dispatch_main_q")
            .allowlist_var("_dispatch_source_type_data_add")
            .allowlist_var("DISPATCH_QUEUE_PRIORITY_HIGH")
            .allowlist_var("DISPATCH_TIME_NOW")
            .allowlist_function("dispatch_get_global_queue")
            .allowlist_function("dispatch_async_f")
            .allowlist_function("dispatch_after_f")
            .allowlist_function("dispatch_time")
            .allowlist_function("dispatch_source_merge_data")
            .allowlist_function("dispatch_source_create")
            .allowlist_function("dispatch_source_set_event_handler_f")
            .allowlist_function("dispatch_resume")
            .allowlist_function("dispatch_suspend")
            .allowlist_function("dispatch_source_cancel")
            .allowlist_function("dispatch_set_context")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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
        config.no_includes = true;
        config.enumeration.prefix_with_name = true;

        let mut builder = cbindgen::Builder::new();

        let src_paths = [
            crate_dir.join("src/scene.rs"),
            crate_dir.join("src/geometry.rs"),
            crate_dir.join("src/color.rs"),
            crate_dir.join("src/window.rs"),
            crate_dir.join("src/platform.rs"),
            crate_dir.join("src/platform/mac/metal_renderer.rs"),
        ];
        for src_path in src_paths {
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

    /// To enable runtime compilation, we need to "stitch" the shaders file with the generated header
    /// so that it is self-contained.
    #[cfg(feature = "runtime_shaders")]
    fn emit_stitched_shaders(header_path: &Path) {
        use std::str::FromStr;
        fn stitch_header(header: &Path, shader_path: &Path) -> std::io::Result<PathBuf> {
            let header_contents = std::fs::read_to_string(header)?;
            let shader_contents = std::fs::read_to_string(shader_path)?;
            let stitched_contents = format!("{header_contents}\n{shader_contents}");
            let out_path =
                PathBuf::from(env::var("OUT_DIR").unwrap()).join("stitched_shaders.metal");
            std::fs::write(&out_path, stitched_contents)?;
            Ok(out_path)
        }
        let shader_source_path = "./src/platform/mac/shaders.metal";
        let shader_path = PathBuf::from_str(shader_source_path).unwrap();
        stitch_header(header_path, &shader_path).unwrap();
        println!("cargo:rerun-if-changed={}", &shader_source_path);
    }

    #[cfg(not(feature = "runtime_shaders"))]
    fn compile_metal_shaders(header_path: &Path) {
        use std::process::{self, Command};
        let shader_path = "./src/platform/mac/shaders.metal";
        let air_output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.air");
        let metallib_output_path =
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.metallib");
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
                (header_path.to_str().unwrap()),
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
            .args(["-sdk", "macosx", "metallib"])
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

#[cfg(target_os = "windows")]
mod windows {
    use std::{
        fs,
        io::Write,
        path::{Path, PathBuf},
        process::{self, Command},
    };

    pub(super) fn build() {
        // Compile HLSL shaders
        #[cfg(not(debug_assertions))]
        compile_shaders();

        // Embed the Windows manifest and resource file
        #[cfg(feature = "windows-manifest")]
        embed_resource();
    }

    #[cfg(feature = "windows-manifest")]
    fn embed_resource() {
        let manifest = std::path::Path::new("resources/windows/gpui.manifest.xml");
        let rc_file = std::path::Path::new("resources/windows/gpui.rc");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rerun-if-changed={}", rc_file.display());
        embed_resource::compile(rc_file, embed_resource::NONE)
            .manifest_required()
            .unwrap();
    }

    /// You can set the `GPUI_FXC_PATH` environment variable to specify the path to the fxc.exe compiler.
    fn compile_shaders() {
        let shader_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src/platform/windows/shaders.hlsl");
        let out_dir = std::env::var("OUT_DIR").unwrap();

        println!("cargo:rerun-if-changed={}", shader_path.display());

        // Check if fxc.exe is available
        let fxc_path = find_fxc_compiler();

        // Define all modules
        let modules = [
            "quad",
            "shadow",
            "path_rasterization",
            "path_sprite",
            "underline",
            "monochrome_sprite",
            "polychrome_sprite",
        ];

        let rust_binding_path = format!("{}/shaders_bytes.rs", out_dir);
        if Path::new(&rust_binding_path).exists() {
            fs::remove_file(&rust_binding_path)
                .expect("Failed to remove existing Rust binding file");
        }
        for module in modules {
            compile_shader_for_module(
                module,
                &out_dir,
                &fxc_path,
                shader_path.to_str().unwrap(),
                &rust_binding_path,
            );
        }

        {
            let shader_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("src/platform/windows/color_text_raster.hlsl");
            compile_shader_for_module(
                "emoji_rasterization",
                &out_dir,
                &fxc_path,
                shader_path.to_str().unwrap(),
                &rust_binding_path,
            );
        }
    }

    /// You can set the `GPUI_FXC_PATH` environment variable to specify the path to the fxc.exe compiler.
    fn find_fxc_compiler() -> String {
        // Check environment variable
        if let Ok(path) = std::env::var("GPUI_FXC_PATH")
            && Path::new(&path).exists()
        {
            return path;
        }

        // Try to find in PATH
        // NOTE: This has to be `where.exe` on Windows, not `where`, it must be ended with `.exe`
        if let Ok(output) = std::process::Command::new("where.exe")
            .arg("fxc.exe")
            .output()
            && output.status.success()
        {
            let path = String::from_utf8_lossy(&output.stdout);
            return path.trim().to_string();
        }

        // Check the default path
        if Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\fxc.exe")
            .exists()
        {
            return r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\fxc.exe"
                .to_string();
        }

        panic!("Failed to find fxc.exe");
    }

    fn compile_shader_for_module(
        module: &str,
        out_dir: &str,
        fxc_path: &str,
        shader_path: &str,
        rust_binding_path: &str,
    ) {
        // Compile vertex shader
        let output_file = format!("{}/{}_vs.h", out_dir, module);
        let const_name = format!("{}_VERTEX_BYTES", module.to_uppercase());
        compile_shader_impl(
            fxc_path,
            &format!("{module}_vertex"),
            &output_file,
            &const_name,
            shader_path,
            "vs_4_1",
        );
        generate_rust_binding(&const_name, &output_file, rust_binding_path);

        // Compile fragment shader
        let output_file = format!("{}/{}_ps.h", out_dir, module);
        let const_name = format!("{}_FRAGMENT_BYTES", module.to_uppercase());
        compile_shader_impl(
            fxc_path,
            &format!("{module}_fragment"),
            &output_file,
            &const_name,
            shader_path,
            "ps_4_1",
        );
        generate_rust_binding(&const_name, &output_file, rust_binding_path);
    }

    fn compile_shader_impl(
        fxc_path: &str,
        entry_point: &str,
        output_path: &str,
        var_name: &str,
        shader_path: &str,
        target: &str,
    ) {
        let output = Command::new(fxc_path)
            .args([
                "/T",
                target,
                "/E",
                entry_point,
                "/Fh",
                output_path,
                "/Vn",
                var_name,
                "/O3",
                shader_path,
            ])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    return;
                }
                println!(
                    "cargo::error=Shader compilation failed for {}:\n{}",
                    entry_point,
                    String::from_utf8_lossy(&result.stderr)
                );
                process::exit(1);
            }
            Err(e) => {
                println!("cargo::error=Failed to run fxc for {}: {}", entry_point, e);
                process::exit(1);
            }
        }
    }

    fn generate_rust_binding(const_name: &str, head_file: &str, output_path: &str) {
        let header_content = fs::read_to_string(head_file).expect("Failed to read header file");
        let const_definition = {
            let global_var_start = header_content.find("const BYTE").unwrap();
            let global_var = &header_content[global_var_start..];
            let equal = global_var.find('=').unwrap();
            global_var[equal + 1..].trim()
        };
        let rust_binding = format!(
            "const {}: &[u8] = &{}\n",
            const_name,
            const_definition.replace('{', "[").replace('}', "]")
        );
        let mut options = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_path)
            .expect("Failed to open Rust binding file");
        options
            .write_all(rust_binding.as_bytes())
            .expect("Failed to write Rust binding file");
    }
}
