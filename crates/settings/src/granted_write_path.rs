use std::path::{Path, PathBuf};

/// A sandbox writable-path grant, paired with the canonical target it was
/// resolved to when it was approved.
///
/// Persisting the resolved canonical path (rather than re-resolving the
/// requested path by string at enforcement time) closes a symlink TOCTOU hole:
/// the path that is actually enforced is the one that was vetted at approval
/// time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrantedWritePath {
    /// The path exactly as the user/model requested it (untrusted; display/provenance).
    pub requested: PathBuf,
    /// The canonical, symlink-resolved target established when the grant was
    /// approved. `None` for a legacy/hand-authored bare-string settings entry,
    /// which is resolved fresh at enforcement time.
    pub resolved: Option<PathBuf>,
    /// Windows/WSL only: whether the canonical target lives on a Windows-hosted
    /// (DrvFs) filesystem, whose sandbox-integrity guarantees are weaker than a
    /// distro-native filesystem. Recorded at approval time (after the user was
    /// shown the weaker-guarantee warning) so later commands can reuse the grant
    /// without re-classifying it. Always `false` on other platforms and for
    /// legacy/bare-string entries.
    pub on_windows_fs: bool,
}

impl GrantedWritePath {
    /// A grant from a bare requested path with no resolved canonical target
    /// (resolved fresh at enforcement time).
    pub fn from_requested(requested: PathBuf) -> Self {
        Self {
            requested,
            resolved: None,
            on_windows_fs: false,
        }
    }

    /// A grant whose canonical target was resolved when it was approved. Assumes
    /// a native (non-Windows-hosted) filesystem; use [`Self::resolved_on_fs`] to
    /// record the Windows/WSL DrvFs classification.
    pub fn resolved(requested: PathBuf, resolved: PathBuf) -> Self {
        Self::resolved_on_fs(requested, resolved, false)
    }

    /// A grant whose canonical target was resolved when it was approved,
    /// recording whether that target lives on a Windows-hosted (DrvFs)
    /// filesystem.
    pub fn resolved_on_fs(requested: PathBuf, resolved: PathBuf, on_windows_fs: bool) -> Self {
        Self {
            requested,
            resolved: Some(resolved),
            on_windows_fs,
        }
    }

    /// The path used for lexical subtree/coverage/dedup logic: the resolved
    /// canonical target when known (the real grant), otherwise the requested
    /// path.
    pub fn canonical_or_requested(&self) -> &Path {
        self.resolved.as_deref().unwrap_or(&self.requested)
    }
}

// Hand-written serde so grants round-trip with legacy persisted data: a bare
// JSON string deserializes to `{ requested, resolved: None }`, and a `None`
// resolved serializes back as a bare string. This mirrors the string-or-object
// approach used by `GrantedWritePathContent` in `settings_content`.
impl serde::Serialize for GrantedWritePath {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.resolved {
            None => self.requested.serialize(serializer),
            Some(resolved) => {
                use serde::ser::SerializeStruct as _;
                // Emit `on_windows_fs` only when set, so native grants (the
                // common case, and every grant on non-Windows platforms) keep
                // the compact two-field form and legacy rows round-trip.
                let field_count = if self.on_windows_fs { 3 } else { 2 };
                let mut state = serializer.serialize_struct("GrantedWritePath", field_count)?;
                state.serialize_field("requested", &self.requested)?;
                state.serialize_field("resolved", resolved)?;
                if self.on_windows_fs {
                    state.serialize_field("on_windows_fs", &self.on_windows_fs)?;
                }
                state.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for GrantedWritePath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Object {
            requested: PathBuf,
            #[serde(default)]
            resolved: Option<PathBuf>,
            #[serde(default)]
            on_windows_fs: bool,
        }

        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum StringOrObject {
            String(PathBuf),
            Object(Object),
        }

        Ok(match StringOrObject::deserialize(deserializer)? {
            StringOrObject::String(requested) => Self {
                requested,
                resolved: None,
                on_windows_fs: false,
            },
            StringOrObject::Object(Object {
                requested,
                resolved,
                on_windows_fs,
            }) => Self {
                requested,
                resolved,
                on_windows_fs,
            },
        })
    }
}
