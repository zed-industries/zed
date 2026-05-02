//! Read/write the Android clipboard via JNI.
//!
//! We don't go through `androidx`/`Kotlin` — we issue raw JNI calls into the
//! framework `android.content.ClipboardManager`. This keeps GPUI free of any
//! Java/Kotlin-side glue: the only thing the host activity has to do is exist.
//!
//! All errors are logged and turned into `None` / no-op so failures here can
//! never crash the UI thread.

use anyhow::{Context as _, Result, anyhow};
use android_activity::AndroidApp;
use gpui::{ClipboardEntry, ClipboardItem};
use jni::{
    Env, jni_sig, jni_str,
    objects::{JObject, JValue},
};

use super::jni_glue::{java_string_to_rust, with_activity};

/// Read the system clipboard as a single plain-text [`ClipboardItem`], or
/// `None` if the clipboard is empty / not text / unavailable.
pub(crate) fn read(app: &AndroidApp) -> Option<ClipboardItem> {
    match with_activity(app, do_read) {
        Ok(item) => item,
        Err(error) => {
            log::warn!("clipboard read failed: {error:#}");
            None
        }
    }
}

/// Write a `ClipboardItem` to the system clipboard. Multi-entry clipboards
/// are flattened to the concatenated string content.
pub(crate) fn write(app: &AndroidApp, item: ClipboardItem) {
    let text = item
        .entries
        .iter()
        .filter_map(|entry| match entry {
            ClipboardEntry::String(s) => Some(s.text().clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");
    if let Err(error) = with_activity(app, |env, activity| do_write(env, activity, &text)) {
        log::warn!("clipboard write failed: {error:#}");
    }
}

fn do_read<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
) -> Result<Option<ClipboardItem>> {
    let clipboard = system_service(env, activity, "clipboard")?;
    if clipboard.is_null() {
        return Ok(None);
    }

    let primary = env
        .call_method(
            &clipboard,
            jni_str!("getPrimaryClip"),
            jni_sig!(() -> "android.content.ClipData"),
            &[],
        )
        .context("ClipboardManager.getPrimaryClip")?
        .l()
        .context("ClipboardManager.getPrimaryClip returned non-object")?;
    if primary.is_null() {
        return Ok(None);
    }

    let count = env
        .call_method(&primary, jni_str!("getItemCount"), jni_sig!(() -> jint), &[])
        .context("ClipData.getItemCount")?
        .i()
        .context("ClipData.getItemCount returned non-int")?;
    if count <= 0 {
        return Ok(None);
    }

    let item = env
        .call_method(
            &primary,
            jni_str!("getItemAt"),
            jni_sig!((index: jint) -> "android.content.ClipData$Item"),
            &[JValue::Int(0)],
        )
        .context("ClipData.getItemAt")?
        .l()
        .context("ClipData.getItemAt returned non-object")?;
    if item.is_null() {
        return Ok(None);
    }

    let text_obj = env
        .call_method(
            &item,
            jni_str!("getText"),
            jni_sig!(() -> "java.lang.CharSequence"),
            &[],
        )
        .context("ClipData.Item.getText")?
        .l()
        .context("ClipData.Item.getText returned non-object")?;
    if text_obj.is_null() {
        return Ok(None);
    }

    let text_str = env
        .call_method(
            &text_obj,
            jni_str!("toString"),
            jni_sig!(() -> "java.lang.String"),
            &[],
        )
        .context("CharSequence.toString")?
        .l()
        .context("CharSequence.toString returned non-object")?;
    if text_str.is_null() {
        return Ok(None);
    }

    let text = java_string_to_rust(env, text_str)?;
    if text.is_empty() {
        return Ok(None);
    }
    Ok(Some(ClipboardItem::new_string(text)))
}

fn do_write<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
    text: &str,
) -> Result<()> {
    let clipboard = system_service(env, activity, "clipboard")?;
    if clipboard.is_null() {
        return Err(anyhow!("getSystemService(\"clipboard\") returned null"));
    }

    let label = env.new_string("gpui").context("alloc clipboard label")?;
    let value = env.new_string(text).context("alloc clipboard text")?;
    let clip_data_class = env
        .find_class(jni_str!("android.content.ClipData"))
        .context("FindClass android.content.ClipData")?;
    let clip = env
        .call_static_method(
            &clip_data_class,
            jni_str!("newPlainText"),
            jni_sig!(
                (label: "java.lang.CharSequence", text: "java.lang.CharSequence")
                    -> "android.content.ClipData"
            ),
            &[JValue::Object(&label), JValue::Object(&value)],
        )
        .context("ClipData.newPlainText")?
        .l()
        .context("ClipData.newPlainText returned non-object")?;

    env.call_method(
        &clipboard,
        jni_str!("setPrimaryClip"),
        jni_sig!((clip: "android.content.ClipData") -> void),
        &[JValue::Object(&clip)],
    )
    .context("ClipboardManager.setPrimaryClip")?;
    Ok(())
}

/// `(Activity)context.getSystemService(name)`.
pub(crate) fn system_service<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
    name: &str,
) -> Result<JObject<'local>> {
    let name_str = env.new_string(name).context("alloc system-service name")?;
    let service = env
        .call_method(
            activity,
            jni_str!("getSystemService"),
            jni_sig!((name: "java.lang.String") -> "java.lang.Object"),
            &[JValue::Object(&name_str)],
        )
        .context("Context.getSystemService")?
        .l()
        .context("Context.getSystemService returned non-object")?;
    Ok(service)
}
