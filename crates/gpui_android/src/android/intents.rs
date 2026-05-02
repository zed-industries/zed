//! `Intent`-based system integration: opening URLs, revealing files, asking
//! Android to open a path with whatever default app handles its MIME type.
//!
//! These all desugar to:
//!
//! ```java
//! Intent intent = new Intent(Intent.ACTION_VIEW);
//! intent.setData(Uri.parse(url));
//! intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
//! activity.startActivity(intent);
//! ```
//!
//! …but in raw JNI, since GPUI doesn't ship a Java/Kotlin glue layer.

use android_activity::AndroidApp;
use anyhow::{Context as _, Result};
use jni::{
    Env, jni_sig, jni_str,
    objects::{JObject, JValue},
};

use super::jni_glue::with_activity;

/// `intent.setData(Uri.parse(url))` + `activity.startActivity(intent)` for a
/// URL-style string (`https://`, `mailto:`, custom schemes, etc.).
pub(crate) fn open_url(app: &AndroidApp, url: &str) {
    if let Err(error) = with_activity(app, |env, activity| start_view_intent(env, activity, url)) {
        log::warn!("open_url({url}): {error:#}");
    }
}

/// Reveal a path-on-disk: roughly `xdg-open` semantics. Wraps the path in a
/// `file://` URI; for a true content provider you'd build a `content://` URI
/// via `FileProvider.getUriForFile`, which requires app-side glue.
pub(crate) fn reveal_path(app: &AndroidApp, path: &std::path::Path) {
    let url = format!("file://{}", path.display());
    open_url(app, &url);
}

/// Same shape as `reveal_path`, kept separate so callers can switch behaviour
/// later (e.g. preferring `ACTION_OPEN_DOCUMENT` for files vs.
/// `ACTION_VIEW` for URLs).
pub(crate) fn open_with_system(app: &AndroidApp, path: &std::path::Path) {
    let url = format!("file://{}", path.display());
    open_url(app, &url);
}

fn start_view_intent<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
    url: &str,
) -> Result<()> {
    // android.net.Uri uri = Uri.parse(url);
    let url_jstr = env.new_string(url).context("alloc URL string")?;
    let uri_class = env
        .find_class(jni_str!("android.net.Uri"))
        .context("FindClass android.net.Uri")?;
    let uri = env
        .call_static_method(
            &uri_class,
            jni_str!("parse"),
            jni_sig!((s: "java.lang.String") -> "android.net.Uri"),
            &[JValue::Object(&url_jstr)],
        )
        .context("Uri.parse")?
        .l()
        .context("Uri.parse returned non-object")?;

    // Intent intent = new Intent(Intent.ACTION_VIEW, uri);
    let action = env
        .new_string("android.intent.action.VIEW")
        .context("alloc ACTION_VIEW")?;
    let intent_class = env
        .find_class(jni_str!("android.content.Intent"))
        .context("FindClass android.content.Intent")?;
    let intent = env
        .new_object(
            &intent_class,
            jni_sig!((action: "java.lang.String", uri: "android.net.Uri") -> void),
            &[JValue::Object(&action), JValue::Object(&uri)],
        )
        .context("new Intent(action, uri)")?;

    // intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
    // FLAG_ACTIVITY_NEW_TASK = 0x10000000 — it's a public constant.
    env.call_method(
        &intent,
        jni_str!("addFlags"),
        jni_sig!((flags: jint) -> "android.content.Intent"),
        &[JValue::Int(0x1000_0000)],
    )
    .context("Intent.addFlags(NEW_TASK)")?;

    // activity.startActivity(intent);
    env.call_method(
        activity,
        jni_str!("startActivity"),
        jni_sig!((intent: "android.content.Intent") -> void),
        &[JValue::Object(&intent)],
    )
    .context("Activity.startActivity")?;
    Ok(())
}
