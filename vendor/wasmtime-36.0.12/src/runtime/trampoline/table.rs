use crate::TableType;
use crate::prelude::*;
use crate::runtime::vm::{Imports, ModuleRuntimeInfo, OnDemandInstanceAllocator};
use crate::store::{AllocateInstanceKind, InstanceId, StoreOpaque};
use alloc::sync::Arc;
use wasmtime_environ::{EntityIndex, Module, TypeTrace};

pub fn create_table(store: &mut StoreOpaque, table: &TableType) -> Result<InstanceId> {
    let mut module = Module::new();

    let wasmtime_table = *table.wasmtime_table();

    debug_assert!(
        wasmtime_table.ref_type.is_canonicalized_for_runtime_usage(),
        "should be canonicalized for runtime usage: {:?}",
        wasmtime_table.ref_type
    );

    let table_id = module.tables.push(wasmtime_table);

    // TODO: can this `exports.insert` get removed?
    module
        .exports
        .insert(String::new(), EntityIndex::Table(table_id));

    let imports = Imports::default();

    unsafe {
        let allocator =
            OnDemandInstanceAllocator::new(store.engine().config().mem_creator.clone(), 0, false);
        let module = Arc::new(module);
        store.allocate_instance(
            AllocateInstanceKind::Dummy {
                allocator: &allocator,
            },
            &ModuleRuntimeInfo::bare_with_registered_type(
                module,
                table.element().clone().into_registered_type(),
            ),
            imports,
        )
    }
}
