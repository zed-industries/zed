fn build_and_link_libfuzzer() {
    println!("cargo:rerun-if-env-changed=CUSTOM_LIBFUZZER_PATH");
    if let Ok(custom) = ::std::env::var("CUSTOM_LIBFUZZER_PATH") {
        println!("cargo:rerun-if-changed={custom}");

        let custom_lib_path = ::std::path::PathBuf::from(&custom);
        let custom_lib_dir = custom_lib_path.parent().unwrap().to_string_lossy();

        let custom_lib_name = custom_lib_path.file_stem().unwrap().to_string_lossy();
        let custom_lib_name = custom_lib_name
            .strip_prefix("lib")
            .unwrap_or(custom_lib_name.as_ref());

        println!("cargo:rustc-link-search=native={}", custom_lib_dir);
        println!("cargo:rustc-link-lib=static={}", custom_lib_name);

        match std::env::var("CUSTOM_LIBFUZZER_STD_CXX") {
            // Default behavior for backwards compat.
            Err(_) => println!("cargo:rustc-link-lib=stdc++"),
            Ok(s) if s == "none" => (),
            Ok(s) => println!("cargo:rustc-link-lib={}", s),
        }
    } else {
        let mut build = cc::Build::new();
        let sources = ::std::fs::read_dir("libfuzzer")
            .expect("listable source directory")
            .map(|de| de.expect("file in directory").path())
            .filter(|p| p.extension().map(|ext| ext == "cpp") == Some(true))
            .collect::<Vec<_>>();
        for source in sources.iter() {
            println!("cargo:rerun-if-changed={}", source.display());
            build.file(source.to_str().unwrap());
        }
        build.cpp(true);
        build.std("c++17");
        build.force_frame_pointer(true);
        build.warnings(false);
        build.compile("libfuzzer.a");
    }
}

fn main() {
    if cfg!(feature = "link_libfuzzer") {
        build_and_link_libfuzzer();
    }
}
