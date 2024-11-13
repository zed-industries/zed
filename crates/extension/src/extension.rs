pub mod extension_builder;
mod extension_manifest;

use anyhow::{anyhow, bail, Context as _, Result};
use semantic_version::SemanticVersion;

pub use crate::extension_manifest::*;

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
