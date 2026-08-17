//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use crate::isa_builder::IsaBuilder;
use anyhow::Result;
use cranelift_codegen::{
    CodegenResult,
    isa::{self, OwnedTargetIsa},
};
use std::fmt;
use std::path;
use std::sync::Arc;
use target_lexicon::Triple;
use wasmtime_environ::{CacheStore, CompilerBuilder, Setting, Tunables};

struct Builder {
    tunables: Option<Tunables>,
    inner: IsaBuilder<CodegenResult<OwnedTargetIsa>>,
    linkopts: LinkOptions,
    cache_store: Option<Arc<dyn CacheStore>>,
    clif_dir: Option<path::PathBuf>,
    wmemcheck: bool,
}

#[derive(Clone, Default)]
pub struct LinkOptions {
    /// A debug-only setting used to synthetically insert 0-byte padding between
    /// compiled functions to simulate huge compiled artifacts and exercise
    /// logic related to jump veneers.
    pub padding_between_functions: usize,

    /// A debug-only setting used to force inter-function calls in a wasm module
    /// to always go through "jump veneers" which are typically only generated
    /// when functions are very far from each other.
    pub force_jump_veneers: bool,
}

pub fn builder(triple: Option<Triple>) -> Result<Box<dyn CompilerBuilder>> {
    Ok(Box::new(Builder {
        tunables: None,
        inner: IsaBuilder::new(triple, |triple| isa::lookup(triple).map_err(|e| e.into()))?,
        linkopts: LinkOptions::default(),
        cache_store: None,
        clif_dir: None,
        wmemcheck: false,
    }))
}

impl CompilerBuilder for Builder {
    fn triple(&self) -> &target_lexicon::Triple {
        self.inner.triple()
    }

    fn clif_dir(&mut self, path: &path::Path) -> Result<()> {
        self.clif_dir = Some(path.to_path_buf());
        Ok(())
    }

    fn target(&mut self, target: target_lexicon::Triple) -> Result<()> {
        self.inner.target(target)?;
        Ok(())
    }

    fn set(&mut self, name: &str, value: &str) -> Result<()> {
        // Special wasmtime-cranelift-only settings first
        match name {
            "wasmtime_linkopt_padding_between_functions" => {
                self.linkopts.padding_between_functions = value.parse()?;
            }
            "wasmtime_linkopt_force_jump_veneer" => {
                self.linkopts.force_jump_veneers = value.parse()?;
            }
            "wasmtime_inlining_intra_module" => {
                self.tunables.as_mut().unwrap().inlining_intra_module = value.parse()?;
            }
            "wasmtime_inlining_small_callee_size" => {
                self.tunables.as_mut().unwrap().inlining_small_callee_size = value.parse()?;
            }
            "wasmtime_inlining_sum_size_threshold" => {
                self.tunables.as_mut().unwrap().inlining_sum_size_threshold = value.parse()?;
            }
            _ => {
                self.inner.set(name, value)?;
            }
        }
        Ok(())
    }

    fn enable(&mut self, name: &str) -> Result<()> {
        self.inner.enable(name)
    }

    fn set_tunables(&mut self, tunables: Tunables) -> Result<()> {
        self.tunables = Some(tunables);
        Ok(())
    }

    fn build(&self) -> Result<Box<dyn wasmtime_environ::Compiler>> {
        let isa = self.inner.build()?;
        Ok(Box::new(crate::compiler::Compiler::new(
            self.tunables
                .as_ref()
                .expect("set_tunables not called")
                .clone(),
            isa,
            self.cache_store.clone(),
            self.linkopts.clone(),
            self.clif_dir.clone(),
            self.wmemcheck,
        )))
    }

    fn settings(&self) -> Vec<Setting> {
        self.inner.settings()
    }

    fn enable_incremental_compilation(
        &mut self,
        cache_store: Arc<dyn wasmtime_environ::CacheStore>,
    ) -> Result<()> {
        self.cache_store = Some(cache_store);
        Ok(())
    }

    fn wmemcheck(&mut self, enable: bool) {
        self.wmemcheck = enable;
    }
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Builder")
            .field("shared_flags", &self.inner.shared_flags().to_string())
            .finish()
    }
}
