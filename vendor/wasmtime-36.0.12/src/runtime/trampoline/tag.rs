use crate::TagType;
use crate::prelude::*;
use crate::runtime::vm::{Imports, ModuleRuntimeInfo, OnDemandInstanceAllocator};
use crate::store::{AllocateInstanceKind, InstanceId, StoreOpaque};
use alloc::sync::Arc;
use wasmtime_environ::EngineOrModuleTypeIndex;
use wasmtime_environ::Tag;
use wasmtime_environ::{EntityIndex, Module, TypeTrace};

pub fn create_tag(store: &mut StoreOpaque, ty: &TagType) -> Result<InstanceId> {
    let mut module = Module::new();
    let func_ty = ty.ty().clone().into_registered_type();

    debug_assert!(
        func_ty.is_canonicalized_for_runtime_usage(),
        "should be canonicalized for runtime usage: {func_ty:?}",
    );

    let tag_id = module.tags.push(Tag {
        signature: EngineOrModuleTypeIndex::Engine(func_ty.index()),
    });

    module
        .exports
        .insert(String::new(), EntityIndex::Tag(tag_id));

    let imports = Imports::default();

    unsafe {
        let allocator =
            OnDemandInstanceAllocator::new(store.engine().config().mem_creator.clone(), 0, false);
        let module = Arc::new(module);
        store.allocate_instance(
            AllocateInstanceKind::Dummy {
                allocator: &allocator,
            },
            &ModuleRuntimeInfo::bare_with_registered_type(module, Some(func_ty)),
            imports,
        )
    }
}
