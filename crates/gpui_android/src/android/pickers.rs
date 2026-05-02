//! Storage Access Framework (file picker) and PhotoPicker (image picker)
//! integration for [`gpui::Platform::prompt_for_paths`] and the higher-level
//! [`pick_files`] / [`pick_images`] helpers.
//!
//! Android pickers are *Activities*: you build an [`Intent`] and hand it to
//! [`Activity.startActivityForResult`], the system shows the picker UI, and
//! the chosen URI(s) come back through the host Activity's
//! [`onActivityResult`] override — asynchronously, on the JVM's main
//! thread. This module wires that round-trip:
//!
//! 1. [`pick_files`] / [`pick_images`] / [`pick_directory`] mint a fresh
//!    request code, stash a [`oneshot::Sender`] in a process-wide registry
//!    keyed by that code, build the right [`Intent`] from Rust via JNI and
//!    call [`Activity.startActivityForResult`].
//! 2. The host Activity overrides `onActivityResult(requestCode,
//!    resultCode, data)` and forwards every call to
//!    `dev.zed.gpui.NativeBridge.onActivityResult` (the static native
//!    method this module exposes — see [`Java_dev_zed_gpui_NativeBridge_onActivityResult`]).
//! 3. The JNI entry point pulls the URIs out of the [`Intent`], looks up
//!    the matching sender by request code, and resolves the receiver.
//!
//! The single Java glue class lives at
//! `dev.zed.gpui.NativeBridge` (a tiny file each host project must
//! include — see `crates/gpui/examples/android-host/app/src/main/java/dev/zed/gpui/NativeBridge.java`).
//! Choosing a fixed package + class lets us export a stable
//! `Java_dev_zed_gpui_NativeBridge_*` symbol from this crate without
//! requiring per-app `RegisterNatives` glue.
//!
//! ## URI vs path
//!
//! Android pickers return `content://` URIs, not filesystem paths. This
//! module returns those URIs as `PathBuf`s — on Unix `PathBuf` happily
//! holds arbitrary byte sequences, and stuffing the URI string in there
//! lets us satisfy the `Platform::prompt_for_paths` signature without a
//! parallel "URI" type. Callers that need an actual readable file should
//! use [`crate::android::content::copy_to_cache`] (TODO) to materialise
//! the URI's bytes into the app's cache dir.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicI32, Ordering};

use anyhow::{Context as _, Result, anyhow};
use futures::channel::oneshot;
use jni::{
    Env, EnvUnowned, jni_sig, jni_str,
    objects::{JClass, JObject, JObjectArray, JValue},
    sys::{jint, jobject},
};

use super::android_app;
use super::jni_glue::{java_string_to_rust, with_activity};

/// Result type shared by all pickers — `Ok(None)` means the user
/// cancelled, `Ok(Some(_))` is a non-empty list of URIs.
type PickResult = Result<Option<Vec<PathBuf>>>;

/// Request codes mint above this floor so they're unlikely to collide
/// with codes the host application is already using for its own
/// `startActivityForResult` calls.
const REQUEST_CODE_BASE: i32 = 0x6770_0000;

static REQUEST_CODE_COUNTER: AtomicI32 = AtomicI32::new(REQUEST_CODE_BASE);

static PENDING: Mutex<Option<HashMap<i32, oneshot::Sender<PickResult>>>> = Mutex::new(None);

fn pending() -> std::sync::MutexGuard<'static, Option<HashMap<i32, oneshot::Sender<PickResult>>>> {
    let mut guard = PENDING.lock().expect("picker registry poisoned");
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    guard
}

fn register(sender: oneshot::Sender<PickResult>) -> i32 {
    let code = REQUEST_CODE_COUNTER.fetch_add(1, Ordering::SeqCst);
    pending()
        .as_mut()
        .expect("registry initialised")
        .insert(code, sender);
    code
}

fn complete(code: i32, result: PickResult) {
    let sender = pending().as_mut().and_then(|map| map.remove(&code));
    match sender {
        Some(sender) => {
            // Receiver dropped is a benign no-op (the caller cancelled the
            // future before the picker resolved).
            let _ = sender.send(result);
        }
        None => log::warn!("Activity result for unknown request code {code}"),
    }
}

/// Launch a Storage Access Framework file picker
/// (`Intent.ACTION_OPEN_DOCUMENT`) configured for the given MIME types.
/// Pass an empty `mime_types` slice to allow any file (`*/*`).
pub fn pick_files(
    multiple: bool,
    mime_types: &[&str],
) -> oneshot::Receiver<PickResult> {
    let (tx, rx) = oneshot::channel();
    let Some(app) = android_app() else {
        let _ = tx.send(Err(anyhow!("AndroidApp not registered")));
        return rx;
    };
    let code = register(tx);
    if let Err(error) = with_activity(&app, |env, activity| {
        launch_open_document(env, activity, code, multiple, mime_types)
    }) {
        complete(code, Err(error));
    }
    rx
}

/// Launch a Storage Access Framework directory picker
/// (`Intent.ACTION_OPEN_DOCUMENT_TREE`). Returns at most one URI.
pub fn pick_directory() -> oneshot::Receiver<PickResult> {
    let (tx, rx) = oneshot::channel();
    let Some(app) = android_app() else {
        let _ = tx.send(Err(anyhow!("AndroidApp not registered")));
        return rx;
    };
    let code = register(tx);
    if let Err(error) =
        with_activity(&app, |env, activity| launch_open_document_tree(env, activity, code))
    {
        complete(code, Err(error));
    }
    rx
}

/// Launch the system PhotoPicker via `MediaStore.ACTION_PICK_IMAGES`
/// (Android 13+) or the
/// [`PickVisualMedia`](https://developer.android.com/reference/androidx/activity/result/contract/ActivityResultContracts.PickVisualMedia)
/// equivalent shape. On older platforms Android implicitly falls back to a
/// document picker filtered to images.
pub fn pick_images(multiple: bool) -> oneshot::Receiver<PickResult> {
    let (tx, rx) = oneshot::channel();
    let Some(app) = android_app() else {
        let _ = tx.send(Err(anyhow!("AndroidApp not registered")));
        return rx;
    };
    let code = register(tx);
    if let Err(error) =
        with_activity(&app, |env, activity| launch_pick_images(env, activity, code, multiple))
    {
        complete(code, Err(error));
    }
    rx
}

fn launch_open_document<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
    request_code: i32,
    multiple: bool,
    mime_types: &[&str],
) -> Result<()> {
    let intent = build_intent(env, "android.intent.action.OPEN_DOCUMENT")?;
    add_category(env, &intent, "android.intent.category.OPENABLE")?;
    // Per the SDK docs, EXTRA_MIME_TYPES is honoured only when the
    // primary type is set to `*/*` — so set it as the wildcard up front
    // and put the real list (if any) into the array extra.
    set_intent_type(env, &intent, "*/*")?;
    if !mime_types.is_empty() {
        put_extra_string_array(
            env,
            &intent,
            "android.intent.extra.MIME_TYPES",
            mime_types,
        )?;
    }
    if multiple {
        put_extra_bool(env, &intent, "android.intent.extra.ALLOW_MULTIPLE", true)?;
    }
    start_activity_for_result(env, activity, &intent, request_code)
}

fn launch_open_document_tree<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
    request_code: i32,
) -> Result<()> {
    let intent = build_intent(env, "android.intent.action.OPEN_DOCUMENT_TREE")?;
    start_activity_for_result(env, activity, &intent, request_code)
}

fn launch_pick_images<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
    request_code: i32,
    multiple: bool,
) -> Result<()> {
    // `MediaStore.ACTION_PICK_IMAGES` was added in Android 13; on older
    // platforms the resolved Activity is the system picker shim that
    // routes back through the document UI. The action string is the one
    // public constant the framework recognises in all places, so passing
    // it as a literal works without an SDK-version check.
    let intent = build_intent(env, "android.provider.action.PICK_IMAGES")?;
    set_intent_type(env, &intent, "image/*")?;
    if multiple {
        // `MediaStore.EXTRA_PICK_IMAGES_MAX` caps the count; passing the
        // platform max (`MediaStore.getPickImagesMaxLimit()`) is what
        // `PickVisualMedia.PickMultipleVisualMedia()` does. We bake in
        // the conservative documented maximum (100) instead of an
        // additional JNI roundtrip.
        put_extra_int(env, &intent, "android.provider.extra.PICK_IMAGES_MAX", 100)?;
    }
    start_activity_for_result(env, activity, &intent, request_code)
}

fn build_intent<'local>(env: &mut Env<'local>, action: &str) -> Result<JObject<'local>> {
    let action_jstr = env.new_string(action).context("alloc Intent action")?;
    let intent_class = env
        .find_class(jni_str!("android.content.Intent"))
        .context("FindClass android.content.Intent")?;
    env.new_object(
        &intent_class,
        jni_sig!((action: "java.lang.String") -> void),
        &[JValue::Object(&action_jstr)],
    )
    .context("new Intent(action)")
}

fn add_category<'local>(
    env: &mut Env<'local>,
    intent: &JObject<'local>,
    category: &str,
) -> Result<()> {
    let cat_jstr = env.new_string(category).context("alloc Intent category")?;
    env.call_method(
        intent,
        jni_str!("addCategory"),
        jni_sig!((category: "java.lang.String") -> "android.content.Intent"),
        &[JValue::Object(&cat_jstr)],
    )
    .context("Intent.addCategory")?;
    Ok(())
}

fn set_intent_type<'local>(
    env: &mut Env<'local>,
    intent: &JObject<'local>,
    mime_type: &str,
) -> Result<()> {
    let type_jstr = env.new_string(mime_type).context("alloc Intent type")?;
    env.call_method(
        intent,
        jni_str!("setType"),
        jni_sig!((mime: "java.lang.String") -> "android.content.Intent"),
        &[JValue::Object(&type_jstr)],
    )
    .context("Intent.setType")?;
    Ok(())
}

fn put_extra_bool<'local>(
    env: &mut Env<'local>,
    intent: &JObject<'local>,
    name: &str,
    value: bool,
) -> Result<()> {
    let name_jstr = env.new_string(name).context("alloc extra name")?;
    env.call_method(
        intent,
        jni_str!("putExtra"),
        jni_sig!((name: "java.lang.String", value: bool) -> "android.content.Intent"),
        &[JValue::Object(&name_jstr), JValue::Bool(value)],
    )
    .context("Intent.putExtra(boolean)")?;
    Ok(())
}

fn put_extra_int<'local>(
    env: &mut Env<'local>,
    intent: &JObject<'local>,
    name: &str,
    value: i32,
) -> Result<()> {
    let name_jstr = env.new_string(name).context("alloc extra name")?;
    env.call_method(
        intent,
        jni_str!("putExtra"),
        jni_sig!((name: "java.lang.String", value: jint) -> "android.content.Intent"),
        &[JValue::Object(&name_jstr), JValue::Int(value)],
    )
    .context("Intent.putExtra(int)")?;
    Ok(())
}

fn put_extra_string_array<'local>(
    env: &mut Env<'local>,
    intent: &JObject<'local>,
    name: &str,
    values: &[&str],
) -> Result<()> {
    let name_jstr = env.new_string(name).context("alloc extra name")?;
    let array = string_array(env, values)?;
    // Picks the `(String, String[])` overload via the explicit
    // `[Ljava/lang/String;` parameter type — without it JNI tries to
    // resolve to `(String, Serializable)` and `EXTRA_MIME_TYPES` reads
    // back as `null`.
    env.call_method(
        intent,
        jni_str!("putExtra"),
        jni_sig!(
            (name: "java.lang.String", values: ["java.lang.String"])
                -> "android.content.Intent"
        ),
        &[JValue::Object(&name_jstr), JValue::Object(&array)],
    )
    .context("Intent.putExtra(String, String[])")?;
    Ok(())
}

fn string_array<'local>(
    env: &mut Env<'local>,
    items: &[&str],
) -> Result<JObjectArray<'local>> {
    let initial = env.new_string("").context("alloc empty initial string")?;
    let array = env
        .new_object_array(items.len() as jint, jni_str!("java.lang.String"), &initial)
        .context("alloc String[]")?;
    for (i, item) in items.iter().enumerate() {
        let jstr = env.new_string(item).context("alloc array element")?;
        array
            .set_element(env, i, &jstr)
            .context("String[i] = …")?;
    }
    Ok(array)
}

fn start_activity_for_result<'local>(
    env: &mut Env<'local>,
    activity: &JObject<'local>,
    intent: &JObject<'local>,
    request_code: i32,
) -> Result<()> {
    env.call_method(
        activity,
        jni_str!("startActivityForResult"),
        jni_sig!((intent: "android.content.Intent", code: jint) -> void),
        &[JValue::Object(intent), JValue::Int(request_code)],
    )
    .context("Activity.startActivityForResult")?;
    Ok(())
}

/// Decode an `Intent` returned via `onActivityResult` into the list of
/// URIs the user picked. Handles three cases:
///
/// - Single-pick: the URI is in `intent.getData()`.
/// - Multi-pick: each URI is an `intent.getClipData().getItemAt(i).getUri()`.
/// - Cancelled / null intent: returns `Ok(None)`.
fn collect_uris<'local>(env: &mut Env<'local>, intent: &JObject<'local>) -> Result<Vec<String>> {
    let mut uris = Vec::new();

    // ClipData first — when the picker returned multiple items it sets
    // ClipData and `getData()` is null. (The system picker emits both
    // for backwards compat sometimes; if so, prefer ClipData.)
    let clip_data = env
        .call_method(
            intent,
            jni_str!("getClipData"),
            jni_sig!(() -> "android.content.ClipData"),
            &[],
        )
        .context("Intent.getClipData")?
        .l()
        .context("getClipData returned non-object")?;
    if !clip_data.is_null() {
        let count = env
            .call_method(&clip_data, jni_str!("getItemCount"), jni_sig!(() -> jint), &[])
            .context("ClipData.getItemCount")?
            .i()
            .context("getItemCount returned non-int")?;
        for i in 0..count {
            let item = env
                .call_method(
                    &clip_data,
                    jni_str!("getItemAt"),
                    jni_sig!((index: jint) -> "android.content.ClipData$Item"),
                    &[JValue::Int(i)],
                )
                .context("ClipData.getItemAt")?
                .l()
                .context("getItemAt returned non-object")?;
            let uri = env
                .call_method(&item, jni_str!("getUri"), jni_sig!(() -> "android.net.Uri"), &[])
                .context("ClipData.Item.getUri")?
                .l()
                .context("getUri returned non-object")?;
            if !uri.is_null() {
                uris.push(uri_to_string(env, uri)?);
            }
        }
        if !uris.is_empty() {
            return Ok(uris);
        }
    }

    let data = env
        .call_method(intent, jni_str!("getData"), jni_sig!(() -> "android.net.Uri"), &[])
        .context("Intent.getData")?
        .l()
        .context("getData returned non-object")?;
    if !data.is_null() {
        uris.push(uri_to_string(env, data)?);
    }

    Ok(uris)
}

fn uri_to_string<'local>(env: &mut Env<'local>, uri: JObject<'local>) -> Result<String> {
    let s = env
        .call_method(&uri, jni_str!("toString"), jni_sig!(() -> "java.lang.String"), &[])
        .context("Uri.toString")?
        .l()
        .context("toString returned non-object")?;
    java_string_to_rust(env, s)
}

fn handle_result<'local>(
    env: &mut Env<'local>,
    request_code: i32,
    result_code: i32,
    data: &JObject<'local>,
) {
    // Activity.RESULT_OK = -1; RESULT_CANCELED = 0.
    const RESULT_OK: i32 = -1;
    if result_code != RESULT_OK || data.is_null() {
        complete(request_code, Ok(None));
        return;
    }
    match collect_uris(env, data) {
        Ok(uris) if uris.is_empty() => complete(request_code, Ok(None)),
        Ok(uris) => {
            let paths = uris.into_iter().map(PathBuf::from).collect();
            complete(request_code, Ok(Some(paths)));
        }
        Err(error) => complete(request_code, Err(error)),
    }
}

/// JNI entry point invoked by the host Activity's `onActivityResult`
/// override.
///
/// The fixed `Java_dev_zed_gpui_NativeBridge_*` symbol name lets us bind
/// without per-app `RegisterNatives` glue — every host project just needs
/// to ship the matching `dev/zed/gpui/NativeBridge.java` shim.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_zed_gpui_NativeBridge_onActivityResult<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _class: JClass<'caller>,
    request_code: jint,
    result_code: jint,
    data: jobject,
) {
    let outcome = unowned_env.with_env(|env| -> jni::errors::Result<()> {
        // Wrap the raw Intent jobject in a transient JObject so we can use
        // jni-rs's safe call machinery. The local ref's lifetime is tied
        // to this native frame, so we don't need to delete it.
        // SAFETY: `data` is a local reference owned by the caller's JNI
        // frame; android-activity's JVM guarantees the frame outlives our
        // `with_env` closure.
        let intent = unsafe { JObject::from_raw(env, data) };
        handle_result(env, request_code as i32, result_code as i32, &intent);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>();
}
