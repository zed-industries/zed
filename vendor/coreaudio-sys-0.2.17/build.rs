extern crate bindgen;

fn sdk_path(target: &str) -> Result<String, std::io::Error> {
    // Use environment variable if set
    println!("cargo:rerun-if-env-changed=COREAUDIO_SDK_PATH");
    if let Ok(path) = std::env::var("COREAUDIO_SDK_PATH") {
        return Ok(path);
    }

    use std::process::Command;

    let sdk = if target.contains("apple-darwin") {
        "macosx"
    } else if target == "x86_64-apple-ios"
        || target == "i386-apple-ios"
        || target == "aarch64-apple-ios-sim"
    {
        "iphonesimulator"
    } else if target == "aarch64-apple-ios"
        || target == "armv7-apple-ios"
        || target == "armv7s-apple-ios"
    {
        "iphoneos"
    } else {
        unreachable!();
    };
    let output = Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

fn build(sdk_path: Option<&str>, target: &str) {
    // Generate one large set of bindings for all frameworks.
    //
    // We do this rather than generating a module per framework as some frameworks depend on other
    // frameworks and in turn share types. To ensure all types are compatible across each
    // framework, we feed all headers to bindgen at once.
    //
    // Only link to each framework and include their headers if their features are enabled and they
    // are available on the target os.

    use std::env;
    use std::path::PathBuf;

    let mut headers: Vec<&'static str> = vec![];

    #[cfg(feature = "audio_unit")]
    {
        // Since iOS 10.0 and macOS 10.12, all the functionality in AudioUnit
        // moved to AudioToolbox, and the AudioUnit headers have been simple
        // wrappers ever since.
        if target.contains("apple-ios") {
            // On iOS, the AudioUnit framework does not have (and never had) an
            // actual dylib to link to, it is just a few header files.
            // The AudioToolbox framework contains the symbols instead.
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
        } else {
            // On macOS, the symbols are present in the AudioToolbox framework,
            // but only on macOS 10.12 and above.
            //
            // However, unlike on iOS, the AudioUnit framework on macOS
            // contains a dylib with the desired symbols, that we can link to
            // (in later versions just re-exports from AudioToolbox).
            println!("cargo:rustc-link-lib=framework=AudioUnit");
        }
        headers.push("AudioUnit/AudioUnit.h");
    }

    #[cfg(feature = "audio_toolbox")]
    {
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        headers.push("AudioToolbox/AudioToolbox.h");
    }

    #[cfg(feature = "core_audio")]
    {
        println!("cargo:rustc-link-lib=framework=CoreAudio");

        if target.contains("apple-ios") {
            headers.push("CoreAudio/CoreAudioTypes.h");
        } else {
            headers.push("CoreAudio/CoreAudio.h");

            #[cfg(feature = "audio_server_plugin")]
            {
                headers.push("CoreAudio/AudioServerPlugIn.h");
            }
        }
    }

    #[cfg(feature = "io_kit_audio")]
    {
        assert!(target.contains("apple-darwin"));
        println!("cargo:rustc-link-lib=framework=IOKit");
        headers.push("IOKit/audio/IOAudioTypes.h");
    }

    #[cfg(feature = "open_al")]
    {
        println!("cargo:rustc-link-lib=framework=OpenAL");
        headers.push("OpenAL/al.h");
        headers.push("OpenAL/alc.h");
    }

    #[cfg(all(feature = "core_midi"))]
    {
        if target.contains("apple-darwin") {
            println!("cargo:rustc-link-lib=framework=CoreMIDI");
            headers.push("CoreMIDI/CoreMIDI.h");
        }
    }

    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    // Get the cargo out directory.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    // Begin building the bindgen params.
    let mut builder = bindgen::Builder::default();

    builder = builder.size_t_is_usize(true);

    if let Some(sdk_path) = sdk_path {
        builder = builder.clang_args(&["-isysroot", sdk_path]);
    }
    if target.contains("apple-ios") {
        // time.h as has a variable called timezone that conflicts with some of the objective-c
        // calls from NSCalendar.h in the Foundation framework. This removes that one variable.
        builder = builder.blocklist_item("timezone");
        builder = builder.blocklist_item("objc_object");
    }

    // bindgen produces alignment tests that cause undefined behavior in some cases.
    // This seems to happen across all apple target tripples :/.
    // https://github.com/rust-lang/rust-bindgen/issues/1651
    builder = builder.layout_tests(false);

    let meta_header: Vec<_> = headers
        .iter()
        .map(|h| format!("#include <{}>\n", h))
        .collect();

    builder = builder.header_contents("coreaudio.h", &meta_header.concat());

    // Generate the bindings.
    builder = builder.trust_clang_mangling(false).derive_default(true);

    let bindings = builder.generate().expect("unable to generate bindings");

    // Write them to the crate root.
    bindings
        .write_to_file(out_dir.join("coreaudio.rs"))
        .expect("could not write bindings");
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if !(target.contains("apple-darwin") || target.contains("apple-ios")) {
        panic!("coreaudio-sys requires macos or ios target");
    }

    let directory = sdk_path(&target).ok();
    build(directory.as_ref().map(String::as_ref), &target);
}
