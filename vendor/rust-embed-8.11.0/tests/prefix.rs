use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/public/"]
#[prefix = "prefix/"]
struct Asset;

#[test]
fn get_with_prefix() {
  assert!(Asset::get("prefix/index.html").is_some());
}

#[test]
fn get_without_prefix() {
  assert!(Asset::get("index.html").is_none());
}

#[test]
fn iter_values_have_prefix() {
  for file in Asset::iter() {
    assert!(file.starts_with("prefix/"));
    assert!(Asset::get(file.as_ref()).is_some());
  }
}
