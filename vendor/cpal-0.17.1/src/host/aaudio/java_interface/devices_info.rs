use num_traits::FromPrimitive;

use crate::{DeviceDirection, SampleFormat};

use super::{
    android_device_flags,
    utils::{
        call_method_no_args_ret_bool, call_method_no_args_ret_char_sequence,
        call_method_no_args_ret_int, call_method_no_args_ret_int_array,
        call_method_no_args_ret_string, get_context, get_devices, get_system_service,
        with_attached, JNIEnv, JObject, JResult,
    },
    AudioDeviceInfo, AudioDeviceType, Context,
};

impl AudioDeviceInfo {
    /**
     * Request audio devices using Android Java API
     */
    pub fn request(direction: DeviceDirection) -> Result<Vec<AudioDeviceInfo>, String> {
        let context = get_context();

        with_attached(context, |env, context| {
            let sdk_version = env
                .get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?
                .i()?;

            if sdk_version >= 23 {
                try_request_devices_info(env, &context, direction)
            } else {
                Err(jni::errors::Error::MethodNotFound {
                    name: "".into(),
                    sig: "".into(),
                })
            }
        })
        .map_err(|error| error.to_string())
    }
}

fn try_request_devices_info<'j>(
    env: &mut JNIEnv<'j>,
    context: &JObject<'j>,
    direction: DeviceDirection,
) -> JResult<Vec<AudioDeviceInfo>> {
    let audio_manager = get_system_service(env, context, Context::AUDIO_SERVICE)?;

    let devices = get_devices(env, &audio_manager, android_device_flags(direction))?;

    let length = env.get_array_length(&devices)?;

    (0..length)
        .map(|index| {
            let device = env.get_object_array_element(&devices, index)?;
            let id = call_method_no_args_ret_int(env, &device, "getId")?;
            let address = call_method_no_args_ret_string(env, &device, "getAddress")?;
            let address = String::from(env.get_string(&address)?);
            let product_name =
                call_method_no_args_ret_char_sequence(env, &device, "getProductName")?;
            let product_name = String::from(env.get_string(&product_name)?);
            let device_type =
                FromPrimitive::from_i32(call_method_no_args_ret_int(env, &device, "getType")?)
                    .unwrap_or(AudioDeviceType::Unsupported);

            let is_source = call_method_no_args_ret_bool(env, &device, "isSource")?;
            let is_sink = call_method_no_args_ret_bool(env, &device, "isSink")?;
            let direction = crate::device_description::direction_from_caps(is_source, is_sink);
            let channel_counts =
                call_method_no_args_ret_int_array(env, &device, "getChannelCounts")?;
            let sample_rates = call_method_no_args_ret_int_array(env, &device, "getSampleRates")?;
            let formats = call_method_no_args_ret_int_array(env, &device, "getEncodings")?
                .into_iter()
                .filter_map(SampleFormat::from_encoding)
                .collect::<Vec<_>>();

            Ok(AudioDeviceInfo {
                id,
                address,
                product_name,
                device_type,
                direction,
                channel_counts,
                sample_rates,
                formats,
            })
        })
        .collect::<Result<Vec<_>, _>>()
}
