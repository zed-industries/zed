use android_activity::{AndroidApp, AndroidAppWaker};
use gpui::{DevicePixels, Edges};
use jni::objects::JObject;
use jni::sys::jint;
use jni::{EnvUnowned, NativeMethod, jni_sig, jni_str};
use parking_lot::Mutex;
use std::ffi::c_void;
use std::sync::OnceLock;

/// The window's system insets in physical pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SystemInsets {
    /// System bars and display cutouts.
    pub(crate) safe_area: Edges<DevicePixels>,
    /// The soft keyboard.
    pub(crate) ime: Edges<DevicePixels>,
}

/// Insets the activity has reported that the window has yet to take.
static REPORTED: Mutex<Option<SystemInsets>> = Mutex::new(None);
static MAIN_THREAD: OnceLock<AndroidAppWaker> = OnceLock::new();

/// Returns the insets reported since the last call, if any.
pub(crate) fn take_reported() -> Option<SystemInsets> {
    REPORTED.lock().take()
}

/// Asks the activity to report its window's insets, now and on every change.
///
/// Targeting SDK 35 forces edge-to-edge, where the window is drawn under the
/// system bars, and Android sends a native activity no event when its insets
/// change, the soft keyboard opening included. `GpuiActivity` listens on its
/// decor view and reports through a native method registered here; a plain
/// `NativeActivity` has neither, and reports no insets.
pub(crate) fn watch(app: &AndroidApp) {
    MAIN_THREAD.set(app.create_waker()).ok();
    // SAFETY: android-activity guarantees `vm_as_ptr` returns the process's
    // valid `JavaVM` pointer for the lifetime of the app.
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr().cast()) };
    let activity_ptr = app.activity_as_ptr();
    let watching = vm.attach_current_thread(|env| {
        // SAFETY: `activity_as_ptr` is a global reference to the activity
        // that lives as long as the app. `JObject` does not delete the
        // reference on drop.
        let activity = unsafe { JObject::from_raw(env, activity_ptr as jni::sys::jobject) };
        let activity_class = env.get_object_class(&activity)?;
        // SAFETY: `on_insets_changed` is an instance method taking eight
        // `int`s and returning nothing, as the signature says.
        unsafe {
            env.register_native_methods(
                &activity_class,
                &[NativeMethod::from_raw_parts(
                    jni_str!("onInsetsChanged"),
                    jni_str!("(IIIIIIII)V"),
                    on_insets_changed as *mut c_void,
                )],
            )?;
        }
        env.call_method(&activity, jni_str!("watchInsets"), jni_sig!("()V"), &[])?;
        anyhow::Ok(())
    });
    if let Err(error) = watching {
        log::error!("failed to watch window insets; is the activity a GpuiActivity? {error:#}");
    }
}

extern "system" fn on_insets_changed<'local>(
    _env: EnvUnowned<'local>,
    _activity: JObject<'local>,
    safe_area_left: jint,
    safe_area_top: jint,
    safe_area_right: jint,
    safe_area_bottom: jint,
    ime_left: jint,
    ime_top: jint,
    ime_right: jint,
    ime_bottom: jint,
) {
    *REPORTED.lock() = Some(SystemInsets {
        safe_area: Edges {
            top: DevicePixels(safe_area_top),
            right: DevicePixels(safe_area_right),
            bottom: DevicePixels(safe_area_bottom),
            left: DevicePixels(safe_area_left),
        },
        ime: Edges {
            top: DevicePixels(ime_top),
            right: DevicePixels(ime_right),
            bottom: DevicePixels(ime_bottom),
            left: DevicePixels(ime_left),
        },
    });
    if let Some(main_thread) = MAIN_THREAD.get() {
        main_thread.wake();
    }
}
