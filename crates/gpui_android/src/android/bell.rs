//! Plays the system "bell" sound (UI haptic + tone). Wraps
//! `AudioManager.playSoundEffect(AudioManager.FX_KEYPRESS_STANDARD)` —
//! standard for cross-platform GUI bells on Android.

use anyhow::{Context as _, Result};
use android_activity::AndroidApp;
use jni::{
    Env, jni_sig, jni_str,
    objects::{JObject, JValue},
};

use super::{clipboard::system_service, jni_glue::with_activity};

/// Best-effort `play_system_bell` — failures are logged.
pub(crate) fn ring(app: &AndroidApp) {
    if let Err(error) = with_activity(app, |env, activity| ring_inner(env, activity)) {
        log::warn!("play_system_bell failed: {error:#}");
    }
}

fn ring_inner<'local>(env: &mut Env<'local>, activity: &JObject<'local>) -> Result<()> {
    let audio = system_service(env, activity, "audio")?;
    if audio.is_null() {
        return Ok(());
    }
    // AudioManager.FX_KEYPRESS_STANDARD = 5
    env.call_method(
        &audio,
        jni_str!("playSoundEffect"),
        jni_sig!((effect: jint) -> void),
        &[JValue::Int(5)],
    )
    .context("AudioManager.playSoundEffect")?;
    Ok(())
}
