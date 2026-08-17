//! State relating to validating a WebAssembly component.

use super::{
    check_max,
    component_types::{
        Abi, AliasableResourceId, ComponentAnyTypeId, ComponentCoreInstanceTypeId,
        ComponentCoreModuleTypeId, ComponentCoreTypeId, ComponentDefinedType,
        ComponentDefinedTypeId, ComponentEntityType, ComponentFuncType, ComponentFuncTypeId,
        ComponentInstanceType, ComponentInstanceTypeId, ComponentType, ComponentTypeId,
        ComponentValType, Context, CoreInstanceTypeKind, InstanceType, ModuleType, RecordType,
        Remap, Remapping, ResourceId, SubtypeCx, TupleType, VariantCase, VariantType,
    },
    core::{InternRecGroup, Module},
    types::{CoreTypeId, EntityType, TypeAlloc, TypeInfo, TypeList},
};
use crate::collections::index_map::Entry;
use crate::limits::*;
use crate::prelude::*;
use crate::validator::names::{ComponentName, ComponentNameKind, KebabStr, KebabString};
use crate::{
    BinaryReaderError, CanonicalFunction, CanonicalOption, ComponentExportName,
    ComponentExternalKind, ComponentOuterAliasKind, ComponentTypeRef, CompositeInnerType,
    ExternalKind, FuncType, GlobalType, InstantiationArgKind, MemoryType, PackedIndex, RefType,
    Result, SubType, TableType, TypeBounds, ValType, WasmFeatures,
};
use core::mem;

fn to_kebab_str<'a>(s: &'a str, desc: &str, offset: usize) -> Result<&'a KebabStr> {
    match KebabStr::new(s) {
        Some(s) => Ok(s),
        None => {
            if s.is_empty() {
                bail!(offset, "{desc} name cannot be empty");
            }

            bail!(offset, "{desc} name `{s}` is not in kebab case");
        }
    }
}

pub(crate) struct ComponentState {
    /// Whether this state is a concrete component, an instance type, or a
    /// component type.
    kind: ComponentKind,
    features: WasmFeatures,

    // Core index spaces
    pub core_types: Vec<ComponentCoreTypeId>,
    pub core_funcs: Vec<CoreTypeId>,
    pub core_tags: Vec<CoreTypeId>,
    pub core_modules: Vec<ComponentCoreModuleTypeId>,
    pub core_instances: Vec<ComponentCoreInstanceTypeId>,
    pub core_memories: Vec<MemoryType>,
    pub core_tables: Vec<TableType>,
    pub core_globals: Vec<GlobalType>,

    // Component index spaces
    pub types: Vec<ComponentAnyTypeId>,
    pub funcs: Vec<ComponentFuncTypeId>,
    pub values: Vec<(ComponentValType, bool)>,
    pub instances: Vec<ComponentInstanceTypeId>,
    pub components: Vec<ComponentTypeId>,

    pub imports: IndexMap<String, ComponentEntityType>,
    pub import_names: IndexSet<ComponentName>,
    pub exports: IndexMap<String, ComponentEntityType>,
    pub export_names: IndexSet<ComponentName>,

    has_start: bool,
    type_info: TypeInfo,

    /// A mapping of imported resources in this component.
    ///
    /// This mapping represents all "type variables" imported into the
    /// component, or resources. This could be resources imported directly as
    /// a top-level type import or additionally transitively through other
    /// imported instances.
    ///
    /// The mapping element here is a "path" which is a list of indexes into
    /// the import map that will be generated for this component. Each index
    /// is an index into an `IndexMap`, and each list is guaranteed to have at
    /// least one element.
    ///
    /// An example of this map is:
    ///
    /// ```wasm
    /// (component
    ///     ;; [0] - the first import
    ///     (import "r" (type (sub resource)))
    ///
    ///     ;; [1] - the second import
    ///     (import "r2" (type (sub resource)))
    ///
    ///     (import "i" (instance
    ///         ;; [2, 0] - the third import, and the first export the instance
    ///         (export "r3" (type (sub resource)))
    ///         ;; [2, 1] - the third import, and the second export the instance
    ///         (export "r4" (type (sub resource)))
    ///     ))
    ///
    ///     ;; ...
    /// )
    /// ```
    ///
    /// The `Vec<usize>` here can be thought of as `Vec<String>` but a
    /// (hopefully) more efficient representation.
    ///
    /// Finally note that this map is listed as an "append only" map because all
    /// insertions into it should always succeed. Any insertion which overlaps
    /// with a previous entry indicates a bug in the validator which needs to be
    /// corrected via other means.
    //
    // TODO: make these `SkolemResourceId` and then go fix all the compile
    // errors, don't add skolem things into the type area
    imported_resources: IndexMapAppendOnly<ResourceId, Vec<usize>>,

    /// A mapping of "defined" resources in this component, or those which
    /// are defined within the instantiation of this component.
    ///
    /// Defined resources, as the name implies, can sort of be thought of as
    /// "these are defined within the component". Note though that the means by
    /// which a local definition can occur are not simply those defined in the
    /// component but also in its transitively instantiated components
    /// internally. This means that this set closes over many transitive
    /// internal items in addition to those defined immediately in the component
    /// itself.
    ///
    /// The `Option<ValType>` in this mapping is whether or not the underlying
    /// representation of the resource is known to this component. Immediately
    /// defined resources, for example, will have `Some(I32)` here. Resources
    /// that come from transitively defined components, for example, will have
    /// `None`. In the type context all entries here are `None`.
    ///
    /// Note that like `imported_resources` all insertions into this map are
    /// expected to succeed to it's declared as append-only.
    defined_resources: IndexMapAppendOnly<ResourceId, Option<ValType>>,

    /// A mapping of explicitly exported resources from this component in
    /// addition to the path that they're exported at.
    ///
    /// For more information on the path here see the documentation for
    /// `imported_resources`. Note that the indexes here index into the
    /// list of exports of this component.
    explicit_resources: IndexMap<ResourceId, Vec<usize>>,

    /// The set of types which are considered "exported" from this component.
    ///
    /// This is added to whenever a type export is found, or an instance export
    /// which itself contains a type export. This additionally includes all
    /// imported types since those are suitable for export as well.
    ///
    /// This set is consulted whenever an exported item is added since all
    /// referenced types must be members of this set.
    exported_types: Set<ComponentAnyTypeId>,

    /// Same as `exported_types`, but for imports.
    imported_types: Set<ComponentAnyTypeId>,

    /// The set of top-level resource exports and their names.
    ///
    /// This context is used to validate method names such as `[method]foo.bar`
    /// to ensure that `foo` is an exported resource and that the type mentioned
    /// in a function type is actually named `foo`.
    ///
    /// Note that imports/exports have disjoint contexts to ensure that they're
    /// validated correctly. Namely you can't retroactively attach methods to an
    /// import, for example.
    toplevel_exported_resources: ComponentNameContext,

    /// Same as `toplevel_exported_resources`, but for imports.
    toplevel_imported_resources: ComponentNameContext,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComponentKind {
    Component,
    InstanceType,
    ComponentType,
}

/// Helper context used to track information about resource names for method
/// name validation.
#[derive(Default)]
struct ComponentNameContext {
    /// A map from a resource type id to an index in the `all_resource_names`
    /// set for the name of that resource.
    resource_name_map: Map<AliasableResourceId, usize>,

    /// All known resource names in this context, used to validate static method
    /// names to by ensuring that static methods' resource names are somewhere
    /// in this set.
    all_resource_names: IndexSet<String>,
}

#[derive(Debug, Copy, Clone)]
pub enum ExternKind {
    Import,
    Export,
}

impl ExternKind {
    pub fn desc(&self) -> &'static str {
        match self {
            ExternKind::Import => "import",
            ExternKind::Export => "export",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum StringEncoding {
    #[default]
    Utf8,
    Utf16,
    CompactUtf16,
}

impl core::fmt::Display for StringEncoding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Utf8 => "utf8",
            Self::Utf16 => "utf16",
            Self::CompactUtf16 => "latin1-utf16",
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Concurrency {
    /// Synchronous.
    #[default]
    Sync,

    /// Asynchronous.
    Async {
        /// When present, this is the function index of the async callback. When
        /// omitted, we are either using stack-switching based asynchrony or are
        /// in an operation that does not support the `callback` option (like
        /// lowering).
        callback: Option<u32>,
    },
}

impl Concurrency {
    pub(crate) fn is_sync(&self) -> bool {
        matches!(self, Self::Sync)
    }

    pub(crate) fn is_async(&self) -> bool {
        !self.is_sync()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CanonicalOptions {
    pub(crate) string_encoding: StringEncoding,
    pub(crate) memory: Option<u32>,
    pub(crate) realloc: Option<u32>,
    pub(crate) post_return: Option<u32>,
    pub(crate) concurrency: Concurrency,
    pub(crate) core_type: Option<CoreTypeId>,
    pub(crate) gc: bool,
}

impl CanonicalOptions {
    pub(crate) fn require_sync(&self, offset: usize, where_: &str) -> Result<&Self> {
        if !self.concurrency.is_sync() {
            bail!(offset, "cannot specify `async` option on `{where_}`")
        }
        Ok(self)
    }

    pub(crate) fn require_memory(&self, offset: usize) -> Result<&Self> {
        if self.memory.is_none() {
            bail!(offset, "canonical option `memory` is required");
        }
        Ok(self)
    }

    pub(crate) fn require_realloc(&self, offset: usize) -> Result<&Self> {
        // Memory is always required when `realloc` is required.
        self.require_memory(offset)?;

        if self.realloc.is_none() {
            bail!(offset, "canonical option `realloc` is required")
        }

        Ok(self)
    }

    pub(crate) fn require_memory_if(
        &self,
        offset: usize,
        when: impl Fn() -> bool,
    ) -> Result<&Self> {
        if self.memory.is_none() && when() {
            self.require_memory(offset)?;
        }
        Ok(self)
    }

    pub(crate) fn require_realloc_if(
        &self,
        offset: usize,
        when: impl Fn() -> bool,
    ) -> Result<&Self> {
        if self.realloc.is_none() && when() {
            self.require_realloc(offset)?;
        }
        Ok(self)
    }

    pub(crate) fn check_lower(&self, offset: usize) -> Result<&Self> {
        if self.post_return.is_some() {
            bail!(
                offset,
                "canonical option `post-return` cannot be specified for lowerings"
            );
        }

        if let Concurrency::Async { callback: Some(_) } = self.concurrency {
            bail!(
                offset,
                "canonical option `callback` cannot be specified for lowerings"
            );
        }

        if self.gc && self.core_type.is_none() {
            bail!(
                offset,
                "cannot specify `gc` without also specifying a `core-type` for lowerings"
            )
        }

        Ok(self)
    }

    pub(crate) fn check_lift(
        &mut self,
        types: &TypeList,
        state: &ComponentState,
        core_ty_id: CoreTypeId,
        offset: usize,
    ) -> Result<&Self> {
        debug_assert!(matches!(
            types[core_ty_id].composite_type.inner,
            CompositeInnerType::Func(_)
        ));

        if let Some(idx) = self.post_return {
            let post_return_func_ty = types[state.core_function_at(idx, offset)?].unwrap_func();
            let core_ty = types[core_ty_id].unwrap_func();
            if post_return_func_ty.params() != core_ty.results()
                || !post_return_func_ty.results().is_empty()
            {
                bail!(
                    offset,
                    "canonical option `post-return` uses a core function with an incorrect signature"
                );
            }
        }

        match self.concurrency {
            Concurrency::Sync => {}

            Concurrency::Async { callback: None } if !state.features.cm_async_stackful() => {
                bail!(
                    offset,
                    "requires the component model async stackful feature"
                )
            }
            Concurrency::Async { callback: None } => {}

            Concurrency::Async {
                callback: Some(idx),
            } => {
                let func_ty = types[state.core_function_at(idx, offset)?].unwrap_func();
                if func_ty.params() != [ValType::I32; 3] && func_ty.params() != [ValType::I32] {
                    return Err(BinaryReaderError::new(
                        "canonical option `callback` uses a core function with an incorrect signature",
                        offset,
                    ));
                }
            }
        }

        if self.core_type.is_some() {
            bail!(
                offset,
                "canonical option `core-type` is not allowed in `canon lift`"
            )
        }
        self.core_type = Some(core_ty_id);

        Ok(self)
    }

    pub(crate) fn check_core_type(
        &self,
        types: &mut TypeAlloc,
        actual: FuncType,
        offset: usize,
    ) -> Result<CoreTypeId> {
        if let Some(declared_id) = self.core_type {
            let declared = types[declared_id].unwrap_func();

            if actual.params() != declared.params() {
                bail!(
                    offset,
                    "declared core type has `{:?}` parameter types, but actual lowering has \
                     `{:?}` parameter types",
                    declared.params(),
                    actual.params(),
                );
            }

            if actual.results() != declared.results() {
                bail!(
                    offset,
                    "declared core type has `{:?}` result types, but actual lowering has \
                     `{:?}` result types",
                    declared.results(),
                    actual.results(),
                );
            }

            Ok(declared_id)
        } else {
            Ok(types.intern_func_type(actual, offset))
        }
    }
}

impl ComponentState {
    pub fn new(kind: ComponentKind, features: WasmFeatures) -> Self {
        Self {
            kind,
            features,
            core_types: Default::default(),
            core_modules: Default::default(),
            core_instances: Default::default(),
            core_funcs: Default::default(),
            core_memories: Default::default(),
            core_tables: Default::default(),
            core_globals: Default::default(),
            core_tags: Default::default(),
            types: Default::default(),
            funcs: Default::default(),
            values: Default::default(),
            instances: Default::default(),
            components: Default::default(),
            imports: Default::default(),
            exports: Default::default(),
            import_names: Default::default(),
            export_names: Default::default(),
            has_start: Default::default(),
            type_info: TypeInfo::new(),
            imported_resources: Default::default(),
            defined_resources: Default::default(),
            explicit_resources: Default::default(),
            exported_types: Default::default(),
            imported_types: Default::default(),
            toplevel_exported_resources: Default::default(),
            toplevel_imported_resources: Default::default(),
        }
    }

    pub fn type_count(&self) -> usize {
        self.core_types.len() + self.types.len()
    }

    pub fn instance_count(&self) -> usize {
        self.core_instances.len() + self.instances.len()
    }

    pub fn function_count(&self) -> usize {
        self.core_funcs.len() + self.funcs.len()
    }

    pub fn add_core_type(
        components: &mut [Self],
        ty: crate::CoreType,
        types: &mut TypeAlloc,
        offset: usize,
        check_limit: bool,
    ) -> Result<()> {
        let current = components.last_mut().unwrap();
        if check_limit {
            check_max(current.type_count(), 1, MAX_WASM_TYPES, "types", offset)?;
        }
        match ty {
            crate::CoreType::Rec(rec) => {
                current.canonicalize_and_intern_rec_group(types, rec, offset)?;
            }
            crate::CoreType::Module(decls) => {
                let mod_ty = Self::create_module_type(components, decls.into_vec(), types, offset)?;
                let id = ComponentCoreTypeId::Module(types.push_ty(mod_ty));
                components.last_mut().unwrap().core_types.push(id);
            }
        }

        Ok(())
    }

    pub fn add_core_module(
        &mut self,
        module: &Module,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let imports = module.imports_for_module_type(offset)?;

        // We have to clone the module's imports and exports here
        // because we cannot take the data out of the `MaybeOwned`
        // as it might be shared with a function validator.
        let mod_ty = ModuleType {
            info: TypeInfo::core(module.type_size),
            imports,
            exports: module.exports.clone(),
        };

        let mod_id = types.push_ty(mod_ty);
        self.core_modules.push(mod_id);

        Ok(())
    }

    pub fn add_core_instance(
        &mut self,
        instance: crate::Instance,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let instance = match instance {
            crate::Instance::Instantiate { module_index, args } => {
                self.instantiate_core_module(module_index, args.into_vec(), types, offset)?
            }
            crate::Instance::FromExports(exports) => {
                self.instantiate_core_exports(exports.into_vec(), types, offset)?
            }
        };

        self.core_instances.push(instance);

        Ok(())
    }

    pub fn add_type(
        components: &mut Vec<Self>,
        ty: crate::ComponentType,
        types: &mut TypeAlloc,
        offset: usize,
        check_limit: bool,
    ) -> Result<()> {
        assert!(!components.is_empty());

        fn current(components: &mut Vec<ComponentState>) -> &mut ComponentState {
            components.last_mut().unwrap()
        }

        let id = match ty {
            crate::ComponentType::Defined(ty) => {
                let ty = current(components).create_defined_type(ty, types, offset)?;
                types.push(ty).into()
            }
            crate::ComponentType::Func(ty) => {
                let ty = current(components).create_function_type(ty, types, offset)?;
                types.push(ty).into()
            }
            crate::ComponentType::Component(decls) => {
                let ty = Self::create_component_type(components, decls.into_vec(), types, offset)?;
                types.push(ty).into()
            }
            crate::ComponentType::Instance(decls) => {
                let ty = Self::create_instance_type(components, decls.into_vec(), types, offset)?;
                types.push(ty).into()
            }
            crate::ComponentType::Resource { rep, dtor } => {
                let component = current(components);

                // Resource types cannot be declared in a type context, only
                // within a component context.
                if component.kind != ComponentKind::Component {
                    bail!(
                        offset,
                        "resources can only be defined within a concrete component"
                    );
                }

                // Current MVP restriction of the component model.
                if rep != ValType::I32 {
                    bail!(offset, "resources can only be represented by `i32`");
                }

                // If specified validate that the destructor is both a valid
                // function and has the correct signature.
                if let Some(dtor) = dtor {
                    let ty = component.core_function_at(dtor, offset)?;
                    let ty = types[ty].composite_type.unwrap_func();
                    if ty.params() != [rep] || ty.results() != [] {
                        bail!(
                            offset,
                            "core function {dtor} has wrong signature for a destructor"
                        );
                    }
                }

                // As this is the introduction of a resource create a fresh new
                // identifier for the resource. This is then added into the
                // list of defined resources for this component, notably with a
                // rep listed to enable getting access to various intrinsics
                // such as `resource.rep`.
                let id = types.alloc_resource_id();
                component.defined_resources.insert(id.resource(), Some(rep));
                id.into()
            }
        };

        let current = current(components);
        if check_limit {
            check_max(current.type_count(), 1, MAX_WASM_TYPES, "types", offset)?;
        }
        current.types.push(id);

        Ok(())
    }

    pub fn add_import(
        &mut self,
        import: crate::ComponentImport,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let mut entity = self.check_type_ref(&import.ty, types, offset)?;
        self.add_entity(
            &mut entity,
            Some((import.name.0, ExternKind::Import)),
            types,
            offset,
        )?;
        self.toplevel_imported_resources.validate_extern(
            import.name.0,
            ExternKind::Import,
            &entity,
            types,
            offset,
            &mut self.import_names,
            &mut self.imports,
            &mut self.type_info,
            &self.features,
        )?;
        Ok(())
    }

    fn add_entity(
        &mut self,
        ty: &mut ComponentEntityType,
        name_and_kind: Option<(&str, ExternKind)>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let kind = name_and_kind.map(|(_, k)| k);
        let (len, max, desc) = match ty {
            ComponentEntityType::Module(id) => {
                self.core_modules.push(*id);
                (self.core_modules.len(), MAX_WASM_MODULES, "modules")
            }
            ComponentEntityType::Component(id) => {
                self.components.push(*id);
                (self.components.len(), MAX_WASM_COMPONENTS, "components")
            }
            ComponentEntityType::Instance(id) => {
                match kind {
                    Some(ExternKind::Import) => self.prepare_instance_import(id, types),
                    Some(ExternKind::Export) => self.prepare_instance_export(id, types),
                    None => {}
                }
                self.instances.push(*id);
                (self.instance_count(), MAX_WASM_INSTANCES, "instances")
            }
            ComponentEntityType::Func(id) => {
                self.funcs.push(*id);
                (self.function_count(), MAX_WASM_FUNCTIONS, "functions")
            }
            ComponentEntityType::Value(ty) => {
                self.check_value_support(offset)?;
                let value_used = match kind {
                    Some(ExternKind::Import) | None => false,
                    Some(ExternKind::Export) => true,
                };
                self.values.push((*ty, value_used));
                (self.values.len(), MAX_WASM_VALUES, "values")
            }
            ComponentEntityType::Type {
                created,
                referenced,
            } => {
                self.types.push(*created);

                // Extra logic here for resources being imported and exported.
                // Note that if `created` is the same as `referenced` then this
                // is the original introduction of the resource which is where
                // `self.{imported,defined}_resources` are updated.
                if let ComponentAnyTypeId::Resource(id) = *created {
                    match kind {
                        Some(ExternKind::Import) => {
                            // A fresh new resource is being imported into a
                            // component. This arises from the import section of
                            // a component or from the import declaration in a
                            // component type. In both cases a new imported
                            // resource is injected with a fresh new identifier
                            // into our state.
                            if created == referenced {
                                self.imported_resources
                                    .insert(id.resource(), vec![self.imports.len()]);
                            }
                        }

                        Some(ExternKind::Export) => {
                            // A fresh resource is being exported from this
                            // component. This arises as part of the
                            // declaration of a component type, for example. In
                            // this situation brand new resource identifier is
                            // allocated and a definition is added, unlike the
                            // import case where an imported resource is added.
                            // Notably the representation of this new resource
                            // is unknown so it's listed as `None`.
                            if created == referenced {
                                self.defined_resources.insert(id.resource(), None);
                            }

                            // If this is a type export of a resource type then
                            // update the `explicit_resources` list. A new
                            // export path is about to be created for this
                            // resource and this keeps track of that.
                            self.explicit_resources
                                .insert(id.resource(), vec![self.exports.len()]);
                        }

                        None => {}
                    }
                }
                (self.types.len(), MAX_WASM_TYPES, "types")
            }
        };

        check_max(len, 0, max, desc, offset)?;

        // Before returning perform the final validation of the type of the item
        // being imported/exported. This will ensure that everything is
        // appropriately named with respect to type definitions, resources, etc.
        if let Some((name, kind)) = name_and_kind {
            if !self.validate_and_register_named_types(Some(name), kind, ty, types) {
                bail!(
                    offset,
                    "{} not valid to be used as {}",
                    ty.desc(),
                    kind.desc()
                );
            }
        }
        Ok(())
    }

    /// Validates that the `ty` referenced only refers to named types internally
    /// and then inserts anything necessary, if applicable, to the defined sets
    /// within this component.
    ///
    /// This function will validate that `ty` only refers to named types. For
    /// example if it's a record then all of its fields must refer to named
    /// types. This consults either `self.imported_types` or
    /// `self.exported_types` as specified by `kind`. Note that this is not
    /// inherently recursive itself but it ends up being recursive since if
    /// recursive members were named then all their components must also be
    /// named. Consequently this check stops at the "one layer deep" position,
    /// or more accurately the position where types must be named (e.g. tuples
    /// aren't required to be named).
    fn validate_and_register_named_types(
        &mut self,
        toplevel_name: Option<&str>,
        kind: ExternKind,
        ty: &ComponentEntityType,
        types: &TypeAlloc,
    ) -> bool {
        if let ComponentEntityType::Type { created, .. } = ty {
            // If this is a top-level resource then register it in the
            // appropriate context so later validation of method-like-names
            // works out.
            if let Some(name) = toplevel_name {
                if let ComponentAnyTypeId::Resource(id) = *created {
                    let cx = match kind {
                        ExternKind::Import => &mut self.toplevel_imported_resources,
                        ExternKind::Export => &mut self.toplevel_exported_resources,
                    };
                    cx.register(name, id);
                }
            }
        }

        match self.kind {
            ComponentKind::Component | ComponentKind::ComponentType => {}
            ComponentKind::InstanceType => return true,
        }
        let set = match kind {
            ExternKind::Import => &self.imported_types,
            ExternKind::Export => &self.exported_types,
        };
        match ty {
            // When a type is imported or exported than any recursive type
            // referred to by that import/export must additionally be exported
            // or imported. Here this walks the "first layer" of the type which
            // delegates to `TypeAlloc::type_named_type_id` to determine whether
            // the components of the type being named here are indeed all they
            // themselves named.
            ComponentEntityType::Type {
                created,
                referenced,
            } => {
                if !self.all_valtypes_named(types, *referenced, set) {
                    return false;
                }
                match kind {
                    // Imported types are both valid for import and valid for
                    // export.
                    ExternKind::Import => {
                        self.imported_types.insert(*created);
                        self.exported_types.insert(*created);
                    }
                    ExternKind::Export => {
                        self.exported_types.insert(*created);
                    }
                }

                true
            }

            // Instances are slightly nuanced here. The general idea is that if
            // an instance is imported, then any type exported by the instance
            // is then also exported. Additionally for exports. To get this to
            // work out this arm will recursively call
            // `validate_and_register_named_types` which means that types are
            // inserted into `self.{imported,exported}_types` as-we-go rather
            // than all at once.
            //
            // This then recursively validates that all items in the instance
            // itself are valid to import/export, recursive instances are
            // captured, and everything is appropriately added to the right
            // imported/exported set.
            ComponentEntityType::Instance(i) => types[*i]
                .exports
                .iter()
                .all(|(_name, ty)| self.validate_and_register_named_types(None, kind, ty, types)),

            // All types referred to by a function must be named.
            ComponentEntityType::Func(id) => self.all_valtypes_named_in_func(types, *id, set),

            ComponentEntityType::Value(ty) => types.type_named_valtype(ty, set),

            // Components/modules are always "closed" or "standalone" and don't
            // need validation with respect to their named types.
            ComponentEntityType::Component(_) | ComponentEntityType::Module(_) => true,
        }
    }

    fn all_valtypes_named(
        &self,
        types: &TypeAlloc,
        id: ComponentAnyTypeId,
        set: &Set<ComponentAnyTypeId>,
    ) -> bool {
        match id {
            // Resource types, in isolation, are always valid to import or
            // export since they're either attached to an import or being
            // exported.
            //
            // Note that further validation of this happens in `finish`, too.
            ComponentAnyTypeId::Resource(_) => true,

            // Component types are validated as they are constructed,
            // so all component types are valid to export if they've
            // already been constructed.
            ComponentAnyTypeId::Component(_) => true,

            ComponentAnyTypeId::Defined(id) => self.all_valtypes_named_in_defined(types, id, set),
            ComponentAnyTypeId::Func(id) => self.all_valtypes_named_in_func(types, id, set),
            ComponentAnyTypeId::Instance(id) => self.all_valtypes_named_in_instance(types, id, set),
        }
    }

    fn all_valtypes_named_in_instance(
        &self,
        types: &TypeAlloc,
        id: ComponentInstanceTypeId,
        set: &Set<ComponentAnyTypeId>,
    ) -> bool {
        // Instances must recursively have all referenced types named.
        let ty = &types[id];
        ty.exports.values().all(|ty| match ty {
            ComponentEntityType::Module(_) => true,
            ComponentEntityType::Func(id) => self.all_valtypes_named_in_func(types, *id, set),
            ComponentEntityType::Type { created: id, .. } => {
                self.all_valtypes_named(types, *id, set)
            }
            ComponentEntityType::Value(ComponentValType::Type(id)) => {
                self.all_valtypes_named_in_defined(types, *id, set)
            }
            ComponentEntityType::Instance(id) => {
                self.all_valtypes_named_in_instance(types, *id, set)
            }
            ComponentEntityType::Component(_)
            | ComponentEntityType::Value(ComponentValType::Primitive(_)) => return true,
        })
    }

    fn all_valtypes_named_in_defined(
        &self,
        types: &TypeAlloc,
        id: ComponentDefinedTypeId,
        set: &Set<ComponentAnyTypeId>,
    ) -> bool {
        let ty = &types[id];
        match ty {
            // These types do not contain anything which must be
            // named.
            ComponentDefinedType::Primitive(_)
            | ComponentDefinedType::Flags(_)
            | ComponentDefinedType::Enum(_) => true,

            // Referenced types of all these aggregates must all be
            // named.
            ComponentDefinedType::Record(r) => {
                r.fields.values().all(|t| types.type_named_valtype(t, set))
            }
            ComponentDefinedType::Tuple(r) => {
                r.types.iter().all(|t| types.type_named_valtype(t, set))
            }
            ComponentDefinedType::Variant(r) => r
                .cases
                .values()
                .filter_map(|t| t.ty.as_ref())
                .all(|t| types.type_named_valtype(t, set)),
            ComponentDefinedType::Result { ok, err } => {
                ok.as_ref()
                    .map(|t| types.type_named_valtype(t, set))
                    .unwrap_or(true)
                    && err
                        .as_ref()
                        .map(|t| types.type_named_valtype(t, set))
                        .unwrap_or(true)
            }
            ComponentDefinedType::List(ty)
            | ComponentDefinedType::FixedSizeList(ty, _)
            | ComponentDefinedType::Option(ty) => types.type_named_valtype(ty, set),
            ComponentDefinedType::Map(k, v) => {
                types.type_named_valtype(k, set) && types.type_named_valtype(v, set)
            }

            // The resource referred to by own/borrow must be named.
            ComponentDefinedType::Own(id) | ComponentDefinedType::Borrow(id) => {
                set.contains(&ComponentAnyTypeId::from(*id))
            }

            ComponentDefinedType::Future(ty) => ty
                .as_ref()
                .map(|ty| types.type_named_valtype(ty, set))
                .unwrap_or(true),
            ComponentDefinedType::Stream(ty) => ty
                .as_ref()
                .map(|ty| types.type_named_valtype(ty, set))
                .unwrap_or(true),
        }
    }

    fn all_valtypes_named_in_func(
        &self,
        types: &TypeAlloc,
        id: ComponentFuncTypeId,
        set: &Set<ComponentAnyTypeId>,
    ) -> bool {
        let ty = &types[id];
        // Function types must have all their parameters/results named.
        ty.params
            .iter()
            .map(|(_, ty)| ty)
            .chain(&ty.result)
            .all(|ty| types.type_named_valtype(ty, set))
    }

    /// Updates the type `id` specified, an identifier for a component instance
    /// type, to be imported into this component.
    ///
    /// Importing an instance type into a component specially handles the
    /// defined resources registered in the instance type. Notably all
    /// defined resources are "freshened" into brand new type variables and
    /// these new variables are substituted within the type. This is what
    /// creates a new `TypeId` and may update the `id` specified.
    ///
    /// One side effect of this operation, for example, is that if an instance
    /// type is used twice to import two different instances then the instances
    /// do not share resource types despite sharing the same original instance
    /// type.
    fn prepare_instance_import(&mut self, id: &mut ComponentInstanceTypeId, types: &mut TypeAlloc) {
        let ty = &types[*id];

        // No special treatment for imports of instances which themselves have
        // no defined resources
        if ty.defined_resources.is_empty() {
            return;
        }

        let mut new_ty = ComponentInstanceType {
            // Copied from the input verbatim
            info: ty.info,

            // Copied over as temporary storage for now, and both of these are
            // filled out and expanded below.
            exports: ty.exports.clone(),
            explicit_resources: ty.explicit_resources.clone(),

            // Explicitly discard this field since the
            // defined resources are lifted into `self`
            defined_resources: Default::default(),
        };

        // Create brand new resources for all defined ones in the instance.
        let resources = (0..ty.defined_resources.len())
            .map(|_| types.alloc_resource_id())
            .collect::<IndexSet<_>>();

        // Build a map from the defined resources in `ty` to those in `new_ty`.
        //
        // As part of this same loop the new resources, which were previously
        // defined in `ty`, now become imported variables in `self`. Their
        // path for where they're imported is updated as well with
        // `self.next_import_index` as the import-to-be soon.
        let mut mapping = Remapping::default();
        let ty = &types[*id];
        for (old, new) in ty.defined_resources.iter().zip(&resources) {
            let prev = mapping.resources.insert(*old, new.resource());
            assert!(prev.is_none());

            let mut base = vec![self.imports.len()];
            base.extend(ty.explicit_resources[old].iter().copied());
            self.imported_resources.insert(new.resource(), base);
        }

        // Using the old-to-new resource mapping perform a substitution on
        // the `exports` and `explicit_resources` fields of `new_ty`
        for ty in new_ty.exports.values_mut() {
            types.remap_component_entity(ty, &mut mapping);
        }
        for (id, path) in mem::take(&mut new_ty.explicit_resources) {
            let id = *mapping.resources.get(&id).unwrap_or(&id);
            new_ty.explicit_resources.insert(id, path);
        }

        // Now that `new_ty` is complete finish its registration and then
        // update `id` on the way out.
        *id = types.push_ty(new_ty);
    }

    /// Prepares an instance type, pointed to `id`, for being exported as a
    /// concrete instance from `self`.
    ///
    /// This will internally perform any resource "freshening" as required and
    /// then additionally update metadata within `self` about resources being
    /// exported or defined.
    fn prepare_instance_export(&mut self, id: &mut ComponentInstanceTypeId, types: &mut TypeAlloc) {
        // Exports of an instance mean that the enclosing context
        // is inheriting the resources that the instance
        // encapsulates. This means that the instance type
        // recorded for this export will itself have no
        // defined resources.
        let ty = &types[*id];

        // Check to see if `defined_resources` is non-empty, and if so then
        // "freshen" all the resources and inherit them to our own defined
        // resources, updating `id` in the process.
        //
        // Note though that this specifically is not rewriting the resources of
        // exported instances. The `defined_resources` set on instance types is
        // a little subtle (see its documentation for more info), but the
        // general idea is that for a concrete instance it's always empty. Only
        // for instance type definitions does it ever have elements in it.
        //
        // That means that if this set is non-empty then what's happening is
        // that we're in a type context an exporting an instance of a previously
        // specified type. In this case all resources are required to be
        // "freshened" to ensure that multiple exports of the same type all
        // export different types of resources.
        //
        // And finally note that this operation empties out the
        // `defined_resources` set of the type that is registered for the
        // instance, as this export is modeled as producing a concrete instance.
        if !ty.defined_resources.is_empty() {
            let mut new_ty = ty.clone();
            let mut mapping = Remapping::default();
            for old in mem::take(&mut new_ty.defined_resources) {
                let new = types.alloc_resource_id();
                mapping.resources.insert(old, new.resource());
                self.defined_resources.insert(new.resource(), None);
            }
            for ty in new_ty.exports.values_mut() {
                types.remap_component_entity(ty, &mut mapping);
            }
            for (id, path) in mem::take(&mut new_ty.explicit_resources) {
                let id = mapping.resources.get(&id).copied().unwrap_or(id);
                new_ty.explicit_resources.insert(id, path);
            }
            *id = types.push_ty(new_ty);
        }

        // Any explicit resources in the instance are now additionally explicit
        // in this component since it's exported.
        //
        // The path to each explicit resources gets one element prepended which
        // is `self.next_export_index`, the index of the export about to be
        // generated.
        let ty = &types[*id];
        for (id, path) in ty.explicit_resources.iter() {
            let mut new_path = vec![self.exports.len()];
            new_path.extend(path);
            self.explicit_resources.insert(*id, new_path);
        }
    }

    pub fn add_export(
        &mut self,
        name: ComponentExportName<'_>,
        mut ty: ComponentEntityType,
        types: &mut TypeAlloc,
        offset: usize,
        check_limit: bool,
    ) -> Result<()> {
        if check_limit {
            check_max(self.exports.len(), 1, MAX_WASM_EXPORTS, "exports", offset)?;
        }
        self.add_entity(&mut ty, Some((name.0, ExternKind::Export)), types, offset)?;
        self.toplevel_exported_resources.validate_extern(
            name.0,
            ExternKind::Export,
            &ty,
            types,
            offset,
            &mut self.export_names,
            &mut self.exports,
            &mut self.type_info,
            &self.features,
        )?;
        Ok(())
    }

    pub fn canonical_function(
        &mut self,
        func: CanonicalFunction,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        match func {
            CanonicalFunction::Lift {
                core_func_index,
                type_index,
                options,
            } => self.lift_function(core_func_index, type_index, &options, types, offset),
            CanonicalFunction::Lower {
                func_index,
                options,
            } => self.lower_function(func_index, &options, types, offset),
            CanonicalFunction::ResourceNew { resource } => {
                self.resource_new(resource, types, offset)
            }
            CanonicalFunction::ResourceDrop { resource } => {
                self.resource_drop(resource, types, offset)
            }
            CanonicalFunction::ResourceDropAsync { resource } => {
                self.resource_drop_async(resource, types, offset)
            }
            CanonicalFunction::ResourceRep { resource } => {
                self.resource_rep(resource, types, offset)
            }
            CanonicalFunction::ThreadSpawnRef { func_ty_index } => {
                self.thread_spawn_ref(func_ty_index, types, offset)
            }
            CanonicalFunction::ThreadSpawnIndirect {
                func_ty_index,
                table_index,
            } => self.thread_spawn_indirect(func_ty_index, table_index, types, offset),
            CanonicalFunction::ThreadAvailableParallelism => {
                self.thread_available_parallelism(types, offset)
            }
            CanonicalFunction::BackpressureInc => self.backpressure_inc(types, offset),
            CanonicalFunction::BackpressureDec => self.backpressure_dec(types, offset),
            CanonicalFunction::TaskReturn { result, options } => {
                self.task_return(&result, &options, types, offset)
            }
            CanonicalFunction::TaskCancel => self.task_cancel(types, offset),
            CanonicalFunction::ContextGet(i) => self.context_get(i, types, offset),
            CanonicalFunction::ContextSet(i) => self.context_set(i, types, offset),
            CanonicalFunction::ThreadYield { cancellable } => {
                self.thread_yield(cancellable, types, offset)
            }
            CanonicalFunction::SubtaskDrop => self.subtask_drop(types, offset),
            CanonicalFunction::SubtaskCancel { async_ } => {
                self.subtask_cancel(async_, types, offset)
            }
            CanonicalFunction::StreamNew { ty } => self.stream_new(ty, types, offset),
            CanonicalFunction::StreamRead { ty, options } => {
                self.stream_read(ty, &options, types, offset)
            }
            CanonicalFunction::StreamWrite { ty, options } => {
                self.stream_write(ty, &options, types, offset)
            }
            CanonicalFunction::StreamCancelRead { ty, async_ } => {
                self.stream_cancel_read(ty, async_, types, offset)
            }
            CanonicalFunction::StreamCancelWrite { ty, async_ } => {
                self.stream_cancel_write(ty, async_, types, offset)
            }
            CanonicalFunction::StreamDropReadable { ty } => {
                self.stream_drop_readable(ty, types, offset)
            }
            CanonicalFunction::StreamDropWritable { ty } => {
                self.stream_drop_writable(ty, types, offset)
            }
            CanonicalFunction::FutureNew { ty } => self.future_new(ty, types, offset),
            CanonicalFunction::FutureRead { ty, options } => {
                self.future_read(ty, &options, types, offset)
            }
            CanonicalFunction::FutureWrite { ty, options } => {
                self.future_write(ty, options.into_vec(), types, offset)
            }
            CanonicalFunction::FutureCancelRead { ty, async_ } => {
                self.future_cancel_read(ty, async_, types, offset)
            }
            CanonicalFunction::FutureCancelWrite { ty, async_ } => {
                self.future_cancel_write(ty, async_, types, offset)
            }
            CanonicalFunction::FutureDropReadable { ty } => {
                self.future_drop_readable(ty, types, offset)
            }
            CanonicalFunction::FutureDropWritable { ty } => {
                self.future_drop_writable(ty, types, offset)
            }
            CanonicalFunction::ErrorContextNew { options } => {
                self.error_context_new(options.into_vec(), types, offset)
            }
            CanonicalFunction::ErrorContextDebugMessage { options } => {
                self.error_context_debug_message(options.into_vec(), types, offset)
            }
            CanonicalFunction::ErrorContextDrop => self.error_context_drop(types, offset),
            CanonicalFunction::WaitableSetNew => self.waitable_set_new(types, offset),
            CanonicalFunction::WaitableSetWait {
                cancellable,
                memory,
            } => self.waitable_set_wait(cancellable, memory, types, offset),
            CanonicalFunction::WaitableSetPoll {
                cancellable,
                memory,
            } => self.waitable_set_poll(cancellable, memory, types, offset),
            CanonicalFunction::WaitableSetDrop => self.waitable_set_drop(types, offset),
            CanonicalFunction::WaitableJoin => self.waitable_join(types, offset),
            CanonicalFunction::ThreadIndex => self.thread_index(types, offset),
            CanonicalFunction::ThreadNewIndirect {
                func_ty_index,
                table_index,
            } => self.thread_new_indirect(func_ty_index, table_index, types, offset),
            CanonicalFunction::ThreadSwitchTo { cancellable } => {
                self.thread_switch_to(cancellable, types, offset)
            }
            CanonicalFunction::ThreadSuspend { cancellable } => {
                self.thread_suspend(cancellable, types, offset)
            }
            CanonicalFunction::ThreadResumeLater => self.thread_resume_later(types, offset),

            CanonicalFunction::ThreadYieldTo { cancellable } => {
                self.thread_yield_to(cancellable, types, offset)
            }
        }
    }

    fn lift_function(
        &mut self,
        core_func_index: u32,
        type_index: u32,
        options: &[CanonicalOption],
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let ty = self.function_type_at(type_index, types, offset)?;
        let core_ty_id = self.core_function_at(core_func_index, offset)?;

        // Lifting a function is for an export, so match the expected canonical ABI
        // export signature
        let mut options = self.check_options(types, options, offset)?;
        options.check_lift(types, self, core_ty_id, offset)?;
        let func_ty = ty.lower(types, &options, Abi::Lift, offset)?;
        let lowered_core_ty_id = func_ty.intern(types, offset);

        if core_ty_id == lowered_core_ty_id {
            self.funcs
                .push(self.types[type_index as usize].unwrap_func());
            return Ok(());
        }

        let ty = types[core_ty_id].unwrap_func();
        let lowered_ty = types[lowered_core_ty_id].unwrap_func();

        if lowered_ty.params() != ty.params() {
            bail!(
                offset,
                "lowered parameter types `{:?}` do not match parameter types `{:?}` of \
                     core function {core_func_index}",
                lowered_ty.params(),
                ty.params()
            );
        }

        if lowered_ty.results() != ty.results() {
            bail!(
                offset,
                "lowered result types `{:?}` do not match result types `{:?}` of \
                     core function {core_func_index}",
                lowered_ty.results(),
                ty.results()
            );
        }

        // Otherwise, must be different rec groups or subtyping (which isn't
        // supported yet) or something.
        bail!(
            offset,
            "lowered function type `{:?}` does not match type `{:?}` of \
                 core function {core_func_index}",
            types[lowered_core_ty_id],
            types[core_ty_id],
        );
    }

    fn lower_function(
        &mut self,
        func_index: u32,
        options: &[CanonicalOption],
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let ty = &types[self.function_at(func_index, offset)?];

        // Lowering a function is for an import, so use a function type that matches
        // the expected canonical ABI import signature.
        let options = self.check_options(types, options, offset)?;
        options.check_lower(offset)?;
        let func_ty = ty.lower(types, &options, Abi::Lower, offset)?;
        let ty_id = func_ty.intern(types, offset);

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn resource_new(&mut self, resource: u32, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        let rep = self.check_local_resource(resource, types, offset)?;
        let id = types.intern_func_type(FuncType::new([rep], [ValType::I32]), offset);
        self.core_funcs.push(id);
        Ok(())
    }

    fn resource_drop(&mut self, resource: u32, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        self.resource_at(resource, types, offset)?;
        let id = types.intern_func_type(FuncType::new([ValType::I32], []), offset);
        self.core_funcs.push(id);
        Ok(())
    }

    fn resource_drop_async(
        &mut self,
        resource: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async_builtins() {
            bail!(
                offset,
                "`resource.drop` as `async` requires the component model async builtins feature"
            )
        }
        self.resource_at(resource, types, offset)?;
        let id = types.intern_func_type(FuncType::new([ValType::I32], []), offset);
        self.core_funcs.push(id);
        Ok(())
    }

    fn resource_rep(&mut self, resource: u32, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        let rep = self.check_local_resource(resource, types, offset)?;
        let id = types.intern_func_type(FuncType::new([ValType::I32], [rep]), offset);
        self.core_funcs.push(id);
        Ok(())
    }

    fn backpressure_inc(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`backpressure.inc` requires the component model async feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], []), offset));
        Ok(())
    }

    fn backpressure_dec(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`backpressure.dec` requires the component model async feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], []), offset));
        Ok(())
    }

    fn task_return(
        &mut self,
        result: &Option<crate::ComponentValType>,
        options: &[CanonicalOption],
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`task.return` requires the component model async feature"
            )
        }

        let func_ty = ComponentFuncType {
            async_: false,
            info: TypeInfo::new(),
            params: result
                .iter()
                .map(|ty| {
                    Ok((
                        KebabString::new("v").unwrap(),
                        match ty {
                            crate::ComponentValType::Primitive(ty) => {
                                ComponentValType::Primitive(*ty)
                            }
                            crate::ComponentValType::Type(index) => {
                                ComponentValType::Type(self.defined_type_at(*index, offset)?)
                            }
                        },
                    ))
                })
                .collect::<Result<_>>()?,
            result: None,
        };

        let options = self.check_options(types, options, offset)?;
        if options.realloc.is_some() {
            bail!(offset, "cannot specify `realloc` option on `task.return`")
        }
        if options.post_return.is_some() {
            bail!(
                offset,
                "cannot specify `post-return` option on `task.return`"
            )
        }
        options.check_lower(offset)?;
        options.require_sync(offset, "task.return")?;

        let func_ty = func_ty.lower(types, &options, Abi::Lower, offset)?;
        let ty_id = func_ty.intern(types, offset);

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn task_cancel(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`task.cancel` requires the component model async feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], []), offset));
        Ok(())
    }

    fn validate_context_immediate(
        &self,
        immediate: u32,
        operation: &str,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_threading() && immediate > 0 {
            bail!(offset, "`{operation}` immediate must be zero: {immediate}")
        } else if immediate > 1 {
            bail!(
                offset,
                "`{operation}` immediate must be zero or one: {immediate}"
            )
        }
        Ok(())
    }

    fn context_get(&mut self, i: u32, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`context.get` requires the component model async feature"
            )
        }
        self.validate_context_immediate(i, "context.get", offset)?;

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], [ValType::I32]), offset));
        Ok(())
    }

    fn context_set(&mut self, i: u32, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`context.set` requires the component model async feature"
            )
        }
        self.validate_context_immediate(i, "context.set", offset)?;

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn thread_yield(
        &mut self,
        cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`thread.yield` requires the component model async feature"
            )
        }
        if cancellable && !self.features.cm_async_stackful() {
            bail!(
                offset,
                "cancellable `thread.yield` requires the component model async stackful feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], [ValType::I32]), offset));
        Ok(())
    }

    fn subtask_drop(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`subtask.drop` requires the component model async feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn subtask_cancel(&mut self, async_: bool, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`subtask.cancel` requires the component model async feature"
            )
        }
        if async_ && !self.features.cm_async_builtins() {
            bail!(
                offset,
                "async `subtask.cancel` requires the component model async builtins feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], [ValType::I32]), offset));
        Ok(())
    }

    fn stream_new(&mut self, ty: u32, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`stream.new` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Stream(_) = &types[ty] else {
            bail!(offset, "`stream.new` requires a stream type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], [ValType::I64]), offset));
        Ok(())
    }

    fn stream_read(
        &mut self,
        ty: u32,
        options: &[CanonicalOption],
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`stream.read` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Stream(elem_ty) = &types[ty] else {
            bail!(offset, "`stream.read` requires a stream type")
        };

        let ty_id = self
            .check_options(types, options, offset)?
            .require_memory_if(offset, || elem_ty.is_some())?
            .require_realloc_if(offset, || elem_ty.is_some_and(|ty| ty.contains_ptr(types)))?
            .check_lower(offset)?
            .check_core_type(
                types,
                FuncType::new([ValType::I32; 3], [ValType::I32]),
                offset,
            )?;

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn stream_write(
        &mut self,
        ty: u32,
        options: &[CanonicalOption],
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`stream.write` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Stream(elem_ty) = &types[ty] else {
            bail!(offset, "`stream.write` requires a stream type")
        };

        let ty_id = self
            .check_options(types, options, offset)?
            .require_memory_if(offset, || elem_ty.is_some())?
            .check_lower(offset)?
            .check_core_type(
                types,
                FuncType::new([ValType::I32; 3], [ValType::I32]),
                offset,
            )?;

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn stream_cancel_read(
        &mut self,
        ty: u32,
        cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`stream.cancel-read` requires the component model async feature"
            )
        }
        if cancellable && !self.features.cm_async_builtins() {
            bail!(
                offset,
                "async `stream.cancel-read` requires the component model async builtins feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Stream(_) = &types[ty] else {
            bail!(offset, "`stream.cancel-read` requires a stream type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], [ValType::I32]), offset));
        Ok(())
    }

    fn stream_cancel_write(
        &mut self,
        ty: u32,
        cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`stream.cancel-write` requires the component model async feature"
            )
        }
        if cancellable && !self.features.cm_async_builtins() {
            bail!(
                offset,
                "async `stream.cancel-write` requires the component model async builtins feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Stream(_) = &types[ty] else {
            bail!(offset, "`stream.cancel-write` requires a stream type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], [ValType::I32]), offset));
        Ok(())
    }

    fn stream_drop_readable(
        &mut self,
        ty: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`stream.drop-readable` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Stream(_) = &types[ty] else {
            bail!(offset, "`stream.drop-readable` requires a stream type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn stream_drop_writable(
        &mut self,
        ty: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`stream.drop-writable` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Stream(_) = &types[ty] else {
            bail!(offset, "`stream.drop-writable` requires a stream type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn future_new(&mut self, ty: u32, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`future.new` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Future(_) = &types[ty] else {
            bail!(offset, "`future.new` requires a future type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], [ValType::I64]), offset));
        Ok(())
    }

    fn future_read(
        &mut self,
        ty: u32,
        options: &[CanonicalOption],
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`future.read` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Future(elem_ty) = &types[ty] else {
            bail!(offset, "`future.read` requires a future type")
        };

        let ty_id = self
            .check_options(types, options, offset)?
            .require_memory_if(offset, || elem_ty.is_some())?
            .require_realloc_if(offset, || elem_ty.is_some_and(|ty| ty.contains_ptr(types)))?
            .check_lower(offset)?
            .check_core_type(
                types,
                FuncType::new([ValType::I32; 2], [ValType::I32]),
                offset,
            )?;

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn future_write(
        &mut self,
        ty: u32,
        options: Vec<CanonicalOption>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`future.write` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Future(elem_ty) = &types[ty] else {
            bail!(offset, "`future.write` requires a future type")
        };

        let ty_id = self
            .check_options(types, &options, offset)?
            .require_memory_if(offset, || elem_ty.is_some())?
            .check_core_type(
                types,
                FuncType::new([ValType::I32; 2], [ValType::I32]),
                offset,
            )?;

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn future_cancel_read(
        &mut self,
        ty: u32,
        cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`future.cancel-read` requires the component model async feature"
            )
        }
        if cancellable && !self.features.cm_async_builtins() {
            bail!(
                offset,
                "async `future.cancel-read` requires the component model async builtins feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Future(_) = &types[ty] else {
            bail!(offset, "`future.cancel-read` requires a future type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], [ValType::I32]), offset));
        Ok(())
    }

    fn future_cancel_write(
        &mut self,
        ty: u32,
        cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`future.cancel-write` requires the component model async feature"
            )
        }
        if cancellable && !self.features.cm_async_builtins() {
            bail!(
                offset,
                "async `future.cancel-write` requires the component model async builtins feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Future(_) = &types[ty] else {
            bail!(offset, "`future.cancel-write` requires a future type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], [ValType::I32]), offset));
        Ok(())
    }

    fn future_drop_readable(
        &mut self,
        ty: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`future.drop-readable` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Future(_) = &types[ty] else {
            bail!(offset, "`future.drop-readable` requires a future type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn future_drop_writable(
        &mut self,
        ty: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`future.drop-writable` requires the component model async feature"
            )
        }

        let ty = self.defined_type_at(ty, offset)?;
        let ComponentDefinedType::Future(_) = &types[ty] else {
            bail!(offset, "`future.drop-writable` requires a future type")
        };

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn error_context_new(
        &mut self,
        options: Vec<CanonicalOption>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_error_context() {
            bail!(
                offset,
                "`error-context.new` requires the component model error-context feature"
            )
        }

        let ty_id = self
            .check_options(types, &options, offset)?
            .require_memory(offset)?
            .require_sync(offset, "error-context.new")?
            .check_lower(offset)?
            .check_core_type(
                types,
                FuncType::new([ValType::I32; 2], [ValType::I32]),
                offset,
            )?;

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn error_context_debug_message(
        &mut self,
        options: Vec<CanonicalOption>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_error_context() {
            bail!(
                offset,
                "`error-context.debug-message` requires the component model error-context feature"
            )
        }

        let ty_id = self
            .check_options(types, &options, offset)?
            .require_memory(offset)?
            .require_realloc(offset)?
            .require_sync(offset, "error-context.debug-message")?
            .check_lower(offset)?
            .check_core_type(types, FuncType::new([ValType::I32; 2], []), offset)?;

        self.core_funcs.push(ty_id);
        Ok(())
    }

    fn error_context_drop(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_error_context() {
            bail!(
                offset,
                "`error-context.drop` requires the component model error-context feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn waitable_set_new(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`waitable-set.new` requires the component model async feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], [ValType::I32]), offset));
        Ok(())
    }

    fn waitable_set_wait(
        &mut self,
        cancellable: bool,
        memory: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`waitable-set.wait` requires the component model async feature"
            )
        }
        if cancellable && !self.features.cm_async_stackful() {
            bail!(
                offset,
                "cancellable `waitable-set.wait` requires the component model async stackful feature"
            )
        }

        self.cabi_memory_at(memory, offset)?;

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32; 2], [ValType::I32]), offset));
        Ok(())
    }

    fn waitable_set_poll(
        &mut self,
        cancellable: bool,
        memory: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`waitable-set.poll` requires the component model async feature"
            )
        }
        if cancellable && !self.features.cm_async_stackful() {
            bail!(
                offset,
                "cancellable `waitable-set.poll` requires the component model async stackful feature"
            )
        }

        self.cabi_memory_at(memory, offset)?;

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32; 2], [ValType::I32]), offset));
        Ok(())
    }

    fn waitable_set_drop(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`waitable-set.drop` requires the component model async feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn waitable_join(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_async() {
            bail!(
                offset,
                "`waitable.join` requires the component model async feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32; 2], []), offset));
        Ok(())
    }

    fn thread_index(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_threading() {
            bail!(
                offset,
                "`thread.index` requires the component model threading feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], [ValType::I32]), offset));
        Ok(())
    }

    fn thread_new_indirect(
        &mut self,
        func_ty_index: u32,
        table_index: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_threading() {
            bail!(
                offset,
                "`thread.new-indirect` requires the component model threading feature"
            )
        }

        let core_type_id = match self.core_type_at(func_ty_index, offset)? {
            ComponentCoreTypeId::Sub(c) => c,
            ComponentCoreTypeId::Module(_) => bail!(offset, "expected a core function type"),
        };
        let sub_ty = &types[core_type_id];
        match &sub_ty.composite_type.inner {
            CompositeInnerType::Func(func_ty) => {
                if func_ty.params() != [ValType::I32] {
                    bail!(
                        offset,
                        "start function must take a single `i32` argument (currently)"
                    );
                }
                if func_ty.results() != [] {
                    bail!(offset, "start function must not return any values");
                }
            }
            _ => bail!(offset, "start type must be a function"),
        }

        let table = self.table_at(table_index, offset)?;

        SubtypeCx::table_type(
            table,
            &TableType {
                initial: 0,
                maximum: None,
                table64: false,
                shared: false,
                element_type: RefType::FUNCREF,
            },
            offset,
        )
        .map_err(|mut e| {
            e.add_context("table is not a 32-bit table of (ref null (func))".into());
            e
        })?;

        self.core_funcs.push(types.intern_func_type(
            FuncType::new([ValType::I32, ValType::I32], [ValType::I32]),
            offset,
        ));
        Ok(())
    }

    fn thread_switch_to(
        &mut self,
        _cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_threading() {
            bail!(
                offset,
                "`thread.switch_to` requires the component model threading feature"
            )
        }

        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], [ValType::I32]), offset));
        Ok(())
    }

    fn thread_suspend(
        &mut self,
        _cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_threading() {
            bail!(
                offset,
                "`thread.suspend` requires the component model threading feature"
            )
        }
        self.core_funcs
            .push(types.intern_func_type(FuncType::new([], [ValType::I32]), offset));
        Ok(())
    }

    fn thread_resume_later(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.cm_threading() {
            bail!(
                offset,
                "`thread.resume_later` requires the component model threading feature"
            )
        }
        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], []), offset));
        Ok(())
    }

    fn thread_yield_to(
        &mut self,
        _cancellable: bool,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_threading() {
            bail!(
                offset,
                "`thread.yield_to` requires the component model threading feature"
            )
        }
        self.core_funcs
            .push(types.intern_func_type(FuncType::new([ValType::I32], [ValType::I32]), offset));
        Ok(())
    }

    fn check_local_resource(&self, idx: u32, types: &TypeList, offset: usize) -> Result<ValType> {
        let resource = self.resource_at(idx, types, offset)?;
        match self
            .defined_resources
            .get(&resource.resource())
            .and_then(|rep| *rep)
        {
            Some(ty) => Ok(ty),
            None => bail!(offset, "type {idx} is not a local resource"),
        }
    }

    fn resource_at<'a>(
        &self,
        idx: u32,
        _types: &'a TypeList,
        offset: usize,
    ) -> Result<AliasableResourceId> {
        if let ComponentAnyTypeId::Resource(id) = self.component_type_at(idx, offset)? {
            return Ok(id);
        }
        bail!(offset, "type index {} is not a resource type", idx)
    }

    fn thread_spawn_ref(
        &mut self,
        func_ty_index: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.shared_everything_threads() {
            bail!(
                offset,
                "`thread.spawn-ref` requires the shared-everything-threads proposal"
            )
        }
        let core_type_id = self.validate_spawn_type(func_ty_index, types, offset)?;

        // Insert the core function.
        let packed_index = PackedIndex::from_id(core_type_id).ok_or_else(|| {
            format_err!(offset, "implementation limit: too many types in `TypeList`")
        })?;
        let start_func_ref = RefType::concrete(true, packed_index);
        let func_ty = FuncType::new([ValType::Ref(start_func_ref), ValType::I32], [ValType::I32]);
        let core_ty = SubType::func(func_ty, true);
        let id = types.intern_sub_type(core_ty, offset);
        self.core_funcs.push(id);

        Ok(())
    }

    fn thread_spawn_indirect(
        &mut self,
        func_ty_index: u32,
        table_index: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if !self.features.shared_everything_threads() {
            bail!(
                offset,
                "`thread.spawn-indirect` requires the shared-everything-threads proposal"
            )
        }
        let _ = self.validate_spawn_type(func_ty_index, types, offset)?;

        // Check this much like `call_indirect` (see
        // `OperatorValidatorTemp::check_call_indirect_ty`), but loosen the
        // table type restrictions to just a `funcref`. See the component model
        // for more details:
        // https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#-canon-threadspawn-indirect.
        let table = self.table_at(table_index, offset)?;

        SubtypeCx::table_type(
            table,
            &TableType {
                initial: 0,
                maximum: None,
                table64: false,
                shared: true,
                element_type: RefType::FUNCREF
                    .shared()
                    .expect("a funcref can always be shared"),
            },
            offset,
        )
        .map_err(|mut e| {
            e.add_context("table is not a 32-bit shared table of (ref null (shared func))".into());
            e
        })?;

        // Insert the core function.
        let func_ty = FuncType::new([ValType::I32, ValType::I32], [ValType::I32]);
        let core_ty = SubType::func(func_ty, true);
        let id = types.intern_sub_type(core_ty, offset);
        self.core_funcs.push(id);

        Ok(())
    }

    /// Validates the type of a `thread.spawn*` instruction.
    ///
    /// This is currently limited to shared functions with the signature `[i32]
    /// -> []`. See component model [explanation] for more details.
    ///
    /// [explanation]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#-canon-threadspawn-ref
    fn validate_spawn_type(
        &self,
        func_ty_index: u32,
        types: &TypeAlloc,
        offset: usize,
    ) -> Result<CoreTypeId> {
        let core_type_id = match self.core_type_at(func_ty_index, offset)? {
            ComponentCoreTypeId::Sub(c) => c,
            ComponentCoreTypeId::Module(_) => bail!(offset, "expected a core function type"),
        };
        let sub_ty = &types[core_type_id];
        if !sub_ty.composite_type.shared {
            bail!(offset, "spawn type must be shared");
        }
        match &sub_ty.composite_type.inner {
            CompositeInnerType::Func(func_ty) => {
                if func_ty.params() != [ValType::I32] {
                    bail!(
                        offset,
                        "spawn function must take a single `i32` argument (currently)"
                    );
                }
                if func_ty.results() != [] {
                    bail!(offset, "spawn function must not return any values");
                }
            }
            _ => bail!(offset, "spawn type must be a function"),
        }
        Ok(core_type_id)
    }

    fn thread_available_parallelism(&mut self, types: &mut TypeAlloc, offset: usize) -> Result<()> {
        if !self.features.shared_everything_threads() {
            bail!(
                offset,
                "`thread.available_parallelism` requires the shared-everything-threads proposal"
            )
        }

        let func_ty = FuncType::new([], [ValType::I32]);
        let core_ty = SubType::func(func_ty, true);
        let id = types.intern_sub_type(core_ty, offset);
        self.core_funcs.push(id);

        Ok(())
    }

    pub fn add_component(&mut self, component: ComponentType, types: &mut TypeAlloc) -> Result<()> {
        let id = types.push_ty(component);
        self.components.push(id);
        Ok(())
    }

    pub fn add_instance(
        &mut self,
        instance: crate::ComponentInstance,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let instance = match instance {
            crate::ComponentInstance::Instantiate {
                component_index,
                args,
            } => self.instantiate_component(component_index, args.into_vec(), types, offset)?,
            crate::ComponentInstance::FromExports(exports) => {
                self.instantiate_component_exports(exports.into_vec(), types, offset)?
            }
        };

        self.instances.push(instance);

        Ok(())
    }

    pub fn add_alias(
        components: &mut [Self],
        alias: crate::ComponentAlias,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        match alias {
            crate::ComponentAlias::InstanceExport {
                instance_index,
                kind,
                name,
            } => components.last_mut().unwrap().alias_instance_export(
                instance_index,
                kind,
                name,
                types,
                offset,
            ),
            crate::ComponentAlias::CoreInstanceExport {
                instance_index,
                kind,
                name,
            } => components.last_mut().unwrap().alias_core_instance_export(
                instance_index,
                kind,
                name,
                types,
                offset,
            ),
            crate::ComponentAlias::Outer { kind, count, index } => match kind {
                ComponentOuterAliasKind::CoreModule => {
                    Self::alias_module(components, count, index, offset)
                }
                ComponentOuterAliasKind::CoreType => {
                    Self::alias_core_type(components, count, index, offset)
                }
                ComponentOuterAliasKind::Type => {
                    Self::alias_type(components, count, index, types, offset)
                }
                ComponentOuterAliasKind::Component => {
                    Self::alias_component(components, count, index, offset)
                }
            },
        }
    }

    pub fn add_start(
        &mut self,
        func_index: u32,
        args: &[u32],
        results: u32,
        types: &mut TypeList,
        offset: usize,
    ) -> Result<()> {
        if !self.features.cm_values() {
            bail!(
                offset,
                "support for component model `value`s is not enabled"
            );
        }
        if self.has_start {
            return Err(BinaryReaderError::new(
                "component cannot have more than one start function",
                offset,
            ));
        }

        let ft = &types[self.function_at(func_index, offset)?];

        if ft.params.len() != args.len() {
            bail!(
                offset,
                "component start function requires {} arguments but was given {}",
                ft.params.len(),
                args.len()
            );
        }

        if u32::from(ft.result.is_some()) != results {
            bail!(
                offset,
                "component start function has a result count of {results} \
                 but the function type has a result count of {type_results}",
                type_results = u32::from(ft.result.is_some()),
            );
        }

        let cx = SubtypeCx::new(types, types);
        for (i, ((_, ty), arg)) in ft.params.iter().zip(args).enumerate() {
            // Ensure the value's type is a subtype of the parameter type
            cx.component_val_type(self.value_at(*arg, offset)?, ty, offset)
                .with_context(|| {
                    format!("value type mismatch for component start function argument {i}")
                })?;
        }

        if let Some(ty) = ft.result {
            self.values.push((ty, false));
        }

        self.has_start = true;

        Ok(())
    }

    fn check_options(
        &self,
        types: &TypeList,
        options: &[CanonicalOption],
        offset: usize,
    ) -> Result<CanonicalOptions> {
        fn display(option: CanonicalOption) -> &'static str {
            match option {
                CanonicalOption::UTF8 => "utf8",
                CanonicalOption::UTF16 => "utf16",
                CanonicalOption::CompactUTF16 => "latin1-utf16",
                CanonicalOption::Memory(_) => "memory",
                CanonicalOption::Realloc(_) => "realloc",
                CanonicalOption::PostReturn(_) => "post-return",
                CanonicalOption::Async => "async",
                CanonicalOption::Callback(_) => "callback",
                CanonicalOption::CoreType(_) => "core-type",
                CanonicalOption::Gc => "gc",
            }
        }

        let mut encoding = None;
        let mut memory = None;
        let mut realloc = None;
        let mut post_return = None;
        let mut is_async = false;
        let mut callback = None;
        let mut core_type = None;
        let mut gc = false;

        for option in options {
            match option {
                CanonicalOption::UTF8 | CanonicalOption::UTF16 | CanonicalOption::CompactUTF16 => {
                    match encoding {
                        Some(existing) => {
                            bail!(
                                offset,
                                "canonical encoding option `{existing}` conflicts with option `{}`",
                                display(*option),
                            )
                        }
                        None => {
                            encoding = Some(match option {
                                CanonicalOption::UTF8 => StringEncoding::Utf8,
                                CanonicalOption::UTF16 => StringEncoding::Utf16,
                                CanonicalOption::CompactUTF16 => StringEncoding::CompactUtf16,
                                _ => unreachable!(),
                            });
                        }
                    }
                }
                CanonicalOption::Memory(idx) => {
                    memory = match memory {
                        None => {
                            self.cabi_memory_at(*idx, offset)?;
                            Some(*idx)
                        }
                        Some(_) => {
                            return Err(BinaryReaderError::new(
                                "canonical option `memory` is specified more than once",
                                offset,
                            ));
                        }
                    }
                }
                CanonicalOption::Realloc(idx) => {
                    realloc = match realloc {
                        None => {
                            let ty_id = self.core_function_at(*idx, offset)?;
                            let func_ty = types[ty_id].unwrap_func();
                            if func_ty.params()
                                != [ValType::I32, ValType::I32, ValType::I32, ValType::I32]
                                || func_ty.results() != [ValType::I32]
                            {
                                return Err(BinaryReaderError::new(
                                    "canonical option `realloc` uses a core function with an incorrect signature",
                                    offset,
                                ));
                            }
                            Some(*idx)
                        }
                        Some(_) => {
                            return Err(BinaryReaderError::new(
                                "canonical option `realloc` is specified more than once",
                                offset,
                            ));
                        }
                    }
                }
                CanonicalOption::PostReturn(idx) => {
                    post_return = match post_return {
                        None => Some(*idx),
                        Some(_) => {
                            return Err(BinaryReaderError::new(
                                "canonical option `post-return` is specified more than once",
                                offset,
                            ));
                        }
                    }
                }
                CanonicalOption::Async => {
                    if is_async {
                        return Err(BinaryReaderError::new(
                            "canonical option `async` is specified more than once",
                            offset,
                        ));
                    } else {
                        if !self.features.cm_async() {
                            bail!(
                                offset,
                                "canonical option `async` requires the component model async feature"
                            );
                        }

                        is_async = true;
                    }
                }
                CanonicalOption::Callback(idx) => {
                    callback = match callback {
                        None => Some(*idx),
                        Some(_) => {
                            return Err(BinaryReaderError::new(
                                "canonical option `callback` is specified more than once",
                                offset,
                            ));
                        }
                    }
                }
                CanonicalOption::CoreType(idx) => {
                    core_type = match core_type {
                        None => {
                            if !self.features.cm_gc() {
                                bail!(
                                    offset,
                                    "canonical option `core type` requires the component model gc feature"
                                )
                            }
                            let ty = match self.core_type_at(*idx, offset)? {
                                ComponentCoreTypeId::Sub(ty) => ty,
                                ComponentCoreTypeId::Module(_) => {
                                    return Err(BinaryReaderError::new(
                                        "canonical option `core type` must reference a core function \
                                     type",
                                        offset,
                                    ));
                                }
                            };
                            match &types[ty].composite_type.inner {
                                CompositeInnerType::Func(_) => {}
                                CompositeInnerType::Array(_)
                                | CompositeInnerType::Struct(_)
                                | CompositeInnerType::Cont(_) => {
                                    return Err(BinaryReaderError::new(
                                        "canonical option `core type` must reference a core function \
                                     type",
                                        offset,
                                    ));
                                }
                            }
                            Some(ty)
                        }
                        Some(_) => {
                            return Err(BinaryReaderError::new(
                                "canonical option `core type` is specified more than once",
                                offset,
                            ));
                        }
                    };
                }
                CanonicalOption::Gc => {
                    if gc {
                        return Err(BinaryReaderError::new(
                            "canonical option `gc` is specified more than once",
                            offset,
                        ));
                    }
                    if !self.features.cm_gc() {
                        return Err(BinaryReaderError::new(
                            "canonical option `gc` requires the `cm-gc` feature",
                            offset,
                        ));
                    }
                    gc = true;
                }
            }
        }

        let string_encoding = encoding.unwrap_or_default();

        let concurrency = match (is_async, callback, post_return.is_some()) {
            (false, Some(_), _) => {
                bail!(offset, "cannot specify callback without async")
            }
            (true, _, true) => {
                bail!(offset, "cannot specify post-return function in async")
            }
            (false, None, _) => Concurrency::Sync,
            (true, callback, false) => Concurrency::Async { callback },
        };

        if !gc && core_type.is_some() {
            bail!(offset, "cannot specify `core-type` without `gc`")
        }

        Ok(CanonicalOptions {
            string_encoding,
            memory,
            realloc,
            post_return,
            concurrency,
            core_type,
            gc,
        })
    }

    fn check_type_ref(
        &mut self,
        ty: &ComponentTypeRef,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentEntityType> {
        Ok(match ty {
            ComponentTypeRef::Module(index) => {
                let id = self.core_type_at(*index, offset)?;
                match id {
                    ComponentCoreTypeId::Sub(_) => {
                        bail!(offset, "core type index {index} is not a module type")
                    }
                    ComponentCoreTypeId::Module(id) => ComponentEntityType::Module(id),
                }
            }
            ComponentTypeRef::Func(index) => {
                let id = self.component_type_at(*index, offset)?;
                match id {
                    ComponentAnyTypeId::Func(id) => ComponentEntityType::Func(id),
                    _ => bail!(offset, "type index {index} is not a function type"),
                }
            }
            ComponentTypeRef::Value(ty) => {
                self.check_value_support(offset)?;
                let ty = match ty {
                    crate::ComponentValType::Primitive(ty) => ComponentValType::Primitive(*ty),
                    crate::ComponentValType::Type(index) => {
                        ComponentValType::Type(self.defined_type_at(*index, offset)?)
                    }
                };
                ComponentEntityType::Value(ty)
            }
            ComponentTypeRef::Type(TypeBounds::Eq(index)) => {
                let referenced = self.component_type_at(*index, offset)?;
                let created = types.with_unique(referenced);
                ComponentEntityType::Type {
                    referenced,
                    created,
                }
            }
            ComponentTypeRef::Type(TypeBounds::SubResource) => {
                let id = types.alloc_resource_id();
                ComponentEntityType::Type {
                    referenced: id.into(),
                    created: id.into(),
                }
            }
            ComponentTypeRef::Instance(index) => {
                let id = self.component_type_at(*index, offset)?;
                match id {
                    ComponentAnyTypeId::Instance(id) => ComponentEntityType::Instance(id),
                    _ => bail!(offset, "type index {index} is not an instance type"),
                }
            }
            ComponentTypeRef::Component(index) => {
                let id = self.component_type_at(*index, offset)?;
                match id {
                    ComponentAnyTypeId::Component(id) => ComponentEntityType::Component(id),
                    _ => bail!(offset, "type index {index} is not a component type"),
                }
            }
        })
    }

    pub fn export_to_entity_type(
        &mut self,
        export: &crate::ComponentExport,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentEntityType> {
        let actual = match export.kind {
            ComponentExternalKind::Module => {
                ComponentEntityType::Module(self.module_at(export.index, offset)?)
            }
            ComponentExternalKind::Func => {
                ComponentEntityType::Func(self.function_at(export.index, offset)?)
            }
            ComponentExternalKind::Value => {
                self.check_value_support(offset)?;
                ComponentEntityType::Value(*self.value_at(export.index, offset)?)
            }
            ComponentExternalKind::Type => {
                let referenced = self.component_type_at(export.index, offset)?;
                let created = types.with_unique(referenced);
                ComponentEntityType::Type {
                    referenced,
                    created,
                }
            }
            ComponentExternalKind::Instance => {
                ComponentEntityType::Instance(self.instance_at(export.index, offset)?)
            }
            ComponentExternalKind::Component => {
                ComponentEntityType::Component(self.component_at(export.index, offset)?)
            }
        };

        let ascribed = match &export.ty {
            Some(ty) => self.check_type_ref(ty, types, offset)?,
            None => return Ok(actual),
        };

        SubtypeCx::new(types, types)
            .component_entity_type(&actual, &ascribed, offset)
            .with_context(|| "ascribed type of export is not compatible with item's type")?;

        Ok(ascribed)
    }

    fn create_module_type(
        components: &[Self],
        decls: Vec<crate::ModuleTypeDeclaration>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ModuleType> {
        let mut state = Module::new(components[0].features);

        for decl in decls {
            match decl {
                crate::ModuleTypeDeclaration::Type(rec) => {
                    state.add_types(rec, types, offset, true)?;
                }
                crate::ModuleTypeDeclaration::Export { name, mut ty } => {
                    let ty = state.check_type_ref(&mut ty, types, offset)?;
                    state.add_export(name, ty, offset, true, types)?;
                }
                crate::ModuleTypeDeclaration::OuterAlias { kind, count, index } => {
                    match kind {
                        crate::OuterAliasKind::Type => {
                            let ty = if count == 0 {
                                // Local alias, check the local module state
                                ComponentCoreTypeId::Sub(state.type_id_at(index, offset)?)
                            } else {
                                // Otherwise, check the enclosing component state
                                let component =
                                    Self::check_alias_count(components, count - 1, offset)?;
                                component.core_type_at(index, offset)?
                            };

                            check_max(state.types.len(), 1, MAX_WASM_TYPES, "types", offset)?;

                            match ty {
                                ComponentCoreTypeId::Sub(ty) => state.types.push(ty),
                                // TODO https://github.com/WebAssembly/component-model/issues/265
                                ComponentCoreTypeId::Module(_) => bail!(
                                    offset,
                                    "not implemented: aliasing core module types into a core \
                                     module's types index space"
                                ),
                            }
                        }
                    }
                }
                crate::ModuleTypeDeclaration::Import(import) => {
                    state.add_import(import, types, offset)?;
                }
            }
        }

        let imports = state.imports_for_module_type(offset)?;

        Ok(ModuleType {
            info: TypeInfo::core(state.type_size),
            imports,
            exports: state.exports,
        })
    }

    fn create_component_type(
        components: &mut Vec<Self>,
        decls: Vec<crate::ComponentTypeDeclaration>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentType> {
        let features = components[0].features;
        components.push(ComponentState::new(ComponentKind::ComponentType, features));

        for decl in decls {
            match decl {
                crate::ComponentTypeDeclaration::CoreType(ty) => {
                    Self::add_core_type(components, ty, types, offset, true)?;
                }
                crate::ComponentTypeDeclaration::Type(ty) => {
                    Self::add_type(components, ty, types, offset, true)?;
                }
                crate::ComponentTypeDeclaration::Export { name, ty } => {
                    let current = components.last_mut().unwrap();
                    let ty = current.check_type_ref(&ty, types, offset)?;
                    current.add_export(name, ty, types, offset, true)?;
                }
                crate::ComponentTypeDeclaration::Import(import) => {
                    components
                        .last_mut()
                        .unwrap()
                        .add_import(import, types, offset)?;
                }
                crate::ComponentTypeDeclaration::Alias(alias) => {
                    Self::add_alias(components, alias, types, offset)?;
                }
            };
        }

        components.pop().unwrap().finish(types, offset)
    }

    fn create_instance_type(
        components: &mut Vec<Self>,
        decls: Vec<crate::InstanceTypeDeclaration>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentInstanceType> {
        let features = components[0].features;
        components.push(ComponentState::new(ComponentKind::InstanceType, features));

        for decl in decls {
            match decl {
                crate::InstanceTypeDeclaration::CoreType(ty) => {
                    Self::add_core_type(components, ty, types, offset, true)?;
                }
                crate::InstanceTypeDeclaration::Type(ty) => {
                    Self::add_type(components, ty, types, offset, true)?;
                }
                crate::InstanceTypeDeclaration::Export { name, ty } => {
                    let current = components.last_mut().unwrap();
                    let ty = current.check_type_ref(&ty, types, offset)?;
                    current.add_export(name, ty, types, offset, true)?;
                }
                crate::InstanceTypeDeclaration::Alias(alias) => {
                    Self::add_alias(components, alias, types, offset)?;
                }
            };
        }

        let mut state = components.pop().unwrap();

        assert!(state.imported_resources.is_empty());

        Ok(ComponentInstanceType {
            info: state.type_info,

            // The defined resources for this instance type are those listed on
            // the component state. The path to each defined resource is
            // guaranteed to live within the `explicit_resources` map since,
            // when in the type context, the introduction of any defined
            // resource must have been done with `(export "x" (type (sub
            // resource)))` which, in a sense, "fuses" the introduction of the
            // variable with the export. This means that all defined resources,
            // if any, should be guaranteed to have an `explicit_resources` path
            // listed.
            defined_resources: mem::take(&mut state.defined_resources)
                .into_iter()
                .map(|(id, rep)| {
                    assert!(rep.is_none());
                    id
                })
                .collect(),

            // The map of what resources are explicitly exported and where
            // they're exported is plumbed through as-is.
            explicit_resources: mem::take(&mut state.explicit_resources),

            exports: mem::take(&mut state.exports),
        })
    }

    fn create_function_type(
        &self,
        ty: crate::ComponentFuncType,
        types: &TypeList,
        offset: usize,
    ) -> Result<ComponentFuncType> {
        let mut info = TypeInfo::new();

        if ty.async_ && !self.features.cm_async() {
            bail!(
                offset,
                "async component functions require the component model async feature"
            );
        }

        let mut set = Set::default();
        set.reserve(core::cmp::max(
            ty.params.len(),
            usize::from(ty.result.is_some()),
        ));

        let params = ty
            .params
            .iter()
            .map(|(name, ty)| {
                let name: &KebabStr = to_kebab_str(name, "function parameter", offset)?;
                if !set.insert(name) {
                    bail!(
                        offset,
                        "function parameter name `{name}` conflicts with previous parameter name `{prev}`",
                        prev = set.get(&name).unwrap(),
                    );
                }

                let ty = self.create_component_val_type(*ty, offset)?;
                info.combine(ty.info(types), offset)?;
                Ok((name.to_owned(), ty))
            })
            .collect::<Result<_>>()?;

        set.clear();

        let result = ty
            .result
            .map(|ty| {
                let ty = self.create_component_val_type(ty, offset)?;
                let ty_info = ty.info(types);
                if ty_info.contains_borrow() {
                    bail!(offset, "function result cannot contain a `borrow` type");
                }
                info.combine(ty.info(types), offset)?;
                Ok(ty)
            })
            .transpose()?;

        Ok(ComponentFuncType {
            async_: ty.async_,
            info,
            params,
            result,
        })
    }

    fn instantiate_core_module(
        &self,
        module_index: u32,
        module_args: Vec<crate::InstantiationArg>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentCoreInstanceTypeId> {
        fn insert_arg<'a>(
            name: &'a str,
            arg: &'a InstanceType,
            args: &mut IndexMap<&'a str, &'a InstanceType>,
            offset: usize,
        ) -> Result<()> {
            if args.insert(name, arg).is_some() {
                bail!(
                    offset,
                    "duplicate module instantiation argument named `{name}`"
                );
            }

            Ok(())
        }

        let module_type_id = self.module_at(module_index, offset)?;
        let mut args = IndexMap::default();

        // Populate the arguments
        for module_arg in module_args {
            match module_arg.kind {
                InstantiationArgKind::Instance => {
                    let instance_type = &types[self.core_instance_at(module_arg.index, offset)?];
                    insert_arg(module_arg.name, instance_type, &mut args, offset)?;
                }
            }
        }

        // Validate the arguments
        let module_type = &types[module_type_id];
        let cx = SubtypeCx::new(types, types);
        for ((module, name), expected) in module_type.imports.iter() {
            let instance = args.get(module.as_str()).ok_or_else(|| {
                format_err!(
                    offset,
                    "missing module instantiation argument named `{module}`"
                )
            })?;

            let arg = instance
                .internal_exports(types)
                .get(name.as_str())
                .ok_or_else(|| {
                    format_err!(
                        offset,
                        "module instantiation argument `{module}` does not \
                         export an item named `{name}`",
                    )
                })?;

            cx.entity_type(arg, expected, offset).with_context(|| {
                format!(
                    "type mismatch for export `{name}` of module \
                     instantiation argument `{module}`"
                )
            })?;
        }

        let mut info = TypeInfo::new();
        for (_, ty) in module_type.exports.iter() {
            info.combine(ty.info(types), offset)?;
        }

        Ok(types.push_ty(InstanceType {
            info,
            kind: CoreInstanceTypeKind::Instantiated(module_type_id),
        }))
    }

    fn instantiate_component(
        &mut self,
        component_index: u32,
        component_args: Vec<crate::ComponentInstantiationArg>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentInstanceTypeId> {
        let component_type_id = self.component_at(component_index, offset)?;
        let mut args = IndexMap::default();

        // Populate the arguments
        for component_arg in component_args {
            let ty = match component_arg.kind {
                ComponentExternalKind::Module => {
                    ComponentEntityType::Module(self.module_at(component_arg.index, offset)?)
                }
                ComponentExternalKind::Component => {
                    ComponentEntityType::Component(self.component_at(component_arg.index, offset)?)
                }
                ComponentExternalKind::Instance => {
                    ComponentEntityType::Instance(self.instance_at(component_arg.index, offset)?)
                }
                ComponentExternalKind::Func => {
                    ComponentEntityType::Func(self.function_at(component_arg.index, offset)?)
                }
                ComponentExternalKind::Value => {
                    self.check_value_support(offset)?;
                    ComponentEntityType::Value(*self.value_at(component_arg.index, offset)?)
                }
                ComponentExternalKind::Type => {
                    let ty = self.component_type_at(component_arg.index, offset)?;
                    ComponentEntityType::Type {
                        referenced: ty,
                        created: ty,
                    }
                }
            };
            match args.entry(component_arg.name.to_string()) {
                Entry::Occupied(e) => {
                    bail!(
                        offset,
                        "instantiation argument `{name}` conflicts with previous argument `{prev}`",
                        prev = e.key(),
                        name = component_arg.name
                    );
                }
                Entry::Vacant(e) => {
                    e.insert(ty);
                }
            }
        }

        // Here comes the fun part of the component model, we're instantiating
        // the component with type `component_type_id` with the `args`
        // specified. Easy enough!
        //
        // This operation, however, is one of the lynchpins of safety in the
        // component model. Additionally what this ends up implementing ranges
        // from "well just check the types are equal" to "let's have a
        // full-blown ML-style module type system in the component model". There
        // are primarily two major tricky pieces to the component model which
        // make this operation, instantiating components, hard:
        //
        // 1. Components can import and exports other components. This means
        //    that arguments to instantiation are along the lines of functions
        //    being passed to functions or similar. Effectively this means that
        //    the term "variance" comes into play with either contravariance
        //    or covariance depending on where you are in typechecking. This is
        //    one of the main rationales, however, that this check below is a
        //    check for subtyping as opposed to exact type equivalence. For
        //    example an instance that exports something is a subtype of an
        //    instance that exports nothing. Components get a bit trick since
        //    they both have imports and exports. My way of thinking about it
        //    is "who's asking for what". If you're asking for imports then
        //    I need to at least supply those imports, but I can possibly
        //    supply more. If you're asking for a thing which you'll give a set
        //    of imports, then I can give you something which takes less imports
        //    because what you give still suffices. (things like that). The
        //    real complication with components, however, comes with...
        //
        // 2. Resources. Resources in the component model are akin to "abstract
        //    types". They're not abstract in the sense that they have no
        //    representation, they're always backed by a 32-bit integer right
        //    now. Instead they're abstract in the sense that some components
        //    aren't allowed to understand the representation of a resource.
        //    For example if you import a resource you can't get the underlying
        //    internals of it. Furthermore the resource is strictly tracked
        //    within the component with `own` and `borrow` runtime semantics.
        //    The hardest part about resources, though, is handling them as
        //    part of instantiation and subtyping.
        //
        //    For example one major aspect of resources is that if a component
        //    exports a resource then each instantiation of the component
        //    produces a fresh resource type. This means that the type recorded
        //    for the instantiation here can't simply be "I instantiated
        //    component X" since in such a situation the type of all
        //    instantiations would be the same, which they aren't.
        //
        //    This sort of subtlety comes up quite frequently for resources.
        //    This file contains references to `imported_resources` and
        //    `defined_resources` for example which refer to the formal
        //    nature of components and their abstract variables. Specifically
        //    for instantiation though we're eventually faced with the problem
        //    of subtype checks where resource subtyping is defined as "does
        //    your id equal mine". Naively implemented that means anything with
        //    resources isn't subtypes of anything else since resource ids are
        //    unique between components. Instead what actually needs to happen
        //    is types need to be substituted.
        //
        // Much of the complexity here is not actually apparent here in this
        // literal one function. Instead it's spread out across validation
        // in this file and type-checking in the `types.rs` module. Note that
        // the "spread out" nature isn't because we're bad maintainers
        // (hopefully), but rather it's quite infectious how many parts need
        // to handle resources and account for defined/imported variables.
        //
        // For example only one subtyping method is called here where `args` is
        // passed in. This method is quite recursive in its nature though and
        // will internally touch all the fields that this file maintains to
        // end up putting into various bits and pieces of type information.
        //
        // Unfortunately there's probably not really a succinct way to read
        // this method and understand everything. If you've written ML module
        // type systems this will probably look quite familiar, but otherwise
        // the whole system is not really easily approachable at this time. It's
        // hoped in the future that there's a formalism to refer to which will
        // make things more clear as the code would be able to reference this
        // hypothetical formalism. Until that's the case, though, these
        // comments are hopefully enough when augmented with communication with
        // the authors.

        let component_type = &types[component_type_id];
        let mut exports = component_type.exports.clone();
        let mut info = TypeInfo::new();
        for (_, ty) in component_type.exports.iter() {
            info.combine(ty.info(types), offset)?;
        }

        // Perform the subtype check that `args` matches the imports of
        // `component_type_id`. The result of this subtype check is the
        // production of a mapping of resource types from the imports to the
        // arguments provided. This is a substitution map which is then used
        // below to perform a substitution into the exports of the instance
        // since the types of the exports are now in terms of whatever was
        // supplied as imports.
        let mut mapping = SubtypeCx::new(types, types).open_instance_type(
            &args,
            component_type_id,
            ExternKind::Import,
            offset,
        )?;

        // Part of the instantiation of a component is that all of its
        // defined resources become "fresh" on each instantiation. This
        // means that each instantiation of a component gets brand new type
        // variables representing its defined resources, modeling that each
        // instantiation produces distinct types. The freshening is performed
        // here by allocating new ids and inserting them into `mapping`.
        //
        // Note that technically the `mapping` from subtyping should be applied
        // first and then the mapping for freshening should be applied
        // afterwards. The keys of the map from subtyping are the imported
        // resources from this component which are disjoint from its defined
        // resources. That means it should be possible to place everything
        // into one large map which maps from:
        //
        // * the component's imported resources go to whatever was explicitly
        //   supplied in the import map
        // * the component's defined resources go to fresh new resources
        //
        // These two remapping operations can then get folded into one by
        // placing everything in the same `mapping` and using that for a remap
        // only once.
        let fresh_defined_resources = (0..component_type.defined_resources.len())
            .map(|_| types.alloc_resource_id().resource())
            .collect::<IndexSet<_>>();
        let component_type = &types[component_type_id];
        for ((old, _path), new) in component_type
            .defined_resources
            .iter()
            .zip(&fresh_defined_resources)
        {
            let prev = mapping.resources.insert(*old, *new);
            assert!(prev.is_none());
        }

        // Perform the remapping operation over all the exports that will be
        // listed for the final instance type. Note that this is performed
        // both for all the export types in addition to the explicitly exported
        // resources list.
        //
        // Note that this is a crucial step of the instantiation process which
        // is intentionally transforming the type of a component based on the
        // variables provided by imports and additionally ensuring that all
        // references to the component's defined resources are rebound to the
        // fresh ones introduced just above.
        for entity in exports.values_mut() {
            types.remap_component_entity(entity, &mut mapping);
        }
        let component_type = &types[component_type_id];
        let explicit_resources = component_type
            .explicit_resources
            .iter()
            .map(|(id, path)| {
                (
                    mapping.resources.get(id).copied().unwrap_or(*id),
                    path.clone(),
                )
            })
            .collect::<IndexMap<_, _>>();

        // Technically in the last formalism that was consulted in writing this
        // implementation there are two further steps that are part of the
        // instantiation process:
        //
        // 1. The set of defined resources from the instance created, which are
        //    added to the outer component, is the subset of the instance's
        //    original defined resources and the free variables of the exports.
        //
        // 2. Each element of this subset is required to be "explicit in" the
        //    instance, or otherwise explicitly exported somewhere within the
        //    instance.
        //
        // With the syntactic structure of the component model, however, neither
        // of these conditions should be necessary. The main reason for this is
        // that this function is specifically dealing with instantiation of
        // components which should already have these properties validated
        // about them. Subsequently we shouldn't have to re-check them.
        //
        // In debug mode, however, do a sanity check.
        if cfg!(debug_assertions) {
            let mut free = IndexSet::default();
            for ty in exports.values() {
                types.free_variables_component_entity(ty, &mut free);
            }
            assert!(fresh_defined_resources.is_subset(&free));
            for resource in fresh_defined_resources.iter() {
                assert!(explicit_resources.contains_key(resource));
            }
        }

        // And as the final step of the instantiation process all of the
        // new defined resources from this component instantiation are moved
        // onto `self`. Note that concrete instances never have defined
        // resources (see more comments in `instantiate_exports`) so the
        // `defined_resources` listing in the final type is always empty. This
        // represents how by having a concrete instance the definitions
        // referred to in that instance are now problems for the outer
        // component rather than the inner instance since the instance is bound
        // to the component.
        //
        // All defined resources here have no known representation, so they're
        // all listed with `None`. Also note that none of the resources were
        // exported yet so `self.explicit_resources` is not updated yet. If
        // this instance is exported, however, it'll consult the type's
        // `explicit_resources` array and use that appropriately.
        for resource in fresh_defined_resources {
            self.defined_resources.insert(resource, None);
        }

        Ok(types.push_ty(ComponentInstanceType {
            info,
            defined_resources: Default::default(),
            explicit_resources,
            exports,
        }))
    }

    fn instantiate_component_exports(
        &mut self,
        exports: Vec<crate::ComponentExport>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentInstanceTypeId> {
        let mut info = TypeInfo::new();
        let mut inst_exports = IndexMap::default();
        let mut explicit_resources = IndexMap::default();
        let mut export_names = IndexSet::default();

        // NB: It's intentional that this context is empty since no indices are
        // introduced in the bag-of-exports construct which means there's no
        // way syntactically to register something inside of this.
        let names = ComponentNameContext::default();

        for export in exports {
            assert!(export.ty.is_none());
            let ty = match export.kind {
                ComponentExternalKind::Module => {
                    ComponentEntityType::Module(self.module_at(export.index, offset)?)
                }
                ComponentExternalKind::Component => {
                    ComponentEntityType::Component(self.component_at(export.index, offset)?)
                }
                ComponentExternalKind::Instance => {
                    let ty = self.instance_at(export.index, offset)?;

                    // When an instance is exported from an instance then
                    // all explicitly exported resources on the sub-instance are
                    // now also listed as exported resources on the outer
                    // instance, just with one more element in their path.
                    explicit_resources.extend(types[ty].explicit_resources.iter().map(
                        |(id, path)| {
                            let mut new_path = vec![inst_exports.len()];
                            new_path.extend(path);
                            (*id, new_path)
                        },
                    ));
                    ComponentEntityType::Instance(ty)
                }
                ComponentExternalKind::Func => {
                    ComponentEntityType::Func(self.function_at(export.index, offset)?)
                }
                ComponentExternalKind::Value => {
                    self.check_value_support(offset)?;
                    ComponentEntityType::Value(*self.value_at(export.index, offset)?)
                }
                ComponentExternalKind::Type => {
                    let ty = self.component_type_at(export.index, offset)?;
                    // If this is an export of a resource type be sure to
                    // record that in the explicit list with the appropriate
                    // path because if this instance ends up getting used
                    // it'll count towards the "explicit in" check.
                    if let ComponentAnyTypeId::Resource(id) = ty {
                        explicit_resources.insert(id.resource(), vec![inst_exports.len()]);
                    }
                    ComponentEntityType::Type {
                        referenced: ty,
                        // The created type index here isn't used anywhere
                        // in index spaces because a "bag of exports"
                        // doesn't build up its own index spaces. Just fill
                        // in the same index here in this case as what's
                        // referenced.
                        created: ty,
                    }
                }
            };

            names.validate_extern(
                export.name.0,
                ExternKind::Export,
                &ty,
                types,
                offset,
                &mut export_names,
                &mut inst_exports,
                &mut info,
                &self.features,
            )?;
        }

        Ok(types.push_ty(ComponentInstanceType {
            info,
            explicit_resources,
            exports: inst_exports,

            // NB: the list of defined resources for this instance itself
            // is always empty. Even if this instance exports resources,
            // it's empty.
            //
            // The reason for this is a bit subtle. The general idea, though, is
            // that the defined resources list here is only used for instance
            // types that are sort of "floating around" and haven't actually
            // been attached to something yet. For example when an instance type
            // is simply declared it can have defined resources introduced
            // through `(export "name" (type (sub resource)))`. These
            // definitions, however, are local to the instance itself and aren't
            // defined elsewhere.
            //
            // Here, though, no new definitions were introduced. The instance
            // created here is a "bag of exports" which could only refer to
            // preexisting items. This means that inherently no new resources
            // were created so there's nothing to put in this list. Any
            // resources referenced by the instance must be bound by the outer
            // component context or further above.
            //
            // Furthermore, however, actual instances of instances, which this
            // is, aren't allowed to have defined resources. Instead the
            // resources would have to be injected into the outer component
            // enclosing the instance. That means that even if bag-of-exports
            // could declare a new resource then the resource would be moved
            // from here to `self.defined_resources`. This doesn't exist at this
            // time, though, so this still remains empty and
            // `self.defined_resources` remains unperturbed.
            defined_resources: Default::default(),
        }))
    }

    fn instantiate_core_exports(
        &mut self,
        exports: Vec<crate::Export>,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<ComponentCoreInstanceTypeId> {
        fn insert_export(
            types: &TypeList,
            name: &str,
            export: EntityType,
            exports: &mut IndexMap<String, EntityType>,
            info: &mut TypeInfo,
            offset: usize,
        ) -> Result<()> {
            info.combine(export.info(types), offset)?;

            if exports.insert(name.to_string(), export).is_some() {
                bail!(
                    offset,
                    "duplicate instantiation export name `{name}` already defined",
                )
            }

            Ok(())
        }

        let mut info = TypeInfo::new();
        let mut inst_exports = IndexMap::default();
        for export in exports {
            match export.kind {
                ExternalKind::Func | ExternalKind::FuncExact => {
                    insert_export(
                        types,
                        export.name,
                        EntityType::Func(self.core_function_at(export.index, offset)?),
                        &mut inst_exports,
                        &mut info,
                        offset,
                    )?;
                }
                ExternalKind::Table => insert_export(
                    types,
                    export.name,
                    EntityType::Table(*self.table_at(export.index, offset)?),
                    &mut inst_exports,
                    &mut info,
                    offset,
                )?,
                ExternalKind::Memory => insert_export(
                    types,
                    export.name,
                    EntityType::Memory(*self.memory_at(export.index, offset)?),
                    &mut inst_exports,
                    &mut info,
                    offset,
                )?,
                ExternalKind::Global => {
                    insert_export(
                        types,
                        export.name,
                        EntityType::Global(*self.global_at(export.index, offset)?),
                        &mut inst_exports,
                        &mut info,
                        offset,
                    )?;
                }
                ExternalKind::Tag => {
                    if !self.features.exceptions() {
                        bail!(offset, "exceptions proposal not enabled");
                    }
                    insert_export(
                        types,
                        export.name,
                        EntityType::Tag(self.tag_at(export.index, offset)?),
                        &mut inst_exports,
                        &mut info,
                        offset,
                    )?
                }
            }
        }

        Ok(types.push_ty(InstanceType {
            info,
            kind: CoreInstanceTypeKind::Exports(inst_exports),
        }))
    }

    fn alias_core_instance_export(
        &mut self,
        instance_index: u32,
        kind: ExternalKind,
        name: &str,
        types: &TypeList,
        offset: usize,
    ) -> Result<()> {
        macro_rules! push_module_export {
            ($expected:path, $collection:ident, $ty:literal) => {{
                match self.core_instance_export(instance_index, name, types, offset)? {
                    $expected(ty) => {
                        self.$collection.push(*ty);
                    }
                    _ => {
                        bail!(
                            offset,
                            "export `{name}` for core instance {instance_index} is not a {}",
                            $ty
                        )
                    }
                }
            }};
        }

        match kind {
            ExternalKind::Func | ExternalKind::FuncExact => {
                check_max(
                    self.function_count(),
                    1,
                    MAX_WASM_FUNCTIONS,
                    "functions",
                    offset,
                )?;
                push_module_export!(EntityType::Func, core_funcs, "function");
            }
            ExternalKind::Table => {
                check_max(
                    self.core_tables.len(),
                    1,
                    MAX_CORE_INDEX_SPACE_ITEMS,
                    "tables",
                    offset,
                )?;
                push_module_export!(EntityType::Table, core_tables, "table");
            }
            ExternalKind::Memory => {
                check_max(
                    self.core_memories.len(),
                    1,
                    MAX_CORE_INDEX_SPACE_ITEMS,
                    "memories",
                    offset,
                )?;
                push_module_export!(EntityType::Memory, core_memories, "memory");
            }
            ExternalKind::Global => {
                check_max(
                    self.core_globals.len(),
                    1,
                    MAX_CORE_INDEX_SPACE_ITEMS,
                    "globals",
                    offset,
                )?;
                push_module_export!(EntityType::Global, core_globals, "global");
            }
            ExternalKind::Tag => {
                if !self.features.exceptions() {
                    bail!(offset, "exceptions proposal not enabled");
                }
                check_max(
                    self.core_tags.len(),
                    1,
                    MAX_CORE_INDEX_SPACE_ITEMS,
                    "tags",
                    offset,
                )?;
                push_module_export!(EntityType::Tag, core_tags, "tag");
            }
        }

        Ok(())
    }

    fn alias_instance_export(
        &mut self,
        instance_index: u32,
        kind: ComponentExternalKind,
        name: &str,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        if let ComponentExternalKind::Value = kind {
            self.check_value_support(offset)?;
        }
        let mut ty = match types[self.instance_at(instance_index, offset)?]
            .exports
            .get(name)
        {
            Some(ty) => *ty,
            None => bail!(
                offset,
                "instance {instance_index} has no export named `{name}`"
            ),
        };

        let ok = match (&ty, kind) {
            (ComponentEntityType::Module(_), ComponentExternalKind::Module) => true,
            (ComponentEntityType::Module(_), _) => false,
            (ComponentEntityType::Component(_), ComponentExternalKind::Component) => true,
            (ComponentEntityType::Component(_), _) => false,
            (ComponentEntityType::Func(_), ComponentExternalKind::Func) => true,
            (ComponentEntityType::Func(_), _) => false,
            (ComponentEntityType::Instance(_), ComponentExternalKind::Instance) => true,
            (ComponentEntityType::Instance(_), _) => false,
            (ComponentEntityType::Value(_), ComponentExternalKind::Value) => true,
            (ComponentEntityType::Value(_), _) => false,
            (ComponentEntityType::Type { .. }, ComponentExternalKind::Type) => true,
            (ComponentEntityType::Type { .. }, _) => false,
        };
        if !ok {
            bail!(
                offset,
                "export `{name}` for instance {instance_index} is not a {}",
                kind.desc(),
            );
        }

        self.add_entity(&mut ty, None, types, offset)?;
        Ok(())
    }

    fn alias_module(components: &mut [Self], count: u32, index: u32, offset: usize) -> Result<()> {
        let component = Self::check_alias_count(components, count, offset)?;
        let ty = component.module_at(index, offset)?;

        let current = components.last_mut().unwrap();
        check_max(
            current.core_modules.len(),
            1,
            MAX_WASM_MODULES,
            "modules",
            offset,
        )?;

        current.core_modules.push(ty);
        Ok(())
    }

    fn alias_component(
        components: &mut [Self],
        count: u32,
        index: u32,
        offset: usize,
    ) -> Result<()> {
        let component = Self::check_alias_count(components, count, offset)?;
        let ty = component.component_at(index, offset)?;

        let current = components.last_mut().unwrap();
        check_max(
            current.components.len(),
            1,
            MAX_WASM_COMPONENTS,
            "components",
            offset,
        )?;

        current.components.push(ty);
        Ok(())
    }

    fn alias_core_type(
        components: &mut [Self],
        count: u32,
        index: u32,
        offset: usize,
    ) -> Result<()> {
        let component = Self::check_alias_count(components, count, offset)?;
        let ty = component.core_type_at(index, offset)?;

        let current = components.last_mut().unwrap();
        check_max(current.type_count(), 1, MAX_WASM_TYPES, "types", offset)?;

        current.core_types.push(ty);

        Ok(())
    }

    fn alias_type(
        components: &mut [Self],
        count: u32,
        index: u32,
        types: &mut TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let component = Self::check_alias_count(components, count, offset)?;
        let ty = component.component_type_at(index, offset)?;

        // If `count` "crossed a component boundary", meaning that it went from
        // one component to another, then this must additionally verify that
        // `ty` has no free variables with respect to resources. This is
        // intended to preserve the property for components where each component
        // is an isolated unit that can theoretically be extracted from other
        // components. If resources from other components were allowed to leak
        // in then it would prevent that.
        //
        // This check is done by calculating the `pos` within `components` that
        // our target `component` above was selected at. Once this is acquired
        // the component to the "right" is checked, and if that's a component
        // then it's considered as crossing a component boundary meaning the
        // free variables check runs.
        //
        // The reason this works is that in the list of `ComponentState` types
        // it's guaranteed that any `is_type` components are contiguous at the
        // end of the array. This means that if state one level deeper than the
        // target of this alias is a `!is_type` component, then the target must
        // be a component as well. If the one-level deeper state `is_type` then
        // the target is either a type or a component, both of which are valid
        // (as aliases can reach the enclosing component and have as many free
        // variables as they want).
        let pos_after_component = components.len() - (count as usize);
        if let Some(component) = components.get(pos_after_component) {
            if component.kind == ComponentKind::Component {
                let mut free = IndexSet::default();
                types.free_variables_any_type_id(ty, &mut free);
                if !free.is_empty() {
                    bail!(
                        offset,
                        "cannot alias outer type which transitively refers \
                         to resources not defined in the current component"
                    );
                }
            }
        }

        let current = components.last_mut().unwrap();
        check_max(current.type_count(), 1, MAX_WASM_TYPES, "types", offset)?;

        current.types.push(ty);

        Ok(())
    }

    fn check_alias_count(components: &[Self], count: u32, offset: usize) -> Result<&Self> {
        let count = count as usize;
        if count >= components.len() {
            bail!(offset, "invalid outer alias count of {count}");
        }

        Ok(&components[components.len() - count - 1])
    }

    fn create_defined_type(
        &self,
        ty: crate::ComponentDefinedType,
        types: &TypeList,
        offset: usize,
    ) -> Result<ComponentDefinedType> {
        match ty {
            crate::ComponentDefinedType::Primitive(ty) => {
                self.check_primitive_type(ty, offset)?;
                Ok(ComponentDefinedType::Primitive(ty))
            }
            crate::ComponentDefinedType::Record(fields) => {
                self.create_record_type(fields.as_ref(), types, offset)
            }
            crate::ComponentDefinedType::Variant(cases) => {
                self.create_variant_type(cases.as_ref(), types, offset)
            }
            crate::ComponentDefinedType::List(ty) => Ok(ComponentDefinedType::List(
                self.create_component_val_type(ty, offset)?,
            )),
            crate::ComponentDefinedType::Map(key, value) => {
                if !self.features.cm_map() {
                    bail!(offset, "Maps require the component model map feature")
                }
                Ok(ComponentDefinedType::Map(
                    self.create_component_val_type(key, offset)?,
                    self.create_component_val_type(value, offset)?,
                ))
            }
            crate::ComponentDefinedType::FixedSizeList(ty, elements) => {
                if !self.features.cm_fixed_size_list() {
                    bail!(
                        offset,
                        "Fixed size lists require the component model fixed size list feature"
                    )
                }
                if elements < 1 {
                    bail!(offset, "Fixed size lists must have more than zero elements")
                }
                Ok(ComponentDefinedType::FixedSizeList(
                    self.create_component_val_type(ty, offset)?,
                    elements,
                ))
            }
            crate::ComponentDefinedType::Tuple(tys) => {
                self.create_tuple_type(tys.as_ref(), types, offset)
            }
            crate::ComponentDefinedType::Flags(names) => {
                self.create_flags_type(names.as_ref(), offset)
            }
            crate::ComponentDefinedType::Enum(cases) => {
                self.create_enum_type(cases.as_ref(), offset)
            }
            crate::ComponentDefinedType::Option(ty) => Ok(ComponentDefinedType::Option(
                self.create_component_val_type(ty, offset)?,
            )),
            crate::ComponentDefinedType::Result { ok, err } => Ok(ComponentDefinedType::Result {
                ok: ok
                    .map(|ty| self.create_component_val_type(ty, offset))
                    .transpose()?,
                err: err
                    .map(|ty| self.create_component_val_type(ty, offset))
                    .transpose()?,
            }),
            crate::ComponentDefinedType::Own(idx) => Ok(ComponentDefinedType::Own(
                self.resource_at(idx, types, offset)?,
            )),
            crate::ComponentDefinedType::Borrow(idx) => Ok(ComponentDefinedType::Borrow(
                self.resource_at(idx, types, offset)?,
            )),
            crate::ComponentDefinedType::Future(ty) => {
                if !self.features.cm_async() {
                    bail!(
                        offset,
                        "`future` requires the component model async feature"
                    )
                }
                Ok(ComponentDefinedType::Future(
                    ty.map(|ty| self.create_component_val_type(ty, offset))
                        .transpose()?,
                ))
            }
            crate::ComponentDefinedType::Stream(ty) => {
                if !self.features.cm_async() {
                    bail!(
                        offset,
                        "`stream` requires the component model async feature"
                    )
                }
                Ok(ComponentDefinedType::Stream(
                    ty.map(|ty| self.create_component_val_type(ty, offset))
                        .transpose()?,
                ))
            }
        }
    }

    fn create_record_type(
        &self,
        fields: &[(&str, crate::ComponentValType)],
        types: &TypeList,
        offset: usize,
    ) -> Result<ComponentDefinedType> {
        let mut info = TypeInfo::new();
        let mut field_map = IndexMap::default();
        field_map.reserve(fields.len());

        if fields.is_empty() {
            bail!(offset, "record type must have at least one field");
        }

        for (name, ty) in fields {
            let name = to_kebab_str(name, "record field", offset)?;
            let ty = self.create_component_val_type(*ty, offset)?;

            match field_map.entry(name.to_owned()) {
                Entry::Occupied(e) => bail!(
                    offset,
                    "record field name `{name}` conflicts with previous field name `{prev}`",
                    prev = e.key()
                ),
                Entry::Vacant(e) => {
                    info.combine(ty.info(types), offset)?;
                    e.insert(ty);
                }
            }
        }

        Ok(ComponentDefinedType::Record(RecordType {
            info,
            fields: field_map,
        }))
    }

    fn create_variant_type(
        &self,
        cases: &[crate::VariantCase],
        types: &TypeList,
        offset: usize,
    ) -> Result<ComponentDefinedType> {
        let mut info = TypeInfo::new();
        let mut case_map: IndexMap<KebabString, VariantCase> = IndexMap::default();
        case_map.reserve(cases.len());

        if cases.is_empty() {
            bail!(offset, "variant type must have at least one case");
        }

        if cases.len() > u32::MAX as usize {
            return Err(BinaryReaderError::new(
                "variant type cannot be represented with a 32-bit discriminant value",
                offset,
            ));
        }

        for (i, case) in cases.iter().enumerate() {
            if let Some(refines) = case.refines {
                if refines >= i as u32 {
                    return Err(BinaryReaderError::new(
                        "variant case can only refine a previously defined case",
                        offset,
                    ));
                }
            }

            let name = to_kebab_str(case.name, "variant case", offset)?;

            let ty = case
                .ty
                .map(|ty| self.create_component_val_type(ty, offset))
                .transpose()?;

            match case_map.entry(name.to_owned()) {
                Entry::Occupied(e) => bail!(
                    offset,
                    "variant case name `{name}` conflicts with previous case name `{prev}`",
                    name = case.name,
                    prev = e.key()
                ),
                Entry::Vacant(e) => {
                    if let Some(ty) = ty {
                        info.combine(ty.info(types), offset)?;
                    }

                    // Safety: the use of `KebabStr::new_unchecked` here is safe because the string
                    // was already verified to be kebab case.
                    e.insert(VariantCase {
                        ty,
                        refines: case
                            .refines
                            .map(|i| KebabStr::new_unchecked(cases[i as usize].name).to_owned()),
                    });
                }
            }
        }

        Ok(ComponentDefinedType::Variant(VariantType {
            info,
            cases: case_map,
        }))
    }

    fn create_tuple_type(
        &self,
        tys: &[crate::ComponentValType],
        types: &TypeList,
        offset: usize,
    ) -> Result<ComponentDefinedType> {
        let mut info = TypeInfo::new();
        if tys.is_empty() {
            bail!(offset, "tuple type must have at least one type");
        }
        let types = tys
            .iter()
            .map(|ty| {
                let ty = self.create_component_val_type(*ty, offset)?;
                info.combine(ty.info(types), offset)?;
                Ok(ty)
            })
            .collect::<Result<_>>()?;

        Ok(ComponentDefinedType::Tuple(TupleType { info, types }))
    }

    fn create_flags_type(&self, names: &[&str], offset: usize) -> Result<ComponentDefinedType> {
        let mut names_set = IndexSet::default();
        names_set.reserve(names.len());

        if names.is_empty() {
            bail!(offset, "flags must have at least one entry");
        }

        if names.len() > 32 {
            bail!(offset, "cannot have more than 32 flags");
        }

        for name in names {
            let name = to_kebab_str(name, "flag", offset)?;
            if !names_set.insert(name.to_owned()) {
                bail!(
                    offset,
                    "flag name `{name}` conflicts with previous flag name `{prev}`",
                    prev = names_set.get(name).unwrap()
                );
            }
        }

        Ok(ComponentDefinedType::Flags(names_set))
    }

    fn create_enum_type(&self, cases: &[&str], offset: usize) -> Result<ComponentDefinedType> {
        if cases.len() > u32::MAX as usize {
            return Err(BinaryReaderError::new(
                "enumeration type cannot be represented with a 32-bit discriminant value",
                offset,
            ));
        }

        if cases.is_empty() {
            bail!(offset, "enum type must have at least one variant");
        }

        let mut tags = IndexSet::default();
        tags.reserve(cases.len());

        for tag in cases {
            let tag = to_kebab_str(tag, "enum tag", offset)?;
            if !tags.insert(tag.to_owned()) {
                bail!(
                    offset,
                    "enum tag name `{tag}` conflicts with previous tag name `{prev}`",
                    prev = tags.get(tag).unwrap()
                );
            }
        }

        Ok(ComponentDefinedType::Enum(tags))
    }

    fn create_component_val_type(
        &self,
        ty: crate::ComponentValType,
        offset: usize,
    ) -> Result<ComponentValType> {
        Ok(match ty {
            crate::ComponentValType::Primitive(pt) => {
                self.check_primitive_type(pt, offset)?;
                ComponentValType::Primitive(pt)
            }
            crate::ComponentValType::Type(idx) => {
                ComponentValType::Type(self.defined_type_at(idx, offset)?)
            }
        })
    }

    pub fn core_type_at(&self, idx: u32, offset: usize) -> Result<ComponentCoreTypeId> {
        self.core_types
            .get(idx as usize)
            .copied()
            .ok_or_else(|| format_err!(offset, "unknown type {idx}: type index out of bounds"))
    }

    pub fn component_type_at(&self, idx: u32, offset: usize) -> Result<ComponentAnyTypeId> {
        self.types
            .get(idx as usize)
            .copied()
            .ok_or_else(|| format_err!(offset, "unknown type {idx}: type index out of bounds"))
    }

    fn function_type_at<'a>(
        &self,
        idx: u32,
        types: &'a TypeList,
        offset: usize,
    ) -> Result<&'a ComponentFuncType> {
        let id = self.component_type_at(idx, offset)?;
        match id {
            ComponentAnyTypeId::Func(id) => Ok(&types[id]),
            _ => bail!(offset, "type index {idx} is not a function type"),
        }
    }

    fn function_at(&self, idx: u32, offset: usize) -> Result<ComponentFuncTypeId> {
        self.funcs.get(idx as usize).copied().ok_or_else(|| {
            format_err!(
                offset,
                "unknown function {idx}: function index out of bounds"
            )
        })
    }

    fn component_at(&self, idx: u32, offset: usize) -> Result<ComponentTypeId> {
        self.components.get(idx as usize).copied().ok_or_else(|| {
            format_err!(
                offset,
                "unknown component {idx}: component index out of bounds"
            )
        })
    }

    fn instance_at(&self, idx: u32, offset: usize) -> Result<ComponentInstanceTypeId> {
        self.instances.get(idx as usize).copied().ok_or_else(|| {
            format_err!(
                offset,
                "unknown instance {idx}: instance index out of bounds"
            )
        })
    }

    fn value_at(&mut self, idx: u32, offset: usize) -> Result<&ComponentValType> {
        match self.values.get_mut(idx as usize) {
            Some((ty, used)) if !*used => {
                *used = true;
                Ok(ty)
            }
            Some(_) => bail!(offset, "value {idx} cannot be used more than once"),
            None => bail!(offset, "unknown value {idx}: value index out of bounds"),
        }
    }

    fn defined_type_at(&self, idx: u32, offset: usize) -> Result<ComponentDefinedTypeId> {
        match self.component_type_at(idx, offset)? {
            ComponentAnyTypeId::Defined(id) => Ok(id),
            _ => bail!(offset, "type index {idx} is not a defined type"),
        }
    }

    fn core_function_at(&self, idx: u32, offset: usize) -> Result<CoreTypeId> {
        match self.core_funcs.get(idx as usize) {
            Some(id) => Ok(*id),
            None => bail!(
                offset,
                "unknown core function {idx}: function index out of bounds"
            ),
        }
    }

    fn module_at(&self, idx: u32, offset: usize) -> Result<ComponentCoreModuleTypeId> {
        match self.core_modules.get(idx as usize) {
            Some(id) => Ok(*id),
            None => bail!(offset, "unknown module {idx}: module index out of bounds"),
        }
    }

    fn core_instance_at(&self, idx: u32, offset: usize) -> Result<ComponentCoreInstanceTypeId> {
        match self.core_instances.get(idx as usize) {
            Some(id) => Ok(*id),
            None => bail!(
                offset,
                "unknown core instance {idx}: instance index out of bounds"
            ),
        }
    }

    fn core_instance_export<'a>(
        &self,
        instance_index: u32,
        name: &str,
        types: &'a TypeList,
        offset: usize,
    ) -> Result<&'a EntityType> {
        match types[self.core_instance_at(instance_index, offset)?]
            .internal_exports(types)
            .get(name)
        {
            Some(export) => Ok(export),
            None => bail!(
                offset,
                "core instance {instance_index} has no export named `{name}`"
            ),
        }
    }

    fn global_at(&self, idx: u32, offset: usize) -> Result<&GlobalType> {
        match self.core_globals.get(idx as usize) {
            Some(t) => Ok(t),
            None => bail!(offset, "unknown global {idx}: global index out of bounds"),
        }
    }

    fn table_at(&self, idx: u32, offset: usize) -> Result<&TableType> {
        match self.core_tables.get(idx as usize) {
            Some(t) => Ok(t),
            None => bail!(offset, "unknown table {idx}: table index out of bounds"),
        }
    }

    fn memory_at(&self, idx: u32, offset: usize) -> Result<&MemoryType> {
        match self.core_memories.get(idx as usize) {
            Some(t) => Ok(t),
            None => bail!(offset, "unknown memory {idx}: memory index out of bounds"),
        }
    }

    fn tag_at(&self, idx: u32, offset: usize) -> Result<CoreTypeId> {
        match self.core_tags.get(idx as usize) {
            Some(t) => Ok(*t),
            None => bail!(offset, "unknown tag {idx}: tag index out of bounds"),
        }
    }

    /// Validates that the linear memory at `idx` is valid to use as a canonical
    /// ABI memory.
    ///
    /// At this time this requires that the memory is a plain 32-bit linear
    /// memory. Notably this disallows shared memory and 64-bit linear memories.
    fn cabi_memory_at(&self, idx: u32, offset: usize) -> Result<()> {
        let ty = self.memory_at(idx, offset)?;
        SubtypeCx::memory_type(
            ty,
            &MemoryType {
                initial: 0,
                maximum: None,
                memory64: false,
                shared: false,
                page_size_log2: None,
            },
            offset,
        )
        .map_err(|mut e| {
            e.add_context("canonical ABI memory is not a 32-bit linear memory".into());
            e
        })
    }

    /// Completes the translation of this component, performing final
    /// validation of its structure.
    ///
    /// This method is required to be called for translating all components.
    /// Internally this will convert local data structures into a
    /// `ComponentType` which is suitable to use to describe the type of this
    /// component.
    pub fn finish(&mut self, types: &TypeAlloc, offset: usize) -> Result<ComponentType> {
        let mut ty = ComponentType {
            // Inherit some fields based on translation of the component.
            info: self.type_info,
            imports: self.imports.clone(),
            exports: self.exports.clone(),

            // This is filled in as a subset of `self.defined_resources`
            // depending on what's actually used by the exports. See the
            // bottom of this function.
            defined_resources: Default::default(),

            // These are inherited directly from what was calculated for this
            // component.
            imported_resources: mem::take(&mut self.imported_resources)
                .into_iter()
                .collect(),
            explicit_resources: mem::take(&mut self.explicit_resources),
        };

        // Collect all "free variables", or resources, from the imports of this
        // component. None of the resources defined within this component can
        // be used as part of the exports. This set is then used to reject any
        // of `self.defined_resources` which show up.
        let mut free = IndexSet::default();
        for ty in ty.imports.values() {
            types.free_variables_component_entity(ty, &mut free);
        }
        for (resource, _path) in self.defined_resources.iter() {
            // FIXME: this error message is quite opaque and doesn't indicate
            // more contextual information such as:
            //
            // * what was the exported resource found in the imports
            // * which import was the resource found within
            //
            // These are possible to calculate here if necessary, however.
            if free.contains(resource) {
                bail!(offset, "local resource type found in imports");
            }
        }

        // The next step in validation a component, with respect to resources,
        // is to minimize the set of defined resources to only those that
        // are actually used by the exports. This weeds out resources that are
        // defined, used within a component, and never exported, for example.
        //
        // The free variables of all exports are inserted into the `free` set
        // (which is reused from the imports after clearing it). The defined
        // resources calculated for this component are then inserted into this
        // type's list of defined resources if it's contained somewhere in
        // the free variables.
        //
        // Note that at the same time all defined resources must be exported,
        // somehow, transitively from this component. The `explicit_resources`
        // map is consulted for this purpose which lists all explicitly
        // exported resources in the component, regardless from whence they
        // came. If not present in this map then it's not exported and an error
        // is returned.
        //
        // NB: the "types are exported" check is probably sufficient nowadays
        // that the check of the `explicit_resources` map is probably not
        // necessary, but it's left here for completeness and out of an
        // abundance of caution.
        free.clear();
        for ty in ty.exports.values() {
            types.free_variables_component_entity(ty, &mut free);
        }
        for (id, _rep) in mem::take(&mut self.defined_resources) {
            if !free.contains(&id) {
                continue;
            }

            let path = match ty.explicit_resources.get(&id).cloned() {
                Some(path) => path,
                // FIXME: this error message is quite opaque and doesn't
                // indicate more contextual information such as:
                //
                // * which resource wasn't found in an export
                // * which export has a reference to the resource
                //
                // These are possible to calculate here if necessary, however.
                None => bail!(
                    offset,
                    "local resource type found in export but not exported itself"
                ),
            };

            ty.defined_resources.push((id, path));
        }

        Ok(ty)
    }

    fn check_value_support(&self, offset: usize) -> Result<()> {
        if !self.features.cm_values() {
            bail!(
                offset,
                "support for component model `value`s is not enabled"
            );
        }
        Ok(())
    }

    fn check_primitive_type(&self, ty: crate::PrimitiveValType, offset: usize) -> Result<()> {
        if ty == crate::PrimitiveValType::ErrorContext && !self.features.cm_error_context() {
            bail!(
                offset,
                "`error-context` requires the component model error-context feature"
            )
        }
        Ok(())
    }
}

impl InternRecGroup for ComponentState {
    fn features(&self) -> &WasmFeatures {
        &self.features
    }

    fn add_type_id(&mut self, id: CoreTypeId) {
        self.core_types.push(ComponentCoreTypeId::Sub(id));
    }

    fn type_id_at(&self, idx: u32, offset: usize) -> Result<CoreTypeId> {
        match self.core_type_at(idx, offset)? {
            ComponentCoreTypeId::Sub(id) => Ok(id),
            ComponentCoreTypeId::Module(_) => {
                bail!(offset, "type index {idx} is a module type, not a sub type");
            }
        }
    }

    fn types_len(&self) -> u32 {
        u32::try_from(self.core_types.len()).unwrap()
    }
}

impl ComponentNameContext {
    /// Registers that the resource `id` is named `name` within this context.
    fn register(&mut self, name: &str, id: AliasableResourceId) {
        let idx = self.all_resource_names.len();
        let prev = self.resource_name_map.insert(id, idx);
        assert!(
            prev.is_none(),
            "for {id:?}, inserted {idx:?} but already had {prev:?}"
        );
        self.all_resource_names.insert(name.to_string());
    }

    fn validate_extern(
        &self,
        name: &str,
        kind: ExternKind,
        ty: &ComponentEntityType,
        types: &TypeAlloc,
        offset: usize,
        kind_names: &mut IndexSet<ComponentName>,
        items: &mut IndexMap<String, ComponentEntityType>,
        info: &mut TypeInfo,
        features: &WasmFeatures,
    ) -> Result<()> {
        // First validate that `name` is even a valid kebab name, meaning it's
        // in kebab-case, is an ID, etc.
        let kebab = ComponentName::new_with_features(name, offset, *features)
            .with_context(|| format!("{} name `{name}` is not a valid extern name", kind.desc()))?;

        if let ExternKind::Export = kind {
            match kebab.kind() {
                ComponentNameKind::Label(_)
                | ComponentNameKind::Method(_)
                | ComponentNameKind::Static(_)
                | ComponentNameKind::Constructor(_)
                | ComponentNameKind::Interface(_) => {}

                ComponentNameKind::Hash(_)
                | ComponentNameKind::Url(_)
                | ComponentNameKind::Dependency(_) => {
                    bail!(offset, "name `{name}` is not a valid export name")
                }
            }
        }

        // Validate that the kebab name, if it has structure such as
        // `[method]a.b`, is indeed valid with respect to known resources.
        self.validate(&kebab, ty, types, offset)
            .with_context(|| format!("{} name `{kebab}` is not valid", kind.desc()))?;

        // Top-level kebab-names must all be unique, even between both imports
        // and exports ot a component. For those names consult the `kebab_names`
        // set.
        if let Some(prev) = kind_names.replace(kebab.clone()) {
            bail!(
                offset,
                "{} name `{kebab}` conflicts with previous name `{prev}`",
                kind.desc()
            );
        }

        // Otherwise all strings must be unique, regardless of their name, so
        // consult the `items` set to ensure that we're not for example
        // importing the same interface ID twice.
        match items.entry(name.to_string()) {
            Entry::Occupied(e) => {
                bail!(
                    offset,
                    "{kind} name `{name}` conflicts with previous name `{prev}`",
                    kind = kind.desc(),
                    prev = e.key(),
                );
            }
            Entry::Vacant(e) => {
                e.insert(*ty);
                info.combine(ty.info(types), offset)?;
            }
        }
        Ok(())
    }

    /// Validates that the `name` provided is allowed to have the type `ty`.
    fn validate(
        &self,
        name: &ComponentName,
        ty: &ComponentEntityType,
        types: &TypeAlloc,
        offset: usize,
    ) -> Result<()> {
        let func = || {
            let id = match ty {
                ComponentEntityType::Func(id) => *id,
                _ => bail!(offset, "item is not a func"),
            };
            Ok(&types[id])
        };

        match name.kind() {
            // No validation necessary for these styles of names
            ComponentNameKind::Label(_)
            | ComponentNameKind::Interface(_)
            | ComponentNameKind::Url(_)
            | ComponentNameKind::Dependency(_)
            | ComponentNameKind::Hash(_) => {}

            // Constructors must return `(own $resource)` or
            // `(result (own $Tresource))` and the `$resource` must be named
            // within this context to match `rname`.
            ComponentNameKind::Constructor(rname) => {
                let ty = func()?;
                if ty.async_ {
                    bail!(offset, "constructor function cannot be async");
                }
                let ty = match ty.result {
                    Some(result) => result,
                    None => bail!(offset, "function should return one value"),
                };
                let resource = match ty {
                    ComponentValType::Primitive(_) => None,
                    ComponentValType::Type(ty) => match &types[ty] {
                        ComponentDefinedType::Own(id) => Some(id),
                        ComponentDefinedType::Result {
                            ok: Some(ComponentValType::Type(ok)),
                            ..
                        } => match &types[*ok] {
                            ComponentDefinedType::Own(id) => Some(id),
                            _ => None,
                        },
                        _ => None,
                    },
                };
                let resource = match resource {
                    Some(id) => id,
                    None => bail!(
                        offset,
                        "function should return `(own $T)` or `(result (own $T))`"
                    ),
                };
                self.validate_resource_name(*resource, rname, offset)?;
            }

            // Methods must take `(param "self" (borrow $resource))` as the
            // first argument where `$resources` matches the name `resource` as
            // named in this context.
            ComponentNameKind::Method(name) => {
                let ty = func()?;
                if ty.params.len() == 0 {
                    bail!(offset, "function should have at least one argument");
                }
                let (pname, pty) = &ty.params[0];
                if pname.as_str() != "self" {
                    bail!(
                        offset,
                        "function should have a first argument called `self`",
                    );
                }
                let id = match pty {
                    ComponentValType::Primitive(_) => None,
                    ComponentValType::Type(ty) => match &types[*ty] {
                        ComponentDefinedType::Borrow(id) => Some(id),
                        _ => None,
                    },
                };
                let id = match id {
                    Some(id) => id,
                    None => bail!(
                        offset,
                        "function should take a first argument of `(borrow $T)`"
                    ),
                };
                self.validate_resource_name(*id, name.resource(), offset)?;
            }

            // Static methods don't have much validation beyond that they must
            // be a function and the resource name referred to must already be
            // in this context.
            ComponentNameKind::Static(name) => {
                func()?;
                if !self.all_resource_names.contains(name.resource().as_str()) {
                    bail!(offset, "static resource name is not known in this context");
                }
            }
        }

        Ok(())
    }

    fn validate_resource_name(
        &self,
        id: AliasableResourceId,
        name: &KebabStr,
        offset: usize,
    ) -> Result<()> {
        let expected_name_idx = match self.resource_name_map.get(&id) {
            Some(idx) => *idx,
            None => {
                bail!(
                    offset,
                    "resource used in function does not have a name in this context"
                )
            }
        };
        let expected_name = &self.all_resource_names[expected_name_idx];
        if name.as_str() != expected_name {
            bail!(
                offset,
                "function does not match expected \
                         resource name `{expected_name}`"
            );
        }
        Ok(())
    }
}

use self::append_only::*;

mod append_only {
    use crate::prelude::IndexMap;
    use core::hash::Hash;
    use core::ops::Deref;

    pub struct IndexMapAppendOnly<K, V>(IndexMap<K, V>);

    impl<K, V> IndexMapAppendOnly<K, V>
    where
        K: Hash + Eq + Ord + PartialEq + Clone,
    {
        pub fn insert(&mut self, key: K, value: V) {
            let prev = self.0.insert(key, value);
            assert!(prev.is_none());
        }
    }

    impl<K, V> Deref for IndexMapAppendOnly<K, V> {
        type Target = IndexMap<K, V>;
        fn deref(&self) -> &IndexMap<K, V> {
            &self.0
        }
    }

    impl<K, V> Default for IndexMapAppendOnly<K, V> {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    impl<K, V> IntoIterator for IndexMapAppendOnly<K, V> {
        type IntoIter = <IndexMap<K, V> as IntoIterator>::IntoIter;
        type Item = <IndexMap<K, V> as IntoIterator>::Item;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
}
