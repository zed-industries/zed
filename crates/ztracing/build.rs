use std::env;

fn main() {
    if env::var_os("ZTRACING").is_some() {
        println!("cargo::rustc-cfg=ztracing");
    }
    if env::var_os("ZTRACING_WITH_MEMORY").is_some() {
        println!("cargo::rustc-cfg=ztracing");
        println!("cargo::rustc-cfg=ztracing_with_memory");
    }
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=ZTRACING");
    println!("cargo::rerun-if-env-changed=ZTRACING_WITH_MEMORY");
}
