use rust_embed::{Embed, EmbeddedFile};
use sha2::Digest;
use std::{fs, time::SystemTime};

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn hash_is_accurate() {
  let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");
  let mut hasher = sha2::Sha256::new();
  hasher.update(index_file.data);
  let expected_hash: [u8; 32] = hasher.finalize().into();

  assert_eq!(index_file.metadata.sha256_hash(), expected_hash);
}

#[test]
#[cfg(not(feature = "deterministic-timestamps"))]
fn last_modified_is_accurate() {
  let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");

  let metadata = fs::metadata(format!("{}/examples/public/index.html", env!("CARGO_MANIFEST_DIR"))).unwrap();
  let expected_datetime_utc = metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

  assert_eq!(index_file.metadata.last_modified(), Some(expected_datetime_utc));
}

#[test]
#[cfg(not(feature = "deterministic-timestamps"))]
fn create_is_accurate() {
  let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");

  let metadata = fs::metadata(format!("{}/examples/public/index.html", env!("CARGO_MANIFEST_DIR"))).unwrap();
  let expected_datetime_utc = metadata.created().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

  assert_eq!(index_file.metadata.created(), Some(expected_datetime_utc));
}

#[test]
#[cfg(feature = "deterministic-timestamps")]
fn deterministic_timestamps_are_zero() {
  let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");

  assert_eq!(
    index_file.metadata.last_modified(),
    Some(0),
    "last_modified should be 0 with deterministic-timestamps"
  );
  assert_eq!(index_file.metadata.created(), Some(0), "created should be 0 with deterministic-timestamps");

  let metadata = fs::metadata(format!("{}/examples/public/index.html", env!("CARGO_MANIFEST_DIR"))).unwrap();
  let fs_modified = metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
  assert_ne!(fs_modified, 0, "Filesystem modified time should not be 0");
}
