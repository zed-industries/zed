use super::{
    utils::{
        get_context, get_package_manager, has_system_feature, with_attached, JNIEnv, JObject,
        JResult,
    },
    PackageManager,
};

/**
 * The Android audio features
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFeature {
    LowLatency,
    Output,
    Pro,
    Microphone,
    Midi,
}

impl From<AudioFeature> for &'static str {
    fn from(feature: AudioFeature) -> Self {
        use AudioFeature::*;
        match feature {
            LowLatency => PackageManager::FEATURE_AUDIO_LOW_LATENCY,
            Output => PackageManager::FEATURE_AUDIO_OUTPUT,
            Pro => PackageManager::FEATURE_AUDIO_PRO,
            Microphone => PackageManager::FEATURE_MICROPHONE,
            Midi => PackageManager::FEATURE_MIDI,
        }
    }
}

impl AudioFeature {
    /**
     * Check availability of an audio feature using Android Java API
     */
    pub fn has(&self) -> Result<bool, String> {
        let context = get_context();

        with_attached(context, |env, activity| {
            try_check_system_feature(env, &activity, (*self).into())
        })
        .map_err(|error| error.to_string())
    }
}

fn try_check_system_feature<'j>(
    env: &mut JNIEnv<'j>,
    activity: &JObject<'j>,
    feature: &str,
) -> JResult<bool> {
    let package_manager = get_package_manager(env, activity)?;

    has_system_feature(env, &package_manager, feature)
}
