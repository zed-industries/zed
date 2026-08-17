pub type IApoAcousticEchoCancellation = *mut ::core::ffi::c_void;
pub type IApoAuxiliaryInputConfiguration = *mut ::core::ffi::c_void;
pub type IApoAuxiliaryInputRT = *mut ::core::ffi::c_void;
pub type IAudioDeviceModulesClient = *mut ::core::ffi::c_void;
pub type IAudioMediaType = *mut ::core::ffi::c_void;
pub type IAudioProcessingObject = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectConfiguration = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectLoggingService = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectNotifications = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectRT = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectRTQueueService = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectVBR = *mut ::core::ffi::c_void;
pub type IAudioSystemEffects = *mut ::core::ffi::c_void;
pub type IAudioSystemEffects2 = *mut ::core::ffi::c_void;
pub type IAudioSystemEffects3 = *mut ::core::ffi::c_void;
pub type IAudioSystemEffectsCustomFormats = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = -2005073919i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_UNLOCKED: ::windows_sys::core::HRESULT = -2005073914i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_APO_LOCKED: ::windows_sys::core::HRESULT = -2005073910i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_BUFFERS_OVERLAP: ::windows_sys::core::HRESULT = -2005073915i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2005073917i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_APO_CLSID: ::windows_sys::core::HRESULT = -2005073916i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFCOUNT: ::windows_sys::core::HRESULT = -2005073909i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFICIENT: ::windows_sys::core::HRESULT = -2005073908i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows_sys::core::HRESULT = -2005073911i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CURVE_PARAM: ::windows_sys::core::HRESULT = -2005073907i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_INPUTID: ::windows_sys::core::HRESULT = -2005073906i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows_sys::core::HRESULT = -2005073912i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2005073918i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows_sys::core::HRESULT = -2005073913i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 16u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const SID_AudioProcessingObjectLoggingService: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8b8008af_09f9_456e_a173_bdb58499bce7);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const SID_AudioProcessingObjectRTQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x458c1a1f_6899_4c12_99ac_e2e6ac253104);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_BUFFER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_VALID: APO_BUFFER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_CONNECTION_BUFFER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_FLAG = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_NONE: APO_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_INPLACE: APO_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_MIXER: APO_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_DEFAULT: APO_FLAG = 14i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_LOG_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type AUDIO_FLOW_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type AUDIO_SYSTEMEFFECT_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type EAudioConstriction = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionOff: EAudioConstriction = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction48_16: EAudioConstriction = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction44_16: EAudioConstriction = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction14_14: EAudioConstriction = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionMute: EAudioConstriction = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for APOInitBaseStruct {}
impl ::core::clone::Clone for APOInitBaseStruct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: super::IMMDeviceCollection,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for APOInitSystemEffects {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_sys::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pServiceProvider: super::super::super::System::Com::IServiceProvider,
    pub pDeviceCollection: super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_sys::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APOInitSystemEffects3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: IAudioMediaType,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_PROPERTY {
    pub pBuffer: usize,
    pub u32ValidFrameCount: u32,
    pub u32BufferFlags: APO_BUFFER_FLAGS,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_PROPERTY_V2 {
    pub property: APO_CONNECTION_PROPERTY,
    pub u64QPCTime: u64,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APO_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APO_NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows_sys::core::GUID,
    pub Flags: APO_FLAG,
    pub szFriendlyName: [u16; 256],
    pub szCopyrightInfo: [u16; 256],
    pub u32MajorVersion: u32,
    pub u32MinorVersion: u32,
    pub u32MinInputConnections: u32,
    pub u32MaxInputConnections: u32,
    pub u32MinOutputConnections: u32,
    pub u32MaxOutputConnections: u32,
    pub u32MaxInstances: u32,
    pub u32NumAPOInterfaces: u32,
    pub iidAPOInterfaceList: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for APO_REG_PROPERTIES {}
impl ::core::clone::Clone for APO_REG_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub propertyStore: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows_sys::core::GUID,
    pub canSetState: super::super::super::Foundation::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
    pub propertyStoreContext: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub propertyStoreContext: ::windows_sys::core::GUID,
    pub propertyStoreType: super::AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE,
    pub propertyStore: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: ::windows_sys::core::PWSTR,
    pub pFxProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows_sys::core::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl ::core::marker::Copy for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::clone::Clone for UNCOMPRESSEDAUDIOFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type FNAPONOTIFICATIONCALLBACK = ::core::option::Option<unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
