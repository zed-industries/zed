use std::env;

fn main() {
    if env::var_os("ZTRACING").is_some() {
        println!(r"cargo::rustc-cfg=ztracing");
    }
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=ZTRACING");
}
