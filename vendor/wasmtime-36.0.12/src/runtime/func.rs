use crate::prelude::*;
use crate::runtime::Uninhabited;
use crate::runtime::vm::{
    InterpreterRef, SendSyncPtr, StoreBox, VMArrayCallHostFuncContext, VMCommonStackInformation,
    VMContext, VMFuncRef, VMFunctionImport, VMOpaqueContext, VMStoreContext,
};
use crate::store::{AutoAssertNoGc, StoreId, StoreOpaque};
use crate::type_registry::RegisteredType;
use crate::{
    AsContext, AsContextMut, CallHook, Engine, Extern, FuncType, Instance, ModuleExport, Ref,
    StoreContext, StoreContextMut, Val, ValRaw, ValType,
};
use alloc::sync::Arc;
use core::ffi::c_void;
#[cfg(feature = "async")]
use core::future::Future;
use core::mem::{self, MaybeUninit};
use core::ptr::NonNull;
use wasmtime_environ::VMSharedTypeIndex;

/// A reference to the abstract `nofunc` heap value.
///
/// The are no instances of `(ref nofunc)`: it is an uninhabited type.
///
/// There is precisely one instance of `(ref null nofunc)`, aka `nullfuncref`:
/// the null reference.
///
/// This `NoFunc` Rust type's sole purpose is for use with [`Func::wrap`]- and
/// [`Func::typed`]-style APIs for statically typing a function as taking or
/// returning a `(ref null nofunc)` (aka `Option<NoFunc>`) which is always
/// `None`.
///
/// # Example
///
/// ```
/// # use wasmtime::*;
/// # fn _foo() -> Result<()> {
/// let mut config = Config::new();
/// config.wasm_function_references(true);
/// let engine = Engine::new(&config)?;
///
/// let module = Module::new(
///     &engine,
///     r#"
///         (module
///             (func (export "f") (param (ref null nofunc))
///                 ;; If the reference is null, return.
///                 local.get 0
///                 ref.is_null nofunc
///                 br_if 0
///
///                 ;; If the reference was not null (which is impossible)
///                 ;; then raise a trap.
///                 unreachable
///             )
///         )
///     "#,
/// )?;
///
/// let mut store = Store::new(&engine, ());
/// let instance = Instance::new(&mut store, &module, &[])?;
/// let f = instance.get_func(&mut store, "f").unwrap();
///
/// // We can cast a `(ref null nofunc)`-taking function into a typed function that
/// // takes an `Option<NoFunc>` via the `Func::typed` method.
/// let f = f.typed::<Option<NoFunc>, ()>(&store)?;
///
/// // We can call the typed function, passing the null `nofunc` reference.
/// let result = f.call(&mut store, NoFunc::null());
///
/// // The function should not have trapped, because the reference we gave it was
/// // null (as it had to be, since `NoFunc` is uninhabited).
/// assert!(result.is_ok());
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NoFunc {
    _inner: Uninhabited,
}

impl NoFunc {
    /// Get the null `(ref null nofunc)` (aka `nullfuncref`) reference.
    #[inline]
    pub fn null() -> Option<NoFunc> {
        None
    }

    /// Get the null `(ref null nofunc)` (aka `nullfuncref`) reference as a
    /// [`Ref`].
    #[inline]
    pub fn null_ref() -> Ref {
        Ref::Func(None)
    }

    /// Get the null `(ref null nofunc)` (aka `nullfuncref`) reference as a
    /// [`Val`].
    #[inline]
    pub fn null_val() -> Val {
        Val::FuncRef(None)
    }
}

/// A WebAssembly function which can be called.
///
/// This type typically represents an exported function from a WebAssembly
/// module instance. In this case a [`Func`] belongs to an [`Instance`] and is
/// loaded from there. A [`Func`] may also represent a host function as well in
/// some cases, too.
///
/// Functions can be called in a few different ways, either synchronous or async
/// and either typed or untyped (more on this below). Note that host functions
/// are normally inserted directly into a [`Linker`](crate::Linker) rather than
/// using this directly, but both options are available.
///
/// # `Func` and `async`
///
/// Functions from the perspective of WebAssembly are always synchronous. You
/// might have an `async` function in Rust, however, which you'd like to make
/// available from WebAssembly. Wasmtime supports asynchronously calling
/// WebAssembly through native stack switching. You can get some more
/// information about [asynchronous configs](crate::Config::async_support), but
/// from the perspective of `Func` it's important to know that whether or not
/// your [`Store`](crate::Store) is asynchronous will dictate whether you call
/// functions through [`Func::call`] or [`Func::call_async`] (or the typed
/// wrappers such as [`TypedFunc::call`] vs [`TypedFunc::call_async`]).
///
/// # To `Func::call` or to `Func::typed().call()`
///
/// There's a 2x2 matrix of methods to call [`Func`]. Invocations can either be
/// asynchronous or synchronous. They can also be statically typed or not.
/// Whether or not an invocation is asynchronous is indicated via the method
/// being `async` and [`call_async`](Func::call_async) being the entry point.
/// Otherwise for statically typed or not your options are:
///
/// * Dynamically typed - if you don't statically know the signature of the
///   function that you're calling you'll be using [`Func::call`] or
///   [`Func::call_async`]. These functions take a variable-length slice of
///   "boxed" arguments in their [`Val`] representation. Additionally the
///   results are returned as an owned slice of [`Val`]. These methods are not
///   optimized due to the dynamic type checks that must occur, in addition to
///   some dynamic allocations for where to put all the arguments. While this
///   allows you to call all possible wasm function signatures, if you're
///   looking for a speedier alternative you can also use...
///
/// * Statically typed - if you statically know the type signature of the wasm
///   function you're calling, then you'll want to use the [`Func::typed`]
///   method to acquire an instance of [`TypedFunc`]. This structure is static proof
///   that the underlying wasm function has the ascripted type, and type
///   validation is only done once up-front. The [`TypedFunc::call`] and
///   [`TypedFunc::call_async`] methods are much more efficient than [`Func::call`]
///   and [`Func::call_async`] because the type signature is statically known.
///   This eschews runtime checks as much as possible to get into wasm as fast
///   as possible.
///
/// # Examples
///
/// One way to get a `Func` is from an [`Instance`] after you've instantiated
/// it:
///
/// ```
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// let module = Module::new(&engine, r#"(module (func (export "foo")))"#)?;
/// let mut store = Store::new(&engine, ());
/// let instance = Instance::new(&mut store, &module, &[])?;
/// let foo = instance.get_func(&mut store, "foo").expect("export wasn't a function");
///
/// // Work with `foo` as a `Func` at this point, such as calling it
/// // dynamically...
/// match foo.call(&mut store, &[], &mut []) {
///     Ok(()) => { /* ... */ }
///     Err(trap) => {
///         panic!("execution of `foo` resulted in a wasm trap: {}", trap);
///     }
/// }
/// foo.call(&mut store, &[], &mut [])?;
///
/// // ... or we can make a static assertion about its signature and call it.
/// // Our first call here can fail if the signatures don't match, and then the
/// // second call can fail if the function traps (like the `match` above).
/// let foo = foo.typed::<(), ()>(&store)?;
/// foo.call(&mut store, ())?;
/// # Ok(())
/// # }
/// ```
///
/// You can also use the [`wrap` function](Func::wrap) to create a
/// `Func`
///
/// ```
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let mut store = Store::<()>::default();
///
/// // Create a custom `Func` which can execute arbitrary code inside of the
/// // closure.
/// let add = Func::wrap(&mut store, |a: i32, b: i32| -> i32 { a + b });
///
/// // Next we can hook that up to a wasm module which uses it.
/// let module = Module::new(
///     store.engine(),
///     r#"
///         (module
///             (import "" "" (func $add (param i32 i32) (result i32)))
///             (func (export "call_add_twice") (result i32)
///                 i32.const 1
///                 i32.const 2
///                 call $add
///                 i32.const 3
///                 i32.const 4
///                 call $add
///                 i32.add))
///     "#,
/// )?;
/// let instance = Instance::new(&mut store, &module, &[add.into()])?;
/// let call_add_twice = instance.get_typed_func::<(), i32>(&mut store, "call_add_twice")?;
///
/// assert_eq!(call_add_twice.call(&mut store, ())?, 10);
/// # Ok(())
/// # }
/// ```
///
/// Or you could also create an entirely dynamic `Func`!
///
/// ```
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let mut store = Store::<()>::default();
///
/// // Here we need to define the type signature of our `Double` function and
/// // then wrap it up in a `Func`
/// let double_type = wasmtime::FuncType::new(
///     store.engine(),
///     [wasmtime::ValType::I32].iter().cloned(),
///     [wasmtime::ValType::I32].iter().cloned(),
/// );
/// let double = Func::new(&mut store, double_type, |_, params, results| {
///     let mut value = params[0].unwrap_i32();
///     value *= 2;
///     results[0] = value.into();
///     Ok(())
/// });
///
/// let module = Module::new(
///     store.engine(),
///     r#"
///         (module
///             (import "" "" (func $double (param i32) (result i32)))
///             (func $start
///                 i32.const 1
///                 call $double
///                 drop)
///             (start $start))
///     "#,
/// )?;
/// let instance = Instance::new(&mut store, &module, &[double.into()])?;
/// // .. work with `instance` if necessary
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Debug)]
#[repr(C)] // here for the C API
pub struct Func {
    /// The store that the below pointer belongs to.
    ///
    /// It's only safe to look at the contents of the pointer below when the
    /// `StoreOpaque` matching this id is in-scope.
    store: StoreId,

    /// The raw `VMFuncRef`, whose lifetime is bound to the store this func
    /// belongs to.
    ///
    /// Note that this field has an `unsafe_*` prefix to discourage use of it.
    /// This is only safe to read/use if `self.store` is validated to belong to
    /// an ambiently provided `StoreOpaque` or similar. Use the
    /// `self.func_ref()` method instead of this field to perform this check.
    unsafe_func_ref: SendSyncPtr<VMFuncRef>,
}

// Double-check that the C representation in `extern.h` matches our in-Rust
// representation here in terms of size/alignment/etc.
const _: () = {
    #[repr(C)]
    struct C(u64, *mut u8);
    assert!(core::mem::size_of::<C>() == core::mem::size_of::<Func>());
    assert!(core::mem::align_of::<C>() == core::mem::align_of::<Func>());
    assert!(core::mem::offset_of!(Func, store) == 0);
};

macro_rules! for_each_function_signature {
    ($mac:ident) => {
        $mac!(0);
        $mac!(1 A1);
        $mac!(2 A1 A2);
        $mac!(3 A1 A2 A3);
        $mac!(4 A1 A2 A3 A4);
        $mac!(5 A1 A2 A3 A4 A5);
        $mac!(6 A1 A2 A3 A4 A5 A6);
        $mac!(7 A1 A2 A3 A4 A5 A6 A7);
        $mac!(8 A1 A2 A3 A4 A5 A6 A7 A8);
        $mac!(9 A1 A2 A3 A4 A5 A6 A7 A8 A9);
        $mac!(10 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10);
        $mac!(11 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11);
        $mac!(12 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12);
        $mac!(13 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13);
        $mac!(14 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14);
        $mac!(15 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15);
        $mac!(16 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15 A16);
        $mac!(17 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15 A16 A17);
    };
}

mod typed;
use crate::runtime::vm::VMStackChain;
pub use typed::*;

impl Func {
    /// Creates a new `Func` with the given arguments, typically to create a
    /// host-defined function to pass as an import to a module.
    ///
    /// * `store` - the store in which to create this [`Func`], which will own
    ///   the return value.
    ///
    /// * `ty` - the signature of this function, used to indicate what the
    ///   inputs and outputs are.
    ///
    /// * `func` - the native code invoked whenever this `Func` will be called.
    ///   This closure is provided a [`Caller`] as its first argument to learn
    ///   information about the caller, and then it's passed a list of
    ///   parameters as a slice along with a mutable slice of where to write
    ///   results.
    ///
    /// Note that the implementation of `func` must adhere to the `ty` signature
    /// given, error or traps may occur if it does not respect the `ty`
    /// signature. For example if the function type declares that it returns one
    /// i32 but the `func` closures does not write anything into the results
    /// slice then a trap may be generated.
    ///
    /// Additionally note that this is quite a dynamic function since signatures
    /// are not statically known. For a more performant and ergonomic `Func`
    /// it's recommended to use [`Func::wrap`] if you can because with
    /// statically known signatures Wasmtime can optimize the implementation
    /// much more.
    ///
    /// For more information about `Send + Sync + 'static` requirements on the
    /// `func`, see [`Func::wrap`](#why-send--sync--static).
    ///
    /// # Errors
    ///
    /// The host-provided function here returns a
    /// [`Result<()>`](anyhow::Result). If the function returns `Ok(())` then
    /// that indicates that the host function completed successfully and wrote
    /// the result into the `&mut [Val]` argument.
    ///
    /// If the function returns `Err(e)`, however, then this is equivalent to
    /// the host function triggering a trap for wasm. WebAssembly execution is
    /// immediately halted and the original caller of [`Func::call`], for
    /// example, will receive the error returned here (possibly with
    /// [`WasmBacktrace`](crate::WasmBacktrace) context information attached).
    ///
    /// For more information about errors in Wasmtime see the [`Trap`]
    /// documentation.
    ///
    /// [`Trap`]: crate::Trap
    ///
    /// # Panics
    ///
    /// Panics if the given function type is not associated with this store's
    /// engine.
    pub fn new<T: 'static>(
        store: impl AsContextMut<Data = T>,
        ty: FuncType,
        func: impl Fn(Caller<'_, T>, &[Val], &mut [Val]) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        assert!(ty.comes_from_same_engine(store.as_context().engine()));
        let ty_clone = ty.clone();
        unsafe {
            Func::new_unchecked(store, ty, move |caller, values| {
                Func::invoke_host_func_for_wasm(caller, &ty_clone, values, &func)
            })
        }
    }

    /// Creates a new [`Func`] with the given arguments, although has fewer
    /// runtime checks than [`Func::new`].
    ///
    /// This function takes a callback of a different signature than
    /// [`Func::new`], instead receiving a raw pointer with a list of [`ValRaw`]
    /// structures. These values have no type information associated with them
    /// so it's up to the caller to provide a function that will correctly
    /// interpret the list of values as those coming from the `ty` specified.
    ///
    /// If you're calling this from Rust it's recommended to either instead use
    /// [`Func::new`] or [`Func::wrap`]. The [`Func::wrap`] API, in particular,
    /// is both safer and faster than this API.
    ///
    /// # Errors
    ///
    /// See [`Func::new`] for the behavior of returning an error from the host
    /// function provided here.
    ///
    /// # Unsafety
    ///
    /// This function is not safe because it's not known at compile time that
    /// the `func` provided correctly interprets the argument types provided to
    /// it, or that the results it produces will be of the correct type.
    ///
    /// # Panics
    ///
    /// Panics if the given function type is not associated with this store's
    /// engine.
    pub unsafe fn new_unchecked<T: 'static>(
        mut store: impl AsContextMut<Data = T>,
        ty: FuncType,
        func: impl Fn(Caller<'_, T>, &mut [ValRaw]) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        assert!(ty.comes_from_same_engine(store.as_context().engine()));
        let store = store.as_context_mut().0;

        // SAFETY: the contract required by `new_unchecked` is the same as the
        // contract required by this function itself.
        let host = unsafe { HostFunc::new_unchecked(store.engine(), ty, func) };

        // SAFETY: the `T` used by `func` matches the `T` of the store we're
        // inserting into via this function's type signature.
        unsafe { host.into_func(store) }
    }

    /// Creates a new host-defined WebAssembly function which, when called,
    /// will run the asynchronous computation defined by `func` to completion
    /// and then return the result to WebAssembly.
    ///
    /// This function is the asynchronous analogue of [`Func::new`] and much of
    /// that documentation applies to this as well. The key difference is that
    /// `func` returns a future instead of simply a `Result`. Note that the
    /// returned future can close over any of the arguments, but it cannot close
    /// over the state of the closure itself. It's recommended to store any
    /// necessary async state in the `T` of the [`Store<T>`](crate::Store) which
    /// can be accessed through [`Caller::data`] or [`Caller::data_mut`].
    ///
    /// For more information on `Send + Sync + 'static`, see
    /// [`Func::wrap`](#why-send--sync--static).
    ///
    /// # Panics
    ///
    /// This function will panic if `store` is not associated with an [async
    /// config](crate::Config::async_support).
    ///
    /// Panics if the given function type is not associated with this store's
    /// engine.
    ///
    /// # Errors
    ///
    /// See [`Func::new`] for the behavior of returning an error from the host
    /// function provided here.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// // Simulate some application-specific state as well as asynchronous
    /// // functions to query that state.
    /// struct MyDatabase {
    ///     // ...
    /// }
    ///
    /// impl MyDatabase {
    ///     async fn get_row_count(&self) -> u32 {
    ///         // ...
    /// #       100
    ///     }
    /// }
    ///
    /// let my_database = MyDatabase {
    ///     // ...
    /// };
    ///
    /// // Using `new_async` we can hook up into calling our async
    /// // `get_row_count` function.
    /// let engine = Engine::new(Config::new().async_support(true))?;
    /// let mut store = Store::new(&engine, MyDatabase {
    ///     // ...
    /// });
    /// let get_row_count_type = wasmtime::FuncType::new(
    ///     &engine,
    ///     None,
    ///     Some(wasmtime::ValType::I32),
    /// );
    /// let get = Func::new_async(&mut store, get_row_count_type, |caller, _params, results| {
    ///     Box::new(async move {
    ///         let count = caller.data().get_row_count().await;
    ///         results[0] = Val::I32(count as i32);
    ///         Ok(())
    ///     })
    /// });
    /// // ...
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "async", feature = "cranelift"))]
    pub fn new_async<T, F>(store: impl AsContextMut<Data = T>, ty: FuncType, func: F) -> Func
    where
        F: for<'a> Fn(
                Caller<'a, T>,
                &'a [Val],
                &'a mut [Val],
            ) -> Box<dyn Future<Output = Result<()>> + Send + 'a>
            + Send
            + Sync
            + 'static,
        T: 'static,
    {
        assert!(
            store.as_context().async_support(),
            "cannot use `new_async` without enabling async support in the config"
        );
        assert!(ty.comes_from_same_engine(store.as_context().engine()));
        return Func::new(
            store,
            ty,
            move |Caller { store, caller }, params, results| {
                store.with_blocking(|store, cx| {
                    cx.block_on(core::pin::Pin::from(func(
                        Caller { store, caller },
                        params,
                        results,
                    )))
                })?
            },
        );
    }

    /// Creates a new `Func` from a store and a funcref within that store.
    ///
    /// # Safety
    ///
    /// The safety of this function requires that `func_ref` is a valid function
    /// pointer owned by `store`.
    pub(crate) unsafe fn from_vm_func_ref(store: StoreId, func_ref: NonNull<VMFuncRef>) -> Func {
        // SAFETY: given the contract of this function it's safe to read the
        // `type_index` field.
        unsafe {
            debug_assert!(func_ref.as_ref().type_index != VMSharedTypeIndex::default());
        }
        Func {
            store,
            unsafe_func_ref: func_ref.into(),
        }
    }

    /// Creates a new `Func` from the given Rust closure.
    ///
    /// This function will create a new `Func` which, when called, will
    /// execute the given Rust closure. Unlike [`Func::new`] the target
    /// function being called is known statically so the type signature can
    /// be inferred. Rust types will map to WebAssembly types as follows:
    ///
    /// | Rust Argument Type                | WebAssembly Type                          |
    /// |-----------------------------------|-------------------------------------------|
    /// | `i32`                             | `i32`                                     |
    /// | `u32`                             | `i32`                                     |
    /// | `i64`                             | `i64`                                     |
    /// | `u64`                             | `i64`                                     |
    /// | `f32`                             | `f32`                                     |
    /// | `f64`                             | `f64`                                     |
    /// | `V128` on x86-64 and aarch64 only | `v128`                                    |
    /// | `Option<Func>`                    | `funcref` aka `(ref null func)`           |
    /// | `Func`                            | `(ref func)`                              |
    /// | `Option<Nofunc>`                  | `nullfuncref` aka `(ref null nofunc)`     |
    /// | `NoFunc`                          | `(ref nofunc)`                            |
    /// | `Option<Rooted<ExternRef>>`       | `externref` aka `(ref null extern)`       |
    /// | `Rooted<ExternRef>`               | `(ref extern)`                            |
    /// | `Option<NoExtern>`                | `nullexternref` aka `(ref null noextern)` |
    /// | `NoExtern`                        | `(ref noextern)`                          |
    /// | `Option<Rooted<AnyRef>>`          | `anyref` aka `(ref null any)`             |
    /// | `Rooted<AnyRef>`                  | `(ref any)`                               |
    /// | `Option<Rooted<EqRef>>`           | `eqref` aka `(ref null eq)`               |
    /// | `Rooted<EqRef>`                   | `(ref eq)`                                |
    /// | `Option<I31>`                     | `i31ref` aka `(ref null i31)`             |
    /// | `I31`                             | `(ref i31)`                               |
    /// | `Option<Rooted<StructRef>>`       | `(ref null struct)`                       |
    /// | `Rooted<StructRef>`               | `(ref struct)`                            |
    /// | `Option<Rooted<ArrayRef>>`        | `(ref null array)`                        |
    /// | `Rooted<ArrayRef>`                | `(ref array)`                             |
    /// | `Option<NoneRef>`                 | `nullref` aka `(ref null none)`           |
    /// | `NoneRef`                         | `(ref none)`                              |
    ///
    /// Note that anywhere a `Rooted<T>` appears, a `ManuallyRooted<T>` may also
    /// be used.
    ///
    /// Any of the Rust types can be returned from the closure as well, in
    /// addition to some extra types
    ///
    /// | Rust Return Type  | WebAssembly Return Type | Meaning               |
    /// |-------------------|-------------------------|-----------------------|
    /// | `()`              | nothing                 | no return value       |
    /// | `T`               | `T`                     | a single return value |
    /// | `(T1, T2, ...)`   | `T1 T2 ...`             | multiple returns      |
    ///
    /// Note that all return types can also be wrapped in `Result<_>` to
    /// indicate that the host function can generate a trap as well as possibly
    /// returning a value.
    ///
    /// Finally you can also optionally take [`Caller`] as the first argument of
    /// your closure. If inserted then you're able to inspect the caller's
    /// state, for example the [`Memory`](crate::Memory) it has exported so you
    /// can read what pointers point to.
    ///
    /// Note that when using this API, the intention is to create as thin of a
    /// layer as possible for when WebAssembly calls the function provided. With
    /// sufficient inlining and optimization the WebAssembly will call straight
    /// into `func` provided, with no extra fluff entailed.
    ///
    /// # Why `Send + Sync + 'static`?
    ///
    /// All host functions defined in a [`Store`](crate::Store) (including
    /// those from [`Func::new`] and other constructors) require that the
    /// `func` provided is `Send + Sync + 'static`. Additionally host functions
    /// always are `Fn` as opposed to `FnMut` or `FnOnce`. This can at-a-glance
    /// feel restrictive since the closure cannot close over as many types as
    /// before. The reason for this, though, is to ensure that
    /// [`Store<T>`](crate::Store) can implement both the `Send` and `Sync`
    /// traits.
    ///
    /// Fear not, however, because this isn't as restrictive as it seems! Host
    /// functions are provided a [`Caller<'_, T>`](crate::Caller) argument which
    /// allows access to the host-defined data within the
    /// [`Store`](crate::Store). The `T` type is not required to be any of
    /// `Send`, `Sync`, or `'static`! This means that you can store whatever
    /// you'd like in `T` and have it accessible by all host functions.
    /// Additionally mutable access to `T` is allowed through
    /// [`Caller::data_mut`].
    ///
    /// Most host-defined [`Func`] values provide closures that end up not
    /// actually closing over any values. These zero-sized types will use the
    /// context from [`Caller`] for host-defined information.
    ///
    /// # Errors
    ///
    /// The closure provided here to `wrap` can optionally return a
    /// [`Result<T>`](anyhow::Result). Returning `Ok(t)` represents the host
    /// function successfully completing with the `t` result. Returning
    /// `Err(e)`, however, is equivalent to raising a custom wasm trap.
    /// Execution of WebAssembly does not resume and the stack is unwound to the
    /// original caller of the function where the error is returned.
    ///
    /// For more information about errors in Wasmtime see the [`Trap`]
    /// documentation.
    ///
    /// [`Trap`]: crate::Trap
    ///
    /// # Examples
    ///
    /// First up we can see how simple wasm imports can be implemented, such
    /// as a function that adds its two arguments and returns the result.
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut store = Store::<()>::default();
    /// let add = Func::wrap(&mut store, |a: i32, b: i32| a + b);
    /// let module = Module::new(
    ///     store.engine(),
    ///     r#"
    ///         (module
    ///             (import "" "" (func $add (param i32 i32) (result i32)))
    ///             (func (export "foo") (param i32 i32) (result i32)
    ///                 local.get 0
    ///                 local.get 1
    ///                 call $add))
    ///     "#,
    /// )?;
    /// let instance = Instance::new(&mut store, &module, &[add.into()])?;
    /// let foo = instance.get_typed_func::<(i32, i32), i32>(&mut store, "foo")?;
    /// assert_eq!(foo.call(&mut store, (1, 2))?, 3);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// We can also do the same thing, but generate a trap if the addition
    /// overflows:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut store = Store::<()>::default();
    /// let add = Func::wrap(&mut store, |a: i32, b: i32| {
    ///     match a.checked_add(b) {
    ///         Some(i) => Ok(i),
    ///         None => anyhow::bail!("overflow"),
    ///     }
    /// });
    /// let module = Module::new(
    ///     store.engine(),
    ///     r#"
    ///         (module
    ///             (import "" "" (func $add (param i32 i32) (result i32)))
    ///             (func (export "foo") (param i32 i32) (result i32)
    ///                 local.get 0
    ///                 local.get 1
    ///                 call $add))
    ///     "#,
    /// )?;
    /// let instance = Instance::new(&mut store, &module, &[add.into()])?;
    /// let foo = instance.get_typed_func::<(i32, i32), i32>(&mut store, "foo")?;
    /// assert_eq!(foo.call(&mut store, (1, 2))?, 3);
    /// assert!(foo.call(&mut store, (i32::max_value(), 1)).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// And don't forget all the wasm types are supported!
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut store = Store::<()>::default();
    /// let debug = Func::wrap(&mut store, |a: i32, b: u32, c: f32, d: i64, e: u64, f: f64| {
    ///
    ///     println!("a={}", a);
    ///     println!("b={}", b);
    ///     println!("c={}", c);
    ///     println!("d={}", d);
    ///     println!("e={}", e);
    ///     println!("f={}", f);
    /// });
    /// let module = Module::new(
    ///     store.engine(),
    ///     r#"
    ///         (module
    ///             (import "" "" (func $debug (param i32 i32 f32 i64 i64 f64)))
    ///             (func (export "foo")
    ///                 i32.const -1
    ///                 i32.const 1
    ///                 f32.const 2
    ///                 i64.const -3
    ///                 i64.const 3
    ///                 f64.const 4
    ///                 call $debug))
    ///     "#,
    /// )?;
    /// let instance = Instance::new(&mut store, &module, &[debug.into()])?;
    /// let foo = instance.get_typed_func::<(), ()>(&mut store, "foo")?;
    /// foo.call(&mut store, ())?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Finally if you want to get really fancy you can also implement
    /// imports that read/write wasm module's memory
    ///
    /// ```
    /// use std::str;
    ///
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut store = Store::default();
    /// let log_str = Func::wrap(&mut store, |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
    ///     let mem = match caller.get_export("memory") {
    ///         Some(Extern::Memory(mem)) => mem,
    ///         _ => anyhow::bail!("failed to find host memory"),
    ///     };
    ///     let data = mem.data(&caller)
    ///         .get(ptr as u32 as usize..)
    ///         .and_then(|arr| arr.get(..len as u32 as usize));
    ///     let string = match data {
    ///         Some(data) => match str::from_utf8(data) {
    ///             Ok(s) => s,
    ///             Err(_) => anyhow::bail!("invalid utf-8"),
    ///         },
    ///         None => anyhow::bail!("pointer/length out of bounds"),
    ///     };
    ///     assert_eq!(string, "Hello, world!");
    ///     println!("{}", string);
    ///     Ok(())
    /// });
    /// let module = Module::new(
    ///     store.engine(),
    ///     r#"
    ///         (module
    ///             (import "" "" (func $log_str (param i32 i32)))
    ///             (func (export "foo")
    ///                 i32.const 4   ;; ptr
    ///                 i32.const 13  ;; len
    ///                 call $log_str)
    ///             (memory (export "memory") 1)
    ///             (data (i32.const 4) "Hello, world!"))
    ///     "#,
    /// )?;
    /// let instance = Instance::new(&mut store, &module, &[log_str.into()])?;
    /// let foo = instance.get_typed_func::<(), ()>(&mut store, "foo")?;
    /// foo.call(&mut store, ())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn wrap<T, Params, Results>(
        mut store: impl AsContextMut<Data = T>,
        func: impl IntoFunc<T, Params, Results>,
    ) -> Func
    where
        T: 'static,
    {
        let store = store.as_context_mut().0;
        let host = HostFunc::wrap(store.engine(), func);

        // SAFETY: The `T` the closure takes is the same as the `T` of the store
        // we're inserting into via the type signature above.
        unsafe { host.into_func(store) }
    }

    #[cfg(feature = "async")]
    fn wrap_inner<F, T, Params, Results>(mut store: impl AsContextMut<Data = T>, func: F) -> Func
    where
        F: Fn(Caller<'_, T>, Params) -> Results + Send + Sync + 'static,
        Params: WasmTyList,
        Results: WasmRet,
        T: 'static,
    {
        let store = store.as_context_mut().0;
        let host = HostFunc::wrap_inner(store.engine(), func);

        // SAFETY: The `T` the closure takes is the same as the `T` of the store
        // we're inserting into via the type signature above.
        unsafe { host.into_func(store) }
    }

    /// Same as [`Func::wrap`], except the closure asynchronously produces the
    /// result and the arguments are passed within a tuple. For more information
    /// see the [`Func`] documentation.
    ///
    /// # Panics
    ///
    /// This function will panic if called with a non-asynchronous store.
    #[cfg(feature = "async")]
    pub fn wrap_async<T, F, P, R>(store: impl AsContextMut<Data = T>, func: F) -> Func
    where
        F: for<'a> Fn(Caller<'a, T>, P) -> Box<dyn Future<Output = R> + Send + 'a>
            + Send
            + Sync
            + 'static,
        P: WasmTyList,
        R: WasmRet,
        T: 'static,
    {
        assert!(
            store.as_context().async_support(),
            concat!("cannot use `wrap_async` without enabling async support on the config")
        );
        Func::wrap_inner(store, move |Caller { store, caller }, args| {
            match store.block_on(|store| func(Caller { store, caller }, args).into()) {
                Ok(ret) => ret.into_fallible(),
                Err(e) => R::fallible_from_error(e),
            }
        })
    }

    /// Returns the underlying wasm type that this `Func` has.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this function.
    pub fn ty(&self, store: impl AsContext) -> FuncType {
        self.load_ty(&store.as_context().0)
    }

    /// Forcibly loads the type of this function from the `Engine`.
    ///
    /// Note that this is a somewhat expensive method since it requires taking a
    /// lock as well as cloning a type.
    pub(crate) fn load_ty(&self, store: &StoreOpaque) -> FuncType {
        FuncType::from_shared_type_index(store.engine(), self.type_index(store))
    }

    /// Does this function match the given type?
    ///
    /// That is, is this function's type a subtype of the given type?
    ///
    /// # Panics
    ///
    /// Panics if this function is not associated with the given store or if the
    /// function type is not associated with the store's engine.
    pub fn matches_ty(&self, store: impl AsContext, func_ty: &FuncType) -> bool {
        self._matches_ty(store.as_context().0, func_ty)
    }

    pub(crate) fn _matches_ty(&self, store: &StoreOpaque, func_ty: &FuncType) -> bool {
        let actual_ty = self.load_ty(store);
        actual_ty.matches(func_ty)
    }

    pub(crate) fn ensure_matches_ty(&self, store: &StoreOpaque, func_ty: &FuncType) -> Result<()> {
        if !self.comes_from_same_store(store) {
            bail!("function used with wrong store");
        }
        if self._matches_ty(store, func_ty) {
            Ok(())
        } else {
            let actual_ty = self.load_ty(store);
            bail!("type mismatch: expected {func_ty}, found {actual_ty}")
        }
    }

    pub(crate) fn type_index(&self, data: &StoreOpaque) -> VMSharedTypeIndex {
        unsafe { self.vm_func_ref(data).as_ref().type_index }
    }

    /// Invokes this function with the `params` given and writes returned values
    /// to `results`.
    ///
    /// The `params` here must match the type signature of this `Func`, or an
    /// error will occur. Additionally `results` must have the same
    /// length as the number of results for this function. Calling this function
    /// will synchronously execute the WebAssembly function referenced to get
    /// the results.
    ///
    /// This function will return `Ok(())` if execution completed without a trap
    /// or error of any kind. In this situation the results will be written to
    /// the provided `results` array.
    ///
    /// # Errors
    ///
    /// Any error which occurs throughout the execution of the function will be
    /// returned as `Err(e)`. The [`Error`](anyhow::Error) type can be inspected
    /// for the precise error cause such as:
    ///
    /// * [`Trap`] - indicates that a wasm trap happened and execution was
    ///   halted.
    /// * [`WasmBacktrace`] - optionally included on errors for backtrace
    ///   information of the trap/error.
    /// * Other string-based errors to indicate issues such as type errors with
    ///   `params`.
    /// * Any host-originating error originally returned from a function defined
    ///   via [`Func::new`], for example.
    ///
    /// Errors typically indicate that execution of WebAssembly was halted
    /// mid-way and did not complete after the error condition happened.
    ///
    /// [`Trap`]: crate::Trap
    ///
    /// # Panics
    ///
    /// This function will panic if called on a function belonging to an async
    /// store. Asynchronous stores must always use `call_async`. Also panics if
    /// `store` does not own this function.
    ///
    /// [`WasmBacktrace`]: crate::WasmBacktrace
    pub fn call(
        &self,
        mut store: impl AsContextMut,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        assert!(
            !store.as_context().async_support(),
            "must use `call_async` when async support is enabled on the config",
        );
        let mut store = store.as_context_mut();

        self.call_impl_check_args(&mut store, params, results)?;

        unsafe { self.call_impl_do_call(&mut store, params, results) }
    }

    /// Invokes this function in an "unchecked" fashion, reading parameters and
    /// writing results to `params_and_returns`.
    ///
    /// This function is the same as [`Func::call`] except that the arguments
    /// and results both use a different representation. If possible it's
    /// recommended to use [`Func::call`] if safety isn't necessary or to use
    /// [`Func::typed`] in conjunction with [`TypedFunc::call`] since that's
    /// both safer and faster than this method of invoking a function.
    ///
    /// Note that if this function takes `externref` arguments then it will
    /// **not** automatically GC unlike the [`Func::call`] and
    /// [`TypedFunc::call`] functions. This means that if this function is
    /// invoked many times with new `ExternRef` values and no other GC happens
    /// via any other means then no values will get collected.
    ///
    /// # Errors
    ///
    /// For more information about errors see the [`Func::call`] documentation.
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because the `params_and_returns` argument is not
    /// validated at all. It must uphold invariants such as:
    ///
    /// * It's a valid pointer to an array
    /// * It has enough space to store all parameters
    /// * It has enough space to store all results (not at the same time as
    ///   parameters)
    /// * Parameters are initially written to the array and have the correct
    ///   types and such.
    /// * Reference types like `externref` and `funcref` are valid at the
    ///   time of this call and for the `store` specified.
    ///
    /// These invariants are all upheld for you with [`Func::call`] and
    /// [`TypedFunc::call`].
    pub unsafe fn call_unchecked(
        &self,
        mut store: impl AsContextMut,
        params_and_returns: *mut [ValRaw],
    ) -> Result<()> {
        let mut store = store.as_context_mut();
        let func_ref = self.vm_func_ref(store.0);
        let params_and_returns = NonNull::new(params_and_returns).unwrap_or(NonNull::from(&mut []));

        // SAFETY: the safety of this function call is the same as the contract
        // of this function.
        unsafe { Self::call_unchecked_raw(&mut store, func_ref, params_and_returns) }
    }

    pub(crate) unsafe fn call_unchecked_raw<T>(
        store: &mut StoreContextMut<'_, T>,
        func_ref: NonNull<VMFuncRef>,
        params_and_returns: NonNull<[ValRaw]>,
    ) -> Result<()> {
        // SAFETY: the safety of this function call is the same as the contract
        // of this function.
        invoke_wasm_and_catch_traps(store, |caller, vm| unsafe {
            VMFuncRef::array_call(func_ref, vm, caller, params_and_returns)
        })
    }

    /// Converts the raw representation of a `funcref` into an `Option<Func>`
    ///
    /// This is intended to be used in conjunction with [`Func::new_unchecked`],
    /// [`Func::call_unchecked`], and [`ValRaw`] with its `funcref` field. This
    /// is the dual of [`Func::to_raw`].
    ///
    /// # Unsafety
    ///
    /// This function is not safe because `raw` is not validated at all. The
    /// caller must guarantee that `raw` is owned by the `store` provided and is
    /// valid within the `store`.
    pub unsafe fn from_raw(mut store: impl AsContextMut, raw: *mut c_void) -> Option<Func> {
        // SAFETY: this function's own contract is that `raw` is owned by store
        // to make this safe.
        unsafe { Self::_from_raw(store.as_context_mut().0, raw) }
    }

    /// Same as `from_raw`, but with the internal `StoreOpaque` type.
    pub(crate) unsafe fn _from_raw(store: &mut StoreOpaque, raw: *mut c_void) -> Option<Func> {
        // SAFETY: this function's own contract is that `raw` is owned by store
        // to make this safe.
        unsafe {
            Some(Func::from_vm_func_ref(
                store.id(),
                NonNull::new(raw.cast())?,
            ))
        }
    }

    /// Extracts the raw value of this `Func`, which is owned by `store`.
    ///
    /// This function returns a value that's suitable for writing into the
    /// `funcref` field of the [`ValRaw`] structure.
    ///
    /// # Safety
    ///
    /// The returned value is only valid for as long as the store is alive.
    /// This value is safe to pass to [`Func::from_raw`] so long as the same
    /// `store` is provided.
    pub fn to_raw(&self, mut store: impl AsContextMut) -> *mut c_void {
        self.vm_func_ref(store.as_context_mut().0).as_ptr().cast()
    }

    /// Invokes this function with the `params` given, returning the results
    /// asynchronously.
    ///
    /// This function is the same as [`Func::call`] except that it is
    /// asynchronous. This is only compatible with stores associated with an
    /// [asynchronous config](crate::Config::async_support).
    ///
    /// It's important to note that the execution of WebAssembly will happen
    /// synchronously in the `poll` method of the future returned from this
    /// function. Wasmtime does not manage its own thread pool or similar to
    /// execute WebAssembly in. Future `poll` methods are generally expected to
    /// resolve quickly, so it's recommended that you run or poll this future
    /// in a "blocking context".
    ///
    /// For more information see the documentation on [asynchronous
    /// configs](crate::Config::async_support).
    ///
    /// # Errors
    ///
    /// For more information on errors see the [`Func::call`] documentation.
    ///
    /// # Panics
    ///
    /// Panics if this is called on a function in a synchronous store. This
    /// only works with functions defined within an asynchronous store. Also
    /// panics if `store` does not own this function.
    #[cfg(feature = "async")]
    pub async fn call_async(
        &self,
        mut store: impl AsContextMut<Data: Send>,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        let mut store = store.as_context_mut();
        assert!(
            store.0.async_support(),
            "cannot use `call_async` without enabling async support in the config",
        );

        self.call_impl_check_args(&mut store, params, results)?;

        let result = store
            .on_fiber(|store| unsafe { self.call_impl_do_call(store, params, results) })
            .await??;
        Ok(result)
    }

    /// Perform dynamic checks that the arguments given to us match
    /// the signature of this function and are appropriate to pass to this
    /// function.
    ///
    /// This involves checking to make sure we have the right number and types
    /// of arguments as well as making sure everything is from the same `Store`.
    ///
    /// This must be called just before `call_impl_do_call`.
    fn call_impl_check_args<T>(
        &self,
        store: &mut StoreContextMut<'_, T>,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        let ty = self.load_ty(store.0);
        if ty.params().len() != params.len() {
            bail!(
                "expected {} arguments, got {}",
                ty.params().len(),
                params.len()
            );
        }
        if ty.results().len() != results.len() {
            bail!(
                "expected {} results, got {}",
                ty.results().len(),
                results.len()
            );
        }

        for (ty, arg) in ty.params().zip(params) {
            arg.ensure_matches_ty(store.0, &ty)
                .context("argument type mismatch")?;
            if !arg.comes_from_same_store(store.0) {
                bail!("cross-`Store` values are not currently supported");
            }
        }

        Ok(())
    }

    /// Do the actual call into Wasm.
    ///
    /// # Safety
    ///
    /// You must have type checked the arguments by calling
    /// `call_impl_check_args` immediately before calling this function. It is
    /// only safe to call this function if that one did not return an error.
    unsafe fn call_impl_do_call<T>(
        &self,
        store: &mut StoreContextMut<'_, T>,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        // Store the argument values into `values_vec`.
        let ty = self.load_ty(store.0);
        let values_vec_size = params.len().max(ty.results().len());
        let mut values_vec = store.0.take_wasm_val_raw_storage();
        debug_assert!(values_vec.is_empty());
        values_vec.resize_with(values_vec_size, || ValRaw::v128(0));
        for (arg, slot) in params.iter().cloned().zip(&mut values_vec) {
            *slot = arg.to_raw(&mut *store)?;
        }

        unsafe {
            self.call_unchecked(
                &mut *store,
                core::ptr::slice_from_raw_parts_mut(values_vec.as_mut_ptr(), values_vec_size),
            )?;
        }

        for ((i, slot), val) in results.iter_mut().enumerate().zip(&values_vec) {
            let ty = ty.results().nth(i).unwrap();
            *slot = unsafe { Val::from_raw(&mut *store, *val, ty) };
        }
        values_vec.truncate(0);
        store.0.save_wasm_val_raw_storage(values_vec);
        Ok(())
    }

    #[inline]
    pub(crate) fn vm_func_ref(&self, store: &StoreOpaque) -> NonNull<VMFuncRef> {
        self.store.assert_belongs_to(store.id());
        self.unsafe_func_ref.as_non_null()
    }

    pub(crate) fn vmimport(&self, store: &StoreOpaque) -> VMFunctionImport {
        unsafe {
            let f = self.vm_func_ref(store);
            VMFunctionImport {
                // Note that this is a load-bearing `unwrap` here, but is
                // never expected to trip at runtime. The general problem is
                // that host functions do not have a `wasm_call` function so
                // the `VMFuncRef` type has an optional pointer there. This is
                // only able to be filled out when a function is "paired" with
                // a module where trampolines are present to fill out
                // `wasm_call` pointers.
                //
                // This pairing of modules doesn't happen explicitly but is
                // instead managed lazily throughout Wasmtime. Specifically the
                // way this works is one of:
                //
                // * When a host function is created the store's list of
                //   modules are searched for a wasm trampoline. If not found
                //   the `wasm_call` field is left blank.
                //
                // * When a module instantiation happens, which uses this
                //   function, the module will be used to fill any outstanding
                //   holes that it has trampolines for.
                //
                // This means that by the time we get to this point any
                // relevant holes should be filled out. Thus if this panic
                // actually triggers then it's indicative of a missing `fill`
                // call somewhere else.
                wasm_call: f.as_ref().wasm_call.unwrap(),
                array_call: f.as_ref().array_call,
                vmctx: f.as_ref().vmctx,
            }
        }
    }

    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        self.store == store.id()
    }

    fn invoke_host_func_for_wasm<T>(
        mut caller: Caller<'_, T>,
        ty: &FuncType,
        values_vec: &mut [ValRaw],
        func: &dyn Fn(Caller<'_, T>, &[Val], &mut [Val]) -> Result<()>,
    ) -> Result<()> {
        // Translate the raw JIT arguments in `values_vec` into a `Val` which
        // we'll be passing as a slice. The storage for our slice-of-`Val` we'll
        // be taking from the `Store`. We preserve our slice back into the
        // `Store` after the hostcall, ideally amortizing the cost of allocating
        // the storage across wasm->host calls.
        //
        // Note that we have a dynamic guarantee that `values_vec` is the
        // appropriate length to both read all arguments from as well as store
        // all results into.
        let mut val_vec = caller.store.0.take_hostcall_val_storage();
        debug_assert!(val_vec.is_empty());
        let nparams = ty.params().len();
        val_vec.reserve(nparams + ty.results().len());
        for (i, ty) in ty.params().enumerate() {
            val_vec.push(unsafe { Val::from_raw(&mut caller.store, values_vec[i], ty) })
        }

        val_vec.extend((0..ty.results().len()).map(|_| Val::null_func_ref()));
        let (params, results) = val_vec.split_at_mut(nparams);
        func(caller.sub_caller(), params, results)?;

        // Unlike our arguments we need to dynamically check that the return
        // values produced are correct. There could be a bug in `func` that
        // produces the wrong number, wrong types, or wrong stores of
        // values, and we need to catch that here.
        for (i, (ret, ty)) in results.iter().zip(ty.results()).enumerate() {
            ret.ensure_matches_ty(caller.store.0, &ty)
                .context("function attempted to return an incompatible value")?;
            values_vec[i] = ret.to_raw(&mut caller.store)?;
        }

        // Restore our `val_vec` back into the store so it's usable for the next
        // hostcall to reuse our own storage.
        val_vec.truncate(0);
        caller.store.0.save_hostcall_val_storage(val_vec);
        Ok(())
    }

    /// Attempts to extract a typed object from this `Func` through which the
    /// function can be called.
    ///
    /// This function serves as an alternative to [`Func::call`] and
    /// [`Func::call_async`]. This method performs a static type check (using
    /// the `Params` and `Results` type parameters on the underlying wasm
    /// function. If the type check passes then a `TypedFunc` object is returned,
    /// otherwise an error is returned describing the typecheck failure.
    ///
    /// The purpose of this relative to [`Func::call`] is that it's much more
    /// efficient when used to invoke WebAssembly functions. With the types
    /// statically known far less setup/teardown is required when invoking
    /// WebAssembly. If speed is desired then this function is recommended to be
    /// used instead of [`Func::call`] (which is more general, hence its
    /// slowdown).
    ///
    /// The `Params` type parameter is used to describe the parameters of the
    /// WebAssembly function. This can either be a single type (like `i32`), or
    /// a tuple of types representing the list of parameters (like `(i32, f32,
    /// f64)`). Additionally you can use `()` to represent that the function has
    /// no parameters.
    ///
    /// The `Results` type parameter is used to describe the results of the
    /// function. This behaves the same way as `Params`, but just for the
    /// results of the function.
    ///
    /// # Translating Between WebAssembly and Rust Types
    ///
    /// Translation between Rust types and WebAssembly types looks like:
    ///
    /// | WebAssembly                               | Rust                                  |
    /// |-------------------------------------------|---------------------------------------|
    /// | `i32`                                     | `i32` or `u32`                        |
    /// | `i64`                                     | `i64` or `u64`                        |
    /// | `f32`                                     | `f32`                                 |
    /// | `f64`                                     | `f64`                                 |
    /// | `externref` aka `(ref null extern)`       | `Option<Rooted<ExternRef>>`           |
    /// | `(ref extern)`                            | `Rooted<ExternRef>`                   |
    /// | `nullexternref` aka `(ref null noextern)` | `Option<NoExtern>`                    |
    /// | `(ref noextern)`                          | `NoExtern`                            |
    /// | `anyref` aka `(ref null any)`             | `Option<Rooted<AnyRef>>`              |
    /// | `(ref any)`                               | `Rooted<AnyRef>`                      |
    /// | `eqref` aka `(ref null eq)`               | `Option<Rooted<EqRef>>`               |
    /// | `(ref eq)`                                | `Rooted<EqRef>`                       |
    /// | `i31ref` aka `(ref null i31)`             | `Option<I31>`                         |
    /// | `(ref i31)`                               | `I31`                                 |
    /// | `structref` aka `(ref null struct)`       | `Option<Rooted<StructRef>>`           |
    /// | `(ref struct)`                            | `Rooted<StructRef>`                   |
    /// | `arrayref` aka `(ref null array)`         | `Option<Rooted<ArrayRef>>`            |
    /// | `(ref array)`                             | `Rooted<ArrayRef>`                    |
    /// | `nullref` aka `(ref null none)`           | `Option<NoneRef>`                     |
    /// | `(ref none)`                              | `NoneRef`                             |
    /// | `funcref` aka `(ref null func)`           | `Option<Func>`                        |
    /// | `(ref func)`                              | `Func`                                |
    /// | `(ref null <func type index>)`            | `Option<Func>`                        |
    /// | `(ref <func type index>)`                 | `Func`                                |
    /// | `nullfuncref` aka `(ref null nofunc)`     | `Option<NoFunc>`                      |
    /// | `(ref nofunc)`                            | `NoFunc`                              |
    /// | `v128`                                    | `V128` on `x86-64` and `aarch64` only |
    ///
    /// (Note that this mapping is the same as that of [`Func::wrap`], and that
    /// anywhere a `Rooted<T>` appears, a `ManuallyRooted<T>` may also appear).
    ///
    /// Note that once the [`TypedFunc`] return value is acquired you'll use either
    /// [`TypedFunc::call`] or [`TypedFunc::call_async`] as necessary to actually invoke
    /// the function. This method does not invoke any WebAssembly code, it
    /// simply performs a typecheck before returning the [`TypedFunc`] value.
    ///
    /// This method also has a convenience wrapper as
    /// [`Instance::get_typed_func`](crate::Instance::get_typed_func) to
    /// directly get a typed function value from an
    /// [`Instance`](crate::Instance).
    ///
    /// ## Subtyping
    ///
    /// For result types, you can always use a supertype of the WebAssembly
    /// function's actual declared result type. For example, if the WebAssembly
    /// function was declared with type `(func (result nullfuncref))` you could
    /// successfully call `f.typed::<(), Option<Func>>()` because `Option<Func>`
    /// corresponds to `funcref`, which is a supertype of `nullfuncref`.
    ///
    /// For parameter types, you can always use a subtype of the WebAssembly
    /// function's actual declared parameter type. For example, if the
    /// WebAssembly function was declared with type `(func (param (ref null
    /// func)))` you could successfully call `f.typed::<Func, ()>()` because
    /// `Func` corresponds to `(ref func)`, which is a subtype of `(ref null
    /// func)`.
    ///
    /// Additionally, for functions which take a reference to a concrete type as
    /// a parameter, you can also use the concrete type's supertype. Consider a
    /// WebAssembly function that takes a reference to a function with a
    /// concrete type: `(ref null <func type index>)`. In this scenario, there
    /// is no static `wasmtime::Foo` Rust type that corresponds to that
    /// particular Wasm-defined concrete reference type because Wasm modules are
    /// loaded dynamically at runtime. You *could* do `f.typed::<Option<NoFunc>,
    /// ()>()`, and while that is correctly typed and valid, it is often overly
    /// restrictive. The only value you could call the resulting typed function
    /// with is the null function reference, but we'd like to call it with
    /// non-null function references that happen to be of the correct
    /// type. Therefore, `f.typed<Option<Func>, ()>()` is also allowed in this
    /// case, even though `Option<Func>` represents `(ref null func)` which is
    /// the supertype, not subtype, of `(ref null <func type index>)`. This does
    /// imply some minimal dynamic type checks in this case, but it is supported
    /// for better ergonomics, to enable passing non-null references into the
    /// function.
    ///
    /// # Errors
    ///
    /// This function will return an error if `Params` or `Results` does not
    /// match the native type of this WebAssembly function.
    ///
    /// # Panics
    ///
    /// This method will panic if `store` does not own this function.
    ///
    /// # Examples
    ///
    /// An end-to-end example of calling a function which takes no parameters
    /// and has no results:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = Engine::default();
    /// let mut store = Store::new(&engine, ());
    /// let module = Module::new(&engine, r#"(module (func (export "foo")))"#)?;
    /// let instance = Instance::new(&mut store, &module, &[])?;
    /// let foo = instance.get_func(&mut store, "foo").expect("export wasn't a function");
    ///
    /// // Note that this call can fail due to the typecheck not passing, but
    /// // in our case we statically know the module so we know this should
    /// // pass.
    /// let typed = foo.typed::<(), ()>(&store)?;
    ///
    /// // Note that this can fail if the wasm traps at runtime.
    /// typed.call(&mut store, ())?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// You can also pass in multiple parameters and get a result back
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn foo(add: &Func, mut store: Store<()>) -> anyhow::Result<()> {
    /// let typed = add.typed::<(i32, i64), f32>(&store)?;
    /// assert_eq!(typed.call(&mut store, (1, 2))?, 3.0);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// and similarly if a function has multiple results you can bind that too
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn foo(add_with_overflow: &Func, mut store: Store<()>) -> anyhow::Result<()> {
    /// let typed = add_with_overflow.typed::<(u32, u32), (u32, i32)>(&store)?;
    /// let (result, overflow) = typed.call(&mut store, (u32::max_value(), 2))?;
    /// assert_eq!(result, 1);
    /// assert_eq!(overflow, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn typed<Params, Results>(
        &self,
        store: impl AsContext,
    ) -> Result<TypedFunc<Params, Results>>
    where
        Params: WasmParams,
        Results: WasmResults,
    {
        // Type-check that the params/results are all valid
        let store = store.as_context().0;
        let ty = self.load_ty(store);
        Params::typecheck(store.engine(), ty.params(), TypeCheckPosition::Param)
            .context("type mismatch with parameters")?;
        Results::typecheck(store.engine(), ty.results(), TypeCheckPosition::Result)
            .context("type mismatch with results")?;

        // and then we can construct the typed version of this function
        // (unsafely), which should be safe since we just did the type check above.
        unsafe { Ok(TypedFunc::_new_unchecked(store, *self)) }
    }

    /// Get a stable hash key for this function.
    ///
    /// Even if the same underlying function is added to the `StoreData`
    /// multiple times and becomes multiple `wasmtime::Func`s, this hash key
    /// will be consistent across all of these functions.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Not used yet, but added for consistency")
    )]
    pub(crate) fn hash_key(&self, store: &mut StoreOpaque) -> impl core::hash::Hash + Eq + use<> {
        self.vm_func_ref(store).as_ptr().addr()
    }
}

/// Prepares for entrance into WebAssembly.
///
/// This function will set up context such that `closure` is allowed to call a
/// raw trampoline or a raw WebAssembly function. This *must* be called to do
/// things like catch traps and set up GC properly.
///
/// The `closure` provided receives a default "caller" `VMContext` parameter it
/// can pass to the called wasm function, if desired.
pub(crate) fn invoke_wasm_and_catch_traps<T>(
    store: &mut StoreContextMut<'_, T>,
    closure: impl FnMut(NonNull<VMContext>, Option<InterpreterRef<'_>>) -> bool,
) -> Result<()> {
    unsafe {
        // The `enter_wasm` call below will reset the store context's
        // `stack_chain` to a new `InitialStack`, pointing to the
        // stack-allocated `initial_stack_csi`.
        let mut initial_stack_csi = VMCommonStackInformation::running_default();
        // Stores some state of the runtime just before entering Wasm. Will be
        // restored upon exiting Wasm. Note that the `CallThreadState` that is
        // created by the `catch_traps` call below will store a pointer to this
        // stack-allocated `previous_runtime_state`.
        let mut previous_runtime_state =
            EntryStoreContext::enter_wasm(store, &mut initial_stack_csi);

        if let Err(trap) = store.0.call_hook(CallHook::CallingWasm) {
            // `previous_runtime_state` implicitly dropped here
            return Err(trap);
        }
        let result = crate::runtime::vm::catch_traps(store, &mut previous_runtime_state, closure);
        core::mem::drop(previous_runtime_state);
        store.0.call_hook(CallHook::ReturningFromWasm)?;
        result.map_err(|t| crate::trap::from_runtime_box(store.0, t))
    }
}

/// This type helps managing the state of the runtime when entering and exiting
/// Wasm. To this end, it contains a subset of the data in `VMStoreContext`.
/// Upon entering Wasm, it updates various runtime fields and their
/// original values saved in this struct. Upon exiting Wasm, the previous values
/// are restored.
pub(crate) struct EntryStoreContext {
    /// If set, contains value of `stack_limit` field to restore in
    /// `VMStoreContext` when exiting Wasm.
    pub stack_limit: Option<usize>,
    /// Contains value of `last_wasm_exit_pc` field to restore in
    /// `VMStoreContext` when exiting Wasm.
    pub last_wasm_exit_pc: usize,
    /// Contains value of `last_wasm_exit_fp` field to restore in
    /// `VMStoreContext` when exiting Wasm.
    pub last_wasm_exit_fp: usize,
    /// Contains value of `last_wasm_entry_fp` field to restore in
    /// `VMStoreContext` when exiting Wasm.
    pub last_wasm_entry_fp: usize,
    /// Contains value of `stack_chain` field to restore in
    /// `VMStoreContext` when exiting Wasm.
    pub stack_chain: VMStackChain,

    /// We need a pointer to the runtime limits, so we can update them from
    /// `drop`/`exit_wasm`.
    vm_store_context: *const VMStoreContext,
}

impl EntryStoreContext {
    /// This function is called to update and save state when
    /// WebAssembly is entered within the `Store`.
    ///
    /// This updates various fields such as:
    ///
    /// * The stack limit. This is what ensures that we limit the stack space
    ///   allocated by WebAssembly code and it's relative to the initial stack
    ///   pointer that called into wasm.
    ///
    /// It also saves the different last_wasm_* values in the `VMStoreContext`.
    pub fn enter_wasm<T>(
        store: &mut StoreContextMut<'_, T>,
        initial_stack_information: *mut VMCommonStackInformation,
    ) -> Self {
        let stack_limit;

        // If this is a recursive call, e.g. our stack limit is already set, then
        // we may be able to skip this function.
        //
        // For synchronous stores there's nothing else to do because all wasm calls
        // happen synchronously and on the same stack. This means that the previous
        // stack limit will suffice for the next recursive call.
        //
        // For asynchronous stores then each call happens on a separate native
        // stack. This means that the previous stack limit is no longer relevant
        // because we're on a separate stack.
        if unsafe { *store.0.vm_store_context().stack_limit.get() } != usize::MAX
            && !store.0.async_support()
        {
            stack_limit = None;
        }
        // Ignore this stack pointer business on miri since we can't execute wasm
        // anyway and the concept of a stack pointer on miri is a bit nebulous
        // regardless.
        else if cfg!(miri) {
            stack_limit = None;
        } else {
            // When Cranelift has support for the host then we might be running native
            // compiled code meaning we need to read the actual stack pointer. If
            // Cranelift can't be used though then we're guaranteed to be running pulley
            // in which case this stack pointer isn't actually used as Pulley has custom
            // mechanisms for stack overflow.
            #[cfg(has_host_compiler_backend)]
            let stack_pointer = crate::runtime::vm::get_stack_pointer();
            #[cfg(not(has_host_compiler_backend))]
            let stack_pointer = {
                use wasmtime_environ::TripleExt;
                debug_assert!(store.engine().target().is_pulley());
                usize::MAX
            };

            // Determine the stack pointer where, after which, any wasm code will
            // immediately trap. This is checked on the entry to all wasm functions.
            //
            // Note that this isn't 100% precise. We are requested to give wasm
            // `max_wasm_stack` bytes, but what we're actually doing is giving wasm
            // probably a little less than `max_wasm_stack` because we're
            // calculating the limit relative to this function's approximate stack
            // pointer. Wasm will be executed on a frame beneath this one (or next
            // to it). In any case it's expected to be at most a few hundred bytes
            // of slop one way or another. When wasm is typically given a MB or so
            // (a million bytes) the slop shouldn't matter too much.
            //
            // After we've got the stack limit then we store it into the `stack_limit`
            // variable.
            let wasm_stack_limit = stack_pointer
                .checked_sub(store.engine().config().max_wasm_stack)
                .unwrap();
            let prev_stack = unsafe {
                mem::replace(
                    &mut *store.0.vm_store_context().stack_limit.get(),
                    wasm_stack_limit,
                )
            };
            stack_limit = Some(prev_stack);
        }

        unsafe {
            let last_wasm_exit_pc = *store.0.vm_store_context().last_wasm_exit_pc.get();
            let last_wasm_exit_fp = *store.0.vm_store_context().last_wasm_exit_fp.get();
            let last_wasm_entry_fp = *store.0.vm_store_context().last_wasm_entry_fp.get();

            let stack_chain = (*store.0.vm_store_context().stack_chain.get()).clone();

            let new_stack_chain = VMStackChain::InitialStack(initial_stack_information);
            *store.0.vm_store_context().stack_chain.get() = new_stack_chain;

            let vm_store_context = store.0.vm_store_context();

            Self {
                stack_limit,
                last_wasm_exit_pc,
                last_wasm_exit_fp,
                last_wasm_entry_fp,
                stack_chain,
                vm_store_context,
            }
        }
    }

    /// This function restores the values stored in this struct. We invoke this
    /// function through this type's `Drop` implementation. This ensures that we
    /// even restore the values if we unwind the stack (e.g., because we are
    /// panicking out of a Wasm execution).
    #[inline]
    fn exit_wasm(&mut self) {
        unsafe {
            if let Some(limit) = self.stack_limit {
                *(&*self.vm_store_context).stack_limit.get() = limit;
            }

            *(*self.vm_store_context).last_wasm_exit_fp.get() = self.last_wasm_exit_fp;
            *(*self.vm_store_context).last_wasm_exit_pc.get() = self.last_wasm_exit_pc;
            *(*self.vm_store_context).last_wasm_entry_fp.get() = self.last_wasm_entry_fp;
            *(*self.vm_store_context).stack_chain.get() = self.stack_chain.clone();
        }
    }
}

impl Drop for EntryStoreContext {
    #[inline]
    fn drop(&mut self) {
        self.exit_wasm();
    }
}

/// A trait implemented for types which can be returned from closures passed to
/// [`Func::wrap`] and friends.
///
/// This trait should not be implemented by user types. This trait may change at
/// any time internally. The types which implement this trait, however, are
/// stable over time.
///
/// For more information see [`Func::wrap`]
pub unsafe trait WasmRet {
    // Same as `WasmTy::compatible_with_store`.
    #[doc(hidden)]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool;

    /// Stores this return value into the `ptr` specified using the rooted
    /// `store`.
    ///
    /// Traps are communicated through the `Result<_>` return value.
    ///
    /// # Unsafety
    ///
    /// This method is unsafe as `ptr` must have the correct length to store
    /// this result. This property is only checked in debug mode, not in release
    /// mode.
    #[doc(hidden)]
    unsafe fn store(
        self,
        store: &mut AutoAssertNoGc<'_>,
        ptr: &mut [MaybeUninit<ValRaw>],
    ) -> Result<()>;

    #[doc(hidden)]
    fn func_type(engine: &Engine, params: impl Iterator<Item = ValType>) -> FuncType;
    #[doc(hidden)]
    fn may_gc() -> bool;

    // Utilities used to convert an instance of this type to a `Result`
    // explicitly, used when wrapping async functions which always bottom-out
    // in a function that returns a trap because futures can be cancelled.
    #[doc(hidden)]
    type Fallible: WasmRet;
    #[doc(hidden)]
    fn into_fallible(self) -> Self::Fallible;
    #[doc(hidden)]
    fn fallible_from_error(error: Error) -> Self::Fallible;
}

unsafe impl<T> WasmRet for T
where
    T: WasmTy,
{
    type Fallible = Result<T>;

    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        <Self as WasmTy>::compatible_with_store(self, store)
    }

    unsafe fn store(
        self,
        store: &mut AutoAssertNoGc<'_>,
        ptr: &mut [MaybeUninit<ValRaw>],
    ) -> Result<()> {
        debug_assert!(ptr.len() > 0);
        // SAFETY: the contract of this function/trait combo is such that `ptr`
        // is valid to store this type's value, thus this lookup should be safe.
        unsafe { <Self as WasmTy>::store(self, store, ptr.get_unchecked_mut(0)) }
    }

    fn may_gc() -> bool {
        T::may_gc()
    }

    fn func_type(engine: &Engine, params: impl Iterator<Item = ValType>) -> FuncType {
        FuncType::new(engine, params, Some(<Self as WasmTy>::valtype()))
    }

    fn into_fallible(self) -> Result<T> {
        Ok(self)
    }

    fn fallible_from_error(error: Error) -> Result<T> {
        Err(error)
    }
}

unsafe impl<T> WasmRet for Result<T>
where
    T: WasmRet,
{
    type Fallible = Self;

    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        match self {
            Ok(x) => <T as WasmRet>::compatible_with_store(x, store),
            Err(_) => true,
        }
    }

    unsafe fn store(
        self,
        store: &mut AutoAssertNoGc<'_>,
        ptr: &mut [MaybeUninit<ValRaw>],
    ) -> Result<()> {
        // SAFETY: the safety of calling this function is the same as calling
        // the inner `store`.
        unsafe { self.and_then(|val| val.store(store, ptr)) }
    }

    fn may_gc() -> bool {
        T::may_gc()
    }

    fn func_type(engine: &Engine, params: impl Iterator<Item = ValType>) -> FuncType {
        T::func_type(engine, params)
    }

    fn into_fallible(self) -> Result<T> {
        self
    }

    fn fallible_from_error(error: Error) -> Result<T> {
        Err(error)
    }
}

macro_rules! impl_wasm_host_results {
    ($n:tt $($t:ident)*) => (
        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($t),*> WasmRet for ($($t,)*)
        where
            $($t: WasmTy,)*
        {
            type Fallible = Result<Self>;

            #[inline]
            fn compatible_with_store(&self, _store: &StoreOpaque) -> bool {
                let ($($t,)*) = self;
                $( $t.compatible_with_store(_store) && )* true
            }

            #[inline]
            unsafe fn store(
                self,
                _store: &mut AutoAssertNoGc<'_>,
                _ptr: &mut [MaybeUninit<ValRaw>],
            ) -> Result<()> {
                let ($($t,)*) = self;
                let mut _cur = 0;
                $(
                    debug_assert!(_cur < _ptr.len());
                    // SAFETY: `store`'s unsafe contract is that `_ptr` is
                    // appropriately sized and additionally safe to call `store`
                    // for sub-types.
                    unsafe {
                        let val = _ptr.get_unchecked_mut(_cur);
                        _cur += 1;
                        WasmTy::store($t, _store, val)?;
                    }
                )*
                Ok(())
            }

            #[doc(hidden)]
            fn may_gc() -> bool {
                $( $t::may_gc() || )* false
            }

            fn func_type(engine: &Engine, params: impl Iterator<Item = ValType>) -> FuncType {
                FuncType::new(
                    engine,
                    params,
                    IntoIterator::into_iter([$($t::valtype(),)*]),
                )
            }

            #[inline]
            fn into_fallible(self) -> Result<Self> {
                Ok(self)
            }

            #[inline]
            fn fallible_from_error(error: Error) -> Result<Self> {
                Err(error)
            }
        }
    )
}

for_each_function_signature!(impl_wasm_host_results);

/// Internal trait implemented for all arguments that can be passed to
/// [`Func::wrap`] and [`Linker::func_wrap`](crate::Linker::func_wrap).
///
/// This trait should not be implemented by external users, it's only intended
/// as an implementation detail of this crate.
pub trait IntoFunc<T, Params, Results>: Send + Sync + 'static {
    /// Convert this function into a `VM{Array,Native}CallHostFuncContext` and
    /// internal `VMFuncRef`.
    #[doc(hidden)]
    fn into_func(self, engine: &Engine) -> HostContext;
}

macro_rules! impl_into_func {
    ($num:tt $arg:ident) => {
        // Implement for functions without a leading `&Caller` parameter,
        // delegating to the implementation below which does have the leading
        // `Caller` parameter.
        #[expect(non_snake_case, reason = "macro-generated code")]
        impl<T, F, $arg, R> IntoFunc<T, $arg, R> for F
        where
            F: Fn($arg) -> R + Send + Sync + 'static,
            $arg: WasmTy,
            R: WasmRet,
            T: 'static,
        {
            fn into_func(self, engine: &Engine) -> HostContext {
                let f = move |_: Caller<'_, T>, $arg: $arg| {
                    self($arg)
                };

                f.into_func(engine)
            }
        }

        #[expect(non_snake_case, reason = "macro-generated code")]
        impl<T, F, $arg, R> IntoFunc<T, (Caller<'_, T>, $arg), R> for F
        where
            F: Fn(Caller<'_, T>, $arg) -> R + Send + Sync + 'static,
            $arg: WasmTy,
            R: WasmRet,
            T: 'static,
        {
            fn into_func(self, engine: &Engine) -> HostContext {
                HostContext::from_closure(engine, move |caller: Caller<'_, T>, ($arg,)| {
                    self(caller, $arg)
                })
            }
        }
    };
    ($num:tt $($args:ident)*) => {
        // Implement for functions without a leading `&Caller` parameter,
        // delegating to the implementation below which does have the leading
        // `Caller` parameter.
        #[allow(non_snake_case, reason = "macro-generated code")]
        impl<T, F, $($args,)* R> IntoFunc<T, ($($args,)*), R> for F
        where
            F: Fn($($args),*) -> R + Send + Sync + 'static,
            $($args: WasmTy,)*
            R: WasmRet,
            T: 'static,
        {
            fn into_func(self, engine: &Engine) -> HostContext {
                let f = move |_: Caller<'_, T>, $($args:$args),*| {
                    self($($args),*)
                };

                f.into_func(engine)
            }
        }

        #[allow(non_snake_case, reason = "macro-generated code")]
        impl<T, F, $($args,)* R> IntoFunc<T, (Caller<'_, T>, $($args,)*), R> for F
        where
            F: Fn(Caller<'_, T>, $($args),*) -> R + Send + Sync + 'static,
            $($args: WasmTy,)*
            R: WasmRet,
            T: 'static,
        {
            fn into_func(self, engine: &Engine) -> HostContext {
                HostContext::from_closure(engine, move |caller: Caller<'_, T>, ( $( $args ),* )| {
                    self(caller, $( $args ),* )
                })
            }
        }
    }
}

for_each_function_signature!(impl_into_func);

/// Trait implemented for various tuples made up of types which implement
/// [`WasmTy`] that can be passed to [`Func::wrap_inner`] and
/// [`HostContext::from_closure`].
pub unsafe trait WasmTyList {
    /// Get the value type that each Type in the list represents.
    fn valtypes() -> impl Iterator<Item = ValType>;

    // Load a version of `Self` from the `values` provided.
    //
    // # Safety
    //
    // This function is unsafe as it's up to the caller to ensure that `values` are
    // valid for this given type.
    #[doc(hidden)]
    unsafe fn load(store: &mut AutoAssertNoGc<'_>, values: &mut [MaybeUninit<ValRaw>]) -> Self;

    #[doc(hidden)]
    fn may_gc() -> bool;
}

macro_rules! impl_wasm_ty_list {
    ($num:tt $($args:ident)*) => (
        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($args),*> WasmTyList for ($($args,)*)
        where
            $($args: WasmTy,)*
        {
            fn valtypes() -> impl Iterator<Item = ValType> {
                IntoIterator::into_iter([$($args::valtype(),)*])
            }

            unsafe fn load(_store: &mut AutoAssertNoGc<'_>, _values: &mut [MaybeUninit<ValRaw>]) -> Self {
                let mut _cur = 0;
                ($({
                    debug_assert!(_cur < _values.len());
                    // SAFETY: this function's own contract means that `_values`
                    // is appropriately sized/typed for the internal loads.
                    unsafe {
                        let ptr = _values.get_unchecked(_cur).assume_init_ref();
                        _cur += 1;
                        $args::load(_store, ptr)
                    }
                },)*)
            }

            fn may_gc() -> bool {
                $( $args::may_gc() || )* false
            }
        }
    );
}

for_each_function_signature!(impl_wasm_ty_list);

/// A structure representing the caller's context when creating a function
/// via [`Func::wrap`].
///
/// This structure can be taken as the first parameter of a closure passed to
/// [`Func::wrap`] or other constructors, and serves two purposes:
///
/// * First consumers can use [`Caller<'_, T>`](crate::Caller) to get access to
///   [`StoreContextMut<'_, T>`](crate::StoreContextMut) and/or get access to
///   `T` itself. This means that the [`Caller`] type can serve as a proxy to
///   the original [`Store`](crate::Store) itself and is used to satisfy
///   [`AsContext`] and [`AsContextMut`] bounds.
///
/// * Second a [`Caller`] can be used as the name implies, learning about the
///   caller's context, namely it's exported memory and exported functions. This
///   allows functions which take pointers as arguments to easily read the
///   memory the pointers point into, or if a function is expected to call
///   malloc in the wasm module to reserve space for the output you can do that.
///
/// Host functions which want access to [`Store`](crate::Store)-level state are
/// recommended to use this type.
pub struct Caller<'a, T: 'static> {
    pub(crate) store: StoreContextMut<'a, T>,
    caller: Instance,
}

impl<T> Caller<'_, T> {
    #[cfg(feature = "async")]
    pub(crate) fn new(store: StoreContextMut<'_, T>, caller: Instance) -> Caller<'_, T> {
        Caller { store, caller }
    }

    #[cfg(feature = "async")]
    pub(crate) fn caller(&self) -> Instance {
        self.caller
    }

    /// Executes `f` with an appropriate `Caller`.
    ///
    /// This is the entrypoint for host functions in core wasm and converts from
    /// `VMContext` to `Caller`
    ///
    /// # Safety
    ///
    /// This requires that `caller` is safe to wrap up as a `Caller`,
    /// effectively meaning that we just entered the host from wasm.
    /// Additionally this `Caller`'s `T` parameter must match the actual `T` in
    /// the store of the vmctx of `caller`.
    unsafe fn with<F, R>(caller: NonNull<VMContext>, f: F) -> R
    where
        F: FnOnce(Caller<'_, T>) -> R,
    {
        // SAFETY: it's a contract of this function itself that `from_vmctx` is
        // safe to call. Additionally it's a contract of this function itself
        // that the `T` of `Caller` matches the store.
        unsafe {
            crate::runtime::vm::InstanceAndStore::from_vmctx(caller, |pair| {
                let (instance, store) = pair.unpack_mut();
                let mut store = store.unchecked_context_mut::<T>();
                let caller = Instance::from_wasmtime(instance.id(), store.0);

                let (gc_lifo_scope, ret) = {
                    let gc_lifo_scope = store.0.gc_roots().enter_lifo_scope();

                    let ret = f(Caller {
                        store: store.as_context_mut(),
                        caller,
                    });

                    (gc_lifo_scope, ret)
                };

                // Safe to recreate a mutable borrow of the store because `ret`
                // cannot be borrowing from the store.
                store.0.exit_gc_lifo_scope(gc_lifo_scope);

                ret
            })
        }
    }

    fn sub_caller(&mut self) -> Caller<'_, T> {
        Caller {
            store: self.store.as_context_mut(),
            caller: self.caller,
        }
    }

    /// Looks up an export from the caller's module by the `name` given.
    ///
    /// This is a low-level function that's typically used to implement passing
    /// of pointers or indices between core Wasm instances, where the callee
    /// needs to consult the caller's exports to perform memory management and
    /// resolve the references.
    ///
    /// For comparison, in components, the component model handles translating
    /// arguments from one component instance to another and managing memory, so
    /// that callees don't need to be aware of their callers, which promotes
    /// virtualizability of APIs.
    ///
    /// # Return
    ///
    /// If an export with the `name` provided was found, then it is returned as an
    /// `Extern`. There are a number of situations, however, where the export may not
    /// be available:
    ///
    /// * The caller instance may not have an export named `name`
    /// * There may not be a caller available, for example if `Func` was called
    ///   directly from host code.
    ///
    /// It's recommended to take care when calling this API and gracefully
    /// handling a `None` return value.
    pub fn get_export(&mut self, name: &str) -> Option<Extern> {
        // All instances created have a `host_state` with a pointer pointing
        // back to themselves. If this caller doesn't have that `host_state`
        // then it probably means it was a host-created object like `Func::new`
        // which doesn't have any exports we want to return anyway.
        self.caller.get_export(&mut self.store, name)
    }

    /// Looks up an exported [`Extern`] value by a [`ModuleExport`] value.
    ///
    /// This is similar to [`Self::get_export`] but uses a [`ModuleExport`] value to avoid
    /// string lookups where possible. [`ModuleExport`]s can be obtained by calling
    /// [`Module::get_export_index`] on the [`Module`] that an instance was instantiated with.
    ///
    /// This method will search the module for an export with a matching entity index and return
    /// the value, if found.
    ///
    /// Returns `None` if there was no export with a matching entity index.
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    ///
    /// # Usage
    /// ```
    /// use std::str;
    ///
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut store = Store::default();
    ///
    /// let module = Module::new(
    ///     store.engine(),
    ///     r#"
    ///         (module
    ///             (import "" "" (func $log_str (param i32 i32)))
    ///             (func (export "foo")
    ///                 i32.const 4   ;; ptr
    ///                 i32.const 13  ;; len
    ///                 call $log_str)
    ///             (memory (export "memory") 1)
    ///             (data (i32.const 4) "Hello, world!"))
    ///     "#,
    /// )?;
    ///
    /// let Some(module_export) = module.get_export_index("memory") else {
    ///    anyhow::bail!("failed to find `memory` export in module");
    /// };
    ///
    /// let log_str = Func::wrap(&mut store, move |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
    ///     let mem = match caller.get_module_export(&module_export) {
    ///         Some(Extern::Memory(mem)) => mem,
    ///         _ => anyhow::bail!("failed to find host memory"),
    ///     };
    ///     let data = mem.data(&caller)
    ///         .get(ptr as u32 as usize..)
    ///         .and_then(|arr| arr.get(..len as u32 as usize));
    ///     let string = match data {
    ///         Some(data) => match str::from_utf8(data) {
    ///             Ok(s) => s,
    ///             Err(_) => anyhow::bail!("invalid utf-8"),
    ///         },
    ///         None => anyhow::bail!("pointer/length out of bounds"),
    ///     };
    ///     assert_eq!(string, "Hello, world!");
    ///     println!("{}", string);
    ///     Ok(())
    /// });
    /// let instance = Instance::new(&mut store, &module, &[log_str.into()])?;
    /// let foo = instance.get_typed_func::<(), ()>(&mut store, "foo")?;
    /// foo.call(&mut store, ())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_module_export(&mut self, export: &ModuleExport) -> Option<Extern> {
        self.caller.get_module_export(&mut self.store, export)
    }

    /// Access the underlying data owned by this `Store`.
    ///
    /// Same as [`Store::data`](crate::Store::data)
    pub fn data(&self) -> &T {
        self.store.data()
    }

    /// Access the underlying data owned by this `Store`.
    ///
    /// Same as [`Store::data_mut`](crate::Store::data_mut)
    pub fn data_mut(&mut self) -> &mut T {
        self.store.data_mut()
    }

    /// Returns the underlying [`Engine`] this store is connected to.
    pub fn engine(&self) -> &Engine {
        self.store.engine()
    }

    /// Perform garbage collection.
    ///
    /// Same as [`Store::gc`](crate::Store::gc).
    #[cfg(feature = "gc")]
    pub fn gc(&mut self, why: Option<&crate::GcHeapOutOfMemory<()>>) {
        self.store.gc(why);
    }

    /// Perform garbage collection asynchronously.
    ///
    /// Same as [`Store::gc_async`](crate::Store::gc_async).
    #[cfg(all(feature = "async", feature = "gc"))]
    pub async fn gc_async(&mut self, why: Option<&crate::GcHeapOutOfMemory<()>>) -> Result<()>
    where
        T: Send + 'static,
    {
        self.store.gc_async(why).await
    }

    /// Returns the remaining fuel in the store.
    ///
    /// For more information see [`Store::get_fuel`](crate::Store::get_fuel)
    pub fn get_fuel(&self) -> Result<u64> {
        self.store.get_fuel()
    }

    /// Set the amount of fuel in this store to be consumed when executing wasm code.
    ///
    /// For more information see [`Store::set_fuel`](crate::Store::set_fuel)
    pub fn set_fuel(&mut self, fuel: u64) -> Result<()> {
        self.store.set_fuel(fuel)
    }

    /// Configures this `Store` to yield while executing futures every N units of fuel.
    ///
    /// For more information see
    /// [`Store::fuel_async_yield_interval`](crate::Store::fuel_async_yield_interval)
    pub fn fuel_async_yield_interval(&mut self, interval: Option<u64>) -> Result<()> {
        self.store.fuel_async_yield_interval(interval)
    }
}

impl<T: 'static> AsContext for Caller<'_, T> {
    type Data = T;
    fn as_context(&self) -> StoreContext<'_, T> {
        self.store.as_context()
    }
}

impl<T: 'static> AsContextMut for Caller<'_, T> {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, T> {
        self.store.as_context_mut()
    }
}

// State stored inside a `VMArrayCallHostFuncContext`.
struct HostFuncState<F> {
    // The actual host function.
    func: F,

    // NB: We have to keep our `VMSharedTypeIndex` registered in the engine for
    // as long as this function exists.
    _ty: RegisteredType,
}

#[doc(hidden)]
pub enum HostContext {
    Array(StoreBox<VMArrayCallHostFuncContext>),
}

impl From<StoreBox<VMArrayCallHostFuncContext>> for HostContext {
    fn from(ctx: StoreBox<VMArrayCallHostFuncContext>) -> Self {
        HostContext::Array(ctx)
    }
}

impl HostContext {
    fn from_closure<F, T, P, R>(engine: &Engine, func: F) -> Self
    where
        F: Fn(Caller<'_, T>, P) -> R + Send + Sync + 'static,
        P: WasmTyList,
        R: WasmRet,
        T: 'static,
    {
        let ty = R::func_type(engine, None::<ValType>.into_iter().chain(P::valtypes()));
        let type_index = ty.type_index();

        let array_call = Self::array_call_trampoline::<T, F, P, R>;

        let ctx = unsafe {
            VMArrayCallHostFuncContext::new(
                array_call,
                type_index,
                Box::new(HostFuncState {
                    func,
                    _ty: ty.into_registered_type(),
                }),
            )
        };

        ctx.into()
    }

    /// Raw entry trampoline for wasm for typed functions.
    ///
    /// # Safety
    ///
    /// The `callee_vmctx`, `caller_vmctx`, and `args` values must basically be
    /// "all valid" in the sense that they're from the same store, appropriately
    /// sized, appropriate to dereference, etc. This requires that `T` matches
    /// the type of the store that the vmctx values point to. The `F` parameter
    /// must match the state in `callee_vmctx`. The `P` and `R` type parameters
    /// must accurately describe the params/results store in `args`.
    unsafe extern "C" fn array_call_trampoline<T, F, P, R>(
        callee_vmctx: NonNull<VMOpaqueContext>,
        caller_vmctx: NonNull<VMContext>,
        args: NonNull<ValRaw>,
        args_len: usize,
    ) -> bool
    where
        F: Fn(Caller<'_, T>, P) -> R + 'static,
        P: WasmTyList,
        R: WasmRet,
        T: 'static,
    {
        // Note that this function is intentionally scoped into a
        // separate closure. Handling traps and panics will involve
        // longjmp-ing from this function which means we won't run
        // destructors. As a result anything requiring a destructor
        // should be part of this closure, and the long-jmp-ing
        // happens after the closure in handling the result.
        let run = move |mut caller: Caller<'_, T>| {
            let mut args =
                NonNull::slice_from_raw_parts(args.cast::<MaybeUninit<ValRaw>>(), args_len);
            // SAFETY: it's a safety contract of this function itself that
            // `callee_vmctx` is safe to read.
            let state = unsafe {
                let vmctx = VMArrayCallHostFuncContext::from_opaque(callee_vmctx);
                vmctx.as_ref().host_state()
            };

            // Double-check ourselves in debug mode, but we control
            // the `Any` here so an unsafe downcast should also
            // work.
            //
            // SAFETY: all typed host functions use `HostFuncState<F>` as their
            // state so this should be safe to effectively do an unchecked
            // downcast.
            let state = unsafe {
                debug_assert!(state.is::<HostFuncState<F>>());
                &*(state as *const _ as *const HostFuncState<F>)
            };
            let func = &state.func;

            let ret = 'ret: {
                if let Err(trap) = caller.store.0.call_hook(CallHook::CallingHost) {
                    break 'ret R::fallible_from_error(trap);
                }

                let mut store = if P::may_gc() {
                    AutoAssertNoGc::new(caller.store.0)
                } else {
                    unsafe { AutoAssertNoGc::disabled(caller.store.0) }
                };
                // SAFETY: this function requires `args` to be valid and the
                // `WasmTyList` trait means that everything should be correctly
                // ascribed/typed, making this valid to load from.
                let params = unsafe { P::load(&mut store, args.as_mut()) };
                let _ = &mut store;
                drop(store);

                let r = func(caller.sub_caller(), params);

                if let Err(trap) = caller.store.0.call_hook(CallHook::ReturningFromHost) {
                    break 'ret R::fallible_from_error(trap);
                }
                r.into_fallible()
            };

            if !ret.compatible_with_store(caller.store.0) {
                bail!("host function attempted to return cross-`Store` value to Wasm")
            } else {
                let mut store = if R::may_gc() {
                    AutoAssertNoGc::new(caller.store.0)
                } else {
                    unsafe { AutoAssertNoGc::disabled(caller.store.0) }
                };
                // SAFETY: this function requires that `args` is safe for this
                // type signature, and the guarantees of `WasmRet` means that
                // everything should be typed appropriately.
                let ret = unsafe { ret.store(&mut store, args.as_mut())? };
                Ok(ret)
            }
        };

        // With nothing else on the stack move `run` into this
        // closure and then run it as part of `Caller::with`.
        //
        // SAFETY: this is an entrypoint of wasm which requires correct type
        // ascription of `T` itself, meaning that this should be safe to call.
        crate::runtime::vm::catch_unwind_and_record_trap(move || unsafe {
            Caller::with(caller_vmctx, run)
        })
    }
}

/// Representation of a host-defined function.
///
/// This is used for `Func::new` but also for `Linker`-defined functions. For
/// `Func::new` this is stored within a `Store`, and for `Linker`-defined
/// functions they wrap this up in `Arc` to enable shared ownership of this
/// across many stores.
///
/// Technically this structure needs a `<T>` type parameter to connect to the
/// `Store<T>` itself, but that's an unsafe contract of using this for now
/// rather than part of the struct type (to avoid `Func<T>` in the API).
pub(crate) struct HostFunc {
    ctx: HostContext,

    // Stored to unregister this function's signature with the engine when this
    // is dropped.
    engine: Engine,
}

impl core::fmt::Debug for HostFunc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HostFunc").finish_non_exhaustive()
    }
}

impl HostFunc {
    /// Analog of [`Func::new`]
    ///
    /// # Panics
    ///
    /// Panics if the given function type is not associated with the given
    /// engine.
    pub fn new<T>(
        engine: &Engine,
        ty: FuncType,
        func: impl Fn(Caller<'_, T>, &[Val], &mut [Val]) -> Result<()> + Send + Sync + 'static,
    ) -> Self
    where
        T: 'static,
    {
        assert!(ty.comes_from_same_engine(engine));
        let ty_clone = ty.clone();
        unsafe {
            HostFunc::new_unchecked(engine, ty, move |caller, values| {
                Func::invoke_host_func_for_wasm(caller, &ty_clone, values, &func)
            })
        }
    }

    /// Analog of [`Func::new_unchecked`]
    ///
    /// # Panics
    ///
    /// Panics if the given function type is not associated with the given
    /// engine.
    ///
    /// # Safety
    ///
    /// The `func` provided must operate according to the `ty` provided to
    /// ensure it's reading the correctly-typed parameters and writing the
    /// correctly-typed results.
    pub unsafe fn new_unchecked<T>(
        engine: &Engine,
        ty: FuncType,
        func: impl Fn(Caller<'_, T>, &mut [ValRaw]) -> Result<()> + Send + Sync + 'static,
    ) -> Self
    where
        T: 'static,
    {
        assert!(ty.comes_from_same_engine(engine));
        // SAFETY: This is only only called in the raw entrypoint of wasm
        // meaning that `caller_vmctx` is appropriate to read, and additionally
        // the later usage of `{,in}to_func` will connect `T` to an actual
        // store's `T` to ensure it's the same.
        let func = move |caller_vmctx, values: &mut [ValRaw]| unsafe {
            Caller::<T>::with(caller_vmctx, |mut caller| {
                caller.store.0.call_hook(CallHook::CallingHost)?;
                let result = func(caller.sub_caller(), values)?;
                caller.store.0.call_hook(CallHook::ReturningFromHost)?;
                Ok(result)
            })
        };
        let ctx = crate::trampoline::create_array_call_function(&ty, func)
            .expect("failed to create function");
        HostFunc::_new(engine, ctx.into())
    }

    /// Analog of [`Func::wrap_inner`]
    #[cfg(any(feature = "component-model", feature = "async"))]
    pub fn wrap_inner<F, T, Params, Results>(engine: &Engine, func: F) -> Self
    where
        F: Fn(Caller<'_, T>, Params) -> Results + Send + Sync + 'static,
        Params: WasmTyList,
        Results: WasmRet,
        T: 'static,
    {
        let ctx = HostContext::from_closure(engine, func);
        HostFunc::_new(engine, ctx)
    }

    /// Analog of [`Func::wrap`]
    pub fn wrap<T, Params, Results>(
        engine: &Engine,
        func: impl IntoFunc<T, Params, Results>,
    ) -> Self
    where
        T: 'static,
    {
        let ctx = func.into_func(engine);
        HostFunc::_new(engine, ctx)
    }

    /// Requires that this function's signature is already registered within
    /// `Engine`. This happens automatically during the above two constructors.
    fn _new(engine: &Engine, ctx: HostContext) -> Self {
        HostFunc {
            ctx,
            engine: engine.clone(),
        }
    }

    /// Inserts this `HostFunc` into a `Store`, returning the `Func` pointing to
    /// it.
    ///
    /// # Unsafety
    ///
    /// Can only be inserted into stores with a matching `T` relative to when
    /// this `HostFunc` was first created.
    pub unsafe fn to_func(self: &Arc<Self>, store: &mut StoreOpaque) -> Func {
        self.validate_store(store);
        let (funcrefs, modules) = store.func_refs_and_modules();
        let funcref = funcrefs.push_arc_host(self.clone(), modules);
        // SAFETY: this funcref was just pushed within the store, so it's safe
        // to say this store owns it.
        unsafe { Func::from_vm_func_ref(store.id(), funcref) }
    }

    /// Inserts this `HostFunc` into a `Store`, returning the `Func` pointing to
    /// it.
    ///
    /// This function is similar to, but not equivalent, to `HostFunc::to_func`.
    /// Notably this function requires that the `Arc<Self>` pointer is otherwise
    /// rooted within the `StoreOpaque` via another means. When in doubt use
    /// `to_func` above as it's safer.
    ///
    /// # Unsafety
    ///
    /// Can only be inserted into stores with a matching `T` relative to when
    /// this `HostFunc` was first created.
    ///
    /// Additionally the `&Arc<Self>` is not cloned in this function. Instead a
    /// raw pointer to `Self` is stored within the `Store` for this function.
    /// The caller must arrange for the `Arc<Self>` to be "rooted" in the store
    /// provided via another means, probably by pushing to
    /// `StoreOpaque::rooted_host_funcs`.
    ///
    /// Similarly, the caller must arrange for `rooted_func_ref` to be rooted in
    /// the same store and additionally be a valid pointer.
    pub unsafe fn to_func_store_rooted(
        self: &Arc<Self>,
        store: &mut StoreOpaque,
        rooted_func_ref: Option<NonNull<VMFuncRef>>,
    ) -> Func {
        self.validate_store(store);

        match rooted_func_ref {
            Some(funcref) => {
                // SAFETY: it's a contract of this function itself that
                // `funcref` is safe to read.
                unsafe {
                    debug_assert!(funcref.as_ref().wasm_call.is_some());
                }
                // SAFETY: it's a contract of this function that `funcref` is
                // owned by `store`.
                unsafe { Func::from_vm_func_ref(store.id(), funcref) }
            }
            None => {
                debug_assert!(self.func_ref().wasm_call.is_some());

                // SAFETY: it's an unsafe contract of this function that we are
                // rooted within the store to say that the store owns a copy of
                // this funcref.
                unsafe { Func::from_vm_func_ref(store.id(), self.func_ref().into()) }
            }
        }
    }

    /// Same as [`HostFunc::to_func`], different ownership.
    unsafe fn into_func(self, store: &mut StoreOpaque) -> Func {
        self.validate_store(store);
        let (funcrefs, modules) = store.func_refs_and_modules();
        let funcref = funcrefs.push_box_host(Box::new(self), modules);
        // SAFETY: this funcref was just pushed within `store`, so it's safe to
        // say it's owned by the store's id.
        unsafe { Func::from_vm_func_ref(store.id(), funcref) }
    }

    fn validate_store(&self, store: &mut StoreOpaque) {
        // This assert is required to ensure that we can indeed safely insert
        // `self` into the `store` provided, otherwise the type information we
        // have listed won't be correct. This is possible to hit with the public
        // API of Wasmtime, and should be documented in relevant functions.
        assert!(
            Engine::same(&self.engine, store.engine()),
            "cannot use a store with a different engine than a linker was created with",
        );
    }

    pub(crate) fn sig_index(&self) -> VMSharedTypeIndex {
        self.func_ref().type_index
    }

    pub(crate) fn func_ref(&self) -> &VMFuncRef {
        match &self.ctx {
            HostContext::Array(ctx) => unsafe { ctx.get().as_ref().func_ref() },
        }
    }

    pub(crate) fn host_ctx(&self) -> &HostContext {
        &self.ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Module, Store};

    #[test]
    #[cfg_attr(miri, ignore)]
    fn hash_key_is_stable_across_duplicate_store_data_entries() -> Result<()> {
        let mut store = Store::<()>::default();
        let module = Module::new(
            store.engine(),
            r#"
                (module
                    (func (export "f")
                        nop
                    )
                )
            "#,
        )?;
        let instance = Instance::new(&mut store, &module, &[])?;

        // Each time we `get_func`, we call `Func::from_wasmtime` which adds a
        // new entry to `StoreData`, so `f1` and `f2` will have different
        // indices into `StoreData`.
        let f1 = instance.get_func(&mut store, "f").unwrap();
        let f2 = instance.get_func(&mut store, "f").unwrap();

        // But their hash keys are the same.
        assert!(
            f1.hash_key(&mut store.as_context_mut().0)
                == f2.hash_key(&mut store.as_context_mut().0)
        );

        // But the hash keys are different from different funcs.
        let instance2 = Instance::new(&mut store, &module, &[])?;
        let f3 = instance2.get_func(&mut store, "f").unwrap();
        assert!(
            f1.hash_key(&mut store.as_context_mut().0)
                != f3.hash_key(&mut store.as_context_mut().0)
        );

        Ok(())
    }
}
