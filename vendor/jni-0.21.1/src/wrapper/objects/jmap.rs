use crate::{
    errors::*,
    objects::{AutoLocal, JClass, JMethodID, JObject, JValue},
    signature::{Primitive, ReturnType},
    JNIEnv,
};

use std::marker::PhantomData;

/// Wrapper for JObjects that implement `java/util/Map`. Provides methods to get
/// and set entries and a way to iterate over key/value pairs.
///
/// Looks up the class and method ids on creation rather than for every method
/// call.
pub struct JMap<'local, 'other_local_1: 'obj_ref, 'obj_ref> {
    internal: &'obj_ref JObject<'other_local_1>,
    class: AutoLocal<'local, JClass<'local>>,
    get: JMethodID,
    put: JMethodID,
    remove: JMethodID,
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> AsRef<JMap<'local, 'other_local_1, 'obj_ref>>
    for JMap<'local, 'other_local_1, 'obj_ref>
{
    fn as_ref(&self) -> &JMap<'local, 'other_local_1, 'obj_ref> {
        self
    }
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> AsRef<JObject<'other_local_1>>
    for JMap<'local, 'other_local_1, 'obj_ref>
{
    fn as_ref(&self) -> &JObject<'other_local_1> {
        self.internal
    }
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> JMap<'local, 'other_local_1, 'obj_ref> {
    /// Create a map from the environment and an object. This looks up the
    /// necessary class and method ids to call all of the methods on it so that
    /// exra work doesn't need to be done on every method call.
    pub fn from_env(
        env: &mut JNIEnv<'local>,
        obj: &'obj_ref JObject<'other_local_1>,
    ) -> Result<JMap<'local, 'other_local_1, 'obj_ref>> {
        let class = AutoLocal::new(env.find_class("java/util/Map")?, env);

        let get = env.get_method_id(&class, "get", "(Ljava/lang/Object;)Ljava/lang/Object;")?;
        let put = env.get_method_id(
            &class,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;\
             )Ljava/lang/Object;",
        )?;

        let remove =
            env.get_method_id(&class, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;")?;

        Ok(JMap {
            internal: obj,
            class,
            get,
            put,
            remove,
        })
    }

    /// Look up the value for a key. Returns `Some` if it's found and `None` if
    /// a null pointer would be returned.
    pub fn get<'other_local_2>(
        &self,
        env: &mut JNIEnv<'other_local_2>,
        key: &JObject,
    ) -> Result<Option<JObject<'other_local_2>>> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a JObject/null, rather than another primitive type.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.get,
                ReturnType::Object,
                &[JValue::from(key).as_jni()],
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

    /// Look up the value for a key. Returns `Some` with the old value if the
    /// key already existed and `None` if it's a new key.
    pub fn put<'other_local_2>(
        &self,
        env: &mut JNIEnv<'other_local_2>,
        key: &JObject,
        value: &JObject,
    ) -> Result<Option<JObject<'other_local_2>>> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a JObject/null, rather than another primitive type.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.put,
                ReturnType::Object,
                &[JValue::from(key).as_jni(), JValue::from(value).as_jni()],
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

    /// Remove a value from the map. Returns `Some` with the removed value and
    /// `None` if there was no value for the key.
    pub fn remove<'other_local_2>(
        &self,
        env: &mut JNIEnv<'other_local_2>,
        key: &JObject,
    ) -> Result<Option<JObject<'other_local_2>>> {
        // SAFETY: We keep the class loaded, and fetched the method ID for this function.
        // Provided argument is statically known as a JObject/null, rather than another primitive type.
        let result = unsafe {
            env.call_method_unchecked(
                self.internal,
                self.remove,
                ReturnType::Object,
                &[JValue::from(key).as_jni()],
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
    /// # use jni::{errors::Result, JNIEnv, objects::{AutoLocal, JMap, JObject}};
    /// #
    /// # fn example(env: &mut JNIEnv, map: JMap) -> Result<()> {
    /// let mut iterator = map.iter(env)?;
    ///
    /// while let Some((key, value)) = iterator.next(env)? {
    ///     let key: AutoLocal<JObject> = env.auto_local(key);
    ///     let value: AutoLocal<JObject> = env.auto_local(value);
    ///
    ///     // Do something with `key` and `value` here.
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Each call to `next` creates two new local references. To prevent
    /// excessive memory usage or overflow error, the local references should
    /// be deleted using [`JNIEnv::delete_local_ref`] or [`JNIEnv::auto_local`]
    /// before the next loop iteration. Alternatively, if the map is known to
    /// have a small, predictable size, the loop could be wrapped in
    /// [`JNIEnv::with_local_frame`] to delete all of the local references at
    /// once.
    pub fn iter<'map, 'iter_local>(
        &'map self,
        env: &mut JNIEnv<'iter_local>,
    ) -> Result<JMapIter<'map, 'local, 'other_local_1, 'obj_ref, 'iter_local>> {
        let iter_class = AutoLocal::new(env.find_class("java/util/Iterator")?, env);

        let has_next = env.get_method_id(&iter_class, "hasNext", "()Z")?;

        let next = env.get_method_id(&iter_class, "next", "()Ljava/lang/Object;")?;

        let entry_class = AutoLocal::new(env.find_class("java/util/Map$Entry")?, env);

        let get_key = env.get_method_id(&entry_class, "getKey", "()Ljava/lang/Object;")?;

        let get_value = env.get_method_id(&entry_class, "getValue", "()Ljava/lang/Object;")?;

        // Get the iterator over Map entries.

        // SAFETY: We keep the class loaded, and fetched the method ID for this function. Arg list is known empty.
        let entry_set = AutoLocal::new(
            unsafe {
                env.call_method_unchecked(
                    self.internal,
                    (&self.class, "entrySet", "()Ljava/util/Set;"),
                    ReturnType::Object,
                    &[],
                )
            }?
            .l()?,
            env,
        );

        // SAFETY: We keep the class loaded, and fetched the method ID for this function. Arg list is known empty.
        let iter = AutoLocal::new(
            unsafe {
                env.call_method_unchecked(
                    entry_set,
                    ("java/util/Set", "iterator", "()Ljava/util/Iterator;"),
                    ReturnType::Object,
                    &[],
                )
            }?
            .l()?,
            env,
        );

        Ok(JMapIter {
            _phantom_map: PhantomData,
            has_next,
            next,
            get_key,
            get_value,
            iter,
        })
    }
}

/// An iterator over the keys and values in a map. See [`JMap::iter`] for more
/// information.
///
/// TODO: make the iterator implementation for java iterators its own thing
/// and generic enough to use elsewhere.
pub struct JMapIter<'map, 'local, 'other_local_1: 'obj_ref, 'obj_ref, 'iter_local> {
    _phantom_map: PhantomData<&'map JMap<'local, 'other_local_1, 'obj_ref>>,
    has_next: JMethodID,
    next: JMethodID,
    get_key: JMethodID,
    get_value: JMethodID,
    iter: AutoLocal<'iter_local, JObject<'iter_local>>,
}

impl<'map, 'local, 'other_local_1: 'obj_ref, 'obj_ref, 'iter_local>
    JMapIter<'map, 'local, 'other_local_1, 'obj_ref, 'iter_local>
{
    /// Advances the iterator and returns the next key-value pair in the
    /// `java.util.Map`, or `None` if there are no more objects.
    ///
    /// See [`JMap::iter`] for more information.
    ///
    /// This method creates two new local references. To prevent excessive
    /// memory usage or overflow error, the local references should be deleted
    /// using [`JNIEnv::delete_local_ref`] or [`JNIEnv::auto_local`] before the
    /// next loop iteration. Alternatively, if the map is known to have a
    /// small, predictable size, the loop could be wrapped in
    /// [`JNIEnv::with_local_frame`] to delete all of the local references at
    /// once.
    ///
    /// This method returns:
    ///
    /// * `Ok(Some(_))`: if there was another key-value pair in the map.
    /// * `Ok(None)`: if there are no more key-value pairs in the map.
    /// * `Err(_)`: if there was an error calling the Java method to
    ///   get the next key-value pair.
    ///
    /// This is like [`std::iter::Iterator::next`], but requires a parameter of
    /// type `&mut JNIEnv` in order to call into Java.
    pub fn next<'other_local_2>(
        &mut self,
        env: &mut JNIEnv<'other_local_2>,
    ) -> Result<Option<(JObject<'other_local_2>, JObject<'other_local_2>)>> {
        // SAFETY: We keep the class loaded, and fetched the method ID for these functions. We know none expect args.

        let has_next = unsafe {
            env.call_method_unchecked(
                &self.iter,
                self.has_next,
                ReturnType::Primitive(Primitive::Boolean),
                &[],
            )
        }?
        .z()?;

        if !has_next {
            return Ok(None);
        }
        let next =
            unsafe { env.call_method_unchecked(&self.iter, self.next, ReturnType::Object, &[]) }?
                .l()?;
        let next = env.auto_local(next);

        let key =
            unsafe { env.call_method_unchecked(&next, self.get_key, ReturnType::Object, &[]) }?
                .l()?;

        let value =
            unsafe { env.call_method_unchecked(&next, self.get_value, ReturnType::Object, &[]) }?
                .l()?;

        Ok(Some((key, value)))
    }
}
