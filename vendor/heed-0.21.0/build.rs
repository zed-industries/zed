use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(master3)");

    let pkgname = env::var("CARGO_PKG_NAME").expect("Cargo didn't set the CARGO_PKG_NAME env var!");
    match pkgname.as_str() {
        "heed" => (),
        "heed3" => println!("cargo:rustc-cfg=master3"),
        _ => panic!("unexpected package name!"),
    }
}
