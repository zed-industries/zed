//! Provides support for language toolchains.
//!
//! A language can have associated toolchains,
//! which is a set of tools used to interact with the projects written in said language.
//! For example, a Python project can have an associated virtual environment; a Rust project can have a toolchain override.

use gpui::SharedString;

/// Represents a single toolchain.
pub struct Toolchain {
    /// User-facing label
    pub label: SharedString,
    /// Action that should be taken in order to activate a given toolchain.
    pub action: (),
}

///
pub trait ToolchainLister {
    fn list(&self) -> ToolchainList;
    fn activate(&self, _: Toolchain);
}

type DefaultIndex = usize;
pub struct ToolchainList(Vec<Toolchain>, DefaultIndex);
