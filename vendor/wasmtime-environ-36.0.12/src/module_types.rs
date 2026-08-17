use crate::{
    ModuleInternedRecGroupIndex, ModuleInternedTypeIndex, PrimaryMap, TypeTrace, WasmSubType,
};
use core::ops::{Index, Range};
use cranelift_entity::{SecondaryMap, packed_option::PackedOption};
use serde_derive::{Deserialize, Serialize};

/// All types used in a core wasm module.
///
/// Note that accessing this type is primarily done through the `Index`
/// implementations for this type.
#[derive(Default, Serialize, Deserialize)]
pub struct ModuleTypes {
    rec_groups: PrimaryMap<ModuleInternedRecGroupIndex, Range<ModuleInternedTypeIndex>>,
    wasm_types: PrimaryMap<ModuleInternedTypeIndex, WasmSubType>,
    trampoline_types: SecondaryMap<ModuleInternedTypeIndex, PackedOption<ModuleInternedTypeIndex>>,
}

impl TypeTrace for ModuleTypes {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(crate::EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.wasm_types.values() {
            ty.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut crate::EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.wasm_types.values_mut() {
            ty.trace_mut(func)?;
        }
        Ok(())
    }
}

impl ModuleTypes {
    /// Returns an iterator over all the wasm function signatures found within
    /// this module.
    pub fn wasm_types(
        &self,
    ) -> impl ExactSizeIterator<Item = (ModuleInternedTypeIndex, &WasmSubType)> {
        self.wasm_types.iter()
    }

    /// Get the type at the specified index, if it exists.
    pub fn get(&self, ty: ModuleInternedTypeIndex) -> Option<&WasmSubType> {
        self.wasm_types.get(ty)
    }

    /// Get an iterator over all recursion groups defined in this module and
    /// their elements.
    pub fn rec_groups(
        &self,
    ) -> impl ExactSizeIterator<Item = (ModuleInternedRecGroupIndex, Range<ModuleInternedTypeIndex>)> + '_
    {
        self.rec_groups.iter().map(|(k, v)| (k, v.clone()))
    }

    /// Get the elements within an already-defined rec group.
    pub fn rec_group_elements(
        &self,
        rec_group: ModuleInternedRecGroupIndex,
    ) -> impl ExactSizeIterator<Item = ModuleInternedTypeIndex> + use<> {
        let range = &self.rec_groups[rec_group];
        (range.start.as_u32()..range.end.as_u32()).map(|i| ModuleInternedTypeIndex::from_u32(i))
    }

    /// Returns the number of types interned.
    pub fn len_types(&self) -> usize {
        self.wasm_types.len()
    }

    /// Adds a new type to this interned list of types.
    pub fn push(&mut self, ty: WasmSubType) -> ModuleInternedTypeIndex {
        self.wasm_types.push(ty)
    }

    /// Iterate over the trampoline function types that this module requires.
    ///
    /// Yields pairs of (1) a function type and (2) its associated trampoline
    /// type. They might be the same.
    ///
    /// See the docs for `WasmFuncType::trampoline_type` for details on
    /// trampoline types.
    pub fn trampoline_types(
        &self,
    ) -> impl Iterator<Item = (ModuleInternedTypeIndex, ModuleInternedTypeIndex)> + '_ {
        self.trampoline_types
            .iter()
            .filter_map(|(k, v)| v.expand().map(|v| (k, v)))
    }

    /// Get the trampoline type for the given function type.
    ///
    /// See the docs for `WasmFuncType::trampoline_type` for details on
    /// trampoline types.
    pub fn trampoline_type(&self, ty: ModuleInternedTypeIndex) -> ModuleInternedTypeIndex {
        debug_assert!(self[ty].is_func());
        self.trampoline_types[ty].unwrap()
    }

    /// Iterate over ever type in this set, mutably.
    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut WasmSubType> {
        self.wasm_types.iter_mut().map(|(_id, ty)| ty)
    }
}

/// Methods that only exist for `ModuleTypesBuilder`.
#[cfg(feature = "compile")]
impl ModuleTypes {
    /// Associate `trampoline_ty` as the trampoline type for `for_ty`.
    pub fn set_trampoline_type(
        &mut self,
        for_ty: ModuleInternedTypeIndex,
        trampoline_ty: ModuleInternedTypeIndex,
    ) {
        use cranelift_entity::packed_option::ReservedValue;

        debug_assert!(!for_ty.is_reserved_value());
        debug_assert!(!trampoline_ty.is_reserved_value());
        debug_assert!(self.wasm_types[for_ty].is_func());
        debug_assert!(self.trampoline_types[for_ty].is_none());
        debug_assert!(
            self.wasm_types[trampoline_ty]
                .unwrap_func()
                .is_trampoline_type()
        );

        self.trampoline_types[for_ty] = Some(trampoline_ty).into();
    }

    /// Adds a new rec group to this interned list of types.
    pub fn push_rec_group(
        &mut self,
        range: Range<ModuleInternedTypeIndex>,
    ) -> ModuleInternedRecGroupIndex {
        self.rec_groups.push(range)
    }

    /// Reserves space for `amt` more types.
    pub fn reserve(&mut self, amt: usize) {
        self.wasm_types.reserve(amt)
    }

    /// Returns the next return value of `push_rec_group`.
    pub fn next_rec_group(&self) -> ModuleInternedRecGroupIndex {
        self.rec_groups.next_key()
    }

    /// Returns the next return value of `push`.
    pub fn next_ty(&self) -> ModuleInternedTypeIndex {
        self.wasm_types.next_key()
    }
}

impl Index<ModuleInternedTypeIndex> for ModuleTypes {
    type Output = WasmSubType;

    fn index(&self, sig: ModuleInternedTypeIndex) -> &WasmSubType {
        &self.wasm_types[sig]
    }
}
