use anyhow::Result;
use wasm_encoder::{ComponentBuilder, ComponentType};
use wit_parser::{PackageId, Resolve, WorldId};

mod v1;
mod v2;

const ENCODE_V2_BY_DEFAULT: bool = true;

fn use_v2_encoding() -> bool {
    match std::env::var("WIT_COMPONENT_ENCODING_V2") {
        Ok(s) => s == "1",
        Err(_) => ENCODE_V2_BY_DEFAULT,
    }
}

/// Encodes the given `package` within `resolve` to a binary WebAssembly
/// representation.
///
/// This function is the root of the implementation of serializing a WIT package
/// into a WebAssembly representation. The wasm representation serves two
/// purposes:
///
/// * One is to be a binary encoding of a WIT document which is ideally more
///   stable than the WIT textual format itself.
/// * Another is to provide a clear mapping of all WIT features into the
///   component model through use of its binary representation.
///
/// The `resolve` provided is a set of packages and types and such and the
/// `package` argument is an ID within the world provided. The documents within
/// `package` will all be encoded into the binary returned.
///
/// The binary returned can be [`decode`d](crate::decode) to recover the WIT
/// package provided.
pub fn encode(use_v2: Option<bool>, resolve: &Resolve, package: PackageId) -> Result<Vec<u8>> {
    let mut component = encode_component(use_v2, resolve, package)?;
    component.raw_custom_section(&crate::base_producers().raw_custom_section());
    Ok(component.finish())
}

/// Exactly like `encode`, except gives an unfinished `ComponentBuilder` in case you need
/// to append anything else before finishing.
pub fn encode_component(
    use_v2: Option<bool>,
    resolve: &Resolve,
    package: PackageId,
) -> Result<ComponentBuilder> {
    if use_v2.unwrap_or_else(use_v2_encoding) {
        v2::encode_component(resolve, package)
    } else {
        v1::encode_component(resolve, package)
    }
}

/// Encodes a `world` as a component type.
pub fn encode_world(resolve: &Resolve, world_id: WorldId) -> Result<ComponentType> {
    v1::encode_world(resolve, world_id)
}
