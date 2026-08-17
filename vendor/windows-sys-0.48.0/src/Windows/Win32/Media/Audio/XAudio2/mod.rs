::windows_targets::link ! ( "xaudio2_8.dll""system" #[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"] fn CreateAudioReverb ( ppapo : *mut ::windows_sys::core::IUnknown ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "xaudio2_8.dll""system" #[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"] fn CreateAudioVolumeMeter ( ppapo : *mut ::windows_sys::core::IUnknown ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "xaudio2_8.dll""cdecl" #[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"] fn CreateFX ( clsid : *const ::windows_sys::core::GUID , peffect : *mut ::windows_sys::core::IUnknown , pinitdat : *const ::core::ffi::c_void , initdatabytesize : u32 ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "hrtfapo.dll""system" #[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"] fn CreateHrtfApo ( init : *const HrtfApoInit , xapo : *mut IXAPO ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "xaudio2_8.dll""system" #[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"] fn XAudio2CreateWithVersionInfo ( ppxaudio2 : *mut IXAudio2 , flags : u32 , xaudio2processor : u32 , ntddiversion : u32 ) -> ::windows_sys::core::HRESULT );
pub type IXAPO = *mut ::core::ffi::c_void;
pub type IXAPOHrtfParameters = *mut ::core::ffi::c_void;
pub type IXAPOParameters = *mut ::core::ffi::c_void;
pub type IXAudio2 = *mut ::core::ffi::c_void;
pub type IXAudio2EngineCallback = *mut ::core::ffi::c_void;
pub type IXAudio2Extension = *mut ::core::ffi::c_void;
pub type IXAudio2MasteringVoice = *mut ::core::ffi::c_void;
pub type IXAudio2SourceVoice = *mut ::core::ffi::c_void;
pub type IXAudio2SubmixVoice = *mut ::core::ffi::c_void;
pub type IXAudio2Voice = *mut ::core::ffi::c_void;
pub type IXAudio2VoiceCallback = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const AudioReverb: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc2633b16_471b_4498_b8c5_4f0959e2ec09);
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const AudioVolumeMeter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4fc3b166_972a_40cf_bc37_7db03db2fba3);
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FACILITY_XAPO: u32 = 2199u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FACILITY_XAUDIO2: u32 = 2198u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_DEFAULT_DELAY: f32 = 500f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_DEFAULT_FEEDBACK: f32 = 0.5f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_DEFAULT_WETDRYMIX: f32 = 0.5f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_MAX_DELAY: f32 = 2000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_MAX_FEEDBACK: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_MAX_WETDRYMIX: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_MIN_DELAY: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_MIN_FEEDBACK: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXECHO_MIN_WETDRYMIX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf5e01117_d6c4_485a_a3f5_695196f3dbfa);
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_DEFAULT_BANDWIDTH: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_0: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_1: f32 = 800f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_2: f32 = 2000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_3: f32 = 10000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_DEFAULT_GAIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MAX_BANDWIDTH: f32 = 2f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MAX_FRAMERATE: u32 = 48000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MAX_FREQUENCY_CENTER: f32 = 20000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MAX_GAIN: f32 = 7.94f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MIN_BANDWIDTH: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MIN_FRAMERATE: u32 = 22000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MIN_FREQUENCY_CENTER: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEQ_MIN_GAIN: f32 = 0.126f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXEcho: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5039d740_f736_449a_84d3_a56202557b87);
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXLOUDNESS_DEFAULT_MOMENTARY_MS: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXLOUDNESS_DEFAULT_SHORTTERM_MS: u32 = 3000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXMASTERINGLIMITER_DEFAULT_LOUDNESS: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXMASTERINGLIMITER_DEFAULT_RELEASE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXMASTERINGLIMITER_MAX_LOUDNESS: u32 = 1800u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXMASTERINGLIMITER_MAX_RELEASE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXMASTERINGLIMITER_MIN_LOUDNESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXMASTERINGLIMITER_MIN_RELEASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXMasteringLimiter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc4137916_2be1_46fd_8599_441536f49856);
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXREVERB_DEFAULT_DIFFUSION: f32 = 0.9f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXREVERB_DEFAULT_ROOMSIZE: f32 = 0.6f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXREVERB_MAX_DIFFUSION: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXREVERB_MAX_ROOMSIZE: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXREVERB_MIN_DIFFUSION: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXREVERB_MIN_ROOMSIZE: f32 = 0.0001f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const FXReverb: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7d9aca56_cb68_4807_b632_b137352e8596);
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const HRTF_DEFAULT_UNITY_GAIN_DISTANCE: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const HRTF_MAX_GAIN_LIMIT: f32 = 12f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const HRTF_MIN_GAIN_LIMIT: f32 = -96f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const HRTF_MIN_UNITY_GAIN_DISTANCE: f32 = 0.05f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor10: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor11: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor12: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor13: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor14: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor15: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor16: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor17: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor18: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor19: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor20: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor21: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor22: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor23: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor24: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor25: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor26: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor27: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor28: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor29: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor30: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor31: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor32: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor4: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor5: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor6: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor7: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor8: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Processor9: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const SPEAKER_MONO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_2PI: f32 = 6.2831855f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_DELAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_DOPPLER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_EMITTER_ANGLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_LPF_DIRECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_LPF_REVERB: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_MATRIX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_REDIRECT_TO_LFE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_REVERB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_CALCULATE_ZEROCENTER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_HANDLE_BYTESIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_PI: f32 = 3.1415927f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const X3DAUDIO_SPEED_OF_SOUND: f32 = 343.5f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_E_FORMAT_UNSUPPORTED: ::windows_sys::core::HRESULT = -2003369983i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_FLAG_BUFFERCOUNT_MUST_MATCH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_FLAG_CHANNELS_MUST_MATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_FLAG_FRAMERATE_MUST_MATCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_FLAG_INPLACE_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_FLAG_INPLACE_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_MAX_CHANNELS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_MAX_FRAMERATE: u32 = 200000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_MIN_CHANNELS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_MIN_FRAMERATE: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_REGISTRATION_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2D_DLL: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("xaudio2_9d.dll");
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2D_DLL_A: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("xaudio2_9d.dll");
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2D_DLL_W: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("xaudio2_9d.dll");
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_REAR_DELAY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_SIDE_DELAY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_DECAY_TIME: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_DENSITY: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_DISABLE_LATE_FIELD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_EARLY_DIFFUSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_CUTOFF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_GAIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_LATE_DIFFUSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_CUTOFF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_GAIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_REAR_DELAY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_DELAY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_GAIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_DELAY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_GAIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_FREQ: f32 = 5000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_HF: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_MAIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_SIZE: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_DEFAULT_WET_DRY_MIX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_7POINT1_REAR_DELAY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_7POINT1_SIDE_DELAY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_DENSITY: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_DIFFUSION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_FRAMERATE: u32 = 48000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_CUTOFF: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_GAIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_CUTOFF: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_GAIN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_POSITION: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_REAR_DELAY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_GAIN: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_REVERB_DELAY: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_REVERB_GAIN: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_FREQ: f32 = 20000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_HF: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_MAIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_ROOM_SIZE: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MAX_WET_DRY_MIX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_7POINT1_REAR_DELAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_7POINT1_SIDE_DELAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_DECAY_TIME: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_DENSITY: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_DIFFUSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_FRAMERATE: u32 = 20000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_CUTOFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_GAIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_CUTOFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_GAIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_POSITION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_REAR_DELAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_DELAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_GAIN: f32 = -100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_REVERB_DELAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_REVERB_GAIN: f32 = -100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_FREQ: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_HF: f32 = -100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_MAIN: f32 = -100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_ROOM_SIZE: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2FX_REVERB_MIN_WET_DRY_MIX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_1024_QUANTUM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_COMMIT_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_COMMIT_NOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DEBUG_ENGINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DEFAULT_PROCESSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DLL: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("xaudio2_9.dll");
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DLL_A: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("xaudio2_9.dll");
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_DLL_W: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("xaudio2_9.dll");
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_END_OF_STREAM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_E_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = -2003435516i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_E_INVALID_CALL: ::windows_sys::core::HRESULT = -2003435519i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_E_XAPO_CREATION_FAILED: ::windows_sys::core::HRESULT = -2003435517i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_E_XMA_DECODER_ERROR: ::windows_sys::core::HRESULT = -2003435518i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_API_CALLS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_DETAIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_ERRORS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_FUNC_CALLS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_LOCKS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_MEMORY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_STREAMING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_TIMING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOG_WARNINGS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_LOOP_INFINITE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_INSTANCES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_LOOP_COUNT: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_QUEUED_BUFFERS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: u32 = 600000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: u32 = 300000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_SAMPLE_RATE: u32 = 200000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216f32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_NO_LOOP_REGION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_PLAY_TAILS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_QUANTUM_DENOMINATOR: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_QUANTUM_NUMERATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_SEND_USEFILTER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_VOICE_NOPITCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_VOICE_NOSRC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAUDIO2_VOICE_USEFILTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub type HrtfDirectivityType = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const OmniDirectional: HrtfDirectivityType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Cardioid: HrtfDirectivityType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Cone: HrtfDirectivityType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub type HrtfDistanceDecayType = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const NaturalDecay: HrtfDistanceDecayType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const CustomDecay: HrtfDistanceDecayType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub type HrtfEnvironment = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Small: HrtfEnvironment = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Medium: HrtfEnvironment = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Large: HrtfEnvironment = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const Outdoors: HrtfEnvironment = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub type XAPO_BUFFER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_BUFFER_SILENT: XAPO_BUFFER_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const XAPO_BUFFER_VALID: XAPO_BUFFER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub type XAUDIO2_FILTER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const NotchFilter: XAUDIO2_FILTER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 5i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct FXECHO_INITDATA {
    pub MaxDelay: f32,
}
impl ::core::marker::Copy for FXECHO_INITDATA {}
impl ::core::clone::Clone for FXECHO_INITDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct FXECHO_PARAMETERS {
    pub WetDryMix: f32,
    pub Feedback: f32,
    pub Delay: f32,
}
impl ::core::marker::Copy for FXECHO_PARAMETERS {}
impl ::core::clone::Clone for FXECHO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct FXEQ_PARAMETERS {
    pub FrequencyCenter0: f32,
    pub Gain0: f32,
    pub Bandwidth0: f32,
    pub FrequencyCenter1: f32,
    pub Gain1: f32,
    pub Bandwidth1: f32,
    pub FrequencyCenter2: f32,
    pub Gain2: f32,
    pub Bandwidth2: f32,
    pub FrequencyCenter3: f32,
    pub Gain3: f32,
    pub Bandwidth3: f32,
}
impl ::core::marker::Copy for FXEQ_PARAMETERS {}
impl ::core::clone::Clone for FXEQ_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct FXMASTERINGLIMITER_PARAMETERS {
    pub Release: u32,
    pub Loudness: u32,
}
impl ::core::marker::Copy for FXMASTERINGLIMITER_PARAMETERS {}
impl ::core::clone::Clone for FXMASTERINGLIMITER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct FXREVERB_PARAMETERS {
    pub Diffusion: f32,
    pub RoomSize: f32,
}
impl ::core::marker::Copy for FXREVERB_PARAMETERS {}
impl ::core::clone::Clone for FXREVERB_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct HrtfApoInit {
    pub distanceDecay: *mut HrtfDistanceDecay,
    pub directivity: *mut HrtfDirectivity,
}
impl ::core::marker::Copy for HrtfApoInit {}
impl ::core::clone::Clone for HrtfApoInit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct HrtfDirectivity {
    pub r#type: HrtfDirectivityType,
    pub scaling: f32,
}
impl ::core::marker::Copy for HrtfDirectivity {}
impl ::core::clone::Clone for HrtfDirectivity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct HrtfDirectivityCardioid {
    pub directivity: HrtfDirectivity,
    pub order: f32,
}
impl ::core::marker::Copy for HrtfDirectivityCardioid {}
impl ::core::clone::Clone for HrtfDirectivityCardioid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct HrtfDirectivityCone {
    pub directivity: HrtfDirectivity,
    pub innerAngle: f32,
    pub outerAngle: f32,
}
impl ::core::marker::Copy for HrtfDirectivityCone {}
impl ::core::clone::Clone for HrtfDirectivityCone {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct HrtfDistanceDecay {
    pub r#type: HrtfDistanceDecayType,
    pub maxGain: f32,
    pub minGain: f32,
    pub unityGainDistance: f32,
    pub cutoffDistance: f32,
}
impl ::core::marker::Copy for HrtfDistanceDecay {}
impl ::core::clone::Clone for HrtfDistanceDecay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct HrtfOrientation {
    pub element: [f32; 9],
}
impl ::core::marker::Copy for HrtfOrientation {}
impl ::core::clone::Clone for HrtfOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct HrtfPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ::core::marker::Copy for HrtfPosition {}
impl ::core::clone::Clone for HrtfPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAPO_LOCKFORPROCESS_PARAMETERS {
    pub pFormat: *const super::WAVEFORMATEX,
    pub MaxFrameCount: u32,
}
impl ::core::marker::Copy for XAPO_LOCKFORPROCESS_PARAMETERS {}
impl ::core::clone::Clone for XAPO_LOCKFORPROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAPO_PROCESS_BUFFER_PARAMETERS {
    pub pBuffer: *mut ::core::ffi::c_void,
    pub BufferFlags: XAPO_BUFFER_FLAGS,
    pub ValidFrameCount: u32,
}
impl ::core::marker::Copy for XAPO_PROCESS_BUFFER_PARAMETERS {}
impl ::core::clone::Clone for XAPO_PROCESS_BUFFER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAPO_REGISTRATION_PROPERTIES {
    pub clsid: ::windows_sys::core::GUID,
    pub FriendlyName: [u16; 256],
    pub CopyrightInfo: [u16; 256],
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub Flags: u32,
    pub MinInputBufferCount: u32,
    pub MaxInputBufferCount: u32,
    pub MinOutputBufferCount: u32,
    pub MaxOutputBufferCount: u32,
}
impl ::core::marker::Copy for XAPO_REGISTRATION_PROPERTIES {}
impl ::core::clone::Clone for XAPO_REGISTRATION_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    pub WetDryMix: f32,
    pub Room: i32,
    pub RoomHF: i32,
    pub RoomRolloffFactor: f32,
    pub DecayTime: f32,
    pub DecayHFRatio: f32,
    pub Reflections: i32,
    pub ReflectionsDelay: f32,
    pub Reverb: i32,
    pub ReverbDelay: f32,
    pub Diffusion: f32,
    pub Density: f32,
    pub HFReference: f32,
}
impl ::core::marker::Copy for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {}
impl ::core::clone::Clone for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2FX_REVERB_PARAMETERS {
    pub WetDryMix: f32,
    pub ReflectionsDelay: u32,
    pub ReverbDelay: u8,
    pub RearDelay: u8,
    pub SideDelay: u8,
    pub PositionLeft: u8,
    pub PositionRight: u8,
    pub PositionMatrixLeft: u8,
    pub PositionMatrixRight: u8,
    pub EarlyDiffusion: u8,
    pub LateDiffusion: u8,
    pub LowEQGain: u8,
    pub LowEQCutoff: u8,
    pub HighEQGain: u8,
    pub HighEQCutoff: u8,
    pub RoomFilterFreq: f32,
    pub RoomFilterMain: f32,
    pub RoomFilterHF: f32,
    pub ReflectionsGain: f32,
    pub ReverbGain: f32,
    pub DecayTime: f32,
    pub Density: f32,
    pub RoomSize: f32,
    pub DisableLateField: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XAUDIO2FX_REVERB_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XAUDIO2FX_REVERB_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2FX_VOLUMEMETER_LEVELS {
    pub pPeakLevels: *mut f32,
    pub pRMSLevels: *mut f32,
    pub ChannelCount: u32,
}
impl ::core::marker::Copy for XAUDIO2FX_VOLUMEMETER_LEVELS {}
impl ::core::clone::Clone for XAUDIO2FX_VOLUMEMETER_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_BUFFER {
    pub Flags: u32,
    pub AudioBytes: u32,
    pub pAudioData: *const u8,
    pub PlayBegin: u32,
    pub PlayLength: u32,
    pub LoopBegin: u32,
    pub LoopLength: u32,
    pub LoopCount: u32,
    pub pContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for XAUDIO2_BUFFER {}
impl ::core::clone::Clone for XAUDIO2_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_BUFFER_WMA {
    pub pDecodedPacketCumulativeBytes: *const u32,
    pub PacketCount: u32,
}
impl ::core::marker::Copy for XAUDIO2_BUFFER_WMA {}
impl ::core::clone::Clone for XAUDIO2_BUFFER_WMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_DEBUG_CONFIGURATION {
    pub TraceMask: u32,
    pub BreakMask: u32,
    pub LogThreadID: super::super::super::Foundation::BOOL,
    pub LogFileline: super::super::super::Foundation::BOOL,
    pub LogFunctionName: super::super::super::Foundation::BOOL,
    pub LogTiming: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XAUDIO2_DEBUG_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XAUDIO2_DEBUG_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount: u32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XAUDIO2_EFFECT_CHAIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XAUDIO2_EFFECT_CHAIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect: ::windows_sys::core::IUnknown,
    pub InitialState: super::super::super::Foundation::BOOL,
    pub OutputChannels: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XAUDIO2_EFFECT_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XAUDIO2_EFFECT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_FILTER_PARAMETERS {
    pub Type: XAUDIO2_FILTER_TYPE,
    pub Frequency: f32,
    pub OneOverQ: f32,
}
impl ::core::marker::Copy for XAUDIO2_FILTER_PARAMETERS {}
impl ::core::clone::Clone for XAUDIO2_FILTER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_PERFORMANCE_DATA {
    pub AudioCyclesSinceLastQuery: u64,
    pub TotalCyclesSinceLastQuery: u64,
    pub MinimumCyclesPerQuantum: u32,
    pub MaximumCyclesPerQuantum: u32,
    pub MemoryUsageInBytes: u32,
    pub CurrentLatencyInSamples: u32,
    pub GlitchesSinceEngineStarted: u32,
    pub ActiveSourceVoiceCount: u32,
    pub TotalSourceVoiceCount: u32,
    pub ActiveSubmixVoiceCount: u32,
    pub ActiveResamplerCount: u32,
    pub ActiveMatrixMixCount: u32,
    pub ActiveXmaSourceVoices: u32,
    pub ActiveXmaStreams: u32,
}
impl ::core::marker::Copy for XAUDIO2_PERFORMANCE_DATA {}
impl ::core::clone::Clone for XAUDIO2_PERFORMANCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: u32,
    pub pOutputVoice: IXAudio2Voice,
}
impl ::core::marker::Copy for XAUDIO2_SEND_DESCRIPTOR {}
impl ::core::clone::Clone for XAUDIO2_SEND_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: u32,
    pub ActiveFlags: u32,
    pub InputChannels: u32,
    pub InputSampleRate: u32,
}
impl ::core::marker::Copy for XAUDIO2_VOICE_DETAILS {}
impl ::core::clone::Clone for XAUDIO2_VOICE_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: u32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}
impl ::core::marker::Copy for XAUDIO2_VOICE_SENDS {}
impl ::core::clone::Clone for XAUDIO2_VOICE_SENDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_XAudio2\"`*"]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut ::core::ffi::c_void,
    pub BuffersQueued: u32,
    pub SamplesPlayed: u64,
}
impl ::core::marker::Copy for XAUDIO2_VOICE_STATE {}
impl ::core::clone::Clone for XAUDIO2_VOICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
