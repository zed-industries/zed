//! Definitions of compilation artifacts of the component compilation process
//! which are serialized with `bincode` into output ELF files.

use crate::{
    CompiledModuleInfo, FunctionLoc, PrimaryMap, StaticModuleIndex,
    component::{Component, ComponentTypes, TrampolineIndex, TypeComponentIndex},
};
use serde_derive::{Deserialize, Serialize};

/// Serializable state that's stored in a compilation artifact.
#[derive(Serialize, Deserialize)]
pub struct ComponentArtifacts {
    /// The type of this component.
    pub ty: TypeComponentIndex,
    /// Information all kept available at runtime as-is.
    pub info: CompiledComponentInfo,
    /// Type information for this component and all contained modules.
    pub types: ComponentTypes,
    /// Serialized metadata about all included core wasm modules.
    pub static_modules: PrimaryMap<StaticModuleIndex, CompiledModuleInfo>,
}

/// Runtime state that a component retains to support its operation.
#[derive(Serialize, Deserialize)]
pub struct CompiledComponentInfo {
    /// Type information calculated during translation about this component.
    pub component: Component,

    /// Where lowered function trampolines are located within the `text`
    /// section of `code_memory`.
    ///
    /// These are the
    ///
    /// 1. Wasm-call,
    /// 2. array-call
    ///
    /// function pointers that end up in a `VMFuncRef` for each
    /// lowering.
    pub trampolines: PrimaryMap<TrampolineIndex, AllCallFunc<FunctionLoc>>,

    /// The location of the wasm-to-array trampoline for the `resource.drop`
    /// intrinsic.
    pub resource_drop_wasm_to_array_trampoline: Option<FunctionLoc>,
}

/// A triple of related functions/trampolines variants with differing calling
/// conventions: `{wasm,array}_call`.
///
/// Generic so we can use this with either the `Box<dyn Any + Send>`s that
/// implementations of the compiler trait return or with `FunctionLoc`s inside
/// an object file, for example.
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct AllCallFunc<T> {
    /// The function exposing the Wasm calling convention.
    pub wasm_call: T,
    /// The function exposing the array calling convention.
    pub array_call: T,
}

impl<T> AllCallFunc<T> {
    /// Map an `AllCallFunc<T>` into an `AllCallFunc<U>`.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> AllCallFunc<U> {
        AllCallFunc {
            wasm_call: f(self.wasm_call),
            array_call: f(self.array_call),
        }
    }
}
