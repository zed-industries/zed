fn main() {
    let block_dir = std::path::Path::new("tree-sitter-markdown").join("src");
    let inline_dir = std::path::Path::new("tree-sitter-markdown-inline").join("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(&block_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    for path in &[
        block_dir.join("parser.c"),
        block_dir.join("scanner.c"),
        inline_dir.join("parser.c"),
        inline_dir.join("scanner.c"),
    ] {
        c_config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    c_config.compile("tree-sitter-markdown");
}
