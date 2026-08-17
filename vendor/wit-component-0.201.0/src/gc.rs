use self::bitvec::BitVec;
use anyhow::{bail, Result};
use indexmap::{IndexMap, IndexSet};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    mem,
    ops::Deref,
};
use wasm_encoder::{Encode, EntityType, Instruction, RawCustomSection};
use wasmparser::*;

const PAGE_SIZE: i32 = 64 * 1024;

/// This function will reduce the input core `wasm` module to only the set of
/// exports `required`.
///
/// This internally performs a "gc" pass after removing exports to ensure that
/// the resulting module imports the minimal set of functions necessary.
pub fn run<T>(
    wasm: &[u8],
    required: &IndexMap<String, T>,
    main_module_realloc: Option<&str>,
) -> Result<Vec<u8>> {
    assert!(!required.is_empty());

    let mut module = Module::default();
    module.parse(wasm)?;

    // Make sure that all required names are present in the module, and then
    // remove all names that are not required.
    for (name, _ty) in required {
        if !module.exports.contains_key(name.as_str()) {
            bail!("adapter module does not have export `{name}`")
        }
    }
    let mut not_required = IndexSet::new();
    for name in module.exports.keys().copied() {
        // If we need `name` then we also need cabi_post_`name`:
        let name = if let Some(suffix) = name.strip_prefix("cabi_post_") {
            suffix
        } else {
            name
        };

        if !required.contains_key(name) && !always_keep(name) {
            not_required.insert(name);
        }
    }
    for name in not_required {
        module.exports.remove(name);
    }
    assert!(!module.exports.is_empty());
    module.liveness()?;
    module.encode(main_module_realloc)
}

fn always_keep(name: &str) -> bool {
    match name {
        // Explicitly keep `cabi_realloc` if it's there in case an interface
        // needs it for a lowering.
        "cabi_realloc" | "cabi_import_realloc" | "cabi_export_realloc" => true,
        _ => false,
    }
}

/// This function generates a Wasm function body which implements `cabi_realloc` in terms of `memory.grow`.  It
/// only accepts new, page-sized allocations.
fn realloc_via_memory_grow() -> wasm_encoder::Function {
    use wasm_encoder::Instruction::*;

    let mut func = wasm_encoder::Function::new([(1, wasm_encoder::ValType::I32)]);

    // Assert `old_ptr` is null.
    func.instruction(&I32Const(0));
    func.instruction(&LocalGet(0));
    func.instruction(&I32Ne);
    func.instruction(&If(wasm_encoder::BlockType::Empty));
    func.instruction(&Unreachable);
    func.instruction(&End);

    // Assert `old_len` is zero.
    func.instruction(&I32Const(0));
    func.instruction(&LocalGet(1));
    func.instruction(&I32Ne);
    func.instruction(&If(wasm_encoder::BlockType::Empty));
    func.instruction(&Unreachable);
    func.instruction(&End);

    // Assert `new_len` is equal to the page size (which is the only value we currently support)
    // Note: we could easily support arbitrary multiples of PAGE_SIZE here if the need arises.
    func.instruction(&I32Const(PAGE_SIZE));
    func.instruction(&LocalGet(3));
    func.instruction(&I32Ne);
    func.instruction(&If(wasm_encoder::BlockType::Empty));
    func.instruction(&Unreachable);
    func.instruction(&End);

    // Grow the memory by 1 page.
    func.instruction(&I32Const(1));
    func.instruction(&MemoryGrow(0));
    func.instruction(&LocalTee(4));

    // Test if the return value of the growth was -1 and, if so, trap due to a failed allocation.
    func.instruction(&I32Const(-1));
    func.instruction(&I32Eq);
    func.instruction(&If(wasm_encoder::BlockType::Empty));
    func.instruction(&Unreachable);
    func.instruction(&End);

    func.instruction(&LocalGet(4));
    func.instruction(&I32Const(16));
    func.instruction(&I32Shl);
    func.instruction(&End);

    func
}

#[repr(i32)]
#[non_exhaustive]
enum StackAllocationState {
    Unallocated,
    Allocating,
    Allocated,
}

fn allocate_stack_via_realloc(
    realloc_index: u32,
    sp: u32,
    allocation_state: Option<u32>,
) -> wasm_encoder::Function {
    use wasm_encoder::Instruction::*;

    let mut func = wasm_encoder::Function::new([]);

    if let Some(allocation_state) = allocation_state {
        // This means we're lazily allocating the stack, keeping track of state via `$allocation_state`
        func.instruction(&GlobalGet(allocation_state));
        func.instruction(&I32Const(StackAllocationState::Unallocated as _));
        func.instruction(&I32Eq);
        func.instruction(&If(wasm_encoder::BlockType::Empty));
        func.instruction(&I32Const(StackAllocationState::Allocating as _));
        func.instruction(&GlobalSet(allocation_state));
        // We could also set `sp` to zero here to ensure the yet-to-be-allocated stack is empty.  However, we
        // assume it defaults to zero anyway, in which case setting it would be redundant.
    }

    func.instruction(&I32Const(0));
    func.instruction(&I32Const(0));
    func.instruction(&I32Const(8));
    func.instruction(&I32Const(PAGE_SIZE));
    func.instruction(&Call(realloc_index));
    func.instruction(&I32Const(PAGE_SIZE));
    func.instruction(&I32Add);
    func.instruction(&GlobalSet(sp));

    if let Some(allocation_state) = allocation_state {
        func.instruction(&I32Const(StackAllocationState::Allocated as _));
        func.instruction(&GlobalSet(allocation_state));
        func.instruction(&End);
    }

    func.instruction(&End);

    func
}

// Represents a function called while processing a module work list.
type WorklistFunc<'a> = fn(&mut Module<'a>, u32) -> Result<()>;

// Representation of a wasm module which is used to GC a module to its minimal
// set of required items necessary to implement the `exports`
//
// Note that this is not a complete representation of a wasm module since it
// doesn't represent everything such as data and element segments. This is only
// used for adapter modules which otherwise have these restrictions and makes
// this gc pass a bit easier to write.
#[derive(Default)]
struct Module<'a> {
    // Definitions found when parsing a module
    types: Vec<FuncType>,
    tables: Vec<Table<'a>>,
    globals: Vec<Global<'a>>,
    memories: Vec<Memory<'a>>,
    funcs: Vec<Func<'a>>,
    exports: IndexMap<&'a str, Export<'a>>,
    func_names: HashMap<u32, &'a str>,
    global_names: HashMap<u32, &'a str>,
    producers: Option<wasm_metadata::Producers>,

    // Known-live sets of indices after the `liveness` pass has run.
    live_types: BitVec,
    live_tables: BitVec,
    live_globals: BitVec,
    live_memories: BitVec,
    live_funcs: BitVec,

    // Helper data structure used during the `liveness` path to avoid recursion.
    // When calculating the liveness of an item this `worklist` is pushed to and
    // then processed until it's empty. An item pushed onto this list represents
    // a new index that has been discovered to be live and the function is what
    // walks the item's definition to find other items that it references.
    worklist: Vec<(u32, WorklistFunc<'a>)>,
}

struct Table<'a> {
    def: Definition<'a, ()>,
    ty: TableType,
}

struct Memory<'a> {
    def: Definition<'a, ()>,
    ty: MemoryType,
}

struct Global<'a> {
    def: Definition<'a, ConstExpr<'a>>,
    ty: GlobalType,
}

#[derive(Clone)]
struct Func<'a> {
    def: Definition<'a, FunctionBody<'a>>,
    ty: u32,
}

#[derive(Clone)]
enum Definition<'a, T> {
    Import(&'a str, &'a str),
    Local(T),
}

impl<'a> Module<'a> {
    fn parse(&mut self, wasm: &'a [u8]) -> Result<()> {
        let mut next_code_index = 0;
        let mut validator = Validator::new();
        for payload in Parser::new(0).parse_all(wasm) {
            let payload = payload?;
            validator.payload(&payload)?;
            match payload {
                Payload::Version { encoding, .. } => {
                    if encoding != Encoding::Module {
                        bail!("adapter must be a core wasm module, not a component");
                    }
                }
                Payload::End(_) => {}
                Payload::TypeSection(s) => {
                    for ty in s.into_iter_err_on_gc_types() {
                        self.types.push(ty?);
                    }
                }
                Payload::ImportSection(s) => {
                    for i in s {
                        let i = i?;
                        match i.ty {
                            TypeRef::Func(ty) => self.funcs.push(Func {
                                def: Definition::Import(i.module, i.name),
                                ty,
                            }),
                            TypeRef::Table(ty) => self.tables.push(Table {
                                def: Definition::Import(i.module, i.name),
                                ty,
                            }),
                            TypeRef::Global(ty) => self.globals.push(Global {
                                def: Definition::Import(i.module, i.name),
                                ty,
                            }),
                            TypeRef::Memory(ty) => self.memories.push(Memory {
                                def: Definition::Import(i.module, i.name),
                                ty,
                            }),
                            TypeRef::Tag(_) => bail!("unsupported `tag` type"),
                        }
                    }
                }
                Payload::TableSection(s) => {
                    for table in s {
                        let table = table?;
                        self.tables.push(Table {
                            def: Definition::Local(()),
                            ty: table.ty,
                        });
                    }
                }
                Payload::MemorySection(s) => {
                    for ty in s {
                        let ty = ty?;
                        self.memories.push(Memory {
                            def: Definition::Local(()),
                            ty,
                        });
                    }
                }
                Payload::GlobalSection(s) => {
                    for g in s {
                        let g = g?;
                        self.globals.push(Global {
                            def: Definition::Local(g.init_expr),
                            ty: g.ty,
                        });
                    }
                }

                Payload::ExportSection(s) => {
                    for e in s {
                        let e = e?;
                        self.exports.insert(e.name, e);
                    }
                }

                Payload::FunctionSection(s) => {
                    next_code_index = self.funcs.len();
                    for ty in s {
                        let ty = ty?;
                        self.funcs.push(Func {
                            // Specify a dummy definition to get filled in later
                            // when parsing the code section.
                            def: Definition::Local(FunctionBody::new(0, &[])),
                            ty,
                        });
                    }
                }

                Payload::CodeSectionStart { .. } => {}
                Payload::CodeSectionEntry(body) => {
                    self.funcs[next_code_index].def = Definition::Local(body);
                    next_code_index += 1;
                }

                // Ignore all custom sections except for the `name` and
                // `producers` sections which we parse, but ignore errors within.
                Payload::CustomSection(s) => {
                    if s.name() == "name" {
                        drop(self.parse_name_section(&s));
                    }
                    if s.name() == "producers" {
                        drop(self.parse_producers_section(&s));
                    }
                }

                // sections that shouldn't appear in the specially-crafted core wasm
                // adapter self we're processing
                Payload::DataCountSection { .. }
                | Payload::ElementSection(_)
                | Payload::DataSection(_)
                | Payload::StartSection { .. }
                | Payload::TagSection(_)
                | Payload::UnknownSection { .. } => {
                    bail!("unsupported section found in adapter module")
                }

                // component-model related things that shouldn't show up
                Payload::ModuleSection { .. }
                | Payload::ComponentSection { .. }
                | Payload::InstanceSection(_)
                | Payload::ComponentInstanceSection(_)
                | Payload::ComponentAliasSection(_)
                | Payload::ComponentCanonicalSection(_)
                | Payload::ComponentStartSection { .. }
                | Payload::ComponentImportSection(_)
                | Payload::CoreTypeSection(_)
                | Payload::ComponentExportSection(_)
                | Payload::ComponentTypeSection(_) => {
                    bail!("component section found in adapter module")
                }
            }
        }

        Ok(())
    }

    fn parse_name_section(&mut self, section: &CustomSectionReader<'a>) -> Result<()> {
        let section = NameSectionReader::new(section.data(), section.data_offset());
        for s in section {
            match s? {
                Name::Function(map) => {
                    for naming in map {
                        let naming = naming?;
                        self.func_names.insert(naming.index, naming.name);
                    }
                }
                Name::Global(map) => {
                    for naming in map {
                        let naming = naming?;
                        self.global_names.insert(naming.index, naming.name);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_producers_section(&mut self, section: &CustomSectionReader<'a>) -> Result<()> {
        let producers =
            wasm_metadata::Producers::from_bytes(section.data(), section.data_offset())?;
        self.producers = Some(producers);
        Ok(())
    }

    /// Iteratively calculates the set of live items within this module
    /// considering all exports as the root of live functions.
    fn liveness(&mut self) -> Result<()> {
        let exports = mem::take(&mut self.exports);
        for (_, e) in exports.iter() {
            match e.kind {
                ExternalKind::Func => self.func(e.index),
                ExternalKind::Global => self.global(e.index),
                ExternalKind::Table => self.table(e.index),
                ExternalKind::Memory => self.memory(e.index),
                ExternalKind::Tag => bail!("unsupported exported tag"),
            }
        }
        self.exports = exports;

        while let Some((idx, func)) = self.worklist.pop() {
            func(self, idx)?;
        }
        Ok(())
    }

    fn func(&mut self, func: u32) {
        if !self.live_funcs.insert(func) {
            return;
        }
        self.worklist.push((func, |me, func| {
            let func = me.funcs[func as usize].clone();
            me.ty(func.ty);
            let mut body = match &func.def {
                Definition::Import(..) => return Ok(()),
                Definition::Local(e) => e.get_binary_reader(),
            };
            let local_count = body.read_var_u32()?;
            for _ in 0..local_count {
                body.read_var_u32()?;
                body.read::<ValType>()?;
            }
            me.operators(body)
        }));
    }

    fn global(&mut self, global: u32) {
        if !self.live_globals.insert(global) {
            return;
        }
        self.worklist.push((global, |me, global| {
            let init = match &me.globals[global as usize].def {
                Definition::Import(..) => return Ok(()),
                Definition::Local(e) => e,
            };
            me.operators(init.get_binary_reader())
        }));
    }

    fn table(&mut self, table: u32) {
        if !self.live_tables.insert(table) {
            return;
        }
        self.worklist.push((table, |me, table| {
            let ty = me.tables[table as usize].ty.element_type;
            me.valty(ty.into());
            Ok(())
        }));
    }

    fn memory(&mut self, memory: u32) {
        self.live_memories.insert(memory);
    }

    fn blockty(&mut self, ty: BlockType) {
        if let BlockType::FuncType(ty) = ty {
            self.ty(ty);
        }
    }

    fn valty(&mut self, ty: ValType) {
        match ty {
            ValType::Ref(r) => self.refty(r),
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => {}
        }
    }

    fn refty(&mut self, ty: RefType) {
        self.heapty(ty.heap_type())
    }

    fn heapty(&mut self, ty: HeapType) {
        match ty {
            HeapType::Func
            | HeapType::Extern
            | HeapType::Any
            | HeapType::None
            | HeapType::NoExtern
            | HeapType::NoFunc
            | HeapType::Eq
            | HeapType::Struct
            | HeapType::Array
            | HeapType::I31
            | HeapType::Exn => {}
            HeapType::Concrete(i) => self.ty(i.as_module_index().unwrap()),
        }
    }

    fn ty(&mut self, ty: u32) {
        if !self.live_types.insert(ty) {
            return;
        }
        self.worklist.push((ty, |me, ty| {
            let ty = me.types[ty as usize].clone();
            for param in ty.params().iter().chain(ty.results()) {
                me.valty(*param);
            }
            Ok(())
        }));
    }

    fn operators(&mut self, mut reader: BinaryReader<'a>) -> Result<()> {
        while !reader.eof() {
            reader.visit_operator(self)?;
        }
        Ok(())
    }

    fn live_types(&self) -> impl Iterator<Item = (u32, &FuncType)> + '_ {
        live_iter(&self.live_types, self.types.iter())
    }

    fn live_funcs(&self) -> impl Iterator<Item = (u32, &Func<'a>)> + '_ {
        live_iter(&self.live_funcs, self.funcs.iter())
    }

    fn live_memories(&self) -> impl Iterator<Item = (u32, &Memory<'a>)> + '_ {
        live_iter(&self.live_memories, self.memories.iter())
    }

    fn live_globals(&self) -> impl Iterator<Item = (u32, &Global<'a>)> + '_ {
        live_iter(&self.live_globals, self.globals.iter())
    }

    fn live_tables(&self) -> impl Iterator<Item = (u32, &Table<'a>)> + '_ {
        live_iter(&self.live_tables, self.tables.iter())
    }

    /// Encodes this `Module` to a new wasm module which is gc'd and only
    /// contains the items that are live as calculated by the `liveness` pass.
    fn encode(&mut self, main_module_realloc: Option<&str>) -> Result<Vec<u8>> {
        // Data structure used to track the mapping of old index to new index
        // for all live items.
        let mut map = Encoder::default();

        // Sections that will be assembled into the final module at the end of
        // this function.
        let mut types = wasm_encoder::TypeSection::new();
        let mut imports = wasm_encoder::ImportSection::new();
        let mut funcs = wasm_encoder::FunctionSection::new();
        let mut tables = wasm_encoder::TableSection::new();
        let mut memories = wasm_encoder::MemorySection::new();
        let mut globals = wasm_encoder::GlobalSection::new();
        let mut code = wasm_encoder::CodeSection::new();

        let mut empty_type = None;
        for (i, ty) in self.live_types() {
            map.types.push(i);

            types.function(
                ty.params().iter().map(|t| map.valty(*t)),
                ty.results().iter().map(|t| map.valty(*t)),
            );

            // Keep track of the "empty type" to see if we can reuse an
            // existing one or one needs to be injected if a `start`
            // function is calculated at the end.
            if ty.params().is_empty() && ty.results().is_empty() {
                empty_type = Some(map.types.remap(i));
            }
        }

        let mut num_memories = 0;
        for (i, mem) in self.live_memories() {
            map.memories.push(i);
            let ty = wasm_encoder::MemoryType {
                minimum: mem.ty.initial,
                maximum: mem.ty.maximum,
                shared: mem.ty.shared,
                memory64: mem.ty.memory64,
            };
            match &mem.def {
                Definition::Import(m, n) => {
                    imports.import(m, n, ty);
                }
                Definition::Local(()) => {
                    memories.memory(ty);
                }
            }
            num_memories += 1;
        }

        for (i, table) in self.live_tables() {
            map.tables.push(i);
            let ty = wasm_encoder::TableType {
                minimum: table.ty.initial,
                maximum: table.ty.maximum,
                element_type: map.refty(table.ty.element_type),
            };
            match &table.def {
                Definition::Import(m, n) => {
                    imports.import(m, n, ty);
                }
                Definition::Local(()) => {
                    tables.table(ty);
                }
            }
        }

        for (i, global) in self.live_globals() {
            map.globals.push(i);
            let ty = wasm_encoder::GlobalType {
                mutable: global.ty.mutable,
                val_type: map.valty(global.ty.content_type),
            };
            match &global.def {
                Definition::Import(m, n) => {
                    imports.import(m, n, ty);
                }
                Definition::Local(init) => {
                    let mut bytes = map.operators(init.get_binary_reader())?;
                    assert_eq!(bytes.pop(), Some(0xb));
                    globals.global(ty, &wasm_encoder::ConstExpr::raw(bytes));
                }
            }
        }

        let mut realloc_index = None;
        let mut num_func_imports = 0;

        // For functions first assign a new index to all functions and then
        // afterwards actually map the body of all functions so the `map` of all
        // index mappings is fully populated before instructions are mapped.

        let is_realloc =
            |m, n| m == "__main_module__" && matches!(n, "canonical_abi_realloc" | "cabi_realloc");

        let (imported, local) =
            self.live_funcs()
                .partition::<Vec<_>, _>(|(_, func)| match &func.def {
                    Definition::Import(m, n) => {
                        !is_realloc(*m, *n) || main_module_realloc.is_some()
                    }
                    Definition::Local(_) => false,
                });

        for (i, func) in imported {
            map.funcs.push(i);
            let ty = map.types.remap(func.ty);
            match &func.def {
                Definition::Import(m, n) => {
                    let name = if is_realloc(*m, *n) {
                        // The adapter is importing `cabi_realloc` from the main module, and the main module
                        // exports that function, but possibly using a different name
                        // (e.g. `canonical_abi_realloc`).  Update the name to match if necessary.
                        realloc_index = Some(num_func_imports);
                        main_module_realloc.unwrap_or(n)
                    } else {
                        n
                    };
                    imports.import(m, name, EntityType::Function(ty));
                    num_func_imports += 1;
                }
                Definition::Local(_) => unreachable!(),
            }
        }

        let add_realloc_type = |types: &mut wasm_encoder::TypeSection| {
            let type_index = types.len();
            types.function(
                [
                    wasm_encoder::ValType::I32,
                    wasm_encoder::ValType::I32,
                    wasm_encoder::ValType::I32,
                    wasm_encoder::ValType::I32,
                ],
                [wasm_encoder::ValType::I32],
            );
            type_index
        };

        let add_empty_type = |types: &mut wasm_encoder::TypeSection| {
            let type_index = types.len();
            types.function([], []);
            type_index
        };

        let sp = self.find_mut_i32_global("__stack_pointer")?;
        let allocation_state = self.find_mut_i32_global("allocation_state")?;

        let mut func_names = Vec::new();

        if let (Some(realloc), Some(_), None) = (main_module_realloc, sp, realloc_index) {
            // The main module exports a realloc function, and although the adapter doesn't import it, we're going
            // to add a function which calls it to allocate some stack space, so let's add an import now.

            // Tell the function remapper we're reserving a slot for our extra import:
            map.funcs.next += 1;

            realloc_index = Some(num_func_imports);
            imports.import(
                "__main_module__",
                realloc,
                EntityType::Function(add_realloc_type(&mut types)),
            );
            func_names.push((num_func_imports, realloc));
            num_func_imports += 1;
        }

        for (i, func) in local {
            map.funcs.push(i);
            let ty = map.types.remap(func.ty);
            match &func.def {
                Definition::Import(_, _) => {
                    // The adapter is importing `cabi_realloc` from the main module, but the main module isn't
                    // exporting it.  In this case, we need to define a local function it can call instead.
                    realloc_index = Some(num_func_imports + funcs.len());
                    funcs.function(ty);
                    code.function(&realloc_via_memory_grow());
                }
                Definition::Local(_) => {
                    funcs.function(ty);
                }
            }
        }

        let lazy_stack_init_index =
            if sp.is_some() && allocation_state.is_some() && main_module_realloc.is_some() {
                // We have a stack pointer, a `cabi_realloc` function from the main module, and a global variable for
                // keeping track of (and short-circuiting) reentrance.  That means we can (and should) do lazy stack
                // allocation.
                let index = num_func_imports + funcs.len();

                // Tell the function remapper we're reserving a slot for our extra function:
                map.funcs.next += 1;

                funcs.function(add_empty_type(&mut types));

                Some(index)
            } else {
                None
            };

        let exported_funcs = self
            .exports
            .values()
            .filter_map(|export| match export.kind {
                ExternalKind::Func => Some(export.index),
                _ => None,
            })
            .collect::<HashSet<_>>();

        for (i, func) in self.live_funcs() {
            let mut body = match &func.def {
                Definition::Import(..) => continue,
                Definition::Local(body) => body.get_binary_reader(),
            };
            let mut locals = Vec::new();
            for _ in 0..body.read_var_u32()? {
                let cnt = body.read_var_u32()?;
                let ty = body.read()?;
                locals.push((cnt, map.valty(ty)));
            }
            // Prepend an `allocate_stack` call to all exports if we're lazily allocating the stack.
            if let (Some(lazy_stack_init_index), true) =
                (lazy_stack_init_index, exported_funcs.contains(&i))
            {
                Instruction::Call(lazy_stack_init_index).encode(&mut map.buf);
            }
            let bytes = map.operators(body)?;
            let mut func = wasm_encoder::Function::new(locals);
            func.raw(bytes);
            code.function(&func);
        }

        if lazy_stack_init_index.is_some() {
            code.function(&allocate_stack_via_realloc(
                realloc_index.unwrap(),
                sp.unwrap(),
                allocation_state,
            ));
        }

        if sp.is_some() && (realloc_index.is_none() || allocation_state.is_none()) {
            // Either the main module does _not_ export a realloc function, or it is not safe to use for stack
            // allocation because we have no way to short-circuit reentrance, so we'll use `memory.grow` instead.
            realloc_index = Some(num_func_imports + funcs.len());
            funcs.function(add_realloc_type(&mut types));
            code.function(&realloc_via_memory_grow());
        }

        // Inject a start function to initialize the stack pointer which will be local to this module. This only
        // happens if a memory is preserved, a stack pointer global is found, and we're not doing lazy stack
        // allocation.
        let mut start = None;
        if let (Some(sp), None) = (sp, lazy_stack_init_index) {
            if num_memories > 0 {
                // If there are any memories or any mutable globals there must be
                // precisely one of each as otherwise we don't know how to filter
                // down to the right one.
                if num_memories != 1 {
                    bail!("adapter modules don't support multi-memory");
                }

                let sp = map.globals.remap(sp);

                let function_index = num_func_imports + funcs.len();

                // Generate a function type for this start function, adding a new
                // function type to the module if necessary.
                let empty_type = empty_type.unwrap_or_else(|| {
                    types.function([], []);
                    types.len() - 1
                });
                funcs.function(empty_type);
                func_names.push((function_index, "allocate_stack"));
                code.function(&allocate_stack_via_realloc(
                    realloc_index.unwrap(),
                    sp,
                    allocation_state,
                ));

                start = Some(wasm_encoder::StartSection { function_index });
            }
        }

        // Sanity-check the shape of the module since some parts won't work if
        // this fails. Note that during parsing we've already validated there
        // are no data segments or element segments.

        // Shouldn't have any tables if there are no element segments since
        // otherwise there's no meaning to a defined or imported table.
        if self.live_tables().count() != 0 {
            bail!("tables should not be present in the final adapter module");
        }

        // multi-memory should not be enabled and if any memory it should be
        // imported.
        if self.live_memories().count() > 1 {
            bail!("the adapter module should not use multi-memory");
        }
        if !memories.is_empty() {
            bail!("locally-defined memories are not allowed define a local memory");
        }

        let mut ret = wasm_encoder::Module::default();
        if !types.is_empty() {
            ret.section(&types);
        }
        if !imports.is_empty() {
            ret.section(&imports);
        }
        if !funcs.is_empty() {
            ret.section(&funcs);
        }
        if !tables.is_empty() {
            ret.section(&tables);
        }
        if !memories.is_empty() {
            ret.section(&memories);
        }
        if !globals.is_empty() {
            ret.section(&globals);
        }

        if !self.exports.is_empty() {
            let mut exports = wasm_encoder::ExportSection::new();
            for (_, export) in self.exports.iter() {
                let (kind, index) = match export.kind {
                    ExternalKind::Func => (
                        wasm_encoder::ExportKind::Func,
                        map.funcs.remap(export.index),
                    ),
                    ExternalKind::Table => (
                        wasm_encoder::ExportKind::Table,
                        map.tables.remap(export.index),
                    ),
                    ExternalKind::Memory => (
                        wasm_encoder::ExportKind::Memory,
                        map.memories.remap(export.index),
                    ),
                    ExternalKind::Global => (
                        wasm_encoder::ExportKind::Global,
                        map.globals.remap(export.index),
                    ),
                    kind => bail!("unsupported export kind {kind:?}"),
                };
                exports.export(export.name, kind, index);
            }
            ret.section(&exports);
        }

        if let Some(start) = &start {
            ret.section(start);
        }

        if !code.is_empty() {
            ret.section(&code);
        }

        // Append a custom `name` section using the names of the functions that
        // were found prior to the GC pass in the original module.
        let mut global_names = Vec::new();
        for (i, _func) in self.live_funcs() {
            let name = match self.func_names.get(&i) {
                Some(name) => name,
                None => continue,
            };
            func_names.push((map.funcs.remap(i), *name));
        }
        for (i, _global) in self.live_globals() {
            let name = match self.global_names.get(&i) {
                Some(name) => name,
                None => continue,
            };
            global_names.push((map.globals.remap(i), *name));
        }
        let mut section = Vec::new();
        let mut encode_subsection = |code: u8, names: &[(u32, &str)]| {
            if names.is_empty() {
                return;
            }
            let mut subsection = Vec::new();
            names.len().encode(&mut subsection);
            for (i, name) in names {
                i.encode(&mut subsection);
                name.encode(&mut subsection);
            }
            section.push(code);
            subsection.encode(&mut section);
        };
        if let (Some(realloc_index), true) = (
            realloc_index,
            main_module_realloc.is_none() || allocation_state.is_none(),
        ) {
            func_names.push((realloc_index, "realloc_via_memory_grow"));
        }
        if let Some(lazy_stack_init_index) = lazy_stack_init_index {
            func_names.push((lazy_stack_init_index, "allocate_stack"));
        }
        encode_subsection(0x01, &func_names);
        encode_subsection(0x07, &global_names);
        if !section.is_empty() {
            ret.section(&wasm_encoder::CustomSection {
                name: "name".into(),
                data: Cow::Borrowed(&section),
            });
        }
        if let Some(producers) = &self.producers {
            ret.section(&RawCustomSection(&producers.raw_custom_section()));
        }

        Ok(ret.finish())
    }

    fn find_mut_i32_global(&self, name: &str) -> Result<Option<u32>> {
        let matches = &self
            .live_globals()
            .filter_map(|(i, g)| {
                if g.ty.mutable
                    && g.ty.content_type == ValType::I32
                    && *self.global_names.get(&i)? == name
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        match matches.deref() {
            [] => Ok(None),
            [i] => Ok(Some(*i)),
            _ => bail!(
                "found {} mutable i32 globals with name {name}",
                matches.len()
            ),
        }
    }
}

// This helper macro is used to define a visitor of all instructions with
// special handling for all payloads of instructions to mark any referenced
// items live.
//
// Currently item identification happens through the field name of the payload.
// While not exactly the most robust solution this should work well enough for
// now.
macro_rules! define_visit {
    ($(@$p:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            fn $visit(&mut self $(, $($arg: $argty),*)?)  {
                $(
                    $(
                        define_visit!(mark_live self $arg $arg);
                    )*
                )?
            }
        )*
    };

    (mark_live $self:ident $arg:ident type_index) => {$self.ty($arg);};
    (mark_live $self:ident $arg:ident array_type_index) => {$self.ty($arg);};
    (mark_live $self:ident $arg:ident array_type_index_dst) => {$self.ty($arg);};
    (mark_live $self:ident $arg:ident array_type_index_src) => {$self.ty($arg);};
    (mark_live $self:ident $arg:ident struct_type_index) => {$self.ty($arg);};
    (mark_live $self:ident $arg:ident src_table) => {$self.table($arg);};
    (mark_live $self:ident $arg:ident dst_table) => {$self.table($arg);};
    (mark_live $self:ident $arg:ident table_index) => {$self.table($arg);};
    (mark_live $self:ident $arg:ident table) => {$self.table($arg);};
    (mark_live $self:ident $arg:ident table_index) => {$self.table($arg);};
    (mark_live $self:ident $arg:ident global_index) => {$self.global($arg);};
    (mark_live $self:ident $arg:ident function_index) => {$self.func($arg);};
    (mark_live $self:ident $arg:ident mem) => {$self.memory($arg);};
    (mark_live $self:ident $arg:ident src_mem) => {$self.memory($arg);};
    (mark_live $self:ident $arg:ident dst_mem) => {$self.memory($arg);};
    (mark_live $self:ident $arg:ident memarg) => {$self.memory($arg.memory);};
    (mark_live $self:ident $arg:ident blockty) => {$self.blockty($arg);};
    (mark_live $self:ident $arg:ident ty) => {$self.valty($arg)};
    (mark_live $self:ident $arg:ident hty) => {$self.heapty($arg)};
    (mark_live $self:ident $arg:ident from_ref_type) => {$self.refty($arg);};
    (mark_live $self:ident $arg:ident to_ref_type) => {$self.refty($arg);};
    (mark_live $self:ident $arg:ident lane) => {};
    (mark_live $self:ident $arg:ident lanes) => {};
    (mark_live $self:ident $arg:ident flags) => {};
    (mark_live $self:ident $arg:ident value) => {};
    (mark_live $self:ident $arg:ident mem_byte) => {};
    (mark_live $self:ident $arg:ident table_byte) => {};
    (mark_live $self:ident $arg:ident local_index) => {};
    (mark_live $self:ident $arg:ident relative_depth) => {};
    (mark_live $self:ident $arg:ident tag_index) => {};
    (mark_live $self:ident $arg:ident targets) => {};
    (mark_live $self:ident $arg:ident data_index) => {};
    (mark_live $self:ident $arg:ident array_data_index) => {};
    (mark_live $self:ident $arg:ident elem_index) => {};
    (mark_live $self:ident $arg:ident array_elem_index) => {};
    (mark_live $self:ident $arg:ident array_size) => {};
    (mark_live $self:ident $arg:ident field_index) => {};
    (mark_live $self:ident $arg:ident from_type_nullable) => {};
    (mark_live $self:ident $arg:ident to_type_nullable) => {};
    (mark_live $self:ident $arg:ident try_table) => {unimplemented!();};
}

impl<'a> VisitOperator<'a> for Module<'a> {
    type Output = ();

    wasmparser::for_each_operator!(define_visit);
}

/// Helper function to filter `iter` based on the `live` set, yielding an
/// iterator over the index of the item that's live as well as the item itself.
fn live_iter<'a, T>(
    live: &'a BitVec,
    iter: impl Iterator<Item = T> + 'a,
) -> impl Iterator<Item = (u32, T)> + 'a {
    iter.enumerate().filter_map(|(i, t)| {
        let i = i as u32;
        if live.contains(i) {
            Some((i, t))
        } else {
            None
        }
    })
}

#[derive(Default)]
struct Encoder {
    types: Remap,
    funcs: Remap,
    memories: Remap,
    globals: Remap,
    tables: Remap,
    buf: Vec<u8>,
}

impl Encoder {
    fn operators(&mut self, mut reader: BinaryReader<'_>) -> Result<Vec<u8>> {
        while !reader.eof() {
            reader.visit_operator(self)?;
        }
        Ok(mem::take(&mut self.buf))
    }

    fn memarg(&self, ty: MemArg) -> wasm_encoder::MemArg {
        wasm_encoder::MemArg {
            offset: ty.offset,
            align: ty.align.into(),
            memory_index: self.memories.remap(ty.memory),
        }
    }

    fn blockty(&self, ty: BlockType) -> wasm_encoder::BlockType {
        match ty {
            BlockType::Empty => wasm_encoder::BlockType::Empty,
            BlockType::Type(ty) => wasm_encoder::BlockType::Result(self.valty(ty)),
            BlockType::FuncType(ty) => wasm_encoder::BlockType::FunctionType(self.types.remap(ty)),
        }
    }

    fn valty(&self, ty: wasmparser::ValType) -> wasm_encoder::ValType {
        match ty {
            wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
            wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
            wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
            wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
            wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
            wasmparser::ValType::Ref(rt) => wasm_encoder::ValType::Ref(self.refty(rt)),
        }
    }

    fn refty(&self, rt: wasmparser::RefType) -> wasm_encoder::RefType {
        wasm_encoder::RefType {
            nullable: rt.is_nullable(),
            heap_type: self.heapty(rt.heap_type()),
        }
    }

    fn heapty(&self, ht: wasmparser::HeapType) -> wasm_encoder::HeapType {
        match ht {
            HeapType::Func => wasm_encoder::HeapType::Func,
            HeapType::Extern => wasm_encoder::HeapType::Extern,
            HeapType::Any => wasm_encoder::HeapType::Any,
            HeapType::None => wasm_encoder::HeapType::None,
            HeapType::NoExtern => wasm_encoder::HeapType::NoExtern,
            HeapType::NoFunc => wasm_encoder::HeapType::NoFunc,
            HeapType::Eq => wasm_encoder::HeapType::Eq,
            HeapType::Struct => wasm_encoder::HeapType::Struct,
            HeapType::Array => wasm_encoder::HeapType::Array,
            HeapType::I31 => wasm_encoder::HeapType::I31,
            HeapType::Exn => wasm_encoder::HeapType::Exn,
            HeapType::Concrete(idx) => {
                wasm_encoder::HeapType::Concrete(self.types.remap(idx.as_module_index().unwrap()))
            }
        }
    }
}

// This is a helper macro to translate all `wasmparser` instructions to
// `wasm-encoder` instructions without having to list out every single
// instruction itself.
//
// The general goal of this macro is to have O(unique instruction payload)
// number of cases while also simultaneously adapting between the styles of
// wasmparser and wasm-encoder.
macro_rules! define_encode {
    ($(@$p:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            fn $visit(&mut self $(, $($arg: $argty),*)?)  {
                #[allow(unused_imports)]
                use wasm_encoder::Instruction::*;
                $(
                    $(
                        let $arg = define_encode!(map self $arg $arg);
                    )*
                )?
                let insn = define_encode!(mk $op $($($arg)*)?);
                insn.encode(&mut self.buf);
            }
        )*
    };

    // No-payload instructions are named the same in wasmparser as they are in
    // wasm-encoder
    (mk $op:ident) => ($op);

    // Instructions which need "special care" to map from wasmparser to
    // wasm-encoder
    (mk BrTable $arg:ident) => ({
        BrTable($arg.0, $arg.1)
    });
    (mk CallIndirect $ty:ident $table:ident $table_byte:ident) => ({
        let _ = $table_byte;
        CallIndirect { ty: $ty, table: $table }
    });
    (mk ReturnCallIndirect $ty:ident $table:ident) => (
        ReturnCallIndirect { ty: $ty, table: $table }
    );
    (mk MemorySize $mem:ident $mem_byte:ident) => ({
        let _ = $mem_byte;
        MemorySize($mem)
    });
    (mk MemoryGrow $mem:ident $mem_byte:ident) => ({
        let _ = $mem_byte;
        MemoryGrow($mem)
    });
    (mk TryTable $try_table:ident) => ({
        let _ = $try_table;
        unimplemented_try_table()
    });
    (mk I32Const $v:ident) => (I32Const($v));
    (mk I64Const $v:ident) => (I64Const($v));
    (mk F32Const $v:ident) => (F32Const(f32::from_bits($v.bits())));
    (mk F64Const $v:ident) => (F64Const(f64::from_bits($v.bits())));
    (mk V128Const $v:ident) => (V128Const($v.i128()));

    // Catch-all for the translation of one payload argument which is typically
    // represented as a tuple-enum in wasm-encoder.
    (mk $op:ident $arg:ident) => ($op($arg));

    // Catch-all of everything else where the wasmparser fields are simply
    // translated to wasm-encoder fields.
    (mk $op:ident $($arg:ident)*) => ($op { $($arg),* });

    // Individual cases of mapping one argument type to another, similar to the
    // `define_visit` macro above.
    (map $self:ident $arg:ident memarg) => {$self.memarg($arg)};
    (map $self:ident $arg:ident blockty) => {$self.blockty($arg)};
    (map $self:ident $arg:ident hty) => {$self.heapty($arg)};
    (map $self:ident $arg:ident from_ref_type) => {$self.refty($arg)};
    (map $self:ident $arg:ident to_ref_type) => {$self.refty($arg)};
    (map $self:ident $arg:ident tag_index) => {$arg};
    (map $self:ident $arg:ident relative_depth) => {$arg};
    (map $self:ident $arg:ident function_index) => {$self.funcs.remap($arg)};
    (map $self:ident $arg:ident global_index) => {$self.globals.remap($arg)};
    (map $self:ident $arg:ident mem) => {$self.memories.remap($arg)};
    (map $self:ident $arg:ident src_mem) => {$self.memories.remap($arg)};
    (map $self:ident $arg:ident dst_mem) => {$self.memories.remap($arg)};
    (map $self:ident $arg:ident table) => {$self.tables.remap($arg)};
    (map $self:ident $arg:ident table_index) => {$self.tables.remap($arg)};
    (map $self:ident $arg:ident src_table) => {$self.tables.remap($arg)};
    (map $self:ident $arg:ident dst_table) => {$self.tables.remap($arg)};
    (map $self:ident $arg:ident type_index) => {$self.types.remap($arg)};
    (map $self:ident $arg:ident array_type_index) => {$self.types.remap($arg)};
    (map $self:ident $arg:ident array_type_index_dst) => {$self.types.remap($arg)};
    (map $self:ident $arg:ident array_type_index_src) => {$self.types.remap($arg)};
    (map $self:ident $arg:ident struct_type_index) => {$self.types.remap($arg)};
    (map $self:ident $arg:ident ty) => {$self.valty($arg)};
    (map $self:ident $arg:ident local_index) => {$arg};
    (map $self:ident $arg:ident lane) => {$arg};
    (map $self:ident $arg:ident lanes) => {$arg};
    (map $self:ident $arg:ident elem_index) => {$arg};
    (map $self:ident $arg:ident data_index) => {$arg};
    (map $self:ident $arg:ident array_elem_index) => {$arg};
    (map $self:ident $arg:ident array_data_index) => {$arg};
    (map $self:ident $arg:ident table_byte) => {$arg};
    (map $self:ident $arg:ident mem_byte) => {$arg};
    (map $self:ident $arg:ident value) => {$arg};
    (map $self:ident $arg:ident array_size) => {$arg};
    (map $self:ident $arg:ident field_index) => {$arg};
    (map $self:ident $arg:ident from_type_nullable) => {$arg};
    (map $self:ident $arg:ident to_type_nullable) => {$arg};
    (map $self:ident $arg:ident try_table) => {$arg};
    (map $self:ident $arg:ident targets) => ((
        $arg.targets().map(|i| i.unwrap()).collect::<Vec<_>>().into(),
        $arg.default(),
    ));
}

fn unimplemented_try_table() -> wasm_encoder::Instruction<'static> {
    unimplemented!()
}

impl<'a> VisitOperator<'a> for Encoder {
    type Output = ();

    wasmparser::for_each_operator!(define_encode);
}

// Minimal definition of a bit vector necessary for the liveness calculations
// above.
mod bitvec {
    use std::mem;

    type T = u64;

    #[derive(Default)]
    pub struct BitVec {
        bits: Vec<T>,
    }

    impl BitVec {
        /// Inserts `idx` into this bit vector, returning whether it was not
        /// previously present.
        pub fn insert(&mut self, idx: u32) -> bool {
            let (idx, bit) = idx_bit(idx);
            match self.bits.get_mut(idx) {
                Some(bits) => {
                    if *bits & bit != 0 {
                        return false;
                    }
                    *bits |= bit;
                }
                None => {
                    self.bits.resize(idx + 1, 0);
                    self.bits[idx] = bit;
                }
            }
            true
        }

        /// Returns whether this bit vector contains the specified `idx`th bit.
        pub fn contains(&self, idx: u32) -> bool {
            let (idx, bit) = idx_bit(idx);
            match self.bits.get(idx) {
                Some(bits) => (*bits & bit) != 0,
                None => false,
            }
        }
    }

    fn idx_bit(idx: u32) -> (usize, T) {
        let idx = idx as usize;
        let size = mem::size_of::<T>() * 8;
        let index = idx / size;
        let bit = 1 << (idx % size);
        (index, bit)
    }
}

/// Small data structure used to track index mappings from an old index space to
/// a new.
#[derive(Default)]
struct Remap {
    /// Map, indexed by the old index set, to the new index set.
    map: HashMap<u32, u32>,
    /// The next available index in the new index space.
    next: u32,
}

impl Remap {
    /// Appends a new live "old index" into this remapping structure.
    ///
    /// This will assign a new index for the old index provided.
    fn push(&mut self, old: u32) {
        self.map.insert(old, self.next);
        self.next += 1;
    }

    /// Returns the new index corresponding to an old index.
    ///
    /// Panics if the `old` index was not added via `push` above.
    fn remap(&self, old: u32) -> u32 {
        *self
            .map
            .get(&old)
            .unwrap_or_else(|| panic!("can't map {old} to a new index"))
    }
}
