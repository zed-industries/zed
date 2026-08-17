use jni::sys::jobject;
use ndk_context::AndroidContext;
use std::sync::Arc;

pub use jni::Executor;

pub use jni::{
    errors::Result as JResult,
    objects::{JIntArray, JObject, JObjectArray, JString},
    JNIEnv, JavaVM,
};

pub fn get_context() -> AndroidContext {
    ndk_context::android_context()
}

pub fn with_attached<F, R>(context: AndroidContext, closure: F) -> JResult<R>
where
    for<'j> F: FnOnce(&mut JNIEnv<'j>, JObject<'j>) -> JResult<R>,
{
    let vm = Arc::new(unsafe { JavaVM::from_raw(context.vm().cast())? });
    let context = context.context();
    let context = unsafe { JObject::from_raw(context as jobject) };
    Executor::new(vm).with_attached(|env| closure(env, context))
}

pub fn call_method_no_args_ret_int_array<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    method: &str,
) -> JResult<Vec<i32>> {
    let array: JIntArray = env.call_method(subject, method, "()[I", &[])?.l()?.into();

    let length = env.get_array_length(&array)?;
    let mut values = Vec::with_capacity(length as usize);

    env.get_int_array_region(array, 0, values.as_mut())?;

    Ok(values)
}

pub fn call_method_no_args_ret_int<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    method: &str,
) -> JResult<i32> {
    env.call_method(subject, method, "()I", &[])?.i()
}

pub fn call_method_no_args_ret_bool<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    method: &str,
) -> JResult<bool> {
    env.call_method(subject, method, "()Z", &[])?.z()
}

pub fn call_method_no_args_ret_string<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    method: &str,
) -> JResult<JString<'j>> {
    Ok(env
        .call_method(subject, method, "()Ljava/lang/String;", &[])?
        .l()?
        .into())
}

pub fn call_method_no_args_ret_char_sequence<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    method: &str,
) -> JResult<JString<'j>> {
    let cseq = env
        .call_method(subject, method, "()Ljava/lang/CharSequence;", &[])?
        .l()?;

    Ok(env
        .call_method(&cseq, "toString", "()Ljava/lang/String;", &[])?
        .l()?
        .into())
}

pub fn call_method_string_arg_ret_bool<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    name: &str,
    arg: impl AsRef<str>,
) -> JResult<bool> {
    env.call_method(
        subject,
        name,
        "(Ljava/lang/String;)Z",
        &[(&env.new_string(arg)?).into()],
    )?
    .z()
}

pub fn call_method_string_arg_ret_string<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    name: &str,
    arg: impl AsRef<str>,
) -> JResult<JString<'j>> {
    Ok(env
        .call_method(
            subject,
            name,
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[(&env.new_string(arg)?).into()],
        )?
        .l()?
        .into())
}

pub fn call_method_string_arg_ret_object<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    method: &str,
    arg: &str,
) -> JResult<JObject<'j>> {
    env.call_method(
        subject,
        method,
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[(&env.new_string(arg)?).into()],
    )?
    .l()
}

pub fn get_package_manager<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
) -> JResult<JObject<'j>> {
    env.call_method(
        subject,
        "getPackageManager",
        "()Landroid/content/pm/PackageManager;",
        &[],
    )?
    .l()
}

pub fn has_system_feature<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    name: &str,
) -> JResult<bool> {
    call_method_string_arg_ret_bool(env, subject, "hasSystemFeature", name)
}

pub fn get_system_service<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    name: &str,
) -> JResult<JObject<'j>> {
    call_method_string_arg_ret_object(env, subject, "getSystemService", name)
}

pub fn get_property<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    name: &str,
) -> JResult<JString<'j>> {
    call_method_string_arg_ret_string(env, subject, "getProperty", name)
}

pub fn get_devices<'j>(
    env: &mut JNIEnv<'j>,
    subject: &JObject<'j>,
    flags: i32,
) -> JResult<JObjectArray<'j>> {
    env.call_method(
        subject,
        "getDevices",
        "(I)[Landroid/media/AudioDeviceInfo;",
        &[flags.into()],
    )?
    .l()
    .map(From::from)
}
