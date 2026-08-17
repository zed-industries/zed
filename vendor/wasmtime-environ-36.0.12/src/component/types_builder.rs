use crate::component::*;
use crate::prelude::*;
use crate::{
    EngineOrModuleTypeIndex, EntityType, ModuleInternedTypeIndex, ModuleTypes, ModuleTypesBuilder,
    PrimaryMap, TypeConvert, WasmHeapType, WasmValType,
};
use anyhow::{Result, bail};
use cranelift_entity::EntityRef;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use wasmparser::component_types::{
    ComponentAnyTypeId, ComponentCoreModuleTypeId, ComponentDefinedType, ComponentDefinedTypeId,
    ComponentEntityType, ComponentFuncTypeId, ComponentInstanceTypeId, ComponentTypeId,
    ComponentValType, RecordType, ResourceId, TupleType, VariantType,
};
use wasmparser::names::KebabString;
use wasmparser::types::TypesRef;
use wasmparser::{PrimitiveValType, Validator};
use wasmtime_component_util::FlagsSize;

mod resources;
pub use resources::ResourcesBuilder;

/// Maximum nesting depth of a type allowed in Wasmtime.
///
/// This constant isn't chosen via any scientific means and its main purpose is
/// to enable most of Wasmtime to handle types via recursion without worrying
/// about stack overflow.
///
/// Some more information about this can be found in #4814
const MAX_TYPE_DEPTH: u32 = 100;

/// Structure used to build a [`ComponentTypes`] during translation.
///
/// This contains tables to intern any component types found as well as
/// managing building up core wasm [`ModuleTypes`] as well.
pub struct ComponentTypesBuilder {
    functions: HashMap<TypeFunc, TypeFuncIndex>,
    lists: HashMap<TypeList, TypeListIndex>,
    records: HashMap<TypeRecord, TypeRecordIndex>,
    variants: HashMap<TypeVariant, TypeVariantIndex>,
    tuples: HashMap<TypeTuple, TypeTupleIndex>,
    enums: HashMap<TypeEnum, TypeEnumIndex>,
    flags: HashMap<TypeFlags, TypeFlagsIndex>,
    options: HashMap<TypeOption, TypeOptionIndex>,
    results: HashMap<TypeResult, TypeResultIndex>,
    futures: HashMap<TypeFuture, TypeFutureIndex>,
    streams: HashMap<TypeStream, TypeStreamIndex>,
    future_tables: HashMap<TypeFutureTable, TypeFutureTableIndex>,
    stream_tables: HashMap<TypeStreamTable, TypeStreamTableIndex>,
    error_context_tables: HashMap<TypeErrorContextTable, TypeComponentLocalErrorContextTableIndex>,

    component_types: ComponentTypes,
    module_types: ModuleTypesBuilder,

    // Cache of what the "flat" representation of all types are which is only
    // used at compile-time and not used at runtime, hence the location here
    // as opposed to `ComponentTypes`.
    type_info: TypeInformationCache,

    resources: ResourcesBuilder,
}

impl<T> Index<T> for ComponentTypesBuilder
where
    ModuleTypes: Index<T>,
{
    type Output = <ModuleTypes as Index<T>>::Output;
    fn index(&self, idx: T) -> &Self::Output {
        self.module_types.index(idx)
    }
}

macro_rules! intern_and_fill_flat_types {
    ($me:ident, $name:ident, $val:ident) => {{
        if let Some(idx) = $me.$name.get(&$val) {
            *idx
        } else {
            let idx = $me.component_types.$name.push($val.clone());
            let mut info = TypeInformation::new();
            info.$name($me, &$val);
            let idx2 = $me.type_info.$name.push(info);
            assert_eq!(idx, idx2);
            $me.$name.insert($val, idx);
            idx
        }
    }};
}

impl ComponentTypesBuilder {
    /// Construct a new `ComponentTypesBuilder` for use with the given validator.
    pub fn new(validator: &Validator) -> Self {
        Self {
            module_types: ModuleTypesBuilder::new(validator),

            functions: HashMap::default(),
            lists: HashMap::default(),
            records: HashMap::default(),
            variants: HashMap::default(),
            tuples: HashMap::default(),
            enums: HashMap::default(),
            flags: HashMap::default(),
            options: HashMap::default(),
            results: HashMap::default(),
            futures: HashMap::default(),
            streams: HashMap::default(),
            future_tables: HashMap::default(),
            stream_tables: HashMap::default(),
            error_context_tables: HashMap::default(),
            component_types: ComponentTypes::default(),
            type_info: TypeInformationCache::default(),
            resources: ResourcesBuilder::default(),
        }
    }

    fn export_type_def(
        &mut self,
        export_items: &PrimaryMap<ExportIndex, Export>,
        idx: ExportIndex,
    ) -> TypeDef {
        match &export_items[idx] {
            Export::LiftedFunction { ty, .. } => TypeDef::ComponentFunc(*ty),
            Export::ModuleStatic { ty, .. } | Export::ModuleImport { ty, .. } => {
                TypeDef::Module(*ty)
            }
            Export::Instance { ty, .. } => TypeDef::ComponentInstance(*ty),
            Export::Type(ty) => *ty,
        }
    }

    /// Finishes this list of component types and returns the finished
    /// structure and the [`TypeComponentIndex`] corresponding to top-level component
    /// with `imports` and `exports` specified.
    pub fn finish(mut self, component: &Component) -> (ComponentTypes, TypeComponentIndex) {
        let mut component_ty = TypeComponent::default();
        for (_, (name, ty)) in component.import_types.iter() {
            component_ty.imports.insert(name.clone(), *ty);
        }
        for (name, ty) in component.exports.raw_iter() {
            component_ty.exports.insert(
                name.clone(),
                self.export_type_def(&component.export_items, *ty),
            );
        }
        let ty = self.component_types.components.push(component_ty);

        self.component_types.module_types = Some(self.module_types.finish());
        (self.component_types, ty)
    }

    /// Smaller helper method to find a `ModuleInternedTypeIndex` which
    /// corresponds to the `resource.drop` intrinsic in components, namely a
    /// core wasm function type which takes one `i32` argument and has no
    /// results.
    ///
    /// This is a bit of a hack right now as ideally this find operation
    /// wouldn't be needed and instead the `ModuleInternedTypeIndex` itself
    /// would be threaded through appropriately, but that's left for a future
    /// refactoring. Try not to lean too hard on this method though.
    pub fn find_resource_drop_signature(&self) -> Option<ModuleInternedTypeIndex> {
        self.module_types
            .wasm_types()
            .find(|(_, ty)| {
                ty.as_func().map_or(false, |sig| {
                    sig.params().len() == 1
                        && sig.returns().len() == 0
                        && sig.params()[0] == WasmValType::I32
                })
            })
            .map(|(i, _)| i)
    }

    /// Returns the underlying builder used to build up core wasm module types.
    ///
    /// Note that this is shared across all modules found within a component to
    /// improve the wins from deduplicating function signatures.
    pub fn module_types_builder(&self) -> &ModuleTypesBuilder {
        &self.module_types
    }

    /// Same as `module_types_builder`, but `mut`.
    pub fn module_types_builder_mut(&mut self) -> &mut ModuleTypesBuilder {
        &mut self.module_types
    }

    /// Returns the internal reference to the in-progress `&ComponentTypes`.
    pub(super) fn component_types(&self) -> &ComponentTypes {
        &self.component_types
    }

    /// Returns the number of resource tables allocated so far, or the maximum
    /// `TypeResourceTableIndex`.
    pub fn num_resource_tables(&self) -> usize {
        self.component_types.resource_tables.len()
    }

    /// Returns the number of future tables allocated so far, or the maximum
    /// `TypeFutureTableIndex`.
    pub fn num_future_tables(&self) -> usize {
        self.component_types.future_tables.len()
    }

    /// Returns the number of stream tables allocated so far, or the maximum
    /// `TypeStreamTableIndex`.
    pub fn num_stream_tables(&self) -> usize {
        self.component_types.stream_tables.len()
    }

    /// Returns the number of error-context tables allocated so far, or the maximum
    /// `TypeComponentLocalErrorContextTableIndex`.
    pub fn num_error_context_tables(&self) -> usize {
        self.component_types.error_context_tables.len()
    }

    /// Returns a mutable reference to the underlying `ResourcesBuilder`.
    pub fn resources_mut(&mut self) -> &mut ResourcesBuilder {
        &mut self.resources
    }

    /// Work around the borrow checker to borrow two sub-fields simultaneously
    /// externally.
    pub fn resources_mut_and_types(&mut self) -> (&mut ResourcesBuilder, &ComponentTypes) {
        (&mut self.resources, &self.component_types)
    }

    /// Converts a wasmparser `ComponentFuncType` into Wasmtime's type
    /// representation.
    pub fn convert_component_func_type(
        &mut self,
        types: TypesRef<'_>,
        id: ComponentFuncTypeId,
    ) -> Result<TypeFuncIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let ty = &types[id];
        let param_names = ty.params.iter().map(|(name, _)| name.to_string()).collect();
        let params = ty
            .params
            .iter()
            .map(|(_name, ty)| self.valtype(types, ty))
            .collect::<Result<_>>()?;
        let results = ty
            .result
            .iter()
            .map(|ty| self.valtype(types, ty))
            .collect::<Result<_>>()?;
        let params = self.new_tuple_type(params);
        let results = self.new_tuple_type(results);
        let ty = TypeFunc {
            param_names,
            params,
            results,
        };
        Ok(self.add_func_type(ty))
    }

    /// Converts a wasmparser `ComponentEntityType` into Wasmtime's type
    /// representation.
    pub fn convert_component_entity_type(
        &mut self,
        types: TypesRef<'_>,
        ty: ComponentEntityType,
    ) -> Result<TypeDef> {
        assert_eq!(types.id(), self.module_types.validator_id());
        Ok(match ty {
            ComponentEntityType::Module(id) => TypeDef::Module(self.convert_module(types, id)?),
            ComponentEntityType::Component(id) => {
                TypeDef::Component(self.convert_component(types, id)?)
            }
            ComponentEntityType::Instance(id) => {
                TypeDef::ComponentInstance(self.convert_instance(types, id)?)
            }
            ComponentEntityType::Func(id) => {
                TypeDef::ComponentFunc(self.convert_component_func_type(types, id)?)
            }
            ComponentEntityType::Type { created, .. } => match created {
                ComponentAnyTypeId::Defined(id) => {
                    TypeDef::Interface(self.defined_type(types, id)?)
                }
                ComponentAnyTypeId::Resource(id) => {
                    TypeDef::Resource(self.resource_id(id.resource()))
                }
                _ => bail!("unsupported type export"),
            },
            ComponentEntityType::Value(_) => bail!("values not supported"),
        })
    }

    /// Converts a wasmparser `Type` into Wasmtime's type representation.
    pub fn convert_type(&mut self, types: TypesRef<'_>, id: ComponentAnyTypeId) -> Result<TypeDef> {
        assert_eq!(types.id(), self.module_types.validator_id());
        Ok(match id {
            ComponentAnyTypeId::Defined(id) => TypeDef::Interface(self.defined_type(types, id)?),
            ComponentAnyTypeId::Component(id) => {
                TypeDef::Component(self.convert_component(types, id)?)
            }
            ComponentAnyTypeId::Instance(id) => {
                TypeDef::ComponentInstance(self.convert_instance(types, id)?)
            }
            ComponentAnyTypeId::Func(id) => {
                TypeDef::ComponentFunc(self.convert_component_func_type(types, id)?)
            }
            ComponentAnyTypeId::Resource(id) => TypeDef::Resource(self.resource_id(id.resource())),
        })
    }

    fn convert_component(
        &mut self,
        types: TypesRef<'_>,
        id: ComponentTypeId,
    ) -> Result<TypeComponentIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let ty = &types[id];
        let mut result = TypeComponent::default();
        for (name, ty) in ty.imports.iter() {
            result.imports.insert(
                name.clone(),
                self.convert_component_entity_type(types, *ty)?,
            );
        }
        for (name, ty) in ty.exports.iter() {
            result.exports.insert(
                name.clone(),
                self.convert_component_entity_type(types, *ty)?,
            );
        }
        Ok(self.component_types.components.push(result))
    }

    pub(crate) fn convert_instance(
        &mut self,
        types: TypesRef<'_>,
        id: ComponentInstanceTypeId,
    ) -> Result<TypeComponentInstanceIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let ty = &types[id];
        let mut result = TypeComponentInstance::default();
        for (name, ty) in ty.exports.iter() {
            result.exports.insert(
                name.clone(),
                self.convert_component_entity_type(types, *ty)?,
            );
        }
        Ok(self.component_types.component_instances.push(result))
    }

    pub(crate) fn convert_module(
        &mut self,
        types: TypesRef<'_>,
        id: ComponentCoreModuleTypeId,
    ) -> Result<TypeModuleIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let ty = &types[id];
        let mut result = TypeModule::default();
        for ((module, field), ty) in ty.imports.iter() {
            result.imports.insert(
                (module.clone(), field.clone()),
                self.entity_type(types, ty)?,
            );
        }
        for (name, ty) in ty.exports.iter() {
            result
                .exports
                .insert(name.clone(), self.entity_type(types, ty)?);
        }
        Ok(self.component_types.modules.push(result))
    }

    fn entity_type(
        &mut self,
        types: TypesRef<'_>,
        ty: &wasmparser::types::EntityType,
    ) -> Result<EntityType> {
        use wasmparser::types::EntityType::*;

        assert_eq!(types.id(), self.module_types.validator_id());
        Ok(match ty {
            Func(id) => EntityType::Function({
                self.module_types_builder_mut()
                    .intern_type(types, *id)?
                    .into()
            }),
            Table(ty) => EntityType::Table(self.convert_table_type(ty)?),
            Memory(ty) => EntityType::Memory((*ty).into()),
            Global(ty) => EntityType::Global(self.convert_global_type(ty)?),
            Tag(_) => bail!("exceptions proposal not implemented"),
        })
    }

    /// Convert a wasmparser `ComponentDefinedTypeId` into Wasmtime's type representation.
    pub fn defined_type(
        &mut self,
        types: TypesRef<'_>,
        id: ComponentDefinedTypeId,
    ) -> Result<InterfaceType> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let ret = match &types[id] {
            ComponentDefinedType::Primitive(ty) => self.primitive_type(ty)?,
            ComponentDefinedType::Record(e) => InterfaceType::Record(self.record_type(types, e)?),
            ComponentDefinedType::Variant(e) => {
                InterfaceType::Variant(self.variant_type(types, e)?)
            }
            ComponentDefinedType::List(e) => InterfaceType::List(self.list_type(types, e)?),
            ComponentDefinedType::Tuple(e) => InterfaceType::Tuple(self.tuple_type(types, e)?),
            ComponentDefinedType::Flags(e) => InterfaceType::Flags(self.flags_type(e)),
            ComponentDefinedType::Enum(e) => InterfaceType::Enum(self.enum_type(e)),
            ComponentDefinedType::Option(e) => InterfaceType::Option(self.option_type(types, e)?),
            ComponentDefinedType::Result { ok, err } => {
                InterfaceType::Result(self.result_type(types, ok, err)?)
            }
            ComponentDefinedType::Own(r) => InterfaceType::Own(self.resource_id(r.resource())),
            ComponentDefinedType::Borrow(r) => {
                InterfaceType::Borrow(self.resource_id(r.resource()))
            }
            ComponentDefinedType::Future(ty) => {
                InterfaceType::Future(self.future_table_type(types, ty)?)
            }
            ComponentDefinedType::Stream(ty) => {
                InterfaceType::Stream(self.stream_table_type(types, ty)?)
            }
            ComponentDefinedType::FixedSizeList(..) => {
                bail!("support not implemented for fixed-size-lists");
            }
        };
        let info = self.type_information(&ret);
        if info.depth > MAX_TYPE_DEPTH {
            bail!("type nesting is too deep");
        }
        Ok(ret)
    }

    /// Retrieve Wasmtime's type representation of the `error-context` type.
    pub fn error_context_type(&mut self) -> Result<TypeComponentLocalErrorContextTableIndex> {
        self.error_context_table_type()
    }

    pub(crate) fn valtype(
        &mut self,
        types: TypesRef<'_>,
        ty: &ComponentValType,
    ) -> Result<InterfaceType> {
        assert_eq!(types.id(), self.module_types.validator_id());
        match ty {
            ComponentValType::Primitive(p) => self.primitive_type(p),
            ComponentValType::Type(id) => self.defined_type(types, *id),
        }
    }

    fn primitive_type(&mut self, ty: &PrimitiveValType) -> Result<InterfaceType> {
        match ty {
            wasmparser::PrimitiveValType::Bool => Ok(InterfaceType::Bool),
            wasmparser::PrimitiveValType::S8 => Ok(InterfaceType::S8),
            wasmparser::PrimitiveValType::U8 => Ok(InterfaceType::U8),
            wasmparser::PrimitiveValType::S16 => Ok(InterfaceType::S16),
            wasmparser::PrimitiveValType::U16 => Ok(InterfaceType::U16),
            wasmparser::PrimitiveValType::S32 => Ok(InterfaceType::S32),
            wasmparser::PrimitiveValType::U32 => Ok(InterfaceType::U32),
            wasmparser::PrimitiveValType::S64 => Ok(InterfaceType::S64),
            wasmparser::PrimitiveValType::U64 => Ok(InterfaceType::U64),
            wasmparser::PrimitiveValType::F32 => Ok(InterfaceType::Float32),
            wasmparser::PrimitiveValType::F64 => Ok(InterfaceType::Float64),
            wasmparser::PrimitiveValType::Char => Ok(InterfaceType::Char),
            wasmparser::PrimitiveValType::String => Ok(InterfaceType::String),
            wasmparser::PrimitiveValType::ErrorContext => Ok(InterfaceType::ErrorContext(
                self.error_context_table_type()?,
            )),
        }
    }

    fn record_type(&mut self, types: TypesRef<'_>, ty: &RecordType) -> Result<TypeRecordIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let fields = ty
            .fields
            .iter()
            .map(|(name, ty)| {
                Ok(RecordField {
                    name: name.to_string(),
                    ty: self.valtype(types, ty)?,
                })
            })
            .collect::<Result<Box<[_]>>>()?;
        let abi = CanonicalAbiInfo::record(
            fields
                .iter()
                .map(|field| self.component_types.canonical_abi(&field.ty)),
        );
        Ok(self.add_record_type(TypeRecord { fields, abi }))
    }

    fn variant_type(&mut self, types: TypesRef<'_>, ty: &VariantType) -> Result<TypeVariantIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let cases = ty
            .cases
            .iter()
            .map(|(name, case)| {
                // FIXME: need to implement `refines`, not sure what that
                // is at this time.
                if case.refines.is_some() {
                    bail!("refines is not supported at this time");
                }
                Ok((
                    name.to_string(),
                    match &case.ty.as_ref() {
                        Some(ty) => Some(self.valtype(types, ty)?),
                        None => None,
                    },
                ))
            })
            .collect::<Result<IndexMap<_, _>>>()?;
        let (info, abi) = VariantInfo::new(
            cases
                .iter()
                .map(|(_, c)| c.as_ref().map(|ty| self.component_types.canonical_abi(ty))),
        );
        Ok(self.add_variant_type(TypeVariant { cases, abi, info }))
    }

    fn tuple_type(&mut self, types: TypesRef<'_>, ty: &TupleType) -> Result<TypeTupleIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let types = ty
            .types
            .iter()
            .map(|ty| self.valtype(types, ty))
            .collect::<Result<Box<[_]>>>()?;
        Ok(self.new_tuple_type(types))
    }

    pub(crate) fn new_tuple_type(&mut self, types: Box<[InterfaceType]>) -> TypeTupleIndex {
        let abi = CanonicalAbiInfo::record(
            types
                .iter()
                .map(|ty| self.component_types.canonical_abi(ty)),
        );
        self.add_tuple_type(TypeTuple { types, abi })
    }

    fn flags_type(&mut self, flags: &IndexSet<KebabString>) -> TypeFlagsIndex {
        let flags = TypeFlags {
            names: flags.iter().map(|s| s.to_string()).collect(),
            abi: CanonicalAbiInfo::flags(flags.len()),
        };
        self.add_flags_type(flags)
    }

    fn enum_type(&mut self, variants: &IndexSet<KebabString>) -> TypeEnumIndex {
        let names = variants
            .iter()
            .map(|s| s.to_string())
            .collect::<IndexSet<_>>();
        let (info, abi) = VariantInfo::new(names.iter().map(|_| None));
        self.add_enum_type(TypeEnum { names, abi, info })
    }

    fn option_type(
        &mut self,
        types: TypesRef<'_>,
        ty: &ComponentValType,
    ) -> Result<TypeOptionIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let ty = self.valtype(types, ty)?;
        let (info, abi) = VariantInfo::new([None, Some(self.component_types.canonical_abi(&ty))]);
        Ok(self.add_option_type(TypeOption { ty, abi, info }))
    }

    fn result_type(
        &mut self,
        types: TypesRef<'_>,
        ok: &Option<ComponentValType>,
        err: &Option<ComponentValType>,
    ) -> Result<TypeResultIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let ok = match ok {
            Some(ty) => Some(self.valtype(types, ty)?),
            None => None,
        };
        let err = match err {
            Some(ty) => Some(self.valtype(types, ty)?),
            None => None,
        };
        let (info, abi) = VariantInfo::new([
            ok.as_ref().map(|t| self.component_types.canonical_abi(t)),
            err.as_ref().map(|t| self.component_types.canonical_abi(t)),
        ]);
        Ok(self.add_result_type(TypeResult { ok, err, abi, info }))
    }

    fn future_table_type(
        &mut self,
        types: TypesRef<'_>,
        ty: &Option<ComponentValType>,
    ) -> Result<TypeFutureTableIndex> {
        let payload = ty.as_ref().map(|ty| self.valtype(types, ty)).transpose()?;
        let ty = self.add_future_type(TypeFuture { payload });
        Ok(self.add_future_table_type(TypeFutureTable {
            ty,
            instance: self.resources.get_current_instance().unwrap(),
        }))
    }

    fn stream_table_type(
        &mut self,
        types: TypesRef<'_>,
        ty: &Option<ComponentValType>,
    ) -> Result<TypeStreamTableIndex> {
        let payload = ty.as_ref().map(|ty| self.valtype(types, ty)).transpose()?;
        let ty = self.add_stream_type(TypeStream { payload });
        Ok(self.add_stream_table_type(TypeStreamTable {
            ty,
            instance: self.resources.get_current_instance().unwrap(),
        }))
    }

    /// Retrieve Wasmtime's type representation of the `error-context` type from
    /// the point of view of the current component instance.
    pub fn error_context_table_type(&mut self) -> Result<TypeComponentLocalErrorContextTableIndex> {
        Ok(self.add_error_context_table_type(TypeErrorContextTable {
            instance: self.resources.get_current_instance().unwrap(),
        }))
    }

    fn list_type(&mut self, types: TypesRef<'_>, ty: &ComponentValType) -> Result<TypeListIndex> {
        assert_eq!(types.id(), self.module_types.validator_id());
        let element = self.valtype(types, ty)?;
        Ok(self.add_list_type(TypeList { element }))
    }

    /// Converts a wasmparser `id`, which must point to a resource, to its
    /// corresponding `TypeResourceTableIndex`.
    pub fn resource_id(&mut self, id: ResourceId) -> TypeResourceTableIndex {
        self.resources.convert(id, &mut self.component_types)
    }

    /// Interns a new function type within this type information.
    pub fn add_func_type(&mut self, ty: TypeFunc) -> TypeFuncIndex {
        intern(&mut self.functions, &mut self.component_types.functions, ty)
    }

    /// Interns a new record type within this type information.
    pub fn add_record_type(&mut self, ty: TypeRecord) -> TypeRecordIndex {
        intern_and_fill_flat_types!(self, records, ty)
    }

    /// Interns a new flags type within this type information.
    pub fn add_flags_type(&mut self, ty: TypeFlags) -> TypeFlagsIndex {
        intern_and_fill_flat_types!(self, flags, ty)
    }

    /// Interns a new tuple type within this type information.
    pub fn add_tuple_type(&mut self, ty: TypeTuple) -> TypeTupleIndex {
        intern_and_fill_flat_types!(self, tuples, ty)
    }

    /// Interns a new variant type within this type information.
    pub fn add_variant_type(&mut self, ty: TypeVariant) -> TypeVariantIndex {
        intern_and_fill_flat_types!(self, variants, ty)
    }

    /// Interns a new enum type within this type information.
    pub fn add_enum_type(&mut self, ty: TypeEnum) -> TypeEnumIndex {
        intern_and_fill_flat_types!(self, enums, ty)
    }

    /// Interns a new option type within this type information.
    pub fn add_option_type(&mut self, ty: TypeOption) -> TypeOptionIndex {
        intern_and_fill_flat_types!(self, options, ty)
    }

    /// Interns a new result type within this type information.
    pub fn add_result_type(&mut self, ty: TypeResult) -> TypeResultIndex {
        intern_and_fill_flat_types!(self, results, ty)
    }

    /// Interns a new list type within this type information.
    pub fn add_list_type(&mut self, ty: TypeList) -> TypeListIndex {
        intern_and_fill_flat_types!(self, lists, ty)
    }

    /// Interns a new future type within this type information.
    pub fn add_future_type(&mut self, ty: TypeFuture) -> TypeFutureIndex {
        intern(&mut self.futures, &mut self.component_types.futures, ty)
    }

    /// Interns a new future table type within this type information.
    pub fn add_future_table_type(&mut self, ty: TypeFutureTable) -> TypeFutureTableIndex {
        intern(
            &mut self.future_tables,
            &mut self.component_types.future_tables,
            ty,
        )
    }

    /// Interns a new stream type within this type information.
    pub fn add_stream_type(&mut self, ty: TypeStream) -> TypeStreamIndex {
        intern(&mut self.streams, &mut self.component_types.streams, ty)
    }

    /// Interns a new stream table type within this type information.
    pub fn add_stream_table_type(&mut self, ty: TypeStreamTable) -> TypeStreamTableIndex {
        intern(
            &mut self.stream_tables,
            &mut self.component_types.stream_tables,
            ty,
        )
    }

    /// Interns a new error context table type within this type information.
    pub fn add_error_context_table_type(
        &mut self,
        ty: TypeErrorContextTable,
    ) -> TypeComponentLocalErrorContextTableIndex {
        intern(
            &mut self.error_context_tables,
            &mut self.component_types.error_context_tables,
            ty,
        )
    }

    /// Returns the canonical ABI information about the specified type.
    pub fn canonical_abi(&self, ty: &InterfaceType) -> &CanonicalAbiInfo {
        self.component_types.canonical_abi(ty)
    }

    /// Returns the "flat types" for the given interface type used in the
    /// canonical ABI.
    ///
    /// Returns `None` if the type is too large to be represented via flat types
    /// in the canonical abi.
    pub fn flat_types(&self, ty: &InterfaceType) -> Option<FlatTypes<'_>> {
        self.type_information(ty).flat.as_flat_types()
    }

    /// Returns whether the type specified contains any borrowed resources
    /// within it.
    pub fn ty_contains_borrow_resource(&self, ty: &InterfaceType) -> bool {
        self.type_information(ty).has_borrow
    }

    fn type_information(&self, ty: &InterfaceType) -> &TypeInformation {
        match ty {
            InterfaceType::U8
            | InterfaceType::S8
            | InterfaceType::Bool
            | InterfaceType::U16
            | InterfaceType::S16
            | InterfaceType::U32
            | InterfaceType::S32
            | InterfaceType::Char
            | InterfaceType::Own(_)
            | InterfaceType::Future(_)
            | InterfaceType::Stream(_)
            | InterfaceType::ErrorContext(_) => {
                static INFO: TypeInformation = TypeInformation::primitive(FlatType::I32);
                &INFO
            }
            InterfaceType::Borrow(_) => {
                static INFO: TypeInformation = {
                    let mut info = TypeInformation::primitive(FlatType::I32);
                    info.has_borrow = true;
                    info
                };
                &INFO
            }
            InterfaceType::U64 | InterfaceType::S64 => {
                static INFO: TypeInformation = TypeInformation::primitive(FlatType::I64);
                &INFO
            }
            InterfaceType::Float32 => {
                static INFO: TypeInformation = TypeInformation::primitive(FlatType::F32);
                &INFO
            }
            InterfaceType::Float64 => {
                static INFO: TypeInformation = TypeInformation::primitive(FlatType::F64);
                &INFO
            }
            InterfaceType::String => {
                static INFO: TypeInformation = TypeInformation::string();
                &INFO
            }

            InterfaceType::List(i) => &self.type_info.lists[*i],
            InterfaceType::Record(i) => &self.type_info.records[*i],
            InterfaceType::Variant(i) => &self.type_info.variants[*i],
            InterfaceType::Tuple(i) => &self.type_info.tuples[*i],
            InterfaceType::Flags(i) => &self.type_info.flags[*i],
            InterfaceType::Enum(i) => &self.type_info.enums[*i],
            InterfaceType::Option(i) => &self.type_info.options[*i],
            InterfaceType::Result(i) => &self.type_info.results[*i],
        }
    }
}

impl TypeConvert for ComponentTypesBuilder {
    fn lookup_heap_type(&self, _index: wasmparser::UnpackedIndex) -> WasmHeapType {
        panic!("heap types are not supported yet")
    }

    fn lookup_type_index(&self, _index: wasmparser::UnpackedIndex) -> EngineOrModuleTypeIndex {
        panic!("typed references are not supported yet")
    }
}

fn intern<T, U>(map: &mut HashMap<T, U>, list: &mut PrimaryMap<U, T>, item: T) -> U
where
    T: Hash + Clone + Eq,
    U: Copy + EntityRef,
{
    if let Some(idx) = map.get(&item) {
        return *idx;
    }
    let idx = list.push(item.clone());
    map.insert(item, idx);
    return idx;
}

struct FlatTypesStorage {
    // This could be represented as `Vec<FlatType>` but on 64-bit architectures
    // that's 24 bytes. Otherwise `FlatType` is 1 byte large and
    // `MAX_FLAT_TYPES` is 16, so it should ideally be more space-efficient to
    // use a flat array instead of a heap-based vector.
    memory32: [FlatType; MAX_FLAT_TYPES],
    memory64: [FlatType; MAX_FLAT_TYPES],

    // Tracks the number of flat types pushed into this storage. If this is
    // `MAX_FLAT_TYPES + 1` then this storage represents an un-reprsentable
    // type in flat types.
    len: u8,
}

impl FlatTypesStorage {
    const fn new() -> FlatTypesStorage {
        FlatTypesStorage {
            memory32: [FlatType::I32; MAX_FLAT_TYPES],
            memory64: [FlatType::I32; MAX_FLAT_TYPES],
            len: 0,
        }
    }

    fn as_flat_types(&self) -> Option<FlatTypes<'_>> {
        let len = usize::from(self.len);
        if len > MAX_FLAT_TYPES {
            assert_eq!(len, MAX_FLAT_TYPES + 1);
            None
        } else {
            Some(FlatTypes {
                memory32: &self.memory32[..len],
                memory64: &self.memory64[..len],
            })
        }
    }

    /// Pushes a new flat type into this list using `t32` for 32-bit memories
    /// and `t64` for 64-bit memories.
    ///
    /// Returns whether the type was actually pushed or whether this list of
    /// flat types just exceeded the maximum meaning that it is now
    /// unrepresentable with a flat list of types.
    fn push(&mut self, t32: FlatType, t64: FlatType) -> bool {
        let len = usize::from(self.len);
        if len < MAX_FLAT_TYPES {
            self.memory32[len] = t32;
            self.memory64[len] = t64;
            self.len += 1;
            true
        } else {
            // If this was the first one to go over then flag the length as
            // being incompatible with a flat representation.
            if len == MAX_FLAT_TYPES {
                self.len += 1;
            }
            false
        }
    }
}

impl FlatType {
    fn join(&mut self, other: FlatType) {
        if *self == other {
            return;
        }
        *self = match (*self, other) {
            (FlatType::I32, FlatType::F32) | (FlatType::F32, FlatType::I32) => FlatType::I32,
            _ => FlatType::I64,
        };
    }
}

#[derive(Default)]
struct TypeInformationCache {
    records: PrimaryMap<TypeRecordIndex, TypeInformation>,
    variants: PrimaryMap<TypeVariantIndex, TypeInformation>,
    tuples: PrimaryMap<TypeTupleIndex, TypeInformation>,
    enums: PrimaryMap<TypeEnumIndex, TypeInformation>,
    flags: PrimaryMap<TypeFlagsIndex, TypeInformation>,
    options: PrimaryMap<TypeOptionIndex, TypeInformation>,
    results: PrimaryMap<TypeResultIndex, TypeInformation>,
    lists: PrimaryMap<TypeListIndex, TypeInformation>,
}

struct TypeInformation {
    depth: u32,
    flat: FlatTypesStorage,
    has_borrow: bool,
}

impl TypeInformation {
    const fn new() -> TypeInformation {
        TypeInformation {
            depth: 0,
            flat: FlatTypesStorage::new(),
            has_borrow: false,
        }
    }

    const fn primitive(flat: FlatType) -> TypeInformation {
        let mut info = TypeInformation::new();
        info.depth = 1;
        info.flat.memory32[0] = flat;
        info.flat.memory64[0] = flat;
        info.flat.len = 1;
        info
    }

    const fn string() -> TypeInformation {
        let mut info = TypeInformation::new();
        info.depth = 1;
        info.flat.memory32[0] = FlatType::I32;
        info.flat.memory32[1] = FlatType::I32;
        info.flat.memory64[0] = FlatType::I64;
        info.flat.memory64[1] = FlatType::I64;
        info.flat.len = 2;
        info
    }

    /// Builds up all flat types internally using the specified representation
    /// for all of the component fields of the record.
    fn build_record<'a>(&mut self, types: impl Iterator<Item = &'a TypeInformation>) {
        self.depth = 1;
        for info in types {
            self.depth = self.depth.max(1 + info.depth);
            self.has_borrow = self.has_borrow || info.has_borrow;
            match info.flat.as_flat_types() {
                Some(types) => {
                    for (t32, t64) in types.memory32.iter().zip(types.memory64) {
                        if !self.flat.push(*t32, *t64) {
                            break;
                        }
                    }
                }
                None => {
                    self.flat.len = u8::try_from(MAX_FLAT_TYPES + 1).unwrap();
                }
            }
        }
    }

    /// Builds up the flat types used to represent a `variant` which notably
    /// handles "join"ing types together so each case is representable as a
    /// single flat list of types.
    ///
    /// The iterator item is:
    ///
    /// * `None` - no payload for this case
    /// * `Some(None)` - this case has a payload but can't be represented with
    ///   flat types
    /// * `Some(Some(types))` - this case has a payload and is represented with
    ///   the types specified in the flat representation.
    fn build_variant<'a, I>(&mut self, cases: I)
    where
        I: IntoIterator<Item = Option<&'a TypeInformation>>,
    {
        let cases = cases.into_iter();
        self.flat.push(FlatType::I32, FlatType::I32);
        self.depth = 1;

        for info in cases {
            let info = match info {
                Some(info) => info,
                // If this case doesn't have a payload then it doesn't change
                // the depth/flat representation
                None => continue,
            };
            self.depth = self.depth.max(1 + info.depth);
            self.has_borrow = self.has_borrow || info.has_borrow;

            // If this variant is already unrepresentable in a flat
            // representation then this can be skipped.
            if usize::from(self.flat.len) > MAX_FLAT_TYPES {
                continue;
            }

            let types = match info.flat.as_flat_types() {
                Some(types) => types,
                // If this case isn't representable with a flat list of types
                // then this variant also isn't representable.
                None => {
                    self.flat.len = u8::try_from(MAX_FLAT_TYPES + 1).unwrap();
                    continue;
                }
            };
            // If the case used all of the flat types then the discriminant
            // added for this variant means that this variant is no longer
            // representable.
            if types.memory32.len() >= MAX_FLAT_TYPES {
                self.flat.len = u8::try_from(MAX_FLAT_TYPES + 1).unwrap();
                continue;
            }
            let dst = self
                .flat
                .memory32
                .iter_mut()
                .zip(&mut self.flat.memory64)
                .skip(1);
            for (i, ((t32, t64), (dst32, dst64))) in types
                .memory32
                .iter()
                .zip(types.memory64)
                .zip(dst)
                .enumerate()
            {
                if i + 1 < usize::from(self.flat.len) {
                    // If this index hs already been set by some previous case
                    // then the types are joined together.
                    dst32.join(*t32);
                    dst64.join(*t64);
                } else {
                    // Otherwise if this is the first time that the
                    // representation has gotten this large then the destination
                    // is simply whatever the type is. The length is also
                    // increased here to indicate this.
                    self.flat.len += 1;
                    *dst32 = *t32;
                    *dst64 = *t64;
                }
            }
        }
    }

    fn records(&mut self, types: &ComponentTypesBuilder, ty: &TypeRecord) {
        self.build_record(ty.fields.iter().map(|f| types.type_information(&f.ty)));
    }

    fn tuples(&mut self, types: &ComponentTypesBuilder, ty: &TypeTuple) {
        self.build_record(ty.types.iter().map(|t| types.type_information(t)));
    }

    fn enums(&mut self, _types: &ComponentTypesBuilder, _ty: &TypeEnum) {
        self.depth = 1;
        self.flat.push(FlatType::I32, FlatType::I32);
    }

    fn flags(&mut self, _types: &ComponentTypesBuilder, ty: &TypeFlags) {
        self.depth = 1;
        match FlagsSize::from_count(ty.names.len()) {
            FlagsSize::Size0 => {}
            FlagsSize::Size1 | FlagsSize::Size2 => {
                self.flat.push(FlatType::I32, FlatType::I32);
            }
            FlagsSize::Size4Plus(n) => {
                for _ in 0..n {
                    self.flat.push(FlatType::I32, FlatType::I32);
                }
            }
        }
    }

    fn variants(&mut self, types: &ComponentTypesBuilder, ty: &TypeVariant) {
        self.build_variant(
            ty.cases
                .iter()
                .map(|(_, c)| c.as_ref().map(|ty| types.type_information(ty))),
        )
    }

    fn results(&mut self, types: &ComponentTypesBuilder, ty: &TypeResult) {
        self.build_variant([
            ty.ok.as_ref().map(|ty| types.type_information(ty)),
            ty.err.as_ref().map(|ty| types.type_information(ty)),
        ])
    }

    fn options(&mut self, types: &ComponentTypesBuilder, ty: &TypeOption) {
        self.build_variant([None, Some(types.type_information(&ty.ty))]);
    }

    fn lists(&mut self, types: &ComponentTypesBuilder, ty: &TypeList) {
        *self = TypeInformation::string();
        let info = types.type_information(&ty.element);
        self.depth += info.depth;
        self.has_borrow = info.has_borrow;
    }
}
