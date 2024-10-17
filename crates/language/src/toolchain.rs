//! Provides support for language toolchains.
//!
//! A language can have associated toolchains,
//! which is a set of tools used to interact with the projects written in said language.
//! For example, a Python project can have an associated virtual environment; a Rust project can have a toolchain override.

use async_trait::async_trait;
use gpui::SharedString;

/// Represents a single toolchain.
pub struct Toolchain {
    /// User-facing label
    pub label: SharedString,
}

#[async_trait]
pub trait ToolchainLister: Send + Sync {
    async fn list(&self) -> ToolchainList;
    async fn activate(&self, _: Toolchain);
}

type DefaultIndex = usize;
#[derive(Default)]
pub struct ToolchainList {
    toolchains: Vec<Toolchain>,
    default: Option<DefaultIndex>,
}

impl ToolchainList {
    pub fn toolchains(&self) -> &[Toolchain] {
        &self.toolchains
    }
}
