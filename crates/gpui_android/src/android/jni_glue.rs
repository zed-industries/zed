//! Shared JNI helpers used by every JNI-backed module in this crate.
//!
//! `android-activity` already manages the `JavaVM` singleton — we lean on
//! that and just hand callers an `attach_current_thread` callback context
//! plus a transient `JObject` view of the activity reference.
//!
//! All errors are `anyhow::Result` so call sites can attach context with `?`
//! and so the high-level `Platform` impl can `log::warn!()` and recover.

use android_activity::AndroidApp;
use anyhow::{Context as _, Result, anyhow};
use jni::{
    Env, JavaVM,
    objects::{JObject, JString},
    sys::{JavaVM as RawJavaVM, jobject},
};

/// Borrows the live `JavaVM` and the `Activity` `jobject` from android-activity.
pub(crate) fn vm_and_activity(app: &AndroidApp) -> Result<(JavaVM, jobject)> {
    let vm_ptr = app.vm_as_ptr();
    if vm_ptr.is_null() {
        return Err(anyhow!("AndroidApp::vm_as_ptr returned null"));
    }
    // SAFETY: android-activity guarantees the pointer is a live `JavaVM*` for
    // the lifetime of the process. `JavaVM::from_raw` stores the pointer in a
    // process-wide singleton, so racing with android-activity's own
    // initialisation is safe.
    let vm = unsafe { JavaVM::from_raw(vm_ptr as *mut RawJavaVM) };

    let activity_ptr = app.activity_as_ptr();
    if activity_ptr.is_null() {
        return Err(anyhow!("AndroidApp::activity_as_ptr returned null"));
    }
    Ok((vm, activity_ptr as jobject))
}

/// Run a JNI callback on the current thread, attaching to the JVM if needed.
/// `f` receives the `Env<'_>` and the activity's `JObject`.
pub(crate) fn with_activity<R>(
    app: &AndroidApp,
    f: impl for<'local> FnOnce(&mut Env<'local>, &JObject<'local>) -> Result<R>,
) -> Result<R> {
    let (vm, activity) = vm_and_activity(app)?;
    vm.attach_current_thread(|env| {
        // SAFETY: `activity` is the global JNI ref kept alive by
        // android-activity for the whole process; we wrap it in a transient
        // `JObject` to use `Env`'s call_method machinery without taking
        // ownership.
        let activity_obj = unsafe { JObject::from_raw(env, activity) };
        f(env, &activity_obj)
    })
}

/// Convenience: read a Java `String` out of a `&JObject` whose runtime type is
/// `java.lang.String`.
pub(crate) fn java_string_to_rust<'local>(
    env: &mut Env<'local>,
    obj: JObject<'local>,
) -> Result<String> {
    if obj.is_null() {
        return Ok(String::new());
    }
    let jstring: JString<'_> = env
        .cast_local::<JString>(obj)
        .context("expected java.lang.String")?;
    if jstring.is_null() {
        return Ok(String::new());
    }
    let mutf8 = jstring
        .mutf8_chars(env)
        .context("MUTF-8 decode of Java String")?;
    Ok(mutf8.to_str().into_owned())
}

/// Allocate a Java `byte[]` populated from a Rust slice.
#[allow(dead_code)]
pub(crate) fn new_byte_array<'local>(
    env: &mut Env<'local>,
    bytes: &[u8],
) -> Result<jni::objects::JByteArray<'local>> {
    env.byte_array_from_slice(bytes).context("alloc Java byte[]")
}

/// Read a Java `byte[]` into a Rust `Vec<u8>`.
#[allow(dead_code)]
pub(crate) fn byte_array_to_vec<'local>(
    env: &Env<'local>,
    array: &jni::objects::JByteArray<'local>,
) -> Result<Vec<u8>> {
    if array.is_null() {
        return Ok(Vec::new());
    }
    let len = array.len(env).context("byte[].length")? as usize;
    let mut signed = vec![0i8; len];
    array
        .get_region(env, 0, &mut signed)
        .context("byte[] copy out")?;
    Ok(signed.into_iter().map(|b| b as u8).collect())
}
