use std::{ops::Range, path::Path};

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::change_tracking::ChangeTracking;

pub(super) struct Bookmark {
    pub id: usize,
    pub name: String,
    pub notes: String,
    anchor: Anchor,
}

enum Anchor {
    Eof,
    Bytes { start: u64, end: Option<u64> },
    Unavailable,
}

impl Bookmark {
    pub fn new(id: usize, name: String, range: Range<usize>, changes: &ChangeTracking) -> Self {
        let anchor = if let Some(start) = changes.identity_at(range.start) {
            Anchor::Bytes {
                start,
                end: (!range.is_empty())
                    .then(|| changes.identity_at(range.end - 1))
                    .flatten(),
            }
        } else {
            Anchor::Eof
        };
        Self {
            id,
            name,
            notes: String::new(),
            anchor,
        }
    }

    pub fn range(&self, changes: &ChangeTracking, length: usize) -> Option<Range<usize>> {
        self.resolve(length, |identity| changes.offset_for(identity))
    }

    fn saved_range(&self, changes: &ChangeTracking, length: usize) -> Option<Range<usize>> {
        self.resolve(length, |identity| changes.saved_offset_for(identity))
    }

    fn resolve(
        &self,
        length: usize,
        offset_for: impl Fn(u64) -> Option<usize>,
    ) -> Option<Range<usize>> {
        let Anchor::Bytes { start, end } = &self.anchor else {
            return matches!(self.anchor, Anchor::Eof).then_some(length..length);
        };
        let start = offset_for(*start)?;
        let end = if let Some(end) = end {
            offset_for(*end)?.checked_add(1)?
        } else {
            start
        };
        (start <= end && end <= length).then_some(start..end)
    }

    pub fn retarget(&mut self, range: Range<usize>, changes: &ChangeTracking) {
        self.anchor = Self::new(self.id, String::new(), range, changes).anchor;
    }
}

pub(super) const MAX_BOOKMARKS: usize = 256;
pub(super) const MAX_NAME_BYTES: usize = 256;
pub(super) const MAX_NOTE_BYTES: usize = 16 * 1024;
pub(super) const METADATA_NAMESPACE: &str = "hex-editor-annotations-v1";

pub(super) fn fingerprint(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn metadata_key(path: &Path) -> String {
    fingerprint(path.as_os_str().as_encoded_bytes())
}

#[derive(Serialize, Deserialize)]
pub(super) struct Metadata {
    version: u8,
    fingerprint: String,
    entries: Vec<StoredBookmark>,
}

#[derive(Serialize, Deserialize)]
struct StoredBookmark {
    name: String,
    notes: String,
    range: Option<Range<usize>>,
}

impl Metadata {
    pub fn capture(
        bookmarks: &[Bookmark],
        changes: &ChangeTracking,
        saved_length: usize,
        fingerprint: &str,
    ) -> Self {
        Self {
            version: 1,
            fingerprint: fingerprint.to_owned(),
            entries: bookmarks
                .iter()
                .map(|bookmark| StoredBookmark {
                    name: bookmark.name.clone(),
                    notes: bookmark.notes.clone(),
                    range: bookmark.saved_range(changes, saved_length),
                })
                .collect(),
        }
    }

    pub fn decode(json: &str) -> Result<Self> {
        ensure!(
            json.len() <= 32 * 1024 * 1024,
            "Annotation metadata is too large"
        );
        let metadata: Self = serde_json::from_str(json)?;
        ensure!(
            metadata.version == 1,
            "Unsupported annotation metadata version"
        );
        ensure!(
            metadata.entries.len() <= MAX_BOOKMARKS,
            "Too many stored annotations"
        );
        for entry in &metadata.entries {
            validate(&entry.name, &entry.notes)?;
        }
        Ok(metadata)
    }

    pub fn restore(
        &self,
        changes: &ChangeTracking,
        length: usize,
        fingerprint: &str,
    ) -> Vec<Bookmark> {
        self.entries
            .iter()
            .enumerate()
            .map(|(id, entry)| {
                let range = entry.range.clone().filter(|range| {
                    self.fingerprint == fingerprint
                        && range.start <= range.end
                        && range.end <= length
                });
                let mut bookmark = if let Some(range) = range {
                    Bookmark::new(id, entry.name.clone(), range, changes)
                } else {
                    Bookmark {
                        id,
                        name: entry.name.clone(),
                        notes: String::new(),
                        anchor: Anchor::Unavailable,
                    }
                };
                bookmark.notes = entry.notes.clone();
                bookmark
            })
            .collect()
    }
}

pub(super) fn validate(name: &str, notes: &str) -> Result<()> {
    ensure!(
        !name.trim().is_empty() && name.len() <= MAX_NAME_BYTES,
        "Enter a bookmark name up to 256 bytes"
    );
    ensure!(
        notes.len() <= MAX_NOTE_BYTES,
        "Keep annotation notes within 16 KiB"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_saved_anchors_and_changed_file() -> Result<()> {
        assert!(validate("", "note").is_err());
        assert!(validate(&"x".repeat(MAX_NAME_BYTES + 1), "note").is_err());
        assert!(validate("valid", &"x".repeat(MAX_NOTE_BYTES + 1)).is_err());
        validate("valid", &"x".repeat(MAX_NOTE_BYTES))?;
        let mut changes = ChangeTracking::new(4);
        let mut bookmark = Bookmark::new(0, "header".into(), 1..3, &changes);
        bookmark.notes = "Length field\nUnsigned, little endian".into();
        let inserted = changes.replacement(0..0, 2)?;
        changes.splice(0..0, &inserted);
        let digest = fingerprint(b"abcd");
        let metadata = Metadata::capture(&[bookmark], &changes, 4, &digest);
        let json = serde_json::to_string(&metadata)?;
        let metadata = Metadata::decode(&json)?;
        let restored = metadata.restore(&ChangeTracking::new(4), 4, &digest);
        assert_eq!(restored[0].range(&ChangeTracking::new(4), 4), Some(1..3));
        assert!(restored[0].notes.contains('\n'));
        let restored = metadata.restore(&ChangeTracking::new(4), 4, &fingerprint(b"wxyz"));
        assert!(restored[0].range(&ChangeTracking::new(4), 4).is_none());
        assert!(restored[0].notes.contains("Length"));
        assert!(Metadata::decode(r#"{"version":2,"fingerprint":"","entries":[]}"#).is_err());
        assert!(Metadata::decode("not json").is_err());
        Ok(())
    }

    #[test]
    fn test_metadata_limits_escaped_notes_eof_and_deleted_targets() -> Result<()> {
        let mut changes = ChangeTracking::new(2);
        let entries: Vec<_> = (0..MAX_BOOKMARKS)
            .map(|id| {
                let mut entry = Bookmark::new(id, format!("note {id}"), 2..2, &changes);
                entry.notes = "\0".repeat(MAX_NOTE_BYTES);
                entry
            })
            .collect();
        let digest = fingerprint(&[1, 2]);
        let metadata = Metadata::capture(&entries, &changes, 2, &digest);
        let json = serde_json::to_string(&metadata)?;
        let metadata = Metadata::decode(&json)?;
        assert_eq!(
            metadata.restore(&changes, 2, &digest)[0].range(&changes, 2),
            Some(2..2)
        );
        let deleted = Bookmark::new(0, "removed".into(), 0..1, &changes);
        changes.splice(0..1, &[]);
        changes.mark_saved();
        let digest = fingerprint(&[2]);
        let metadata = Metadata::capture(&[deleted], &changes, 1, &digest);
        let restored = metadata.restore(&ChangeTracking::new(1), 1, &digest);
        assert_eq!(restored[0].name, "removed");
        assert!(restored[0].range(&ChangeTracking::new(1), 1).is_none());
        Ok(())
    }
}
