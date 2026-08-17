//! # external_editor
//!
//! Dedicated bidirectional integration protocol crate ("Edit with Zed")
//! allowing external editors and IDEs (VS Code, JetBrains, Emacs) to stream
//! buffers, synchronize cursors, and apply atomic patches via Zed.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use zed_core_lib::ZedEngine;

/// Helper to safely acquire a mutex guard even if poisoned
fn safe_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Supported IDE capabilities for negotiation
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorCapability {
    BufferStream,
    CursorSync,
    MultiCursorCoordination,
    SelectionSync,
    SaveOnBlur,
    AtomicPatch,
    DiagnosticFeedback,
}

/// Client handshake payload from an external IDE
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalEditorHandshake {
    pub client_ide: String,
    pub protocol_version: String,
    #[serde(default)]
    pub requested_capabilities: Vec<EditorCapability>,
}

/// Cursor Position Representation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPosition {
    pub line: u32,
    pub character: u32,
    pub offset: usize,
}

/// Selection Range Representation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionRange {
    pub start: CursorPosition,
    pub end: CursorPosition,
    pub is_reversed: bool,
}

/// Cursor and Selection Sync Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CursorSyncEvent {
    pub buffer_id: u64,
    pub client_id: String,
    pub primary_cursor: CursorPosition,
    #[serde(default)]
    pub secondary_cursors: Vec<CursorPosition>,
    #[serde(default)]
    pub selections: Vec<SelectionRange>,
}

/// Bidirectional bridge coordinator
#[derive(Clone)]
pub struct ExternalEditorBridge {
    engine: Arc<ZedEngine>,
    cursors: Arc<Mutex<HashMap<u64, HashMap<String, CursorSyncEvent>>>>,
}

impl ExternalEditorBridge {
    pub fn new(engine: Arc<ZedEngine>) -> Self {
        Self {
            engine,
            cursors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Perform handshake with external client and negotiate capabilities
    pub fn handshake(&self, handshake: ExternalEditorHandshake) -> serde_json::Value {
        let server_capabilities = vec![
            EditorCapability::BufferStream,
            EditorCapability::CursorSync,
            EditorCapability::MultiCursorCoordination,
            EditorCapability::SelectionSync,
            EditorCapability::SaveOnBlur,
            EditorCapability::AtomicPatch,
            EditorCapability::DiagnosticFeedback,
        ];

        let negotiated: Vec<EditorCapability> = if handshake.requested_capabilities.is_empty() {
            server_capabilities.clone()
        } else {
            handshake
                .requested_capabilities
                .into_iter()
                .filter(|c| server_capabilities.contains(c))
                .collect()
        };

        serde_json::json!({
            "status": "connected",
            "server": "zed_external_editor_bridge",
            "client_ide": handshake.client_ide,
            "protocol_version": handshake.protocol_version,
            "capabilities": negotiated
        })
    }

    /// Synchronize external patch into internal buffer
    pub fn sync_patch(&self, buffer_id: u64, patch: &str) -> bool {
        let current_text = self.engine.get_text(buffer_id).unwrap_or_default();
        let updated = format!("{}{}", current_text, patch);
        self.engine.apply_transaction(buffer_id, vec![(0, current_text.len(), updated)])
    }

    /// Update live cursor and selection coordinates for an external client
    pub fn update_cursor(&self, event: CursorSyncEvent) -> bool {
        let mut guard = safe_lock(&self.cursors);
        let buffer_cursors = guard.entry(event.buffer_id).or_default();
        buffer_cursors.insert(event.client_id.clone(), event);
        true
    }

    /// Retrieve all active cursors for a given buffer
    pub fn get_cursors(&self, buffer_id: u64) -> Vec<CursorSyncEvent> {
        let guard = safe_lock(&self.cursors);
        guard
            .get(&buffer_id)
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_editor_bridge_flow() {
        let engine = Arc::new(ZedEngine::new());
        let buf_id = engine.create_buffer("initial ".into());
        let bridge = ExternalEditorBridge::new(engine.clone());

        let hs = bridge.handshake(ExternalEditorHandshake {
            client_ide: "vscode".into(),
            protocol_version: "1.0".into(),
            requested_capabilities: vec![
                EditorCapability::BufferStream,
                EditorCapability::CursorSync,
                EditorCapability::AtomicPatch,
            ],
        });
        assert_eq!(hs["status"], "connected");

        let synced = bridge.sync_patch(buf_id, "patch content");
        assert!(synced);
        assert_eq!(engine.get_text(buf_id).unwrap(), "initial patch content");
    }

    #[test]
    fn test_cursor_and_selection_sync() {
        let engine = Arc::new(ZedEngine::new());
        let buf_id = engine.create_buffer("line1\nline2".into());
        let bridge = ExternalEditorBridge::new(engine);

        let event = CursorSyncEvent {
            buffer_id: buf_id,
            client_id: "jetbrains-rust-plugin".into(),
            primary_cursor: CursorPosition {
                line: 1,
                character: 4,
                offset: 10,
            },
            secondary_cursors: vec![CursorPosition {
                line: 0,
                character: 2,
                offset: 2,
            }],
            selections: vec![SelectionRange {
                start: CursorPosition {
                    line: 0,
                    character: 0,
                    offset: 0,
                },
                end: CursorPosition {
                    line: 0,
                    character: 5,
                    offset: 5,
                },
                is_reversed: false,
            }],
        };

        assert!(bridge.update_cursor(event.clone()));
        let active = bridge.get_cursors(buf_id);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].client_id, "jetbrains-rust-plugin");
        assert_eq!(active[0].primary_cursor.line, 1);
        assert_eq!(active[0].selections.len(), 1);
    }
}

