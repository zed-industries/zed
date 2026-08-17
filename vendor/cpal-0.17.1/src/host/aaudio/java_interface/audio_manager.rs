use super::{
    utils::{
        get_context, get_property, get_system_service, with_attached, JNIEnv, JObject, JResult,
    },
    AudioManager, Context,
};

impl AudioManager {
    /// Get the frames per buffer using Android Java API
    pub fn get_frames_per_buffer() -> Result<i32, String> {
        let context = get_context();

        with_attached(context, |env, context| get_frames_per_buffer(env, &context))
            .map_err(|error| error.to_string())
    }
}

fn get_frames_per_buffer<'j>(env: &mut JNIEnv<'j>, context: &JObject<'j>) -> JResult<i32> {
    let audio_manager = get_system_service(env, context, Context::AUDIO_SERVICE)?;

    let frames_per_buffer = get_property(
        env,
        &audio_manager,
        AudioManager::PROPERTY_OUTPUT_FRAMES_PER_BUFFER,
    )?;

    let frames_per_buffer_string = String::from(env.get_string(&frames_per_buffer)?);

    // TODO: Use jni::errors::Error::ParseFailed instead of jni::errors::Error::JniCall once jni > v0.21.1 is released
    frames_per_buffer_string
        .parse::<i32>()
        .map_err(|_| jni::errors::Error::JniCall(jni::errors::JniError::Unknown))
}
