use std::{
  env, path::PathBuf,
};

fn main() {
  println!(
    "cargo:rustc-env=EXAMPLES_PATH={}",
    PathBuf::from(env::var("OUT_DIR").unwrap())
    .join("../../../examples")
    .to_str()
    .unwrap()
  );
}
