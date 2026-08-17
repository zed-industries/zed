/*!
This module provides functionality for polyfilling `f16` input/output variables
when the `StorageInputOutput16` capability is not available or disabled.

It works by:

1. Declaring `f16` I/O variables as `f32` in SPIR-V
2. Converting between `f16` and `f32` at runtime using `OpFConvert`
3. Maintaining mappings to track which variables need conversion
*/

use crate::back::spv::{Instruction, LocalType, NumericType, Word};
use alloc::vec::Vec;

/// Manages `f16` I/O polyfill state and operations.
#[derive(Default)]
pub(in crate::back::spv) struct F16IoPolyfill {
    use_native: bool,
    io_var_to_f32_type: crate::FastHashMap<Word, Word>,
}

impl F16IoPolyfill {
    pub fn new(use_storage_input_output_16: bool) -> Self {
        Self {
            use_native: use_storage_input_output_16,
            io_var_to_f32_type: crate::FastHashMap::default(),
        }
    }

    pub fn needs_polyfill(&self, ty_inner: &crate::TypeInner) -> bool {
        use crate::{ScalarKind as Sk, TypeInner};

        !self.use_native
            && match *ty_inner {
                TypeInner::Scalar(ref s) if s.kind == Sk::Float && s.width == 2 => true,
                TypeInner::Vector { scalar, .. }
                    if scalar.kind == Sk::Float && scalar.width == 2 =>
                {
                    true
                }
                _ => false,
            }
    }

    pub fn register_io_var(&mut self, variable_id: Word, f32_type_id: Word) {
        self.io_var_to_f32_type.insert(variable_id, f32_type_id);
    }

    pub fn get_f32_io_type(&self, variable_id: Word) -> Option<Word> {
        self.io_var_to_f32_type.get(&variable_id).copied()
    }

    pub fn emit_f16_to_f32_conversion(
        f16_value_id: Word,
        f32_type_id: Word,
        converted_id: Word,
        body: &mut Vec<Instruction>,
    ) {
        body.push(Instruction::unary(
            spirv::Op::FConvert,
            f32_type_id,
            converted_id,
            f16_value_id,
        ));
    }

    pub fn emit_f32_to_f16_conversion(
        f32_value_id: Word,
        f16_type_id: Word,
        converted_id: Word,
        body: &mut Vec<Instruction>,
    ) {
        body.push(Instruction::unary(
            spirv::Op::FConvert,
            f16_type_id,
            converted_id,
            f32_value_id,
        ));
    }

    pub fn create_polyfill_type(ty_inner: &crate::TypeInner) -> Option<LocalType> {
        use crate::{ScalarKind as Sk, TypeInner};

        match *ty_inner {
            TypeInner::Scalar(ref s) if s.kind == Sk::Float && s.width == 2 => {
                Some(LocalType::Numeric(NumericType::Scalar(crate::Scalar::F32)))
            }
            TypeInner::Vector { size, scalar } if scalar.kind == Sk::Float && scalar.width == 2 => {
                Some(LocalType::Numeric(NumericType::Vector {
                    size,
                    scalar: crate::Scalar::F32,
                }))
            }
            _ => None,
        }
    }
}

impl crate::back::spv::reclaimable::Reclaimable for F16IoPolyfill {
    fn reclaim(mut self) -> Self {
        self.io_var_to_f32_type = self.io_var_to_f32_type.reclaim();
        self
    }
}
