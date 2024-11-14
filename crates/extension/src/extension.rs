pub mod extension_builder;
mod extension_manifest;
mod slash_command;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, bail, Context as _, Result};
use async_trait::async_trait;
use gpui::Task;
use semantic_version::SemanticVersion;

pub use crate::extension_manifest::*;
pub use crate::slash_command::*;

#[async_trait]
pub trait WorktreeDelegate: Send + Sync + 'static {
    fn id(&self) -> u64;
    fn root_path(&self) -> String;
    async fn read_text_file(&self, path: PathBuf) -> Result<String>;
    async fn which(&self, binary_name: String) -> Option<String>;
    async fn shell_env(&self) -> Vec<(String, String)>;
}

pub trait KeyValueStoreDelegate: Send + Sync + 'static {
    fn insert(&self, key: String, docs: String) -> Task<Result<()>>;
}

#[async_trait]
pub trait Extension: Send + Sync + 'static {
    /// Returns the [`ExtensionManifest`] for this extension.
    fn manifest(&self) -> Arc<ExtensionManifest>;

    /// Returns the path to this extension's working directory.
    fn work_dir(&self) -> Arc<Path>;

    async fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        arguments: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>>;

    async fn run_slash_command(
        &self,
        command: SlashCommand,
        arguments: Vec<String>,
        resource: Option<Arc<dyn WorktreeDelegate>>,
    ) -> Result<SlashCommandOutput>;

    async fn suggest_docs_packages(&self, provider: Arc<str>) -> Result<Vec<String>>;

    async fn index_docs(
        &self,
        provider: Arc<str>,
        package_name: Arc<str>,
        kv_store: Arc<dyn KeyValueStoreDelegate>,
    ) -> Result<()>;
}

pub fn parse_wasm_extension_version(
    extension_id: &str,
    wasm_bytes: &[u8],
) -> Result<SemanticVersion> {
    let mut version = None;

    for part in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
        if let wasmparser::Payload::CustomSection(s) =
            part.context("error parsing wasm extension")?
        {
            if s.name() == "zed:api-version" {
                version = parse_wasm_extension_version_custom_section(s.data());
                if version.is_none() {
                    bail!(
                        "extension {} has invalid zed:api-version section: {:?}",
                        extension_id,
                        s.data()
                    );
                }
            }
        }
    }

    // The reason we wait until we're done parsing all of the Wasm bytes to return the version
    // is to work around a panic that can happen inside of Wasmtime when the bytes are invalid.
    //
    // By parsing the entirety of the Wasm bytes before we return, we're able to detect this problem
    // earlier as an `Err` rather than as a panic.
    version.ok_or_else(|| anyhow!("extension {} has no zed:api-version section", extension_id))
}

fn parse_wasm_extension_version_custom_section(data: &[u8]) -> Option<SemanticVersion> {
    if data.len() == 6 {
        Some(SemanticVersion::new(
            u16::from_be_bytes([data[0], data[1]]) as _,
            u16::from_be_bytes([data[2], data[3]]) as _,
            u16::from_be_bytes([data[4], data[5]]) as _,
        ))
    } else {
        None
    }
}
