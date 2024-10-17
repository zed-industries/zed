//! Provides support for language toolchains.
//!
//! A language can have associated toolchains,
//! which is a set of tools used to interact with the projects written in said language.
//! For example, a Python project can have an associated virtual environment; a Rust project can have a toolchain override.

use std::sync::Arc;

use async_trait::async_trait;
use gpui::{AppContext, SharedString};

use crate::{language_settings::all_language_settings, File, LanguageName};

/// Represents a single toolchain.
#[derive(Clone)]
pub struct Toolchain {
    /// User-facing label
    pub label: SharedString,
}

#[async_trait(?Send)]
pub trait ToolchainLister: Send + Sync {
    fn language_name(&self) -> LanguageName;
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
    pub fn default_toolchain(&self) -> Option<Toolchain> {
        self.default.and_then(|ix| self.toolchains.get(ix)).cloned()
    }
}
