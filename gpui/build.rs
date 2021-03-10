use std::{env, path::PathBuf};

fn main() {
    generate_dispatch_bindings();
    compile_context_predicate_parser();
}

fn generate_dispatch_bindings() {
    println!("cargo:rustc-link-lib=framework=System");
    println!("cargo:rerun-if-changed=src/platform/mac/dispatch.h");

    let bindings = bindgen::Builder::default()
        .header("src/platform/mac/dispatch.h")
        .whitelist_var("_dispatch_main_q")
        .whitelist_function("dispatch_async_f")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dispatch_sys.rs"))
        .expect("couldn't write bindings");
}

fn compile_context_predicate_parser() {
    let dir = PathBuf::from("./grammars/context-predicate/src");
    let parser_c = dir.join("parser.c");

    println!("cargo:rerun-if-changed={}", &parser_c.to_str().unwrap());
    cc::Build::new()
        .include(&dir)
        .file(parser_c)
        .compile("tree_sitter_context_predicate");
}
