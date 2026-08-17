use crate::{
    errors::*,
    objects::{AutoLocal, JObject},
    JNIEnv,
};

#[cfg(doc)]
use crate::objects::{JClass, JMethodID};

/// Trait for things that can be looked up through the JNI via a descriptor.
/// This will be something like the fully-qualified class name
/// `java/lang/String` or a tuple containing a class descriptor, method name,
/// and method signature. For convenience, this is also implemented for the
/// concrete types themselves in addition to their descriptors.
///
/// # Safety
///
/// Implementations of this trait must return the correct value from the
/// `lookup` method. It must not, for example, return a random [`JMethodID`] or
/// the [`JClass`] of a class other than the one requested. Returning such an
/// incorrect value results in undefined behavior. This requirement also
/// applies to the returned value's implementation of `AsRef<T>`.
pub unsafe trait Desc<'local, T> {
    /// The type that this `Desc` returns.
    type Output: AsRef<T>;

    /// Look up the concrete type from the JVM.
    ///
    /// Note that this method does not return exactly `T`. Instead, it returns
    /// some type that implements `AsRef<T>`. For this reason, it is often
    /// necessary to use turbofish syntax when calling this method:
    ///
    /// ```rust,no_run
    /// # use jni::{descriptors::Desc, errors::Result, JNIEnv, objects::JClass};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// // The value returned by `lookup` is not exactly `JClass`.
    /// let class/*: impl AsRef<JClass> */ =
    ///     Desc::<JClass>::lookup("java/lang/Object", env)?;
    ///
    /// // But `&JClass` can be borrowed from it.
    /// let class: &JClass = class.as_ref();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// **Warning:** Many built-in implementations of this trait return
    /// [`AutoLocal`] from this method. If you then call [`JObject::as_raw`] on
    /// the returned object reference, this may result in the reference being
    /// [deleted][JNIEnv::delete_local_ref] before it is used, causing
    /// undefined behavior.
    ///
    /// For example, don't do this:
    ///
    /// ```rust,no_run
    /// # use jni::{descriptors::Desc, errors::Result, JNIEnv, objects::JClass};
    /// #
    /// # fn some_function<T>(ptr: *mut T) {}
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// // Undefined behavior: the `JClass` is dropped before the raw pointer
    /// // is passed to `some_function`!
    /// some_function(Desc::<JClass>::lookup("java/lang/Object", env)?.as_raw());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Instead, do this:
    ///
    /// ```rust,no_run
    /// # use jni::{descriptors::Desc, errors::Result, JNIEnv, objects::JClass};
    /// #
    /// # fn some_function<T>(ptr: *mut T) {}
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// let class = Desc::<JClass>::lookup("java/lang/Object", env)?;
    ///
    /// some_function(class.as_raw());
    ///
    /// drop(class);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// This will still work without the call to `drop` at the end, but calling
    /// `drop` ensures that the reference is not accidentally dropped earlier
    /// than it should be.
    fn lookup(self, _: &mut JNIEnv<'local>) -> Result<Self::Output>;
}

unsafe impl<'local, T> Desc<'local, T> for T
where
    T: AsRef<T>,
{
    type Output = Self;

    fn lookup(self, _: &mut JNIEnv<'local>) -> Result<T> {
        Ok(self)
    }
}

unsafe impl<'local, 't_ref, T> Desc<'local, T> for &'t_ref T
where
    T: AsRef<T>,
{
    type Output = Self;

    fn lookup(self, _: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Ok(self)
    }
}

unsafe impl<'local, 'other_local, T> Desc<'local, T> for AutoLocal<'other_local, T>
where
    T: AsRef<T> + Into<JObject<'other_local>>,
{
    type Output = Self;

    fn lookup(self, _: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Ok(self)
    }
}

unsafe impl<'local, 'other_local, 'obj_ref, T> Desc<'local, T>
    for &'obj_ref AutoLocal<'other_local, T>
where
    T: AsRef<T> + Into<JObject<'other_local>>,
{
    type Output = Self;

    fn lookup(self, _: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Ok(self)
    }
}
