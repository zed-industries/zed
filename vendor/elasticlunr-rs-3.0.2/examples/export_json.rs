use elasticlunr::Index;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut index = Index::new(&["title", "body"]);
    index.add_doc(
        "1",
        &[
            "This Week in Rust 207",
            "Hello and welcome to another issue of This Week in Rust!",
        ],
    );
    index.add_doc(
        "2",
        &[
            "This Week in Rust 206",
            "Hello and welcome to another issue of This Week in Rust!",
        ],
    );
    let mut file = File::create("examples/out.json").unwrap();
    file.write_all(index.to_json_pretty().as_bytes()).unwrap();
}
