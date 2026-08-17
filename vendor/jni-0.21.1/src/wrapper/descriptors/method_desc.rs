use crate::{
    descriptors::Desc,
    errors::*,
    objects::{JClass, JMethodID, JStaticMethodID},
    strings::JNIString,
    JNIEnv,
};

unsafe impl<'local, 'other_local, T, U, V> Desc<'local, JMethodID> for (T, U, V)
where
    T: Desc<'local, JClass<'other_local>>,
    U: Into<JNIString>,
    V: Into<JNIString>,
{
    type Output = JMethodID;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        env.get_method_id(self.0, self.1, self.2)
    }
}

unsafe impl<'local, 'other_local, T, Signature> Desc<'local, JMethodID> for (T, Signature)
where
    T: Desc<'local, JClass<'other_local>>,
    Signature: Into<JNIString>,
{
    type Output = JMethodID;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Desc::<JMethodID>::lookup((self.0, "<init>", self.1), env)
    }
}

unsafe impl<'local, 'other_local, T, U, V> Desc<'local, JStaticMethodID> for (T, U, V)
where
    T: Desc<'local, JClass<'other_local>>,
    U: Into<JNIString>,
    V: Into<JNIString>,
{
    type Output = JStaticMethodID;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        env.get_static_method_id(self.0, self.1, self.2)
    }
}
