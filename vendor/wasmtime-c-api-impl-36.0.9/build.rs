use std::env;
use std::process::Command;

// WASMTIME_FEATURE_LIST
const FEATURES: &[&str] = &[
    "ASYNC",
    "PROFILING",
    "CACHE",
    "PARALLEL_COMPILATION",
    "WASI",
    "LOGGING",
    "DISABLE_LOGGING",
    "COREDUMP",
    "ADDR2LINE",
    "DEMANGLE",
    "THREADS",
    "GC",
    "GC_DRC",
    "GC_NULL",
    "CRANELIFT",
    "WINCH",
    "DEBUG_BUILTINS",
    "WAT",
    "POOLING_ALLOCATOR",
    "COMPONENT_MODEL",
];
// ... if you add a line above this be sure to change the other locations
// marked WASMTIME_FEATURE_LIST

fn main() {
    println!("cargo:rerun-if-changed=cmake/features.cmake");
    println!("cargo:rerun-if-changed=cmake/install-headers.cmake");
    println!("cargo:rerun-if-changed=include");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut cmake = Command::new("cmake");
    cmake.arg("-DWASMTIME_DISABLE_ALL_FEATURES=ON");
    cmake.arg(format!("-DCMAKE_INSTALL_PREFIX={out_dir}"));
    for f in FEATURES {
        if env::var_os(format!("CARGO_FEATURE_{f}")).is_some() {
            cmake.arg(format!("-DWASMTIME_FEATURE_{f}=ON"));
        }
    }

    cmake.arg("-P").arg("cmake/install-headers.cmake");

    let status = cmake.status().expect("failed to spawn `cmake`");
    assert!(status.success());

    println!("cargo:include={out_dir}/include");
}
