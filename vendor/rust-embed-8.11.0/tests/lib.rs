use rust_embed::{Embed, RustEmbed};

/// Test doc comment
#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct AssetOld;

#[test]
fn get_works() {
  assert!(Asset::get("index.html").is_some(), "index.html should exist");
  assert!(Asset::get("gg.html").is_none(), "gg.html should not exist");
  assert!(Asset::get("images/llama.png").is_some(), "llama.png should exist");
}

// Todo remove this test and rename RustEmbed trait to Embed on a new major release
#[test]
fn get_old_name_works() {
  assert!(AssetOld::get("index.html").is_some(), "index.html should exist");
  assert!(AssetOld::get("gg.html").is_none(), "gg.html should not exist");
  assert!(AssetOld::get("images/llama.png").is_some(), "llama.png should exist");
}

/// Using Windows-style path separators (`\`) is acceptable
#[test]
fn get_windows_style() {
  assert!(
    Asset::get("images\\llama.png").is_some(),
    "llama.png should be accessible via \"images\\lama.png\""
  );
}

#[test]
fn iter_works() {
  let mut num_files = 0;
  for file in Asset::iter() {
    assert!(Asset::get(file.as_ref()).is_some());
    num_files += 1;
  }
  assert_eq!(num_files, 7);
}

#[test]
fn trait_works_generic() {
  trait_works_generic_helper::<Asset>();
}
fn trait_works_generic_helper<E: rust_embed::Embed>() {
  let mut num_files = 0;
  for file in E::iter() {
    assert!(E::get(file.as_ref()).is_some());
    num_files += 1;
  }
  assert_eq!(num_files, 7);
  assert!(E::get("gg.html").is_none(), "gg.html should not exist");
}

/// Test that iter() can be boxed into a trait object.
/// This is required for libraries like i18n-embed that need to return
/// `Box<dyn Iterator<Item = String>>` from a trait implementation.
#[test]
fn iter_can_be_boxed() {
  fn get_boxed_iter<E: rust_embed::Embed>() -> Box<dyn Iterator<Item = String>> {
    Box::new(E::iter().map(|filename| filename.to_string()))
  }

  let filenames: Vec<String> = get_boxed_iter::<Asset>().collect();
  assert_eq!(filenames.len(), 7);
  assert!(filenames.contains(&"index.html".to_string()));
}
