use android_activity::AndroidApp;
use gpui::{DevicePixels, Edges};
use jni::objects::{JObject, JValue};
use jni::{jni_sig, jni_str};
use parking_lot::Mutex;
use std::sync::Arc;

/// The window's system insets in physical pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SystemInsets {
    /// System bars and display cutouts.
    pub(crate) safe_area: Edges<DevicePixels>,
    /// The soft keyboard.
    pub(crate) ime: Edges<DevicePixels>,
}

/// Samples [`SystemInsets`] from the activity's decor view.
///
/// Targeting SDK 35 forces edge-to-edge, where a `NativeActivity` is drawn
/// under the system bars and is not told how large they are, so the insets
/// are queried over JNI. View methods must run on the Java main thread;
/// [`Self::refresh`] schedules the read there and [`Self::take`] collects it.
pub(crate) struct SystemInsetsReader {
    app: AndroidApp,
    latest: Arc<Mutex<Option<SystemInsets>>>,
}

impl SystemInsetsReader {
    pub(crate) fn new(app: AndroidApp) -> Self {
        Self {
            app,
            latest: Arc::default(),
        }
    }

    /// Returns the insets read since the last call, if any.
    pub(crate) fn take(&self) -> Option<SystemInsets> {
        self.latest.lock().take()
    }

    pub(crate) fn refresh(&self) {
        let app = self.app.clone();
        let latest = Arc::clone(&self.latest);
        self.app.run_on_java_main_thread(Box::new(move || {
            // SAFETY: android-activity guarantees `vm_as_ptr` returns the
            // process's valid `JavaVM` pointer for the lifetime of the app.
            let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr().cast()) };
            let activity_ptr = app.activity_as_ptr();
            let insets = vm.attach_current_thread(|env| {
                // SAFETY: `activity_as_ptr` is a global reference to the
                // activity that lives as long as the app. `JObject` does not
                // delete the reference on drop.
                let activity = unsafe { JObject::from_raw(env, activity_ptr as jni::sys::jobject) };
                let window = env
                    .call_method(
                        &activity,
                        jni_str!("getWindow"),
                        jni_sig!("()Landroid/view/Window;"),
                        &[],
                    )?
                    .l()?;
                let decor_view = env
                    .call_method(
                        &window,
                        jni_str!("getDecorView"),
                        jni_sig!("()Landroid/view/View;"),
                        &[],
                    )?
                    .l()?;
                let root_insets = env
                    .call_method(
                        &decor_view,
                        jni_str!("getRootWindowInsets"),
                        jni_sig!("()Landroid/view/WindowInsets;"),
                        &[],
                    )?
                    .l()?;
                // Null while the decor view is not attached to a window.
                if root_insets.is_null() {
                    return anyhow::Ok(None);
                }

                let mut type_mask = |name| {
                    anyhow::Ok(
                        env.call_static_method(
                            jni_str!("android/view/WindowInsets$Type"),
                            name,
                            jni_sig!("()I"),
                            &[],
                        )?
                        .i()?,
                    )
                };
                let safe_area_types =
                    type_mask(jni_str!("systemBars"))? | type_mask(jni_str!("displayCutout"))?;
                let ime_types = type_mask(jni_str!("ime"))?;

                let mut edges = |types| {
                    let insets = env
                        .call_method(
                            &root_insets,
                            jni_str!("getInsets"),
                            jni_sig!("(I)Landroid/graphics/Insets;"),
                            &[JValue::Int(types)],
                        )?
                        .l()?;
                    let mut edge = |name| {
                        anyhow::Ok(DevicePixels(
                            env.get_field(&insets, name, jni_sig!("I"))?.i()?,
                        ))
                    };
                    anyhow::Ok(Edges {
                        top: edge(jni_str!("top"))?,
                        right: edge(jni_str!("right"))?,
                        bottom: edge(jni_str!("bottom"))?,
                        left: edge(jni_str!("left"))?,
                    })
                };
                anyhow::Ok(Some(SystemInsets {
                    safe_area: edges(safe_area_types)?,
                    ime: edges(ime_types)?,
                }))
            });
            match insets {
                Ok(Some(insets)) => *latest.lock() = Some(insets),
                Ok(None) => {}
                Err(error) => log::error!("failed to read window insets: {error:#}"),
            }
        }));
    }
}
