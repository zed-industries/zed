use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("cargo:rustc-env=TARGET={}", target);

    if let Ok(bundled) = env::var("BUNDLE") {
        println!("cargo:rustc-env=BUNDLE={}", bundled);
    }
}
