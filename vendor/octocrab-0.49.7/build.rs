use cargo_metadata::MetadataCommand;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap(); // Cargo.toml path
    let manifest_path = Path::new(&manifest_dir).join("Cargo.toml");

    let metadata = MetadataCommand::new()
        .manifest_path(&manifest_path)
        .no_deps()
        .exec()
        .expect("failed to parse `cargo metadata`");

    let root_package = metadata.root_package().unwrap();
    let custom_metadata = &root_package.metadata;

    let set_headers_array = custom_metadata["github-api"]["request-headers"]
        .as_array()
        .cloned()
        .unwrap_or(Default::default());
    let set_headers_array = set_headers_array
        .iter()
        .map(|x| x.as_str().unwrap_or_default())
        .map(|x| {
            // split either by ":" or by ": "
            x.split_once(": ")
                .unwrap_or_else(|| x.split_once(":").unwrap_or_default())
        })
        .collect::<Vec<(&str, &str)>>();
    let array_creation_static = format!(
        "pub const _SET_HEADERS_MAP: [(&str, &str); {}] = {:?};",
        &set_headers_array.len(),
        &set_headers_array
    );

    let out_dir = env::var("OUT_DIR").unwrap(); // build's OUT path
    let dest_path = Path::new(&out_dir).join("headers_metadata.rs");
    fs::write(&dest_path, array_creation_static)
        .unwrap_or_else(|_| panic!("failed to write {}", dest_path.display()));
}
