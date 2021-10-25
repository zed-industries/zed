use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("cargo:rustc-env=ZED_TARGET={}", target);

    if let Ok(bundled) = env::var("ZED_BUNDLE") {
        println!("cargo:rustc-env=ZED_BUNDLE={}", bundled);
    }
}
