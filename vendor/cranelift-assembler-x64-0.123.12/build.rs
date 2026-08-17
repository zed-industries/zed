use cranelift_assembler_x64_meta as meta;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").expect("The OUT_DIR environment variable must be set");
    let out_dir = Path::new(&out_dir);
    let built_files = [meta::generate_rust_assembler(out_dir, "assembler.rs")];

    // Generating this additional bit of Rust is necessary for listing the
    // generated files.
    let mut vec_of_built_files = File::create(out_dir.join("generated-files.rs")).unwrap();
    writeln!(vec_of_built_files, "vec![").unwrap();
    for file in &built_files {
        writeln!(vec_of_built_files, "  {:?}.into(),", file.display()).unwrap();
    }
    writeln!(vec_of_built_files, "]").unwrap();
}
