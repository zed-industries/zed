use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use language::DiagnosticEntry;
use lsp::{DiagnosticSeverity, LanguageServerId};
use rpc::proto;
use serde::Serialize;

mod ignore;
pub mod project_settings;
pub mod worktree;
#[cfg(test)]
mod worktree_tests;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectEntryId(usize);

impl ProjectEntryId {
    pub const MAX: Self = Self(usize::MAX);

    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize)]
pub struct DiagnosticSummary {
    pub error_count: usize,
    pub warning_count: usize,
}

impl DiagnosticSummary {
    fn new<'a, T: 'a>(diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<T>>) -> Self {
        let mut this = Self {
            error_count: 0,
            warning_count: 0,
        };

        for entry in diagnostics {
            if entry.diagnostic.is_primary {
                match entry.diagnostic.severity {
                    DiagnosticSeverity::ERROR => this.error_count += 1,
                    DiagnosticSeverity::WARNING => this.warning_count += 1,
                    _ => {}
                }
            }
        }

        this
    }

    pub fn is_empty(&self) -> bool {
        self.error_count == 0 && self.warning_count == 0
    }

    pub fn to_proto(
        &self,
        language_server_id: LanguageServerId,
        path: &Path,
    ) -> proto::DiagnosticSummary {
        proto::DiagnosticSummary {
            path: path.to_string_lossy().to_string(),
            language_server_id: language_server_id.0 as u64,
            error_count: self.error_count as u32,
            warning_count: self.warning_count as u32,
        }
    }
}
