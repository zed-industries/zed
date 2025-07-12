//! Virtual document handling for Deno external dependencies.
//!
//! This module manages virtual documents fetched from Deno's language server
//! for external modules (from jsr.io, deno.land, npm, etc).

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use collections::HashMap;
use fs::MTime;
use gpui::{App, AppContext, Context, Entity};
use language::{Buffer, DiskState};
use worktree::ProjectEntryId;

/// A virtual file for Deno external dependencies
pub(crate) struct DenoVirtualFile {
    pub(crate) uri: lsp::Url,
    entry_id: ProjectEntryId,
    mtime: Option<SystemTime>,
    path: Arc<Path>,
    worktree_id: worktree::WorktreeId,
}

impl DenoVirtualFile {}

impl language::File for DenoVirtualFile {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        DiskState::Present {
            mtime: self
                .mtime
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| MTime::from_seconds_and_nanos(d.as_secs(), d.subsec_nanos()))
                .unwrap_or(MTime::from_seconds_and_nanos(0, 0)),
        }
    }

    fn worktree_id(&self, _cx: &App) -> worktree::WorktreeId {
        self.worktree_id
    }

    fn path(&self) -> &Arc<Path> {
        &self.path
    }

    fn full_path(&self, _cx: &App) -> PathBuf {
        PathBuf::from(self.uri.as_str())
    }

    fn file_name<'a>(&'a self, _cx: &'a App) -> &'a OsStr {
        self.path.file_name().unwrap_or(OsStr::new("virtual"))
    }

    fn to_proto(&self, _cx: &App) -> rpc::proto::File {
        rpc::proto::File {
            worktree_id: 0,
            entry_id: Some(self.entry_id.to_proto()),
            path: self.uri.as_str().to_string(),
            mtime: self.mtime.and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| rpc::proto::Timestamp {
                        seconds: d.as_secs(),
                        nanos: d.subsec_nanos(),
                    })
            }),
            is_deleted: false,
        }
    }

    fn is_private(&self) -> bool {
        false
    }

    fn lsp_url(&self, _cx: &App) -> Option<lsp::Url> {
        // Deno virtual documents always have their URI
        Some(self.uri.clone())
    }
}

/// Generates unique entry IDs for virtual documents
static VIRTUAL_ENTRY_ID_COUNTER: AtomicU64 = AtomicU64::new(u64::MAX / 2);

/// Stores virtual documents fetched from Deno's language server.
pub(crate) struct DenoVirtualDocumentStore {
    /// Maps Deno URIs to buffer IDs containing the document content.
    virtual_docs: HashMap<lsp::Url, Entity<Buffer>>,
}

impl DenoVirtualDocumentStore {
    pub fn new(_fs: Arc<dyn fs::Fs>, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            virtual_docs: Default::default(),
        })
    }

    /// Get a cached virtual document buffer if it exists.
    pub fn get_cached_buffer(&self, uri: &lsp::Url) -> Option<Entity<Buffer>> {
        self.virtual_docs.get(uri).cloned()
    }

    /// Store a virtual document as an in-memory buffer.
    pub fn store_virtual_document(
        &mut self,
        uri: lsp::Url,
        content: String,
        language_registry: Arc<language::LanguageRegistry>,
        worktree_id: worktree::WorktreeId,
        cx: &mut Context<Self>,
    ) -> Entity<Buffer> {
        // Check if we already have this buffer
        if let Some(buffer) = self.virtual_docs.get(&uri) {
            return buffer.clone();
        }

        // Generate a unique entry ID for this virtual document
        let entry_id = ProjectEntryId::from_usize(
            VIRTUAL_ENTRY_ID_COUNTER.fetch_add(1, Ordering::SeqCst) as usize,
        );

        // Create a virtual file for this document
        let virtual_file = Arc::new(DenoVirtualFile {
            uri: uri.clone(),
            entry_id,
            mtime: Some(SystemTime::now()),
            path: Arc::from(Path::new(uri.as_str())),
            worktree_id,
        });

        // Create a read-only in-memory buffer with the virtual file
        let buffer = cx.new(|cx| {
            let text_buffer =
                text::Buffer::new(0, cx.entity_id().as_non_zero_u64().into(), content);
            let buffer = Buffer::build(
                text_buffer,
                Some(virtual_file),
                language::Capability::ReadOnly,
            );

            // Set the language registry so language detection works
            buffer.set_language_registry(language_registry.clone());

            buffer
        });

        // Cache the buffer
        self.virtual_docs.insert(uri, buffer.clone());

        buffer
    }
}
