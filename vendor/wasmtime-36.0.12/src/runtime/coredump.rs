use crate::hash_map::HashMap;
use crate::prelude::*;
use crate::{
    AsContextMut, FrameInfo, Global, HeapType, Instance, Memory, Module, StoreContextMut, Val,
    ValType, WasmBacktrace, store::StoreOpaque,
};
use std::fmt;

/// Representation of a core dump of a WebAssembly module
///
/// When the Config::coredump_on_trap option is enabled this structure is
/// attached to the [`anyhow::Error`] returned from many Wasmtime functions that
/// execute WebAssembly such as [`Instance::new`] or [`Func::call`]. This can be
/// acquired with the [`anyhow::Error::downcast`] family of methods to
/// programmatically inspect the coredump. Otherwise since it's part of the
/// error returned this will get printed along with the rest of the error when
/// the error is logged.
///
/// Note that some state, such as Wasm locals or values on the operand stack,
/// may be optimized away by the compiler or otherwise not recovered in the
/// coredump.
///
/// Capturing of wasm coredumps can be configured through the
/// [`Config::coredump_on_trap`][crate::Config::coredump_on_trap] method.
///
/// For more information about errors in wasmtime see the documentation of the
/// [`Trap`][crate::Trap] type.
///
/// [`Func::call`]: crate::Func::call
/// [`Instance::new`]: crate::Instance::new
pub struct WasmCoreDump {
    name: String,
    modules: Vec<Module>,
    instances: Vec<Instance>,
    memories: Vec<Memory>,
    globals: Vec<Global>,
    backtrace: WasmBacktrace,
}

impl WasmCoreDump {
    pub(crate) fn new(store: &mut StoreOpaque, backtrace: WasmBacktrace) -> WasmCoreDump {
        let modules: Vec<_> = store.modules().all_modules().cloned().collect();
        let instances: Vec<Instance> = store.all_instances().collect();
        let mut store_memories: Vec<Memory> = store.all_memories().collect();
        store_memories.retain(|m| !m.wasmtime_ty(store).shared);

        let mut store_globals: Vec<Global> = vec![];
        store.for_each_global(|_store, global| store_globals.push(global));

        WasmCoreDump {
            name: String::from("store_name"),
            modules,
            instances,
            memories: store_memories,
            globals: store_globals,
            backtrace,
        }
    }

    /// The stack frames for this core dump.
    ///
    /// Frames appear in callee to caller order, that is youngest to oldest
    /// frames.
    pub fn frames(&self) -> &[FrameInfo] {
        self.backtrace.frames()
    }

    /// All modules instantiated inside the store when the core dump was
    /// created.
    pub fn modules(&self) -> &[Module] {
        self.modules.as_ref()
    }

    /// All instances within the store when the core dump was created.
    pub fn instances(&self) -> &[Instance] {
        self.instances.as_ref()
    }

    /// All globals, instance- or host-defined, within the store when the core
    /// dump was created.
    pub fn globals(&self) -> &[Global] {
        self.globals.as_ref()
    }

    /// All memories, instance- or host-defined, within the store when the core
    /// dump was created.
    pub fn memories(&self) -> &[Memory] {
        self.memories.as_ref()
    }

    /// Serialize this core dump into [the standard core dump binary
    /// format][spec].
    ///
    /// The `name` parameter may be a file path, URL, or arbitrary name for the
    /// "main" Wasm service or executable that was running in this store.
    ///
    /// Once serialized, you can write this core dump to disk, send it over the
    /// network, or pass it to other debugging tools that consume Wasm core
    /// dumps.
    ///
    /// [spec]: https://github.com/WebAssembly/tool-conventions/blob/main/Coredump.md
    pub fn serialize(&self, mut store: impl AsContextMut, name: &str) -> Vec<u8> {
        let store = store.as_context_mut();
        self._serialize(store, name)
    }

    fn _serialize<T: 'static>(&self, mut store: StoreContextMut<'_, T>, name: &str) -> Vec<u8> {
        let mut core_dump = wasm_encoder::Module::new();

        core_dump.section(&wasm_encoder::CoreDumpSection::new(name));

        // A map from each memory to its index in the core dump's memories
        // section.
        let mut memory_to_idx = HashMap::new();

        let mut data = wasm_encoder::DataSection::new();

        {
            let mut memories = wasm_encoder::MemorySection::new();
            for mem in self.memories() {
                let memory_idx = memories.len();
                memory_to_idx.insert(mem.hash_key(&store.0), memory_idx);
                let ty = mem.ty(&store);
                memories.memory(wasm_encoder::MemoryType {
                    minimum: mem.size(&store),
                    maximum: ty.maximum(),
                    memory64: ty.is_64(),
                    shared: ty.is_shared(),
                    page_size_log2: None,
                });

                // Attach the memory data, balancing number of data segments and
                // binary size. We don't want to attach the whole memory in one
                // big segment, since it likely contains a bunch of large runs
                // of zeroes. But we can't encode the data without any potential
                // runs of zeroes (i.e. including only non-zero data in our
                // segments) because we can run up against the implementation
                // limits for number of segments in a Wasm module this way. So
                // to balance these conflicting desires, we break the memory up
                // into reasonably-sized chunks and then trim runs of zeroes
                // from the start and end of each chunk.
                const CHUNK_SIZE: usize = 4096;
                for (i, chunk) in mem.data(&store).chunks_exact(CHUNK_SIZE).enumerate() {
                    if let Some(start) = chunk.iter().position(|byte| *byte != 0) {
                        let end = chunk.iter().rposition(|byte| *byte != 0).unwrap() + 1;
                        let offset = i * CHUNK_SIZE + start;
                        let offset = if ty.is_64() {
                            let offset = u64::try_from(offset).unwrap();
                            wasm_encoder::ConstExpr::i64_const(offset as i64)
                        } else {
                            let offset = u32::try_from(offset).unwrap();
                            wasm_encoder::ConstExpr::i32_const(offset as i32)
                        };
                        data.active(memory_idx, &offset, chunk[start..end].iter().copied());
                    }
                }
            }
            core_dump.section(&memories);
        }

        // A map from each global to its index in the core dump's globals
        // section.
        let mut global_to_idx = HashMap::new();

        {
            let mut globals = wasm_encoder::GlobalSection::new();
            for g in self.globals() {
                global_to_idx.insert(g.hash_key(&store.0), globals.len());
                let ty = g.ty(&store);
                let mutable = matches!(ty.mutability(), crate::Mutability::Var);
                let val_type = match ty.content() {
                    ValType::I32 => wasm_encoder::ValType::I32,
                    ValType::I64 => wasm_encoder::ValType::I64,
                    ValType::F32 => wasm_encoder::ValType::F32,
                    ValType::F64 => wasm_encoder::ValType::F64,
                    ValType::V128 => wasm_encoder::ValType::V128,

                    // We encode all references as null in the core dump, so
                    // choose the common super type of all the actual function
                    // reference types. This lets us avoid needing to figure out
                    // what a concrete type reference's index is in the local
                    // core dump index space.
                    ValType::Ref(r) => match r.heap_type().top() {
                        HeapType::Extern => wasm_encoder::ValType::EXTERNREF,

                        HeapType::Func => wasm_encoder::ValType::FUNCREF,

                        HeapType::Any => wasm_encoder::ValType::Ref(wasm_encoder::RefType::ANYREF),

                        ty => unreachable!("not a top type: {ty:?}"),
                    },
                };
                let init = match g.get(&mut store) {
                    Val::I32(x) => wasm_encoder::ConstExpr::i32_const(x),
                    Val::I64(x) => wasm_encoder::ConstExpr::i64_const(x),
                    Val::F32(x) => wasm_encoder::ConstExpr::f32_const(f32::from_bits(x).into()),
                    Val::F64(x) => wasm_encoder::ConstExpr::f64_const(f64::from_bits(x).into()),
                    Val::V128(x) => wasm_encoder::ConstExpr::v128_const(x.as_u128() as i128),
                    Val::FuncRef(_) => {
                        wasm_encoder::ConstExpr::ref_null(wasm_encoder::HeapType::FUNC)
                    }
                    Val::ExternRef(_) => {
                        wasm_encoder::ConstExpr::ref_null(wasm_encoder::HeapType::EXTERN)
                    }
                    Val::AnyRef(_) => {
                        wasm_encoder::ConstExpr::ref_null(wasm_encoder::HeapType::ANY)
                    }
                    Val::ExnRef(_) => {
                        wasm_encoder::ConstExpr::ref_null(wasm_encoder::HeapType::Abstract {
                            shared: false,
                            ty: wasm_encoder::AbstractHeapType::Exn,
                        })
                    }
                };
                globals.global(
                    wasm_encoder::GlobalType {
                        val_type,
                        mutable,
                        shared: false,
                    },
                    &init,
                );
            }
            core_dump.section(&globals);
        }

        core_dump.section(&data);
        drop(data);

        // A map from module id to its index within the core dump's modules
        // section.
        let mut module_to_index = HashMap::new();

        {
            let mut modules = wasm_encoder::CoreDumpModulesSection::new();
            for module in self.modules() {
                module_to_index.insert(module.id(), modules.len());
                match module.name() {
                    Some(name) => modules.module(name),
                    None => modules.module(&format!("<anonymous-module-{}>", modules.len())),
                };
            }
            core_dump.section(&modules);
        }

        // TODO: We can't currently recover instances from stack frames. We can
        // recover module via the frame's PC, but if there are multiple
        // instances of the same module, we don't know which instance the frame
        // is associated with. Therefore, we do a best effort job: remember the
        // last instance of each module and always choose that one. We record
        // that information here.
        let mut module_to_instance = HashMap::new();

        {
            let mut instances = wasm_encoder::CoreDumpInstancesSection::new();
            for instance in self.instances() {
                let module = instance.module(&store);
                module_to_instance.insert(module.id(), instances.len());

                let module_index = module_to_index[&module.id()];

                let memories = instance
                    .all_memories(store.0)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|(_i, memory)| {
                        memory_to_idx
                            .get(&memory.hash_key(&store.0))
                            .copied()
                            .unwrap_or(u32::MAX)
                    })
                    .collect::<Vec<_>>();

                let globals = instance
                    .all_globals(store.0)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|(_i, global)| global_to_idx[&global.hash_key(&store.0)])
                    .collect::<Vec<_>>();

                instances.instance(module_index, memories, globals);
            }
            core_dump.section(&instances);
        }

        {
            let thread_name = "main";
            let mut stack = wasm_encoder::CoreDumpStackSection::new(thread_name);
            for frame in self.frames() {
                // This isn't necessarily the right instance if there are
                // multiple instances of the same module. See comment above
                // `module_to_instance` for details.
                let instance = module_to_instance[&frame.module().id()];

                let func = frame.func_index();

                let offset = frame
                    .func_offset()
                    .and_then(|o| u32::try_from(o).ok())
                    .unwrap_or(0);

                // We can't currently recover locals and the operand stack. We
                // should eventually be able to do that with Winch though.
                let locals = [];
                let operand_stack = [];

                stack.frame(instance, func, offset, locals, operand_stack);
            }
            core_dump.section(&stack);
        }

        core_dump.finish()
    }
}

impl fmt::Display for WasmCoreDump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "wasm coredump generated while executing {}:", self.name)?;
        writeln!(f, "modules:")?;
        for module in self.modules.iter() {
            writeln!(f, "  {}", module.name().unwrap_or("<module>"))?;
        }

        writeln!(f, "instances:")?;
        for instance in self.instances.iter() {
            writeln!(f, "  {instance:?}")?;
        }

        writeln!(f, "memories:")?;
        for memory in self.memories.iter() {
            writeln!(f, "  {memory:?}")?;
        }

        writeln!(f, "globals:")?;
        for global in self.globals.iter() {
            writeln!(f, "  {global:?}")?;
        }

        writeln!(f, "backtrace:")?;
        write!(f, "{}", self.backtrace)?;

        Ok(())
    }
}

impl fmt::Debug for WasmCoreDump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<wasm core dump>")
    }
}
