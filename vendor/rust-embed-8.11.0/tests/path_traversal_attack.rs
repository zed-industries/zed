use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/public/"]
struct Assets;

/// Prevent attempts to access files outside of the embedded folder.
/// This is mainly a concern when running in debug mode, since that loads from
/// the file system at runtime.
#[test]
fn path_traversal_attack_fails() {
  assert!(Assets::get("../basic.rs").is_none());
}

#[derive(Embed)]
#[folder = "examples/axum-spa/"]
struct AxumAssets;

// TODO:
/// Prevent attempts to access symlinks outside of the embedded folder.
/// This is mainly a concern when running in debug mode, since that loads from
/// the file system at runtime.
#[test]
#[ignore = "see https://github.com/pyrossh/rust-embed/pull/235"]
fn path_traversal_attack_symlink_fails() {
  assert!(Assets::get("../public/symlinks/main.js").is_none());
}
