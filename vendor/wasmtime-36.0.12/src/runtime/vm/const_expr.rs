//! Evaluating const expressions.

use crate::prelude::*;
use crate::runtime::vm::{I31, VMGcRef, ValRaw};
use crate::store::{AutoAssertNoGc, InstanceId, StoreOpaque};
#[cfg(feature = "gc")]
use crate::{
    AnyRef, ArrayRef, ArrayRefPre, ArrayType, ExternRef, StructRef, StructRefPre, StructType, Val,
};
use smallvec::SmallVec;
use wasmtime_environ::{ConstExpr, ConstOp, FuncIndex, GlobalIndex};
#[cfg(feature = "gc")]
use wasmtime_environ::{
    Unsigned, VMSharedTypeIndex, WasmCompositeInnerType, WasmCompositeType, WasmSubType,
};

/// An interpreter for const expressions.
///
/// This can be reused across many const expression evaluations to reuse
/// allocated resources, if any.
#[derive(Default)]
pub struct ConstExprEvaluator {
    stack: SmallVec<[ValRaw; 2]>,
}

/// The context within which a particular const expression is evaluated.
pub struct ConstEvalContext {
    pub(crate) instance: InstanceId,
}

impl ConstEvalContext {
    /// Create a new context.
    pub fn new(instance: InstanceId) -> Self {
        Self { instance }
    }

    fn global_get(&mut self, store: &mut AutoAssertNoGc<'_>, index: GlobalIndex) -> Result<ValRaw> {
        unsafe {
            let mut instance = store.instance_mut(self.instance);
            let global = instance
                .as_mut()
                .defined_or_imported_global_ptr(index)
                .as_ref();
            let wasm_ty = instance.env_module().globals[index].wasm_ty;
            global.to_val_raw(store, wasm_ty)
        }
    }

    fn ref_func(&mut self, store: &mut AutoAssertNoGc<'_>, index: FuncIndex) -> Result<ValRaw> {
        Ok(ValRaw::funcref(
            store
                .instance_mut(self.instance)
                .get_func_ref(index)
                .unwrap()
                .as_ptr()
                .cast(),
        ))
    }

    #[cfg(feature = "gc")]
    fn struct_fields_len(
        &self,
        store: &mut AutoAssertNoGc<'_>,
        shared_ty: VMSharedTypeIndex,
    ) -> usize {
        let struct_ty = StructType::from_shared_type_index(store.engine(), shared_ty);
        let fields = struct_ty.fields();
        fields.len()
    }

    /// Safety: field values must be of the correct types.
    #[cfg(feature = "gc")]
    unsafe fn struct_new(
        &mut self,
        store: &mut AutoAssertNoGc<'_>,
        shared_ty: VMSharedTypeIndex,
        fields: &[ValRaw],
    ) -> Result<ValRaw> {
        let struct_ty = StructType::from_shared_type_index(store.engine(), shared_ty);
        let fields = fields
            .iter()
            .zip(struct_ty.fields())
            .map(|(raw, ty)| {
                let ty = ty.element_type().unpack();
                unsafe { Val::_from_raw(store, *raw, ty) }
            })
            .collect::<Vec<_>>();

        let allocator = StructRefPre::_new(store, struct_ty);
        let struct_ref = unsafe { StructRef::new_maybe_async(store, &allocator, &fields)? };
        let raw = struct_ref.to_anyref()._to_raw(store)?;
        Ok(ValRaw::anyref(raw))
    }

    #[cfg(feature = "gc")]
    fn struct_new_default(
        &mut self,
        store: &mut AutoAssertNoGc<'_>,
        shared_ty: VMSharedTypeIndex,
    ) -> Result<ValRaw> {
        let module = store
            .instance(self.instance)
            .runtime_module()
            .expect("should never be allocating a struct type defined in a dummy module");

        let borrowed = module
            .engine()
            .signatures()
            .borrow(shared_ty)
            .expect("should have a registered type for struct");
        let WasmSubType {
            composite_type:
                WasmCompositeType {
                    shared: false,
                    inner: WasmCompositeInnerType::Struct(struct_ty),
                },
            ..
        } = &*borrowed
        else {
            unreachable!("registered type should be a struct");
        };

        let fields = struct_ty
            .fields
            .iter()
            .map(|ty| match &ty.element_type {
                wasmtime_environ::WasmStorageType::I8 | wasmtime_environ::WasmStorageType::I16 => {
                    ValRaw::i32(0)
                }
                wasmtime_environ::WasmStorageType::Val(v) => match v {
                    wasmtime_environ::WasmValType::I32 => ValRaw::i32(0),
                    wasmtime_environ::WasmValType::I64 => ValRaw::i64(0),
                    wasmtime_environ::WasmValType::F32 => ValRaw::f32(0.0f32.to_bits()),
                    wasmtime_environ::WasmValType::F64 => ValRaw::f64(0.0f64.to_bits()),
                    wasmtime_environ::WasmValType::V128 => ValRaw::v128(0),
                    wasmtime_environ::WasmValType::Ref(r) => {
                        assert!(r.nullable);
                        ValRaw::null()
                    }
                },
            })
            .collect::<SmallVec<[_; 8]>>();

        unsafe { self.struct_new(store, shared_ty, &fields) }
    }
}

impl ConstExprEvaluator {
    /// Evaluate the given const expression in the given context.
    ///
    /// # Unsafety
    ///
    /// When async is enabled, this may only be executed on a fiber stack.
    ///
    /// The given const expression must be valid within the given context,
    /// e.g. the const expression must be well-typed and the context must return
    /// global values of the expected types. This evaluator operates directly on
    /// untyped `ValRaw`s and does not and cannot check that its operands are of
    /// the correct type.
    ///
    /// If given async store, then this must be called from on an async fiber
    /// stack.
    pub unsafe fn eval(
        &mut self,
        store: &mut StoreOpaque,
        context: &mut ConstEvalContext,
        expr: &ConstExpr,
    ) -> Result<ValRaw> {
        log::trace!("evaluating const expr: {expr:?}");

        self.stack.clear();

        // Ensure that we don't permanently root any GC references we allocate
        // during const evaluation, keeping them alive for the duration of the
        // store's lifetime.
        #[cfg(feature = "gc")]
        let mut store = crate::OpaqueRootScope::new(store);
        #[cfg(not(feature = "gc"))]
        let mut store = store;

        // We cannot allow GC during const evaluation because the stack of
        // `ValRaw`s are not rooted. If we had a GC reference on our stack, and
        // then performed a collection, that on-stack reference's object could
        // be reclaimed or relocated by the collector, and then when we use the
        // reference again we would basically get a use-after-free bug.
        let mut store = AutoAssertNoGc::new(&mut store);

        for op in expr.ops() {
            log::trace!("const-evaluating op: {op:?}");
            match op {
                ConstOp::I32Const(i) => self.stack.push(ValRaw::i32(*i)),
                ConstOp::I64Const(i) => self.stack.push(ValRaw::i64(*i)),
                ConstOp::F32Const(f) => self.stack.push(ValRaw::f32(*f)),
                ConstOp::F64Const(f) => self.stack.push(ValRaw::f64(*f)),
                ConstOp::V128Const(v) => self.stack.push(ValRaw::v128(*v)),
                ConstOp::GlobalGet(g) => self.stack.push(context.global_get(&mut store, *g)?),
                ConstOp::RefNull => self.stack.push(ValRaw::null()),
                ConstOp::RefFunc(f) => self.stack.push(context.ref_func(&mut store, *f)?),
                ConstOp::RefI31 => {
                    let i = self.pop()?.get_i32();
                    let i31 = I31::wrapping_i32(i);
                    let raw = VMGcRef::from_i31(i31).as_raw_u32();
                    self.stack.push(ValRaw::anyref(raw));
                }
                ConstOp::I32Add => {
                    let b = self.pop()?.get_i32();
                    let a = self.pop()?.get_i32();
                    self.stack.push(ValRaw::i32(a.wrapping_add(b)));
                }
                ConstOp::I32Sub => {
                    let b = self.pop()?.get_i32();
                    let a = self.pop()?.get_i32();
                    self.stack.push(ValRaw::i32(a.wrapping_sub(b)));
                }
                ConstOp::I32Mul => {
                    let b = self.pop()?.get_i32();
                    let a = self.pop()?.get_i32();
                    self.stack.push(ValRaw::i32(a.wrapping_mul(b)));
                }
                ConstOp::I64Add => {
                    let b = self.pop()?.get_i64();
                    let a = self.pop()?.get_i64();
                    self.stack.push(ValRaw::i64(a.wrapping_add(b)));
                }
                ConstOp::I64Sub => {
                    let b = self.pop()?.get_i64();
                    let a = self.pop()?.get_i64();
                    self.stack.push(ValRaw::i64(a.wrapping_sub(b)));
                }
                ConstOp::I64Mul => {
                    let b = self.pop()?.get_i64();
                    let a = self.pop()?.get_i64();
                    self.stack.push(ValRaw::i64(a.wrapping_mul(b)));
                }

                #[cfg(not(feature = "gc"))]
                ConstOp::StructNew { .. }
                | ConstOp::StructNewDefault { .. }
                | ConstOp::ArrayNew { .. }
                | ConstOp::ArrayNewDefault { .. }
                | ConstOp::ArrayNewFixed { .. }
                | ConstOp::ExternConvertAny
                | ConstOp::AnyConvertExtern => {
                    bail!(
                        "const expr evaluation error: struct operations are not \
                         supported without the `gc` feature"
                    )
                }

                #[cfg(feature = "gc")]
                ConstOp::StructNew { struct_type_index } => {
                    let interned_type_index = store.instance(context.instance).env_module().types
                        [*struct_type_index]
                        .unwrap_engine_type_index();
                    let len = context.struct_fields_len(&mut store, interned_type_index);

                    if self.stack.len() < len {
                        bail!(
                            "const expr evaluation error: expected at least {len} values on the stack, found {}",
                            self.stack.len()
                        )
                    }

                    let start = self.stack.len() - len;
                    let s = unsafe {
                        context.struct_new(&mut store, interned_type_index, &self.stack[start..])?
                    };
                    self.stack.truncate(start);
                    self.stack.push(s);
                }

                #[cfg(feature = "gc")]
                ConstOp::StructNewDefault { struct_type_index } => {
                    let ty = store.instance(context.instance).env_module().types
                        [*struct_type_index]
                        .unwrap_engine_type_index();
                    self.stack.push(context.struct_new_default(&mut store, ty)?);
                }

                #[cfg(feature = "gc")]
                ConstOp::ArrayNew { array_type_index } => {
                    let ty = store.instance(context.instance).env_module().types[*array_type_index]
                        .unwrap_engine_type_index();
                    let ty = ArrayType::from_shared_type_index(store.engine(), ty);

                    let len = self.pop()?.get_i32().unsigned();

                    let elem = unsafe {
                        Val::_from_raw(&mut store, self.pop()?, ty.element_type().unpack())
                    };

                    let pre = ArrayRefPre::_new(&mut store, ty);
                    let array = unsafe { ArrayRef::new_maybe_async(&mut store, &pre, &elem, len)? };

                    self.stack
                        .push(ValRaw::anyref(array.to_anyref()._to_raw(&mut store)?));
                }

                #[cfg(feature = "gc")]
                ConstOp::ArrayNewDefault { array_type_index } => {
                    let ty = store.instance(context.instance).env_module().types[*array_type_index]
                        .unwrap_engine_type_index();
                    let ty = ArrayType::from_shared_type_index(store.engine(), ty);

                    let len = self.pop()?.get_i32().unsigned();

                    let elem = Val::default_for_ty(ty.element_type().unpack())
                        .expect("type should have a default value");

                    let pre = ArrayRefPre::_new(&mut store, ty);
                    let array = unsafe { ArrayRef::new_maybe_async(&mut store, &pre, &elem, len)? };

                    self.stack
                        .push(ValRaw::anyref(array.to_anyref()._to_raw(&mut store)?));
                }

                #[cfg(feature = "gc")]
                ConstOp::ArrayNewFixed {
                    array_type_index,
                    array_size,
                } => {
                    let ty = store.instance(context.instance).env_module().types[*array_type_index]
                        .unwrap_engine_type_index();
                    let ty = ArrayType::from_shared_type_index(store.engine(), ty);

                    let array_size = usize::try_from(*array_size).unwrap();
                    if self.stack.len() < array_size {
                        bail!(
                            "const expr evaluation error: expected at least {array_size} values on the stack, found {}",
                            self.stack.len()
                        )
                    }

                    let start = self.stack.len() - array_size;

                    let elem_ty = ty.element_type();
                    let elem_ty = elem_ty.unpack();

                    let elems = self
                        .stack
                        .drain(start..)
                        .map(|raw| unsafe { Val::_from_raw(&mut store, raw, elem_ty) })
                        .collect::<SmallVec<[_; 8]>>();

                    let pre = ArrayRefPre::_new(&mut store, ty);
                    let array =
                        unsafe { ArrayRef::new_fixed_maybe_async(&mut store, &pre, &elems)? };

                    self.stack
                        .push(ValRaw::anyref(array.to_anyref()._to_raw(&mut store)?));
                }

                #[cfg(feature = "gc")]
                ConstOp::ExternConvertAny => {
                    let result = match AnyRef::_from_raw(&mut store, self.pop()?.get_anyref()) {
                        Some(anyref) => {
                            ExternRef::_convert_any(&mut store, anyref)?._to_raw(&mut store)?
                        }
                        None => 0,
                    };
                    self.stack.push(ValRaw::externref(result));
                }

                #[cfg(feature = "gc")]
                ConstOp::AnyConvertExtern => {
                    let result =
                        match ExternRef::_from_raw(&mut store, self.pop()?.get_externref()) {
                            Some(externref) => AnyRef::_convert_extern(&mut store, externref)?
                                ._to_raw(&mut store)?,
                            None => 0,
                        };
                    self.stack.push(ValRaw::anyref(result));
                }
            }
        }

        if self.stack.len() == 1 {
            log::trace!("const expr evaluated to {:?}", self.stack[0]);
            Ok(self.stack[0])
        } else {
            bail!(
                "const expr evaluation error: expected 1 resulting value, found {}",
                self.stack.len()
            )
        }
    }

    fn pop(&mut self) -> Result<ValRaw> {
        self.stack.pop().ok_or_else(|| {
            anyhow!(
                "const expr evaluation error: attempted to pop from an empty \
                 evaluation stack"
            )
        })
    }
}
