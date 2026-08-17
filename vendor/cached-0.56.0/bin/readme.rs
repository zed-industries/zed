// cargo-deps: sha2="0.9"

use std::io::Read;
use sha2::Digest;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut file = std::fs::File::open("README.md").expect("no file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read error");

    let mut hasher = sha2::Sha256::new();
    hasher.update(contents.as_bytes());
    let result = hasher.finalize();
    println!("{:?}", result);
}