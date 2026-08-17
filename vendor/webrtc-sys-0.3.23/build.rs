// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::Path;
use std::path::PathBuf;
use std::{env, path, process::Command};

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let is_desktop = target_os == "linux" || target_os == "windows" || target_os == "macos";

    println!("cargo:rerun-if-env-changed=LK_DEBUG_WEBRTC");
    println!("cargo:rerun-if-env-changed=LK_CUSTOM_WEBRTC");

    let mut rust_files = vec![
        "src/peer_connection.rs",
        "src/peer_connection_factory.rs",
        "src/media_stream.rs",
        "src/media_stream_track.rs",
        "src/audio_track.rs",
        "src/video_track.rs",
        "src/data_channel.rs",
        "src/frame_cryptor.rs",
        "src/jsep.rs",
        "src/candidate.rs",
        "src/rtp_parameters.rs",
        "src/rtp_sender.rs",
        "src/rtp_receiver.rs",
        "src/rtp_transceiver.rs",
        "src/rtc_error.rs",
        "src/webrtc.rs",
        "src/video_frame.rs",
        "src/video_frame_buffer.rs",
        "src/helper.rs",
        "src/yuv_helper.rs",
        "src/audio_resampler.rs",
        "src/android.rs",
        "src/prohibit_libsrtp_initialization.rs",
        "src/apm.rs",
        "src/audio_mixer.rs",
    ];

    if is_desktop {
        rust_files.push("src/desktop_capturer.rs");
    }

    let mut builder = cxx_build::bridges(rust_files);

    builder.files(&[
        "src/peer_connection.cpp",
        "src/peer_connection_factory.cpp",
        "src/media_stream.cpp",
        "src/media_stream_track.cpp",
        "src/audio_track.cpp",
        "src/video_track.cpp",
        "src/data_channel.cpp",
        "src/jsep.cpp",
        "src/candidate.cpp",
        "src/rtp_receiver.cpp",
        "src/rtp_sender.cpp",
        "src/rtp_transceiver.cpp",
        "src/rtp_parameters.cpp",
        "src/rtc_error.cpp",
        "src/webrtc.cpp",
        "src/video_frame.cpp",
        "src/video_frame_buffer.cpp",
        "src/video_encoder_factory.cpp",
        "src/video_decoder_factory.cpp",
        "src/audio_device.cpp",
        "src/audio_resampler.cpp",
        "src/frame_cryptor.cpp",
        "src/global_task_queue.cpp",
        "src/prohibit_libsrtp_initialization.cpp",
        "src/apm.cpp",
        "src/audio_mixer.cpp",
    ]);

    if is_desktop {
        builder.file("src/desktop_capturer.cpp");
    }

    let webrtc_dir = webrtc_sys_build::webrtc_dir();
    let webrtc_include = webrtc_dir.join("include");
    let webrtc_lib = webrtc_dir.join("lib");

    webrtc_sys_build::download_webrtc().unwrap();
    if !webrtc_dir.exists() {
        panic!("download_webrtc didn't create webrtc_dir at {}", webrtc_dir.display());
    }

    builder.includes(&[
        path::PathBuf::from("./include"),
        webrtc_include.clone(),
        webrtc_include.join("third_party/abseil-cpp/"),
        webrtc_include.join("third_party/libyuv/include/"),
        webrtc_include.join("third_party/libc++/"),
        // For mac & ios
        webrtc_include.join("sdk/objc"),
        webrtc_include.join("sdk/objc/base"),
    ]);
    builder.define("WEBRTC_APM_DEBUG_DUMP", "0");

    println!("cargo:rustc-link-search=native={}", webrtc_lib.to_str().unwrap());

    for (key, value) in webrtc_sys_build::webrtc_defines() {
        let value = value.as_deref();
        builder.define(key.as_str(), value);
    }

    // Link webrtc library
    println!("cargo:rustc-link-lib=static=webrtc");
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=msdmo");
            println!("cargo:rustc-link-lib=dylib=wmcodecdspuuid");
            println!("cargo:rustc-link-lib=dylib=dmoguids");
            println!("cargo:rustc-link-lib=dylib=crypt32");
            println!("cargo:rustc-link-lib=dylib=iphlpapi");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=secur32");
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=strmiids");
            println!("cargo:rustc-link-lib=dylib=d3d11");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=dxgi");
            println!("cargo:rustc-link-lib=dylib=dwmapi");
            println!("cargo:rustc-link-lib=dylib=shcore");

            //let path = env::current_dir().unwrap();
            //println!("cargo:rustc-link-search=native={}/vaapi-windows/x64/lib", path.display());
            //println!("cargo:rustc-link-lib=dylib=va");
            //println!("cargo:rustc-link-lib=dylib=va_win32");

            builder
                //.include("./vaapi-windows/DirectX-Headers-1.0/include")
                //.include(path::PathBuf::from("./vaapi-windows/x64/include"))
                //.file("vaapi-windows/DirectX-Headers-1.0/src/dxguids.cpp")
                //.file("src/vaapi/vaapi_display_win32.cpp")
                //.file("src/vaapi/vaapi_h264_encoder_wrapper.cpp")
                //.file("src/vaapi/vaapi_encoder_factory.cpp")
                //.file("src/vaapi/h264_encoder_impl.cpp")
                .flag("/std:c++20")
                //.flag("/wd4819")
                //.flag("/wd4068")
                .flag("/EHsc");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=rt");
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=m");

            // In order to avoid any ABI mismatches we use the sysroot's headers.
            add_gio_headers(&mut builder);

            for lib_name in ["glib-2.0", "gobject-2.0", "gio-2.0"] {
                pkg_config::probe_library(lib_name).unwrap();
            }

            add_lazy_load_so(
                &mut builder,
                "desktop_capturer",
                ["drm", "gbm", "X11", "Xfixes", "Xdamage", "Xrandr", "Xcomposite", "Xext"]
                    .map(String::from)
                    .to_vec(),
            );

            let x86 = target_arch == "x86_64" || target_arch == "i686";
            let arm = target_arch == "aarch64" || target_arch.contains("arm");

            if x86 {
                if let Some(libva_include) = pkg_config::get_variable("libva", "includedir").ok() {
                    // Do not use pkg_config::probe_library because libva is dlopened
                    // and pkg_config::probe_library would link it.
                    builder
                        .include(libva_include)
                        .file("src/vaapi/vaapi_display_drm.cpp")
                        .file("src/vaapi/vaapi_h264_encoder_wrapper.cpp")
                        .file("src/vaapi/vaapi_encoder_factory.cpp")
                        .file("src/vaapi/h264_encoder_impl.cpp")
                        .flag("-DUSE_VAAPI_VIDEO_CODEC=1");

                    add_lazy_load_so(
                        &mut builder,
                        "vaapi",
                        ["va", "va-drm"].map(String::from).to_vec(),
                    );
                } else {
                    println!("cargo:warning=libva not found; building without hardware accelerated video codecs");
                }
            }

            if x86 || arm {
                let cuda_home = PathBuf::from(match env::var("CUDA_HOME") {
                    Ok(p) => p,
                    Err(_) => "/usr/local/cuda".to_owned(),
                });
                let cuda_include_dir = cuda_home.join("include");

                // libcuda and libnvcuvid are dlopened, so do not link them.
                if cuda_include_dir.join("cuda.h").exists() {
                    builder
                        .include(cuda_include_dir)
                        .flag("-Isrc/nvidia/NvCodec/include")
                        .flag("-Isrc/nvidia/NvCodec/NvCodec")
                        .file("src/nvidia/NvCodec/NvCodec/NvDecoder/NvDecoder.cpp")
                        .file("src/nvidia/NvCodec/NvCodec/NvEncoder/NvEncoder.cpp")
                        .file("src/nvidia/NvCodec/NvCodec/NvEncoder/NvEncoderCuda.cpp")
                        .file("src/nvidia/h264_encoder_impl.cpp")
                        .file("src/nvidia/h265_encoder_impl.cpp")
                        .file("src/nvidia/h264_decoder_impl.cpp")
                        .file("src/nvidia/h265_decoder_impl.cpp")
                        .file("src/nvidia/nvidia_decoder_factory.cpp")
                        .file("src/nvidia/nvidia_encoder_factory.cpp")
                        .file("src/nvidia/cuda_context.cpp")
                        .flag("-Wno-deprecated-declarations")
                        .flag("-DUSE_NVIDIA_VIDEO_CODEC=1");

                    add_lazy_load_so(
                        &mut builder,
                        "nvidia",
                        ["cuda", "nvcuvid"].map(String::from).to_vec(),
                    );
                } else {
                    println!("cargo:warning=cuda.h not found; building without hardware accelerated video codec support for NVidia GPUs");
                }
            }

            builder
                .flag("-Wno-changes-meaning")
                .flag("-Wno-deprecated-declarations")
                .flag("-std=c++20");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=Appkit");
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=VideoToolbox");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalKit");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=IOSurface");
            println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");

            configure_darwin_sysroot(&mut builder);

            builder
                .file("src/objc_video_factory.mm")
                .file("src/objc_video_frame_buffer.mm")
                .flag("-stdlib=libc++")
                .flag("-std=c++20")
                .flag("-Wno-nullability-completeness");
        }
        "ios" => {
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=UIKit");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=VideoToolbox");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=OpenGLES");
            println!("cargo:rustc-link-lib=framework=GLKit");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalKit");
            println!("cargo:rustc-link-lib=framework=Network");
            println!("cargo:rustc-link-lib=framework=QuartzCore");

            configure_darwin_sysroot(&mut builder);

            builder
                .file("src/objc_video_factory.mm")
                .file("src/objc_video_frame_buffer.mm")
                .flag("-std=c++20");
        }
        "android" => {
            webrtc_sys_build::configure_jni_symbols().unwrap();

            println!("cargo:rustc-link-lib=EGL");
            println!("cargo:rustc-link-lib=OpenSLES");
            println!("cargo:rustc-link-lib=c++_static");
            println!("cargo:rustc-link-lib=c++abi");

            configure_android_sysroot(&mut builder);
            builder.file("src/android.cpp").flag("-std=c++20");
        }
        _ => {
            panic!("Unsupported target, {}", target_os);
        }
    }

    // TODO(theomonnom) Only add this define when building tests
    builder.define("LIVEKIT_TEST", None);
    builder.warnings(false).compile("webrtcsys-cxx");

    for entry in glob::glob("./src/**/*.cpp").unwrap() {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }

    for entry in glob::glob("./src/**/*.mm").unwrap() {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }

    for entry in glob::glob("./include/**/*.h").unwrap() {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }

    if target_os.as_str() == "android" {
        copy_libwebrtc_jar(&PathBuf::from(Path::new(&webrtc_dir)));
    }
}

fn copy_libwebrtc_jar(webrtc_dir: &PathBuf) {
    let jar_path = webrtc_dir.join("libwebrtc.jar");
    let output_path = get_output_path();
    let output_jar_path = output_path.join("libwebrtc.jar");
    let res = std::fs::copy(jar_path, output_jar_path);
    if let Err(e) = res {
        println!("Failed to copy libwebrtc.jar: {}", e);
    }
}

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let build_target = env::var("TARGET").unwrap();
    let path =
        Path::new(&manifest_dir_string).join("../target").join(build_target).join(build_type);
    return PathBuf::from(path);
}

fn configure_darwin_sysroot(builder: &mut cc::Build) {
    let target_os = webrtc_sys_build::target_os();

    let sdk = match target_os.as_str() {
        "mac" => "macosx",
        "ios-device" => "iphoneos",
        "ios-simulator" => "iphonesimulator",
        _ => panic!("Unsupported target_os: {}", target_os),
    };

    let clang_rt = match target_os.as_str() {
        "mac" => "clang_rt.osx",
        "ios-device" => "clang_rt.ios",
        "ios-simulator" => "clang_rt.iossim",
        _ => panic!("Unsupported target_os: {}", target_os),
    };

    println!("cargo:rustc-link-lib={}", clang_rt);
    println!("cargo:rustc-link-arg=-ObjC");

    let sysroot = Command::new("xcrun").args(["--sdk", sdk, "--show-sdk-path"]).output().unwrap();

    let sysroot = String::from_utf8_lossy(&sysroot.stdout);
    let sysroot = sysroot.trim();

    let search_dirs = Command::new("cc").arg("--print-search-dirs").output().unwrap();

    let search_dirs = String::from_utf8_lossy(&search_dirs.stdout);
    for line in search_dirs.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1).unwrap();
            let path = format!("{}/lib/darwin", path);
            println!("cargo:rustc-link-search={}", path);
        }
    }

    builder.flag(format!("-isysroot{}", sysroot).as_str());
}

fn configure_android_sysroot(builder: &mut cc::Build) {
    let toolchain = webrtc_sys_build::android_ndk_toolchain().unwrap();
    let sysroot = toolchain.join("sysroot").canonicalize().unwrap();
    builder.flag(format!("-isysroot{}", sysroot.display()).as_str());
}

fn add_lazy_load_so(builder: &mut cc::Build, name: &str, libraries: Vec<String>) {
    let target_arch = webrtc_sys_build::target_arch();
    for lib_name in libraries {
        let mut arch_dir = "x86_64-linux-gnu";
        if target_arch.contains("arm64") {
            arch_dir = "aarch64-linux-gnu";
        }
        let implib_file_c_name = "src/lazy_load_deps_for/".to_owned()
            + name
            + "/"
            + arch_dir
            + "/lib"
            + &lib_name
            + ".so.init.c";
        let implib_file_asm_name = "src/lazy_load_deps_for/".to_owned()
            + name
            + "/"
            + arch_dir
            + "/lib"
            + &lib_name
            + ".so.tramp.S";
        builder.file(implib_file_c_name).file(implib_file_asm_name);
    }
}

fn add_gio_headers(builder: &mut cc::Build) {
    let webrtc_dir = webrtc_sys_build::webrtc_dir();
    let target_arch = webrtc_sys_build::target_arch();
    let target_arch_sysroot = match target_arch.as_str() {
        "arm64" => "arm64",
        "x64" => "amd64",
        _ => panic!("unsupported arch"),
    };
    let sysroot_path = format!("include/build/linux/debian_bullseye_{target_arch_sysroot}-sysroot");
    let sysroot = webrtc_dir.join(sysroot_path);
    let glib_path = sysroot.join("usr/include/glib-2.0");
    println!("cargo:info=add_gio_headers {}", glib_path.display());

    builder.include(&glib_path);
    let arch_specific_path = match target_arch.as_str() {
        "x64" => "x86_64-linux-gnu",
        "arm64" => "aarch64-linux-gnu",
        _ => panic!("unsupported target"),
    };

    let glib_path_config = sysroot.join("usr/lib");
    let glib_path_config = glib_path_config.join(arch_specific_path);
    let glib_path_config = glib_path_config.join("glib-2.0/include");
    builder.include(&glib_path_config);
}
