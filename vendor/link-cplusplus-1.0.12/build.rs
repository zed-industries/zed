use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let libstdcxx = cfg!(feature = "libstdc++");
    let libcxx = cfg!(feature = "libc++");
    let nothing = cfg!(feature = "nothing");

    if nothing {
        return;
    }

    if libstdcxx && libcxx {
        println!(
            "cargo:warning=-lstdc++ and -lc++ are both requested, \
             using the platform's default"
        );
    }

    match (libstdcxx, libcxx) {
        (true, false) => println!("cargo:rustc-link-lib=stdc++"),
        (false, true) => println!("cargo:rustc-link-lib=c++"),
        (false, false) | (true, true) => {
            // The platform's default.
            let manifest_dir =
                env::var_os("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR");
            let path = PathBuf::from(manifest_dir).join("src").join("dummy.cc");
            cc::Build::new()
                .cpp(true)
                .file(&path)
                .compile("link-cplusplus");
        }
    }
}
