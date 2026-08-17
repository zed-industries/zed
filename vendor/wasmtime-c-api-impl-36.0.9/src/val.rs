use crate::r#ref::ref_to_val;
use crate::{
    WASM_I32, WasmtimeStoreContextMut, from_valtype, into_valtype, wasm_ref_t, wasm_valkind_t,
    wasmtime_anyref_t, wasmtime_externref_t, wasmtime_valkind_t,
};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr;
use wasmtime::{AsContextMut, Func, HeapType, Ref, RootScope, Val, ValType};

#[repr(C)]
pub struct wasm_val_t {
    pub kind: wasm_valkind_t,
    pub of: wasm_val_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union wasm_val_union {
    pub i32: i32,
    pub i64: i64,
    pub u32: u32,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
    pub ref_: *mut wasm_ref_t,
}

impl Drop for wasm_val_t {
    fn drop(&mut self) {
        match into_valtype(self.kind) {
            ValType::Ref(_) => unsafe {
                if !self.of.ref_.is_null() {
                    drop(Box::from_raw(self.of.ref_));
                }
            },
            _ => {}
        }
    }
}

impl Clone for wasm_val_t {
    fn clone(&self) -> Self {
        let mut ret = wasm_val_t {
            kind: self.kind,
            of: self.of,
        };
        unsafe {
            match into_valtype(self.kind) {
                ValType::Ref(_) if !self.of.ref_.is_null() => {
                    ret.of.ref_ = Box::into_raw(Box::new((*self.of.ref_).clone()));
                }
                _ => {}
            }
        }
        return ret;
    }
}

impl Default for wasm_val_t {
    fn default() -> Self {
        wasm_val_t {
            kind: WASM_I32,
            of: wasm_val_union { i32: 0 },
        }
    }
}

impl wasm_val_t {
    pub fn from_val(val: Val) -> wasm_val_t {
        match val {
            Val::I32(i) => wasm_val_t {
                kind: from_valtype(&ValType::I32),
                of: wasm_val_union { i32: i },
            },
            Val::I64(i) => wasm_val_t {
                kind: from_valtype(&ValType::I64),
                of: wasm_val_union { i64: i },
            },
            Val::F32(f) => wasm_val_t {
                kind: from_valtype(&ValType::F32),
                of: wasm_val_union { u32: f },
            },
            Val::F64(f) => wasm_val_t {
                kind: from_valtype(&ValType::F64),
                of: wasm_val_union { u64: f },
            },
            Val::FuncRef(f) => wasm_val_t {
                kind: from_valtype(&ValType::FUNCREF),
                of: wasm_val_union {
                    ref_: f.map_or(ptr::null_mut(), |f| {
                        Box::into_raw(Box::new(wasm_ref_t {
                            r: Ref::Func(Some(f)),
                        }))
                    }),
                },
            },
            Val::AnyRef(_) => crate::abort("creating a wasm_val_t from an anyref"),
            Val::ExternRef(_) => crate::abort("creating a wasm_val_t from an externref"),
            Val::ExnRef(_) => crate::abort("creating a wasm_val_t from  an exnref"),
            Val::V128(_) => crate::abort("creating a wasm_val_t from a v128"),
        }
    }

    pub fn val(&self) -> Val {
        match into_valtype(self.kind) {
            ValType::I32 => Val::from(unsafe { self.of.i32 }),
            ValType::I64 => Val::from(unsafe { self.of.i64 }),
            ValType::F32 => Val::from(unsafe { self.of.f32 }),
            ValType::F64 => Val::from(unsafe { self.of.f64 }),
            ValType::Ref(r) => match r.heap_type() {
                HeapType::Func => unsafe {
                    if self.of.ref_.is_null() {
                        assert!(r.is_nullable());
                        Val::FuncRef(None)
                    } else {
                        ref_to_val(&*self.of.ref_)
                    }
                },
                _ => unreachable!("wasm_val_t cannot contain non-function reference values"),
            },
            ValType::V128 => unimplemented!("wasm_val_t: v128"),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_val_copy(out: &mut MaybeUninit<wasm_val_t>, source: &wasm_val_t) {
    crate::initialize(out, source.clone());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_val_delete(val: *mut wasm_val_t) {
    ptr::drop_in_place(val);
}

#[repr(C)]
pub struct wasmtime_val_t {
    pub kind: wasmtime_valkind_t,
    pub of: wasmtime_val_union,
}

#[repr(C)]
pub union wasmtime_val_union {
    pub i32: i32,
    pub i64: i64,
    pub f32: u32,
    pub f64: u64,
    pub anyref: ManuallyDrop<wasmtime_anyref_t>,
    pub externref: ManuallyDrop<wasmtime_externref_t>,
    pub funcref: wasmtime_func_t,
    pub v128: [u8; 16],
}

const _: () = {
    assert!(std::mem::size_of::<wasmtime_val_union>() == 16);
    assert!(std::mem::align_of::<wasmtime_val_union>() == std::mem::align_of::<u64>());
};

// The raw pointers are actually optional boxes.
unsafe impl Send for wasmtime_val_union
where
    Option<Box<wasmtime_anyref_t>>: Send,
    Option<Box<wasmtime_externref_t>>: Send,
{
}
unsafe impl Sync for wasmtime_val_union
where
    Option<Box<wasmtime_anyref_t>>: Sync,
    Option<Box<wasmtime_externref_t>>: Sync,
{
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union wasmtime_func_t {
    store_id: u64,
    func: Func,
}

impl wasmtime_func_t {
    unsafe fn as_wasmtime(&self) -> Option<Func> {
        if self.store_id == 0 {
            None
        } else {
            Some(self.func)
        }
    }
}

impl From<Option<Func>> for wasmtime_func_t {
    fn from(func: Option<Func>) -> wasmtime_func_t {
        match func {
            Some(func) => wasmtime_func_t { func },
            None => wasmtime_func_t { store_id: 0 },
        }
    }
}

impl wasmtime_val_t {
    /// Creates a new `wasmtime_val_t` from a `wasmtime::Val`.
    ///
    /// Note that this requires a `RootScope` to be present to serve as proof
    /// that `val` is not require to be rooted in the store itself which would
    /// prevent GC. Callers should prefer this API where possible, creating a
    /// temporary `RootScope` when needed.
    pub fn from_val(cx: &mut RootScope<impl AsContextMut>, val: Val) -> wasmtime_val_t {
        Self::from_val_unscoped(cx, val)
    }

    /// Equivalent of [`wasmtime_val_t::from_val`] except that a `RootScope`
    /// is not required.
    ///
    /// This method should only be used when a `RootScope` is known to be
    /// elsewhere on the stack. For example this is used when we call back out
    /// to the embedder. In such a situation we know we previously entered with
    /// some other call so the root scope is on the stack there.
    pub fn from_val_unscoped(cx: impl AsContextMut, val: Val) -> wasmtime_val_t {
        match val {
            Val::I32(i) => wasmtime_val_t {
                kind: crate::WASMTIME_I32,
                of: wasmtime_val_union { i32: i },
            },
            Val::I64(i) => wasmtime_val_t {
                kind: crate::WASMTIME_I64,
                of: wasmtime_val_union { i64: i },
            },
            Val::F32(i) => wasmtime_val_t {
                kind: crate::WASMTIME_F32,
                of: wasmtime_val_union { f32: i },
            },
            Val::F64(i) => wasmtime_val_t {
                kind: crate::WASMTIME_F64,
                of: wasmtime_val_union { f64: i },
            },
            Val::AnyRef(a) => wasmtime_val_t {
                kind: crate::WASMTIME_ANYREF,
                of: wasmtime_val_union {
                    anyref: ManuallyDrop::new(a.and_then(|a| a.to_manually_rooted(cx).ok()).into()),
                },
            },
            Val::ExternRef(e) => wasmtime_val_t {
                kind: crate::WASMTIME_EXTERNREF,
                of: wasmtime_val_union {
                    externref: ManuallyDrop::new(
                        e.and_then(|e| e.to_manually_rooted(cx).ok()).into(),
                    ),
                },
            },
            Val::FuncRef(func) => wasmtime_val_t {
                kind: crate::WASMTIME_FUNCREF,
                of: wasmtime_val_union {
                    funcref: func.into(),
                },
            },
            Val::ExnRef(_) => crate::abort("exnrefs not yet supported in C API"),
            Val::V128(val) => wasmtime_val_t {
                kind: crate::WASMTIME_V128,
                of: wasmtime_val_union {
                    v128: val.as_u128().to_le_bytes(),
                },
            },
        }
    }

    /// Convert this `wasmtime_val_t` into a `wasmtime::Val`.
    ///
    /// See [`wasmtime_val_t::from_val`] for notes on the `RootScope`
    /// requirement here. Note that this is particularly meaningful for this
    /// API as the `Val` returned may contain a `Rooted<T>` which requires a
    /// `RootScope` if we don't want the value to live for the entire lifetime
    /// of the `Store`.
    pub unsafe fn to_val(&self, cx: &mut RootScope<impl AsContextMut>) -> Val {
        self.to_val_unscoped(cx)
    }

    /// Equivalent of `to_val` except doesn't require a `RootScope`.
    ///
    /// See notes on [`wasmtime_val_t::from_val_unscoped`] for notes on when to
    /// use this.
    pub unsafe fn to_val_unscoped(&self, cx: impl AsContextMut) -> Val {
        match self.kind {
            crate::WASMTIME_I32 => Val::I32(self.of.i32),
            crate::WASMTIME_I64 => Val::I64(self.of.i64),
            crate::WASMTIME_F32 => Val::F32(self.of.f32),
            crate::WASMTIME_F64 => Val::F64(self.of.f64),
            crate::WASMTIME_V128 => Val::V128(u128::from_le_bytes(self.of.v128).into()),
            crate::WASMTIME_ANYREF => {
                Val::AnyRef(self.of.anyref.as_wasmtime().map(|a| a.to_rooted(cx)))
            }
            crate::WASMTIME_EXTERNREF => {
                Val::ExternRef(self.of.externref.as_wasmtime().map(|e| e.to_rooted(cx)))
            }
            crate::WASMTIME_FUNCREF => Val::FuncRef(self.of.funcref.as_wasmtime()),
            other => panic!("unknown wasmtime_valkind_t: {other}"),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_val_unroot(
    cx: WasmtimeStoreContextMut<'_>,
    val: &mut MaybeUninit<wasmtime_val_t>,
) {
    let val = val.assume_init_read();
    match val.kind {
        crate::WASMTIME_ANYREF => {
            if let Some(val) = ManuallyDrop::into_inner(val.of.anyref).as_wasmtime() {
                val.unroot(cx);
            }
        }
        crate::WASMTIME_EXTERNREF => {
            if let Some(val) = ManuallyDrop::into_inner(val.of.externref).as_wasmtime() {
                val.unroot(cx);
            }
        }
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_val_clone(
    cx: WasmtimeStoreContextMut<'_>,
    src: &wasmtime_val_t,
    dst: &mut MaybeUninit<wasmtime_val_t>,
) {
    let mut scope = RootScope::new(cx);
    let val = src.to_val(&mut scope);
    crate::initialize(dst, wasmtime_val_t::from_val(&mut scope, val))
}
