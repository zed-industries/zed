//! Dynamic IR types

use crate::ir::GlobalValue;
use crate::ir::PrimaryMap;
use crate::ir::entities::DynamicType;
use crate::ir::types::*;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// A dynamic type object which has a base vector type and a scaling factor.
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct DynamicTypeData {
    /// Base vector type, this is the minimum size of the type.
    pub base_vector_ty: Type,
    /// The dynamic scaling factor of the base vector type.
    pub dynamic_scale: GlobalValue,
}

impl DynamicTypeData {
    /// Create a new dynamic type.
    pub fn new(base_vector_ty: Type, dynamic_scale: GlobalValue) -> Self {
        assert!(base_vector_ty.is_vector());
        Self {
            base_vector_ty,
            dynamic_scale,
        }
    }

    /// Convert 'base_vector_ty' into a concrete dynamic vector type.
    pub fn concrete(&self) -> Option<Type> {
        self.base_vector_ty.vector_to_dynamic()
    }
}

/// All allocated dynamic types.
pub type DynamicTypes = PrimaryMap<DynamicType, DynamicTypeData>;

/// Convert a dynamic-vector type to a fixed-vector type.
pub fn dynamic_to_fixed(ty: Type) -> Type {
    match ty {
        I8X8XN => I8X8,
        I8X16XN => I8X16,
        I16X4XN => I16X4,
        I16X8XN => I16X8,
        I32X2XN => I32X2,
        I32X4XN => I32X4,
        I64X2XN => I64X2,
        F32X4XN => F32X4,
        F64X2XN => F64X2,
        _ => unreachable!("unhandled type: {}", ty),
    }
}
