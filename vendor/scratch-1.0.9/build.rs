use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let _ = fs::remove_dir_all(&out_dir);
    let _ = fs::create_dir(&out_dir);
}
