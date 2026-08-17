use std::{
    marker::PhantomData,
    os::raw::{c_char, c_void},
    ptr, str,
    str::FromStr,
    sync::{Mutex, MutexGuard},
};

use log::warn;

use crate::{
    descriptors::Desc,
    errors::*,
    objects::{
        AutoElements, AutoElementsCritical, AutoLocal, GlobalRef, JByteBuffer, JClass, JFieldID,
        JList, JMap, JMethodID, JObject, JStaticFieldID, JStaticMethodID, JString, JThrowable,
        JValue, JValueOwned, ReleaseMode, TypeArray, WeakRef,
    },
    signature::{JavaType, Primitive, TypeSignature},
    strings::{JNIString, JavaStr},
    sys::{
        self, jarray, jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort, jsize, jvalue,
        JNINativeMethod,
    },
    JNIVersion, JavaVM,
};
use crate::{
    errors::Error::JniCall,
    objects::{
        JBooleanArray, JByteArray, JCharArray, JDoubleArray, JFloatArray, JIntArray, JLongArray,
        JObjectArray, JPrimitiveArray, JShortArray,
    },
};
use crate::{objects::AsJArrayRaw, signature::ReturnType};

/// FFI-compatible JNIEnv struct. You can safely use this as the JNIEnv argument
/// to exported methods that will be called by java. This is where most of the
/// magic happens. All methods on this object are wrappers around JNI functions,
/// so the documentation on their behavior is still pretty applicable.
///
/// # Exception handling
///
/// Since we're calling into the JVM with this, many methods also have the
/// potential to cause an exception to get thrown. If this is the case, an `Err`
/// result will be returned with the error kind `JavaException`. Note that this
/// will _not_ clear the exception - it's up to the caller to decide whether to
/// do so or to let it continue being thrown.
///
/// # References and Lifetimes
///
/// As in C JNI, interactions with Java objects happen through <dfn>references</dfn>, either local
/// or global, represented by [`JObject`] and [`GlobalRef`] respectively. So long as there is at
/// least one such reference to a Java object, the JVM garbage collector will not reclaim it.
///
/// <dfn>Global references</dfn> exist until deleted. Deletion occurs when the `GlobalRef` is
/// dropped.
///
/// <dfn>Local references</dfn> belong to a local reference frame, and exist until
/// [deleted][JNIEnv::delete_local_ref] or until the local reference frame is exited. A <dfn>local
/// reference frame</dfn> is entered when a native method is called from Java, or when Rust code
/// does so explicitly using [`JNIEnv::with_local_frame`]. That local reference frame is exited
/// when the native method or `with_local_frame` returns. When a local reference frame is exited,
/// all local references created inside it are deleted.
///
/// Unlike C JNI, this crate creates a separate `JNIEnv` for each local reference frame. The
/// associated Rust lifetime `'local` represents that local reference frame. Rust's borrow checker
/// will ensure that local references are not used after their local reference frame exits (which
/// would cause undefined behavior).
///
/// Unlike global references, local references are not deleted when dropped by default. This is for
/// performance: it is faster for the JVM to delete all of the local references in a frame all at
/// once, than to delete each local reference one at a time. However, this can cause a memory leak
/// if the local reference frame remains entered for a long time, such as a long-lasting loop, in
/// which case local references should be deleted explicitly. Local references can be deleted when
/// dropped if desired; use [`JNIEnv::auto_local`] to arrange that.
///
/// ## Lifetime Names
///
/// This crate uses the following convention for lifetime names:
///
/// * `'local` is the lifetime of a local reference frame, as described above.
///
/// * `'other_local`, `'other_local_1`, and `'other_local_2` are the lifetimes of some other local
///   reference frame, which may be but doesn't have to be the same as `'local`. For example,
///   [`JNIEnv::new_local_ref`] accepts a local reference in any local reference frame
///   `'other_local` and creates a new local reference to the same object in `'local`.
///
/// * `'obj_ref` is the lifetime of a borrow of a JNI reference, like <code>&amp;[JObject]</code>
///   or <code>&amp;[GlobalRef]</code>. For example, [`JNIEnv::get_list`] constructs a new
///   [`JList`] that borrows a `&'obj_ref JObject`.
///
/// ## `null` Java references
/// `null` Java references are handled by the following rules:
///   - If a `null` Java reference is passed to a method that expects a non-`null`
///   argument, an `Err` result with the kind `NullPtr` is returned.
///   - If a JNI function returns `null` to indicate an error (e.g. `new_int_array`),
///     it is converted to `Err`/`NullPtr` or, where possible, to a more applicable
///     error type, such as `MethodNotFound`. If the JNI function also throws
///     an exception, the `JavaException` error kind will be preferred.
///   - If a JNI function may return `null` Java reference as one of possible reference
///     values (e.g., `get_object_array_element` or `get_field_unchecked`),
///     it is converted to `JObject::null()`.
///
/// # `&self` and `&mut self`
///
/// Most of the methods on this type take a `&mut self` reference, specifically all methods that
/// can enter a new local reference frame. This includes anything that might invoke user-defined
/// Java code, which can indirectly enter a new local reference frame by calling a native method.
///
/// The reason for this restriction is to ensure that a `JNIEnv` instance can only be used in the
/// local reference frame that it belongs to. This, in turn, ensures that it is not possible to
/// create [`JObject`]s with the lifetime of a different local reference frame, which would lead to
/// undefined behavior. (See [issue #392] for background discussion.)
///
/// [issue #392]: https://github.com/jni-rs/jni-rs/issues/392
///
/// ## `cannot borrow as mutable`
///
/// If a function takes two or more parameters, one of them is `JNIEnv`, and another is something
/// returned by a `JNIEnv` method (like [`JObject`]), then calls to that function may not compile:
///
/// ```rust,compile_fail
/// # use jni::{errors::Result, JNIEnv, objects::*};
/// #
/// # fn f(env: &mut JNIEnv) -> Result<()> {
/// fn example_function(
///     env: &mut JNIEnv,
///     obj: &JObject,
/// ) {
///     // …
/// }
///
/// example_function(
///     env,
///     // ERROR: cannot borrow `*env` as mutable more than once at a time
///     &env.new_object(
///         "com/example/SomeClass",
///         "()V",
///         &[],
///     )?,
/// )
/// # ; Ok(())
/// # }
/// ```
///
/// To fix this, the `JNIEnv` parameter needs to come *last*:
///
/// ```rust,no_run
/// # use jni::{errors::Result, JNIEnv, objects::*};
/// #
/// # fn f(env: &mut JNIEnv) -> Result<()> {
/// fn example_function(
///     obj: &JObject,
///     env: &mut JNIEnv,
/// ) {
///     // …
/// }
///
/// example_function(
///     &env.new_object(
///         "com/example/SomeClass",
///         "()V",
///         &[],
///     )?,
///     env,
/// )
/// # ; Ok(())
/// # }
/// ```
///
/// # Checked and unchecked methods
///
/// Some of the methods come in two versions: checked (e.g. `call_method`) and
/// unchecked (e.g. `call_method_unchecked`). Under the hood, checked methods
/// perform some checks to ensure the validity of provided signatures, names
/// and arguments, and then call the corresponding unchecked method.
///
/// Checked methods are more flexible as they allow passing class names
/// and method/field descriptors as strings and may perform lookups
/// of class objects and method/field ids for you, also performing
/// all the needed precondition checks. However, these lookup operations
/// are expensive, so if you need to call the same method (or access
/// the same field) multiple times, it is
/// [recommended](https://docs.oracle.com/en/java/javase/11/docs/specs/jni/design.html#accessing-fields-and-methods)
/// to cache the instance of the class and the method/field id, e.g.
///   - in loops
///   - when calling the same Java callback repeatedly.
///
/// If you do not cache references to classes and method/field ids,
/// you will *not* benefit from the unchecked methods.
///
/// Calling unchecked methods with invalid arguments and/or invalid class and
/// method descriptors may lead to segmentation fault.
#[repr(transparent)]
#[derive(Debug)]
pub struct JNIEnv<'local> {
    internal: *mut sys::JNIEnv,
    lifetime: PhantomData<&'local ()>,
}

impl<'local> JNIEnv<'local> {
    /// Create a JNIEnv from a raw pointer.
    ///
    /// # Safety
    ///
    /// Expects a valid pointer retrieved from the `GetEnv` JNI function or [Self::get_raw] function. Only does a null check.
    pub unsafe fn from_raw(ptr: *mut sys::JNIEnv) -> Result<Self> {
        non_null!(ptr, "from_raw ptr argument");
        Ok(JNIEnv {
            internal: ptr,
            lifetime: PhantomData,
        })
    }

    /// Get the raw JNIEnv pointer
    pub fn get_raw(&self) -> *mut sys::JNIEnv {
        self.internal
    }

    /// Duplicates this `JNIEnv`.
    ///
    /// # Safety
    ///
    /// The duplicate `JNIEnv` must not be used to create any local references, unless they are
    /// discarded before the current [local reference frame] is exited. Otherwise, they may have a
    /// lifetime longer than they are actually valid for, resulting in a use-after-free bug and
    /// undefined behavior.
    ///
    /// See [issue #392] for background.
    ///
    /// [local reference frame]: JNIEnv::with_local_frame
    /// [issue #392]: https://github.com/jni-rs/jni-rs/issues/392
    pub unsafe fn unsafe_clone(&self) -> Self {
        Self {
            internal: self.internal,
            lifetime: self.lifetime,
        }
    }

    /// Get the java version that we're being executed from.
    pub fn get_version(&self) -> Result<JNIVersion> {
        Ok(jni_unchecked!(self.internal, GetVersion).into())
    }

    /// Load a class from a buffer of raw class data. The name of the class must match the name
    /// encoded within the class file data.
    pub fn define_class<S>(
        &mut self,
        name: S,
        loader: &JObject,
        buf: &[u8],
    ) -> Result<JClass<'local>>
    where
        S: Into<JNIString>,
    {
        let name = name.into();
        self.define_class_impl(name.as_ptr(), loader, buf)
    }

    /// Load a class from a buffer of raw class data. The name of the class is inferred from the
    /// buffer.
    pub fn define_unnamed_class(&mut self, loader: &JObject, buf: &[u8]) -> Result<JClass<'local>> {
        self.define_class_impl(ptr::null(), loader, buf)
    }

    // Note: This requires `&mut` because it might invoke a method on a user-defined `ClassLoader`.
    fn define_class_impl(
        &mut self,
        name: *const c_char,
        loader: &JObject,
        buf: &[u8],
    ) -> Result<JClass<'local>> {
        let class = jni_non_null_call!(
            self.internal,
            DefineClass,
            name,
            loader.as_raw(),
            buf.as_ptr() as *const jbyte,
            buf.len() as jsize
        );
        Ok(unsafe { JClass::from_raw(class) })
    }

    /// Load a class from a buffer of raw class data. The name of the class must match the name
    /// encoded within the class file data.
    pub fn define_class_bytearray<S>(
        &mut self,
        name: S,
        loader: &JObject,
        buf: &AutoElements<'_, '_, '_, jbyte>,
    ) -> Result<JClass<'local>>
    where
        S: Into<JNIString>,
    {
        let name = name.into();
        let class = jni_non_null_call!(
            self.internal,
            DefineClass,
            name.as_ptr(),
            loader.as_raw(),
            buf.as_ptr(),
            buf.len() as _
        );
        Ok(unsafe { JClass::from_raw(class) })
    }

    /// Look up a class by name.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv, objects::JClass};
    /// #
    /// # fn example<'local>(env: &mut JNIEnv<'local>) -> Result<()> {
    /// let class: JClass<'local> = env.find_class("java/lang/String")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn find_class<S>(&mut self, name: S) -> Result<JClass<'local>>
    where
        S: Into<JNIString>,
    {
        let name = name.into();
        let class = jni_non_null_call!(self.internal, FindClass, name.as_ptr());
        Ok(unsafe { JClass::from_raw(class) })
    }

    /// Returns the superclass for a particular class. Returns None for `java.lang.Object` or
    /// an interface. As with [Self::find_class], takes a descriptor
    ///
    /// # Errors
    ///
    /// If a JNI call fails
    pub fn get_superclass<'other_local, T>(&mut self, class: T) -> Result<Option<JClass<'local>>>
    where
        T: Desc<'local, JClass<'other_local>>,
    {
        let class = class.lookup(self)?;
        let superclass = unsafe {
            JClass::from_raw(jni_unchecked!(
                self.internal,
                GetSuperclass,
                class.as_ref().as_raw()
            ))
        };

        Ok((!superclass.is_null()).then_some(superclass))
    }

    /// Tests whether class1 is assignable from class2.
    pub fn is_assignable_from<'other_local_1, 'other_local_2, T, U>(
        &mut self,
        class1: T,
        class2: U,
    ) -> Result<bool>
    where
        T: Desc<'local, JClass<'other_local_1>>,
        U: Desc<'local, JClass<'other_local_2>>,
    {
        let class1 = class1.lookup(self)?;
        let class2 = class2.lookup(self)?;
        Ok(jni_unchecked!(
            self.internal,
            IsAssignableFrom,
            class1.as_ref().as_raw(),
            class2.as_ref().as_raw()
        ) == sys::JNI_TRUE)
    }

    /// Returns true if the object reference can be cast to the given type.
    ///
    /// _NB: Unlike the operator `instanceof`, function `IsInstanceOf` *returns `true`*
    /// for all classes *if `object` is `null`.*_
    ///
    /// See [JNI documentation](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html#IsInstanceOf)
    /// for details.
    pub fn is_instance_of<'other_local_1, 'other_local_2, O, T>(
        &mut self,
        object: O,
        class: T,
    ) -> Result<bool>
    where
        O: AsRef<JObject<'other_local_1>>,
        T: Desc<'local, JClass<'other_local_2>>,
    {
        let class = class.lookup(self)?;
        Ok(jni_unchecked!(
            self.internal,
            IsInstanceOf,
            object.as_ref().as_raw(),
            class.as_ref().as_raw()
        ) == sys::JNI_TRUE)
    }

    /// Returns true if ref1 and ref2 refer to the same Java object, or are both `NULL`. Otherwise,
    /// returns false.
    pub fn is_same_object<'other_local_1, 'other_local_2, O, T>(
        &self,
        ref1: O,
        ref2: T,
    ) -> Result<bool>
    where
        O: AsRef<JObject<'other_local_1>>,
        T: AsRef<JObject<'other_local_2>>,
    {
        Ok(jni_unchecked!(
            self.internal,
            IsSameObject,
            ref1.as_ref().as_raw(),
            ref2.as_ref().as_raw()
        ) == sys::JNI_TRUE)
    }

    /// Raise an exception from an existing object. This will continue being
    /// thrown in java unless `exception_clear` is called.
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// env.throw(("java/lang/Exception", "something bad happened"))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Defaulting to "java/lang/Exception":
    ///
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// env.throw("something bad happened")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn throw<'other_local, E>(&mut self, obj: E) -> Result<()>
    where
        E: Desc<'local, JThrowable<'other_local>>,
    {
        let throwable = obj.lookup(self)?;
        let res: i32 = jni_unchecked!(self.internal, Throw, throwable.as_ref().as_raw());

        // Ensure that `throwable` isn't dropped before the JNI call returns.
        drop(throwable);

        if res == 0 {
            Ok(())
        } else {
            Err(Error::ThrowFailed(res))
        }
    }

    /// Create and throw a new exception from a class descriptor and an error
    /// message.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// env.throw_new("java/lang/Exception", "something bad happened")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn throw_new<'other_local, S, T>(&mut self, class: T, msg: S) -> Result<()>
    where
        S: Into<JNIString>,
        T: Desc<'local, JClass<'other_local>>,
    {
        let class = class.lookup(self)?;
        let msg = msg.into();
        let res: i32 = jni_unchecked!(
            self.internal,
            ThrowNew,
            class.as_ref().as_raw(),
            msg.as_ptr()
        );

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        if res == 0 {
            Ok(())
        } else {
            Err(Error::ThrowFailed(res))
        }
    }

    /// Check whether or not an exception is currently in the process of being
    /// thrown. An exception is in this state from the time it gets thrown and
    /// not caught in a java function until `exception_clear` is called.
    pub fn exception_occurred(&mut self) -> Result<JThrowable<'local>> {
        let throwable = jni_unchecked!(self.internal, ExceptionOccurred);
        Ok(unsafe { JThrowable::from_raw(throwable) })
    }

    /// Print exception information to the console.
    pub fn exception_describe(&self) -> Result<()> {
        jni_unchecked!(self.internal, ExceptionDescribe);
        Ok(())
    }

    /// Clear an exception in the process of being thrown. If this is never
    /// called, the exception will continue being thrown when control is
    /// returned to java.
    pub fn exception_clear(&self) -> Result<()> {
        jni_unchecked!(self.internal, ExceptionClear);
        Ok(())
    }

    /// Abort the JVM with an error message.
    #[allow(unused_variables, unreachable_code)]
    pub fn fatal_error<S: Into<JNIString>>(&self, msg: S) -> ! {
        let msg = msg.into();
        let res: Result<()> = catch!({
            jni_unchecked!(self.internal, FatalError, msg.as_ptr());
            unreachable!()
        });

        panic!("{:?}", res.unwrap_err());
    }

    /// Check to see if an exception is being thrown. This only differs from
    /// `exception_occurred` in that it doesn't return the actual thrown
    /// exception.
    pub fn exception_check(&self) -> Result<bool> {
        let check = jni_unchecked!(self.internal, ExceptionCheck) == sys::JNI_TRUE;
        Ok(check)
    }

    /// Create a new instance of a direct java.nio.ByteBuffer
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// let buf = vec![0; 1024 * 1024];
    /// let (addr, len) = { // (use buf.into_raw_parts() on nightly)
    ///     let buf = buf.leak();
    ///     (buf.as_mut_ptr(), buf.len())
    /// };
    /// let direct_buffer = unsafe { env.new_direct_byte_buffer(addr, len) }?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Safety
    ///
    /// Expects a valid (non-null) pointer and length
    ///
    /// Caller must ensure the lifetime of `data` extends to all uses of the returned
    /// `ByteBuffer`. The JVM may maintain references to the `ByteBuffer` beyond the lifetime
    /// of this `JNIEnv`.
    pub unsafe fn new_direct_byte_buffer(
        &mut self,
        data: *mut u8,
        len: usize,
    ) -> Result<JByteBuffer<'local>> {
        non_null!(data, "new_direct_byte_buffer data argument");
        let obj = jni_non_null_call!(
            self.internal,
            NewDirectByteBuffer,
            data as *mut c_void,
            len as jlong
        );
        Ok(JByteBuffer::from_raw(obj))
    }

    /// Returns the starting address of the memory of the direct
    /// java.nio.ByteBuffer.
    pub fn get_direct_buffer_address(&self, buf: &JByteBuffer) -> Result<*mut u8> {
        non_null!(buf, "get_direct_buffer_address argument");
        let ptr = jni_unchecked!(self.internal, GetDirectBufferAddress, buf.as_raw());
        non_null!(ptr, "get_direct_buffer_address return value");
        Ok(ptr as _)
    }

    /// Returns the capacity (length) of the direct java.nio.ByteBuffer.
    ///
    /// # Terminology
    ///
    /// "capacity" here means the length that was passed to [`Self::new_direct_byte_buffer()`]
    /// which does not reflect the (potentially) larger size of the underlying allocation (unlike the `Vec`
    /// API).
    ///
    /// The terminology is simply kept from the original JNI API (`GetDirectBufferCapacity`).
    pub fn get_direct_buffer_capacity(&self, buf: &JByteBuffer) -> Result<usize> {
        non_null!(buf, "get_direct_buffer_capacity argument");
        let capacity = jni_unchecked!(self.internal, GetDirectBufferCapacity, buf.as_raw());
        match capacity {
            -1 => Err(Error::JniCall(JniError::Unknown)),
            _ => Ok(capacity as usize),
        }
    }

    /// Turns an object into a global ref. This has the benefit of removing the
    /// lifetime bounds since it's guaranteed to not get GC'd by java. It
    /// releases the GC pin upon being dropped.
    pub fn new_global_ref<'other_local, O>(&self, obj: O) -> Result<GlobalRef>
    where
        O: AsRef<JObject<'other_local>>,
    {
        let jvm = self.get_java_vm()?;
        let new_ref = jni_unchecked!(self.internal, NewGlobalRef, obj.as_ref().as_raw());
        let global = unsafe { GlobalRef::from_raw(jvm, new_ref) };
        Ok(global)
    }

    /// Creates a new [weak global reference][WeakRef].
    ///
    /// If the provided object is null, this method returns `None`. Otherwise, it returns `Some`
    /// containing the new weak global reference.
    pub fn new_weak_ref<'other_local, O>(&self, obj: O) -> Result<Option<WeakRef>>
    where
        O: AsRef<JObject<'other_local>>,
    {
        // We need the `JavaVM` in order to construct a `WeakRef` below. But because `get_java_vm`
        // is fallible, we need to call it before doing anything else, so that we don't leak
        // memory if it fails.
        let vm = self.get_java_vm()?;

        let obj = obj.as_ref().as_raw();

        // Check if the pointer is null *before* calling `NewWeakGlobalRef`.
        //
        // This avoids a bug in some JVM implementations which, contrary to the JNI specification,
        // will throw `java.lang.OutOfMemoryError: C heap space` from `NewWeakGlobalRef` if it is
        // passed a null pointer. (The specification says it will return a null pointer in that
        // situation, not throw an exception.)
        if obj.is_null() {
            return Ok(None);
        }

        let weak: sys::jweak = jni_non_void_call!(self.internal, NewWeakGlobalRef, obj);

        // Check if the pointer returned by `NewWeakGlobalRef` is null. This can happen if `obj` is
        // itself a weak reference that was already garbage collected.
        if weak.is_null() {
            return Ok(None);
        }

        let weak = unsafe { WeakRef::from_raw(vm, weak) };

        Ok(Some(weak))
    }

    /// Create a new local reference to an object.
    ///
    /// Specifically, this calls the JNI function [`NewLocalRef`], which creates a reference in the
    /// current local reference frame, regardless of whether the original reference belongs to the
    /// same local reference frame, a different one, or is a [global reference][GlobalRef]. In Rust
    /// terms, this method accepts a JNI reference with any valid lifetime and produces a clone of
    /// that reference with the lifetime of this `JNIEnv`. The returned reference can outlive the
    /// original.
    ///
    /// This method is useful when you have a strong global reference and you can't prevent it from
    /// being dropped before you're finished with it. In that case, you can use this method to
    /// create a new local reference that's guaranteed to remain valid for the duration of the
    /// current local reference frame, regardless of what later happens to the original global
    /// reference.
    ///
    /// # Lifetimes
    ///
    /// `'local` is the lifetime of the local reference frame that this `JNIEnv` belongs to. This
    /// method creates a new local reference in that frame, with lifetime `'local`.
    ///
    /// `'other_local` is the lifetime of the original reference's frame. It can be any valid
    /// lifetime, even one that `'local` outlives or vice versa.
    ///
    /// Think of `'local` as meaning `'new` and `'other_local` as meaning `'original`. (It is
    /// unfortunately not possible to actually give these names to the two lifetimes because
    /// `'local` is a parameter to the `JNIEnv` type, not a parameter to this method.)
    ///
    /// # Example
    ///
    /// In the following example, the `ExampleError::extract_throwable` method uses
    /// `JNIEnv::new_local_ref` to create a new local reference that outlives the original global
    /// reference:
    ///
    /// ```no_run
    /// # use jni::{JNIEnv, objects::*};
    /// # use std::fmt::Display;
    /// #
    /// # type SomeOtherErrorType = Box<dyn Display>;
    /// #
    /// /// An error that may be caused by either a Java exception or something going wrong in Rust
    /// /// code.
    /// enum ExampleError {
    ///     /// This variant represents a Java exception.
    ///     ///
    ///     /// The enclosed `GlobalRef` points to a Java object of class `java.lang.Throwable`
    ///     /// (or one of its many subclasses).
    ///     Exception(GlobalRef),
    ///
    ///     /// This variant represents an error in Rust code, not a Java exception.
    ///     Other(SomeOtherErrorType),
    /// }
    ///
    /// impl ExampleError {
    ///     /// Consumes this `ExampleError` and produces a `JThrowable`, suitable for throwing
    ///     /// back to Java code.
    ///     ///
    ///     /// If this is an `ExampleError::Exception`, then this extracts the enclosed Java
    ///     /// exception object. Otherwise, a new exception object is created to represent this
    ///     /// error.
    ///     fn extract_throwable<'local>(self, env: &mut JNIEnv<'local>) -> jni::errors::Result<JThrowable<'local>> {
    ///         let throwable: JObject = match self {
    ///             ExampleError::Exception(exception) => {
    ///                 // The error was caused by a Java exception.
    ///
    ///                 // Here, `exception` is a `GlobalRef` pointing to a Java `Throwable`. It
    ///                 // will be dropped at the end of this `match` arm. We'll use
    ///                 // `new_local_ref` to create a local reference that will outlive the
    ///                 // `GlobalRef`.
    ///
    ///                 env.new_local_ref(&exception)?
    ///             }
    ///
    ///             ExampleError::Other(error) => {
    ///                 // The error was caused by something that happened in Rust code. Create a
    ///                 // new `java.lang.Error` to represent it.
    ///
    ///                 let error_string = env.new_string(error.to_string())?;
    ///
    ///                 env.new_object(
    ///                     "java/lang/Error",
    ///                     "(Ljava/lang/String;)V",
    ///                     &[
    ///                         (&error_string).into(),
    ///                     ],
    ///                 )?
    ///             }
    ///         };
    ///
    ///         Ok(JThrowable::from(throwable))
    ///     }
    /// }
    /// ```
    ///
    /// [`NewLocalRef`]: https://docs.oracle.com/en/java/javase/11/docs/specs/jni/functions.html#newlocalref
    pub fn new_local_ref<'other_local, O>(&self, obj: O) -> Result<JObject<'local>>
    where
        O: AsRef<JObject<'other_local>>,
    {
        let local = jni_unchecked!(self.internal, NewLocalRef, obj.as_ref().as_raw());
        Ok(unsafe { JObject::from_raw(local) })
    }

    /// Creates a new auto-deleted local reference.
    ///
    /// See also [`with_local_frame`](struct.JNIEnv.html#method.with_local_frame) method that
    /// can be more convenient when you create a _bounded_ number of local references
    /// but cannot rely on automatic de-allocation (e.g., in case of recursion, deep call stacks,
    /// [permanently-attached](struct.JavaVM.html#attaching-native-threads) native threads, etc.).
    pub fn auto_local<O>(&self, obj: O) -> AutoLocal<'local, O>
    where
        O: Into<JObject<'local>>,
    {
        AutoLocal::new(obj, self)
    }

    /// Deletes the local reference.
    ///
    /// Local references are valid for the duration of a native method call.
    /// They are
    /// freed automatically after the native method returns. Each local
    /// reference costs
    /// some amount of Java Virtual Machine resource. Programmers need to make
    /// sure that
    /// native methods do not excessively allocate local references. Although
    /// local
    /// references are automatically freed after the native method returns to
    /// Java,
    /// excessive allocation of local references may cause the VM to run out of
    /// memory
    /// during the execution of a native method.
    ///
    /// In most cases it is better to use `AutoLocal` (see `auto_local` method)
    /// or `with_local_frame` instead of direct `delete_local_ref` calls.
    ///
    /// `obj` can be a mutable borrow of a local reference (such as
    /// `&mut JObject`) instead of the local reference itself (such as
    /// `JObject`). In this case, the local reference will still exist after
    /// this method returns, but it will be null.
    pub fn delete_local_ref<'other_local, O>(&self, obj: O) -> Result<()>
    where
        O: Into<JObject<'other_local>>,
    {
        let raw = obj.into().into_raw();
        jni_unchecked!(self.internal, DeleteLocalRef, raw);
        Ok(())
    }

    /// Creates a new local reference frame, in which at least a given number
    /// of local references can be created.
    ///
    /// Returns `Err` on failure, with a pending `OutOfMemoryError`.
    ///
    /// Prefer to use
    /// [`with_local_frame`](struct.JNIEnv.html#method.with_local_frame)
    /// instead of direct `push_local_frame`/`pop_local_frame` calls.
    ///
    /// See also [`auto_local`](struct.JNIEnv.html#method.auto_local) method
    /// and `AutoLocal` type — that approach can be more convenient in loops.
    pub fn push_local_frame(&self, capacity: i32) -> Result<()> {
        // This method is safe to call in case of pending exceptions (see chapter 2 of the spec)
        let res = jni_unchecked!(self.internal, PushLocalFrame, capacity);
        jni_error_code_to_result(res)
    }

    /// Pops off the current local reference frame, frees all the local
    /// references allocated on the current stack frame, except the `result`,
    /// which is returned from this function and remains valid.
    ///
    /// The resulting `JObject` will be `NULL` iff `result` is `NULL`.
    ///
    /// This method allows direct control of local frames, but it can cause
    /// undefined behavior and is therefore unsafe. Prefer
    /// [`JNIEnv::with_local_frame`] instead.
    ///
    /// # Safety
    ///
    /// Any local references created after the most recent call to
    /// [`JNIEnv::push_local_frame`] (or the underlying JNI function) must not
    /// be used after calling this method.
    pub unsafe fn pop_local_frame(&self, result: &JObject) -> Result<JObject<'local>> {
        // This method is safe to call in case of pending exceptions (see chapter 2 of the spec)
        Ok(JObject::from_raw(jni_unchecked!(
            self.internal,
            PopLocalFrame,
            result.as_raw()
        )))
    }

    /// Executes the given function in a new local reference frame, in which at least a given number
    /// of references can be created. Once this method returns, all references allocated
    /// in the frame are freed.
    ///
    /// If a frame can't be allocated with the requested capacity for local
    /// references, returns `Err` with a pending `OutOfMemoryError`.
    ///
    /// Since local references created within this frame won't be accessible to the calling
    /// frame then if you need to pass an object back to the caller then you can do that via a
    /// [`GlobalRef`] / [`Self::make_global`].
    pub fn with_local_frame<F, T, E>(&mut self, capacity: i32, f: F) -> std::result::Result<T, E>
    where
        F: FnOnce(&mut JNIEnv) -> std::result::Result<T, E>,
        E: From<Error>,
    {
        unsafe {
            self.push_local_frame(capacity)?;
            let ret = f(self);
            self.pop_local_frame(&JObject::null())?;
            ret
        }
    }

    /// Executes the given function in a new local reference frame, in which at least a given number
    /// of references can be created. Once this method returns, all references allocated
    /// in the frame are freed, except the one that the function returns, which remains valid.
    ///
    /// If a frame can't be allocated with the requested capacity for local
    /// references, returns `Err` with a pending `OutOfMemoryError`.
    ///
    /// Since the low-level JNI interface has support for passing back a single local reference
    /// from a local frame as special-case optimization, this alternative to `with_local_frame`
    /// exposes that capability to return a local reference without needing to create a
    /// temporary [`GlobalRef`].
    pub fn with_local_frame_returning_local<F, E>(
        &mut self,
        capacity: i32,
        f: F,
    ) -> std::result::Result<JObject<'local>, E>
    where
        F: for<'new_local> FnOnce(
            &mut JNIEnv<'new_local>,
        ) -> std::result::Result<JObject<'new_local>, E>,
        E: From<Error>,
    {
        unsafe {
            self.push_local_frame(capacity)?;
            match f(self) {
                Ok(obj) => {
                    let obj = self.pop_local_frame(&obj)?;
                    Ok(obj)
                }
                Err(err) => {
                    self.pop_local_frame(&JObject::null())?;
                    Err(err)
                }
            }
        }
    }

    /// Allocates a new object from a class descriptor without running a
    /// constructor.
    pub fn alloc_object<'other_local, T>(&mut self, class: T) -> Result<JObject<'local>>
    where
        T: Desc<'local, JClass<'other_local>>,
    {
        let class = class.lookup(self)?;
        let obj = jni_non_null_call!(self.internal, AllocObject, class.as_ref().as_raw());

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        Ok(unsafe { JObject::from_raw(obj) })
    }

    /// Common functionality for finding methods.
    #[allow(clippy::redundant_closure_call)]
    fn get_method_id_base<'other_local_1, T, U, V, C, R>(
        &mut self,
        class: T,
        name: U,
        sig: V,
        get_method: C,
    ) -> Result<R>
    where
        T: Desc<'local, JClass<'other_local_1>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
        C: for<'other_local_2> Fn(
            &mut Self,
            &JClass<'other_local_2>,
            &JNIString,
            &JNIString,
        ) -> Result<R>,
    {
        let class = class.lookup(self)?;
        let ffi_name = name.into();
        let sig = sig.into();

        let res: Result<R> = catch!({ get_method(self, class.as_ref(), &ffi_name, &sig) });

        match res {
            Ok(m) => Ok(m),
            Err(e) => match e {
                Error::NullPtr(_) => {
                    let name: String = ffi_name.into();
                    let sig: String = sig.into();
                    Err(Error::MethodNotFound { name, sig })
                }
                _ => Err(e),
            },
        }
    }

    /// Look up a method by class descriptor, name, and
    /// signature.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv, objects::JMethodID};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// let method_id: JMethodID =
    ///     env.get_method_id("java/lang/String", "substring", "(II)Ljava/lang/String;")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_method_id<'other_local, T, U, V>(
        &mut self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JMethodID>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        self.get_method_id_base(class, name, sig, |env, class, name, sig| {
            let method_id = jni_non_null_call!(
                env.internal,
                GetMethodID,
                class.as_raw(),
                name.as_ptr(),
                sig.as_ptr()
            );
            Ok(unsafe { JMethodID::from_raw(method_id) })
        })
    }

    /// Look up a static method by class descriptor, name, and
    /// signature.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv, objects::JStaticMethodID};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// let method_id: JStaticMethodID =
    ///     env.get_static_method_id("java/lang/String", "valueOf", "(I)Ljava/lang/String;")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_static_method_id<'other_local, T, U, V>(
        &mut self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JStaticMethodID>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        self.get_method_id_base(class, name, sig, |env, class, name, sig| {
            let method_id = jni_non_null_call!(
                env.internal,
                GetStaticMethodID,
                class.as_raw(),
                name.as_ptr(),
                sig.as_ptr()
            );
            Ok(unsafe { JStaticMethodID::from_raw(method_id) })
        })
    }

    /// Look up the field ID for a class/name/type combination.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv, objects::JFieldID};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// let field_id: JFieldID = env.get_field_id("com/my/Class", "intField", "I")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_field_id<'other_local, T, U, V>(
        &mut self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JFieldID>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        let class = class.lookup(self)?;
        let ffi_name = name.into();
        let ffi_sig = sig.into();

        let res: Result<JFieldID> = catch!({
            let field_id = jni_non_null_call!(
                self.internal,
                GetFieldID,
                class.as_ref().as_raw(),
                ffi_name.as_ptr(),
                ffi_sig.as_ptr()
            );
            Ok(unsafe { JFieldID::from_raw(field_id) })
        });

        match res {
            Ok(m) => Ok(m),
            Err(e) => match e {
                Error::NullPtr(_) => {
                    let name: String = ffi_name.into();
                    let sig: String = ffi_sig.into();
                    Err(Error::FieldNotFound { name, sig })
                }
                _ => Err(e),
            },
        }
    }

    /// Look up the static field ID for a class/name/type combination.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv, objects::JStaticFieldID};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// let field_id: JStaticFieldID = env.get_static_field_id("com/my/Class", "intField", "I")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_static_field_id<'other_local, T, U, V>(
        &mut self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JStaticFieldID>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        let class = class.lookup(self)?;
        let ffi_name = name.into();
        let ffi_sig = sig.into();

        let res: Result<JStaticFieldID> = catch!({
            let field_id = jni_non_null_call!(
                self.internal,
                GetStaticFieldID,
                class.as_ref().as_raw(),
                ffi_name.as_ptr(),
                ffi_sig.as_ptr()
            );
            Ok(unsafe { JStaticFieldID::from_raw(field_id) })
        });

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        match res {
            Ok(m) => Ok(m),
            Err(e) => match e {
                Error::NullPtr(_) => {
                    let name: String = ffi_name.into();
                    let sig: String = ffi_sig.into();
                    Err(Error::FieldNotFound { name, sig })
                }
                _ => Err(e),
            },
        }
    }

    /// Get the class for an object.
    pub fn get_object_class<'other_local, O>(&self, obj: O) -> Result<JClass<'local>>
    where
        O: AsRef<JObject<'other_local>>,
    {
        let obj = obj.as_ref();
        non_null!(obj, "get_object_class");
        unsafe {
            Ok(JClass::from_raw(jni_unchecked!(
                self.internal,
                GetObjectClass,
                obj.as_raw()
            )))
        }
    }

    /// Call a static method in an unsafe manner. This does nothing to check
    /// whether the method is valid to call on the class, whether the return
    /// type is correct, or whether the number of args is valid for the method.
    ///
    /// Under the hood, this simply calls the `CallStatic<Type>MethodA` method
    /// with the provided arguments.
    ///
    /// # Safety
    ///
    /// The provided JMethodID must be valid, and match the types and number of arguments, and return type.
    /// If these are incorrect, the JVM may crash. The JMethodID must also match the passed type.
    pub unsafe fn call_static_method_unchecked<'other_local, T, U>(
        &mut self,
        class: T,
        method_id: U,
        ret: ReturnType,
        args: &[jvalue],
    ) -> Result<JValueOwned<'local>>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Desc<'local, JStaticMethodID>,
    {
        let class = class.lookup(self)?;

        let method_id = method_id.lookup(self)?.as_ref().into_raw();

        let class_raw = class.as_ref().as_raw();
        let jni_args = args.as_ptr();

        // TODO clean this up
        let ret = Ok(match ret {
            ReturnType::Object | ReturnType::Array => {
                let obj = jni_non_void_call!(
                    self.internal,
                    CallStaticObjectMethodA,
                    class_raw,
                    method_id,
                    jni_args
                );
                let obj = unsafe { JObject::from_raw(obj) };
                obj.into()
            }
            ReturnType::Primitive(p) => match p {
                Primitive::Boolean => jni_non_void_call!(
                    self.internal,
                    CallStaticBooleanMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Char => jni_non_void_call!(
                    self.internal,
                    CallStaticCharMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Short => jni_non_void_call!(
                    self.internal,
                    CallStaticShortMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Int => jni_non_void_call!(
                    self.internal,
                    CallStaticIntMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Long => jni_non_void_call!(
                    self.internal,
                    CallStaticLongMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Float => jni_non_void_call!(
                    self.internal,
                    CallStaticFloatMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Double => jni_non_void_call!(
                    self.internal,
                    CallStaticDoubleMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Byte => jni_non_void_call!(
                    self.internal,
                    CallStaticByteMethodA,
                    class_raw,
                    method_id,
                    jni_args
                )
                .into(),
                Primitive::Void => {
                    jni_void_call!(
                        self.internal,
                        CallStaticVoidMethodA,
                        class_raw,
                        method_id,
                        jni_args
                    );
                    return Ok(JValueOwned::Void);
                }
            }, // JavaType::Primitive
        }); // match parsed.ret

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        ret
    }

    /// Call an object method in an unsafe manner. This does nothing to check
    /// whether the method is valid to call on the object, whether the return
    /// type is correct, or whether the number of args is valid for the method.
    ///
    /// Under the hood, this simply calls the `Call<Type>MethodA` method with
    /// the provided arguments.
    ///
    /// # Safety
    ///
    /// The provided JMethodID must be valid, and match the types and number of arguments, and return type.
    /// If these are incorrect, the JVM may crash. The JMethodID must also match the passed type.
    pub unsafe fn call_method_unchecked<'other_local, O, T>(
        &mut self,
        obj: O,
        method_id: T,
        ret: ReturnType,
        args: &[jvalue],
    ) -> Result<JValueOwned<'local>>
    where
        O: AsRef<JObject<'other_local>>,
        T: Desc<'local, JMethodID>,
    {
        let method_id = method_id.lookup(self)?.as_ref().into_raw();

        let obj = obj.as_ref().as_raw();

        let jni_args = args.as_ptr();

        // TODO clean this up
        Ok(match ret {
            ReturnType::Object | ReturnType::Array => {
                let obj =
                    jni_non_void_call!(self.internal, CallObjectMethodA, obj, method_id, jni_args);
                let obj = unsafe { JObject::from_raw(obj) };
                obj.into()
            }
            ReturnType::Primitive(p) => match p {
                Primitive::Boolean => {
                    jni_non_void_call!(self.internal, CallBooleanMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Char => {
                    jni_non_void_call!(self.internal, CallCharMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Short => {
                    jni_non_void_call!(self.internal, CallShortMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Int => {
                    jni_non_void_call!(self.internal, CallIntMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Long => {
                    jni_non_void_call!(self.internal, CallLongMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Float => {
                    jni_non_void_call!(self.internal, CallFloatMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Double => {
                    jni_non_void_call!(self.internal, CallDoubleMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Byte => {
                    jni_non_void_call!(self.internal, CallByteMethodA, obj, method_id, jni_args)
                        .into()
                }
                Primitive::Void => {
                    jni_void_call!(self.internal, CallVoidMethodA, obj, method_id, jni_args);
                    return Ok(JValueOwned::Void);
                }
            }, // JavaType::Primitive
        }) // match parsed.ret
    }

    /// Calls an object method safely. This comes with a number of
    /// lookups/checks. It
    ///
    /// * Parses the type signature to find the number of arguments and return
    ///   type
    /// * Looks up the JClass for the given object.
    /// * Looks up the JMethodID for the class/name/signature combination
    /// * Ensures that the number/types of args matches the signature
    ///   * Cannot check an object's type - but primitive types are matched against each other (including Object)
    /// * Calls `call_method_unchecked` with the verified safe arguments.
    ///
    /// Note: this may cause a Java exception if the arguments are the wrong
    /// type, in addition to if the method itself throws.
    pub fn call_method<'other_local, O, S, T>(
        &mut self,
        obj: O,
        name: S,
        sig: T,
        args: &[JValue],
    ) -> Result<JValueOwned<'local>>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        let obj = obj.as_ref();
        non_null!(obj, "call_method obj argument");

        // parse the signature
        let parsed = TypeSignature::from_str(sig.as_ref())?;
        if parsed.args.len() != args.len() {
            return Err(Error::InvalidArgList(parsed));
        }

        // check arguments types
        let base_types_match = parsed
            .args
            .iter()
            .zip(args.iter())
            .all(|(exp, act)| match exp {
                JavaType::Primitive(p) => act.primitive_type() == Some(*p),
                JavaType::Object(_) | JavaType::Array(_) => act.primitive_type().is_none(),
                JavaType::Method(_) => {
                    unreachable!("JavaType::Method(_) should not come from parsing a method sig")
                }
            });
        if !base_types_match {
            return Err(Error::InvalidArgList(parsed));
        }

        let class = self.auto_local(self.get_object_class(obj)?);

        let args: Vec<jvalue> = args.iter().map(|v| v.as_jni()).collect();

        // SAFETY: We've obtained the method_id above, so it is valid for this class.
        // We've also validated the argument counts and types using the same type signature
        // we fetched the original method ID from.
        unsafe { self.call_method_unchecked(obj, (&class, name, sig), parsed.ret, &args) }
    }

    /// Calls a static method safely. This comes with a number of
    /// lookups/checks. It
    ///
    /// * Parses the type signature to find the number of arguments and return
    ///   type
    /// * Looks up the JMethodID for the class/name/signature combination
    /// * Ensures that the number/types of args matches the signature
    ///   * Cannot check an object's type - but primitive types are matched against each other (including Object)
    /// * Calls `call_method_unchecked` with the verified safe arguments.
    ///
    /// Note: this may cause a Java exception if the arguments are the wrong
    /// type, in addition to if the method itself throws.
    pub fn call_static_method<'other_local, T, U, V>(
        &mut self,
        class: T,
        name: U,
        sig: V,
        args: &[JValue],
    ) -> Result<JValueOwned<'local>>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString> + AsRef<str>,
    {
        let parsed = TypeSignature::from_str(&sig)?;
        if parsed.args.len() != args.len() {
            return Err(Error::InvalidArgList(parsed));
        }

        // check arguments types
        let base_types_match = parsed
            .args
            .iter()
            .zip(args.iter())
            .all(|(exp, act)| match exp {
                JavaType::Primitive(p) => act.primitive_type() == Some(*p),
                JavaType::Object(_) | JavaType::Array(_) => act.primitive_type().is_none(),
                JavaType::Method(_) => {
                    unreachable!("JavaType::Method(_) should not come from parsing a method sig")
                }
            });
        if !base_types_match {
            return Err(Error::InvalidArgList(parsed));
        }

        // go ahead and look up the class since we'll need that for the next call.
        let class = class.lookup(self)?;
        let class = class.as_ref();

        let args: Vec<jvalue> = args.iter().map(|v| v.as_jni()).collect();

        // SAFETY: We've obtained the method_id above, so it is valid for this class.
        // We've also validated the argument counts and types using the same type signature
        // we fetched the original method ID from.
        unsafe { self.call_static_method_unchecked(class, (class, name, sig), parsed.ret, &args) }
    }

    /// Create a new object using a constructor. This is done safely using
    /// checks similar to those in `call_static_method`.
    pub fn new_object<'other_local, T, U>(
        &mut self,
        class: T,
        ctor_sig: U,
        ctor_args: &[JValue],
    ) -> Result<JObject<'local>>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString> + AsRef<str>,
    {
        // parse the signature
        let parsed = TypeSignature::from_str(&ctor_sig)?;

        // check arguments length
        if parsed.args.len() != ctor_args.len() {
            return Err(Error::InvalidArgList(parsed));
        }

        // check arguments types
        let base_types_match =
            parsed
                .args
                .iter()
                .zip(ctor_args.iter())
                .all(|(exp, act)| match exp {
                    JavaType::Primitive(p) => act.primitive_type() == Some(*p),
                    JavaType::Object(_) | JavaType::Array(_) => act.primitive_type().is_none(),
                    JavaType::Method(_) => {
                        unreachable!("JavaType::Method(_) should not come from parsing a ctor sig")
                    }
                });
        if !base_types_match {
            return Err(Error::InvalidArgList(parsed));
        }

        // check return value
        if parsed.ret != ReturnType::Primitive(Primitive::Void) {
            return Err(Error::InvalidCtorReturn);
        }

        // build strings
        let class = class.lookup(self)?;
        let class = class.as_ref();

        let method_id: JMethodID = Desc::<JMethodID>::lookup((class, ctor_sig), self)?;

        let ctor_args: Vec<jvalue> = ctor_args.iter().map(|v| v.as_jni()).collect();
        // SAFETY: We've obtained the method_id above, so it is valid for this class.
        // We've also validated the argument counts and types using the same type signature
        // we fetched the original method ID from.
        unsafe { self.new_object_unchecked(class, method_id, &ctor_args) }
    }

    /// Create a new object using a constructor. Arguments aren't checked
    /// because of the `JMethodID` usage.
    ///
    /// # Safety
    ///
    /// The provided JMethodID must be valid, and match the types and number of arguments, as well as return type
    /// (always an Object for a constructor). If these are incorrect, the JVM may crash.  The JMethodID must also match
    /// the passed type.
    pub unsafe fn new_object_unchecked<'other_local, T>(
        &mut self,
        class: T,
        ctor_id: JMethodID,
        ctor_args: &[jvalue],
    ) -> Result<JObject<'local>>
    where
        T: Desc<'local, JClass<'other_local>>,
    {
        let class = class.lookup(self)?;

        let jni_args = ctor_args.as_ptr();

        let obj = jni_non_null_call!(
            self.internal,
            NewObjectA,
            class.as_ref().as_raw(),
            ctor_id.into_raw(),
            jni_args
        );

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        Ok(unsafe { JObject::from_raw(obj) })
    }

    /// Cast a JObject to a `JList`. This won't throw exceptions or return errors
    /// in the event that the object isn't actually a list, but the methods on
    /// the resulting map object will.
    pub fn get_list<'other_local_1, 'obj_ref>(
        &mut self,
        obj: &'obj_ref JObject<'other_local_1>,
    ) -> Result<JList<'local, 'other_local_1, 'obj_ref>>
    where
        'other_local_1: 'obj_ref,
    {
        non_null!(obj, "get_list obj argument");
        JList::from_env(self, obj)
    }

    /// Cast a JObject to a JMap. This won't throw exceptions or return errors
    /// in the event that the object isn't actually a map, but the methods on
    /// the resulting map object will.
    pub fn get_map<'other_local_1, 'obj_ref>(
        &mut self,
        obj: &'obj_ref JObject<'other_local_1>,
    ) -> Result<JMap<'local, 'other_local_1, 'obj_ref>>
    where
        'other_local_1: 'obj_ref,
    {
        non_null!(obj, "get_map obj argument");
        JMap::from_env(self, obj)
    }

    /// Get a [`JavaStr`] from a [`JString`]. This allows conversions from java string
    /// objects to rust strings.
    ///
    /// This only entails calling `GetStringUTFChars`, which will return a [`JavaStr`] in Java's
    /// [Modified UTF-8](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8)
    /// format.
    ///
    /// This doesn't automatically decode Java's modified UTF-8 format but you
    /// can use `.into()` to convert the returned [`JavaStr`] into a Rust [`String`].
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the Object passed in is an instance of `java.lang.String`,
    /// passing in anything else will lead to undefined behaviour (The JNI implementation
    /// is likely to crash or abort the process).
    ///
    /// # Errors
    ///
    /// Returns an error if `obj` is `null`
    pub unsafe fn get_string_unchecked<'other_local: 'obj_ref, 'obj_ref>(
        &self,
        obj: &'obj_ref JString<'other_local>,
    ) -> Result<JavaStr<'local, 'other_local, 'obj_ref>> {
        non_null!(obj, "get_string obj argument");
        JavaStr::from_env(self, obj)
    }

    /// Get a [`JavaStr`] from a [`JString`]. This allows conversions from java string
    /// objects to rust strings.
    ///
    /// This entails checking that the given object is a `java.lang.String` and
    /// calling `GetStringUTFChars`, which will return a [`JavaStr`] in Java's
    /// [Modified UTF-8](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8)
    /// format.
    ///
    /// This doesn't automatically decode Java's modified UTF-8 format but you
    /// can use `.into()` to convert the returned [`JavaStr`] into a Rust [`String`].
    ///
    /// # Performance
    ///
    /// This function has a large relative performance impact compared to
    /// [Self::get_string_unchecked]. For example it may be about five times
    /// slower than `get_string_unchecked` for very short string. This
    /// performance penalty comes from the extra validation performed by this
    /// function. If and only if you can guarantee that your `obj` is of
    /// `java.lang.String`, use [Self::get_string_unchecked].
    ///
    /// # Errors
    ///
    /// Returns an error if `obj` is `null` or is not an instance of `java.lang.String`
    pub fn get_string<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        obj: &'obj_ref JString<'other_local>,
    ) -> Result<JavaStr<'local, 'other_local, 'obj_ref>> {
        let string_class = self.find_class("java/lang/String")?;
        if !self.is_assignable_from(string_class, self.get_object_class(obj)?)? {
            return Err(JniCall(JniError::InvalidArguments));
        }

        // SAFETY: We check that the passed in Object is actually a java.lang.String
        unsafe { self.get_string_unchecked(obj) }
    }

    /// Create a new java string object from a rust string. This requires a
    /// re-encoding of rusts *real* UTF-8 strings to java's modified UTF-8
    /// format.
    pub fn new_string<S: Into<JNIString>>(&self, from: S) -> Result<JString<'local>> {
        let ffi_str = from.into();
        let s = jni_non_null_call!(self.internal, NewStringUTF, ffi_str.as_ptr());
        Ok(unsafe { JString::from_raw(s) })
    }

    /// Get the length of a [`JPrimitiveArray`] or [`JObjectArray`].
    pub fn get_array_length<'other_local, 'array>(
        &self,
        array: &'array impl AsJArrayRaw<'other_local>,
    ) -> Result<jsize> {
        non_null!(array.as_jarray_raw(), "get_array_length array argument");
        let len: jsize = jni_unchecked!(self.internal, GetArrayLength, array.as_jarray_raw());
        Ok(len)
    }

    /// Construct a new array holding objects in class `element_class`.
    /// All elements are initially set to `initial_element`.
    ///
    /// This function returns a local reference, that must not be allocated
    /// excessively.
    /// See [Java documentation][1] for details.
    ///
    /// [1]: https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/design.html#global_and_local_references
    pub fn new_object_array<'other_local_1, 'other_local_2, T, U>(
        &mut self,
        length: jsize,
        element_class: T,
        initial_element: U,
    ) -> Result<JObjectArray<'local>>
    where
        T: Desc<'local, JClass<'other_local_2>>,
        U: AsRef<JObject<'other_local_1>>,
    {
        let class = element_class.lookup(self)?;

        let array: jarray = jni_non_null_call!(
            self.internal,
            NewObjectArray,
            length,
            class.as_ref().as_raw(),
            initial_element.as_ref().as_raw()
        );

        let array = unsafe { JObjectArray::from_raw(array) };

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        Ok(array)
    }

    /// Returns a local reference to an element of the [`JObjectArray`] `array`.
    pub fn get_object_array_element<'other_local>(
        &mut self,
        array: impl AsRef<JObjectArray<'other_local>>,
        index: jsize,
    ) -> Result<JObject<'local>> {
        non_null!(array.as_ref(), "get_object_array_element array argument");
        Ok(unsafe {
            JObject::from_raw(jni_non_void_call!(
                self.internal,
                GetObjectArrayElement,
                array.as_ref().as_raw(),
                index
            ))
        })
    }

    /// Sets an element of the [`JObjectArray`] `array`.
    pub fn set_object_array_element<'other_local_1, 'other_local_2>(
        &self,
        array: impl AsRef<JObjectArray<'other_local_1>>,
        index: jsize,
        value: impl AsRef<JObject<'other_local_2>>,
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_object_array_element array argument");
        jni_void_call!(
            self.internal,
            SetObjectArrayElement,
            array.as_ref().as_raw(),
            index,
            value.as_ref().as_raw()
        );
        Ok(())
    }

    /// Create a new java byte array from a rust byte slice.
    pub fn byte_array_from_slice(&self, buf: &[u8]) -> Result<JByteArray<'local>> {
        let length = buf.len() as i32;
        let bytes = self.new_byte_array(length)?;
        jni_unchecked!(
            self.internal,
            SetByteArrayRegion,
            bytes.as_raw(),
            0,
            length,
            buf.as_ptr() as *const i8
        );
        Ok(bytes)
    }

    /// Converts a java byte array to a rust vector of bytes.
    pub fn convert_byte_array<'other_local>(
        &self,
        array: impl AsRef<JByteArray<'other_local>>,
    ) -> Result<Vec<u8>> {
        let array = array.as_ref().as_raw();
        non_null!(array, "convert_byte_array array argument");
        let length = jni_non_void_call!(self.internal, GetArrayLength, array);
        let mut vec = vec![0u8; length as usize];
        jni_unchecked!(
            self.internal,
            GetByteArrayRegion,
            array,
            0,
            length,
            vec.as_mut_ptr() as *mut i8
        );
        Ok(vec)
    }

    /// Create a new java boolean array of supplied length.
    pub fn new_boolean_array(&self, length: jsize) -> Result<JBooleanArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewBooleanArray, length);
        let array = unsafe { JBooleanArray::from_raw(array) };
        Ok(array)
    }

    /// Create a new java byte array of supplied length.
    pub fn new_byte_array(&self, length: jsize) -> Result<JByteArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewByteArray, length);
        let array = unsafe { JByteArray::from_raw(array) };
        Ok(array)
    }

    /// Create a new java char array of supplied length.
    pub fn new_char_array(&self, length: jsize) -> Result<JCharArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewCharArray, length);
        let array = unsafe { JCharArray::from_raw(array) };
        Ok(array)
    }

    /// Create a new java short array of supplied length.
    pub fn new_short_array(&self, length: jsize) -> Result<JShortArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewShortArray, length);
        let array = unsafe { JShortArray::from_raw(array) };
        Ok(array)
    }

    /// Create a new java int array of supplied length.
    pub fn new_int_array(&self, length: jsize) -> Result<JIntArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewIntArray, length);
        let array = unsafe { JIntArray::from_raw(array) };
        Ok(array)
    }

    /// Create a new java long array of supplied length.
    pub fn new_long_array(&self, length: jsize) -> Result<JLongArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewLongArray, length);
        let array = unsafe { JLongArray::from_raw(array) };
        Ok(array)
    }

    /// Create a new java float array of supplied length.
    pub fn new_float_array(&self, length: jsize) -> Result<JFloatArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewFloatArray, length);
        let array = unsafe { JFloatArray::from_raw(array) };
        Ok(array)
    }

    /// Create a new java double array of supplied length.
    pub fn new_double_array(&self, length: jsize) -> Result<JDoubleArray<'local>> {
        let array: jarray = jni_non_null_call!(self.internal, NewDoubleArray, length);
        let array = unsafe { JDoubleArray::from_raw(array) };
        Ok(array)
    }

    /// Copy elements of the java boolean array from the `start` index to the
    /// `buf` slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_boolean_array_region<'other_local>(
        &self,
        array: impl AsRef<JBooleanArray<'other_local>>,
        start: jsize,
        buf: &mut [jboolean],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_boolean_array_region array argument");
        jni_void_call!(
            self.internal,
            GetBooleanArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );
        Ok(())
    }

    /// Copy elements of the java byte array from the `start` index to the `buf`
    /// slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_byte_array_region<'other_local>(
        &self,
        array: impl AsRef<JByteArray<'other_local>>,
        start: jsize,
        buf: &mut [jbyte],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_byte_array_region array argument");
        jni_void_call!(
            self.internal,
            GetByteArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );

        Ok(())
    }

    /// Copy elements of the java char array from the `start` index to the
    /// `buf` slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_char_array_region<'other_local>(
        &self,
        array: impl AsRef<JCharArray<'other_local>>,
        start: jsize,
        buf: &mut [jchar],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_char_array_region array argument");
        jni_void_call!(
            self.internal,
            GetCharArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );
        Ok(())
    }

    /// Copy elements of the java short array from the `start` index to the
    /// `buf` slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_short_array_region<'other_local>(
        &self,
        array: impl AsRef<JShortArray<'other_local>>,
        start: jsize,
        buf: &mut [jshort],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_short_array_region array argument");
        jni_void_call!(
            self.internal,
            GetShortArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );
        Ok(())
    }

    /// Copy elements of the java int array from the `start` index to the
    /// `buf` slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_int_array_region<'other_local>(
        &self,
        array: impl AsRef<JIntArray<'other_local>>,
        start: jsize,
        buf: &mut [jint],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_int_array_region array argument");
        jni_void_call!(
            self.internal,
            GetIntArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );
        Ok(())
    }

    /// Copy elements of the java long array from the `start` index to the
    /// `buf` slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_long_array_region<'other_local>(
        &self,
        array: impl AsRef<JLongArray<'other_local>>,
        start: jsize,
        buf: &mut [jlong],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_long_array_region array argument");
        jni_void_call!(
            self.internal,
            GetLongArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );
        Ok(())
    }

    /// Copy elements of the java float array from the `start` index to the
    /// `buf` slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_float_array_region<'other_local>(
        &self,
        array: impl AsRef<JFloatArray<'other_local>>,
        start: jsize,
        buf: &mut [jfloat],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_float_array_region array argument");
        jni_void_call!(
            self.internal,
            GetFloatArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );
        Ok(())
    }

    /// Copy elements of the java double array from the `start` index to the
    /// `buf` slice. The number of copied elements is equal to the `buf` length.
    ///
    /// # Errors
    /// If `start` is negative _or_ `start + buf.len()` is greater than [`array.length`]
    /// then no elements are copied, an `ArrayIndexOutOfBoundsException` is thrown,
    /// and `Err` is returned.
    ///
    /// [`array.length`]: struct.JNIEnv.html#method.get_array_length
    pub fn get_double_array_region<'other_local>(
        &self,
        array: impl AsRef<JDoubleArray<'other_local>>,
        start: jsize,
        buf: &mut [jdouble],
    ) -> Result<()> {
        non_null!(array.as_ref(), "get_double_array_region array argument");
        jni_void_call!(
            self.internal,
            GetDoubleArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_mut_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java boolean array at the
    /// `start` index.
    pub fn set_boolean_array_region<'other_local>(
        &self,
        array: impl AsRef<JBooleanArray<'other_local>>,
        start: jsize,
        buf: &[jboolean],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_boolean_array_region array argument");
        jni_void_call!(
            self.internal,
            SetBooleanArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java byte array at the
    /// `start` index.
    pub fn set_byte_array_region<'other_local>(
        &self,
        array: impl AsRef<JByteArray<'other_local>>,
        start: jsize,
        buf: &[jbyte],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_byte_array_region array argument");
        jni_void_call!(
            self.internal,
            SetByteArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java char array at the
    /// `start` index.
    pub fn set_char_array_region<'other_local>(
        &self,
        array: impl AsRef<JCharArray<'other_local>>,
        start: jsize,
        buf: &[jchar],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_char_array_region array argument");
        jni_void_call!(
            self.internal,
            SetCharArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java short array at the
    /// `start` index.
    pub fn set_short_array_region<'other_local>(
        &self,
        array: impl AsRef<JShortArray<'other_local>>,
        start: jsize,
        buf: &[jshort],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_short_array_region array argument");
        jni_void_call!(
            self.internal,
            SetShortArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java int array at the
    /// `start` index.
    pub fn set_int_array_region<'other_local>(
        &self,
        array: impl AsRef<JIntArray<'other_local>>,
        start: jsize,
        buf: &[jint],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_int_array_region array argument");
        jni_void_call!(
            self.internal,
            SetIntArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java long array at the
    /// `start` index.
    pub fn set_long_array_region<'other_local>(
        &self,
        array: impl AsRef<JLongArray<'other_local>>,
        start: jsize,
        buf: &[jlong],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_long_array_region array argument");
        jni_void_call!(
            self.internal,
            SetLongArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java float array at the
    /// `start` index.
    pub fn set_float_array_region<'other_local>(
        &self,
        array: impl AsRef<JFloatArray<'other_local>>,
        start: jsize,
        buf: &[jfloat],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_float_array_region array argument");
        jni_void_call!(
            self.internal,
            SetFloatArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Copy the contents of the `buf` slice to the java double array at the
    /// `start` index.
    pub fn set_double_array_region<'other_local>(
        &self,
        array: impl AsRef<JDoubleArray<'other_local>>,
        start: jsize,
        buf: &[jdouble],
    ) -> Result<()> {
        non_null!(array.as_ref(), "set_double_array_region array argument");
        jni_void_call!(
            self.internal,
            SetDoubleArrayRegion,
            array.as_ref().as_raw(),
            start,
            buf.len() as jsize,
            buf.as_ptr()
        );
        Ok(())
    }

    /// Get a field without checking the provided type against the actual field.
    pub fn get_field_unchecked<'other_local, O, T>(
        &mut self,
        obj: O,
        field: T,
        ty: ReturnType,
    ) -> Result<JValueOwned<'local>>
    where
        O: AsRef<JObject<'other_local>>,
        T: Desc<'local, JFieldID>,
    {
        let obj = obj.as_ref();
        non_null!(obj, "get_field_typed obj argument");

        let field = field.lookup(self)?.as_ref().into_raw();
        let obj = obj.as_raw();

        // TODO clean this up
        Ok(match ty {
            ReturnType::Object | ReturnType::Array => {
                let obj = jni_non_void_call!(self.internal, GetObjectField, obj, field);
                let obj = unsafe { JObject::from_raw(obj) };
                obj.into()
            }
            ReturnType::Primitive(p) => match p {
                Primitive::Boolean => {
                    jni_unchecked!(self.internal, GetBooleanField, obj, field).into()
                }
                Primitive::Char => jni_unchecked!(self.internal, GetCharField, obj, field).into(),
                Primitive::Short => jni_unchecked!(self.internal, GetShortField, obj, field).into(),
                Primitive::Int => jni_unchecked!(self.internal, GetIntField, obj, field).into(),
                Primitive::Long => jni_unchecked!(self.internal, GetLongField, obj, field).into(),
                Primitive::Float => jni_unchecked!(self.internal, GetFloatField, obj, field).into(),
                Primitive::Double => {
                    jni_unchecked!(self.internal, GetDoubleField, obj, field).into()
                }
                Primitive::Byte => jni_unchecked!(self.internal, GetByteField, obj, field).into(),
                Primitive::Void => {
                    return Err(Error::WrongJValueType("void", "see java field"));
                }
            },
        })
    }

    /// Set a field without any type checking.
    pub fn set_field_unchecked<'other_local, O, T>(
        &mut self,
        obj: O,
        field: T,
        val: JValue,
    ) -> Result<()>
    where
        O: AsRef<JObject<'other_local>>,
        T: Desc<'local, JFieldID>,
    {
        let obj = obj.as_ref();
        non_null!(obj, "set_field_typed obj argument");

        let field = field.lookup(self)?.as_ref().into_raw();
        let obj = obj.as_raw();

        // TODO clean this up
        match val {
            JValue::Object(o) => {
                jni_unchecked!(self.internal, SetObjectField, obj, field, o.as_raw());
            }
            // JavaType::Object
            JValue::Bool(b) => {
                jni_unchecked!(self.internal, SetBooleanField, obj, field, b);
            }
            JValue::Char(c) => {
                jni_unchecked!(self.internal, SetCharField, obj, field, c);
            }
            JValue::Short(s) => {
                jni_unchecked!(self.internal, SetShortField, obj, field, s);
            }
            JValue::Int(i) => {
                jni_unchecked!(self.internal, SetIntField, obj, field, i);
            }
            JValue::Long(l) => {
                jni_unchecked!(self.internal, SetLongField, obj, field, l);
            }
            JValue::Float(f) => {
                jni_unchecked!(self.internal, SetFloatField, obj, field, f);
            }
            JValue::Double(d) => {
                jni_unchecked!(self.internal, SetDoubleField, obj, field, d);
            }
            JValue::Byte(b) => {
                jni_unchecked!(self.internal, SetByteField, obj, field, b);
            }
            JValue::Void => {
                return Err(Error::WrongJValueType("void", "see java field"));
            }
        };

        Ok(())
    }

    /// Get a field. Requires an object class lookup and a field id lookup
    /// internally.
    pub fn get_field<'other_local, O, S, T>(
        &mut self,
        obj: O,
        name: S,
        ty: T,
    ) -> Result<JValueOwned<'local>>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        let obj = obj.as_ref();
        let class = self.auto_local(self.get_object_class(obj)?);

        let parsed = ReturnType::from_str(ty.as_ref())?;

        let field_id: JFieldID = Desc::<JFieldID>::lookup((&class, name, ty), self)?;

        self.get_field_unchecked(obj, field_id, parsed)
    }

    /// Set a field. Does the same lookups as `get_field` and ensures that the
    /// type matches the given value.
    pub fn set_field<'other_local, O, S, T>(
        &mut self,
        obj: O,
        name: S,
        ty: T,
        val: JValue,
    ) -> Result<()>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        let obj = obj.as_ref();
        let parsed = JavaType::from_str(ty.as_ref())?;
        let in_type = val.primitive_type();

        match parsed {
            JavaType::Object(_) | JavaType::Array(_) => {
                if in_type.is_some() {
                    return Err(Error::WrongJValueType(val.type_name(), "see java field"));
                }
            }
            JavaType::Primitive(p) => {
                if let Some(in_p) = in_type {
                    if in_p == p {
                        // good
                    } else {
                        return Err(Error::WrongJValueType(val.type_name(), "see java field"));
                    }
                } else {
                    return Err(Error::WrongJValueType(val.type_name(), "see java field"));
                }
            }
            JavaType::Method(_) => unimplemented!(),
        }

        let class = self.auto_local(self.get_object_class(obj)?);

        self.set_field_unchecked(obj, (&class, name, ty), val)
    }

    /// Get a static field without checking the provided type against the actual
    /// field.
    pub fn get_static_field_unchecked<'other_local, T, U>(
        &mut self,
        class: T,
        field: U,
        ty: JavaType,
    ) -> Result<JValueOwned<'local>>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Desc<'local, JStaticFieldID>,
    {
        use JavaType::Primitive as JP;

        let class = class.lookup(self)?;
        let field = field.lookup(self)?;

        let result = match ty {
            JavaType::Object(_) | JavaType::Array(_) => {
                let obj = jni_non_void_call!(
                    self.internal,
                    GetStaticObjectField,
                    class.as_ref().as_raw(),
                    field.as_ref().into_raw()
                );
                let obj = unsafe { JObject::from_raw(obj) };
                obj.into()
            }
            JavaType::Method(_) => return Err(Error::WrongJValueType("Method", "see java field")),
            JP(Primitive::Boolean) => jni_unchecked!(
                self.internal,
                GetStaticBooleanField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Char) => jni_unchecked!(
                self.internal,
                GetStaticCharField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Short) => jni_unchecked!(
                self.internal,
                GetStaticShortField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Int) => jni_unchecked!(
                self.internal,
                GetStaticIntField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Long) => jni_unchecked!(
                self.internal,
                GetStaticLongField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Float) => jni_unchecked!(
                self.internal,
                GetStaticFloatField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Double) => jni_unchecked!(
                self.internal,
                GetStaticDoubleField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Byte) => jni_unchecked!(
                self.internal,
                GetStaticByteField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw()
            )
            .into(),
            JP(Primitive::Void) => return Err(Error::WrongJValueType("void", "see java field")),
        };

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        Ok(result)
    }

    /// Get a static field. Requires a class lookup and a field id lookup
    /// internally.
    pub fn get_static_field<'other_local, T, U, V>(
        &mut self,
        class: T,
        field: U,
        sig: V,
    ) -> Result<JValueOwned<'local>>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString> + AsRef<str>,
    {
        let ty = JavaType::from_str(sig.as_ref())?;

        // go ahead and look up the class sincewe'll need that for the next
        // call.
        let class = class.lookup(self)?;

        self.get_static_field_unchecked(class.as_ref(), (class.as_ref(), field, sig), ty)
    }

    /// Set a static field. Requires a class lookup and a field id lookup internally.
    pub fn set_static_field<'other_local, T, U>(
        &mut self,
        class: T,
        field: U,
        value: JValue,
    ) -> Result<()>
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Desc<'local, JStaticFieldID>,
    {
        let class = class.lookup(self)?;
        let field = field.lookup(self)?;

        match value {
            JValue::Object(v) => jni_unchecked!(
                self.internal,
                SetStaticObjectField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw(),
                v.as_raw()
            ),
            JValue::Byte(v) => jni_unchecked!(
                self.internal,
                SetStaticByteField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw(),
                v
            ),
            JValue::Char(v) => jni_unchecked!(
                self.internal,
                SetStaticCharField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw(),
                v
            ),
            JValue::Short(v) => jni_unchecked!(
                self.internal,
                SetStaticShortField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw(),
                v
            ),
            JValue::Int(v) => jni_unchecked!(
                self.internal,
                SetStaticIntField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw(),
                v
            ),
            JValue::Long(v) => jni_unchecked!(
                self.internal,
                SetStaticLongField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw(),
                v
            ),
            JValue::Bool(v) => {
                jni_unchecked!(
                    self.internal,
                    SetStaticBooleanField,
                    class.as_ref().as_raw(),
                    field.as_ref().into_raw(),
                    v
                )
            }
            JValue::Float(v) => jni_unchecked!(
                self.internal,
                SetStaticFloatField,
                class.as_ref().as_raw(),
                field.as_ref().into_raw(),
                v
            ),
            JValue::Double(v) => {
                jni_unchecked!(
                    self.internal,
                    SetStaticDoubleField,
                    class.as_ref().as_raw(),
                    field.as_ref().into_raw(),
                    v
                )
            }
            JValue::Void => return Err(Error::WrongJValueType("void", "?")),
        }

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        Ok(())
    }

    /// Surrenders ownership of a Rust value to Java.
    ///
    /// This requires an object with a `long` field to store the pointer.
    ///
    /// In Java the property may look like:
    /// ```java
    /// private long myRustValueHandle = 0;
    /// ```
    ///
    /// Or, in Kotlin the property may look like:
    /// ```java
    /// private var myRustValueHandle: Long = 0
    /// ```
    ///
    /// _Note that `private` properties are accessible to JNI which may be
    /// preferable to avoid exposing the handles to more code than necessary
    /// (since the handles are usually only meaningful to Rust code)_.
    ///
    /// The Rust value will be implicitly wrapped in a `Box<Mutex<T>>`.
    ///
    /// The Java object will be locked while changing the field value.
    ///
    /// # Safety
    ///
    /// It's important to note that using this API will leak memory if
    /// [`Self::take_rust_field`] is never called so that the Rust type may be
    /// dropped.
    ///
    /// One suggestion that may help ensure that a set Rust field will be
    /// cleaned up later is for the Java object to implement `Closeable` and let
    /// people use a `use` block (Kotlin) or `try-with-resources` (Java).
    ///
    /// **DO NOT** make a copy of the handle stored in one of these fields
    /// since that could lead to a use-after-free error if the Rust type is
    /// taken and dropped multiple times from Rust. If you need to copy an
    /// object with one of these fields then the field should be zero
    /// initialized in the copy.
    #[allow(unused_variables)]
    pub unsafe fn set_rust_field<'other_local, O, S, T>(
        &mut self,
        obj: O,
        field: S,
        rust_object: T,
    ) -> Result<()>
    where
        O: AsRef<JObject<'other_local>>,
        S: AsRef<str>,
        T: Send + 'static,
    {
        let obj = obj.as_ref();
        let class = self.auto_local(self.get_object_class(obj)?);
        let field_id: JFieldID = Desc::<JFieldID>::lookup((&class, &field, "J"), self)?;

        let guard = self.lock_obj(obj)?;

        // Check to see if we've already set this value. If it's not null, that
        // means that we're going to leak memory if it gets overwritten.
        let field_ptr = self
            .get_field_unchecked(obj, field_id, ReturnType::Primitive(Primitive::Long))?
            .j()? as *mut Mutex<T>;
        if !field_ptr.is_null() {
            return Err(Error::FieldAlreadySet(field.as_ref().to_owned()));
        }

        let mbox = Box::new(::std::sync::Mutex::new(rust_object));
        let ptr: *mut Mutex<T> = Box::into_raw(mbox);

        self.set_field_unchecked(obj, field_id, (ptr as crate::sys::jlong).into())
    }

    /// Gets a lock on a Rust value that's been given to a Java object.
    ///
    /// Java still retains ownership and [`Self::take_rust_field`] will still
    /// need to be called at some point.
    ///
    /// The Java object will be locked before reading the field value but the
    /// Java object lock will be released after the Rust `Mutex` lock for the
    /// field value has been taken (i.e the Java object won't be locked once
    /// this function returns).
    ///
    /// # Safety
    ///
    /// Checks for a null pointer, but assumes that the data it points to is valid for T.
    #[allow(unused_variables)]
    pub unsafe fn get_rust_field<'other_local, O, S, T>(
        &mut self,
        obj: O,
        field: S,
    ) -> Result<MutexGuard<T>>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Send + 'static,
    {
        let obj = obj.as_ref();
        let guard = self.lock_obj(obj)?;

        let ptr = self.get_field(obj, field, "J")?.j()? as *mut Mutex<T>;
        non_null!(ptr, "rust value from Java");
        // dereferencing is safe, because we checked it for null
        Ok((*ptr).lock().unwrap())
    }

    /// Take a Rust field back from Java.
    ///
    /// It sets the field to a null pointer to signal that it's empty.
    ///
    /// The Java object will be locked before taking the field value.
    ///
    /// # Safety
    ///
    /// This will make sure that the pointer is non-null, but still assumes that
    /// the data it points to is valid for T.
    #[allow(unused_variables)]
    pub unsafe fn take_rust_field<'other_local, O, S, T>(&mut self, obj: O, field: S) -> Result<T>
    where
        O: AsRef<JObject<'other_local>>,
        S: AsRef<str>,
        T: Send + 'static,
    {
        let obj = obj.as_ref();
        let class = self.auto_local(self.get_object_class(obj)?);
        let field_id: JFieldID = Desc::<JFieldID>::lookup((&class, &field, "J"), self)?;

        let mbox = {
            let guard = self.lock_obj(obj)?;

            let ptr = self
                .get_field_unchecked(obj, field_id, ReturnType::Primitive(Primitive::Long))?
                .j()? as *mut Mutex<T>;

            non_null!(ptr, "rust value from Java");

            let mbox = Box::from_raw(ptr);

            // attempt to acquire the lock. This prevents us from consuming the
            // mutex if there's an outstanding lock. No one else will be able to
            // get a new one as long as we're in the guarded scope.
            drop(mbox.try_lock()?);

            self.set_field_unchecked(
                obj,
                field_id,
                (::std::ptr::null_mut::<()>() as sys::jlong).into(),
            )?;

            mbox
        };

        Ok(mbox.into_inner().unwrap())
    }

    /// Lock a Java object. The MonitorGuard that this returns is responsible
    /// for ensuring that it gets unlocked.
    pub fn lock_obj<'other_local, O>(&self, obj: O) -> Result<MonitorGuard<'local>>
    where
        O: AsRef<JObject<'other_local>>,
    {
        let inner = obj.as_ref().as_raw();
        let _ = jni_unchecked!(self.internal, MonitorEnter, inner);

        Ok(MonitorGuard {
            obj: inner,
            env: self.internal,
            life: Default::default(),
        })
    }

    /// Returns underlying `sys::JNIEnv` interface.
    pub fn get_native_interface(&self) -> *mut sys::JNIEnv {
        self.internal
    }

    /// Returns the Java VM interface.
    pub fn get_java_vm(&self) -> Result<JavaVM> {
        let mut raw = ptr::null_mut();
        let res = jni_unchecked!(self.internal, GetJavaVM, &mut raw);
        jni_error_code_to_result(res)?;
        unsafe { JavaVM::from_raw(raw) }
    }

    /// Ensures that at least a given number of local references can be created
    /// in the current thread.
    pub fn ensure_local_capacity(&self, capacity: jint) -> Result<()> {
        jni_void_call!(self.internal, EnsureLocalCapacity, capacity);
        Ok(())
    }

    /// Bind function pointers to native methods of class
    /// according to method name and signature.
    /// For details see [documentation](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html#RegisterNatives).
    pub fn register_native_methods<'other_local, T>(
        &mut self,
        class: T,
        methods: &[NativeMethod],
    ) -> Result<()>
    where
        T: Desc<'local, JClass<'other_local>>,
    {
        let class = class.lookup(self)?;
        let jni_native_methods: Vec<JNINativeMethod> = methods
            .iter()
            .map(|nm| JNINativeMethod {
                name: nm.name.as_ptr() as *mut c_char,
                signature: nm.sig.as_ptr() as *mut c_char,
                fnPtr: nm.fn_ptr,
            })
            .collect();
        let res = jni_non_void_call!(
            self.internal,
            RegisterNatives,
            class.as_ref().as_raw(),
            jni_native_methods.as_ptr(),
            jni_native_methods.len() as jint
        );

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        jni_error_code_to_result(res)
    }

    /// Unbind all native methods of class.
    pub fn unregister_native_methods<'other_local, T>(&mut self, class: T) -> Result<()>
    where
        T: Desc<'local, JClass<'other_local>>,
    {
        let class = class.lookup(self)?;
        let res = jni_non_void_call!(self.internal, UnregisterNatives, class.as_ref().as_raw());

        // Ensure that `class` isn't dropped before the JNI call returns.
        drop(class);

        jni_error_code_to_result(res)
    }

    /// Returns an [`AutoElements`] to access the elements of the given Java `array`.
    ///
    /// The elements are accessible until the returned auto-release guard is dropped.
    ///
    /// The returned array may be a copy of the Java array and changes made to
    /// the returned array will not necessarily be reflected in the original
    /// array until the [`AutoElements`] guard is dropped.
    ///
    /// If you know in advance that you will only be reading from the array then
    /// pass [`ReleaseMode::NoCopyBack`] so that the JNI implementation knows
    /// that it's not necessary to copy any data back to the original Java array
    /// when the [`AutoElements`] guard is dropped.
    ///
    /// Since the returned array may be a copy of the Java array, changes made to the
    /// returned array will not necessarily be reflected in the original array until
    /// the corresponding `Release*ArrayElements` JNI method is called.
    /// [`AutoElements`] has a commit() method, to force a copy back of pending
    /// array changes if needed (and without releasing it).
    ///
    /// # Safety
    ///
    /// ## No data races
    ///
    /// This API has no built-in synchronization that ensures there won't be any data
    /// races while accessing the array elements.
    ///
    /// To avoid undefined behaviour it is the caller's responsibility to ensure there
    /// will be no data races between other Rust or Java threads trying to access the
    /// same array.
    ///
    /// Acquiring a [`MonitorGuard`] lock for the `array` could be one way of ensuring
    /// mutual exclusion between Rust and Java threads, so long as the Java threads
    /// also acquire the same lock via `synchronized(array) {}`.
    ///
    /// ## No aliasing
    ///
    /// Callers must not create more than one [`AutoElements`] or
    /// [`AutoElementsCritical`] per Java array at the same time - even if
    /// there is no risk of a data race.
    ///
    /// The reason for this restriction is that [`AutoElements`] and
    /// [`AutoElementsCritical`] implement `DerefMut` which can provide a
    /// mutable `&mut [T]` slice reference for the elements and it would
    /// constitute undefined behaviour to allow there to be more than one
    /// mutable reference that points to the same memory.
    ///
    /// # jboolean elements
    ///
    /// Keep in mind that arrays of `jboolean` values should only ever hold
    /// values of `0` or `1` because any other value could lead to undefined
    /// behaviour within the JVM.
    ///
    /// Also see
    /// [`get_array_elements_critical`](Self::get_array_elements_critical) which
    /// imposes additional restrictions that make it less likely to incur the
    /// cost of copying the array elements.
    pub unsafe fn get_array_elements<'other_local, 'array, T: TypeArray>(
        &mut self,
        array: &'array JPrimitiveArray<'other_local, T>,
        mode: ReleaseMode,
    ) -> Result<AutoElements<'local, 'other_local, 'array, T>> {
        non_null!(array, "get_array_elements array argument");
        AutoElements::new(self, array, mode)
    }

    /// Returns an [`AutoElementsCritical`] to access the elements of the given Java `array`.
    ///
    /// The elements are accessible during the critical section that exists until the
    /// returned auto-release guard is dropped.
    ///
    /// This API imposes some strict restrictions that help the JNI implementation
    /// avoid any need to copy the underlying array elements before making them
    /// accessible to native code:
    ///
    /// 1. No other use of JNI calls are allowed (on the same thread) within the critical
    /// section that exists while holding the [`AutoElementsCritical`] guard.
    /// 2. No system calls can be made (Such as `read`) that may depend on a result
    /// from another Java thread.
    ///
    /// The JNI spec does not specify what will happen if these rules aren't adhered to
    /// but it should be assumed it will lead to undefined behaviour, likely deadlock
    /// and possible program termination.
    ///
    /// Even with these restrictions the returned array may still be a copy of
    /// the Java array and changes made to the returned array will not
    /// necessarily be reflected in the original array until the [`AutoElementsCritical`]
    /// guard is dropped.
    ///
    /// If you know in advance that you will only be reading from the array then
    /// pass [`ReleaseMode::NoCopyBack`] so that the JNI implementation knows
    /// that it's not necessary to copy any data back to the original Java array
    /// when the [`AutoElementsCritical`] guard is dropped.
    ///
    /// A nested scope or explicit use of `std::mem::drop` can be used to
    /// control when the returned [`AutoElementsCritical`] is dropped to
    /// minimize the length of the critical section.
    ///
    /// If the given array is `null`, an `Error::NullPtr` is returned.
    ///
    /// # Safety
    ///
    /// ## Critical Section Restrictions
    ///
    /// Although this API takes a mutable reference to a [`JNIEnv`] which should
    /// ensure that it's not possible to call JNI, this API is still marked as
    /// `unsafe` due to the complex, far-reaching nature of the critical-section
    /// restrictions imposed here that can't be guaranteed simply through Rust's
    /// borrow checker rules.
    ///
    /// The rules above about JNI usage and system calls _must_ be adhered to.
    ///
    /// Using this API implies:
    ///
    /// 1. All garbage collection will likely be paused during the critical section
    /// 2. Any use of JNI in other threads may block if they need to allocate memory
    ///    (due to the garbage collector being paused)
    /// 3. Any use of system calls that will wait for a result from another Java thread
    ///    could deadlock if that other thread is blocked by a paused garbage collector.
    ///
    /// A failure to adhere to the critical section rules could lead to any
    /// undefined behaviour, including aborting the program.
    ///
    /// ## No data races
    ///
    /// This API has no built-in synchronization that ensures there won't be any data
    /// races while accessing the array elements.
    ///
    /// To avoid undefined behaviour it is the caller's responsibility to ensure there
    /// will be no data races between other Rust or Java threads trying to access the
    /// same array.
    ///
    /// Acquiring a [`MonitorGuard`] lock for the `array` could be one way of ensuring
    /// mutual exclusion between Rust and Java threads, so long as the Java threads
    /// also acquire the same lock via `synchronized(array) {}`.
    ///
    /// ## No aliasing
    ///
    /// Callers must not create more than one [`AutoElements`] or
    /// [`AutoElementsCritical`] per Java array at the same time - even if
    /// there is no risk of a data race.
    ///
    /// The reason for this restriction is that [`AutoElements`] and
    /// [`AutoElementsCritical`] implement `DerefMut` which can provide a
    /// mutable `&mut [T]` slice reference for the elements and it would
    /// constitute undefined behaviour to allow there to be more than one
    /// mutable reference that points to the same memory.
    ///
    /// ## jboolean elements
    ///
    /// Keep in mind that arrays of `jboolean` values should only ever hold
    /// values of `0` or `1` because any other value could lead to undefined
    /// behaviour within the JVM.
    ///
    /// Also see [`get_array_elements`](Self::get_array_elements) which has fewer
    /// restrictions, but is is more likely to incur a cost from copying the
    /// array elements.
    pub unsafe fn get_array_elements_critical<'other_local, 'array, 'env, T: TypeArray>(
        &'env mut self,
        array: &'array JPrimitiveArray<'other_local, T>,
        mode: ReleaseMode,
    ) -> Result<AutoElementsCritical<'local, 'other_local, 'array, 'env, T>> {
        non_null!(array, "get_primitive_array_critical array argument");
        AutoElementsCritical::new(self, array, mode)
    }
}

/// Native method descriptor.
pub struct NativeMethod {
    /// Name of method.
    pub name: JNIString,
    /// Method signature.
    pub sig: JNIString,
    /// Pointer to native function with signature
    /// `fn(env: JNIEnv, class: JClass, ...arguments according to sig) -> RetType`
    /// for static methods or
    /// `fn(env: JNIEnv, object: JObject, ...arguments according to sig) -> RetType`
    /// for instance methods.
    pub fn_ptr: *mut c_void,
}

/// Guard for a lock on a java object. This gets returned from the `lock_obj`
/// method.
pub struct MonitorGuard<'local> {
    obj: sys::jobject,
    env: *mut sys::JNIEnv,
    life: PhantomData<&'local ()>,
}

impl<'local> Drop for MonitorGuard<'local> {
    fn drop(&mut self) {
        let res: Result<()> = catch!({
            jni_unchecked!(self.env, MonitorExit, self.obj);
            Ok(())
        });

        if let Err(e) = res {
            warn!("error releasing java monitor: {}", e)
        }
    }
}
