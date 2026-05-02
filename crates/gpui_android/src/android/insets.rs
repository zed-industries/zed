//! IME inset query.
//!
//! With `windowSoftInputMode="adjustResize"` and edge-to-edge enabled, modern
//! Android no longer auto-resizes the GameActivity surface when the IME
//! shows — apps are expected to inspect `WindowInsets.Type.ime()` and adapt
//! their layout. This module exposes that bottom inset (in logical pixels)
//! so [`AndroidWindow`](super::window::AndroidWindow) can shrink its
//! reported bounds while the soft keyboard is visible, which keeps the
//! focused input visible above the keyboard instead of behind it.
//!
//! Pre-API-30 devices don't have the typed `WindowInsets.Type.ime()` API.
//! We detect the runtime SDK with `Build.VERSION.SDK_INT` and fall back to
//! `0` for older devices — the IME will overlap content there, but that's
//! the documented Android behaviour and most users are on API 30+ in
//! practice.

use android_activity::AndroidApp;
use anyhow::{Context as _, Result};
use jni::{Env, jni_sig, jni_str, objects::JObject};

use super::jni_glue::with_activity;

/// Bottom inset (in logical pixels) currently occupied by the soft keyboard.
/// Returns `0.0` when the IME is hidden, when the view tree hasn't laid out
/// yet, or on pre-API-30 devices that don't expose the typed inset.
///
/// `scale_factor` converts the platform's physical-pixel inset to GPUI's
/// logical pixels — pass [`AndroidWindow::scale_factor`].
pub(crate) fn ime_bottom_inset_logical(app: &AndroidApp, scale_factor: f32) -> f32 {
    match query(app, scale_factor) {
        Ok(inset) => inset,
        Err(error) => {
            // Don't spam the log on every InsetsChanged event — the IME
            // inset query frequently races with view-tree teardown during
            // surface transitions, which throws benign exceptions.
            log::debug!("ime_bottom_inset_logical: {error:#}");
            0.0
        }
    }
}

fn query(app: &AndroidApp, scale_factor: f32) -> Result<f32> {
    if scale_factor <= 0.0 {
        return Ok(0.0);
    }
    with_activity(app, |env, activity| {
        if sdk_int(env)? < 30 {
            return Ok(0.0);
        }
        let bottom_px = ime_bottom_pixels(env, activity)?;
        Ok(bottom_px as f32 / scale_factor)
    })
}

fn sdk_int<'local>(env: &mut Env<'local>) -> Result<i32> {
    let class = env
        .find_class(jni_str!("android.os.Build$VERSION"))
        .context("FindClass Build$VERSION")?;
    let value = env
        .get_static_field(&class, jni_str!("SDK_INT"), jni_sig!(jint))
        .context("Build.VERSION.SDK_INT")?
        .i()
        .context("SDK_INT not int")?;
    Ok(value)
}

fn ime_bottom_pixels<'local>(env: &mut Env<'local>, activity: &JObject<'local>) -> Result<i32> {
    let window = env
        .call_method(
            activity,
            jni_str!("getWindow"),
            jni_sig!(() -> "android.view.Window"),
            &[],
        )
        .context("Activity.getWindow")?
        .l()
        .context("getWindow returned non-object")?;
    let decor = env
        .call_method(
            &window,
            jni_str!("getDecorView"),
            jni_sig!(() -> "android.view.View"),
            &[],
        )
        .context("Window.getDecorView")?
        .l()
        .context("getDecorView returned non-object")?;
    let insets = env
        .call_method(
            &decor,
            jni_str!("getRootWindowInsets"),
            jni_sig!(() -> "android.view.WindowInsets"),
            &[],
        )
        .context("View.getRootWindowInsets")?
        .l()
        .context("getRootWindowInsets returned non-object")?;
    if insets.is_null() {
        return Ok(0);
    }

    let type_class = env
        .find_class(jni_str!("android.view.WindowInsets$Type"))
        .context("FindClass WindowInsets$Type")?;
    let ime_type = env
        .call_static_method(
            &type_class,
            jni_str!("ime"),
            jni_sig!(() -> jint),
            &[],
        )
        .context("WindowInsets$Type.ime()")?
        .i()
        .context("ime() returned non-int")?;

    let ime_insets = env
        .call_method(
            &insets,
            jni_str!("getInsets"),
            jni_sig!((mask: jint) -> "android.graphics.Insets"),
            &[jni::objects::JValue::Int(ime_type)],
        )
        .context("WindowInsets.getInsets(ime)")?
        .l()
        .context("getInsets returned non-object")?;
    if ime_insets.is_null() {
        return Ok(0);
    }

    let bottom = env
        .get_field(&ime_insets, jni_str!("bottom"), jni_sig!(jint))
        .context("Insets.bottom")?
        .i()
        .context("bottom not int")?;
    Ok(bottom.max(0))
}
