/* Copyright 2019 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    types::CoreTypeId, BinaryReaderError, FuncType, GlobalType, HeapType, MemoryType, RefType,
    SubType, TableType, ValType, WasmFeatures,
};

/// Types that qualify as Wasm validation database.
///
/// # Note
///
/// The `wasmparser` crate provides a builtin validation framework but allows
/// users of this crate to also feed the parsed Wasm into their own data
/// structure while parsing and also validate at the same time without
/// the need of an additional parsing or validation step or copying data around.
pub trait WasmModuleResources {
    /// Returns the table at given index if any.
    ///
    /// The table element type must be canonicalized.
    fn table_at(&self, at: u32) -> Option<TableType>;

    /// Returns the linear memory at given index.
    fn memory_at(&self, at: u32) -> Option<MemoryType>;

    /// Returns the tag at given index.
    ///
    /// The tag's function type must be canonicalized.
    fn tag_at(&self, at: u32) -> Option<&FuncType>;

    /// Returns the global variable at given index.
    ///
    /// The global's value type must be canonicalized.
    fn global_at(&self, at: u32) -> Option<GlobalType>;

    /// Returns the `SubType` associated with the given type index.
    ///
    /// The sub type must be canonicalized.
    fn sub_type_at(&self, type_index: u32) -> Option<&SubType>;

    /// Returns the `SubType` associated with the given core type id.
    fn sub_type_at_id(&self, id: CoreTypeId) -> &SubType;

    /// Returns the type ID associated with the given function index.
    fn type_id_of_function(&self, func_idx: u32) -> Option<CoreTypeId>;

    /// Returns the type index associated with the given function index.
    fn type_index_of_function(&self, func_index: u32) -> Option<u32>;

    /// Returns the element type at the given index.
    ///
    /// The `RefType` must be canonicalized.
    fn element_type_at(&self, at: u32) -> Option<RefType>;

    /// Is `a` a subtype of `b`?
    fn is_subtype(&self, a: ValType, b: ValType) -> bool;

    /// Is the given reference type `shared`?
    ///
    /// While abstract heap types do carry along a `shared` flag, concrete heap
    /// types do not. This function resolves those concrete heap types to
    /// determine `shared`-ness.
    fn is_shared(&self, ty: RefType) -> bool;

    /// Check and canonicalize a value type.
    ///
    /// This will validate that `t` is valid under the `features` provided and
    /// then additionally validate the structure of `t`. For example any type
    /// references that `t` makes are validated and canonicalized.
    fn check_value_type(
        &self,
        t: &mut ValType,
        features: &WasmFeatures,
        offset: usize,
    ) -> Result<(), BinaryReaderError> {
        features
            .check_value_type(*t)
            .map_err(|s| BinaryReaderError::new(s, offset))?;
        match t {
            ValType::Ref(r) => self.check_ref_type(r, offset),
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => Ok(()),
        }
    }

    /// Check and canonicalize a reference type.
    fn check_ref_type(
        &self,
        ref_type: &mut RefType,
        offset: usize,
    ) -> Result<(), BinaryReaderError> {
        let is_nullable = ref_type.is_nullable();
        let mut heap_ty = ref_type.heap_type();
        self.check_heap_type(&mut heap_ty, offset)?;
        *ref_type = RefType::new(is_nullable, heap_ty).unwrap();
        Ok(())
    }

    /// Checks that a `HeapType` is valid and then additionally place it in its
    /// canonical form.
    ///
    /// Similar to `check_value_type` but for heap types.
    fn check_heap_type(
        &self,
        heap_type: &mut HeapType,
        offset: usize,
    ) -> Result<(), BinaryReaderError>;

    /// Get the top type for the given heap type.
    fn top_type(&self, heap_type: &HeapType) -> HeapType;

    /// Returns the number of elements.
    fn element_count(&self) -> u32;

    /// Returns the number of bytes in the Wasm data section.
    fn data_count(&self) -> Option<u32>;

    /// Returns whether the function index is referenced in the module anywhere
    /// outside of the start/function sections.
    fn is_function_referenced(&self, idx: u32) -> bool;
}

impl<T> WasmModuleResources for &'_ T
where
    T: ?Sized + WasmModuleResources,
{
    fn table_at(&self, at: u32) -> Option<TableType> {
        T::table_at(self, at)
    }
    fn memory_at(&self, at: u32) -> Option<MemoryType> {
        T::memory_at(self, at)
    }
    fn tag_at(&self, at: u32) -> Option<&FuncType> {
        T::tag_at(self, at)
    }
    fn global_at(&self, at: u32) -> Option<GlobalType> {
        T::global_at(self, at)
    }
    fn sub_type_at(&self, at: u32) -> Option<&SubType> {
        T::sub_type_at(self, at)
    }
    fn sub_type_at_id(&self, at: CoreTypeId) -> &SubType {
        T::sub_type_at_id(self, at)
    }
    fn type_id_of_function(&self, func_idx: u32) -> Option<CoreTypeId> {
        T::type_id_of_function(self, func_idx)
    }
    fn type_index_of_function(&self, func_idx: u32) -> Option<u32> {
        T::type_index_of_function(self, func_idx)
    }
    fn check_heap_type(&self, t: &mut HeapType, offset: usize) -> Result<(), BinaryReaderError> {
        T::check_heap_type(self, t, offset)
    }
    fn top_type(&self, heap_type: &HeapType) -> HeapType {
        T::top_type(self, heap_type)
    }
    fn element_type_at(&self, at: u32) -> Option<RefType> {
        T::element_type_at(self, at)
    }
    fn is_subtype(&self, a: ValType, b: ValType) -> bool {
        T::is_subtype(self, a, b)
    }
    fn is_shared(&self, ty: RefType) -> bool {
        T::is_shared(self, ty)
    }
    fn element_count(&self) -> u32 {
        T::element_count(self)
    }
    fn data_count(&self) -> Option<u32> {
        T::data_count(self)
    }
    fn is_function_referenced(&self, idx: u32) -> bool {
        T::is_function_referenced(self, idx)
    }
}

impl<T> WasmModuleResources for alloc::sync::Arc<T>
where
    T: WasmModuleResources,
{
    fn table_at(&self, at: u32) -> Option<TableType> {
        T::table_at(self, at)
    }

    fn memory_at(&self, at: u32) -> Option<MemoryType> {
        T::memory_at(self, at)
    }

    fn tag_at(&self, at: u32) -> Option<&FuncType> {
        T::tag_at(self, at)
    }

    fn global_at(&self, at: u32) -> Option<GlobalType> {
        T::global_at(self, at)
    }

    fn sub_type_at(&self, type_idx: u32) -> Option<&SubType> {
        T::sub_type_at(self, type_idx)
    }

    fn sub_type_at_id(&self, id: CoreTypeId) -> &SubType {
        T::sub_type_at_id(self, id)
    }

    fn type_id_of_function(&self, func_idx: u32) -> Option<CoreTypeId> {
        T::type_id_of_function(self, func_idx)
    }

    fn type_index_of_function(&self, func_idx: u32) -> Option<u32> {
        T::type_index_of_function(self, func_idx)
    }

    fn check_heap_type(&self, t: &mut HeapType, offset: usize) -> Result<(), BinaryReaderError> {
        T::check_heap_type(self, t, offset)
    }

    fn top_type(&self, heap_type: &HeapType) -> HeapType {
        T::top_type(self, heap_type)
    }

    fn element_type_at(&self, at: u32) -> Option<RefType> {
        T::element_type_at(self, at)
    }

    fn is_subtype(&self, a: ValType, b: ValType) -> bool {
        T::is_subtype(self, a, b)
    }

    fn is_shared(&self, ty: RefType) -> bool {
        T::is_shared(self, ty)
    }

    fn element_count(&self) -> u32 {
        T::element_count(self)
    }

    fn data_count(&self) -> Option<u32> {
        T::data_count(self)
    }

    fn is_function_referenced(&self, idx: u32) -> bool {
        T::is_function_referenced(self, idx)
    }
}
