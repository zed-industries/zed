#![cfg(feature = "invocation")]
#![feature(test)]

extern crate test;

use jni_sys::jvalue;
use lazy_static::lazy_static;

use jni::{
    descriptors::Desc,
    objects::{JClass, JMethodID, JObject, JStaticMethodID, JValue},
    signature::{Primitive, ReturnType},
    sys::jint,
    InitArgsBuilder, JNIEnv, JNIVersion, JavaVM,
};

static CLASS_MATH: &str = "java/lang/Math";
static CLASS_OBJECT: &str = "java/lang/Object";
static CLASS_LOCAL_DATE_TIME: &str = "java/time/LocalDateTime";
static METHOD_MATH_ABS: &str = "abs";
static METHOD_OBJECT_HASH_CODE: &str = "hashCode";
static METHOD_CTOR: &str = "<init>";
static METHOD_LOCAL_DATE_TIME_OF: &str = "of";
static SIG_OBJECT_CTOR: &str = "()V";
static SIG_MATH_ABS: &str = "(I)I";
static SIG_OBJECT_HASH_CODE: &str = "()I";
static SIG_LOCAL_DATE_TIME_OF: &str = "(IIIIIII)Ljava/time/LocalDateTime;";

// 32 characters
static TEST_STRING_UNICODE: &str = "_񍷕㳧~δ򗊁᪘׷ġ˥쩽|ņ/򖕡ٶԦ萴퀉֒ٞHy󢕒%ӓ娎񢞊ăꊦȮ񳗌";

#[inline(never)]
fn native_abs(x: i32) -> i32 {
    x.abs()
}

fn jni_abs_safe(env: &mut JNIEnv, x: jint) -> jint {
    let x = JValue::from(x);
    let v = env
        .call_static_method(CLASS_MATH, METHOD_MATH_ABS, SIG_MATH_ABS, &[x])
        .unwrap();
    v.i().unwrap()
}

fn jni_hash_safe(env: &mut JNIEnv, obj: &JObject) -> jint {
    let v = env
        .call_method(obj, METHOD_OBJECT_HASH_CODE, SIG_OBJECT_HASH_CODE, &[])
        .unwrap();
    v.i().unwrap()
}

fn jni_local_date_time_of_safe<'local>(
    env: &mut JNIEnv<'local>,
    year: jint,
    month: jint,
    day_of_month: jint,
    hour: jint,
    minute: jint,
    second: jint,
    nanosecond: jint,
) -> JObject<'local> {
    let v = env
        .call_static_method(
            CLASS_LOCAL_DATE_TIME,
            METHOD_LOCAL_DATE_TIME_OF,
            SIG_LOCAL_DATE_TIME_OF,
            &[
                year.into(),
                month.into(),
                day_of_month.into(),
                hour.into(),
                minute.into(),
                second.into(),
                nanosecond.into(),
            ],
        )
        .unwrap();
    v.l().unwrap()
}

fn jni_int_call_static_unchecked<'local, C>(
    env: &mut JNIEnv<'local>,
    class: C,
    method_id: JStaticMethodID,
    x: jint,
) -> jint
where
    C: Desc<'local, JClass<'local>>,
{
    let x = JValue::from(x);
    let ret = ReturnType::Primitive(Primitive::Int);
    let v =
        unsafe { env.call_static_method_unchecked(class, method_id, ret, &[x.as_jni()]) }.unwrap();
    v.i().unwrap()
}

fn jni_int_call_unchecked<'local, M>(
    env: &mut JNIEnv<'local>,
    obj: &JObject<'local>,
    method_id: M,
) -> jint
where
    M: Desc<'local, JMethodID>,
{
    let ret = ReturnType::Primitive(Primitive::Int);
    // SAFETY: Caller retrieved method ID + class specifically for this use: Object.hashCode()I
    let v = unsafe { env.call_method_unchecked(obj, method_id, ret, &[]) }.unwrap();
    v.i().unwrap()
}

fn jni_object_call_static_unchecked<'local, C>(
    env: &mut JNIEnv<'local>,
    class: C,
    method_id: JStaticMethodID,
    args: &[jvalue],
) -> JObject<'local>
where
    C: Desc<'local, JClass<'local>>,
{
    // SAFETY: Caller retrieved method ID and constructed arguments
    let v = unsafe { env.call_static_method_unchecked(class, method_id, ReturnType::Object, args) }
        .unwrap();
    v.l().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use jni::objects::GlobalRef;
    use std::rc::Rc;
    use std::sync::Arc;
    use test::{black_box, Bencher};

    lazy_static! {
        static ref VM: JavaVM = {
            let args = InitArgsBuilder::new()
                .version(JNIVersion::V8)
                .build()
                .unwrap();
            JavaVM::new(args).unwrap()
        };
    }

    #[bench]
    fn native_call_function(b: &mut Bencher) {
        b.iter(|| {
            let _ = native_abs(black_box(-3));
        });
    }

    #[bench]
    fn jni_call_static_abs_method_safe(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();

        b.iter(|| jni_abs_safe(&mut env, -3));
    }

    #[bench]
    fn jni_call_static_abs_method_unchecked_str(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = CLASS_MATH;
        let method_id = env
            .get_static_method_id(class, METHOD_MATH_ABS, SIG_MATH_ABS)
            .unwrap();

        b.iter(|| jni_int_call_static_unchecked(&mut env, class, method_id, -3));
    }

    #[bench]
    fn jni_call_static_abs_method_unchecked_jclass(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = Desc::<JClass>::lookup(CLASS_MATH, &mut env).unwrap();
        let method_id = env
            .get_static_method_id(&class, METHOD_MATH_ABS, SIG_MATH_ABS)
            .unwrap();

        b.iter(|| jni_int_call_static_unchecked(&mut env, &class, method_id, -3));
    }

    #[bench]
    fn jni_call_static_date_time_method_safe(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        b.iter(|| {
            let obj = jni_local_date_time_of_safe(&mut env, 1, 1, 1, 1, 1, 1, 1);
            env.delete_local_ref(obj).unwrap();
        });
    }

    #[bench]
    fn jni_call_static_date_time_method_unchecked_jclass(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = Desc::<JClass>::lookup(CLASS_LOCAL_DATE_TIME, &mut env).unwrap();
        let method_id = env
            .get_static_method_id(&class, METHOD_LOCAL_DATE_TIME_OF, SIG_LOCAL_DATE_TIME_OF)
            .unwrap();

        b.iter(|| {
            let obj = jni_object_call_static_unchecked(
                &mut env,
                &class,
                method_id,
                &[
                    JValue::Int(1).as_jni(),
                    JValue::Int(1).as_jni(),
                    JValue::Int(1).as_jni(),
                    JValue::Int(1).as_jni(),
                    JValue::Int(1).as_jni(),
                    JValue::Int(1).as_jni(),
                    JValue::Int(1).as_jni(),
                ],
            );
            env.delete_local_ref(obj).unwrap();
        });
    }

    #[bench]
    fn jni_call_object_hash_method_safe(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let s = env.new_string("").unwrap();
        let obj = black_box(JObject::from(s));

        b.iter(|| jni_hash_safe(&mut env, &obj));
    }

    #[bench]
    fn jni_call_object_hash_method_unchecked(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let s = env.new_string("").unwrap();
        let obj = black_box(JObject::from(s));
        let obj_class = env.get_object_class(&obj).unwrap();
        let method_id = env
            .get_method_id(&obj_class, METHOD_OBJECT_HASH_CODE, SIG_OBJECT_HASH_CODE)
            .unwrap();

        b.iter(|| jni_int_call_unchecked(&mut env, &obj, method_id));
    }

    #[bench]
    fn jni_new_object_str(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = CLASS_OBJECT;

        b.iter(|| {
            let obj = env.new_object(class, SIG_OBJECT_CTOR, &[]).unwrap();
            env.delete_local_ref(obj).unwrap();
        });
    }

    #[bench]
    fn jni_new_object_by_id_str(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = CLASS_OBJECT;
        let ctor_id = env
            .get_method_id(class, METHOD_CTOR, SIG_OBJECT_CTOR)
            .unwrap();

        b.iter(|| {
            let obj = unsafe { env.new_object_unchecked(class, ctor_id, &[]) }.unwrap();
            env.delete_local_ref(obj).unwrap();
        });
    }

    #[bench]
    fn jni_new_object_jclass(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = Desc::<JClass>::lookup(CLASS_OBJECT, &mut env).unwrap();

        b.iter(|| {
            let obj = env.new_object(&class, SIG_OBJECT_CTOR, &[]).unwrap();
            env.delete_local_ref(obj).unwrap();
        });
    }

    #[bench]
    fn jni_new_object_by_id_jclass(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = Desc::<JClass>::lookup(CLASS_OBJECT, &mut env).unwrap();
        let ctor_id = env
            .get_method_id(&class, METHOD_CTOR, SIG_OBJECT_CTOR)
            .unwrap();

        b.iter(|| {
            let obj = unsafe { env.new_object_unchecked(&class, ctor_id, &[]) }.unwrap();
            env.delete_local_ref(obj).unwrap();
        });
    }

    #[bench]
    fn jni_new_global_ref(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = CLASS_OBJECT;
        let obj = env.new_object(class, SIG_OBJECT_CTOR, &[]).unwrap();
        let global_ref = env.new_global_ref(&obj).unwrap();
        env.delete_local_ref(obj).unwrap();

        b.iter(|| env.new_global_ref(&global_ref).unwrap());
    }

    /// Checks the overhead of checking if exception has occurred.
    ///
    /// Such checks are required each time a Java method is called, but
    /// can be omitted if we call a JNI method that returns an error status.
    ///
    /// See also #58
    #[bench]
    fn jni_check_exception(b: &mut Bencher) {
        let env = VM.attach_current_thread().unwrap();

        b.iter(|| env.exception_check().unwrap());
    }

    #[bench]
    fn jni_get_java_vm(b: &mut Bencher) {
        let env = VM.attach_current_thread().unwrap();

        b.iter(|| {
            let _jvm = env.get_java_vm().unwrap();
        });
    }

    #[bench]
    fn jni_get_string(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let string = env.new_string(TEST_STRING_UNICODE).unwrap();

        b.iter(|| {
            let s: String = env.get_string(&string).unwrap().into();
            assert_eq!(s, TEST_STRING_UNICODE);
        });
    }

    #[bench]
    fn jni_get_string_unchecked(b: &mut Bencher) {
        let env = VM.attach_current_thread().unwrap();
        let string = env.new_string(TEST_STRING_UNICODE).unwrap();

        b.iter(|| {
            let s: String = unsafe { env.get_string_unchecked(&string) }.unwrap().into();
            assert_eq!(s, TEST_STRING_UNICODE);
        });
    }

    /// A benchmark measuring Push/PopLocalFrame overhead.
    ///
    /// Such operations are *required* if one attaches a long-running
    /// native thread to the JVM because there is no 'return-from-native-method'
    /// event when created local references are freed, hence no way for
    /// the JVM to know that the local references are no longer used in the native code.
    #[bench]
    fn jni_noop_with_local_frame(b: &mut Bencher) {
        // Local frame size actually doesn't matter since JVM does not preallocate anything.
        const LOCAL_FRAME_SIZE: i32 = 32;
        let mut env = VM.attach_current_thread().unwrap();
        b.iter(|| {
            env.with_local_frame(LOCAL_FRAME_SIZE, |_| -> Result<_, jni::errors::Error> {
                Ok(())
            })
            .unwrap()
        });
    }

    /// A benchmark measuring Push/PopLocalFrame overhead while retuning a local reference
    #[bench]
    fn jni_with_local_frame_returning_local(b: &mut Bencher) {
        // Local frame size actually doesn't matter since JVM does not preallocate anything.
        const LOCAL_FRAME_SIZE: i32 = 32;
        let mut env = VM.attach_current_thread().unwrap();

        let class = env.find_class(CLASS_OBJECT).unwrap();
        b.iter(|| {
            env.with_local_frame_returning_local(LOCAL_FRAME_SIZE, |env| {
                env.new_object(&class, SIG_OBJECT_CTOR, &[])
            })
        });
    }

    /// A benchmark measuring Push/PopLocalFrame overhead while retuning a global
    /// object reference that then gets converted into a local reference before
    /// dropping the global
    #[bench]
    fn jni_with_local_frame_returning_global_to_local(b: &mut Bencher) {
        // Local frame size actually doesn't matter since JVM does not preallocate anything.
        const LOCAL_FRAME_SIZE: i32 = 32;
        let mut env = VM.attach_current_thread().unwrap();

        let class = env.find_class(CLASS_OBJECT).unwrap();
        b.iter(|| {
            let global = env
                .with_local_frame::<_, GlobalRef, jni::errors::Error>(LOCAL_FRAME_SIZE, |env| {
                    let local = env.new_object(&class, SIG_OBJECT_CTOR, &[])?;
                    let global = env.new_global_ref(local)?;
                    Ok(global)
                })
                .unwrap();
            let _local = env.new_local_ref(global).unwrap();
        });
    }

    /// A benchmark of the overhead of attaching and detaching a native thread.
    ///
    /// It is *huge* — two orders of magnitude higher than calling a single
    /// Java method using unchecked APIs (e.g., `jni_call_static_unchecked`).
    ///
    #[bench]
    fn jvm_noop_attach_detach_native_thread(b: &mut Bencher) {
        b.iter(|| {
            let env = VM.attach_current_thread().unwrap();
            black_box(&env);
        });
    }

    #[bench]
    fn native_arc(b: &mut Bencher) {
        let mut env = VM.attach_current_thread().unwrap();
        let class = CLASS_OBJECT;
        let obj = env.new_object(class, SIG_OBJECT_CTOR, &[]).unwrap();
        let global_ref = env.new_global_ref(&obj).unwrap();
        env.delete_local_ref(obj).unwrap();
        let arc = Arc::new(global_ref);

        b.iter(|| {
            let _ = black_box(Arc::clone(&arc));
        });
    }

    #[bench]
    fn native_rc(b: &mut Bencher) {
        let _env = VM.attach_current_thread().unwrap();
        let env = VM.get_env().unwrap();
        let rc = Rc::new(env);

        b.iter(|| {
            let _ = black_box(Rc::clone(&rc));
        });
    }
}
