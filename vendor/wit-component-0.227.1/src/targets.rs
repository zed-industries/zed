use crate::encoding::encode_world;
use anyhow::{Context, Result};
use wasm_encoder::{ComponentBuilder, ComponentExportKind, ComponentTypeRef};
use wasmparser::Validator;
use wit_parser::{Resolve, WorldId};

/// This function checks whether `component_to_test` correctly conforms to the world specified.
/// It does so by instantiating a generated component that imports a component instance with
/// the component type as described by the "target" world.
pub fn targets(resolve: &Resolve, world: WorldId, component_to_test: &[u8]) -> Result<()> {
    let mut root_component = ComponentBuilder::default();

    // (1) Embed the component to test.
    let component_to_test_idx = root_component.component_raw(component_to_test);

    // (2) Encode the world to a component type and embed a new component which
    // imports the encoded component type.
    let test_component_idx = {
        let component_ty = encode_world(resolve, world)?;
        let mut component = ComponentBuilder::default();
        let component_ty_idx = component.type_component(&component_ty);
        component.import(
            &resolve.worlds[world].name,
            ComponentTypeRef::Component(component_ty_idx),
        );
        root_component.component(component)
    };

    // (3) Instantiate the component from (2) with the component to test from (1).
    let args: Vec<(String, ComponentExportKind, u32)> = vec![(
        resolve.worlds[world].name.clone(),
        ComponentExportKind::Component,
        component_to_test_idx,
    )];
    root_component.instantiate(test_component_idx, args);

    let bytes = root_component.finish();

    Validator::new()
        .validate_all(&bytes)
        .context("failed to validate encoded bytes")?;

    Ok(())
}
