use crate::{
    descriptors::Desc,
    errors::*,
    objects::{AutoLocal, JClass, JObject, JThrowable, JValue},
    strings::JNIString,
    JNIEnv,
};

const DEFAULT_EXCEPTION_CLASS: &str = "java/lang/RuntimeException";

unsafe impl<'local, 'other_local, C, M> Desc<'local, JThrowable<'local>> for (C, M)
where
    C: Desc<'local, JClass<'other_local>>,
    M: Into<JNIString>,
{
    type Output = AutoLocal<'local, JThrowable<'local>>;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        let jmsg: AutoLocal<JObject> = env.auto_local(env.new_string(self.1)?.into());
        let obj: JThrowable = env
            .new_object(self.0, "(Ljava/lang/String;)V", &[JValue::from(&jmsg)])?
            .into();
        Ok(env.auto_local(obj))
    }
}

unsafe impl<'local> Desc<'local, JThrowable<'local>> for Exception {
    type Output = AutoLocal<'local, JThrowable<'local>>;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Desc::<JThrowable>::lookup((self.class, self.msg), env)
    }
}

unsafe impl<'local, 'str_ref> Desc<'local, JThrowable<'local>> for &'str_ref str {
    type Output = AutoLocal<'local, JThrowable<'local>>;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Desc::<JThrowable>::lookup((DEFAULT_EXCEPTION_CLASS, self), env)
    }
}

unsafe impl<'local> Desc<'local, JThrowable<'local>> for String {
    type Output = AutoLocal<'local, JThrowable<'local>>;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Desc::<JThrowable>::lookup((DEFAULT_EXCEPTION_CLASS, self), env)
    }
}

unsafe impl<'local> Desc<'local, JThrowable<'local>> for JNIString {
    type Output = AutoLocal<'local, JThrowable<'local>>;

    fn lookup(self, env: &mut JNIEnv<'local>) -> Result<Self::Output> {
        Desc::<JThrowable>::lookup((DEFAULT_EXCEPTION_CLASS, self), env)
    }
}
