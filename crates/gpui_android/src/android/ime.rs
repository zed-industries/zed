//! Android-specific knobs for working with GPUI's standard input bridge.
//!
//! GPUI already has a complete IME story — apps register their input
//! widgets via [`Window::handle_input`] (see `crates/gpui/examples/input.rs`
//! for the canonical pattern), and platform backends route the OS's IME
//! events through [`PlatformInputHandler`]. This module exposes only the
//! pieces of Android's input model that GPUI proper doesn't speak:
//!
//! - [`request_ime_input_type`] — choose the soft keyboard layout (text vs
//!   numeric vs phone vs date/time) for the focused field. Without this
//!   every focused input gets the default text keyboard.
//! - [`keyboard_bottom_inset`] — query the current IME bottom inset in
//!   logical pixels, so apps can pad their layouts or scroll focused fields
//!   into view above the keyboard.
//!
//! Everything else (focus, selection, composition, paste, autocorrect)
//! flows through GPUI's normal `EntityInputHandler` path with no extra
//! per-platform code.
//!
//! [`Window::handle_input`]: gpui::Window::handle_input
//! [`PlatformInputHandler`]: gpui::PlatformInputHandler

use std::cell::Cell;

use android_activity::AndroidApp;
use android_activity::input::{ImeOptions, InputType, TextInputAction};
use anyhow::{Context as _, Result};
use gpui::{Pixels, px};
use jni::{jni_sig, jni_str, objects::JValue};

use super::jni_glue::with_activity;

thread_local! {
    /// Pending IME descriptor stashed by the most recently focused input
    /// widget; consumed by the platform's `reconcile_keyboard` once per
    /// run-loop iteration. Thread-local because GPUI's foreground-only
    /// execution model guarantees focus-handlers and the run loop share a
    /// thread, so a `Cell` is enough.
    static REQUESTED_IME_TYPE: Cell<Option<(InputType, TextInputAction)>> = const { Cell::new(None) };
}

/// Stash a custom IME [`InputType`] / [`TextInputAction`] for the platform
/// to apply on the next run-loop iteration. Call this from your input
/// widget's focus path (e.g. `on_mouse_down` after `window.focus(...)`)
/// when you want a non-default keyboard layout — most commonly a numeric
/// pad for an amount field, a phone pad, or a URL keyboard.
///
/// The default — applied when no widget has called this for the current
/// focus — is multi-line plain text with autocorrect off, which matches
/// the prior behaviour of the IME bridge.
///
/// Pair the call with `gpui_android::ime::restart_pending()` if you want
/// the change to take effect on a keyboard that's already visible — the
/// editor-info update alone is silently ignored by Samsung's IME (and
/// some others) until `InputMethodManager.restartInput` runs.
pub fn request_ime_input_type(input_type: InputType, action: TextInputAction) {
    REQUESTED_IME_TYPE.with(|cell| cell.set(Some((input_type, action))));
}

/// Drain the pending IME descriptor, returning the value the platform
/// should apply (or `None` if no widget claimed the focus).
pub(crate) fn take_requested_ime_type() -> Option<(InputType, TextInputAction)> {
    REQUESTED_IME_TYPE.with(|cell| cell.take())
}

/// Default editor info applied when no widget claimed the focus —
/// multi-line plain text, no autocorrect, no fullscreen IME.
pub(crate) fn default_descriptor() -> (InputType, TextInputAction, ImeOptions) {
    (
        InputType::TYPE_CLASS_TEXT
            | InputType::TYPE_TEXT_FLAG_MULTI_LINE
            | InputType::TYPE_TEXT_FLAG_NO_SUGGESTIONS,
        TextInputAction::None,
        ImeOptions::IME_FLAG_NO_FULLSCREEN,
    )
}

/// Bottom inset (in logical pixels) currently occupied by the soft
/// keyboard, or `0` if it's hidden / pre-API-30.
///
/// Apps that lay out their UI relative to the visible viewport can use
/// this to pad / shift / scroll their content so the focused input stays
/// above the keyboard. Re-read it on every render — the value updates
/// during the slide-up / slide-down animation.
pub fn keyboard_bottom_inset() -> Pixels {
    super::android_app()
        .map(|app| {
            let scale_factor = app
                .config()
                .density()
                .map(|d| d as f32 / 160.0)
                .unwrap_or(1.0);
            super::ime_bottom_inset_logical(&app, scale_factor)
        })
        .map(px)
        .unwrap_or(px(0.0))
}

/// Force the system IME to re-read the editor info we last pushed via
/// [`AndroidApp::set_ime_editor_info`]. Most IMEs ignore mid-focus editor
/// info changes unless `InputMethodManager.restartInput(view)` is called
/// — which is why switching from a text field to a number field while
/// the keyboard is up doesn't change the keyboard layout without this.
///
/// Best-effort: failures (e.g. no focused view yet) are logged at
/// `debug` and otherwise ignored. The next visibility transition will
/// re-apply the editor info anyway.
pub(crate) fn restart_input(app: &AndroidApp) {
    if let Err(error) = restart_input_inner(app) {
        log::debug!("restart_input: {error:#}");
    }
}

fn restart_input_inner(app: &AndroidApp) -> Result<()> {
    with_activity(app, |env, activity| {
        // View view = activity.getCurrentFocus();
        // (For our GameActivity surface this returns the
        // `InputEnabledSurfaceView` once it's the IME target; null before
        // the first soft-input show, in which case we fall back to the
        // window's decor view.)
        let view = env
            .call_method(
                activity,
                jni_str!("getCurrentFocus"),
                jni_sig!(() -> "android.view.View"),
                &[],
            )
            .context("Activity.getCurrentFocus")?
            .l()
            .context("getCurrentFocus returned non-object")?;
        let view = if view.is_null() {
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
            env.call_method(
                &window,
                jni_str!("getDecorView"),
                jni_sig!(() -> "android.view.View"),
                &[],
            )
            .context("Window.getDecorView")?
            .l()
            .context("getDecorView returned non-object")?
        } else {
            view
        };

        // Object imm = activity.getSystemService(Context.INPUT_METHOD_SERVICE);
        // INPUT_METHOD_SERVICE is the literal string "input_method".
        let service_name = env
            .new_string("input_method")
            .context("alloc service name")?;
        let imm = env
            .call_method(
                activity,
                jni_str!("getSystemService"),
                jni_sig!((s: "java.lang.String") -> "java.lang.Object"),
                &[JValue::Object(&service_name)],
            )
            .context("Context.getSystemService(INPUT_METHOD_SERVICE)")?
            .l()
            .context("getSystemService returned non-object")?;
        if imm.is_null() {
            return Ok(());
        }

        // imm.restartInput(view);
        env.call_method(
            &imm,
            jni_str!("restartInput"),
            jni_sig!((v: "android.view.View") -> void),
            &[JValue::Object(&view)],
        )
        .context("InputMethodManager.restartInput")?;
        Ok(())
    })
}
