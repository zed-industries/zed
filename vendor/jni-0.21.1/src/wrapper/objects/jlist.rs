use crate::{
    errors::*,
    objects::{AutoLocal, JClass, JMethodID, JObject, JValue},
    signature::{Primitive, ReturnType},
    sys::jint,
    JNIEnv,
};

use std::marker::PhantomData;

/// Wrapper for JObjects that implement `java/util/List`. Provides methods to get,
/// add, and remove elements.
///
/// Looks up the class and method ids on creation rather than for every method
/// call.
pub struct JList<'local, 'other_local_1: 'obj_ref, 'obj_ref> {
    internal: &'obj_ref JObject<'other_local_1>,
    _phantom_class: PhantomData<AutoLocal<'local, JClass<'local>>>,
    get: JMethodID,
    add: JMethodID,
    add_idx: JMethodID,
    remove: JMethodID,
    size: JMethodID,
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> AsRef<JList<'local, 'other_local_1, 'obj_ref>>
    for JList<'local, 'other_local_1, 'obj_ref>
{
    fn as_ref(&self) -> &JList<'local, 'other_local_1, 'obj_ref> {
        self
    }
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> AsRef<JObject<'other_local_1>>
    for JList<'local, 'other_local_1, 'obj_ref>
{
    fn as_ref(&self) -> &JObject<'other_local_1> {
        self.internal
    }
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> JList<'local, 'other_local_1, 'obj_ref> {
    /// Create a map from the environment and an object. This looks up the
    /// necessary class and method ids to call all of the methods on it so that
    /// exra work doesn't need to be done on every method call.
    pub fn from_env(
        env: &mut JNIEnv<'local>,
        obj: &'obj_ref JObject<'other_local_1>,
    ) -> Result<JList<'local, 'other_local_1, 'obj_ref>> {
        let class = AutoLocal::new(env.find_class("java/util/List")?, env);

        let get = env.get_method_id(&class, "get", "(I)Ljava/lang/Object;")?;
        let add = env.get_method_id(&class, "add", "(Ljava/lang/Object;)Z")?;
        let add_idx = env.get_method_id(&class, "add", "(ILjava/lang/Object;)V")?;
        let remove = env.get_method_id(&class, "remove", "(I)Ljava/lang/Object;")?;
        let size = env.get_method_id(&class, "size", "()I")?;

        Ok(JList {
            internal: obj,
            _phantom_class: PhantomData,
            get,
            add,
            add_idx,
            remove,
            size,
        })
    }

    /// Look up the value for a key. Returns `Some` if it's found and `None` if
    /// a null pointer would be returned.
    pub fn get<'other_local_2>(
        &self,
        env: &mut JNIEnv<'other_local_2>,
        idx: jint,
    ) -> Result<Option<JObject<'other_local_2>>> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a JObject/null, rather than another primitive type.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.get,
                ReturnType::Object,
                &[JValue::from(idx).as_jni()],
            )
        };

        match result {
            Ok(val) => Ok(Some(val.l()?)),
            Err(e) => match e {
                Error::NullPtr(_) => Ok(None),
                _ => Err(e),
            },
        }
    }

    /// Append an element to the list
    pub fn add(&self, env: &mut JNIEnv, value: &JObject) -> Result<()> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a JObject/null, rather than another primitive type.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.add,
                ReturnType::Primitive(Primitive::Boolean),
                &[JValue::from(value).as_jni()],
            )
        };

        let _ = result?;
        Ok(())
    }

    /// Insert an element at a specific index
    pub fn insert(&self, env: &mut JNIEnv, idx: jint, value: &JObject) -> Result<()> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a JObject/null, rather than another primitive type.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.add_idx,
                ReturnType::Primitive(Primitive::Void),
                &[JValue::from(idx).as_jni(), JValue::from(value).as_jni()],
            )
        };

        let _ = result?;
        Ok(())
    }

    /// Remove an element from the list by index
    pub fn remove<'other_local_2>(
        &self,
        env: &mut JNIEnv<'other_local_2>,
        idx: jint,
    ) -> Result<Option<JObject<'other_local_2>>> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a int, rather than any other java type.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.remove,
                ReturnType::Object,
                &[JValue::from(idx).as_jni()],
            )
        };

        match result {
            Ok(val) => Ok(Some(val.l()?)),
            Err(e) => match e {
                Error::NullPtr(_) => Ok(None),
                _ => Err(e),
            },
        }
    }

    /// Get the size of the list
    pub fn size(&self, env: &mut JNIEnv) -> Result<jint> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.size,
                ReturnType::Primitive(Primitive::Int),
                &[],
            )
        };

        result.and_then(|v| v.i())
    }

    /// Pop the last element from the list
    ///
    /// Note that this calls `size()` to determine the last index.
    pub fn pop<'other_local_2>(
        &self,
        env: &mut JNIEnv<'other_local_2>,
    ) -> Result<Option<JObject<'other_local_2>>> {
        let size = self.size(env)?;
        if size == 0 {
            return Ok(None);
        }

        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a int.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.remove,
                ReturnType::Object,
                &[JValue::from(size - 1).as_jni()],
            )
        };

        match result {
            Ok(val) => Ok(Some(val.l()?)),
            Err(e) => match e {
                Error::NullPtr(_) => Ok(None),
                _ => Err(e),
            },
        }
    }

    /// Get key/value iterator for the map. This is done by getting the
    /// `EntrySet` from java and iterating over it.
    ///
    /// The returned iterator does not implement [`std::iter::Iterator`] and
    /// cannot be used with a `for` loop. This is because its `next` method
    /// uses a `&mut JNIEnv` to call the Java iterator. Use a `while let` loop
    /// instead:
    ///
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv, objects::{AutoLocal, JList, JObject}};
    /// #
    /// # fn example(env: &mut JNIEnv, list: JList) -> Result<()> {
    /// let mut iterator = list.iter(env)?;
    ///
    /// while let Some(obj) = iterator.next(env)? {
    ///     let obj: AutoLocal<JObject> = env.auto_local(obj);
    ///
    ///     // Do something with `obj` here.
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Each call to `next` creates a new local reference. To prevent excessive
    /// memory usage or overflow error, the local reference should be deleted
    /// using [`JNIEnv::delete_local_ref`] or [`JNIEnv::auto_local`] before the
    /// next loop iteration. Alternatively, if the list is known to have a
    /// small, predictable size, the loop could be wrapped in
    /// [`JNIEnv::with_local_frame`] to delete all of the local references at
    /// once.
    pub fn iter<'list>(
        &'list self,
        env: &mut JNIEnv,
    ) -> Result<JListIter<'list, 'local, 'obj_ref, 'other_local_1>> {
        Ok(JListIter {
            list: self,
            current: 0,
            size: self.size(env)?,
        })
    }
}

/// An iterator over the keys and values in a `java.util.List`. See
/// [`JList::iter`] for more information.
///
/// TODO: make the iterator implementation for java iterators its own thing
/// and generic enough to use elsewhere.
pub struct JListIter<'list, 'local, 'other_local_1: 'obj_ref, 'obj_ref> {
    list: &'list JList<'local, 'other_local_1, 'obj_ref>,
    current: jint,
    size: jint,
}

impl<'list, 'local, 'other_local_1: 'obj_ref, 'obj_ref>
    JListIter<'list, 'local, 'other_local_1, 'obj_ref>
{
    /// Advances the iterator and returns the next object in the
    /// `java.util.List`, or `None` if there are no more objects.
    ///
    /// See [`JList::iter`] for more information.
    ///
    /// This method creates a new local reference. To prevent excessive memory
    /// usage or overflow error, the local reference should be deleted using
    /// [`JNIEnv::delete_local_ref`] or [`JNIEnv::auto_local`] before the next
    /// loop iteration. Alternatively, if the list is known to have a small,
    /// predictable size, the loop could be wrapped in
    /// [`JNIEnv::with_local_frame`] to delete all of the local references at
    /// once.
    ///
    /// This method returns:
    ///
    /// * `Ok(Some(_))`: if there was another object in the list.
    /// * `Ok(None)`: if there are no more objects in the list.
    /// * `Err(_)`: if there was an error calling the Java method to
    ///   get the next object.
    ///
    /// This is like [`std::iter::Iterator::next`], but requires a parameter of
    /// type `&mut JNIEnv` in order to call into Java.
    pub fn next<'other_local_2>(
        &mut self,
        env: &mut JNIEnv<'other_local_2>,
    ) -> Result<Option<JObject<'other_local_2>>> {
        if self.current == self.size {
            return Ok(None);
        }

        let res = self.list.get(env, self.current);

        self.current = match &res {
            Ok(Some(_)) => self.current + 1,
            Ok(None) => self.current,
            Err(_) => self.size,
        };

        res
    }
}
