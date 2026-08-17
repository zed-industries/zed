use crate::{
    ComponentEncoder, StringEncoding, dummy_module, embed_component_metadata,
    encoding::encode_world,
};
use anyhow::{Context, Result, bail};
use wasm_encoder::{ComponentBuilder, ComponentExportKind, ComponentTypeRef};
use wasmparser::Validator;
use wit_parser::{ManglingAndAbi, Resolve, WorldId};

/// Tests whether `new` is a semver-compatible upgrade from the world `prev`.
///
/// This function is will test whether a WIT-level semver-compatible predicate
/// holds. Internally this will ignore all versions associated with packages and
/// will instead test for structural equality between types and such. For
/// example `new` is allowed to have more imports and fewer exports, but types
/// and such must have the exact same structure otherwise (e.g. function params,
/// record fields, etc).
//
// NB: the general implementation strategy here is similar to the `targets`
// function where components are synthesized and we effectively rely on
// wasmparser to figure out everything for us. Specifically what happens is:
//
// 1. A dummy component representing `prev` is created.
// 2. A component importing a component of shape `new` is created.
// 3. The component from (2) is instantiated with the component from (1).
//
// If that all type-checks and is valid then the semver compatible predicate
// holds. Otherwise something has gone wrong.
//
// Note that this does not produce great error messages, so this implementation
// likely wants to be improved in the future.
pub fn semver_check(mut resolve: Resolve, prev: WorldId, new: WorldId) -> Result<()> {
    // First up clear out all version information. This is required to ensure
    // that the strings line up for wasmparser's validation which does exact
    // string matching.
    //
    // This can leave `resolve` in a weird state which is why this function
    // takes ownership of `resolve`. Specifically the internal maps for package
    // names won't be updated and additionally this could leave two packages
    // with the same name (e.g. no version) which would be a bit odd.
    //
    // NB: this will probably cause confusing errors below if a world imports
    // two versions of a package's interfaces. I think that'll result in weird
    // wasmparser validation errors as there would be two imports of the same
    // name for example.
    for (_id, pkg) in resolve.packages.iter_mut() {
        pkg.name.version = None;
    }

    let old_pkg_id = resolve.worlds[prev]
        .package
        .context("old world not in named package")?;
    let old_pkg_name = &resolve.packages[old_pkg_id].name;
    let new_pkg_id = resolve.worlds[new]
        .package
        .context("new world not in named package")?;
    let new_pkg_name = &resolve.packages[new_pkg_id].name;
    if old_pkg_name != new_pkg_name {
        bail!(
            "the old world is in package {old_pkg_name}, which is not the same as the new world, which is in package {new_pkg_name}",
        )
    }

    // Component that will be validated at the end.
    let mut root_component = ComponentBuilder::default();

    // (1) above - create a dummy component which has the shape of `prev`.
    let mut prev_as_module = dummy_module(&resolve, prev, ManglingAndAbi::Standard32);
    embed_component_metadata(&mut prev_as_module, &resolve, prev, StringEncoding::UTF8)
        .context("failed to embed component metadata")?;
    let prev_as_component = ComponentEncoder::default()
        .module(&prev_as_module)
        .context("failed to register previous world encoded as a module")?
        .encode()
        .context("failed to encode previous world as a component")?;
    let component_to_test_idx = root_component.component_raw(None, &prev_as_component);

    // (2) above - create a component which imports a component of the shape of
    // `new`.
    let test_component_idx = {
        let component_ty =
            encode_world(&resolve, new).context("failed to encode the new world as a type")?;
        let mut component = ComponentBuilder::default();
        let component_ty_idx = component.type_component(None, &component_ty);
        component.import(
            &resolve.worlds[new].name,
            ComponentTypeRef::Component(component_ty_idx),
        );
        root_component.component(None, component)
    };

    // (3) Instantiate the component from (2) with the component to test from (1).
    root_component.instantiate(
        None,
        test_component_idx,
        [(
            resolve.worlds[new].name.clone(),
            ComponentExportKind::Component,
            component_to_test_idx,
        )],
    );
    let bytes = root_component.finish();

    // The final step is validating that this component is indeed valid. If any
    // error message is produced here an attempt is made to make it more
    // understandable but there's only but so good these errors can be with this
    // strategy.
    Validator::new()
        .validate_all(&bytes)
        .context("new world is not semver-compatible with the previous world")?;

    Ok(())
}
