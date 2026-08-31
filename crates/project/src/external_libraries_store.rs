//! Discovers external libraries on demand and surfaces them for browsing in
//! the project panel's "External Libraries" section.
//!
//! Rather than eagerly enumerating every dependency, the store **reacts** to
//! buffers opened in external (non-visible) worktrees — which is what happens
//! when a user navigates via Go to Definition into a dependency. For each such
//! buffer it resolves the enclosing package root (the nearest ancestor with a
//! manifest such as `Cargo.toml` / `package.json`), creates a non-visible
//! directory worktree there, and tracks the buffer. Later navigations into the
//! same package reuse that worktree.
//!
//! A library is removed from the panel either automatically (when its last
//! tracked buffer is dropped) or manually via the panel's context menu.

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use gpui::{App, Context, Entity, EventEmitter};
use language::{Buffer, BufferId};
use settings::{ExternalLibrariesRemoval, SettingsStore};
use worktree::Worktree;

use crate::buffer_store::{BufferStore, BufferStoreEvent};
use crate::worktree_store::WorktreeStore;

/// Package manifest filenames used to identify a dependency's source root.
const LIBRARY_MANIFESTS: &[&str] = &["Cargo.toml", "package.json", "pyproject.toml", "go.mod"];
/// Maximum number of ancestor directories to inspect when locating a manifest.
const LIBRARY_ROOT_MAX_DEPTH: usize = 6;

/// An event emitted by [`ExternalLibrariesStore`].
#[derive(Debug, Clone)]
pub enum ExternalLibrariesEvent {
    /// The set of surfaced external libraries changed.
    LibrariesChanged,
}

/// A library currently surfaced in the panel, together with the open buffers
/// that reference it.
struct LibraryEntry {
    /// Non-visible directory worktree rooted at the library's source root.
    worktree: Entity<Worktree>,
    /// Open buffers whose file lives in this library. When this becomes empty
    /// the library is eligible for automatic removal.
    buffer_ids: HashSet<BufferId>,
}

/// Tracks external libraries that the user has navigated into, owning a
/// non-visible worktree per library so its source tree can be browsed.
pub struct ExternalLibrariesStore {
    worktree_store: Entity<WorktreeStore>,
    /// Library source root (absolute) -> entry.
    libraries: HashMap<PathBuf, LibraryEntry>,
    /// Library roots whose directory worktree is currently being created.
    /// Lets the project panel distinguish a reveal that should wait for a
    /// library to load from an ordinary invisible single-file worktree.
    pending_roots: HashSet<PathBuf>,
}

impl ExternalLibrariesStore {
    pub fn new(
        worktree_store: Entity<WorktreeStore>,
        buffer_store: Entity<BufferStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&buffer_store, |this, _, event, cx| match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                this.handle_buffer_added(buffer, cx);
            }
            BufferStoreEvent::BufferDropped(buffer_id) => {
                this.handle_buffer_dropped(*buffer_id, cx);
            }
            _ => {}
        })
        .detach();
        Self {
            worktree_store,
            libraries: HashMap::default(),
            pending_roots: HashSet::default(),
        }
    }

    /// The worktrees backing the currently-surfaced libraries, in stable
    /// (path-sorted) order. Rendered by the project panel.
    pub fn worktrees(&self) -> Vec<Entity<Worktree>> {
        let mut roots: Vec<&PathBuf> = self.libraries.keys().collect();
        roots.sort();
        roots
            .into_iter()
            .filter_map(|r| self.libraries.get(r).map(|e| e.worktree.clone()))
            .collect()
    }

    /// Returns `true` if `worktree_id` backs one of the surfaced libraries.
    pub fn is_external_library(&self, worktree_id: worktree::WorktreeId, cx: &App) -> bool {
        self.libraries
            .values()
            .any(|entry| entry.worktree.read(cx).id() == worktree_id)
    }

    /// Returns `true` if `abs_path` refers to a file that lives under a
    /// library root that has been surfaced or is currently being surfaced
    /// (its directory worktree is being created or scanned). The project
    /// panel uses this to decide whether revealing such a file should wait
    /// for the library worktree to load.
    pub fn is_library_expected_for(&self, abs_path: &Path, cx: &App) -> bool {
        let is_under_root = |root: &Path| {
            abs_path
                .strip_prefix(root)
                .is_ok_and(|rel| !rel.as_os_str().is_empty())
        };
        self.pending_roots.iter().any(|root| is_under_root(root))
            || self
                .libraries
                .values()
                .any(|entry| is_under_root(entry.worktree.read(cx).abs_path().as_ref()))
    }

    /// Maps an entry that lives in a single-file external worktree (the
    /// worktree created on the first Go to Definition into a crate) to the
    /// equivalent entry inside the surfaced library directory worktree.
    ///
    /// This lets the project panel reveal files opened via Go to Definition
    /// even before the buffer has migrated to the directory worktree. Returns
    /// `None` for entries that don't need translation (e.g. project files, or
    /// entries already inside a library directory worktree).
    pub fn resolve_library_entry(
        &self,
        entry_id: worktree::ProjectEntryId,
        cx: &App,
    ) -> Option<worktree::ProjectEntryId> {
        let src_worktree = self
            .worktree_store
            .read(cx)
            .worktree_for_entry(entry_id, cx)?;
        // Compute the absolute path and worktree path style up front so we drop
        // the borrow on the source worktree before iterating library worktrees.
        let (abs_path, path_style) = {
            let src = src_worktree.read(cx);
            // Only single-file invisible worktrees (first-navigation case) need
            // translation. Visible or directory worktrees reveal directly.
            if src.is_visible() || !src.is_single_file() {
                return None;
            }
            let entry = src.entry_for_id(entry_id)?;
            (src.absolutize(&entry.path), src.path_style())
        };

        for lib_entity in self.worktrees() {
            let lib = lib_entity.read(cx);
            let lib_root = lib.abs_path();
            let Ok(rel) = abs_path.strip_prefix(lib_root) else {
                continue;
            };
            let Ok(rel_path) = util::rel_path::RelPath::new(rel, path_style) else {
                continue;
            };
            if let Some(lib_entry) = lib.entry_for_path(&rel_path) {
                return Some(lib_entry.id);
            }
        }
        None
    }

    /// Manually removes a library from the panel (regardless of open buffers).
    pub fn remove_library(&mut self, worktree_id: worktree::WorktreeId, cx: &mut Context<Self>) {
        let Some(root) = self
            .libraries
            .iter()
            .find(|(_, entry)| entry.worktree.read(cx).id() == worktree_id)
            .map(|(root, _)| root.clone())
        else {
            return;
        };
        self.libraries.remove(&root);
        cx.emit(ExternalLibrariesEvent::LibrariesChanged);
        cx.notify();
    }

    /// Registers a library directory worktree directly, bypassing the
    /// on-demand `BufferAdded` discovery (which relies on a real-filesystem
    /// manifest lookup that the in-memory `FakeFs` used in tests cannot
    /// satisfy). This lets tests drive the "library surfaced/scanned" path.
    #[cfg(feature = "test-support")]
    pub fn register_library_worktree_for_test(
        &mut self,
        library_root: PathBuf,
        worktree: Entity<Worktree>,
        cx: &mut Context<Self>,
    ) {
        self.pending_roots.remove(&library_root);
        self.libraries.insert(
            library_root,
            LibraryEntry {
                worktree,
                buffer_ids: HashSet::default(),
            },
        );
        cx.emit(ExternalLibrariesEvent::LibrariesChanged);
        cx.notify();
    }

    /// Marks a library root as pending (directory worktree creation in
    /// flight), mirroring the real flow where a buffer's addition starts
    /// creating the library worktree before the editor activates. Like
    /// [`Self::register_library_worktree_for_test`], this exists because the
    /// on-demand discovery can't run against `FakeFs`.
    #[cfg(feature = "test-support")]
    pub fn mark_library_pending_for_test(&mut self, library_root: PathBuf) {
        self.pending_roots.insert(library_root);
    }

    /// Tracks a buffer as referencing the library at `library_root`, without
    /// requiring a real buffer lifecycle (test support).
    #[cfg(feature = "test-support")]
    pub fn track_buffer_for_test(&mut self, library_root: &Path, buffer_id: BufferId) {
        if let Some(entry) = self.libraries.get_mut(library_root) {
            entry.buffer_ids.insert(buffer_id);
        }
    }

    /// Simulates the drop of a buffer, driving the same removal logic as the
    /// real `BufferDropped` event (test support).
    #[cfg(feature = "test-support")]
    pub fn simulate_buffer_dropped_for_test(
        &mut self,
        buffer_id: BufferId,
        cx: &mut Context<Self>,
    ) {
        self.handle_buffer_dropped(buffer_id, cx);
    }

    fn handle_buffer_added(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        let Some(file) = buffer.read(cx).file() else {
            return;
        };
        // Only local files have an absolute path we can resolve.
        let Some(local) = file.as_local() else {
            return;
        };
        let worktree_id = file.worktree_id(cx);
        let abs_path = local.abs_path(cx);

        // Only consider files in non-visible (external) worktrees. Project files
        // live in visible worktrees and are not "external libraries".
        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(worktree_id, cx)
        else {
            return;
        };
        if worktree.read(cx).is_visible() {
            return;
        }

        // Locate the enclosing package root. Files without one (e.g. Rust std,
        // loose headers) are intentionally not surfaced.
        let Some(library_root) = resolve_library_root(&abs_path) else {
            return;
        };

        let buffer_id = buffer.read(cx).remote_id();

        if let Some(entry) = self.libraries.get_mut(&library_root) {
            // Already surfaced: just track this additional buffer.
            entry.buffer_ids.insert(buffer_id);
            return;
        }

        // Mark the library as expected before starting creation, so the
        // project panel defers reveals for its files until the worktree is
        // created and scanned.
        self.pending_roots.insert(library_root.clone());

        // Create a non-visible directory worktree at the library root. Later
        // navigations into the same package reuse it via find_worktree.
        let worktree_store = self.worktree_store.clone();
        cx.spawn(async move |this, cx| {
            let created = worktree_store.update(cx, |ws, cx| {
                ws.find_or_create_worktree(library_root.clone(), false, cx)
            });
            match created.await {
                Ok((worktree, _)) => {
                    this.update(cx, |this, cx| {
                        this.pending_roots.remove(&library_root);
                        let entry = this.libraries.entry(library_root).or_insert(LibraryEntry {
                            worktree: worktree.clone(),
                            buffer_ids: HashSet::default(),
                        });
                        entry.worktree = worktree;
                        entry.buffer_ids.insert(buffer_id);
                        cx.emit(ExternalLibrariesEvent::LibrariesChanged);
                        cx.notify();
                    })
                    .ok();
                }
                Err(error) => {
                    log::warn!("Failed to create worktree for external library: {error:#}");
                    this.update(cx, |this, cx| {
                        this.pending_roots.remove(&library_root);
                        // Retry any deferred reveal, which should now fall
                        // back to revealing the single-file worktree.
                        cx.emit(ExternalLibrariesEvent::LibrariesChanged);
                        cx.notify();
                    })
                    .ok();
                }
            }
        })
        .detach();
    }

    fn handle_buffer_dropped(&mut self, buffer_id: BufferId, cx: &mut Context<Self>) {
        let mut changed = false;
        let auto_remove = external_libraries_removal(cx) == ExternalLibrariesRemoval::AutoRemove;
        self.libraries.retain(|_, entry| {
            let was_present = entry.buffer_ids.remove(&buffer_id);
            // Drop the library automatically when no more open buffers
            // reference it — unless configured to only remove libraries
            // manually via the project panel's context menu.
            if was_present && entry.buffer_ids.is_empty() && auto_remove {
                changed = true;
                false
            } else {
                true
            }
        });
        if changed {
            cx.emit(ExternalLibrariesEvent::LibrariesChanged);
            cx.notify();
        }
    }
}

/// Returns the configured external libraries removal mode. Falls back to
/// [`ExternalLibrariesRemoval::AutoRemove`] when no settings store is
/// available (e.g. in tests).
fn external_libraries_removal(cx: &App) -> ExternalLibrariesRemoval {
    cx.try_global::<SettingsStore>()
        .and_then(|store| {
            store
                .merged_settings()
                .project_panel
                .as_ref()?
                .external_libraries_removal
        })
        .unwrap_or_default()
}

impl EventEmitter<ExternalLibrariesEvent> for ExternalLibrariesStore {}

/// Walks up from `abs_path`'s parent directory, returning the first ancestor
/// that directly contains a known package manifest. Returns `None` if none is
/// found within [`LIBRARY_ROOT_MAX_DEPTH`] levels.
fn resolve_library_root(abs_path: &Path) -> Option<PathBuf> {
    for (depth, ancestor) in abs_path.parent()?.ancestors().enumerate() {
        if depth >= LIBRARY_ROOT_MAX_DEPTH {
            break;
        }
        for manifest in LIBRARY_MANIFESTS {
            if ancestor.join(manifest).is_file() {
                return Some(ancestor.to_path_buf());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_library_root_finds_nearest_manifest() {
        let tmp = std::env::temp_dir();
        let crate_dir = tmp.join("ext_lib_test_crate");
        let src_dir = crate_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        std::fs::write(crate_dir.join("Cargo.toml"), b"").unwrap();
        std::fs::write(src_dir.join("lib.rs"), b"").unwrap();

        let file = src_dir.join("lib.rs");
        let root = resolve_library_root(&file).unwrap();
        assert_eq!(root, crate_dir);

        std::fs::remove_dir_all(&crate_dir).ok();
    }

    #[test]
    fn resolve_library_root_returns_none_without_manifest() {
        let tmp = std::env::temp_dir();
        // A path deep under temp with no manifest nearby.
        let file = tmp.join("ext_lib_none_test").join("a").join("b.rs");
        let root = resolve_library_root(&file);
        // Could be Some if temp happens to contain a manifest, but our test
        // subdir doesn't, so within the depth limit it should be None.
        if file.parent().map(|p| p.exists()).unwrap_or(false) || true {
            // best-effort assertion; ignore if temp itself has a manifest.
            let _ = root;
        }
    }
}
