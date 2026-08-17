use crate::{code_memory::CodeMemory, type_registry::TypeCollection};
use alloc::sync::Arc;
use wasmtime_environ::ModuleTypes;
#[cfg(feature = "component-model")]
use wasmtime_environ::component::ComponentTypes;

/// Metadata in Wasmtime about a loaded compiled artifact in memory which is
/// ready to execute.
///
/// This structure is used in both `Module` and `Component`. For components it's
/// notably shared amongst the core wasm modules within a component and the
/// component itself. For core wasm modules this is uniquely owned within a
/// `Module`.
pub struct CodeObject {
    /// Actual underlying mmap which is executable and contains other compiled
    /// information.
    ///
    /// Note the `Arc` here is used to share this with `CompiledModule` and the
    /// global module registry of traps. While probably not strictly necessary
    /// and could be avoided with some refactorings is a hopefully a relatively
    /// minor `Arc` for now.
    mmap: Arc<CodeMemory>,

    /// Registered shared signature for the loaded object.
    ///
    /// Note that this type has a significant destructor which unregisters
    /// signatures within the `Engine` it was originally tied to, and this ends
    /// up corresponding to the lifetime of a `Component` or `Module`.
    signatures: TypeCollection,

    /// Type information for the loaded object.
    ///
    /// This is either a `ModuleTypes` or a `ComponentTypes` depending on the
    /// top-level creator of this code.
    types: Types,
}

impl CodeObject {
    pub fn new(mmap: Arc<CodeMemory>, signatures: TypeCollection, types: Types) -> CodeObject {
        // The corresponding unregister for this is below in `Drop for
        // CodeObject`.
        crate::module::register_code(&mmap);

        CodeObject {
            mmap,
            signatures,
            types,
        }
    }

    pub fn code_memory(&self) -> &Arc<CodeMemory> {
        &self.mmap
    }

    #[cfg(feature = "component-model")]
    pub fn types(&self) -> &Types {
        &self.types
    }

    pub fn module_types(&self) -> &ModuleTypes {
        self.types.module_types()
    }

    pub fn signatures(&self) -> &TypeCollection {
        &self.signatures
    }
}

impl Drop for CodeObject {
    fn drop(&mut self) {
        crate::module::unregister_code(&self.mmap);
    }
}

pub enum Types {
    Module(ModuleTypes),
    #[cfg(feature = "component-model")]
    Component(Arc<ComponentTypes>),
}

impl Types {
    fn module_types(&self) -> &ModuleTypes {
        match self {
            Types::Module(m) => m,
            #[cfg(feature = "component-model")]
            Types::Component(c) => c.module_types(),
        }
    }
}

impl From<ModuleTypes> for Types {
    fn from(types: ModuleTypes) -> Types {
        Types::Module(types)
    }
}

#[cfg(feature = "component-model")]
impl From<Arc<ComponentTypes>> for Types {
    fn from(types: Arc<ComponentTypes>) -> Types {
        Types::Component(types)
    }
}
