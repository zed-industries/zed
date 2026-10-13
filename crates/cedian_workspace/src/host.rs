//! `WorkspaceHost`: the editor boundary (plan §12 trait shape).
//!
//! Text snapshots of the files the caller opened: the app feeds Zed's
//! buffers, the headless CLI the disk. Selection and active file are
//! caller-provided. OMP edits with its own tools (ADR-0057 decision 6).
//!
//! Threading: the texts live behind `parking_lot::Mutex`; host-tool handler
//! threads (vendored client) lock briefly per call — no await, no UI thread.

use omp_rpc::HostUri;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// What a code-intelligence read answers without Zed's project.
pub const HEADLESS_LSP: &str = "served by the cedian app from Zed's language servers; \
     headless, use OMP's own lsp tool";

/// Normalize any key-shaped path to canonical `/rel` form (leading `/`
/// enforced, `.` components kept — keys never touch disk by themselves).
fn normalize_key(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    let stripped = s.strip_prefix('/').unwrap_or(&s);
    PathBuf::from(format!("/{stripped}"))
}

/// One published diagnostic (headless subset of LSP `Diagnostic`; the fork
/// binding converts real `lsp::Diagnostic` into this shape).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Buffer path.
    pub path: PathBuf,
    /// 0-based line.
    pub line: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
}

/// Diagnostic severity (LSP subset).
/// Ordered most severe first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

/// Editor boundary: active file, selection, buffer text, diagnostics.
pub trait WorkspaceHost: Send + Sync {
    /// File the user is editing, if any.
    fn active_file(&self) -> Option<PathBuf>;
    /// Current selection as `(path, start, end)` byte offsets, if any.
    fn selection(&self) -> Option<(PathBuf, usize, usize)>;
    /// Read buffer text (agents see buffers, not just the filesystem — §13).
    fn read_buffer(&self, path: &Path) -> Option<String>;
    /// Published diagnostics for one buffer (fork: real LSP state).
    fn diagnostics(&self, path: &Path) -> Vec<Diagnostic>;
}

/// Host: opened texts + caller-set editor chrome + published
/// diagnostics (headless stand-in for Zed `Project::diagnostics`; the fork
/// binding publishes real LSP diagnostics here).
///
/// Path discipline: buffers are keyed by workspace-RELATIVE keys (`/src/a.rs`);
/// `workdir` anchors them to disk. Host-tool/URI paths resolve through
/// [`HostTools::resolve`] — absolute paths under workdir, `/`-prefixed keys,
/// and bare relative paths all land on the same buffer; paths escaping the
/// workspace are rejected (fail closed, never a jailbreak).
pub struct HostTools {
    buffers: Mutex<BTreeMap<PathBuf, String>>,
    active_file: Mutex<Option<PathBuf>>,
    selection: Mutex<Option<(PathBuf, usize, usize)>>,
    diagnostics: Mutex<HashMap<PathBuf, Vec<Diagnostic>>>,
    workdir: PathBuf,
    /// ADR-0044 state dir: `cedian://corrections` reads `corrections.jsonl` here.
    state_dir: Mutex<Option<PathBuf>>,
}

impl HostTools {
    /// Empty host rooted at `workdir` (disk anchor for resolve + auto-open).
    /// Canonicalizes (macOS `/tmp` → `/private/tmp`): without this, server
    /// URIs never match `resolve` and diagnostics silently vanish.
    pub fn new(workdir: &Path) -> Self {
        Self {
            buffers: Mutex::default(),
            active_file: Mutex::new(None),
            selection: Mutex::new(None),
            diagnostics: Mutex::new(HashMap::new()),
            workdir: workdir
                .canonicalize()
                .unwrap_or_else(|_| workdir.to_path_buf()),
            state_dir: Mutex::new(None),
        }
    }

    /// Shared handle (host-tool/URI handlers capture this).
    pub fn shared(workdir: &Path) -> Arc<Self> {
        Arc::new(Self::new(workdir))
    }

    /// Point `cedian://corrections` at the workspace state dir (ADR-0032).
    pub fn set_state_dir(&self, dir: Option<PathBuf>) {
        *self.state_dir.lock() = dir;
    }

    /// Open buffer keys (for CLI sync + `cedian://open-editors`).
    pub fn open_keys(&self) -> Vec<PathBuf> {
        self.buffers.lock().keys().cloned().collect()
    }

    /// Open a buffer with initial text (test setup / workspace scan). Accepts
    /// canonical keys directly — no re-resolve (a `/`-key is absolute-shaped
    /// but is NOT outside workdir; resolving it again would wrongly reject).
    /// Untrusted spellings go through [`HostTools::resolve`] first.
    pub fn open(&self, path: &Path, text: &str) {
        let key = normalize_key(path);
        self.buffers.lock().insert(key, text.to_string());
    }

    /// Resolve any workspace path spelling to its canonical buffer key:
    /// absolute-under-workdir, `/`-prefixed key, or bare relative. Rejects
    /// escapes (`..` past root, absolute outside workdir) with a visible error.
    pub fn resolve(&self, path: &Path) -> Result<PathBuf, String> {
        // Normalize BEFORE prefix-strip: server URIs may use uncanonical
        // spellings (`/tmp/...` vs `/private/tmp/...`) while workdir is
        // canonical. `canonicalize` needs existence — lexical `/tmp`
        // fallback needs none.
        let path = &self.workdir.join(path);
        let canon = path.canonicalize().unwrap_or_else(|_| {
            let s = path.to_string_lossy();
            if s.starts_with("/tmp/") {
                PathBuf::from(s.replacen("/tmp/", "/private/tmp/", 1))
            } else {
                path.to_path_buf()
            }
        });
        let rel = if canon.is_absolute() {
            match canon.strip_prefix(&self.workdir) {
                Ok(r) => r.to_path_buf(),
                // A `/`-key (`/notes.txt`) is absolute-shaped too. Accept it
                // only when it names an open buffer or a file inside the
                // workspace, so it can never address anything outside.
                Err(_) if self.is_workspace_key(path) => {
                    path.strip_prefix("/").unwrap_or(path).to_path_buf()
                }
                Err(_) => return Err(format!("path escapes workspace: {}", path.display())),
            }
        } else {
            path.strip_prefix("/").unwrap_or(path).to_path_buf()
        };
        if rel
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(format!("path escapes workspace: {}", path.display()));
        }
        Ok(PathBuf::from(format!("/{}", rel.display())))
    }
    /// Whether `/`-prefixed `path` names an open buffer or a workspace file.
    fn is_workspace_key(&self, path: &Path) -> bool {
        let key = normalize_key(path);
        if self.buffers.lock().contains_key(&key) {
            return true;
        }
        let rel = path.strip_prefix("/").unwrap_or(path);
        !rel.components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
            && self.workdir.join(rel).is_file()
    }

    /// Ensure a buffer is open for a key, loading from disk on first use.
    /// Returns `false` when there is nothing to open (no disk file either) —
    /// callers fail with a visible "not open" error, never an empty buffer.
    fn ensure_open(&self, key: &Path) -> bool {
        if self.buffers.lock().contains_key(key) {
            return true;
        }
        let rel = key.strip_prefix("/").unwrap_or(key);
        let local = self.workdir.join(rel);
        match std::fs::read_to_string(&local) {
            Ok(text) => {
                self.buffers.lock().insert(key.to_path_buf(), text);
                true
            }
            Err(_) => false,
        }
    }

    /// Set the active file (Zed binding: cursor moves).
    pub fn set_active_file(&self, path: Option<PathBuf>) {
        *self.active_file.lock() = path;
    }

    /// Set the selection (Zed binding: selection changes).
    pub fn set_selection(&self, sel: Option<(PathBuf, usize, usize)>) {
        *self.selection.lock() = sel;
    }

    /// Build the `cedian` URI scheme: serves every Phase 6 kind (buffer,
    /// selection, active-file, diagnostics, open-editors). Unknown kinds fail
    /// with `isError`, never silent empty (forward-compat for browser/ios/review).
    pub fn cedian_uri_scheme(self: &Arc<Self>) -> HostUri {
        let host = Arc::clone(self);
        HostUri::new("cedian", move |url, _ctx| host.read_uri(url)).expect("cedian scheme is valid")
    }

    /// Serve one `cedian://` read.
    pub fn read_uri(&self, url: &str) -> Result<omp_rpc::HostUriRead, omp_rpc::HostUriError> {
        let uri = crate::parse_cedian_uri(url)
            .map_err(|e| -> omp_rpc::HostUriError { e.to_string().into() })?;
        self.serve_uri(&uri)
    }

    /// Serve one parsed `cedian://` URL. Pure dispatch — each kind has its own
    /// renderer below so Phase 7+ (browser/ios/review) extends by adding arms.
    fn serve_uri(
        &self,
        uri: &crate::CedianUri,
    ) -> Result<omp_rpc::HostUriRead, omp_rpc::HostUriError> {
        use crate::UriKind;
        match &uri.kind {
            UriKind::Buffer => {
                let key = self
                    .resolve(Path::new(&uri.path))
                    .map_err(|e| -> omp_rpc::HostUriError { e.into() })?;
                if !self.ensure_open(&key) {
                    return Err(format!("buffer not open: {}", uri.path).into());
                }
                self.read_buffer(&key).map(|text| text.into()).ok_or_else(
                    || -> omp_rpc::HostUriError { format!("buffer not open: {}", uri.path).into() },
                )
            }
            UriKind::Selection => Ok(self.render_selection().into()),
            UriKind::ActiveFile => Ok(self.render_active_file().into()),
            UriKind::OpenEditors => Ok(self.render_open_editors().into()),
            UriKind::Corrections => self
                .render_corrections()
                .map(|text| text.into())
                .map_err(|e| -> omp_rpc::HostUriError { e.into() }),
            UriKind::Diagnostics
            | UriKind::Symbols
            | UriKind::Definitions
            | UriKind::References => Err(HEADLESS_LSP.to_string().into()),
            UriKind::Unknown(kind) => Err(format!("unknown cedian:// kind: {kind}").into()),
        }
    }

    /// `cedian://corrections`: the ledger JSONL, or empty when none.
    fn render_corrections(&self) -> Result<String, String> {
        let Some(dir) = self.state_dir.lock().clone() else {
            return Err("cedian://corrections needs a state dir".into());
        };
        match std::fs::read_to_string(dir.join("corrections.jsonl")) {
            Ok(text) => Ok(text),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
            Err(e) => Err(format!("corrections.jsonl: {e}")),
        }
    }
    /// Publish diagnostics for a buffer (fork: LSP publishDiagnostics lands here).
    pub fn publish_diagnostics(&self, path: &Path, diagnostics: Vec<Diagnostic>) {
        self.diagnostics
            .lock()
            .insert(path.to_path_buf(), diagnostics);
    }

    /// Render `cedian://selection`: `path:start-end` + selected text, or empty
    /// (no silent stale — empty means no selection NOW).
    fn render_selection(&self) -> String {
        match self.selection() {
            Some((path, start, end)) => {
                let text = self.read_buffer(&path).unwrap_or_default();
                let end = end.min(text.len());
                let start = start.min(end);
                format!(
                    "{}:{}-{}\n{}",
                    path.display(),
                    start,
                    end,
                    &text[start..end]
                )
            }
            None => String::new(),
        }
    }

    /// Render `cedian://active-file`: the active path, or empty.
    fn render_active_file(&self) -> String {
        self.active_file()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    /// Render `cedian://diagnostics[/path]`: `severity path:line message` lines.
    /// Empty path = all buffers. Empty output = no diagnostics (not an error).
    /// Path matches with or without a leading `/` (URI paths strip it).
    pub fn render_diagnostics(&self, path: &str) -> String {
        let all = self.diagnostics.lock();
        let mut lines = Vec::new();
        for (buf, diags) in all.iter() {
            if !path.is_empty() {
                let buf_str = buf.to_string_lossy();
                let buf_no_slash = buf_str.strip_prefix('/').unwrap_or(&buf_str);
                if buf_no_slash != path && buf_str != path {
                    continue;
                }
            }
            for d in diags {
                let sev = match d.severity {
                    DiagnosticSeverity::Error => "error",
                    DiagnosticSeverity::Warning => "warning",
                    DiagnosticSeverity::Info => "info",
                };
                lines.push(format!(
                    "{sev} {}:{} {}",
                    d.path.display(),
                    d.line,
                    d.message
                ));
            }
        }
        lines.sort();
        lines.join("\n")
    }

    /// Render `cedian://open-editors`: one open buffer path per line.
    fn render_open_editors(&self) -> String {
        let mut paths: Vec<String> = self
            .buffers
            .lock()
            .keys()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        paths.sort();
        paths.join("\n")
    }
}

impl WorkspaceHost for HostTools {
    fn active_file(&self) -> Option<PathBuf> {
        self.active_file.lock().clone()
    }

    fn selection(&self) -> Option<(PathBuf, usize, usize)> {
        self.selection.lock().clone()
    }

    fn read_buffer(&self, path: &Path) -> Option<String> {
        self.buffers.lock().get(path).cloned()
    }

    fn diagnostics(&self, path: &Path) -> Vec<Diagnostic> {
        self.diagnostics
            .lock()
            .get(path)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_kinds_serve() {
        use crate::{CedianUri, UriKind};
        let h = HostTools::new(Path::new("/"));
        h.open(Path::new("/a.rs"), "hello\nworld\n");
        h.open(Path::new("/b.rs"), "x\n");
        h.set_active_file(Some(PathBuf::from("/a.rs")));
        h.set_selection(Some((PathBuf::from("/a.rs"), 0, 5)));
        h.publish_diagnostics(
            Path::new("/a.rs"),
            vec![Diagnostic {
                path: PathBuf::from("/a.rs"),
                line: 1,
                severity: DiagnosticSeverity::Warning,
                message: "unused".to_string(),
            }],
        );
        let serve = |kind: UriKind, path: &str| {
            h.serve_uri(&CedianUri {
                kind,
                path: path.to_string(),
            })
            .unwrap()
            .content
        };
        assert_eq!(serve(UriKind::Buffer, "/a.rs"), "hello\nworld\n");
        assert!(serve(UriKind::Selection, "").contains("/a.rs:0-5"));
        assert_eq!(serve(UriKind::ActiveFile, ""), "/a.rs");
        assert!(h.render_diagnostics("").contains("warning /a.rs:1 unused"));
        assert!(h.render_diagnostics("a.rs").contains("unused"));
        assert!(!h.render_diagnostics("b.rs").contains("unused"));
        let editors = serve(UriKind::OpenEditors, "");
        assert!(editors.contains("/a.rs") && editors.contains("/b.rs"));
        // Unknown kind: visible error, never silent empty.
        assert!(
            h.serve_uri(&CedianUri {
                kind: UriKind::Unknown("nope".into()),
                path: String::new()
            })
            .is_err()
        );
        // Code intelligence is the app's (Zed's language servers).
        for kind in [
            UriKind::Diagnostics,
            UriKind::Symbols,
            UriKind::Definitions,
            UriKind::References,
        ] {
            let e = h
                .serve_uri(&CedianUri {
                    kind,
                    path: "a.rs:0:0".into(),
                })
                .unwrap_err();
            assert!(e.to_string().contains("OMP's own lsp tool"), "{e}");
        }
        // Missing buffer: visible error.
        assert!(
            h.serve_uri(&CedianUri {
                kind: UriKind::Buffer,
                path: "/zzz.rs".into()
            })
            .is_err()
        );
        // Corrections need a state dir; empty ledger is empty, not an error.
        assert!(
            h.serve_uri(&CedianUri {
                kind: UriKind::Corrections,
                path: String::new(),
            })
            .is_err()
        );
        let state =
            std::env::temp_dir().join(format!("cedian-corrections-uri-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&state);
        std::fs::create_dir_all(&state).unwrap();
        h.set_state_dir(Some(state.clone()));
        assert_eq!(serve(UriKind::Corrections, ""), "");
        std::fs::write(state.join("corrections.jsonl"), "{\"id\":\"c1\"}\n").unwrap();
        assert!(serve(UriKind::Corrections, "").contains("\"id\":\"c1\""));
        let _ = std::fs::remove_dir_all(state);
    }

    /// A relative path names a file of the workspace, whatever the
    /// process's cwd holds.
    #[test]
    fn relative_path_resolves_against_the_workdir() {
        let dir = std::env::temp_dir().join(format!("cedian-resolve-rel-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let h = HostTools::new(&dir);
        // The crate's own Cargo.toml exists in the test's cwd, not in `dir`.
        assert_eq!(
            h.resolve(Path::new("Cargo.toml")),
            Ok(PathBuf::from("/Cargo.toml"))
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn tmp_symlink_resolves() {
        // macOS `/tmp` → `/private/tmp`: server URIs use either spelling.
        std::fs::create_dir_all("/tmp/fake-ws-resolve").unwrap();
        let h = HostTools::new(Path::new("/tmp/fake-ws-resolve"));
        let key = h
            .resolve(Path::new("/tmp/fake-ws-resolve/fake.rs"))
            .expect("resolves");
        assert_eq!(key, PathBuf::from("/fake.rs"));
    }

    #[test]
    fn slash_key_resolves_only_inside_workspace() {
        let ws = std::env::temp_dir().join(format!("cedian-ws-slashkey-{}", std::process::id()));
        std::fs::create_dir_all(&ws).unwrap();
        std::fs::write(ws.join("notes.txt"), "x\n").unwrap();
        let ws = ws.canonicalize().unwrap();
        let h = HostTools::new(&ws);
        // `/`-key naming a workspace file (what models copy from buffer keys).
        assert_eq!(
            h.resolve(Path::new("/notes.txt")).unwrap(),
            PathBuf::from("/notes.txt")
        );
        // `/`-key naming an open (unsaved) buffer.
        h.open(Path::new("/draft.rs"), "fn main() {}\n");
        assert_eq!(
            h.resolve(Path::new("/draft.rs")).unwrap(),
            PathBuf::from("/draft.rs")
        );
        // Real absolute paths outside the workspace stay rejected.
        assert!(h.resolve(Path::new("/etc/hosts")).is_err());
        assert!(h.resolve(Path::new("/nope.txt")).is_err());
        assert!(h.resolve(Path::new("/../etc/hosts")).is_err());
        let _ = std::fs::remove_dir_all(&ws);
    }
}
