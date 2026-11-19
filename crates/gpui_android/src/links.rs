use android_activity::AndroidApp;
use jni::objects::{JObject, JString, JValue};
use jni::{jni_sig, jni_str};

/// Takes the URL the activity was most recently opened with, if any.
///
/// The URL is cleared from the activity's intent so that it is reported once.
/// A plain `NativeActivity` only ever has the URL that launched it; one that
/// arrives later is seen when the activity is `GpuiActivity`.
pub(crate) fn take_opened_url(app: &AndroidApp) -> Option<String> {
    // SAFETY: android-activity guarantees `vm_as_ptr` returns the process's
    // valid `JavaVM` pointer for the lifetime of the app.
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr().cast()) };
    let activity_ptr = app.activity_as_ptr();
    let url = vm.attach_current_thread(|env| {
        // SAFETY: `activity_as_ptr` is a global reference to the activity
        // that lives as long as the app. `JObject` does not delete the
        // reference on drop.
        let activity = unsafe { JObject::from_raw(env, activity_ptr as jni::sys::jobject) };
        let intent = env
            .call_method(
                &activity,
                jni_str!("getIntent"),
                jni_sig!("()Landroid/content/Intent;"),
                &[],
            )?
            .l()?;
        let url = env
            .call_method(
                &intent,
                jni_str!("getDataString"),
                jni_sig!("()Ljava/lang/String;"),
                &[],
            )?
            .l()?;
        if url.is_null() {
            return Ok(None);
        }
        let url = env.cast_local::<JString>(url)?.try_to_string(env)?;
        env.call_method(
            &intent,
            jni_str!("setData"),
            jni_sig!("(Landroid/net/Uri;)Landroid/content/Intent;"),
            &[JValue::Object(&JObject::null())],
        )?;
        Ok::<_, jni::errors::Error>(Some(url))
    });
    match url {
        Ok(url) => url,
        Err(error) => {
            log::error!("failed to read the URL the app was opened with: {error}");
            None
        }
    }
}
