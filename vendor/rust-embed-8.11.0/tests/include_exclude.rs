use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/public/"]
struct AllAssets;

#[test]
fn get_works() {
  assert!(AllAssets::get("index.html").is_some(), "index.html should exist");
  assert!(AllAssets::get("gg.html").is_none(), "gg.html should not exist");
  assert!(AllAssets::get("images/llama.png").is_some(), "llama.png should exist");
  assert!(AllAssets::get("symlinks/main.js").is_some(), "main.js should exist");
  assert_eq!(AllAssets::iter().count(), 7);
}

#[derive(Embed)]
#[folder = "examples/public/"]
#[include = "*.html"]
#[include = "images/*"]
struct IncludeSomeAssets;

#[test]
fn including_some_assets_works() {
  assert!(IncludeSomeAssets::get("index.html").is_some(), "index.html should exist");
  assert!(IncludeSomeAssets::get("main.js").is_none(), "main.js should not exist");
  assert!(IncludeSomeAssets::get("images/llama.png").is_some(), "llama.png should exist");
  assert_eq!(IncludeSomeAssets::iter().count(), 4);
}

#[derive(Embed)]
#[folder = "examples/public/"]
#[exclude = "*.html"]
#[exclude = "images/*"]
struct ExcludeSomeAssets;

#[test]
fn excluding_some_assets_works() {
  assert!(ExcludeSomeAssets::get("index.html").is_none(), "index.html should not exist");
  assert!(ExcludeSomeAssets::get("main.js").is_some(), "main.js should exist");
  assert!(ExcludeSomeAssets::get("symlinks/main.js").is_some(), "main.js symlink should exist");
  assert!(ExcludeSomeAssets::get("images/llama.png").is_none(), "llama.png should not exist");
  assert_eq!(ExcludeSomeAssets::iter().count(), 3);
}

#[derive(Embed)]
#[folder = "examples/public/"]
#[include = "images/*"]
#[exclude = "*.txt"]
struct ExcludePriorityAssets;

#[test]
fn exclude_has_higher_priority() {
  assert!(ExcludePriorityAssets::get("images/doc.txt").is_none(), "doc.txt should not exist");
  assert!(ExcludePriorityAssets::get("images/llama.png").is_some(), "llama.png should exist");
  assert_eq!(ExcludePriorityAssets::iter().count(), 2);
}

#[derive(Embed)]
#[folder = "examples/public/symlinks"]
#[include = "main.js"]
struct IncludeSymlink;

#[test]
fn include_symlink() {
  assert_eq!(IncludeSymlink::iter().count(), 1);
  assert_eq!(IncludeSymlink::iter().next(), Some(std::borrow::Cow::Borrowed("main.js")));
  assert!(IncludeSymlink::get("main.js").is_some())
}
