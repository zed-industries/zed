//! Print the path to the generated code.

fn main() {
    let paths: Vec<std::path::PathBuf> = include!(concat!(env!("OUT_DIR"), "/generated-files.rs"));
    for path in paths {
        println!("{}", path.display());
    }
}
