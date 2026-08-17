// Take a look at the license at the top of the repository in the LICENSE file.

// TODO: support marshaller.

use std::{mem, ptr, slice};

use libc::{c_uint, c_void};

use crate::{gobject_ffi, prelude::*, translate::*, Type, Value};

wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    #[doc(alias = "GClosure")]
    pub struct Closure(Shared<gobject_ffi::GClosure>);

    match fn {
        ref => |ptr| {
            gobject_ffi::g_closure_ref(ptr);
            gobject_ffi::g_closure_sink(ptr);
        },
        unref => |ptr| gobject_ffi::g_closure_unref(ptr),
        type_ => || gobject_ffi::g_closure_get_type(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct RustClosure(Closure);

impl RustClosure {
    // rustdoc-stripper-ignore-next
    /// Creates a new closure around a Rust closure.
    ///
    /// See [`glib::closure!`](macro@crate::closure) for a way to create a closure with concrete
    /// types.
    ///
    /// # Panics
    ///
    /// Invoking the closure with wrong argument types or returning the wrong return value type
    /// will panic.
    ///
    /// # Example
    ///
    /// ```
    /// use glib::prelude::*;
    ///
    /// let closure = glib::RustClosure::new(|values| {
    ///     let x = values[0].get::<i32>().unwrap();
    ///     Some((x + 1).to_value())
    /// });
    ///
    /// assert_eq!(
    ///     closure.invoke::<i32>(&[&1i32]),
    ///     2,
    /// );
    /// ```
    #[doc(alias = "g_closure_new")]
    pub fn new<F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static>(callback: F) -> Self {
        Self(Closure::new(callback))
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new closure around a Rust closure.
    ///
    /// See [`glib::closure_local!`](crate::closure_local) for a way to create a closure with
    /// concrete types.
    ///
    /// # Panics
    ///
    /// Invoking the closure with wrong argument types or returning the wrong return value type
    /// will panic.
    ///
    /// Invoking the closure from a different thread than this one will panic.
    #[doc(alias = "g_closure_new")]
    pub fn new_local<F: Fn(&[Value]) -> Option<Value> + 'static>(callback: F) -> Self {
        Self(Closure::new_local(callback))
    }

    // rustdoc-stripper-ignore-next
    /// Invokes the closure with the given arguments.
    ///
    /// For invalidated closures this returns the "default" value of the return type. For nullable
    /// types this is `None`, which means that e.g. requesting `R = String` will panic will `R =
    /// Option<String>` will return `None`.
    ///
    /// # Panics
    ///
    /// The argument types and return value type must match the ones expected by the closure or
    /// otherwise this function panics.
    #[doc(alias = "g_closure_invoke")]
    pub fn invoke<R: TryFromClosureReturnValue>(&self, values: &[&dyn ToValue]) -> R {
        let values = values
            .iter()
            .copied()
            .map(ToValue::to_value)
            .collect::<smallvec::SmallVec<[_; 10]>>();

        R::try_from_closure_return_value(self.invoke_with_values(R::static_type(), &values))
            .expect("Invalid return value")
    }

    // rustdoc-stripper-ignore-next
    /// Invokes the closure with the given arguments.
    ///
    /// For invalidated closures this returns the "default" value of the return type.
    ///
    /// # Panics
    ///
    /// The argument types and return value type must match the ones expected by the closure or
    /// otherwise this function panics.
    #[doc(alias = "g_closure_invoke")]
    pub fn invoke_with_values(&self, return_type: Type, values: &[Value]) -> Option<Value> {
        unsafe { self.0.invoke_with_values(return_type, values) }
    }

    // rustdoc-stripper-ignore-next
    /// Invalidates the closure.
    ///
    /// Invoking an invalidated closure has no effect.
    #[doc(alias = "g_closure_invalidate")]
    pub fn invalidate(&self) {
        self.0.invalidate();
    }
}

impl From<RustClosure> for Closure {
    #[inline]
    fn from(c: RustClosure) -> Self {
        c.0
    }
}

impl AsRef<Closure> for RustClosure {
    #[inline]
    fn as_ref(&self) -> &Closure {
        &self.0
    }
}

impl AsRef<Closure> for Closure {
    #[inline]
    fn as_ref(&self) -> &Closure {
        self
    }
}

impl Closure {
    // rustdoc-stripper-ignore-next
    /// Creates a new closure around a Rust closure.
    ///
    /// Note that [`RustClosure`] provides more convenient and non-unsafe API for invoking
    /// closures. This type mostly exists for FFI interop.
    ///
    /// # Panics
    ///
    /// Invoking the closure with wrong argument types or returning the wrong return value type
    /// will panic.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use glib::prelude::*;
    ///
    /// let closure = glib::Closure::new(|values| {
    ///     let x = values[0].get::<i32>().unwrap();
    ///     Some((x + 1).to_value())
    /// });
    ///
    /// // Invoking non-Rust closures is unsafe because of possibly missing
    /// // argument and return value type checks.
    /// let res = unsafe {
    ///     closure
    ///         .invoke_with_values(glib::Type::I32, &[1i32.to_value()])
    ///         .and_then(|v| v.get::<i32>().ok())
    ///         .expect("Invalid return value")
    /// };
    ///
    /// assert_eq!(res, 2);
    /// ```
    #[doc(alias = "g_closure_new")]
    pub fn new<F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static>(callback: F) -> Self {
        unsafe { Self::new_unsafe(callback) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new closure around a Rust closure.
    ///
    /// Note that [`RustClosure`] provides more convenient and non-unsafe API for invoking
    /// closures. This type mostly exists for FFI interop.
    ///
    /// # Panics
    ///
    /// Invoking the closure with wrong argument types or returning the wrong return value type
    /// will panic.
    ///
    /// Invoking the closure from a different thread than this one will panic.
    #[doc(alias = "g_closure_new")]
    pub fn new_local<F: Fn(&[Value]) -> Option<Value> + 'static>(callback: F) -> Self {
        let callback = crate::thread_guard::ThreadGuard::new(callback);

        unsafe { Self::new_unsafe(move |values| (callback.get_ref())(values)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new closure around a Rust closure.
    ///
    /// # Safety
    ///
    /// The captured variables of the closure must stay valid as long as the return value of this
    /// constructor does, and it must be valid to call the closure from any thread that is used by
    /// callers.
    #[doc(alias = "g_closure_new")]
    pub unsafe fn new_unsafe<F: Fn(&[Value]) -> Option<Value>>(callback: F) -> Self {
        unsafe extern "C" fn marshal<F>(
            _closure: *mut gobject_ffi::GClosure,
            return_value: *mut gobject_ffi::GValue,
            n_param_values: c_uint,
            param_values: *const gobject_ffi::GValue,
            _invocation_hint: *mut c_void,
            marshal_data: *mut c_void,
        ) where
            F: Fn(&[Value]) -> Option<Value>,
        {
            let values = if n_param_values == 0 {
                &[]
            } else {
                slice::from_raw_parts(param_values as *const _, n_param_values as usize)
            };
            let callback: &F = &*(marshal_data as *mut _);
            let result = callback(values);

            if return_value.is_null() {
                assert!(
                    result.is_none(),
                    "Closure returned a return value but the caller did not expect one"
                );
            } else {
                let return_value = &mut *(return_value as *mut Value);
                match result {
                    Some(result) => {
                        assert!(
                            result.type_().is_a(return_value.type_()),
                            "Closure returned a value of type {} but caller expected {}",
                            result.type_(),
                            return_value.type_()
                        );
                        *return_value = result;
                    }
                    None if return_value.type_() == Type::INVALID => (),
                    None => {
                        panic!(
                            "Closure returned no value but the caller expected a value of type {}",
                            return_value.type_()
                        );
                    }
                }
            }
        }

        unsafe extern "C" fn finalize<F>(
            notify_data: *mut c_void,
            _closure: *mut gobject_ffi::GClosure,
        ) where
            F: Fn(&[Value]) -> Option<Value>,
        {
            let _callback: Box<F> = Box::from_raw(notify_data as *mut _);
            // callback is dropped here.
        }

        // Due to bitfields we have to do our own calculations here for the size of the GClosure:
        // - 4: 32 bits in guint bitfields at the beginning
        // - padding due to alignment needed for the following pointer
        // - 3 * size_of<*mut c_void>: 3 pointers
        // We don't store any custom data ourselves in the GClosure
        let size = u32::max(4, mem::align_of::<*mut c_void>() as u32)
            + 3 * mem::size_of::<*mut c_void>() as u32;
        let closure = gobject_ffi::g_closure_new_simple(size, ptr::null_mut());
        let callback = Box::new(callback);
        let ptr: *mut F = Box::into_raw(callback);
        let ptr: *mut c_void = ptr as *mut _;
        gobject_ffi::g_closure_set_meta_marshal(closure, ptr, Some(marshal::<F>));
        gobject_ffi::g_closure_add_finalize_notifier(closure, ptr, Some(finalize::<F>));
        from_glib_none(closure)
    }

    // rustdoc-stripper-ignore-next
    /// Invokes the closure with the given arguments.
    ///
    /// For invalidated closures this returns the "default" value of the return type.
    ///
    /// # Safety
    ///
    /// The argument types and return value type must match the ones expected by the closure or
    /// otherwise the behaviour is undefined.
    ///
    /// Closures created from Rust via e.g. [`Closure::new`] will panic on type mismatches but
    /// this is not guaranteed for closures created from other languages.
    #[doc(alias = "g_closure_invoke")]
    pub unsafe fn invoke_with_values(&self, return_type: Type, values: &[Value]) -> Option<Value> {
        let mut result = if return_type == Type::UNIT {
            Value::uninitialized()
        } else {
            Value::from_type(return_type)
        };
        let result_ptr = if return_type == Type::UNIT {
            ptr::null_mut()
        } else {
            result.to_glib_none_mut().0
        };

        gobject_ffi::g_closure_invoke(
            self.to_glib_none().0,
            result_ptr,
            values.len() as u32,
            mut_override(values.as_ptr()) as *mut gobject_ffi::GValue,
            ptr::null_mut(),
        );

        if return_type == Type::UNIT {
            None
        } else {
            Some(result)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Invalidates the closure.
    ///
    /// Invoking an invalidated closure has no effect.
    #[doc(alias = "g_closure_invalidate")]
    pub fn invalidate(&self) {
        unsafe {
            gobject_ffi::g_closure_invalidate(self.to_glib_none().0);
        }
    }
}

pub trait IntoClosureReturnValue {
    fn into_closure_return_value(self) -> Option<Value>;
}

impl IntoClosureReturnValue for () {
    #[inline]
    fn into_closure_return_value(self) -> Option<Value> {
        None
    }
}

impl<T: Into<Value>> IntoClosureReturnValue for T {
    #[inline]
    fn into_closure_return_value(self) -> Option<Value> {
        Some(self.into())
    }
}

pub trait TryFromClosureReturnValue: StaticType + Sized + 'static {
    fn try_from_closure_return_value(v: Option<Value>) -> Result<Self, crate::BoolError>;
}

impl TryFromClosureReturnValue for () {
    #[inline]
    fn try_from_closure_return_value(v: Option<Value>) -> Result<Self, crate::BoolError> {
        match v {
            None => Ok(()),
            Some(v) => Err(bool_error!(
                "Invalid return value: expected (), got {}",
                v.type_()
            )),
        }
    }
}

impl<T: for<'a> crate::value::FromValue<'a> + StaticType + 'static> TryFromClosureReturnValue
    for T
{
    #[inline]
    fn try_from_closure_return_value(v: Option<Value>) -> Result<Self, crate::BoolError> {
        v.ok_or_else(|| {
            bool_error!(
                "Invalid return value: expected {}, got ()",
                T::static_type()
            )
        })
        .and_then(|v| {
            v.get_owned::<T>().map_err(|_| {
                bool_error!(
                    "Invalid return value: expected {}, got {}",
                    T::static_type(),
                    v.type_()
                )
            })
        })
    }
}

unsafe impl Send for Closure {}
unsafe impl Sync for Closure {}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use super::*;

    #[allow(clippy::unnecessary_wraps)]
    fn closure_fn(values: &[Value]) -> Option<Value> {
        assert_eq!(values.len(), 2);
        let string_arg = values[0].get::<&str>();
        assert_eq!(string_arg, Ok("test"));
        let int_arg = values[1].get::<i32>();
        assert_eq!(int_arg, Ok(42));
        Some(24.to_value())
    }

    #[test]
    fn test_closure() {
        let call_count = Arc::new(AtomicUsize::new(0));

        let count = call_count.clone();
        let closure = RustClosure::new(move |values| {
            count.fetch_add(1, Ordering::Relaxed);
            assert_eq!(values.len(), 2);
            let string_arg = values[0].get::<&str>();
            assert_eq!(string_arg, Ok("test"));
            let int_arg = values[1].get::<i32>();
            assert_eq!(int_arg, Ok(42));
            None
        });
        closure.invoke::<()>(&[&"test", &42]);
        assert_eq!(call_count.load(Ordering::Relaxed), 1);

        closure.invoke::<()>(&[&"test", &42]);
        assert_eq!(call_count.load(Ordering::Relaxed), 2);

        closure.invalidate();
        closure.invoke::<()>(&[&"test", &42]);
        assert_eq!(call_count.load(Ordering::Relaxed), 2);

        let closure = RustClosure::new(closure_fn);
        let result = closure.invoke::<i32>(&[&"test", &42]);
        assert_eq!(result, 24);
        closure.invalidate();
        let result = closure.invoke::<i32>(&[&"test", &42]);
        assert_eq!(result, 0);
    }
}
