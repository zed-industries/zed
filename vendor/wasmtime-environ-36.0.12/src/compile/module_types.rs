use crate::{
    EngineOrModuleTypeIndex, EntityRef, ModuleInternedRecGroupIndex, ModuleInternedTypeIndex,
    ModuleTypes, TypeConvert, TypeIndex, WasmArrayType, WasmCompositeInnerType, WasmCompositeType,
    WasmFuncType, WasmHeapType, WasmResult, WasmStructType, WasmSubType, wasm_unsupported,
};
use std::{borrow::Cow, collections::HashMap, ops::Index};
use wasmparser::{UnpackedIndex, Validator, ValidatorId};

/// A type marking the start of a recursion group's definition.
///
/// This is initialized by `ModuleTypesBuilder::start_rec_group` and then
/// finished in `ModuleTypes::end_rec_group` after all of the types in the rec
/// group have been defined.
struct RecGroupStart {
    rec_group_index: ModuleInternedRecGroupIndex,
    start: ModuleInternedTypeIndex,
    end: ModuleInternedTypeIndex,
}

/// A builder for [`ModuleTypes`].
pub struct ModuleTypesBuilder {
    /// The ID of the validator that this builder is configured for. Using a
    /// different validator, or multiple validators, with this builder would
    /// result in silliness because our `wasmparser::types::*Id`s are only
    /// unique within the context of a particular validator. Getting this wrong
    /// could result in generating calls to functions of the wrong type, for
    /// example. So therefore we always assert that a builder instances is only
    /// ever paired with a particular validator context.
    validator_id: ValidatorId,

    /// The canonicalized and deduplicated set of types we are building.
    types: ModuleTypes,

    /// The set of trampoline-compatible function types we have already added to
    /// `self.types`. We do this additional level of deduping, on top of what
    /// `wasmparser` already does, so we can quickly and easily get the
    /// trampoline type for a given function type if we've already interned one.
    trampoline_types: HashMap<WasmFuncType, ModuleInternedTypeIndex>,

    /// A map from already-interned `wasmparser` types to their corresponding
    /// Wasmtime type.
    wasmparser_to_wasmtime: HashMap<wasmparser::types::CoreTypeId, ModuleInternedTypeIndex>,

    /// The set of recursion groups we have already seen and interned.
    already_seen: HashMap<wasmparser::types::RecGroupId, ModuleInternedRecGroupIndex>,

    /// If we are in the middle of defining a recursion group, this is the
    /// metadata about the recursion group we started defining.
    defining_rec_group: Option<RecGroupStart>,
}

impl ModuleTypesBuilder {
    /// Construct a new `ModuleTypesBuilder` using the given validator.
    pub fn new(validator: &Validator) -> Self {
        Self {
            validator_id: validator.id(),
            types: ModuleTypes::default(),
            trampoline_types: HashMap::default(),
            wasmparser_to_wasmtime: HashMap::default(),
            already_seen: HashMap::default(),
            defining_rec_group: None,
        }
    }

    /// Reserves space for `amt` more type signatures.
    pub fn reserve_wasm_signatures(&mut self, amt: usize) {
        self.types.reserve(amt);
        self.wasmparser_to_wasmtime.reserve(amt);
        self.already_seen.reserve(amt);
    }

    /// Get the id of the validator that this builder is configured for.
    pub fn validator_id(&self) -> ValidatorId {
        self.validator_id
    }

    /// Intern a recursion group and all of its types into this builder.
    ///
    /// If the recursion group has already been interned, then it is reused.
    ///
    /// Panics if given types from a different validator than the one that this
    /// builder is associated with.
    pub fn intern_rec_group(
        &mut self,
        validator_types: wasmparser::types::TypesRef<'_>,
        rec_group_id: wasmparser::types::RecGroupId,
    ) -> WasmResult<ModuleInternedRecGroupIndex> {
        assert_eq!(validator_types.id(), self.validator_id);

        if let Some(interned) = self.already_seen.get(&rec_group_id) {
            return Ok(*interned);
        }

        self.define_new_rec_group(validator_types, rec_group_id)
    }

    /// Define a new recursion group that we haven't already interned.
    fn define_new_rec_group(
        &mut self,
        validator_types: wasmparser::types::TypesRef<'_>,
        rec_group_id: wasmparser::types::RecGroupId,
    ) -> WasmResult<ModuleInternedRecGroupIndex> {
        assert_eq!(validator_types.id(), self.validator_id);

        self.start_rec_group(
            validator_types,
            validator_types.rec_group_elements(rec_group_id),
        );

        for id in validator_types.rec_group_elements(rec_group_id) {
            let ty = &validator_types[id];
            let wasm_ty = WasmparserTypeConverter::new(self, |_| {
                unreachable!("no need to lookup indexes; we already have core type IDs")
            })
            .with_rec_group(validator_types, rec_group_id)
            .convert_sub_type(ty)?;
            self.wasm_sub_type_in_rec_group(id, wasm_ty);
        }

        let rec_group_index = self.end_rec_group(rec_group_id);

        // Iterate over all the types we just defined and make sure that every
        // function type has an associated trampoline type. This needs to happen
        // *after* we finish defining the rec group because we may need to
        // intern new function types, which would conflict with the contiguous
        // range of type indices we pre-reserved for the rec group elements.
        for ty in self.rec_group_elements(rec_group_index) {
            if self.types[ty].is_func() {
                let trampoline = self.intern_trampoline_type(ty);
                self.types.set_trampoline_type(ty, trampoline);
            }
        }

        Ok(rec_group_index)
    }

    /// Get or create the trampoline function type for the given function
    /// type. Returns the interned type index of the trampoline function type.
    fn intern_trampoline_type(
        &mut self,
        for_func_ty: ModuleInternedTypeIndex,
    ) -> ModuleInternedTypeIndex {
        let sub_ty = &self.types[for_func_ty];
        let trampoline = sub_ty.unwrap_func().trampoline_type();

        if let Some(idx) = self.trampoline_types.get(&trampoline) {
            // We've already interned this trampoline type; reuse it.
            *idx
        } else {
            // We have not already interned this trampoline type.
            match trampoline {
                // The trampoline type is the same as the original function
                // type. We can reuse the definition and its index, but still
                // need to intern the type into our `trampoline_types` map so we
                // can reuse it in the future.
                Cow::Borrowed(f) => {
                    self.trampoline_types.insert(f.clone(), for_func_ty);
                    for_func_ty
                }
                // The trampoline type is different from the original function
                // type. Define the trampoline type and then intern it in
                // `trampoline_types` so we can reuse it in the future.
                Cow::Owned(f) => {
                    let idx = self.types.push(WasmSubType {
                        is_final: true,
                        supertype: None,
                        composite_type: WasmCompositeType {
                            inner: WasmCompositeInnerType::Func(f.clone()),
                            shared: sub_ty.composite_type.shared,
                        },
                    });

                    // The trampoline type is its own trampoline type.
                    self.types.set_trampoline_type(idx, idx);

                    let next = self.types.next_ty();
                    self.types.push_rec_group(idx..next);
                    self.trampoline_types.insert(f, idx);
                    idx
                }
            }
        }
    }

    /// Start defining a recursion group.
    fn start_rec_group(
        &mut self,
        validator_types: wasmparser::types::TypesRef<'_>,
        elems: impl ExactSizeIterator<Item = wasmparser::types::CoreTypeId>,
    ) {
        log::trace!("Starting rec group of length {}", elems.len());

        assert!(self.defining_rec_group.is_none());
        assert_eq!(validator_types.id(), self.validator_id);

        // Eagerly define the reverse map's entries for this rec group's types
        // so that we can use them when converting `wasmparser` types to our
        // types.
        let len = elems.len();
        for (i, wasmparser_id) in elems.enumerate() {
            let interned = ModuleInternedTypeIndex::new(self.types.len_types() + i);
            log::trace!(
                "Reserving {interned:?} for {wasmparser_id:?} = {:?}",
                validator_types[wasmparser_id]
            );

            let old_entry = self.wasmparser_to_wasmtime.insert(wasmparser_id, interned);
            debug_assert_eq!(
                old_entry, None,
                "should not have already inserted {wasmparser_id:?}"
            );
        }

        self.defining_rec_group = Some(RecGroupStart {
            rec_group_index: self.types.next_rec_group(),
            start: self.types.next_ty(),
            end: ModuleInternedTypeIndex::new(self.types.len_types() + len),
        });
    }

    /// Finish defining a recursion group.
    fn end_rec_group(
        &mut self,
        rec_group_id: wasmparser::types::RecGroupId,
    ) -> ModuleInternedRecGroupIndex {
        let RecGroupStart {
            rec_group_index,
            start,
            end,
        } = self
            .defining_rec_group
            .take()
            .expect("should be defining a rec group");

        log::trace!("Ending rec group {start:?}..{end:?}");

        debug_assert!(start.index() < self.types.len_types());
        debug_assert_eq!(
            end,
            self.types.next_ty(),
            "should have defined the number of types declared in `start_rec_group`"
        );

        let idx = self.types.push_rec_group(start..end);
        debug_assert_eq!(idx, rec_group_index);

        self.already_seen.insert(rec_group_id, rec_group_index);
        rec_group_index
    }

    /// Intern a type into this builder and get its Wasmtime index.
    ///
    /// This will intern not only the single given type, but the type's entire
    /// rec group. This helper method is provided as a convenience so that
    /// callers don't have to get the type's rec group, intern the rec group,
    /// and then look up the Wasmtime index for the original type themselves.
    pub fn intern_type(
        &mut self,
        validator_types: wasmparser::types::TypesRef<'_>,
        id: wasmparser::types::CoreTypeId,
    ) -> WasmResult<ModuleInternedTypeIndex> {
        assert!(self.defining_rec_group.is_none());
        assert_eq!(validator_types.id(), self.validator_id);

        let rec_group_id = validator_types.rec_group_id_of(id);
        debug_assert!(
            validator_types
                .rec_group_elements(rec_group_id)
                .any(|e| e == id)
        );

        let interned_rec_group = self.intern_rec_group(validator_types, rec_group_id)?;

        let interned_type = self.wasmparser_to_wasmtime[&id];
        debug_assert!(
            self.rec_group_elements(interned_rec_group)
                .any(|e| e == interned_type)
        );

        Ok(interned_type)
    }

    /// Define a new Wasm type while we are defining a rec group.
    fn wasm_sub_type_in_rec_group(
        &mut self,
        id: wasmparser::types::CoreTypeId,
        ty: WasmSubType,
    ) -> ModuleInternedTypeIndex {
        assert!(
            self.defining_rec_group.is_some(),
            "must be defining a rec group to define new types"
        );

        let module_interned_index = self.types.push(ty);
        debug_assert_eq!(
            self.wasmparser_to_wasmtime.get(&id),
            Some(&module_interned_index),
            "should have reserved the right module-interned index for this wasmparser type already"
        );

        module_interned_index
    }

    /// Returns the result [`ModuleTypes`] of this builder.
    pub fn finish(self) -> ModuleTypes {
        self.types
    }

    /// Get the elements within an already-defined rec group.
    pub fn rec_group_elements(
        &self,
        rec_group: ModuleInternedRecGroupIndex,
    ) -> impl ExactSizeIterator<Item = ModuleInternedTypeIndex> + use<> {
        self.types.rec_group_elements(rec_group)
    }

    /// Returns an iterator over all the unique wasm types defined thus far
    /// within this builder.
    pub fn wasm_types(&self) -> impl Iterator<Item = (ModuleInternedTypeIndex, &WasmSubType)> {
        self.types.wasm_types()
    }

    /// Get an iterator over all function types and their associated trampoline
    /// type.
    pub fn trampoline_types(
        &self,
    ) -> impl Iterator<Item = (ModuleInternedTypeIndex, ModuleInternedTypeIndex)> + '_ {
        self.types.trampoline_types()
    }

    /// Get the associated trampoline type for the given function type.
    pub fn trampoline_type(&self, ty: ModuleInternedTypeIndex) -> ModuleInternedTypeIndex {
        self.types.trampoline_type(ty)
    }

    /// Get and unwrap a [`WasmStructType`] for the given struct index.
    ///
    /// # Panics
    ///
    /// Panics if the unwrapped type is not a struct.
    ///
    /// # Errors
    ///
    /// For now, fails with an unsupported error if the type is shared.
    pub fn unwrap_struct(&self, ty: ModuleInternedTypeIndex) -> WasmResult<&WasmStructType> {
        let composite_type = &self.types[ty].composite_type;
        if composite_type.shared {
            return Err(wasm_unsupported!("shared structs are not yet implemented"));
        }
        match &composite_type.inner {
            WasmCompositeInnerType::Struct(s) => Ok(s),
            _ => unreachable!(),
        }
    }

    /// Get and unwrap a [`WasmArrayType`] for the given array index.
    ///
    /// # Panics
    ///
    /// Panics if the unwrapped type is not an array.
    ///
    /// # Errors
    ///
    /// For now, fails with an unsupported error if the type is shared.
    pub fn unwrap_array(&self, interned_ty: ModuleInternedTypeIndex) -> WasmResult<&WasmArrayType> {
        let composite_type = &self.types[interned_ty].composite_type;
        if composite_type.shared {
            return Err(wasm_unsupported!("shared arrays are not yet implemented"));
        }
        match &composite_type.inner {
            WasmCompositeInnerType::Array(a) => Ok(a),
            _ => unreachable!(),
        }
    }
}

// Forward the indexing impl to the internal `ModuleTypes`
impl<T> Index<T> for ModuleTypesBuilder
where
    ModuleTypes: Index<T>,
{
    type Output = <ModuleTypes as Index<T>>::Output;

    fn index(&self, sig: T) -> &Self::Output {
        &self.types[sig]
    }
}

/// A convert from `wasmparser` types to Wasmtime types.
pub struct WasmparserTypeConverter<'a, F> {
    types: &'a ModuleTypesBuilder,
    lookup_type_idx: F,
    rec_group_context: Option<(
        wasmparser::types::TypesRef<'a>,
        wasmparser::types::RecGroupId,
    )>,
}

impl<'a, F> WasmparserTypeConverter<'a, F> {
    /// Construct a new type converter from `wasmparser` types to Wasmtime types.
    pub fn new(types: &'a ModuleTypesBuilder, lookup_type_idx: F) -> Self {
        Self {
            types,
            lookup_type_idx,
            rec_group_context: None,
        }
    }

    /// Configure this converter to be within the context of defining the
    /// current rec group.
    pub fn with_rec_group(
        &mut self,
        wasmparser_types: wasmparser::types::TypesRef<'a>,
        rec_group: wasmparser::types::RecGroupId,
    ) -> &Self {
        self.rec_group_context = Some((wasmparser_types, rec_group));
        self
    }
}

impl<F> TypeConvert for WasmparserTypeConverter<'_, F>
where
    F: Fn(TypeIndex) -> ModuleInternedTypeIndex,
{
    fn lookup_heap_type(&self, index: UnpackedIndex) -> WasmHeapType {
        match index {
            UnpackedIndex::Id(id) => {
                let interned = self.types.wasmparser_to_wasmtime[&id];
                let index = EngineOrModuleTypeIndex::Module(interned);

                // If this is a forward reference to a type in this type's rec
                // group that we haven't converted yet, then we won't have an
                // entry in `wasm_types` yet. In this case, fallback to a
                // different means of determining whether this is a concrete
                // array vs struct vs func reference. In this case, we can use
                // the validator's type context.
                if let Some(ty) = self.types.types.get(interned) {
                    assert!(!ty.composite_type.shared);
                    match &ty.composite_type.inner {
                        WasmCompositeInnerType::Array(_) => WasmHeapType::ConcreteArray(index),
                        WasmCompositeInnerType::Func(_) => WasmHeapType::ConcreteFunc(index),
                        WasmCompositeInnerType::Struct(_) => WasmHeapType::ConcreteStruct(index),
                        WasmCompositeInnerType::Cont(_) => WasmHeapType::ConcreteCont(index),
                        WasmCompositeInnerType::Exn(_) => WasmHeapType::ConcreteExn(index),
                    }
                } else if let Some((wasmparser_types, _)) = self.rec_group_context.as_ref() {
                    let wasmparser_ty = &wasmparser_types[id].composite_type;
                    assert!(!wasmparser_ty.shared);
                    match &wasmparser_ty.inner {
                        wasmparser::CompositeInnerType::Array(_) => {
                            WasmHeapType::ConcreteArray(index)
                        }
                        wasmparser::CompositeInnerType::Func(_) => {
                            WasmHeapType::ConcreteFunc(index)
                        }
                        wasmparser::CompositeInnerType::Struct(_) => {
                            WasmHeapType::ConcreteStruct(index)
                        }
                        wasmparser::CompositeInnerType::Cont(_) => {
                            WasmHeapType::ConcreteCont(index)
                        }
                    }
                } else {
                    panic!("forward reference to type outside of rec group?")
                }
            }

            UnpackedIndex::Module(module_index) => {
                let module_index = TypeIndex::from_u32(module_index);
                let interned = (self.lookup_type_idx)(module_index);
                let index = EngineOrModuleTypeIndex::Module(interned);

                // See comment above about `wasm_types` maybe not having the
                // converted sub type yet. However in this case we don't have a
                // `wasmparser::types::CoreTypeId` on hand, so we have to
                // indirectly get one by looking it up inside the current rec
                // group.
                if let Some(ty) = self.types.types.get(interned) {
                    assert!(!ty.composite_type.shared);
                    match &ty.composite_type.inner {
                        WasmCompositeInnerType::Array(_) => WasmHeapType::ConcreteArray(index),
                        WasmCompositeInnerType::Func(_) => WasmHeapType::ConcreteFunc(index),
                        WasmCompositeInnerType::Struct(_) => WasmHeapType::ConcreteStruct(index),
                        WasmCompositeInnerType::Cont(_) => WasmHeapType::ConcreteCont(index),
                        WasmCompositeInnerType::Exn(_) => WasmHeapType::ConcreteExn(index),
                    }
                } else if let Some((parser_types, rec_group)) = self.rec_group_context.as_ref() {
                    let rec_group_index = interned.index() - self.types.types.len_types();
                    let id = parser_types
                        .rec_group_elements(*rec_group)
                        .nth(rec_group_index)
                        .unwrap();
                    let wasmparser_ty = &parser_types[id].composite_type;
                    assert!(!wasmparser_ty.shared);
                    match &wasmparser_ty.inner {
                        wasmparser::CompositeInnerType::Array(_) => {
                            WasmHeapType::ConcreteArray(index)
                        }
                        wasmparser::CompositeInnerType::Func(_) => {
                            WasmHeapType::ConcreteFunc(index)
                        }
                        wasmparser::CompositeInnerType::Struct(_) => {
                            WasmHeapType::ConcreteStruct(index)
                        }
                        wasmparser::CompositeInnerType::Cont(_) => {
                            WasmHeapType::ConcreteCont(index)
                        }
                    }
                } else {
                    panic!("forward reference to type outside of rec group?")
                }
            }

            UnpackedIndex::RecGroup(_) => unreachable!(),
        }
    }

    fn lookup_type_index(&self, index: wasmparser::UnpackedIndex) -> EngineOrModuleTypeIndex {
        match index {
            UnpackedIndex::Id(id) => {
                let interned = self.types.wasmparser_to_wasmtime[&id];
                EngineOrModuleTypeIndex::Module(interned)
            }
            UnpackedIndex::Module(module_index) => {
                let module_index = TypeIndex::from_u32(module_index);
                let interned = (self.lookup_type_idx)(module_index);
                EngineOrModuleTypeIndex::Module(interned)
            }
            UnpackedIndex::RecGroup(_) => unreachable!(),
        }
    }
}
