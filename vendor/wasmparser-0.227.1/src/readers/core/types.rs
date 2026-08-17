/* Copyright 2018 Mozilla Foundation
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

use crate::binary_reader::BinaryReaderErrorKind;
use crate::limits::{
    MAX_WASM_FUNCTION_PARAMS, MAX_WASM_FUNCTION_RETURNS, MAX_WASM_STRUCT_FIELDS,
    MAX_WASM_SUPERTYPES, MAX_WASM_TYPES,
};
use crate::prelude::*;
#[cfg(feature = "validate")]
use crate::types::CoreTypeId;
use crate::{BinaryReader, BinaryReaderError, FromReader, Result, SectionLimited};
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};

#[cfg(feature = "validate")]
mod matches;
#[cfg(feature = "validate")]
pub(crate) use self::matches::{Matches, WithRecGroup};

/// A packed representation of a type index.
///
/// This type is morally an `enum` of either:
///
/// 1. An index into a Wasm module's type space.
///
/// 2. A `CoreTypeId` identifier.
///
/// 3. An index into a recursion group's elements.
///
/// The latter two variants are *canonical* while the first is not. Reading raw
/// types will produce (1), while working with types after validation will
/// produce (2) and (3).
//
// This is a bit-packed `u32` with the following layout:
//
//     [ unused:u10 kind:u2 index:u20 ]
//
// It must fit in 22 bits to keep `RefType` in 24 bits and `ValType` in 32 bits,
// so the top ten bits are unused.
//
// The `index` field's interpretation depends on the `kind` field, which may be
// one of the following:
//
// * `00`: The `index` is an index into the module's type space.
//
// * `01`: The `index` is an index into the containing type's recursion group.
//
// * `10`: The `index` is a `CoreTypeId`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackedIndex(u32);

// Assert that we can fit indices up to `MAX_WASM_TYPES` inside `RefType`.
#[test]
fn can_fit_max_wasm_types_in_packed_index() {
    assert!(PackedIndex::can_represent_index(
        crate::limits::MAX_WASM_TYPES as u32
    ));
    assert!(PackedIndex::can_represent_index(
        0b00000000_00001111_00000000_00000000
    ));
    assert!(PackedIndex::can_represent_index(
        0b00000000_00000000_11111111_00000000
    ));
    assert!(PackedIndex::can_represent_index(
        0b00000000_00000000_00000000_11111111
    ));
    assert!(PackedIndex::can_represent_index(0));
}

impl PackedIndex {
    const UNUSED_MASK: u32 = u32::MAX & !(Self::KIND_MASK | Self::INDEX_MASK);
    const KIND_MASK: u32 = 0b11 << 20;
    const INDEX_MASK: u32 = (1 << 20) - 1;

    const MODULE_KIND: u32 = 0b00 << 20;
    const REC_GROUP_KIND: u32 = 0b01 << 20;
    #[cfg(feature = "validate")]
    const ID_KIND: u32 = 0b10 << 20;

    #[inline]
    pub(crate) fn unchecked_from_u32(x: u32) -> Self {
        debug_assert_eq!(Self::UNUSED_MASK & x, 0);
        Self(x)
    }

    #[inline]
    pub(crate) fn to_u32(id: Self) -> u32 {
        let x = id.0;
        debug_assert_eq!(Self::UNUSED_MASK & x, 0);
        x
    }

    #[inline]
    fn can_represent_index(index: u32) -> bool {
        index & Self::INDEX_MASK == index
    }

    #[inline]
    fn kind(&self) -> u32 {
        self.0 & Self::KIND_MASK
    }

    #[inline]
    fn index(&self) -> u32 {
        self.0 & Self::INDEX_MASK
    }

    /// Construct a `PackedIndex` from an index into a module's types space.
    #[inline]
    pub fn from_module_index(index: u32) -> Option<Self> {
        if PackedIndex::can_represent_index(index) {
            Some(PackedIndex(PackedIndex::MODULE_KIND | index))
        } else {
            None
        }
    }

    /// Construct a `PackedIndex` from an index into the index's containing
    /// recursion group.
    #[inline]
    pub fn from_rec_group_index(index: u32) -> Option<Self> {
        if PackedIndex::can_represent_index(index) {
            Some(PackedIndex(PackedIndex::REC_GROUP_KIND | index))
        } else {
            None
        }
    }

    /// Construct a `PackedIndex` from the given `CoreTypeId`.
    #[inline]
    #[cfg(feature = "validate")]
    pub fn from_id(id: CoreTypeId) -> Option<Self> {
        let index = u32::try_from(crate::types::TypeIdentifier::index(&id)).unwrap();
        if PackedIndex::can_represent_index(index) {
            Some(PackedIndex(PackedIndex::ID_KIND | index))
        } else {
            None
        }
    }

    /// Is this index in canonical form?
    #[inline]
    #[cfg(feature = "validate")]
    pub fn is_canonical(&self) -> bool {
        match self.kind() {
            Self::REC_GROUP_KIND | Self::ID_KIND => true,
            Self::MODULE_KIND => false,
            _ => unreachable!(),
        }
    }

    /// Uncompress this packed index into an actual `enum` that can be matched
    /// on.
    #[inline]
    pub fn unpack(&self) -> UnpackedIndex {
        match self.kind() {
            Self::MODULE_KIND => UnpackedIndex::Module(self.index()),
            Self::REC_GROUP_KIND => UnpackedIndex::RecGroup(self.index()),
            #[cfg(feature = "validate")]
            Self::ID_KIND => UnpackedIndex::Id(
                <CoreTypeId as crate::types::TypeIdentifier>::from_index(self.index()),
            ),
            _ => unreachable!(),
        }
    }

    /// Get the underlying index into a module's types space, if any.
    #[inline]
    pub fn as_module_index(&self) -> Option<u32> {
        if self.kind() == Self::MODULE_KIND {
            Some(self.index())
        } else {
            None
        }
    }

    /// Get the underlying index into the containing recursion group, if any.
    #[inline]
    pub fn as_rec_group_index(&self) -> Option<u32> {
        if self.kind() == Self::REC_GROUP_KIND {
            Some(self.index())
        } else {
            None
        }
    }

    /// Get the underlying `CoreTypeId`, if any.
    #[inline]
    #[cfg(feature = "validate")]
    pub fn as_core_type_id(&self) -> Option<CoreTypeId> {
        if self.kind() == Self::ID_KIND {
            Some(<CoreTypeId as crate::types::TypeIdentifier>::from_index(
                self.index(),
            ))
        } else {
            None
        }
    }
}

impl fmt::Debug for PackedIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CoreTypeIndex")
            .field(
                "kind",
                match self.kind() {
                    Self::MODULE_KIND => &"module",
                    Self::REC_GROUP_KIND => &"recgroup",
                    #[cfg(feature = "validate")]
                    Self::ID_KIND => &"id",
                    _ => unreachable!(),
                },
            )
            .field("index", &self.index())
            .finish()
    }
}

impl fmt::Display for PackedIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        fmt::Display::fmt(&self.unpack(), f)
    }
}

/// The uncompressed form of a `PackedIndex`.
///
/// Can be used for `match` statements.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum UnpackedIndex {
    /// An index into a Wasm module's types space.
    Module(u32),

    /// An index into the containing recursion group's elements.
    RecGroup(u32),

    /// A type identifier.
    #[cfg(feature = "validate")]
    Id(CoreTypeId),
}

impl UnpackedIndex {
    /// Compress this index into its packed form.
    ///
    /// Returns `None` if an index is beyond implementation limits.
    pub fn pack(&self) -> Option<PackedIndex> {
        match self {
            UnpackedIndex::Module(i) => PackedIndex::from_module_index(*i),
            UnpackedIndex::RecGroup(i) => PackedIndex::from_rec_group_index(*i),
            #[cfg(feature = "validate")]
            UnpackedIndex::Id(id) => PackedIndex::from_id(*id),
        }
    }

    /// Is this index in canonical form?
    #[inline]
    #[cfg(feature = "validate")]
    pub fn is_canonical(&self) -> bool {
        matches!(self, UnpackedIndex::RecGroup(_) | UnpackedIndex::Id(_))
    }

    /// Get the underlying index into a module's types space, if any.
    #[inline]
    pub fn as_module_index(&self) -> Option<u32> {
        if let Self::Module(i) = *self {
            Some(i)
        } else {
            None
        }
    }

    /// Get the underlying index into the containing recursion group, if any.
    #[inline]
    pub fn as_rec_group_index(&self) -> Option<u32> {
        if let Self::RecGroup(i) = *self {
            Some(i)
        } else {
            None
        }
    }

    /// Get the underlying `CoreTypeId`, if any.
    #[inline]
    #[cfg(feature = "validate")]
    pub fn as_core_type_id(&self) -> Option<CoreTypeId> {
        if let Self::Id(id) = *self {
            Some(id)
        } else {
            None
        }
    }
}

impl fmt::Display for UnpackedIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UnpackedIndex::Module(i) => write!(f, "(module {i})"),
            UnpackedIndex::RecGroup(i) => write!(f, "(recgroup {i})"),
            #[cfg(feature = "validate")]
            UnpackedIndex::Id(id) => write!(f, "(id {})", crate::types::TypeIdentifier::index(id)),
        }
    }
}

/// Represents a recursive type group in a WebAssembly module.
#[derive(Debug, Clone)]
pub struct RecGroup {
    inner: RecGroupInner,
}

#[derive(Debug, Clone)]
enum RecGroupInner {
    Implicit((usize, SubType)),
    Explicit(Vec<(usize, SubType)>),
}

impl RecGroup {
    /// Create an explicit `RecGroup` for the given types.
    pub(crate) fn explicit(types: Vec<(usize, SubType)>) -> Self {
        RecGroup {
            inner: RecGroupInner::Explicit(types),
        }
    }

    /// Create an implicit `RecGroup` for a type that was not contained
    /// in a `(rec ...)`.
    pub(crate) fn implicit(offset: usize, ty: SubType) -> Self {
        RecGroup {
            inner: RecGroupInner::Implicit((offset, ty)),
        }
    }

    /// Is this an explicit recursion group?
    pub fn is_explicit_rec_group(&self) -> bool {
        matches!(self.inner, RecGroupInner::Explicit(..))
    }

    /// Returns the list of subtypes in the recursive type group.
    pub fn types(&self) -> impl ExactSizeIterator<Item = &SubType> + '_ {
        let types = match &self.inner {
            RecGroupInner::Implicit(ty) => core::slice::from_ref(ty),
            RecGroupInner::Explicit(types) => types,
        };
        types.iter().map(|(_, ty)| ty)
    }

    /// Return a mutable borrow of the list of subtypes in this
    /// recursive type group.
    #[cfg(feature = "validate")]
    pub(crate) fn types_mut(&mut self) -> impl ExactSizeIterator<Item = &mut SubType> + '_ {
        let types = match &mut self.inner {
            RecGroupInner::Implicit(ty) => core::slice::from_mut(ty),
            RecGroupInner::Explicit(types) => types,
        };
        types.iter_mut().map(|(_, ty)| ty)
    }

    /// Returns an owning iterator of all subtypes in this recursion
    /// group.
    pub fn into_types(self) -> impl ExactSizeIterator<Item = SubType> {
        self.into_types_and_offsets().map(|(_, ty)| ty)
    }

    /// Returns an owning iterator of all subtypes in this recursion
    /// group, along with their offset.
    pub fn into_types_and_offsets(self) -> impl ExactSizeIterator<Item = (usize, SubType)> {
        return match self.inner {
            RecGroupInner::Implicit(tup) => Iter::Implicit(Some(tup)),
            RecGroupInner::Explicit(types) => Iter::Explicit(types.into_iter()),
        };

        enum Iter {
            Implicit(Option<(usize, SubType)>),
            Explicit(alloc::vec::IntoIter<(usize, SubType)>),
        }

        impl Iterator for Iter {
            type Item = (usize, SubType);

            fn next(&mut self) -> Option<(usize, SubType)> {
                match self {
                    Self::Implicit(ty) => ty.take(),
                    Self::Explicit(types) => types.next(),
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                match self {
                    Self::Implicit(None) => (0, Some(0)),
                    Self::Implicit(Some(_)) => (1, Some(1)),
                    Self::Explicit(types) => types.size_hint(),
                }
            }
        }

        impl ExactSizeIterator for Iter {}
    }
}

impl Hash for RecGroup {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let types = self.types();
        types.len().hash(hasher);
        for ty in types {
            ty.hash(hasher);
        }
    }
}

impl PartialEq for RecGroup {
    fn eq(&self, other: &RecGroup) -> bool {
        let self_tys = self.types();
        let other_tys = other.types();
        self_tys.len() == other_tys.len() && self_tys.zip(other_tys).all(|(a, b)| a == b)
    }
}

impl Eq for RecGroup {}

impl Ord for RecGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_tys = self.types();
        let other_tys = other.types();
        self_tys.cmp(other_tys)
    }
}

impl PartialOrd for RecGroup {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents a subtype of possible other types in a WebAssembly module.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubType {
    /// Is the subtype final.
    pub is_final: bool,
    /// The list of supertype indexes. As of GC MVP, there can be at most one supertype.
    pub supertype_idx: Option<PackedIndex>,
    /// The composite type of the subtype.
    pub composite_type: CompositeType,
}

impl fmt::Display for SubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_final && self.supertype_idx.is_none() {
            fmt::Display::fmt(&self.composite_type, f)
        } else {
            write!(f, "(sub ")?;
            if self.is_final {
                write!(f, "final ")?;
            }
            if let Some(idx) = self.supertype_idx {
                write!(f, "{idx} ")?;
            }
            fmt::Display::fmt(&self.composite_type, f)?;
            write!(f, ")")
        }
    }
}

impl SubType {
    /// Unwrap an `ArrayType` or panic.
    ///
    /// Does not check finality or whether there is a supertype.
    pub fn unwrap_array(&self) -> &ArrayType {
        self.composite_type.unwrap_array()
    }

    /// Construct a function `SubType`.
    pub fn func(signature: FuncType, shared: bool) -> Self {
        Self {
            is_final: true,
            supertype_idx: None,
            composite_type: CompositeType {
                inner: CompositeInnerType::Func(signature),
                shared,
            },
        }
    }

    /// Unwrap an `FuncType` or panic.
    ///
    /// Does not check finality or whether there is a supertype.
    pub fn unwrap_func(&self) -> &FuncType {
        self.composite_type.unwrap_func()
    }

    /// Unwrap an `StructType` or panic.
    ///
    /// Does not check finality or whether there is a supertype.
    pub fn unwrap_struct(&self) -> &StructType {
        self.composite_type.unwrap_struct()
    }

    /// Unwrap an `ContType` or panic.
    ///
    /// Does not check finality or whether there is a supertype.
    pub fn unwrap_cont(&self) -> &ContType {
        self.composite_type.unwrap_cont()
    }

    /// Maps any `UnpackedIndex` via the specified closure.
    #[cfg(feature = "validate")]
    pub(crate) fn remap_indices(
        &mut self,
        f: &mut dyn FnMut(&mut PackedIndex) -> Result<()>,
    ) -> Result<()> {
        if let Some(idx) = &mut self.supertype_idx {
            f(idx)?;
        }
        match &mut self.composite_type.inner {
            CompositeInnerType::Func(ty) => {
                for ty in ty.params_mut() {
                    ty.remap_indices(f)?;
                }
                for ty in ty.results_mut() {
                    ty.remap_indices(f)?;
                }
            }
            CompositeInnerType::Array(ty) => {
                ty.0.remap_indices(f)?;
            }
            CompositeInnerType::Struct(ty) => {
                for field in ty.fields.iter_mut() {
                    field.remap_indices(f)?;
                }
            }
            CompositeInnerType::Cont(ty) => {
                ty.remap_indices(f)?;
            }
        }
        Ok(())
    }
}

/// Represents a composite type in a WebAssembly module.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeType {
    /// The type defined inside the composite type.
    pub inner: CompositeInnerType,
    /// Is the composite type shared? This is part of the
    /// shared-everything-threads proposal.
    pub shared: bool,
}

/// A [`CompositeType`] can contain one of these types.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompositeInnerType {
    /// The type is for a function.
    Func(FuncType),
    /// The type is for an array.
    Array(ArrayType),
    /// The type is for a struct.
    Struct(StructType),
    /// The type is for a continuation.
    Cont(ContType),
}

impl fmt::Display for CompositeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CompositeInnerType::*;
        if self.shared {
            write!(f, "(shared ")?;
        }
        match self.inner {
            Array(_) => write!(f, "(array ...)"),
            Func(_) => write!(f, "(func ...)"),
            Struct(_) => write!(f, "(struct ...)"),
            Cont(_) => write!(f, "(cont ...)"),
        }?;
        if self.shared {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl CompositeType {
    /// Unwrap a `FuncType` or panic.
    pub fn unwrap_func(&self) -> &FuncType {
        match &self.inner {
            CompositeInnerType::Func(f) => f,
            _ => panic!("not a func"),
        }
    }

    /// Unwrap a `ArrayType` or panic.
    pub fn unwrap_array(&self) -> &ArrayType {
        match &self.inner {
            CompositeInnerType::Array(a) => a,
            _ => panic!("not a array"),
        }
    }

    /// Unwrap a `StructType` or panic.
    pub fn unwrap_struct(&self) -> &StructType {
        match &self.inner {
            CompositeInnerType::Struct(s) => s,
            _ => panic!("not a struct"),
        }
    }

    /// Unwrap a `ContType` or panic.
    pub fn unwrap_cont(&self) -> &ContType {
        match &self.inner {
            CompositeInnerType::Cont(c) => c,
            _ => panic!("not a cont"),
        }
    }
}

/// Represents a type of a function in a WebAssembly module.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FuncType {
    /// The combined parameters and result types.
    params_results: Box<[ValType]>,
    /// The number of parameter types.
    len_params: usize,
}

impl fmt::Debug for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FuncType")
            .field("params", &self.params())
            .field("results", &self.results())
            .finish()
    }
}

impl FuncType {
    /// Creates a new [`FuncType`] from the given `params` and `results`.
    pub fn new<P, R>(params: P, results: R) -> Self
    where
        P: IntoIterator<Item = ValType>,
        R: IntoIterator<Item = ValType>,
    {
        let mut buffer = params.into_iter().collect::<Vec<_>>();
        let len_params = buffer.len();
        buffer.extend(results);
        Self {
            params_results: buffer.into(),
            len_params,
        }
    }

    /// Creates a new [`FuncType`] fom its raw parts.
    ///
    /// # Panics
    ///
    /// If `len_params` is greater than the length of `params_results` combined.
    pub(crate) fn from_raw_parts(params_results: Box<[ValType]>, len_params: usize) -> Self {
        assert!(len_params <= params_results.len());
        Self {
            params_results,
            len_params,
        }
    }

    /// Returns a shared slice to the parameter types of the [`FuncType`].
    #[inline]
    pub fn params(&self) -> &[ValType] {
        &self.params_results[..self.len_params]
    }

    /// Returns an exclusive slice to the parameter types of the
    /// [`FuncType`].
    #[inline]
    #[cfg(feature = "validate")]
    pub(crate) fn params_mut(&mut self) -> &mut [ValType] {
        &mut self.params_results[..self.len_params]
    }

    /// Returns a shared slice to the result types of the [`FuncType`].
    #[inline]
    pub fn results(&self) -> &[ValType] {
        &self.params_results[self.len_params..]
    }

    /// Returns an exclusive slice to the result types of the
    /// [`FuncType`].
    #[inline]
    #[cfg(feature = "validate")]
    pub(crate) fn results_mut(&mut self) -> &mut [ValType] {
        &mut self.params_results[self.len_params..]
    }

    #[cfg(all(feature = "validate", feature = "component-model"))]
    pub(crate) fn desc(&self) -> String {
        use core::fmt::Write;

        let mut s = String::new();
        s.push_str("[");
        for (i, param) in self.params().iter().enumerate() {
            if i > 0 {
                s.push_str(" ");
            }
            write!(s, "{param}").unwrap();
        }
        s.push_str("] -> [");
        for (i, result) in self.results().iter().enumerate() {
            if i > 0 {
                s.push_str(" ");
            }
            write!(s, "{result}").unwrap();
        }
        s.push_str("]");
        s
    }
}

/// Represents a type of an array in a WebAssembly module.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArrayType(pub FieldType);

/// Represents a field type of an array or a struct.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FieldType {
    /// Array element type.
    pub element_type: StorageType,
    /// Are elements mutable.
    pub mutable: bool,
}

impl FieldType {
    /// Maps any `UnpackedIndex` via the specified closure.
    #[cfg(feature = "validate")]
    pub(crate) fn remap_indices(
        &mut self,
        f: &mut dyn FnMut(&mut PackedIndex) -> Result<()>,
    ) -> Result<()> {
        match &mut self.element_type {
            StorageType::I8 | StorageType::I16 => Ok(()),
            StorageType::Val(ty) => ty.remap_indices(f),
        }
    }
}

/// Represents storage types introduced in the GC spec for array and struct fields.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StorageType {
    /// The storage type is i8.
    I8,
    /// The storage type is i16.
    I16,
    /// The storage type is a value type.
    Val(ValType),
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::Val(v) => fmt::Display::fmt(v, f),
        }
    }
}

impl StorageType {
    /// Is this a packed storage type, i.e. one that must be sign- or
    /// zero-extended when converted to a `ValType`?
    pub fn is_packed(&self) -> bool {
        match self {
            Self::I8 | Self::I16 => true,
            Self::Val(_) => false,
        }
    }

    /// Unpack this storage type into the valtype that it is represented as on
    /// the operand stack.
    pub fn unpack(&self) -> ValType {
        match *self {
            Self::Val(ty) => ty,
            Self::I8 | Self::I16 => ValType::I32,
        }
    }
}

/// Represents a type of a struct in a WebAssembly module.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StructType {
    /// Struct fields.
    pub fields: Box<[FieldType]>,
}

/// Represents a type of a continuation in a WebAssembly module.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ContType(pub PackedIndex);

impl ContType {
    /// Maps any `UnpackedIndex` via the specified closure.
    #[cfg(feature = "validate")]
    pub(crate) fn remap_indices(
        &mut self,
        map: &mut dyn FnMut(&mut PackedIndex) -> Result<()>,
    ) -> Result<()> {
        map(&mut self.0)?;
        Ok(())
    }
}

/// Represents the types of values in a WebAssembly module.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValType {
    /// The value type is i32.
    I32,
    /// The value type is i64.
    I64,
    /// The value type is f32.
    F32,
    /// The value type is f64.
    F64,
    /// The value type is v128.
    V128,
    /// The value type is a reference.
    Ref(RefType),
}

impl From<RefType> for ValType {
    #[inline]
    fn from(ty: RefType) -> ValType {
        ValType::Ref(ty)
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValType::I32 => f.write_str("i32"),
            ValType::I64 => f.write_str("i64"),
            ValType::F32 => f.write_str("f32"),
            ValType::F64 => f.write_str("f64"),
            ValType::V128 => f.write_str("v128"),
            ValType::Ref(r) => fmt::Display::fmt(r, f),
        }
    }
}

impl ValType {
    /// Alias for the wasm `funcref` type.
    pub const FUNCREF: ValType = ValType::Ref(RefType::FUNCREF);

    /// Alias for the wasm `externref` type.
    pub const EXTERNREF: ValType = ValType::Ref(RefType::EXTERNREF);

    /// Alias for the wasm `exnref` type.
    pub const EXNREF: ValType = ValType::Ref(RefType::EXNREF);

    /// Alias for the wasm `contref` type.
    pub const CONTREF: ValType = ValType::Ref(RefType::CONTREF);

    /// Returns whether this value type is a "reference type".
    ///
    /// Only reference types are allowed in tables, for example, and with some
    /// instructions. Current reference types include `funcref` and `externref`.
    pub fn is_reference_type(&self) -> bool {
        matches!(self, ValType::Ref(_))
    }

    /// Get the underlying reference type, if any.
    pub fn as_reference_type(&self) -> Option<RefType> {
        match *self {
            ValType::Ref(r) => Some(r),
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => None,
        }
    }

    /// Whether the type is defaultable, i.e. it is not a non-nullable reference
    /// type.
    pub fn is_defaultable(&self) -> bool {
        match *self {
            Self::I32 | Self::I64 | Self::F32 | Self::F64 | Self::V128 => true,
            Self::Ref(rt) => rt.is_nullable(),
        }
    }

    /// Maps any `UnpackedIndex` via the specified closure.
    #[cfg(feature = "validate")]
    pub(crate) fn remap_indices(
        &mut self,
        map: &mut dyn FnMut(&mut PackedIndex) -> Result<()>,
    ) -> Result<()> {
        match self {
            ValType::Ref(r) => {
                if let Some(mut idx) = r.type_index() {
                    map(&mut idx)?;
                    *r = RefType::concrete(r.is_nullable(), idx);
                }
            }
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => {}
        }
        Ok(())
    }
}

/// A reference type.
///
/// The reference types proposal first introduced `externref` and
/// `funcref`.
///
/// The function references proposal introduced typed function
/// references.
///
/// The GC proposal introduces heap types: any, eq, i31, struct, array,
/// nofunc, noextern, none.
//
// RefType is a bit-packed enum that fits in a `u24` aka `[u8; 3]`. Note that
// its content is opaque (and subject to change), but its API is stable.
//
// It has the following internal structure:
//
// ```
// [nullable:u1 concrete==1:u1 index:u22]
// [nullable:u1 concrete==0:u1 shared:u1 abstype:u4 (unused):u17]
// ```
//
// Where
//
// - `nullable` determines nullability of the ref,
//
// - `concrete` determines if the ref is of a dynamically defined type with an
//   index (encoded in a following bit-packing section) or of a known fixed
//   type,
//
// - `index` is the type index,
//
// - `shared` determines if the ref is shared, but only if it is not concrete in
//   which case we would need to examine the type at the concrete index,
//
// - `abstype` is an enumeration of abstract types:
//
//   ```
//   1111 = any
//
//   1101 = eq
//   1000 = i31
//   1001 = struct
//   1100 = array
//
//   0101 = func
//   0100 = nofunc
//
//   0011 = extern
//   0010 = noextern
//
//   0111 = cont
//   0110 = nocont
//
//   0001 = exn
//
//   0000 = none
//   ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefType([u8; 3]);

impl fmt::Debug for RefType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let heap_type = self.heap_type();
        match heap_type {
            // All abstract types follow the same general pattern.
            HeapType::Abstract { shared, ty } => {
                let nullable = self.is_nullable();

                let name = ty.as_str(nullable);
                match (nullable, shared) {
                    // Print the shortened form if nullable; i.e., `*ref`.
                    (true, true) => write!(f, "(shared {}ref)", name),
                    (true, false) => write!(f, "{}ref", name),
                    // Print the long form otherwise; i.e., `(ref *)`.
                    (false, true) => write!(f, "(ref (shared {}))", name),
                    (false, false) => write!(f, "(ref {})", name),
                }
            }
            // Handle concrete types separately; they always use the long form
            // and don't show `shared`-ness.
            HeapType::Concrete(index) => {
                if self.is_nullable() {
                    write!(f, "(ref null {})", index)
                } else {
                    write!(f, "(ref {})", index)
                }
            }
        }
    }
}

impl fmt::Display for RefType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

// Assert that we can fit indices up to `MAX_WASM_TYPES` inside `RefType`.
#[test]
fn can_fit_max_wasm_types_in_ref_type() {
    fn can_roundtrip_index(index: u32) -> bool {
        assert!(RefType::can_represent_type_index(index));
        let rt = RefType::concrete(true, PackedIndex::from_module_index(index).unwrap());
        assert!(rt.is_nullable());
        let actual_index = match rt.type_index() {
            Some(i) => i,
            None => panic!(),
        };
        actual_index.as_module_index() == Some(index)
    }

    assert!(can_roundtrip_index(crate::limits::MAX_WASM_TYPES as u32));
    assert!(can_roundtrip_index(0b00000000_00001111_00000000_00000000));
    assert!(can_roundtrip_index(0b00000000_00000000_11111111_00000000));
    assert!(can_roundtrip_index(0b00000000_00000000_00000000_11111111));
    assert!(can_roundtrip_index(0));
}

impl RefType {
    // These bits are valid for all `RefType`s.
    const NULLABLE_BIT: u32 = 1 << 23;
    const CONCRETE_BIT: u32 = 1 << 22;

    // The `abstype` field is valid only when `concrete == 0`.
    const SHARED_BIT: u32 = 1 << 21;
    const ABSTYPE_MASK: u32 = 0b1111 << 17;
    const ANY_ABSTYPE: u32 = 0b1111 << 17;
    const EQ_ABSTYPE: u32 = 0b1101 << 17;
    const I31_ABSTYPE: u32 = 0b1000 << 17;
    const STRUCT_ABSTYPE: u32 = 0b1001 << 17;
    const ARRAY_ABSTYPE: u32 = 0b1100 << 17;
    const FUNC_ABSTYPE: u32 = 0b0101 << 17;
    const NOFUNC_ABSTYPE: u32 = 0b0100 << 17;
    const EXTERN_ABSTYPE: u32 = 0b0011 << 17;
    const NOEXTERN_ABSTYPE: u32 = 0b0010 << 17;
    const EXN_ABSTYPE: u32 = 0b0001 << 17;
    const NOEXN_ABSTYPE: u32 = 0b1110 << 17;
    const NONE_ABSTYPE: u32 = 0b0000 << 17;
    const CONT_ABSTYPE: u32 = 0b0111 << 17;
    const NOCONT_ABSTYPE: u32 = 0b0110 << 17;

    // The `index` is valid only when `concrete == 1`.
    const INDEX_MASK: u32 = (1 << 22) - 1;

    /// A nullable untyped function reference aka `(ref null func)` aka
    /// `funcref` aka `anyfunc`.
    pub const FUNCREF: Self = RefType::FUNC.nullable();

    /// A nullable reference to an extern object aka `(ref null extern)` aka
    /// `externref`.
    pub const EXTERNREF: Self = RefType::EXTERN.nullable();

    /// A nullable reference to any object aka `(ref null any)` aka `anyref`.
    pub const ANYREF: Self = RefType::ANY.nullable();

    /// A nullable reference to no object aka `(ref null none)` aka `nullref`.
    pub const NULLREF: Self = RefType::NONE.nullable();

    /// A nullable reference to a noextern object aka `(ref null noextern)` aka
    /// `nullexternref`.
    pub const NULLEXTERNREF: Self = RefType::NOEXTERN.nullable();

    /// A nullable reference to a nofunc object aka `(ref null nofunc)` aka
    /// `nullfuncref`.
    pub const NULLFUNCREF: Self = RefType::NOFUNC.nullable();

    /// A nullable reference to an eq object aka `(ref null eq)` aka `eqref`.
    pub const EQREF: Self = RefType::EQ.nullable();

    /// A nullable reference to a struct aka `(ref null struct)` aka
    /// `structref`.
    pub const STRUCTREF: Self = RefType::STRUCT.nullable();

    /// A nullable reference to an array aka `(ref null array)` aka `arrayref`.
    pub const ARRAYREF: Self = RefType::ARRAY.nullable();

    /// A nullable reference to an i31 object aka `(ref null i31)` aka `i31ref`.
    pub const I31REF: Self = RefType::I31.nullable();

    /// A nullable reference to an exception object aka `(ref null exn)` aka
    /// `exnref`.
    pub const EXNREF: Self = RefType::EXN.nullable();

    /// A nullable reference to a noexn object aka `(ref null noexn)` aka
    /// `nullexnref`.
    pub const NULLEXNREF: Self = RefType::NOEXN.nullable();

    /// A nullable reference to a cont object aka `(ref null cont)` aka
    /// `contref`.
    pub const CONTREF: Self = RefType::CONT.nullable();

    /// A nullable reference to a nocont object aka `(ref null nocont)` aka
    /// `nullcontref`.
    pub const NULLCONTREF: Self = RefType::NOCONT.nullable();

    /// A non-nullable untyped function reference aka `(ref func)`.
    pub const FUNC: Self = RefType::from_u32(Self::FUNC_ABSTYPE);

    /// A non-nullable reference to an extern object aka `(ref extern)`.
    pub const EXTERN: Self = RefType::from_u32(Self::EXTERN_ABSTYPE);

    /// A non-nullable reference to any object aka `(ref any)`.
    pub const ANY: Self = RefType::from_u32(Self::ANY_ABSTYPE);

    /// A non-nullable reference to no object aka `(ref none)`.
    pub const NONE: Self = RefType::from_u32(Self::NONE_ABSTYPE);

    /// A non-nullable reference to a noextern object aka `(ref noextern)`.
    pub const NOEXTERN: Self = RefType::from_u32(Self::NOEXTERN_ABSTYPE);

    /// A non-nullable reference to a nofunc object aka `(ref nofunc)`.
    pub const NOFUNC: Self = RefType::from_u32(Self::NOFUNC_ABSTYPE);

    /// A non-nullable reference to an eq object aka `(ref eq)`.
    pub const EQ: Self = RefType::from_u32(Self::EQ_ABSTYPE);

    /// A non-nullable reference to a struct aka `(ref struct)`.
    pub const STRUCT: Self = RefType::from_u32(Self::STRUCT_ABSTYPE);

    /// A non-nullable reference to an array aka `(ref array)`.
    pub const ARRAY: Self = RefType::from_u32(Self::ARRAY_ABSTYPE);

    /// A non-nullable reference to an i31 object aka `(ref i31)`.
    pub const I31: Self = RefType::from_u32(Self::I31_ABSTYPE);

    /// A non-nullable reference to an exn object aka `(ref exn)`.
    pub const EXN: Self = RefType::from_u32(Self::EXN_ABSTYPE);

    /// A non-nullable reference to a noexn object aka `(ref noexn)`.
    pub const NOEXN: Self = RefType::from_u32(Self::NOEXN_ABSTYPE);

    /// A non-nullable reference to a cont object aka `(ref cont)`.
    pub const CONT: Self = RefType::from_u32(Self::CONT_ABSTYPE);

    /// A non-nullable reference to a nocont object aka `(ref nocont)`.
    pub const NOCONT: Self = RefType::from_u32(Self::NOCONT_ABSTYPE);

    const fn can_represent_type_index(index: u32) -> bool {
        index & Self::INDEX_MASK == index
    }

    const fn u24_to_u32(bytes: [u8; 3]) -> u32 {
        let expanded_bytes = [bytes[0], bytes[1], bytes[2], 0];
        u32::from_le_bytes(expanded_bytes)
    }

    const fn u32_to_u24(x: u32) -> [u8; 3] {
        let bytes = x.to_le_bytes();
        debug_assert!(bytes[3] == 0);
        [bytes[0], bytes[1], bytes[2]]
    }

    #[inline]
    const fn as_u32(&self) -> u32 {
        Self::u24_to_u32(self.0)
    }

    #[inline]
    const fn from_u32(x: u32) -> Self {
        debug_assert!(x & (0b11111111 << 24) == 0);

        // Either concrete or it must be a known abstract type.
        debug_assert!(
            x & Self::CONCRETE_BIT != 0
                || matches!(
                    x & Self::ABSTYPE_MASK,
                    Self::ANY_ABSTYPE
                        | Self::EQ_ABSTYPE
                        | Self::I31_ABSTYPE
                        | Self::STRUCT_ABSTYPE
                        | Self::ARRAY_ABSTYPE
                        | Self::FUNC_ABSTYPE
                        | Self::NOFUNC_ABSTYPE
                        | Self::EXTERN_ABSTYPE
                        | Self::NOEXTERN_ABSTYPE
                        | Self::NONE_ABSTYPE
                        | Self::EXN_ABSTYPE
                        | Self::NOEXN_ABSTYPE
                        | Self::CONT_ABSTYPE
                        | Self::NOCONT_ABSTYPE
                )
        );

        RefType(Self::u32_to_u24(x))
    }

    /// Create a reference to a concrete Wasm-defined type at the given
    /// index.
    ///
    /// Returns `None` when the type index is beyond this crate's
    /// implementation limits and therefore is not representable.
    pub fn concrete(nullable: bool, index: PackedIndex) -> Self {
        let index: u32 = PackedIndex::to_u32(index);
        debug_assert!(Self::can_represent_type_index(index));
        let nullable32 = Self::NULLABLE_BIT * nullable as u32;
        RefType::from_u32(nullable32 | Self::CONCRETE_BIT | index)
    }

    /// Create a new `RefType`.
    ///
    /// Returns `None` when the heap type's type index (if any) is beyond this
    /// crate's implementation limits and therefore is not representable.
    pub fn new(nullable: bool, heap_type: HeapType) -> Option<Self> {
        let base32 = Self::NULLABLE_BIT * (nullable as u32);
        match heap_type {
            HeapType::Concrete(index) => Some(RefType::concrete(nullable, index.pack()?)),
            HeapType::Abstract { shared, ty } => {
                use AbstractHeapType::*;
                let base32 = base32 | (Self::SHARED_BIT * (shared as u32));
                match ty {
                    Func => Some(Self::from_u32(base32 | Self::FUNC_ABSTYPE)),
                    Extern => Some(Self::from_u32(base32 | Self::EXTERN_ABSTYPE)),
                    Any => Some(Self::from_u32(base32 | Self::ANY_ABSTYPE)),
                    None => Some(Self::from_u32(base32 | Self::NONE_ABSTYPE)),
                    NoExtern => Some(Self::from_u32(base32 | Self::NOEXTERN_ABSTYPE)),
                    NoFunc => Some(Self::from_u32(base32 | Self::NOFUNC_ABSTYPE)),
                    Eq => Some(Self::from_u32(base32 | Self::EQ_ABSTYPE)),
                    Struct => Some(Self::from_u32(base32 | Self::STRUCT_ABSTYPE)),
                    Array => Some(Self::from_u32(base32 | Self::ARRAY_ABSTYPE)),
                    I31 => Some(Self::from_u32(base32 | Self::I31_ABSTYPE)),
                    Exn => Some(Self::from_u32(base32 | Self::EXN_ABSTYPE)),
                    NoExn => Some(Self::from_u32(base32 | Self::NOEXN_ABSTYPE)),
                    Cont => Some(Self::from_u32(base32 | Self::CONT_ABSTYPE)),
                    NoCont => Some(Self::from_u32(base32 | Self::NOCONT_ABSTYPE)),
                }
            }
        }
    }

    /// Compute the [type difference] between the two given ref types.
    ///
    /// [type difference]: https://webassembly.github.io/gc/core/valid/conventions.html#aux-reftypediff
    pub fn difference(a: RefType, b: RefType) -> RefType {
        RefType::new(
            if b.is_nullable() {
                false
            } else {
                a.is_nullable()
            },
            a.heap_type(),
        )
        .unwrap()
    }

    /// Is this a reference to an concrete type?
    pub const fn is_concrete_type_ref(&self) -> bool {
        self.as_u32() & Self::CONCRETE_BIT != 0
    }

    /// If this is a reference to a concrete Wasm-defined type, get its
    /// type index.
    pub fn type_index(&self) -> Option<PackedIndex> {
        if self.is_concrete_type_ref() {
            let index = self.as_u32() & Self::INDEX_MASK;
            Some(PackedIndex::unchecked_from_u32(index))
        } else {
            None
        }
    }

    const fn abstype(&self) -> u32 {
        debug_assert!(!self.is_concrete_type_ref());
        self.as_u32() & Self::ABSTYPE_MASK
    }

    /// Is this the abstract untyped function reference type aka `(ref
    /// null func)` aka `funcref` aka `anyfunc`?
    pub const fn is_func_ref(&self) -> bool {
        !self.is_concrete_type_ref() && self.abstype() == Self::FUNC_ABSTYPE
    }

    /// Is this the abstract external reference type aka `(ref null
    /// extern)` aka `externref`?
    pub const fn is_extern_ref(&self) -> bool {
        !self.is_concrete_type_ref() && self.abstype() == Self::EXTERN_ABSTYPE
    }

    /// Is this the abstract untyped array reference type aka `(ref null
    /// array)` aka `arrayref`?
    pub const fn is_array_ref(&self) -> bool {
        !self.is_concrete_type_ref() && self.abstype() == Self::ARRAY_ABSTYPE
    }

    /// Is this the abstract untyped struct reference type aka `(ref
    /// null struct)` aka `structref`?
    pub const fn is_struct_ref(&self) -> bool {
        !self.is_concrete_type_ref() && self.abstype() == Self::STRUCT_ABSTYPE
    }

    /// Is this the abstract untyped cont reference type aka `(ref
    /// null cont)` aka `contref`?
    pub const fn is_cont_ref(&self) -> bool {
        !self.is_concrete_type_ref() && self.abstype() == Self::CONT_ABSTYPE
    }

    /// Is this ref type nullable?
    pub const fn is_nullable(&self) -> bool {
        self.as_u32() & Self::NULLABLE_BIT != 0
    }

    /// Get the non-nullable version of this ref type.
    pub const fn as_non_null(&self) -> Self {
        Self::from_u32(self.as_u32() & !Self::NULLABLE_BIT)
    }

    /// Get the nullable version of this ref type.
    pub const fn nullable(&self) -> Self {
        Self::from_u32(self.as_u32() | Self::NULLABLE_BIT)
    }

    /// Get the shared version of this ref type as long as it is abstract.
    pub const fn shared(&self) -> Option<Self> {
        if self.is_concrete_type_ref() {
            None
        } else {
            Some(Self::from_u32(self.as_u32() | Self::SHARED_BIT))
        }
    }

    /// Get the heap type that this is a reference to.
    pub fn heap_type(&self) -> HeapType {
        let s = self.as_u32();
        if self.is_concrete_type_ref() {
            HeapType::Concrete(self.type_index().unwrap().unpack())
        } else {
            use AbstractHeapType::*;
            let shared = s & Self::SHARED_BIT != 0;
            let ty = match s & Self::ABSTYPE_MASK {
                Self::FUNC_ABSTYPE => Func,
                Self::EXTERN_ABSTYPE => Extern,
                Self::ANY_ABSTYPE => Any,
                Self::NONE_ABSTYPE => None,
                Self::NOEXTERN_ABSTYPE => NoExtern,
                Self::NOFUNC_ABSTYPE => NoFunc,
                Self::EQ_ABSTYPE => Eq,
                Self::STRUCT_ABSTYPE => Struct,
                Self::ARRAY_ABSTYPE => Array,
                Self::I31_ABSTYPE => I31,
                Self::EXN_ABSTYPE => Exn,
                Self::NOEXN_ABSTYPE => NoExn,
                Self::CONT_ABSTYPE => Cont,
                Self::NOCONT_ABSTYPE => NoCont,
                _ => unreachable!(),
            };
            HeapType::Abstract { shared, ty }
        }
    }

    // Note that this is similar to `Display for RefType` except that it has
    // the indexes stubbed out.
    #[cfg(feature = "validate")]
    pub(crate) fn wat(&self) -> &'static str {
        let nullable = self.is_nullable();
        match self.heap_type() {
            HeapType::Abstract { shared, ty } => {
                use AbstractHeapType::*;
                match (shared, nullable, ty) {
                    // Shared and nullable.
                    (true, true, Func) => "(shared funcref)",
                    (true, true, Extern) => "(shared externref)",
                    (true, true, Any) => "(shared anyref)",
                    (true, true, None) => "(shared nullref)",
                    (true, true, NoExtern) => "(shared nullexternref)",
                    (true, true, NoFunc) => "(shared nullfuncref)",
                    (true, true, Eq) => "(shared eqref)",
                    (true, true, Struct) => "(shared structref)",
                    (true, true, Array) => "(shared arrayref)",
                    (true, true, I31) => "(shared i31ref)",
                    (true, true, Exn) => "(shared exnref)",
                    (true, true, NoExn) => "(shared nullexnref)",
                    (true, true, Cont) => "(shared contref)",
                    (true, true, NoCont) => "(shared nullcontref)",
                    // Unshared but nullable.
                    (false, true, Func) => "funcref",
                    (false, true, Extern) => "externref",
                    (false, true, Any) => "anyref",
                    (false, true, None) => "nullref",
                    (false, true, NoExtern) => "nullexternref",
                    (false, true, NoFunc) => "nullfuncref",
                    (false, true, Eq) => "eqref",
                    (false, true, Struct) => "structref",
                    (false, true, Array) => "arrayref",
                    (false, true, I31) => "i31ref",
                    (false, true, Exn) => "exnref",
                    (false, true, NoExn) => "nullexnref",
                    (false, true, Cont) => "contref",
                    (false, true, NoCont) => "nullcontref",
                    // Shared but not nullable.
                    (true, false, Func) => "(ref (shared func))",
                    (true, false, Extern) => "(ref (shared extern))",
                    (true, false, Any) => "(ref (shared any))",
                    (true, false, None) => "(ref (shared none))",
                    (true, false, NoExtern) => "(ref (shared noextern))",
                    (true, false, NoFunc) => "(ref (shared nofunc))",
                    (true, false, Eq) => "(ref (shared eq))",
                    (true, false, Struct) => "(ref (shared struct))",
                    (true, false, Array) => "(ref (shared array))",
                    (true, false, I31) => "(ref (shared i31))",
                    (true, false, Exn) => "(ref (shared exn))",
                    (true, false, NoExn) => "(ref (shared noexn))",
                    (true, false, Cont) => "(ref (shared cont))",
                    (true, false, NoCont) => "(ref (shared nocont))",
                    // Neither shared nor nullable.
                    (false, false, Func) => "(ref func)",
                    (false, false, Extern) => "(ref extern)",
                    (false, false, Any) => "(ref any)",
                    (false, false, None) => "(ref none)",
                    (false, false, NoExtern) => "(ref noextern)",
                    (false, false, NoFunc) => "(ref nofunc)",
                    (false, false, Eq) => "(ref eq)",
                    (false, false, Struct) => "(ref struct)",
                    (false, false, Array) => "(ref array)",
                    (false, false, I31) => "(ref i31)",
                    (false, false, Exn) => "(ref exn)",
                    (false, false, NoExn) => "(ref noexn)",
                    (false, false, Cont) => "(ref cont)",
                    (false, false, NoCont) => "(ref nocont)",
                }
            }
            HeapType::Concrete(_) => {
                if nullable {
                    "(ref null $type)"
                } else {
                    "(ref $type)"
                }
            }
        }
    }
}

/// A heap type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HeapType {
    /// An abstract heap type; e.g., `anyref`.
    Abstract {
        /// Whether the type is shared.
        ///
        /// Introduced in the shared-everything-threads proposal.
        shared: bool,
        /// The actual heap type.
        ty: AbstractHeapType,
    },
    /// A concrete, user-defined type.
    ///
    /// Introduced in the function-references proposal.
    Concrete(UnpackedIndex),
}

impl HeapType {
    /// Alias for an unshared `func` heap type.
    pub const FUNC: Self = Self::Abstract {
        shared: false,
        ty: AbstractHeapType::Func,
    };

    /// Alias for an unshared `extern` heap type.
    pub const EXTERN: Self = Self::Abstract {
        shared: false,
        ty: AbstractHeapType::Extern,
    };
}

/// An abstract heap type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbstractHeapType {
    /// The abstract, untyped (any) function.
    ///
    /// Introduced in the references-types proposal.
    Func,

    /// The abstract, external heap type.
    ///
    /// Introduced in the references-types proposal.
    Extern,

    /// The abstract `any` heap type.
    ///
    /// The common supertype (a.k.a. top) of all internal types.
    ///
    /// Introduced in the GC proposal.
    Any,

    /// The abstract `none` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all internal types.
    ///
    /// Introduced in the GC proposal.
    None,

    /// The abstract `noextern` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all external types.
    ///
    /// Introduced in the GC proposal.
    NoExtern,

    /// The abstract `nofunc` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all function types.
    ///
    /// Introduced in the GC proposal.
    NoFunc,

    /// The abstract `eq` heap type.
    ///
    /// The common supertype of all heap types on which the `ref.eq`
    /// instruction is allowed.
    ///
    /// Introduced in the GC proposal.
    Eq,

    /// The abstract `struct` heap type.
    ///
    /// The common supertype of all struct types.
    ///
    /// Introduced in the GC proposal.
    Struct,

    /// The abstract `array` heap type.
    ///
    /// The common supertype of all array types.
    ///
    /// Introduced in the GC proposal.
    Array,

    /// The abstract `i31` heap type.
    ///
    /// It is not expected that Wasm runtimes actually store these
    /// values on the heap, but unbox them inline into the `i31ref`s
    /// themselves instead.
    ///
    /// Introduced in the GC proposal.
    I31,

    /// The abstraction `exception` heap type.
    ///
    /// Introduced in the exception-handling proposal.
    Exn,

    /// The abstract `noexn` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all exception types.
    ///
    /// Introduced in the exception-handling proposal.
    NoExn,

    /// The abstract `continuation` heap type.
    ///
    /// Introduced in the stack-switching proposal.
    Cont,

    /// The abstract `noexn` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all continuation types.
    ///
    /// Introduced in the stack-switching proposal.
    NoCont,
}

impl AbstractHeapType {
    pub(crate) const fn as_str(&self, nullable: bool) -> &str {
        use AbstractHeapType::*;
        match (nullable, self) {
            (_, Any) => "any",
            (true, None) => "null",
            (false, None) => "none",
            (true, NoExtern) => "nullextern",
            (false, NoExtern) => "noextern",
            (true, NoFunc) => "nullfunc",
            (false, NoFunc) => "nofunc",
            (_, Eq) => "eq",
            (_, Struct) => "struct",
            (_, Array) => "array",
            (_, I31) => "i31",
            (_, Extern) => "extern",
            (_, Func) => "func",
            (_, Exn) => "exn",
            (true, NoExn) => "nullexn",
            (false, NoExn) => "noexn",
            (_, Cont) => "cont",
            (true, NoCont) => "nullcont",
            (false, NoCont) => "nocont",
        }
    }

    #[cfg(feature = "validate")]
    pub(crate) fn is_subtype_of(&self, other: AbstractHeapType) -> bool {
        use AbstractHeapType::*;

        match (self, other) {
            (a, b) if *a == b => true,
            (Eq | I31 | Struct | Array | None, Any) => true,
            (I31 | Struct | Array | None, Eq) => true,
            (NoExtern, Extern) => true,
            (NoFunc, Func) => true,
            (None, I31 | Array | Struct) => true,
            (NoExn, Exn) => true,
            (NoCont, Cont) => true,
            // Nothing else matches. (Avoid full wildcard matches so
            // that adding/modifying variants is easier in the
            // future.)
            (
                Func | Extern | Exn | Any | Eq | Array | I31 | Struct | Cont | None | NoFunc
                | NoExtern | NoExn | NoCont,
                _,
            ) => false,
        }
    }
}

impl<'a> FromReader<'a> for StorageType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // NB: See `FromReader<'a> for ValType` for a table of how this
        // interacts with other value encodings.
        match reader.peek()? {
            0x78 => {
                reader.read_u8()?;
                Ok(StorageType::I8)
            }
            0x77 => {
                reader.read_u8()?;
                Ok(StorageType::I16)
            }
            _ => Ok(StorageType::Val(reader.read()?)),
        }
    }
}

impl<'a> FromReader<'a> for ValType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // Decoding value types is sort of subtle because the space of what's
        // being decoded here is actually spread out across an number of
        // locations. This comment here is intended to serve as a bit of a
        // reference to what's being decoded here and how it interacts with
        // other locations.
        //
        // Note that all value types are encoded as canonical-form negative
        // numbers in the sleb128 encoding scheme. Currently in the wasm spec
        // sleb128 isn't actually used but it looks to be modelled to allow it
        // one day. In the meantime the current values used are:
        //
        // | sleb128 | decimal | type         | notes                        |
        // |---------|---------|--------------|------------------------------|
        // | 0x7F    | -1      | i32          |                              |
        // | 0x7E    | -2      | i64          |                              |
        // | 0x7D    | -3      | f32          |                              |
        // | 0x7C    | -4      | f64          |                              |
        // | 0x7B    | -5      | v128         | simd proposal                |
        // | 0x78    | -8      | i8           | gc proposal, in `FieldType`  |
        // | 0x77    | -9      | i16          | gc proposal, in `FieldType`  |
        // | 0x75    | -11     | nocont       | stack switching proposal     |
        // | 0x74    | -12     | noexn        | gc + exceptions proposal     |
        // | 0x73    | -13     | nofunc       | gc proposal                  |
        // | 0x72    | -14     | noextern     | gc proposal                  |
        // | 0x71    | -15     | nullref      | gc proposal                  |
        // | 0x70    | -16     | func         | reference types proposal     |
        // | 0x6F    | -17     | extern       | reference types proposal     |
        // | 0x6E    | -18     | any          | gc proposal                  |
        // | 0x6D    | -19     | eq           | gc proposal                  |
        // | 0x6C    | -20     | i31          | gc proposal                  |
        // | 0x6B    | -21     | struct       | gc proposal                  |
        // | 0x6A    | -22     | array        | gc proposal                  |
        // | 0x69    | -23     | exnref       | gc + exceptions proposal     |
        // | 0x68    | -24     | contref      | stack switching proposal     |
        // | 0x65    | -27     | shared $t    | shared-everything proposal   |
        // | 0x64    | -28     | ref $t       | gc proposal, prefix byte     |
        // | 0x63    | -29     | ref null $t  | gc proposal, prefix byte     |
        // | 0x60    | -32     | func $t      | prefix byte                  |
        // | 0x5f    | -33     | struct $t    | gc proposal, prefix byte     |
        // | 0x5e    | -34     | array $t     | gc proposal, prefix byte     |
        // | 0x5d    | -35     | cont $t      | stack switching proposal     |
        // | 0x50    | -48     | sub $t       | gc proposal, prefix byte     |
        // | 0x4F    | -49     | sub final $t | gc proposal, prefix byte     |
        // | 0x4E    | -50     | rec $t       | gc proposal, prefix byte     |
        // | 0x40    | -64     | ε            | empty block type             |
        //
        // Note that not all of these encodings are parsed here, for example
        // 0x78 as the encoding for `i8` is parsed only in `FieldType`. The
        // parsing of `FieldType` will delegate here without actually consuming
        // anything though so the encoding 0x78 still must be disjoint and not
        // read here otherwise.
        match reader.peek()? {
            0x7F => {
                reader.read_u8()?;
                Ok(ValType::I32)
            }
            0x7E => {
                reader.read_u8()?;
                Ok(ValType::I64)
            }
            0x7D => {
                reader.read_u8()?;
                Ok(ValType::F32)
            }
            0x7C => {
                reader.read_u8()?;
                Ok(ValType::F64)
            }
            0x7B => {
                reader.read_u8()?;
                Ok(ValType::V128)
            }
            _ => {
                // Reclassify errors as invalid value types here because
                // that's the "root" of what was being parsed rather than
                // reference types.
                let refty = reader.read().map_err(|mut e| {
                    if let BinaryReaderErrorKind::Invalid = e.kind() {
                        e.set_message("invalid value type");
                    }
                    e
                })?;
                Ok(ValType::Ref(refty))
            }
        }
    }
}

impl<'a> FromReader<'a> for RefType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // NB: See `FromReader<'a> for ValType` for a table of how this
        // interacts with other value encodings.
        let pos = reader.original_position();
        match reader.peek()? {
            0x63 | 0x64 => {
                let nullable = reader.read_u8()? == 0x63;
                RefType::new(nullable, reader.read()?)
                    .ok_or_else(|| crate::BinaryReaderError::new("type index too large", pos))
            }
            _ => {
                // Reclassify errors as invalid reference types here because
                // that's the "root" of what was being parsed rather than
                // heap types.
                let hty = reader.read().map_err(|mut e| {
                    if let BinaryReaderErrorKind::Invalid = e.kind() {
                        e.set_message("malformed reference type");
                    }
                    e
                })?;
                RefType::new(true, hty)
                    .ok_or_else(|| crate::BinaryReaderError::new("type index too large", pos))
            }
        }
    }
}

impl<'a> FromReader<'a> for HeapType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // Unconditionally read a `s33` value. If it's positive then that means
        // it fits in a `u32` meaning that this is a valid concrete type index.
        // If it's negative, however, then it must be an abstract heap type due
        // to how non-concrete types are encoded (see `ValType` comments). In
        // that situation "rewind" the reader and go back to the original bytes
        // to parse them as an abstract heap type.
        let mut clone = reader.clone();
        let s33 = clone.read_var_s33()?;
        match u32::try_from(s33) {
            Ok(idx) => {
                // Be sure to update `reader` with the state after the s33 was
                // read.
                *reader = clone;
                let idx = PackedIndex::from_module_index(idx).ok_or_else(|| {
                    BinaryReaderError::new(
                        "type index greater than implementation limits",
                        reader.original_position(),
                    )
                })?;
                Ok(HeapType::Concrete(idx.unpack()))
            }
            Err(_) => match reader.peek()? {
                0x65 => {
                    reader.read_u8()?;
                    let ty = reader.read()?;
                    Ok(HeapType::Abstract { shared: true, ty })
                }
                _ => {
                    // Reclassify errors as "invalid heap type" here because
                    // that's the "root" of what was being parsed rather than
                    // abstract heap types.
                    let ty = reader.read().map_err(|mut e| {
                        if let BinaryReaderErrorKind::Invalid = e.kind() {
                            e.set_message("invalid heap type");
                        }
                        e
                    })?;
                    Ok(HeapType::Abstract { shared: false, ty })
                }
            },
        }
    }
}

impl<'a> FromReader<'a> for AbstractHeapType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        use AbstractHeapType::*;
        match reader.read_u8()? {
            0x70 => Ok(Func),
            0x6F => Ok(Extern),
            0x6E => Ok(Any),
            0x71 => Ok(None),
            0x72 => Ok(NoExtern),
            0x73 => Ok(NoFunc),
            0x6D => Ok(Eq),
            0x6B => Ok(Struct),
            0x6A => Ok(Array),
            0x6C => Ok(I31),
            0x69 => Ok(Exn),
            0x74 => Ok(NoExn),
            0x68 => Ok(Cont),
            0x75 => Ok(NoCont),
            _ => {
                return Err(BinaryReaderError::invalid(
                    "invalid abstract heap type",
                    reader.original_position() - 1,
                ))
            }
        }
    }
}

/// Represents a table's type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TableType {
    /// The table's element type.
    pub element_type: RefType,
    /// Whether or not this is a 64-bit table.
    ///
    /// This is part of the memory64 proposal in WebAssembly.
    pub table64: bool,
    /// Initial size of this table, in elements.
    ///
    /// For 32-bit tables (when `table64` is `false`) this is guaranteed to
    /// be at most `u32::MAX` for valid types.
    pub initial: u64,
    /// Optional maximum size of the table, in elements.
    ///
    /// For 32-bit tables (when `table64` is `false`) this is guaranteed to
    /// be at most `u32::MAX` for valid types.
    pub maximum: Option<u64>,
    /// Whether this table is shared or not.
    ///
    /// This is included the shared-everything-threads proposal.
    pub shared: bool,
}

impl TableType {
    /// Gets the index type for the table.
    pub fn index_type(&self) -> ValType {
        if self.table64 {
            ValType::I64
        } else {
            ValType::I32
        }
    }
}

/// Represents a memory's type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemoryType {
    /// Whether or not this is a 64-bit memory, using i64 as an index. If this
    /// is false it's a 32-bit memory using i32 as an index.
    ///
    /// This is part of the memory64 proposal in WebAssembly.
    pub memory64: bool,

    /// Whether or not this is a "shared" memory, indicating that it should be
    /// send-able across threads and the `maximum` field is always present for
    /// valid types.
    ///
    /// This is part of the threads proposal in WebAssembly.
    pub shared: bool,

    /// Initial size of this memory, in wasm pages.
    ///
    /// For 32-bit memories (when `memory64` is `false`) this is guaranteed to
    /// be at most `u32::MAX` for valid types.
    pub initial: u64,

    /// Optional maximum size of this memory, in wasm pages.
    ///
    /// For 32-bit memories (when `memory64` is `false`) this is guaranteed to
    /// be at most `u32::MAX` for valid types. This field is always present for
    /// valid wasm memories when `shared` is `true`.
    pub maximum: Option<u64>,

    /// The log base 2 of the memory's custom page size.
    ///
    /// Memory pages are, by default, 64KiB large (i.e. 2<sup>16</sup> or
    /// `65536`).
    ///
    /// [The custom-page-sizes proposal] allows changing it to other values.
    ///
    /// [The custom-page-sizes proposal]: https://github.com/WebAssembly/custom-page-sizes
    pub page_size_log2: Option<u32>,
}

impl MemoryType {
    /// Gets the index type for the memory.
    pub fn index_type(&self) -> ValType {
        if self.memory64 {
            ValType::I64
        } else {
            ValType::I32
        }
    }
}

/// Represents a global's type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GlobalType {
    /// The global's type.
    pub content_type: ValType,
    /// Whether or not the global is mutable.
    pub mutable: bool,
    /// Whether or not the global is shared.
    pub shared: bool,
}

/// Represents a tag kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagKind {
    /// The tag is an exception type.
    Exception,
}

/// A tag's type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TagType {
    /// The kind of tag
    pub kind: TagKind,
    /// The function type this tag uses.
    pub func_type_idx: u32,
}

/// A reader for the type section of a WebAssembly module.
pub type TypeSectionReader<'a> = SectionLimited<'a, RecGroup>;

impl<'a> TypeSectionReader<'a> {
    /// Returns an iterator over this type section which will only yield
    /// function types and any usage of GC types from the GC proposal will
    /// be translated into an error.
    pub fn into_iter_err_on_gc_types(self) -> impl Iterator<Item = Result<FuncType>> + 'a {
        self.into_iter_with_offsets().map(|item| {
            let (offset, group) = item?;
            let mut types = group.into_types();
            let ty = match (types.next(), types.next()) {
                (Some(ty), None) => ty,
                _ => bail!(offset, "gc proposal not supported"),
            };
            if !ty.is_final || ty.supertype_idx.is_some() {
                bail!(offset, "gc proposal not supported");
            }
            match ty.composite_type.inner {
                CompositeInnerType::Func(f) => Ok(f),
                CompositeInnerType::Array(_) | CompositeInnerType::Struct(_) => {
                    bail!(offset, "gc proposal not supported");
                }
                CompositeInnerType::Cont(_) => {
                    bail!(offset, "stack switching proposal not supported");
                }
            }
        })
    }
}

impl<'a> FromReader<'a> for CompositeType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        read_composite_type(reader.read_u8()?, reader)
    }
}

fn read_composite_type(
    opcode: u8,
    reader: &mut BinaryReader,
) -> Result<CompositeType, BinaryReaderError> {
    // NB: See `FromReader<'a> for ValType` for a table of how this
    // interacts with other value encodings.
    let (shared, opcode) = if opcode == 0x65 {
        (true, reader.read_u8()?)
    } else {
        (false, opcode)
    };
    let inner = match opcode {
        0x60 => CompositeInnerType::Func(reader.read()?),
        0x5e => CompositeInnerType::Array(reader.read()?),
        0x5f => CompositeInnerType::Struct(reader.read()?),
        0x5d => CompositeInnerType::Cont(reader.read()?),
        x => return reader.invalid_leading_byte(x, "type"),
    };
    Ok(CompositeType { shared, inner })
}

impl<'a> FromReader<'a> for RecGroup {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // NB: See `FromReader<'a> for ValType` for a table of how this
        // interacts with other value encodings.
        match reader.peek()? {
            0x4e => {
                reader.read_u8()?;
                let mut iter = reader.read_iter(MAX_WASM_TYPES, "rec group types")?;
                let mut types = Vec::with_capacity(iter.size_hint().0);
                let mut offset = iter.reader.original_position();
                while let Some(ty) = iter.next() {
                    types.push((offset, ty?));
                    offset = iter.reader.original_position();
                }
                Ok(RecGroup::explicit(types))
            }
            _ => Ok(RecGroup::implicit(
                reader.original_position(),
                reader.read()?,
            )),
        }
    }
}

impl<'a> FromReader<'a> for SubType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let pos = reader.original_position();
        // NB: See `FromReader<'a> for ValType` for a table of how this
        // interacts with other value encodings.
        Ok(match reader.read_u8()? {
            opcode @ (0x4f | 0x50) => {
                let idx_iter = reader.read_iter(MAX_WASM_SUPERTYPES, "supertype idxs")?;
                let idxs = idx_iter.collect::<Result<Vec<u32>>>()?;
                if idxs.len() > 1 {
                    return Err(BinaryReaderError::new(
                        "multiple supertypes not supported",
                        pos,
                    ));
                }
                let supertype_idx = idxs
                    .first()
                    .copied()
                    .map(|idx| {
                        PackedIndex::from_module_index(idx).ok_or_else(|| {
                            BinaryReaderError::new(
                                "type index greater than implementation limits",
                                reader.original_position(),
                            )
                        })
                    })
                    .transpose()?;
                SubType {
                    is_final: opcode == 0x4f,
                    supertype_idx,
                    composite_type: read_composite_type(reader.read_u8()?, reader)?,
                }
            }
            opcode => SubType {
                is_final: true,
                supertype_idx: None,
                composite_type: read_composite_type(opcode, reader)?,
            },
        })
    }
}

impl<'a> FromReader<'a> for FuncType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let mut params_results = reader
            .read_iter(MAX_WASM_FUNCTION_PARAMS, "function params")?
            .collect::<Result<Vec<_>>>()?;
        let len_params = params_results.len();
        let results = reader.read_iter(MAX_WASM_FUNCTION_RETURNS, "function returns")?;
        params_results.reserve(results.size_hint().0);
        for result in results {
            params_results.push(result?);
        }
        Ok(FuncType::from_raw_parts(params_results.into(), len_params))
    }
}

impl<'a> FromReader<'a> for FieldType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let element_type = reader.read()?;
        let mutable = reader.read_u8()?;
        Ok(FieldType {
            element_type,
            mutable: match mutable {
                0 => false,
                1 => true,
                _ => bail!(
                    reader.original_position(),
                    "malformed mutability byte for field type"
                ),
            },
        })
    }
}

impl<'a> FromReader<'a> for ArrayType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(ArrayType(FieldType::from_reader(reader)?))
    }
}

impl<'a> FromReader<'a> for StructType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let fields = reader.read_iter(MAX_WASM_STRUCT_FIELDS, "struct fields")?;
        Ok(StructType {
            fields: fields.collect::<Result<_>>()?,
        })
    }
}

impl<'a> FromReader<'a> for ContType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let idx = match u32::try_from(reader.read_var_s33()?) {
            Ok(idx) => idx,
            Err(_) => {
                bail!(reader.original_position(), "invalid continuation type");
            }
        };
        let idx = PackedIndex::from_module_index(idx).ok_or_else(|| {
            BinaryReaderError::new(
                "type index greater than implementation limits",
                reader.original_position(),
            )
        })?;
        Ok(ContType(idx))
    }
}
