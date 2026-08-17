use std::{path::PathBuf, str::FromStr};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/missing/"]
#[allow_missing = true]
struct Asset;

#[test]
fn missing_is_empty() {
  let path = PathBuf::from_str("./examples/missing").unwrap();
  assert!(!path.exists());
  assert_eq!(Asset::iter().count(), 0);
}
