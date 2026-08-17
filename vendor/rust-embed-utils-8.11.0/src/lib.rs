#![forbid(unsafe_code)]

use sha2::Digest;
use std::borrow::Cow;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, io};

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub struct FileEntry {
  pub rel_path: String,
  pub full_canonical_path: String,
}

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub fn get_files(folder_path: String, matcher: PathMatcher) -> impl Iterator<Item = FileEntry> {
  walkdir::WalkDir::new(&folder_path)
    .follow_links(true)
    .sort_by_file_name()
    .into_iter()
    .filter_map(std::result::Result::ok)
    .filter(|e| e.file_type().is_file())
    .filter_map(move |e| {
      let rel_path = path_to_str(e.path().strip_prefix(&folder_path).unwrap());
      let full_canonical_path = path_to_str(std::fs::canonicalize(e.path()).ok()?);

      let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
        rel_path.replace('\\', "/")
      } else {
        rel_path
      };
      if matcher.is_path_included(&rel_path) {
        Some(FileEntry { rel_path, full_canonical_path })
      } else {
        None
      }
    })
}

/// A file embedded into the binary
#[derive(Clone)]
pub struct EmbeddedFile {
  pub data: Cow<'static, [u8]>,
  pub metadata: Metadata,
}

/// Metadata about an embedded file
#[derive(Clone)]
pub struct Metadata {
  hash: [u8; 32],
  last_modified: Option<u64>,
  created: Option<u64>,
  #[cfg(feature = "mime-guess")]
  mimetype: Cow<'static, str>,
}

impl Metadata {
  #[doc(hidden)]
  pub const fn __rust_embed_new(
    hash: [u8; 32], last_modified: Option<u64>, created: Option<u64>, #[cfg(feature = "mime-guess")] mimetype: &'static str,
  ) -> Self {
    Self {
      hash,
      last_modified,
      created,
      #[cfg(feature = "mime-guess")]
      mimetype: Cow::Borrowed(mimetype),
    }
  }

  /// The SHA256 hash of the file
  pub fn sha256_hash(&self) -> [u8; 32] {
    self.hash
  }

  /// The last modified date in seconds since the UNIX epoch. If the underlying
  /// platform/file-system does not support this, None is returned.
  pub fn last_modified(&self) -> Option<u64> {
    self.last_modified
  }

  /// The created data in seconds since the UNIX epoch. If the underlying
  /// platform/file-system does not support this, None is returned.
  pub fn created(&self) -> Option<u64> {
    self.created
  }

  /// The mime type of the file
  #[cfg(feature = "mime-guess")]
  pub fn mimetype(&self) -> &str {
    &self.mimetype
  }
}

pub fn read_file_from_fs(file_path: &Path) -> io::Result<EmbeddedFile> {
  let data = fs::read(file_path)?;
  let data = Cow::from(data);

  let mut hasher = sha2::Sha256::new();
  hasher.update(&data);
  let hash: [u8; 32] = hasher.finalize().into();

  let source_date_epoch = match std::env::var("SOURCE_DATE_EPOCH") {
    Ok(value) => value.parse::<u64>().ok(),
    Err(_) => None,
  };

  let metadata = fs::metadata(file_path)?;
  let last_modified = metadata
    .modified()
    .ok()
    .and_then(|modified| modified.duration_since(SystemTime::UNIX_EPOCH).ok())
    .map(|secs| secs.as_secs());

  let created = metadata
    .created()
    .ok()
    .and_then(|created| created.duration_since(SystemTime::UNIX_EPOCH).ok())
    .map(|secs| secs.as_secs());

  #[cfg(feature = "mime-guess")]
  let mimetype = mime_guess::from_path(file_path).first_or_octet_stream().to_string();

  Ok(EmbeddedFile {
    data,
    metadata: Metadata {
      hash,
      last_modified: source_date_epoch.or(last_modified),
      created: source_date_epoch.or(created),
      #[cfg(feature = "mime-guess")]
      mimetype: mimetype.into(),
    },
  })
}

fn path_to_str<P: AsRef<std::path::Path>>(p: P) -> String {
  p.as_ref().to_str().expect("Path does not have a string representation").to_owned()
}

#[derive(Clone)]
pub struct PathMatcher {
  #[cfg(feature = "include-exclude")]
  include_matcher: globset::GlobSet,
  #[cfg(feature = "include-exclude")]
  exclude_matcher: globset::GlobSet,
}

#[cfg(feature = "include-exclude")]
impl PathMatcher {
  pub fn new(includes: &[&str], excludes: &[&str]) -> Self {
    let mut include_matcher = globset::GlobSetBuilder::new();
    for include in includes {
      include_matcher.add(globset::Glob::new(include).unwrap_or_else(|_| panic!("invalid include pattern '{}'", include)));
    }
    let include_matcher = include_matcher
      .build()
      .unwrap_or_else(|_| panic!("Could not compile included patterns matcher"));

    let mut exclude_matcher = globset::GlobSetBuilder::new();
    for exclude in excludes {
      exclude_matcher.add(globset::Glob::new(exclude).unwrap_or_else(|_| panic!("invalid exclude pattern '{}'", exclude)));
    }
    let exclude_matcher = exclude_matcher
      .build()
      .unwrap_or_else(|_| panic!("Could not compile excluded patterns matcher"));

    Self {
      include_matcher,
      exclude_matcher,
    }
  }
  pub fn is_path_included(&self, path: &str) -> bool {
    !self.exclude_matcher.is_match(path) && (self.include_matcher.is_empty() || self.include_matcher.is_match(path))
  }
}

#[cfg(not(feature = "include-exclude"))]
impl PathMatcher {
  pub fn new(_includes: &[&str], _excludes: &[&str]) -> Self {
    Self {}
  }
  pub fn is_path_included(&self, _path: &str) -> bool {
    true
  }
}
