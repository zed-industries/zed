/* Copyright 2024 Mozilla Foundation
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

use crate::prelude::*;
use crate::{
    BlockType, BrTable, CompositeInnerType, ContType, FrameKind, FuncType, Operator, RefType,
    ResumeTable, SubType, TryTable, ValType,
};

/// To compute the arity (param and result counts) of "variable-arity"
/// operators, the operator_arity macro needs information about the
/// module's types and the current control stack. The ModuleArity
/// trait exposes this information.
pub trait ModuleArity {
    /// Type with given index
    fn sub_type_at(&self, type_idx: u32) -> Option<&SubType>;

    /// Arity (param and result counts) of tag with given index
    fn tag_type_arity(&self, at: u32) -> Option<(u32, u32)>;

    /// Type index of function with given index
    fn type_index_of_function(&self, function_idx: u32) -> Option<u32>;

    /// Function type for a given continuation type
    fn func_type_of_cont_type(&self, c: &ContType) -> Option<&FuncType>;

    /// Sub type for a given reference type
    fn sub_type_of_ref_type(&self, rt: &RefType) -> Option<&SubType>;

    /// Current height of control stack
    fn control_stack_height(&self) -> u32;

    /// BlockType and FrameKind of label with given index
    fn label_block(&self, depth: u32) -> Option<(BlockType, FrameKind)>;

    /// Computes arity of given SubType
    fn sub_type_arity(&self, t: &SubType) -> Option<(u32, u32)> {
        match &t.composite_type.inner {
            CompositeInnerType::Func(f) => {
                Some((f.params().len() as u32, f.results().len() as u32))
            }
            CompositeInnerType::Struct(s) => Some((s.fields.len() as u32, s.fields.len() as u32)),
            CompositeInnerType::Array(_) => None,
            CompositeInnerType::Cont(c) => {
                let f = self.func_type_of_cont_type(c)?;
                Some((f.params().len() as u32, f.results().len() as u32))
            }
        }
    }

    /// Computes arity of given BlockType
    fn block_type_arity(&self, ty: BlockType) -> Option<(u32, u32)> {
        match ty {
            BlockType::Empty => Some((0, 0)),
            BlockType::Type(_) => Some((0, 1)),
            BlockType::FuncType(t) => self.sub_type_arity(self.sub_type_at(t)?),
        }
    }
}

impl Operator<'_> {
    /// Compute the arity (param and result counts) of the operator, given
    /// an impl ModuleArity, which stores the necessary module state.
    pub fn operator_arity(&self, module: &impl ModuleArity) -> Option<(u32, u32)> {
        #[cfg_attr(not(feature = "simd"), allow(unused_macro_rules))]
        macro_rules! operator_arity {
            ($visit:ident $args:tt arity $a:tt -> $b:tt) => (Some(($a, $b)));
            ($visit:ident { $($args:ident)* } arity custom) => ($visit(module, $($args),*));
        }
        macro_rules! define_arity {
            ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*) )*) => (
                match self.clone() {
                    $(
                        Operator::$op $({ $($arg),* })? => {
                            $(
                                $(let _ = $arg;)*
                            )?
                            operator_arity!($visit {$( $($arg)* )?} $($ann)*)
                        }
                    )*
                }
            );
        }
        crate::for_each_operator!(define_arity)
    }
}

fn visit_block(module: &dyn ModuleArity, block: BlockType) -> Option<(u32, u32)> {
    let (params, _) = module.block_type_arity(block)?;
    Some((params, params))
}

fn visit_loop(module: &dyn ModuleArity, block: BlockType) -> Option<(u32, u32)> {
    visit_block(module, block)
}

fn visit_if(module: &dyn ModuleArity, block: BlockType) -> Option<(u32, u32)> {
    let (params, results) = visit_block(module, block)?;
    Some((params + 1, results))
}

fn visit_else(module: &dyn ModuleArity) -> Option<(u32, u32)> {
    let (ty, _kind) = module.label_block(0)?;
    let (params, results) = module.block_type_arity(ty)?;
    Some((results, params))
}

fn visit_end(module: &dyn ModuleArity) -> Option<(u32, u32)> {
    let (ty, _kind) = module.label_block(0)?;
    let (_params, results) = module.block_type_arity(ty)?;
    Some((results, results))
}

fn visit_br(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)> {
    let (ty, kind) = module.label_block(depth)?;
    let (params, results) = module.block_type_arity(ty)?;
    let n = match kind {
        FrameKind::Loop => params,
        _ => results,
    };
    Some((n, 0))
}

fn visit_br_if(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)> {
    let (params, _) = visit_br(module, depth)?;
    Some((params + 1, params))
}

fn visit_br_table(module: &dyn ModuleArity, table: BrTable<'_>) -> Option<(u32, u32)> {
    let (params, results) = visit_br(module, table.default())?;
    Some((params + 1, results))
}

fn visit_return(module: &dyn ModuleArity) -> Option<(u32, u32)> {
    let height = module.control_stack_height().checked_sub(1)?;
    let (ty, _) = module.label_block(height)?;
    let (_, results) = module.block_type_arity(ty)?;
    Some((results, 0))
}

fn visit_call(module: &dyn ModuleArity, func: u32) -> Option<(u32, u32)> {
    module.sub_type_arity(module.sub_type_at(module.type_index_of_function(func)?)?)
}

fn visit_call_indirect(module: &dyn ModuleArity, ty: u32, _table: u32) -> Option<(u32, u32)> {
    let (params, results) = module.sub_type_arity(module.sub_type_at(ty)?)?;
    Some((params + 1, results))
}

fn visit_struct_new(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)> {
    let ty = module.sub_type_at(ty)?;
    let (params, _results) = module.sub_type_arity(ty)?;
    Some((params, 1))
}

fn visit_struct_new_desc(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)> {
    let ty = module.sub_type_at(ty)?;
    let (params, _results) = module.sub_type_arity(ty)?;
    Some((params + 1, 1))
}

fn visit_array_new_fixed(_module: &dyn ModuleArity, _ty: u32, size: u32) -> Option<(u32, u32)> {
    Some((size, 1))
}

fn visit_br_on_cast(
    module: &dyn ModuleArity,
    depth: u32,
    _from: RefType,
    _to: RefType,
) -> Option<(u32, u32)> {
    let (params, _) = visit_br(module, depth)?;
    Some((params, params))
}

fn visit_br_on_cast_fail(
    module: &dyn ModuleArity,
    depth: u32,
    _from: RefType,
    _to: RefType,
) -> Option<(u32, u32)> {
    let (params, _) = visit_br(module, depth)?;
    Some((params, params))
}

fn visit_typed_select_multi(_module: &dyn ModuleArity, tys: Vec<ValType>) -> Option<(u32, u32)> {
    let len = u32::try_from(tys.len()).unwrap();
    Some((1 + 2 * len, len))
}

fn visit_return_call(module: &dyn ModuleArity, func: u32) -> Option<(u32, u32)> {
    let (params, _) = visit_call(module, func)?;
    Some((params, 0))
}

fn visit_return_call_indirect(module: &dyn ModuleArity, ty: u32, table: u32) -> Option<(u32, u32)> {
    let (params, _) = visit_call_indirect(module, ty, table)?;
    Some((params, 0))
}

fn visit_try_table(module: &dyn ModuleArity, table: TryTable) -> Option<(u32, u32)> {
    let (params, _) = module.block_type_arity(table.ty)?;
    Some((params, params))
}

fn visit_throw(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)> {
    let (params, _) = module.tag_type_arity(tag)?;
    Some((params, 0))
}

fn visit_try(module: &dyn ModuleArity, ty: BlockType) -> Option<(u32, u32)> {
    visit_block(module, ty)
}

fn visit_catch(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)> {
    let (params, _) = visit_end(module)?;
    let (tag_params, _) = module.tag_type_arity(tag)?;
    Some((params, tag_params))
}

fn visit_delegate(module: &dyn ModuleArity, _depth: u32) -> Option<(u32, u32)> {
    visit_end(module)
}

fn visit_catch_all(module: &dyn ModuleArity) -> Option<(u32, u32)> {
    let (params, _) = visit_end(module)?;
    Some((params, 0))
}

fn visit_call_ref(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)> {
    let (params, results) = module.sub_type_arity(module.sub_type_at(ty)?)?;
    Some((params + 1, results))
}

fn visit_return_call_ref(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)> {
    let (params, _) = visit_call_ref(module, ty)?;
    Some((params, 0))
}

fn visit_br_on_null(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)> {
    let (params, _results) = visit_br(module, depth)?;
    Some((params + 1, params + 1))
}

fn visit_br_on_non_null(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)> {
    let (params, _results) = visit_br(module, depth)?;
    Some((params, params.checked_sub(1)?))
}

fn visit_cont_bind(module: &dyn ModuleArity, arg: u32, result: u32) -> Option<(u32, u32)> {
    let (arg_params, _) = module.sub_type_arity(module.sub_type_at(arg)?)?;
    let (result_params, _) = module.sub_type_arity(module.sub_type_at(result)?)?;
    Some((arg_params.checked_sub(result_params)? + 1, 1))
}

fn visit_suspend(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)> {
    module.tag_type_arity(tag)
}

fn visit_resume(module: &dyn ModuleArity, cont: u32, _table: ResumeTable) -> Option<(u32, u32)> {
    let (params, results) = module.sub_type_arity(module.sub_type_at(cont)?)?;
    Some((params + 1, results))
}

fn visit_resume_throw(
    module: &dyn ModuleArity,
    cont: u32,
    tag: u32,
    _table: ResumeTable,
) -> Option<(u32, u32)> {
    let (params, _) = module.tag_type_arity(tag)?;
    let (_, results) = module.sub_type_arity(module.sub_type_at(cont)?)?;
    Some((params + 1, results))
}

fn visit_switch(module: &dyn ModuleArity, cont: u32, _tag: u32) -> Option<(u32, u32)> {
    let (params, _) = module.sub_type_arity(module.sub_type_at(cont)?)?;
    let st = &module.sub_type_at(cont)?.composite_type.inner;
    let CompositeInnerType::Cont(ct) = &st else {
        return None;
    };
    let last_param = module.func_type_of_cont_type(ct)?.params().last()?;
    let (cont_params, _) =
        module.sub_type_arity(module.sub_type_of_ref_type(&last_param.as_reference_type()?)?)?;
    Some((params, cont_params))
}

fn visit_br_on_cast_desc(
    module: &dyn ModuleArity,
    depth: u32,
    _from: RefType,
    _to: RefType,
) -> Option<(u32, u32)> {
    let (params, _) = visit_br(module, depth)?;
    Some((params + 1, params))
}

fn visit_br_on_cast_desc_fail(
    module: &dyn ModuleArity,
    depth: u32,
    _from: RefType,
    _to: RefType,
) -> Option<(u32, u32)> {
    let (params, _) = visit_br(module, depth)?;
    Some((params + 1, params))
}
