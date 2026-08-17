//! Unboxed 31-bit integers.
//!
//! Note that ideally, we would just re-export `wasmtime_runtime::I31` here, but
//! in order to get doc comments with example code that shows the use of `I31`
//! with typed functions and such, we need to wrap it in a newtype.

use crate::runtime::vm::{VMGcRef, ValRaw};
use crate::{
    HeapType, RefType, Result, ValType, WasmTy,
    store::{AutoAssertNoGc, StoreOpaque},
};
use core::fmt;
use core::mem::MaybeUninit;

/// A 31-bit integer.
///
/// Represents WebAssembly's `(ref i31)` and `(ref null i31)` (aka `i31ref`)
/// references.
///
/// You can convert this into any of the `(ref i31)` supertypes such as `(ref
/// eq)` or `(ref any)`, and their nullable equivalents. After conversion, the
/// resulting reference does not actually point at a GC object in the heap,
/// instead it is a 31-bit integer that is stored unboxed/inline in the
/// reference itself.
///
/// # Example
///
/// ```
/// # use wasmtime::*;
/// # fn _foo() -> Result<()> {
/// // Enable the Wasm GC proposal for Wasm to use i31 references.
/// let mut config = Config::new();
/// config.wasm_gc(true);
///
/// let engine = Engine::new(&config)?;
/// let mut store = Store::new(&engine, ());
///
/// // A Wasm module that exports a function that increments an i31.
/// let module = Module::new(&engine, r#"
///     (module
///         (func (export "inc_i31") (param (ref i31)) (result (ref i31))
///             local.get 0
///             i31.get_u
///             i32.const 1
///             i32.add
///             ref.i31
///         )
/// "#)?;
///
/// // Instantiate the module.
/// let instance = Instance::new(&mut store, &module, &[])?;
///
/// // Get the exported `inc_i31` function.
/// let inc_i31 = instance.get_func(&mut store, "inc_i31").unwrap();
///
/// // Call the function using the untyped functions API, meaning we need to
/// // pack our `I31` argument into an `AnyRef` that is packed into a `Val`, and
/// // then we need to do the opposite unpacking to extract the result.
/// let i31 = I31::wrapping_u32(0x1234);
/// let anyref = AnyRef::from_i31(&mut store, i31);
/// let val = Val::AnyRef(Some(anyref));
/// let mut results = [Val::null_any_ref()];
/// inc_i31.call(&mut store, &[val], &mut results)?;
/// let nullable_anyref = results[0].unwrap_anyref();
/// let anyref = nullable_anyref.unwrap();
/// let i31 = anyref.unwrap_i31(&store)?;
/// assert_eq!(i31.get_u32(), 0x1235);
///
/// // Alternatively, we can use the typed function API to make this all a lot
/// // more ergonomic.
/// let inc_i31 = inc_i31.typed::<I31, I31>(&mut store)?;
/// let i31 = I31::wrapping_u32(0x5678);
/// let result = inc_i31.call(&mut store, i31)?;
/// assert_eq!(result.get_u32(), 0x5679);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct I31(crate::runtime::vm::I31);

impl fmt::Debug for I31 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("I31")
            .field("as_u32", &self.get_u32())
            .field("as_i32", &self.get_i32())
            .finish()
    }
}

impl From<crate::runtime::vm::I31> for I31 {
    #[inline]
    fn from(value: crate::runtime::vm::I31) -> Self {
        Self(value)
    }
}

impl From<I31> for crate::runtime::vm::I31 {
    #[inline]
    fn from(value: I31) -> Self {
        value.0
    }
}

impl I31 {
    /// Get this `wasmtime::I31`'s internal `crate::runtime::vm::I31`.
    pub(crate) fn runtime_i31(self) -> crate::runtime::vm::I31 {
        self.0
    }

    /// Construct a new `I31` from the given unsigned value.
    ///
    /// Returns `None` if the value does not fit in the bottom 31 bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// // This value does not fit into 31 bits.
    /// assert!(I31::new_u32(0x8000_0000).is_none());
    ///
    /// // This value does fit into 31 bits.
    /// let x = I31::new_u32(5).unwrap();
    /// assert_eq!(x.get_u32(), 5);
    /// ```
    #[inline]
    pub fn new_u32(value: u32) -> Option<Self> {
        crate::runtime::vm::I31::new_u32(value).map(Self)
    }

    /// Construct a new `I31` from the given signed value.
    ///
    /// Returns `None` if the value does not fit in the bottom 31 bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// // This value does not fit into 31 bits.
    /// assert!(I31::new_i32(-2147483648).is_none());
    ///
    /// // This value does fit into 31 bits.
    /// let x = I31::new_i32(-5).unwrap();
    /// assert_eq!(x.get_i32(), -5);
    /// ```
    #[inline]
    pub fn new_i32(value: i32) -> Option<Self> {
        crate::runtime::vm::I31::new_i32(value).map(Self)
    }

    /// Construct a new `I31` from the given unsigned value.
    ///
    /// If the value doesn't fit in the bottom 31 bits, it is wrapped such that
    /// the wrapped value does.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// // Values that fit in 31 bits are preserved.
    /// let x = I31::wrapping_u32(5);
    /// assert_eq!(x.get_u32(), 5);
    ///
    /// // Values that do not fit in 31 bits are wrapped to 31 bits.
    /// let y = I31::wrapping_u32(0xffff_ffff);
    /// assert_eq!(y.get_u32(), 0x7fff_ffff);
    /// ```
    #[inline]
    pub fn wrapping_u32(value: u32) -> Self {
        Self(crate::runtime::vm::I31::wrapping_u32(value))
    }

    /// Construct a new `I31` from the given signed value.
    ///
    /// If the value doesn't fit in the bottom 31 bits, it is wrapped such that
    /// the wrapped value does.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// // Values that fit in 31 bits are preserved.
    /// let x = I31::wrapping_i32(-5);
    /// assert_eq!(x.get_i32(), -5);
    ///
    /// // Values that do not fit in 31 bits are wrapped to 31 bits.
    /// let y = I31::wrapping_i32(-1073741825); // 0xbfffffff
    /// assert_eq!(y.get_i32(), 1073741823);    // 0x3fffffff
    /// ```
    #[inline]
    pub fn wrapping_i32(value: i32) -> Self {
        Self(crate::runtime::vm::I31::wrapping_i32(value))
    }

    /// Get this `I31`'s value as an unsigned integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// let x = I31::new_i32(-1).unwrap();
    /// assert_eq!(x.get_u32(), 0x7fff_ffff);
    /// ```
    #[inline]
    pub fn get_u32(&self) -> u32 {
        self.0.get_u32()
    }

    /// Get this `I31`'s value as a signed integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// let x = I31::new_u32(0x7fff_ffff).unwrap();
    /// assert_eq!(x.get_i32(), -1);
    /// ```
    #[inline]
    pub fn get_i32(&self) -> i32 {
        self.0.get_i32()
    }
}

unsafe impl WasmTy for I31 {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::I31))
    }

    #[inline]
    fn compatible_with_store(&self, _store: &StoreOpaque) -> bool {
        true
    }

    fn dynamic_concrete_type_check(
        &self,
        _store: &StoreOpaque,
        _nullable: bool,
        _actual: &HeapType,
    ) -> Result<()> {
        unreachable!()
    }

    fn store(self, _store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        let gc_ref = VMGcRef::from_i31(self.into()).as_raw_u32();
        ptr.write(ValRaw::anyref(gc_ref));
        Ok(())
    }

    unsafe fn load(_store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        let raw = ptr.get_anyref();
        let gc_ref = VMGcRef::from_raw_u32(raw).expect("non-null");
        gc_ref.unwrap_i31().into()
    }
}

unsafe impl WasmTy for Option<I31> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(true, HeapType::I31))
    }

    #[inline]
    fn compatible_with_store(&self, _store: &StoreOpaque) -> bool {
        true
    }

    fn dynamic_concrete_type_check(
        &self,
        _store: &StoreOpaque,
        _nullable: bool,
        _actual: &HeapType,
    ) -> Result<()> {
        unreachable!()
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        match self {
            Some(i) => i.store(store, ptr),
            None => {
                ptr.write(ValRaw::anyref(0));
                Ok(())
            }
        }
    }

    unsafe fn load(_store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        let raw = ptr.get_anyref();
        let gc_ref = VMGcRef::from_raw_u32(raw)?;
        Some(I31(gc_ref.unwrap_i31()))
    }
}
