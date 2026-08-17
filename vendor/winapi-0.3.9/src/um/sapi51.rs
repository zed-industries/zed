// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! SAPI 5.1 definitions
use ctypes::{c_char, c_float, c_long, c_short, c_ushort, c_void};
use shared::guiddef::{CLSID, GUID, IID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FILETIME, HKEY, HMODULE, LPARAM, UINT, ULONG, USHORT, WORD, WPARAM
};
use shared::mmreg::WAVEFORMATEX;
use shared::rpcndr::byte;
use shared::windef::HWND;
use shared::wtypes::{BSTR, VARIANT_BOOL};
use shared::wtypesbase::{
    CLSCTX_INPROC_HANDLER, CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, CLSCTX_REMOTE_SERVER
};
use um::oaidl::{DISPID_NEWENUM, DISPID_VALUE, IDispatch, IDispatchVtbl, VARIANT};
use um::objidlbase::{IStream, IStreamVtbl, STREAM_SEEK_CUR, STREAM_SEEK_END, STREAM_SEEK_SET};
use um::servprov::{IServiceProvider, IServiceProviderVtbl};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LONG, LONGLONG, LPCWSTR, LPWSTR, ULONGLONG, WCHAR};
ENUM!{enum SPDATAKEYLOCATION {
    SPDKL_DefaultLocation = 0,
    SPDKL_CurrentUser = 1,
    SPDKL_LocalMachine = 2,
    SPDKL_CurrentConfig = 5,
}}
pub const SPDUI_EngineProperties: &'static str = "EngineProperties";
pub const SPDUI_AddRemoveWord: &'static str = "AddRemoveWord";
pub const SPDUI_UserTraining: &'static str = "UserTraining";
pub const SPDUI_MicTraining: &'static str = "MicTraining";
pub const SPDUI_RecoProfileProperties: &'static str = "RecoProfileProperties";
pub const SPDUI_AudioProperties: &'static str = "AudioProperties";
pub const SPDUI_AudioVolume: &'static str = "AudioVolume";
pub const SPDUI_UserEnrollment: &'static str = "UserEnrollment";
pub const SPDUI_ShareData: &'static str = "ShareData";
pub const SPDUI_Tutorial: &'static str = "Tutorial";
ENUM!{enum SPSTREAMFORMAT {
    SPSF_Default = -1i32 as u32,
    SPSF_NoAssignedFormat = 0,
    SPSF_Text = 1,
    SPSF_NonStandardFormat = 2,
    SPSF_ExtendedAudioFormat = 3,
    SPSF_8kHz8BitMono = 4,
    SPSF_8kHz8BitStereo = 5,
    SPSF_8kHz16BitMono = 6,
    SPSF_8kHz16BitStereo = 7,
    SPSF_11kHz8BitMono = 8,
    SPSF_11kHz8BitStereo = 9,
    SPSF_11kHz16BitMono = 10,
    SPSF_11kHz16BitStereo = 11,
    SPSF_12kHz8BitMono = 12,
    SPSF_12kHz8BitStereo = 13,
    SPSF_12kHz16BitMono = 14,
    SPSF_12kHz16BitStereo = 15,
    SPSF_16kHz8BitMono = 16,
    SPSF_16kHz8BitStereo = 17,
    SPSF_16kHz16BitMono = 18,
    SPSF_16kHz16BitStereo = 19,
    SPSF_22kHz8BitMono = 20,
    SPSF_22kHz8BitStereo = 21,
    SPSF_22kHz16BitMono = 22,
    SPSF_22kHz16BitStereo = 23,
    SPSF_24kHz8BitMono = 24,
    SPSF_24kHz8BitStereo = 25,
    SPSF_24kHz16BitMono = 26,
    SPSF_24kHz16BitStereo = 27,
    SPSF_32kHz8BitMono = 28,
    SPSF_32kHz8BitStereo = 29,
    SPSF_32kHz16BitMono = 30,
    SPSF_32kHz16BitStereo = 31,
    SPSF_44kHz8BitMono = 32,
    SPSF_44kHz8BitStereo = 33,
    SPSF_44kHz16BitMono = 34,
    SPSF_44kHz16BitStereo = 35,
    SPSF_48kHz8BitMono = 36,
    SPSF_48kHz8BitStereo = 37,
    SPSF_48kHz16BitMono = 38,
    SPSF_48kHz16BitStereo = 39,
    SPSF_TrueSpeech_8kHz1BitMono = 40,
    SPSF_CCITT_ALaw_8kHzMono = 41,
    SPSF_CCITT_ALaw_8kHzStereo = 42,
    SPSF_CCITT_ALaw_11kHzMono = 43,
    SPSF_CCITT_ALaw_11kHzStereo = 44,
    SPSF_CCITT_ALaw_22kHzMono = 45,
    SPSF_CCITT_ALaw_22kHzStereo = 46,
    SPSF_CCITT_ALaw_44kHzMono = 47,
    SPSF_CCITT_ALaw_44kHzStereo = 48,
    SPSF_CCITT_uLaw_8kHzMono = 49,
    SPSF_CCITT_uLaw_8kHzStereo = 50,
    SPSF_CCITT_uLaw_11kHzMono = 51,
    SPSF_CCITT_uLaw_11kHzStereo = 52,
    SPSF_CCITT_uLaw_22kHzMono = 53,
    SPSF_CCITT_uLaw_22kHzStereo = 54,
    SPSF_CCITT_uLaw_44kHzMono = 55,
    SPSF_CCITT_uLaw_44kHzStereo = 56,
    SPSF_ADPCM_8kHzMono = 57,
    SPSF_ADPCM_8kHzStereo = 58,
    SPSF_ADPCM_11kHzMono = 59,
    SPSF_ADPCM_11kHzStereo = 60,
    SPSF_ADPCM_22kHzMono = 61,
    SPSF_ADPCM_22kHzStereo = 62,
    SPSF_ADPCM_44kHzMono = 63,
    SPSF_ADPCM_44kHzStereo = 64,
    SPSF_GSM610_8kHzMono = 65,
    SPSF_GSM610_11kHzMono = 66,
    SPSF_GSM610_22kHzMono = 67,
    SPSF_GSM610_44kHzMono = 68,
    SPSF_NUM_FORMATS = 69,
}}
extern {
    pub static SPDFID_Text: GUID;
    pub static SPDFID_WaveFormatEx: GUID;
}
pub const SPREG_USER_ROOT: &'static str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech";
pub const SPREG_LOCAL_MACHINE_ROOT: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech";
pub const SPCAT_AUDIOOUT: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput";
pub const SPCAT_AUDIOIN: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput";
pub const SPCAT_VOICES: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices";
pub const SPCAT_RECOGNIZERS: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Recognizers";
pub const SPCAT_APPLEXICONS: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AppLexicons";
pub const SPCAT_PHONECONVERTERS: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\PhoneConverters";
pub const SPCAT_TEXTNORMALIZERS: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\TextNormalizers";
pub const SPCAT_RECOPROFILES: &'static str
    = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\RecoProfiles";
pub const SPMMSYS_AUDIO_IN_TOKEN_ID: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput\\TokenEnums\\MMAudioIn\\";
pub const SPMMSYS_AUDIO_OUT_TOKEN_ID: &'static str
    = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput\\TokenEnums\\MMAudioOut\\";
pub const SPCURRENT_USER_LEXICON_TOKEN_ID: &'static str
    = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserLexicon";
pub const SPCURRENT_USER_SHORTCUT_TOKEN_ID: &'static str
    = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserShortcut";
pub const SPTOKENVALUE_CLSID: &'static str = "CLSID";
pub const SPTOKENKEY_FILES: &'static str = "Files";
pub const SPTOKENKEY_UI: &'static str = "UI";
pub const SPTOKENKEY_ATTRIBUTES: &'static str = "Attributes";
pub const SPVOICECATEGORY_TTSRATE: &'static str = "DefaultTTSRate";
pub const SPPROP_RESOURCE_USAGE: &'static str = "ResourceUsage";
pub const SPPROP_HIGH_CONFIDENCE_THRESHOLD: &'static str = "HighConfidenceThreshold";
pub const SPPROP_NORMAL_CONFIDENCE_THRESHOLD: &'static str = "NormalConfidenceThreshold";
pub const SPPROP_LOW_CONFIDENCE_THRESHOLD: &'static str = "LowConfidenceThreshold";
pub const SPPROP_RESPONSE_SPEED: &'static str = "ResponseSpeed";
pub const SPPROP_COMPLEX_RESPONSE_SPEED: &'static str = "ComplexResponseSpeed";
pub const SPPROP_ADAPTATION_ON: &'static str = "AdaptationOn";
pub const SPPROP_PERSISTED_BACKGROUND_ADAPTATION: &'static str = "PersistedBackgroundAdaptation";
pub const SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION: &'static str
    = "PersistedLanguageModelAdaptation";
pub const SPPROP_UX_IS_LISTENING: &'static str = "UXIsListening";
pub const SPTOPIC_SPELLING: &'static str = "Spelling";
pub const SPWILDCARD: &'static str = "...";
pub const SPDICTATION: &'static str = "*";
pub const SPINFDICTATION: &'static str = "*+";
pub const SP_LOW_CONFIDENCE: c_char = -1;
pub const SP_NORMAL_CONFIDENCE: c_char = 0;
pub const SP_HIGH_CONFIDENCE: c_char = 1;
pub const DEFAULT_WEIGHT: c_float = 1.0;
pub const SP_MAX_WORD_LENGTH: ULONG = 128;
pub const SP_MAX_PRON_LENGTH: ULONG = 384;
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)]
interface ISpNotifyCallback(ISpNotifyCallbackVtbl) {
    fn NotifyCallback(
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
}}
FN!{stdcall SPNOTIFYCALLBACK(
    wParam: WPARAM,
    lParam: LPARAM,
) -> ()}
RIDL!{#[uuid(0x5eff4aef, 0x8487, 0x11d2, 0x96, 0x1c, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifySource(ISpNotifySourceVtbl): IUnknown(IUnknownVtbl) {
    fn SetNotifySink(
        pNotifySink: *mut ISpNotifySink,
    ) -> HRESULT,
    fn SetNotifyWindowMessage(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
    fn SetNotifyCallbackFunction(
        pfnCallback: SPNOTIFYCALLBACK,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
    fn SetNotifyCallbackInterface(
        pSpCallback: *mut ISpNotifyCallback,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
    fn SetNotifyWin32Event() -> HRESULT,
    fn WaitForNotifyEvent(
        dwMilliseconds: DWORD,
    ) -> HRESULT,
    fn GetNotifyEventHandle() -> HANDLE,
}}
RIDL!{#[uuid(0x259684dc, 0x37c3, 0x11d2, 0x96, 0x03, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifySink(ISpNotifySinkVtbl): IUnknown(IUnknownVtbl) {
    fn Notify() -> HRESULT,
}}
RIDL!{#[uuid(0xaca16614, 0x5d3d, 0x11d2, 0x96, 0x0e, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifyTranslator(ISpNotifyTranslatorVtbl): ISpNotifySink(ISpNotifySinkVtbl) {
    fn InitWindowMessage(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
    fn InitCallback(
        pfnCallback: SPNOTIFYCALLBACK,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
    fn InitSpNotifyCallback(
        pSpCallback: *mut ISpNotifyCallback,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
    fn InitWin32Event(
        hEvent: HANDLE,
        fCloseHandleOnRelease: BOOL,
    ) -> HRESULT,
    fn Wait(
        dwMilliseconds: DWORD,
    ) -> HRESULT,
    fn GetEventHandle() -> HANDLE,
}}
RIDL!{#[uuid(0x14056581, 0xe16c, 0x11d2, 0xbb, 0x90, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpDataKey(ISpDataKeyVtbl): IUnknown(IUnknownVtbl) {
    fn SetData(
        pszValueName: LPCWSTR,
        cbData: ULONG,
        pData: *const BYTE,
    ) -> HRESULT,
    fn GetData(
        pszValueName: LPCWSTR,
        pcbData: *mut ULONG,
        pData: *mut BYTE,
    ) -> HRESULT,
    fn SetStringValue(
        pszValueName: LPCWSTR,
        pszValue: LPCWSTR,
    ) -> HRESULT,
    fn GetStringValue(
        pszValueName: LPCWSTR,
        ppszValue: *mut LPWSTR,
    ) -> HRESULT,
    fn SetDWORD(
        pszValueName: LPCWSTR,
        dwValue: DWORD,
    ) -> HRESULT,
    fn GetDWORD(
        pszValueName: LPCWSTR,
        pdwValue: *mut DWORD,
    ) -> HRESULT,
    fn OpenKey(
        pszSubKeyName: LPCWSTR,
        ppSubKey: *mut *mut ISpDataKey,
    ) -> HRESULT,
    fn CreateKey(
        pszSubKey: LPCWSTR,
        ppSubKey: *mut *mut ISpDataKey,
    ) -> HRESULT,
    fn DeleteKey(
        pszSubKey: LPCWSTR,
    ) -> HRESULT,
    fn DeleteValue(
        pszValueName: LPCWSTR,
    ) -> HRESULT,
    fn EnumKeys(
        Index: ULONG,
        ppszSubKeyName: *mut LPWSTR,
    ) -> HRESULT,
    fn EnumValues(
        Index: ULONG,
        ppszValueName: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x92a66e2b, 0xc830, 0x4149, 0x83, 0xdf, 0x6f, 0xc2, 0xba, 0x1e, 0x7a, 0x5b)]
interface ISpRegDataKey(ISpRegDataKeyVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetKey(
        hkey: HKEY,
        fReadOnly: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2d3d3845, 0x39af, 0x4850, 0xbb, 0xf9, 0x40, 0xb4, 0x97, 0x80, 0x01, 0x1d)]
interface ISpObjectTokenCategory(ISpObjectTokenCategoryVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetId(
        pszCategoryId: LPCWSTR,
        fCreateIfNotExist: BOOL,
    ) -> HRESULT,
    fn GetId(
        ppszCoMemCategoryId: *mut LPWSTR,
    ) -> HRESULT,
    fn GetDataKey(
        spdkl: SPDATAKEYLOCATION,
        pppDataKey: *mut *mut ISpDataKey,
    ) -> HRESULT,
    fn EnumTokens(
        pzsReqAttribs: LPCWSTR,
        pszOptAttribs: LPCWSTR,
        ppEnum: *mut *mut IEnumSpObjectTokens,
    ) -> HRESULT,
    fn SetDefaultTokenId(
        pszTokenId: LPCWSTR,
    ) -> HRESULT,
    fn GetDefaultTokenId(
        ppszCoMemTokenId: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x14056589, 0xe16c, 0x11d2, 0xbb, 0x90, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpObjectToken(ISpObjectTokenVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetId(
        pszCategoryId: LPCWSTR,
        pszTokenId: LPCWSTR,
        fCreateIfNotExist: BOOL,
    ) -> HRESULT,
    fn GetId(
        ppszCoMemTokenId: *mut LPWSTR,
    ) -> HRESULT,
    fn GetCategory(
        ppTokenCategory: *mut *mut ISpObjectTokenCategory,
    ) -> HRESULT,
    fn CreateInstance(
        pUnkOuter: *mut IUnknown,
        dwClsContext: DWORD,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn GetStorageFileName(
        clsidCaller: REFCLSID,
        pszValueName: LPCWSTR,
        pszFileNameSpecifier: LPCWSTR,
        nFolder: ULONG,
        ppszFilePath: *mut LPWSTR,
    ) -> HRESULT,
    fn RemoveStorageFileName(
        pszKeyName: LPCWSTR,
        fDeleteFile: BOOL,
    ) -> HRESULT,
    fn Remove(
        pclsidCaller: *const CLSID,
    ) -> HRESULT,
    fn IsUISupported(
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        punkObject: *mut IUnknown,
        pfSupported: *mut BOOL,
    ) -> HRESULT,
    fn DisplayUI(
        hwndParent: HWND,
        pszTitle: LPCWSTR,
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        punkObject: *mut IUnknown,
    ) -> HRESULT,
    fn MatchesAttributes(
        pszAttributes: LPCWSTR,
        pfMatches: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb8aab0cf, 0x346f, 0x49d8, 0x94, 0x99, 0xc8, 0xb0, 0x3f, 0x16, 0x1d, 0x51)]
interface ISpObjectTokenInit(ISpObjectTokenInitVtbl): ISpObjectToken(ISpObjectTokenVtbl) {
    fn InitFromDataKey(
        pszCategoryId: LPCWSTR,
        pszTokenId: LPCWSTR,
        pDataKey: *mut ISpDataKey,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x06b64f9e, 0x7fda, 0x11d2, 0xb4, 0xf2, 0x00, 0xc0, 0x4f, 0x79, 0x73, 0x96)]
interface IEnumSpObjectTokens(IEnumSpObjectTokensVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        pelt: *mut *mut ISpObjectToken,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumSpObjectTokens,
    ) -> HRESULT,
    fn Item(
        Index: ULONG,
        ppToken: *mut *mut ISpObjectToken,
    ) -> HRESULT,
    fn GetCount(
        pCount: *mut ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5b559f40, 0xe952, 0x11d2, 0xbb, 0x91, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpObjectWithToken(ISpObjectWithTokenVtbl): IUnknown(IUnknownVtbl) {
    fn SetObjectToken(
        pToken: *mut ISpObjectToken,
    ) -> HRESULT,
    fn GetObjectToken(
        ppToken: *mut *mut ISpObjectToken,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x93384e18, 0x5014, 0x43d5, 0xad, 0xbb, 0xa7, 0x8e, 0x05, 0x59, 0x26, 0xbd)]
interface ISpResourceManager(ISpResourceManagerVtbl): IServiceProvider(IServiceProviderVtbl) {
    fn SetObject(
        guidServiceId: REFGUID,
        pUnkObject: *mut IUnknown,
    ) -> HRESULT,
    fn GetObject(
        guidServiceId: REFGUID,
        ObjectCLSID: REFCLSID,
        ObjectIID: REFIID,
        fReleaseWhenLastExternalRefReleased: BOOL,
        ppObject: *mut *mut c_void,
    ) -> HRESULT,
}}
ENUM!{enum SPEVENTLPARAMTYPE {
    SPET_LPARAM_IS_UNDEFINED = 0,
    SPET_LPARAM_IS_TOKEN,
    SPET_LPARAM_IS_OBJECT,
    SPET_LPARAM_IS_POINTER,
    SPET_LPARAM_IS_STRING,
}}
ENUM!{enum SPEVENTENUM {
    SPEI_UNDEFINED = 0,
    SPEI_START_INPUT_STREAM = 1,
    SPEI_END_INPUT_STREAM = 2,
    SPEI_VOICE_CHANGE = 3,
    SPEI_TTS_BOOKMARK = 4,
    SPEI_WORD_BOUNDARY = 5,
    SPEI_PHONEME = 6,
    SPEI_SENTENCE_BOUNDARY = 7,
    SPEI_VISEME = 8,
    SPEI_TTS_AUDIO_LEVEL = 9,
    SPEI_TTS_PRIVATE = 15,
    SPEI_MIN_TTS = 1,
    SPEI_MAX_TTS = 15,
    SPEI_END_SR_STREAM = 34,
    SPEI_SOUND_START = 35,
    SPEI_SOUND_END = 36,
    SPEI_PHRASE_START = 37,
    SPEI_RECOGNITION = 38,
    SPEI_HYPOTHESIS = 39,
    SPEI_SR_BOOKMARK = 40,
    SPEI_PROPERTY_NUM_CHANGE = 41,
    SPEI_PROPERTY_STRING_CHANGE = 42,
    SPEI_FALSE_RECOGNITION = 43,
    SPEI_INTERFERENCE = 44,
    SPEI_REQUEST_UI = 45,
    SPEI_RECO_STATE_CHANGE = 46,
    SPEI_ADAPTATION = 47,
    SPEI_START_SR_STREAM = 48,
    SPEI_RECO_OTHER_CONTEXT = 49,
    SPEI_SR_AUDIO_LEVEL = 50,
    SPEI_SR_PRIVATE = 52,
    SPEI_MIN_SR = 34,
    SPEI_MAX_SR = 52,
    SPEI_RESERVED1 = 30,
    SPEI_RESERVED2 = 33,
    SPEI_RESERVED3 = 63,
}}
pub const SPFEI_FLAGCHECK: ULONGLONG = (1 << SPEI_RESERVED1) | (1 << SPEI_RESERVED2);
pub const SPFEI_ALL_TTS_EVENTS: ULONGLONG = 0x000000000000FFFE | SPFEI_FLAGCHECK;
pub const SPFEI_ALL_SR_EVENTS: ULONGLONG = 0x003FFFFC00000000 | SPFEI_FLAGCHECK;
pub const SPFEI_ALL_EVENTS: ULONGLONG = 0xEFFFFFFFFFFFFFFF;
#[inline]
pub fn SPFEI(
        SPEI_ord: SPEVENTENUM,
    ) -> ULONGLONG {
    (1 << SPEI_ord) | SPFEI_FLAGCHECK
}
STRUCT!{struct SPEVENT {
    bitfields: DWORD,
    ulStreamNum: ULONG,
    ullAudioStreamOffset: ULONGLONG,
    wParam: WPARAM,
    lParam: LPARAM,
}}
BITFIELD!{SPEVENT bitfields: SPEVENTENUM [ eEventId set_eEventId[0..16], ]}
BITFIELD!{SPEVENT bitfields: SPEVENTLPARAMTYPE [ elParamType set_elParamType[16..32], ]}
STRUCT!{struct SPSERIALIZEDEVENT {
    bitfields: DWORD,
    ulStreamNum: ULONG,
    ullAudioStreamOffset: ULONGLONG,
    SerializedwParam: ULONG,
    SerializedlParam: LONG,
}}
BITFIELD!{SPSERIALIZEDEVENT bitfields: SPEVENTENUM [ eEventId set_eEventId[0..16], ]}
BITFIELD!{SPSERIALIZEDEVENT bitfields: SPEVENTLPARAMTYPE [ elParamType set_elParamType[16..32], ]}
STRUCT!{struct SPSERIALIZEDEVENT64 {
    bitfields: DWORD,
    ulStreamNum: ULONG,
    ullAudioStreamOffset: ULONGLONG,
    SerializedwParam: ULONGLONG,
    SerializedlParam: LONGLONG,
}}
BITFIELD!{SPSERIALIZEDEVENT64 bitfields: SPEVENTENUM [
    eEventId set_eEventId[0..16],
]}
BITFIELD!{SPSERIALIZEDEVENT64 bitfields: SPEVENTLPARAMTYPE [
    elParamType set_elParamType[16..32],
]}
ENUM!{enum SPINTERFERENCE {
    SPINTERFERENCE_NONE = 0,
    SPINTERFERENCE_NOISE,
    SPINTERFERENCE_NOSIGNAL,
    SPINTERFERENCE_TOOLOUD,
    SPINTERFERENCE_TOOQUIET,
    SPINTERFERENCE_TOOFAST,
    SPINTERFERENCE_TOOSLOW,
    SPINTERFERENCE_LATENCY_WARNING,
    SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN ,
    SPINTERFERENCE_LATENCY_TRUNCATE_END,
}}
ENUM!{enum SPENDSRSTREAMFLAGS {
    SPESF_NONE = 0,
    SPESF_STREAM_RELEASED = 1 << 0,
    SPESF_EMULATED = 1 << 1,
}}
ENUM!{enum SPVFEATURE {
    SPVFEATURE_STRESSED = 1 << 0,
    SPVFEATURE_EMPHASIS = 1 << 1,
}}
ENUM!{enum SPVISEMES {
    SP_VISEME_0 = 0,
    SP_VISEME_1,
    SP_VISEME_2,
    SP_VISEME_3,
    SP_VISEME_4,
    SP_VISEME_5,
    SP_VISEME_6,
    SP_VISEME_7,
    SP_VISEME_8,
    SP_VISEME_9,
    SP_VISEME_10,
    SP_VISEME_11,
    SP_VISEME_12,
    SP_VISEME_13,
    SP_VISEME_14,
    SP_VISEME_15,
    SP_VISEME_16,
    SP_VISEME_17,
    SP_VISEME_18,
    SP_VISEME_19,
    SP_VISEME_20,
    SP_VISEME_21,
}}
STRUCT!{struct SPEVENTSOURCEINFO {
    ullEventInterest: ULONGLONG,
    ullQueuedInterest: ULONGLONG,
    ulCount: ULONG,
}}
RIDL!{#[uuid(0xbe7a9cce, 0x5f9e, 0x11d2, 0x96, 0x0f, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpEventSource(ISpEventSourceVtbl): ISpNotifySource(ISpNotifySourceVtbl) {
    fn SetInterest(
        ullEventInterest: ULONGLONG,
        ullQueuedInterest: ULONGLONG,
    ) -> HRESULT,
    fn GetEvents(
        ulCount: ULONG,
        pEventArray: *mut SPEVENT,
        pulFetched: *mut ULONG,
    ) -> HRESULT,
    fn GetInfo(
        pInfo: *mut SPEVENTSOURCEINFO,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbe7a9cc9, 0x5f9e, 0x11d2, 0x96, 0x0f, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpEventSink(ISpEventSinkVtbl): IUnknown(IUnknownVtbl) {
    fn AddEvents(
        pEventArray: *const SPEVENT,
        ulCount: ULONG,
    ) -> HRESULT,
    fn GetEventInterest(
        pullEventInterest: *mut ULONGLONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbed530be, 0x2606, 0x4f4d, 0xa1, 0xc0, 0x54, 0xc5, 0xcd, 0xa5, 0x56, 0x6f)]
interface ISpStreamFormat(ISpStreamFormatVtbl): IStream(IStreamVtbl) {
    fn GetFormat(
        pguidFormatId: *mut GUID,
        ppCoMemWaveFormatEx: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
}}
ENUM!{enum SPFILEMODE {
    SPFM_OPEN_READONLY,
    SPFM_OPEN_READWRITE,
    SPFM_CREATE,
    SPFM_CREATE_ALWAYS,
    SPFM_NUM_MODES,
}}
RIDL!{#[uuid(0x12e3cca9, 0x7518, 0x44c5, 0xa5, 0xe7, 0xba, 0x5a, 0x79, 0xcb, 0x92, 0x9e)]
interface ISpStream(ISpStreamVtbl): ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetBaseStream(
        pStream: *mut IStream,
        rguidFormat: REFGUID,
        pWaveFormatEx: *const WAVEFORMATEX,
    ) -> HRESULT,
    fn GetBaseStream(
        ppStream: *mut *mut IStream,
    ) -> HRESULT,
    fn BindToFile(
        pszFileName: LPCWSTR,
        eMode: SPFILEMODE,
        pFormatId: *const GUID,
        pWaveFormatEx: *const WAVEFORMATEX,
        ullEventInterest: ULONGLONG,
    ) -> HRESULT,
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0x678a932c, 0xea71, 0x4446, 0x9b, 0x41, 0x78, 0xfd, 0xa6, 0x28, 0x0a, 0x29)]
interface ISpStreamFormatConverter(ISpStreamFormatConverterVtbl):
    ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetBaseStream(
        pStream: *mut ISpStreamFormat,
        fSetFormatToBaseStreamFormat: BOOL,
        fWriteToBaseStream: BOOL,
    ) -> HRESULT,
    fn GetBaseStream(
        ppStream: *mut *mut ISpStreamFormat,
    ) -> HRESULT,
    fn SetFormat(
        rguidFormatIdOfConvertedStream: REFGUID,
        pWaveFormatExOfConvertedStream: *const WAVEFORMATEX,
    ) -> HRESULT,
    fn ResetSeekPosition() -> HRESULT,
    fn ScaleConvertedToBaseOffset(
        ullOffsetConvertedStream: ULONGLONG,
        pullOffsetBaseStream: *mut ULONGLONG,
    ) -> HRESULT,
    fn ScaleBaseToConvertedOffset(
        ullOffsetBaseStream: ULONGLONG,
        pullOffsetConvertedStream: *mut ULONGLONG,
    ) -> HRESULT,
}}
ENUM!{enum SPAUDIOSTATE {
    SPAS_CLOSED,
    SPAS_STOP,
    SPAS_PAUSE,
    SPAS_RUN,
}}
STRUCT!{struct SPAUDIOSTATUS {
    cbFreeBuffSpace: c_long,
    cbNonBlockingIO: ULONG,
    State: SPAUDIOSTATE,
    CurSeekPos: ULONGLONG,
    CurDevicePos: ULONGLONG,
    dwAudioLevel: DWORD,
    dwReserved2: DWORD,
}}
STRUCT!{struct SPAUDIOBUFFERINFO {
    ulMsMinNotification: ULONG,
    ulMsBufferSize: ULONG,
    ulMsEventBias: ULONG,
}}
RIDL!{#[uuid(0xc05c768f, 0xfae8, 0x4ec2, 0x8e, 0x07, 0x33, 0x83, 0x21, 0xc1, 0x24, 0x52)]
interface ISpAudio(ISpAudioVtbl): ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetState(
        NewState: SPAUDIOSTATE,
        ullReserved: ULONGLONG,
    ) -> HRESULT,
    fn SetFormat(
        rguidFmtId: REFGUID,
        pWaveFormatEx: *const WAVEFORMATEX,
    ) -> HRESULT,
    fn GetStatus(
        pStatus: *mut SPAUDIOSTATUS,
    ) -> HRESULT,
    fn SetBufferInfo(
        pBuffInfo: *const SPAUDIOBUFFERINFO,
    ) -> HRESULT,
    fn GetBufferInfo(
        pBuffInfo: *mut SPAUDIOBUFFERINFO,
    ) -> HRESULT,
    fn GetDefaultFormat(
        pFormatId: *mut GUID,
        ppCoMemWaveFormatEx: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn EventHandle() -> HANDLE,
    fn GetVolumeLevel(
        pLevel: *mut ULONG,
    ) -> HRESULT,
    fn SetVolumeLevel(
        Level: ULONG,
    ) -> HRESULT,
    fn GetBufferNotifySize(
        pcbSize: *mut ULONG,
    ) -> HRESULT,
    fn SetBufferNotifySize(
        cbSize: ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x15806f6e, 0x1d70, 0x4b48, 0x98, 0xe6, 0x3b, 0x1a, 0x00, 0x75, 0x09, 0xab)]
interface ISpMMSysAudio(ISpMMSysAudioVtbl): ISpAudio(ISpAudioVtbl) {
    fn GetDeviceId(
        puDeviceId: *mut UINT,
    ) -> HRESULT,
    fn SetDeviceId(
        uDeviceId: UINT,
    ) -> HRESULT,
    fn GetMMHandle(
        pHandle: *mut *mut c_void,
    ) -> HRESULT,
    fn GetLineId(
        puLineId: *mut UINT,
    ) -> HRESULT,
    fn SetLineId(
        uLineId: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x10f63bce, 0x201a, 0x11d3, 0xac, 0x70, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpTranscript(ISpTranscriptVtbl): IUnknown(IUnknownVtbl) {
    fn GetTranscript(
        ppszTranscript: *mut LPWSTR,
    ) -> HRESULT,
    fn AppendTranscript(
        pszTranscript: LPCWSTR,
    ) -> HRESULT,
}}
ENUM!{enum SPDISPLAYATTRIBUTES {
    SPAF_ONE_TRAILING_SPACE = 0x2,
    SPAF_TWO_TRAILING_SPACES = 0x4,
    SPAF_CONSUME_LEADING_SPACES = 0x8,
    SPAF_ALL = 0xf,
}}
pub type SPPHONEID = WCHAR;
pub type PSPPHONEID = LPWSTR;
pub type PCSPPHONEID = LPCWSTR;
STRUCT!{struct SPPHRASEELEMENT {
    ulAudioTimeOffset: ULONG,
    ulAudioSizeTime: ULONG,
    ulAudioStreamOffset: ULONG,
    ulAudioSizeBytes: ULONG,
    ulRetainedStreamOffset: ULONG,
    ulRetainedSizeBytes: ULONG,
    pszDisplayText: LPCWSTR,
    pszLexicalForm: LPCWSTR,
    pszPronunciation: *const SPPHONEID,
    bDisplayAttributes: BYTE,
    RequiredConfidence: c_char,
    ActualConfidence: c_char,
    Reserved: BYTE,
    SREngineConfidence: c_float,
}}
STRUCT!{struct SPPHRASERULE {
    pszName: LPCWSTR,
    ulId: ULONG,
    ulFirstElement: ULONG,
    ulCountOfElements: ULONG,
    pNextSibling: *const SPPHRASERULE,
    pFirstChild: *const SPPHRASERULE,
    SREngineConfidence: c_float,
    Confidence: c_char,
}}
ENUM!{enum SPPHRASEPROPERTYUNIONTYPE {
    SPPPUT_UNUSED = 0,
    SPPPUT_ARRAY_INDEX,
}}
STRUCT!{struct SPPHRASEPROPERTY_u_s {
    bType: byte,
    bReserved: byte,
    usArrayIndex: c_ushort,
}}
UNION!{union SPPHRASEPROPERTY_u {
    [u32; 1],
    ulId ulId_mut: ULONG,
    s s_mut: SPPHRASEPROPERTY_u_s,
}}
STRUCT!{struct SPPHRASEPROPERTY {
    pszName: LPCWSTR,
    u: SPPHRASEPROPERTY_u_s,
    pszValue: LPCWSTR,
    vValue: VARIANT,
    ulFirstElement: ULONG,
    ulCountOfElements: ULONG,
    pNextSibling: *const SPPHRASEPROPERTY,
    pFirstChild: *const SPPHRASEPROPERTY,
    SREngineConfidence: c_float,
    Confidence: c_char,
}}
STRUCT!{struct SPPHRASEREPLACEMENT {
    bDisplayAttributes: BYTE,
    pszReplacementText: LPCWSTR,
    ulFirstElement: ULONG,
    ulCountOfElements: ULONG,
}}
STRUCT!{struct SPPHRASE {
    cbSize: ULONG,
    LangID: WORD,
    wHomophoneGroupId: WORD,
    ullGrammarID: ULONGLONG,
    ftStartTime: ULONGLONG,
    ullAudioStreamPosition: ULONGLONG,
    ulAudioSizeBytes: ULONG,
    ulRetainedSizeBytes: ULONG,
    ulAudioSizeTime: ULONG,
    Rule: SPPHRASERULE,
    pProperties: *const SPPHRASEPROPERTY,
    pElements: *const SPPHRASEELEMENT,
    cReplacements: ULONG,
    pReplacements: *const SPPHRASEREPLACEMENT,
    SREngineID: GUID,
    ulSREnginePrivateDataSize: ULONG,
    pSREnginePrivateData: *const BYTE,
}}
STRUCT!{struct SPSERIALIZEDPHRASE {
    ulSerializedSize: ULONG,
}}
ENUM!{enum SPVALUETYPE {
    SPDF_PROPERTY = 0x1,
    SPDF_REPLACEMENT = 0x2,
    SPDF_RULE = 0x4,
    SPDF_DISPLAYTEXT = 0x8,
    SPDF_LEXICALFORM = 0x10,
    SPDF_PRONUNCIATION = 0x20,
    SPDF_AUDIO = 0x40,
    SPDF_ALTERNATES = 0x80,
    SPDF_ALL = 0xff,
}}
STRUCT!{struct SPBINARYGRAMMAR {
    ulTotalSerializedSize: ULONG,
}}
ENUM!{enum SPPHRASERNG {
    SPPR_ALL_ELEMENTS = -1i32 as u32,
}}
pub const SP_GETWHOLEPHRASE: SPPHRASERNG = SPPR_ALL_ELEMENTS;
pub const SPRR_ALL_ELEMENTS: SPPHRASERNG = SPPR_ALL_ELEMENTS;
DECLARE_HANDLE!{SPSTATEHANDLE, SPSTATEHANDLE__}
ENUM!{enum SPRECOEVENTFLAGS {
    SPREF_AutoPause = 1 << 0,
    SPREF_Emulated = 1 << 1,
}}
ENUM!{enum SPPARTOFSPEECH {
    SPPS_NotOverriden = -1i32 as u32,
    SPPS_Unknown = 0,
    SPPS_Noun = 0x1000,
    SPPS_Verb = 0x2000,
    SPPS_Modifier = 0x3000,
    SPPS_Function = 0x4000,
    SPPS_Interjection = 0x5000,
}}
ENUM!{enum SPLEXICONTYPE {
    eLEXTYPE_USER = 1 << 0,
    eLEXTYPE_APP = 1 << 1,
    eLEXTYPE_VENDORLEXICON = 1 << 2,
    eLEXTYPE_LETTERTOSOUND = 1 << 3,
    eLEXTYPE_MORPHOLOGY = 1 << 4,
    eLEXTYPE_RESERVED4 = 1 << 5,
    eLEXTYPE_USER_SHORTCUT = 1 << 6,
    eLEXTYPE_RESERVED6 = 1 << 7,
    eLEXTYPE_RESERVED7 = 1 << 8,
    eLEXTYPE_RESERVED8 = 1 << 9,
    eLEXTYPE_RESERVED9 = 1 << 10,
    eLEXTYPE_RESERVED10 = 1 << 11,
    eLEXTYPE_PRIVATE1 = 1 << 12,
    eLEXTYPE_PRIVATE2 = 1 << 13,
    eLEXTYPE_PRIVATE3 = 1 << 14,
    eLEXTYPE_PRIVATE4 = 1 << 15,
    eLEXTYPE_PRIVATE5 = 1 << 16,
    eLEXTYPE_PRIVATE6 = 1 << 17,
    eLEXTYPE_PRIVATE7 = 1 << 18,
    eLEXTYPE_PRIVATE8 = 1 << 19,
    eLEXTYPE_PRIVATE9 = 1 << 20,
    eLEXTYPE_PRIVATE10 = 1 << 21,
    eLEXTYPE_PRIVATE11 = 1 << 22,
    eLEXTYPE_PRIVATE12 = 1 << 23,
    eLEXTYPE_PRIVATE13 = 1 << 24,
    eLEXTYPE_PRIVATE14 = 1 << 25,
    eLEXTYPE_PRIVATE15 = 1 << 26,
    eLEXTYPE_PRIVATE16 = 1 << 27,
    eLEXTYPE_PRIVATE17 = 1 << 28,
    eLEXTYPE_PRIVATE18 = 1 << 29,
    eLEXTYPE_PRIVATE19 = 1 << 30,
    eLEXTYPE_PRIVATE20 = 1 << 31,
}}
ENUM!{enum SPWORDTYPE {
    eWORDTYPE_ADDED = 1 << 0,
    eWORDTYPE_DELETED = 1 << 1,
}}
STRUCT!{struct SPWORDPRONUNCIATION {
    pNextWordPronunciation: *mut SPWORDPRONUNCIATION,
    eLexiconType: SPLEXICONTYPE,
    LangID: WORD,
    wPronunciationFlags: WORD,
    ePartOfSpeech: SPPARTOFSPEECH,
    szPronunciation: [SPPHONEID; 1],
}}
STRUCT!{struct SPWORDPRONUNCIATIONLIST {
    ulSize: ULONG,
    pvBuffer: *mut BYTE,
    pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}}
STRUCT!{struct SPWORD {
    pNextWord: *mut SPWORD,
    LangID: WORD,
    wReserved: WORD,
    eWordType: SPWORDTYPE,
    pszWord: LPWSTR,
    pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}}
STRUCT!{struct SPWORDLIST {
    ulSize: ULONG,
    pvBuffer: *mut BYTE,
    pFirstWord: *mut SPWORD,
}}
RIDL!{#[uuid(0xda41a7c2, 0x5383, 0x4db2, 0x91, 0x6b, 0x6c, 0x17, 0x19, 0xe3, 0xdb, 0x58)]
interface ISpLexicon(ISpLexiconVtbl): IUnknown(IUnknownVtbl) {
    fn GetPronunciations(
        pszWord: LPCWSTR,
        LangID: WORD,
        dwFlags: DWORD,
        pWordPronunciationList: *mut SPWORDPRONUNCIATIONLIST,
    ) -> HRESULT,
    fn AddPronunciation(
        pszWord: LPCWSTR,
        LangID: WORD,
        ePartOfSpeech: SPPARTOFSPEECH,
        pszPronunciation: PCSPPHONEID,
    ) -> HRESULT,
    fn RemovePronunciation(
        pszWord: LPCWSTR,
        LangID: WORD,
        ePartOfSpeech: SPPARTOFSPEECH,
        pszPronunciation: PCSPPHONEID,
    ) -> HRESULT,
    fn GetGeneration(
        pdwGeneration: *mut DWORD,
    ) -> HRESULT,
    fn GetGenerationChange(
        dwFlags: DWORD,
        pdwGeneration: *mut DWORD,
        pWordList: *mut SPWORDLIST,
    ) -> HRESULT,
    fn GetWords(
        dwFlags: DWORD,
        pdwGeneration: *mut DWORD,
        pdwCookie: *mut DWORD,
        pWordList: *mut SPWORDLIST,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8565572f, 0xc094, 0x41cc, 0xb5, 0x6e, 0x10, 0xbd, 0x9c, 0x3f, 0xf0, 0x44)]
interface ISpContainerLexicon(ISpContainerLexiconVtbl): ISpLexicon(ISpLexiconVtbl) {
    fn AddLexicon(
        pAddLexicon: *mut ISpLexicon,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8445c581, 0x0cac, 0x4a38, 0xab, 0xfe, 0x9b, 0x2c, 0xe2, 0x82, 0x64, 0x55)]
interface ISpPhoneConverter(ISpPhoneConverterVtbl): ISpObjectWithToken(ISpObjectWithTokenVtbl) {
    fn PhoneToId(
        pszPhone: LPCWSTR,
        pId: *mut SPPHONEID,
    ) -> HRESULT,
    fn IdToPhone(
        pId: PCSPPHONEID,
        pszPhone: *mut WCHAR,
    ) -> HRESULT,
}}
STRUCT!{struct SPVPITCH {
    MiddleAdj: c_long,
    RangeAdj: c_long,
}}
ENUM!{enum SPVACTIONS {
    SPVA_Speak = 0,
    SPVA_Silence,
    SPVA_Pronounce,
    SPVA_Bookmark,
    SPVA_SpellOut,
    SPVA_Section,
    SPVA_ParseUnknownTag,
}}
STRUCT!{struct SPVCONTEXT {
    pCategory: LPCWSTR,
    pBefore: LPCWSTR,
    pAfter: LPCWSTR,
}}
STRUCT!{struct SPVSTATE {
    eAction: SPVACTIONS,
    LangID: WORD,
    wReserved: WORD,
    EmphAdj: c_long,
    RateAdj: c_long,
    Volume: ULONG,
    PitchAdj: SPVPITCH,
    SilenceMSecs: ULONG,
    pPhoneIds: *mut SPPHONEID,
    ePartOfSpeech: SPPARTOFSPEECH,
    Context: SPVCONTEXT,
}}
ENUM!{enum SPRUNSTATE {
    SPRS_DONE = 1 << 0,
    SPRS_IS_SPEAKING = 1 << 1,
}}
ENUM!{enum SPVLIMITS {
    SPMIN_VOLUME = 0,
    SPMAX_VOLUME = 100,
    SPMIN_RATE = -10i32 as u32,
    SPMAX_RATE = 10,
}}
ENUM!{enum SPVPRIORITY {
    SPVPRI_NORMAL = 0,
    SPVPRI_ALERT = 1 << 0,
    SPVPRI_OVER = 1 << 1,
}}
STRUCT!{struct SPVOICESTATUS {
    ulCurrentStream: ULONG,
    ulLastStreamQueued: ULONG,
    hrLastResult: HRESULT,
    dwRunningState: DWORD,
    ulInputWordPos: ULONG,
    ulInputWordLen: ULONG,
    ulInputSentPos: ULONG,
    ulInputSentLen: ULONG,
    lBookmarkId: LONG,
    PhonemeId: SPPHONEID,
    VisemeId: SPVISEMES,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
}}
ENUM!{enum SPEAKFLAGS {
    SPF_DEFAULT = 0,
    SPF_ASYNC = 1 << 0,
    SPF_PURGEBEFORESPEAK = 1 << 1,
    SPF_IS_FILENAME = 1 << 2,
    SPF_IS_XML = 1 << 3,
    SPF_IS_NOT_XML = 1 << 4,
    SPF_PERSIST_XML = 1 << 5,
    SPF_NLP_SPEAK_PUNC = 1 << 6,
    SPF_NLP_MASK = SPF_NLP_SPEAK_PUNC,
    SPF_VOICE_MASK = SPF_ASYNC | SPF_PURGEBEFORESPEAK
        | SPF_IS_FILENAME | SPF_IS_XML | SPF_IS_NOT_XML
        | SPF_NLP_MASK | SPF_PERSIST_XML,
    SPF_UNUSED_FLAGS = !SPF_VOICE_MASK,
}}
RIDL!{#[uuid(0x6c44df74, 0x72b9, 0x4992, 0xa1, 0xec, 0xef, 0x99, 0x6e, 0x04, 0x22, 0xd4)]
interface ISpVoice(ISpVoiceVtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn SetOutput(
        pUnkOutput: *mut IUnknown,
        fAllowFormatChanges: BOOL,
    ) -> HRESULT,
    fn GetOutputObjectToken(
        ppObjectToken: *mut *mut ISpObjectToken,
    ) -> HRESULT,
    fn GetOutputStream(
        ppStream: *mut *mut ISpStreamFormat,
    ) -> HRESULT,
    fn Pause() -> HRESULT,
    fn Resume() -> HRESULT,
    fn SetVoice(
        pToken: *mut ISpObjectToken,
    ) -> HRESULT,
    fn GetVoice(
        ppToken: *mut *mut ISpObjectToken,
    ) -> HRESULT,
    fn Speak(
        pwcs: LPCWSTR,
        dwFlags: DWORD,
        pulStreamNumber: *mut ULONG,
    ) -> HRESULT,
    fn SpeakStream(
        pStream: *mut IStream,
        dwFlags: DWORD,
        pulStreamNumber: *mut ULONG,
    ) -> HRESULT,
    fn GetStatus(
        pStatus: *mut SPVOICESTATUS,
        ppszLastBookmark: *mut LPWSTR,
    ) -> HRESULT,
    fn Skip(
        pItemType: LPCWSTR,
        lNumItems: c_long,
        pulNumSkipped: *mut ULONG,
    ) -> HRESULT,
    fn SetPriority(
        ePriority: SPVPRIORITY,
    ) -> HRESULT,
    fn GetPriority(
        pePriority: *mut SPVPRIORITY,
    ) -> HRESULT,
    fn SetAlertBoundary(
        eBoundary: SPEVENTENUM,
    ) -> HRESULT,
    fn GetAlertBoundary(
        peBoundary: *mut SPEVENTENUM,
    ) -> HRESULT,
    fn SetRate(
        RateAdjust: c_long,
    ) -> HRESULT,
    fn GetRate(
        pRateAdjust: *mut c_long,
    ) -> HRESULT,
    fn SetVolume(
        usVolume: USHORT,
    ) -> HRESULT,
    fn GetVolume(
        pusVolume: *mut USHORT,
    ) -> HRESULT,
    fn WaitUntilDone(
        msTimeout: ULONG,
    ) -> HRESULT,
    fn SetSyncSpeakTimeout(
        msTimeout: ULONG,
    ) -> HRESULT,
    fn GetSyncSpeakTimeout(
        pmsTimeout: *mut ULONG,
    ) -> HRESULT,
    fn SpeakCompleteEvent() -> HANDLE,
    fn IsUISupported(
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        pfSupported: *mut BOOL,
    ) -> HRESULT,
    fn DisplayUI(
        hwndParent: HWND,
        pszTitle: LPCWSTR,
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1a5c0354, 0xb621, 0x4b5a, 0x87, 0x91, 0xd3, 0x06, 0xed, 0x37, 0x9e, 0x53)]
interface ISpPhrase(ISpPhraseVtbl): IUnknown(IUnknownVtbl) {
    fn GetPhrase(
        ppCoMemPhrase: *mut *mut SPPHRASE,
    ) -> HRESULT,
    fn GetSerializedPhrase(
        ppCoMemPhrase: *mut *mut SPSERIALIZEDPHRASE,
    ) -> HRESULT,
    fn GetText(
        ulStart: ULONG,
        ulCount: ULONG,
        fUseTextReplacements: BOOL,
        ppszCoMemText: *mut LPWSTR,
        pbDisplayAttributes: *mut BYTE,
    ) -> HRESULT,
    fn Discard(
        dwValueTypes: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8fcebc98, 0x4e49, 0x4067, 0x9c, 0x6c, 0xd8, 0x6a, 0x0e, 0x09, 0x2e, 0x3d)]
interface ISpPhraseAlt(ISpPhraseAltVtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetAltInfo(
        pParent: *mut *mut ISpPhrase,
        pulStartElementInParent: *mut ULONG,
        pcElementsInParent: *mut ULONG,
        pcElementsInAlt: *mut ULONG,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
}}
STRUCT!{struct SPRECORESULTTIMES {
    ftStreamTime: FILETIME,
    ullLength: ULONGLONG,
    dwTickCount: DWORD,
    ullStart: ULONGLONG,
}}
STRUCT!{struct SPSERIALIZEDRESULT {
    ulSerializedSize: ULONG,
}}
RIDL!{#[uuid(0x20b053be, 0xe235, 0x43cd, 0x9a, 0x2a, 0x8d, 0x17, 0xa4, 0x8b, 0x78, 0x42)]
interface ISpRecoResult(ISpRecoResultVtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetResultTimes(
        pTimes: *mut SPRECORESULTTIMES,
    ) -> HRESULT,
    fn GetAlternates(
        ulStartElement: ULONG,
        cElements: ULONG,
        ulRequestCount: ULONG,
        ppPhrases: *mut *mut ISpPhraseAlt,
        pcPhrasesReturned: *mut ULONG,
    ) -> HRESULT,
    fn GetAudio(
        ulStartElement: ULONG,
        cElements: ULONG,
        ppStream: *mut *mut ISpStreamFormat,
    ) -> HRESULT,
    fn SpeakAudio(
        ulStartElement: ULONG,
        cElements: ULONG,
        dwFlags: DWORD,
        pulStreamNumber: *mut ULONG,
    ) -> HRESULT,
    fn Serialize(
        ppCoMemSerializedResult: *mut *mut SPSERIALIZEDRESULT,
    ) -> HRESULT,
    fn ScaleAudio(
        pAudioFormatId: *const GUID,
        pWaveFormatEx: *const WAVEFORMATEX,
    ) -> HRESULT,
    fn GetRecoContext(
        ppRecoContext: *mut *mut ISpRecoContext,
    ) -> HRESULT,
}}
STRUCT!{struct SPTEXTSELECTIONINFO {
    ulStartActiveOffset: ULONG,
    cchActiveChars: ULONG,
    ulStartSelection: ULONG,
    cchSelection: ULONG,
}}
ENUM!{enum SPWORDPRONOUNCEABLE {
    SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE = 0,
    SPWP_UNKNOWN_WORD_PRONOUNCEABLE = 1,
    SPWP_KNOWN_WORD_PRONOUNCEABLE = 2,
}}
ENUM!{enum SPGRAMMARSTATE {
    SPGS_DISABLED = 0,
    SPGS_ENABLED = 1,
    SPGS_EXCLUSIVE = 3,
}}
ENUM!{enum SPCONTEXTSTATE {
    SPCS_DISABLED = 0,
    SPCS_ENABLED = 1,
}}
ENUM!{enum SPRULESTATE {
    SPRS_INACTIVE = 0,
    SPRS_ACTIVE = 1,
    SPRS_ACTIVE_WITH_AUTO_PAUSE = 3,
}}
pub const SP_STREAMPOS_ASAP: ULONGLONG = 0;
pub const SP_STREAMPOS_REALTIME: ULONGLONG = -1i64 as u64;
pub const SPRULETRANS_TEXTBUFFER: SPSTATEHANDLE = -1isize as SPSTATEHANDLE;
pub const SPRULETRANS_WILDCARD: SPSTATEHANDLE = -2isize as SPSTATEHANDLE;
pub const SPRULETRANS_DICTATION: SPSTATEHANDLE = -3isize as SPSTATEHANDLE;
ENUM!{enum SPGRAMMARWORDTYPE {
    SPWT_DISPLAY,
    SPWT_LEXICAL,
    SPWT_PRONUNCIATION,
    SPWT_LEXICAL_NO_SPECIAL_CHARS,
}}
STRUCT!{struct SPPROPERTYINFO {
    pszName: LPCWSTR,
    ulId: ULONG,
    pszValue: LPCWSTR,
    vValue: VARIANT,
}}
ENUM!{enum SPCFGRULEATTRIBUTES {
    SPRAF_TopLevel = 1 << 0,
    SPRAF_Active = 1 << 1,
    SPRAF_Export = 1 << 2,
    SPRAF_Import = 1 << 3,
    SPRAF_Interpreter = 1 << 4,
    SPRAF_Dynamic = 1 << 5,
    SPRAF_AutoPause = 1 << 16,
}}
RIDL!{#[uuid(0x8137828f, 0x591a, 0x4a42, 0xbe, 0x58, 0x49, 0xea, 0x7e, 0xba, 0xac, 0x68)]
interface ISpGrammarBuilder(ISpGrammarBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn ResetGrammar(
        NewLanguage: WORD,
    ) -> HRESULT,
    fn GetRule(
        pszRuleName: LPCWSTR,
        dwRuleId: DWORD,
        dwAttributes: DWORD,
        fCreateIfNotExist: BOOL,
        phInitialState: *mut SPSTATEHANDLE,
    ) -> HRESULT,
    fn ClearRule(
        hState: SPSTATEHANDLE,
    ) -> HRESULT,
    fn CreateNewState(
        hState: SPSTATEHANDLE,
        phState: *mut SPSTATEHANDLE,
    ) -> HRESULT,
    fn AddWordTransition(
        hFromState: SPSTATEHANDLE,
        hToState: SPSTATEHANDLE,
        psz: LPCWSTR,
        pszSeparators: LPCWSTR,
        eWordType: SPGRAMMARWORDTYPE,
        Weight: c_float,
        pPropInfo: *const SPPROPERTYINFO,
    ) -> HRESULT,
    fn AddRuleTransition(
        hFromState: SPSTATEHANDLE,
        hToState: SPSTATEHANDLE,
        hRule: SPSTATEHANDLE,
        Weight: c_float,
        pPropInfo: *const SPPROPERTYINFO,
    ) -> HRESULT,
    fn AddResource(
        hRuleState: SPSTATEHANDLE,
        pszResourceName: LPCWSTR,
        pszResourceValue: LPCWSTR,
    ) -> HRESULT,
    fn Commit(
        dwReserved: DWORD,
    ) -> HRESULT,
}}
ENUM!{enum SPLOADOPTIONS {
    SPLO_STATIC = 0,
    SPLO_DYNAMIC = 1,
}}
RIDL!{#[uuid(0x2177db29, 0x7f45, 0x47d0, 0x85, 0x54, 0x06, 0x7e, 0x91, 0xc8, 0x05, 0x02)]
interface ISpRecoGrammar(ISpRecoGrammarVtbl): ISpGrammarBuilder(ISpGrammarBuilderVtbl) {
    fn GetGrammarId(
        pullGrammarId: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetRecoContext(
        ppRecoCtxt: *mut *mut ISpRecoContext,
    ) -> HRESULT,
    fn LoadCmdFromFile(
        pszFileName: LPCWSTR,
        Options: SPLOADOPTIONS,
    ) -> HRESULT,
    fn LoadCmdFromObject(
        rcid: REFCLSID,
        pszGrammarName: LPCWSTR,
        Options: SPLOADOPTIONS,
    ) -> HRESULT,
    fn LoadCmdFromResource(
        hModule: HMODULE,
        pszResourceName: LPCWSTR,
        pszResourceType: LPCWSTR,
        wLanguage: WORD,
        Options: SPLOADOPTIONS,
    ) -> HRESULT,
    fn LoadCmdFromMemory(
        pGrammar: *const SPBINARYGRAMMAR,
        Options: SPLOADOPTIONS,
    ) -> HRESULT,
    fn LoadCmdFromProprietaryGrammar(
        rguidParam: REFGUID,
        pszStringParam: LPCWSTR,
        pvDataPrarm: *const c_void,
        cbDataSize: ULONG,
        Options: SPLOADOPTIONS,
    ) -> HRESULT,
    fn SetRuleState(
        pszName: LPCWSTR,
        pReserved: *mut c_void,
        NewState: SPRULESTATE,
    ) -> HRESULT,
    fn SetRuleIdState(
        ulRuleId: ULONG,
        NewState: SPRULESTATE,
    ) -> HRESULT,
    fn LoadDictation(
        pszTopicName: LPCWSTR,
        Options: SPLOADOPTIONS,
    ) -> HRESULT,
    fn UnloadDictation() -> HRESULT,
    fn SetDictationState(
        NewState: SPRULESTATE,
    ) -> HRESULT,
    fn SetWordSequenceData(
        pText: *const WCHAR,
        cchText: ULONG,
        pInfo: *const SPTEXTSELECTIONINFO,
    ) -> HRESULT,
    fn SetTextSelection(
        pInfo: *const SPTEXTSELECTIONINFO,
    ) -> HRESULT,
    fn IsPronounceable(
        pszWord: LPCWSTR,
        pWordPronounceable: *mut SPWORDPRONOUNCEABLE,
    ) -> HRESULT,
    fn SetGrammarState(
        eGrammarState: SPGRAMMARSTATE,
    ) -> HRESULT,
    fn SaveCmd(
        pStream: *mut IStream,
        ppszCoMemErrorText: *mut LPWSTR,
    ) -> HRESULT,
    fn GetGrammarState(
        peGrammarState: *mut SPGRAMMARSTATE,
    ) -> HRESULT,
}}
STRUCT!{struct SPRECOCONTEXTSTATUS {
    eInterference: SPINTERFERENCE,
    szRequestTypeOfUI: [WCHAR; 255],
    dwReserved1: DWORD,
    dwReserved2: DWORD,
}}
ENUM!{enum SPBOOKMARKOPTIONS {
    SPBO_NONE = 0,
    SPBO_PAUSE = 1 << 0,
}}
ENUM!{enum SPAUDIOOPTIONS {
    SPAO_NONE = 0,
    SPAO_RETAIN_AUDIO = 1 << 0,
}}
RIDL!{#[uuid(0xf740a62f, 0x7c15, 0x489e, 0x82, 0x34, 0x94, 0x0a, 0x33, 0xd9, 0x27, 0x2d)]
interface ISpRecoContext(ISpRecoContextVtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn GetRecognizer(
        ppRecognizer: *mut *mut ISpRecognizer,
    ) -> HRESULT,
    fn CreateGrammer(
        ullGrammarId: ULONGLONG,
        ppGrammar: *mut *mut ISpRecoGrammar,
    ) -> HRESULT,
    fn GetStatus(
        pState: *mut SPRECOCONTEXTSTATUS,
    ) -> HRESULT,
    fn GetMaxAlternates(
        pcAlternates: *mut ULONG,
    ) -> HRESULT,
    fn SetMaxAlternates(
        cAlternates: ULONG,
    ) -> HRESULT,
    fn SetAudioOptions(
        Options: SPAUDIOOPTIONS,
        pAudioFormatId: *const GUID,
        pWaveFormatEx: *const WAVEFORMATEX,
    ) -> HRESULT,
    fn GetAudioOptions(
        pOptions: *mut SPAUDIOOPTIONS,
        pAudioFormatId: *mut GUID,
        ppCoMemWFEX: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn DeserializeResult(
        pSerializedResult: *const SPSERIALIZEDRESULT,
        ppResult: *mut *mut ISpRecoResult,
    ) -> HRESULT,
    fn Bookmark(
        Options: SPBOOKMARKOPTIONS,
        ullStreamPosition: ULONGLONG,
        lparamEvent: LPARAM,
    ) -> HRESULT,
    fn SetAdaptionData(
        pAdaptionData: LPCWSTR,
        cch: ULONG,
    ) -> HRESULT,
    fn Pause(
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Resume(
        dwReserved: DWORD,
    ) -> HRESULT,
    fn SetVoice(
        pVoice: *mut ISpVoice,
        fAllowFormatChanges: BOOL,
    ) -> HRESULT,
    fn GetVoice(
        ppVoice: *mut *mut ISpVoice,
    ) -> HRESULT,
    fn SetVoicePurgeEvent(
        ullEventIntereset: ULONGLONG,
    ) -> HRESULT,
    fn GetVoicePurgeEvent(
        pullEventIntereset: *mut ULONGLONG,
    ) -> HRESULT,
    fn SetContextState(
        eContextState: SPCONTEXTSTATE,
    ) -> HRESULT,
    fn GetContextState(
        peContextState: *mut SPCONTEXTSTATE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5b4fb971, 0xb115, 0x4de1, 0xad, 0x97, 0xe4, 0x82, 0xe3, 0xbf, 0x6e, 0xe4)]
interface ISpProperties(ISpPropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn SetPropertyNum(
        pName: LPCWSTR,
        lValue: LONG,
    ) -> HRESULT,
    fn GetPropertyNum(
        pName: LPCWSTR,
        plValue: *mut LONG,
    ) -> HRESULT,
    fn SetPropertyString(
        pName: LPCWSTR,
        pValue: LPCWSTR,
    ) -> HRESULT,
    fn GetPropertyString(
        pName: LPCWSTR,
        ppCoMemValue: *mut LPWSTR,
    ) -> HRESULT,
}}
pub const SP_MAX_LANGIDS: usize = 20;
STRUCT!{struct SPRECOGNIZERSTATUS {
    AudioStatus: SPAUDIOSTATUS,
    ullRecognitionStreamPos: ULONGLONG,
    ulStreamNumber: ULONG,
    ulNumActive: ULONG,
    clsidEngine: CLSID,
    cLangIDs: ULONG,
    aLangID: [WORD; SP_MAX_LANGIDS],
    ullRecognitionStreamTime: ULONGLONG,
}}
ENUM!{enum SPWAVEFORMATTYPE {
    SPWF_INPUT,
    SPWF_SRENGINE,
}}
pub type SPSTREAMFORMATTYPE = SPWAVEFORMATTYPE;
ENUM!{enum SPRECOSTATE {
    SPRST_INACTIVE,
    SPRST_ACTIVE,
    SPRST_ACTIVE_ALWAYS,
    SPRST_INACTIVE_WITH_PURGE,
    SPRST_NUM_STATES,
}}
RIDL!{#[uuid(0xc2b5f241, 0xdaa0, 0x4507, 0x9e, 0x16, 0x5a, 0x1e, 0xaa, 0x2b, 0x7a, 0x5c)]
interface ISpRecognizer(ISpRecognizerVtbl): ISpProperties(ISpPropertiesVtbl) {
    fn SetRecognizer(
        pRecognizer: *mut ISpObjectToken,
    ) -> HRESULT,
    fn GetRecognizer(
        ppRecognizer: *mut *mut ISpObjectToken,
    ) -> HRESULT,
    fn SetInput(
        pUnkInput: *mut IUnknown,
        fAllowFormatChanges: BOOL,
    ) -> HRESULT,
    fn GetInputObjectToken(
        ppToken: *mut *mut ISpObjectToken,
    ) -> HRESULT,
    fn GetInputStream(
        ppStream: *mut *mut ISpStreamFormat,
    ) -> HRESULT,
    fn CreateRecoContext(
        ppNewCtxt: *mut *mut ISpRecoContext,
    ) -> HRESULT,
    fn GetRecoProfile(
        ppToken: *mut *mut ISpObjectToken,
    ) -> HRESULT,
    fn SetRecoProfile(
        pToken: *mut ISpObjectToken,
    ) -> HRESULT,
    fn IsSharedInstance() -> HRESULT,
    fn GetRecoState(
        pState: *mut SPRECOSTATE,
    ) -> HRESULT,
    fn SetRecoState(
        NewState: SPRECOSTATE,
    ) -> HRESULT,
    fn GetStatus(
        pStatus: *mut SPRECOGNIZERSTATUS,
    ) -> HRESULT,
    fn GetFormat(
        WaveFormatType: SPSTREAMFORMATTYPE,
        pFormatId: *mut GUID,
        ppCoMemWFEX: *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn IsUISupported(
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        pfSupported: *mut BOOL,
    ) -> HRESULT,
    fn DisplayUI(
        hwndParent: HWND,
        pszTitle: LPCWSTR,
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
    ) -> HRESULT,
    fn EmulateRecognition(
        pPhrase: *mut ISpPhrase,
    ) -> HRESULT,
}}
pub type SpeechLanguageId = c_long;
ENUM!{enum DISPID_SpeechDataKey {
    DISPID_SDKSetBinaryValue = 1,
    DISPID_SDKGetBinaryValue,
    DISPID_SDKSetStringValue,
    DISPID_SDKGetStringValue,
    DISPID_SDKSetLongValue,
    DISPID_SDKGetlongValue,
    DISPID_SDKOpenKey,
    DISPID_SDKCreateKey,
    DISPID_SDKDeleteKey,
    DISPID_SDKDeleteValue,
    DISPID_SDKEnumKeys,
    DISPID_SDKEnumValues,
}}
ENUM!{enum DISPID_SpeechObjectToken {
    DISPID_SOTId = 1,
    DISPID_SOTDataKey,
    DISPID_SOTCategory,
    DISPID_SOTGetDescription,
    DISPID_SOTSetId,
    DISPID_SOTGetAttribute,
    DISPID_SOTCreateInstance,
    DISPID_SOTRemove,
    DISPID_SOTGetStorageFileName,
    DISPID_SOTRemoveStorageFileName,
    DISPID_SOTIsUISupported,
    DISPID_SOTDisplayUI,
    DISPID_SOTMatchesAttributes,
}}
ENUM!{enum SpeechDataKeyLocation {
    SDKLDefaultLocation = SPDKL_DefaultLocation,
    SDKLCurrentUser = SPDKL_CurrentUser,
    SDKLLocalMachine = SPDKL_LocalMachine,
    SDKLCurrentConfig = SPDKL_CurrentConfig,
}}
ENUM!{enum SpeechTokenContext {
    STCInprocServer = CLSCTX_INPROC_SERVER,
    STCInprocHandler = CLSCTX_INPROC_HANDLER,
    STCLocalServer = CLSCTX_LOCAL_SERVER,
    STCRemoteServer = CLSCTX_REMOTE_SERVER,
    STCAll = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER
        | CLSCTX_LOCAL_SERVER | CLSCTX_REMOTE_SERVER,
}}
ENUM!{enum SpeechTokenShellFolder {
    STSF_AppData = 0x1a,
    STSF_LocalAppData = 0x1c,
    STSF_CommonAppData = 0x23,
    STSF_FlagCreate = 0x8000,
}}
ENUM!{enum DISPID_SpeechObjectTokens {
    DISPID_SOTsCount = 1,
    DISPID_SOTsItem = DISPID_VALUE as u32,
    DISPID_SOTs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechObjectTokenCategory {
    DISPID_SOTCId = 1,
    DISPID_SOTCDefault,
    DISPID_SOTCSetId,
    DISPID_SOTCGetDataKey,
    DISPID_SOTCEnumerateTokens,
}}
ENUM!{enum SpeechAudioFormatType {
    SAFTDefault = -1i32 as u32,
    SAFTNoAssignedFormat = 0,
    SAFTText = 1,
    SAFTNonStandardFormat = 2,
    SAFTExtendedAudioFormat = 3,
    SAFT8kHz8BitMono = 4,
    SAFT8kHz8BitStereo = 5,
    SAFT8kHz16BitMono = 6,
    SAFT8kHz16BitStereo = 7,
    SAFT11kHz8BitMono = 8,
    SAFT11kHz8BitStereo = 9,
    SAFT11kHz16BitMono = 10,
    SAFT11kHz16BitStereo = 11,
    SAFT12kHz8BitMono = 12,
    SAFT12kHz8BitStereo = 13,
    SAFT12kHz16BitMono = 14,
    SAFT12kHz16BitStereo = 15,
    SAFT16kHz8BitMono = 16,
    SAFT16kHz8BitStereo = 17,
    SAFT16kHz16BitMono = 18,
    SAFT16kHz16BitStereo = 19,
    SAFT22kHz8BitMono = 20,
    SAFT22kHz8BitStereo = 21,
    SAFT22kHz16BitMono = 22,
    SAFT22kHz16BitStereo = 23,
    SAFT24kHz8BitMono = 24,
    SAFT24kHz8BitStereo = 25,
    SAFT24kHz16BitMono = 26,
    SAFT24kHz16BitStereo = 27,
    SAFT32kHz8BitMono = 28,
    SAFT32kHz8BitStereo = 29,
    SAFT32kHz16BitMono = 30,
    SAFT32kHz16BitStereo = 31,
    SAFT44kHz8BitMono = 32,
    SAFT44kHz8BitStereo = 33,
    SAFT44kHz16BitMono = 34,
    SAFT44kHz16BitStereo = 35,
    SAFT48kHz8BitMono = 36,
    SAFT48kHz8BitStereo = 37,
    SAFT48kHz16BitMono = 38,
    SAFT48kHz16BitStereo = 39,
    SAFTTrueSpeech_8kHz1BitMono = 40,
    SAFTCCITT_ALaw_8kHzMono = 41,
    SAFTCCITT_ALaw_8kHzStereo = 42,
    SAFTCCITT_ALaw_11kHzMono = 43,
    SAFTCCITT_ALaw_11kHzStereo = 44,
    SAFTCCITT_ALaw_22kHzMono = 45,
    SAFTCCITT_ALaw_22kHzStereo = 46,
    SAFTCCITT_ALaw_44kHzMono = 47,
    SAFTCCITT_ALaw_44kHzStereo = 48,
    SAFTCCITT_uLaw_8kHzMono = 49,
    SAFTCCITT_uLaw_8kHzStereo = 50,
    SAFTCCITT_uLaw_11kHzMono = 51,
    SAFTCCITT_uLaw_11kHzStereo = 52,
    SAFTCCITT_uLaw_22kHzMono = 53,
    SAFTCCITT_uLaw_22kHzStereo = 54,
    SAFTCCITT_uLaw_44kHzMono = 55,
    SAFTCCITT_uLaw_44kHzStereo = 56,
    SAFTADPCM_8kHzMono = 57,
    SAFTADPCM_8kHzStereo = 58,
    SAFTADPCM_11kHzMono = 59,
    SAFTADPCM_11kHzStereo = 60,
    SAFTADPCM_22kHzMono = 61,
    SAFTADPCM_22kHzStereo = 62,
    SAFTADPCM_44kHzMono = 63,
    SAFTADPCM_44kHzStereo = 64,
    SAFTGSM610_8kHzMono = 65,
    SAFTGSM610_11kHzMono = 66,
    SAFTGSM610_22kHzMono = 67,
    SAFTGSM610_44kHzMono = 68,
}}
ENUM!{enum DISPID_SpeechAudioFormat {
    DISPID_SAFType = 1,
    DISPID_SAFGuid,
    DISPID_SAFGetWaveFormatEx,
    DISPID_SAFSetWaveFormatEx,
}}
ENUM!{enum DISPID_SpeechBaseStream {
    DISPID_SBSFormat = 1,
    DISPID_SBSRead,
    DISPID_SBSWrite,
    DISPID_SBSSeek,
}}
ENUM!{enum SpeechStreamSeekPositionType {
    SSSPTRelativeToStart = STREAM_SEEK_SET,
    SSSPTRelativeToCurrentPosition = STREAM_SEEK_CUR,
    SSSPTRelativeToEnd = STREAM_SEEK_END,
}}
ENUM!{enum DISPID_SpeechAudio {
    DISPID_SAStatus = 200,
    DISPID_SABufferInfo,
    DISPID_SADefaultFormat,
    DISPID_SAVolume,
    DISPID_SABufferNotifySize,
    DISPID_SAEventHandle,
    DISPID_SASetState,
}}
ENUM!{enum SpeechAudioState {
    SASClosed = SPAS_CLOSED,
    SASStop = SPAS_STOP,
    SASPause = SPAS_PAUSE,
    SASRun = SPAS_RUN,
}}
ENUM!{enum DISPID_SpeechMMSysAudio {
    DISPID_SMSADeviceId = 300,
    DISPID_SMSALineId,
    DISPID_SMSAMMHandle,
}}
ENUM!{enum DISPID_SpeechFileStream {
    DISPID_SFSOpen = 100,
    DISPID_SFSClose,
}}
ENUM!{enum SpeechStreamFileMode {
    SSFMOpenForRead = SPFM_OPEN_READONLY,
    SSFMOpenReadWrite = SPFM_OPEN_READWRITE,
    SSFMCreate = SPFM_CREATE,
    SSFMCreateForWrite = SPFM_CREATE_ALWAYS,
}}
ENUM!{enum DISPID_SpeechCustomStream {
    DISPID_SCSBaseStream = 100,
}}
ENUM!{enum DISPID_SpeechMemoryStream {
    DISPID_SMSSetData = 100,
    DISPID_SMSGetData,
}}
ENUM!{enum DISPID_SpeechAudioStatus {
    DISPID_SASFreeBufferSpace = 1,
    DISPID_SASNonBlockingIO,
    DISPID_SASState,
    DISPID_SASCurrentSeekPosition,
    DISPID_SASCurrentDevicePosition,
}}
ENUM!{enum DISPID_SpeechAudioBufferInfo {
    DISPID_SABIMinNotification = 1,
    DISPID_SABIBufferSize,
    DISPID_SABIEventBias,
}}
ENUM!{enum DISPID_SpeechWaveFormatEx {
    DISPID_SWFEFormatTag = 1,
    DISPID_SWFEChannels,
    DISPID_SWFESamplesPerSec,
    DISPID_SWFEAvgBytesPerSec,
    DISPID_SWFEBlockAlign,
    DISPID_SWFEBitsPerSample,
    DISPID_SWFEExtraData,
}}
ENUM!{enum DISPID_SpeechVoice {
    DISPID_SVStatus = 1,
    DISPID_SVVoice,
    DISPID_SVAudioOutput,
    DISPID_SVAudioOutputStream,
    DISPID_SVRate,
    DISPID_SVVolume,
    DISPID_SVAllowAudioOuputFormatChangesOnNextSet,
    DISPID_SVEventInterests,
    DISPID_SVPriority,
    DISPID_SVAlertBoundary,
    DISPID_SVSyncronousSpeakTimeout,
    DISPID_SVSpeak,
    DISPID_SVSpeakStream,
    DISPID_SVPause,
    DISPID_SVResume,
    DISPID_SVSkip,
    DISPID_SVGetVoices,
    DISPID_SVGetAudioOutputs,
    DISPID_SVWaitUntilDone,
    DISPID_SVSpeakCompleteEvent,
    DISPID_SVIsUISupported,
    DISPID_SVDisplayUI,
}}
ENUM!{enum SpeechVoicePriority {
    SVPNormal = SPVPRI_NORMAL,
    SVPAlert = SPVPRI_ALERT,
    SVPOver = SPVPRI_OVER,
}}
ENUM!{enum SpeechVoiceSpeakFlags {
    SVSFDefault = SPF_DEFAULT,
    SVSFlagsAsync = SPF_ASYNC,
    SVSFPurgeBeforeSpeak = SPF_PURGEBEFORESPEAK,
    SVSFIsFilename = SPF_IS_FILENAME,
    SVSFIsXML = SPF_IS_XML,
    SVSFIsNotXML = SPF_IS_NOT_XML,
    SVSFPersistXML = SPF_PERSIST_XML,
    SVSFNLPSpeakPunc = SPF_NLP_SPEAK_PUNC,
    SVSFNLPMask = SPF_NLP_MASK,
    SVSFVoiceMask = SPF_VOICE_MASK as u32,
    SVSFUnusedFlags = SPF_UNUSED_FLAGS as u32,
}}
ENUM!{enum SpeechVoiceEvents {
    SVEStartInputStream = 1 << 1,
    SVEEndInputStream = 1 << 2,
    SVEVoiceChange = 1 << 3,
    SVEBookmark = 1 << 4,
    SVEWordBoundary = 1 << 5,
    SVEPhoneme = 1 << 6,
    SVESentenceBoundary = 1 << 7,
    SVEViseme = 1 << 8,
    SVEAudioLevel = 1 << 9,
    SVEPrivate = 1 << 15,
    SVEAllEvents = 0x83fe,
}}
ENUM!{enum DISPID_SpeechVoiceStatus {
    DISPID_SVSCurrentStreamNumber = 1,
    DISPID_SVSLastStreamNumberQueued,
    DISPID_SVSLastResult,
    DISPID_SVSRunningState,
    DISPID_SVSInputWordPosition,
    DISPID_SVSInputWordLength,
    DISPID_SVSInputSentencePosition,
    DISPID_SVSInputSentenceLength,
    DISPID_SVSLastBookmark,
    DISPID_SVSLastBookmarkId,
    DISPID_SVSPhonemeId,
    DISPID_SVSVisemeId,
}}
ENUM!{enum SpeechRunState {
    SRSEDone = SPRS_DONE,
    SRSEIsSpeaking = SPRS_IS_SPEAKING,
}}
ENUM!{enum SpeechVisemeType {
    SVP_0 = 0,
    SVP_1,
    SVP_2,
    SVP_3,
    SVP_4,
    SVP_5,
    SVP_6,
    SVP_7,
    SVP_8,
    SVP_9,
    SVP_10,
    SVP_11,
    SVP_12,
    SVP_13,
    SVP_14,
    SVP_15,
    SVP_16,
    SVP_17,
    SVP_18,
    SVP_19,
    SVP_20,
    SVP_21,
}}
ENUM!{enum SpeechVisemeFeature {
    SVF_None = 0,
    SVF_Stressed = SPVFEATURE_STRESSED,
    SVF_Emphasis = SPVFEATURE_EMPHASIS,
}}
ENUM!{enum DISPID_SpeechVoiceEvent {
    DISPID_SVEStreamStart = 1,
    DISPID_SVEStreamEnd,
    DISPID_SVEVoiceChange,
    DISPID_SVEBookmark,
    DISPID_SVEWord,
    DISPID_SVEPhoneme,
    DISPID_SVESentenceBoundary,
    DISPID_SVEViseme,
    DISPID_SVEAudioLevel,
    DISPID_SVEEnginePrivate,
}}
ENUM!{enum DISPID_SpeechRecognizer {
    DISPID_SRRecognizer = 1,
    DISPID_SRAllowAudioInputFormatChangesOnNextSet,
    DISPID_SRAudioInput,
    DISPID_SRAudioInputStream,
    DISPID_SRIsShared,
    DISPID_SRState,
    DISPID_SRStatus,
    DISPID_SRProfile,
    DISPID_SREmulateRecognition,
    DISPID_SRCreateRecoContext,
    DISPID_SRGetFormat,
    DISPID_SRSetPropertyNumber,
    DISPID_SRGetPropertyNumber,
    DISPID_SRSetPropertyString,
    DISPID_SRGetPropertyString,
    DISPID_SRIsUISupported,
    DISPID_SRDisplayUI,
    DISPID_SRGetRecognizers,
    DISPID_SVGetAudioInputs,
    DISPID_SVGetProfiles,
}}
ENUM!{enum SpeechRecognizerState {
    SRSInactive = SPRST_INACTIVE,
    SRSActive = SPRST_ACTIVE,
    SRSActiveAlways = SPRST_ACTIVE_ALWAYS,
    SRSInactiveWithPurge = SPRST_INACTIVE_WITH_PURGE,
}}
ENUM!{enum SpeechDisplayAttributes {
    SDA_No_Trailing_Space = 0,
    SDA_One_Trailing_Space = SPAF_ONE_TRAILING_SPACE,
    SDA_Two_Trailing_Spaces = SPAF_TWO_TRAILING_SPACES,
    SDA_Consume_Leading_Spaces = SPAF_CONSUME_LEADING_SPACES,
}}
ENUM!{enum SpeechFormatType {
    SFTInput = SPWF_INPUT,
    SFTSREngine = SPWF_SRENGINE,
}}
ENUM!{enum DISPID_SpeechRecognizerStatus {
    DISPID_SRSAudioStatus = 1,
    DISPID_SRSCurrentStreamPosition,
    DISPID_SRSCurrentStreamNumber,
    DISPID_SRSNumberOfActiveRules,
    DISPID_SRSClsidEngine,
    DISPID_SRSSupportedLanguages,
}}
ENUM!{enum DISPID_SpeechRecoContext {
    DISPID_SRCRecognizer = 1,
    DISPID_SRCAudioInInterferenceStatus,
    DISPID_SRCRequestedUIType,
    DISPID_SRCVoice,
    DISPID_SRAllowVoiceFormatMatchingOnNextSet,
    DISPID_SRCVoicePurgeEvent,
    DISPID_SRCEventInterests,
    DISPID_SRCCmdMaxAlternates,
    DISPID_SRCState,
    DISPID_SRCRetainedAudio,
    DISPID_SRCRetainedAudioFormat,
    DISPID_SRCPause,
    DISPID_SRCResume,
    DISPID_SRCCreateGrammar,
    DISPID_SRCCreateResultFromMemory,
    DISPID_SRCBookmark,
    DISPID_SRCSetAdaptationData,
}}
ENUM!{enum SpeechRetainedAudioOptions {
    SRAONone = SPAO_NONE,
    SRAORetainAudio = SPAO_RETAIN_AUDIO,
}}
ENUM!{enum SpeechBookmarkOptions {
    SBONone = SPBO_NONE,
    SBOPause = SPBO_PAUSE,
}}
ENUM!{enum SpeechInterference {
    SINone = SPINTERFERENCE_NONE,
    SINoise = SPINTERFERENCE_NOISE,
    SINoSignal = SPINTERFERENCE_NOSIGNAL,
    SITooLoud = SPINTERFERENCE_TOOLOUD,
    SITooQuiet = SPINTERFERENCE_TOOQUIET,
    SITooFast = SPINTERFERENCE_TOOFAST,
    SITooSlow = SPINTERFERENCE_TOOSLOW,
}}
ENUM!{enum SpeechRecoEvents {
    SREStreamEnd = 1 << 0,
    SRESoundStart = 1 << 1,
    SRESoundEnd = 1 << 2,
    SREPhraseStart = 1 << 3,
    SRERecognition = 1 << 4,
    SREHypothesis = 1 << 5,
    SREBookmark = 1 << 6,
    SREPropertyNumChange = 1 << 7,
    SREPropertyStringChange = 1 << 8,
    SREFalseRecognition = 1 << 9,
    SREInterference = 1 << 10,
    SRERequestUI = 1 << 11,
    SREStateChange = 1 << 12,
    SREAdaptation = 1 << 13,
    SREStreamStart = 1 << 14,
    SRERecoOtherContext = 1 << 15,
    SREAudioLevel = 1 << 16,
    SREPrivate = 1 << 18,
    SREAllEvents = 0x5ffff,
}}
ENUM!{enum SpeechRecoContextState {
    SRCS_Disabled = SPCS_DISABLED,
    SRCS_Enabled = SPCS_ENABLED,
}}
ENUM!{enum DISPIDSPRG {
    DISPID_SRGId = 1,
    DISPID_SRGRecoContext,
    DISPID_SRGState,
    DISPID_SRGRules,
    DISPID_SRGReset,
    DISPID_SRGCommit,
    DISPID_SRGCmdLoadFromFile,
    DISPID_SRGCmdLoadFromObject,
    DISPID_SRGCmdLoadFromResource,
    DISPID_SRGCmdLoadFromMemory,
    DISPID_SRGCmdLoadFromProprietaryGrammar,
    DISPID_SRGCmdSetRuleState,
    DISPID_SRGCmdSetRuleIdState,
    DISPID_SRGDictationLoad,
    DISPID_SRGDictationUnload,
    DISPID_SRGDictationSetState,
    DISPID_SRGSetWordSequenceData,
    DISPID_SRGSetTextSelection,
    DISPID_SRGIsPronounceable,
}}
ENUM!{enum SpeechLoadOption {
    SLOStatic = SPLO_STATIC,
    SLODynamic = SPLO_DYNAMIC,
}}
ENUM!{enum SpeechWordPronounceable {
    SWPUnknownWordUnpronounceable = SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE,
    SWPUnknownWordPronounceable = SPWP_UNKNOWN_WORD_PRONOUNCEABLE,
    SWPKnownWordPronounceable = SPWP_KNOWN_WORD_PRONOUNCEABLE,
}}
ENUM!{enum SpeechGrammarState {
    SGSEnabled = SPGS_ENABLED,
    SGSDisabled = SPGS_DISABLED,
    SGSExclusive = SPGS_EXCLUSIVE,
}}
ENUM!{enum SpeechRuleState {
    SGDSInactive = SPRS_INACTIVE,
    SGDSActive = SPRS_ACTIVE,
    SGDSActiveWithAutoPause = SPRS_ACTIVE_WITH_AUTO_PAUSE,
}}
ENUM!{enum SpeechRuleAttributes {
    SRATopLevel = SPRAF_TopLevel,
    SRADefaultToActive = SPRAF_Active,
    SRAExport = SPRAF_Export,
    SRAImport = SPRAF_Import,
    SRAInterpreter = SPRAF_Interpreter,
    SRADynamic = SPRAF_Dynamic,
}}
ENUM!{enum SpeechGrammarWordType {
    SGDisplay = SPWT_DISPLAY,
    SGLexical = SPWT_LEXICAL,
    SGPronounciation = SPWT_PRONUNCIATION,
}}
ENUM!{enum DISPID_SpeechRecoContextEvents {
    DISPID_SRCEStartStream = 1,
    DISPID_SRCEEndStream,
    DISPID_SRCEBookmark,
    DISPID_SRCESoundStart,
    DISPID_SRCESoundEnd,
    DISPID_SRCEPhraseStart,
    DISPID_SRCERecognition,
    DISPID_SRCEHypothesis,
    DISPID_SRCEPropertyNumberChange,
    DISPID_SRCEPropertyStringChange,
    DISPID_SRCEFalseRecognition,
    DISPID_SRCEInterference,
    DISPID_SRCERequestUI,
    DISPID_SRCERecognizerStateChange,
    DISPID_SRCEAdaptation,
    DISPID_SRCERecognitionForOtherContext,
    DISPID_SRCEAudioLevel,
    DISPID_SRCEEnginePrivate,
}}
ENUM!{enum SpeechRecognitionType {
    SRTStandard = 0,
    SRTAutopause = SPREF_AutoPause,
    SRTEmulated = SPREF_Emulated,
}}
ENUM!{enum DISPID_SpeechGrammarRule {
    DISPID_SGRAttributes = 1,
    DISPID_SGRInitialState,
    DISPID_SGRName,
    DISPID_SGRId,
    DISPID_SGRClear,
    DISPID_SGRAddResource,
    DISPID_SGRAddState,
}}
ENUM!{enum DISPID_SpeechGrammarRules {
    DISPID_SGRsCount = 1,
    DISPID_SGRsDynamic,
    DISPID_SGRsAdd,
    DISPID_SGRsCommit,
    DISPID_SGRsCommitAndSave,
    DISPID_SGRsFindRule,
    DISPID_SGRsItem = DISPID_VALUE as u32,
    DISPID_SGRs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechGrammarRuleState {
    DISPID_SGRSRule = 1,
    DISPID_SGRSTransitions,
    DISPID_SGRSAddWordTransition,
    DISPID_SGRSAddRuleTransition,
    DISPID_SGRSAddSpecialTransition,
}}
ENUM!{enum SpeechSpecialTransitionType {
    SSTTWildcard = 1,
    SSTTDictation,
    SSTTTextBuffer,
}}
ENUM!{enum DISPID_SpeechGrammarRuleStateTransitions {
    DISPID_SGRSTsCount = 1,
    DISPID_SGRSTsItem = DISPID_VALUE as u32,
    DISPID_SGRSTs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechGrammarRuleStateTransition {
    DISPID_SGRSTType = 1,
    DISPID_SGRSTText,
    DISPID_SGRSTRule,
    DISPID_SGRSTWeight,
    DISPID_SGRSTPropertyName,
    DISPID_SGRSTPropertyId,
    DISPID_SGRSTPropertyValue,
    DISPID_SGRSTNextState,
}}
ENUM!{enum SpeechGrammarRuleStateTransitionType {
    SGRSTTEpsilon = 0,
    SGRSTTWord,
    SGRSTTRule,
    SGRSTTDictation,
    SGRSTTWildcard,
    SGRSTTTextBuffer,
}}
ENUM!{enum DISPIDSPTSI {
    DISPIDSPTSI_ActiveOffset = 1,
    DISPIDSPTSI_ActiveLength,
    DISPIDSPTSI_SelectionOffset,
    DISPIDSPTSI_SelectionLength,
}}
ENUM!{enum DISPID_SpeechRecoResult {
    DISPID_SRRRecoContext = 1,
    DISPID_SRRTimes,
    DISPID_SRRAudioFormat,
    DISPID_SRRPhraseInfo,
    DISPID_SRRAlternates,
    DISPID_SRRAudio,
    DISPID_SRRSpeakAudio,
    DISPID_SRRSaveToMemory,
    DISPID_SRRDiscardResultInfo,
}}
ENUM!{enum SpeechDiscardType {
    SDTProperty = SPDF_PROPERTY,
    SDTReplacement = SPDF_REPLACEMENT,
    SDTRule = SPDF_RULE,
    SDTDisplayText = SPDF_DISPLAYTEXT,
    SDTLexicalForm = SPDF_LEXICALFORM,
    SDTPronunciation = SPDF_PRONUNCIATION,
    SDTAudio = SPDF_AUDIO,
    SDTAlternates = SPDF_ALTERNATES,
    SDTAll = SPDF_ALL,
}}
ENUM!{enum DISPID_SpeechPhraseBuilder {
    DISPID_SPPBRestorePhraseFromMemory = 1,
}}
ENUM!{enum DISPID_SpeechRecoResultTimes {
    DISPID_SRRTStreamTime = 1,
    DISPID_SRRTLength,
    DISPID_SRRTTickCount,
    DISPID_SRRTOffsetFromStart,
}}
ENUM!{enum DISPID_SpeechPhraseAlternate {
    DISPID_SPARecoResult = 1,
    DISPID_SPAStartElementInResult,
    DISPID_SPANumberOfElementsInResult,
    DISPID_SPAPhraseInfo,
    DISPID_SPACommit,
}}
ENUM!{enum DISPID_SpeechPhraseAlternates {
    DISPID_SPAsCount = 1,
    DISPID_SPAsItem = DISPID_VALUE as u32,
    DISPID_SPAs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseInfo {
    DISPID_SPILanguageId = 1,
    DISPID_SPIGrammarId,
    DISPID_SPIStartTime,
    DISPID_SPIAudioStreamPosition,
    DISPID_SPIAudioSizeBytes,
    DISPID_SPIRetainedSizeBytes,
    DISPID_SPIAudioSizeTime,
    DISPID_SPIRule,
    DISPID_SPIProperties,
    DISPID_SPIElements,
    DISPID_SPIReplacements,
    DISPID_SPIEngineId,
    DISPID_SPIEnginePrivateData,
    DISPID_SPISaveToMemory,
    DISPID_SPIGetText,
    DISPID_SPIGetDisplayAttributes,
}}
ENUM!{enum DISPID_SpeechPhraseElement {
    DISPID_SPEAudioTimeOffset = 1,
    DISPID_SPEAudioSizeTime,
    DISPID_SPEAudioStreamOffset,
    DISPID_SPEAudioSizeBytes,
    DISPID_SPERetainedStreamOffset,
    DISPID_SPERetainedSizeBytes,
    DISPID_SPEDisplayText,
    DISPID_SPELexicalForm,
    DISPID_SPEPronunciation,
    DISPID_SPEDisplayAttributes,
    DISPID_SPERequiredConfidence,
    DISPID_SPEActualConfidence,
    DISPID_SPEEngineConfidence,
}}
ENUM!{enum SpeechEngineConfidence {
    SECLowConfidence = -1i32 as u32,
    SECNormalConfidence = 0,
    SECHighConfidence = 1,
}}
ENUM!{enum DISPID_SpeechPhraseElements {
    DISPID_SPEsCount = 1,
    DISPID_SPEsItem = DISPID_VALUE as u32,
    DISPID_SPEs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseReplacement {
    DISPID_SPRDisplayAttributes = 1,
    DISPID_SPRText,
    DISPID_SPRFirstElement,
    DISPID_SPRNumberOfElements,
}}
ENUM!{enum DISPID_SpeechPhraseReplacements {
    DISPID_SPRsCount = 1,
    DISPID_SPRsItem = DISPID_VALUE as u32,
    DISPID_SPRs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseProperty {
    DISPID_SPPName = 1,
    DISPID_SPPId,
    DISPID_SPPValue,
    DISPID_SPPFirstElement,
    DISPID_SPPNumberOfElements,
    DISPID_SPPEngineConfidence,
    DISPID_SPPConfidence,
    DISPID_SPPParent,
    DISPID_SPPChildren,
}}
ENUM!{enum DISPID_SpeechPhraseProperties {
    DISPID_SPPsCount = 1,
    DISPID_SPPsItem = DISPID_VALUE as u32,
    DISPID_SPPs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseRule {
    DISPID_SPRuleName = 1,
    DISPID_SPRuleId,
    DISPID_SPRuleFirstElement,
    DISPID_SPRuleNumberOfElements,
    DISPID_SPRuleParent,
    DISPID_SPRuleChildren,
    DISPID_SPRuleConfidence,
    DISPID_SPRuleEngineConfidence,
}}
ENUM!{enum DISPID_SpeechPhraseRules {
    DISPID_SPRulesCount = 1,
    DISPID_SPRulesItem = DISPID_VALUE as u32,
    DISPID_SPRules_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechLexicon {
    DISPID_SLGenerationId = 1,
    DISPID_SLGetWords,
    DISPID_SLAddPronunciation,
    DISPID_SLAddPronunciationByPhoneIds,
    DISPID_SLRemovePronunciation,
    DISPID_SLRemovePronunciationByPhoneIds,
    DISPID_SLGetPronunciations,
    DISPID_SLGetGenerationChange,
}}
ENUM!{enum SpeechLexiconType {
    SLTUser = eLEXTYPE_USER,
    SLTApp = eLEXTYPE_APP,
}}
ENUM!{enum SpeechPartOfSpeech {
    SPSNotOverriden = SPPS_NotOverriden,
    SPSUnknown = SPPS_Unknown,
    SPSNoun = SPPS_Noun,
    SPSVerb = SPPS_Verb,
    SPSModifier = SPPS_Modifier,
    SPSFunction = SPPS_Function,
    SPSInterjection = SPPS_Interjection,
}}
ENUM!{enum DISPID_SpeechLexiconWords {
    DISPID_SLWsCount = 1,
    DISPID_SLWsItem = DISPID_VALUE as u32,
    DISPID_SLWs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum SpeechWordType {
    SWTAdded = eWORDTYPE_ADDED,
    SWTDeleted = eWORDTYPE_DELETED,
}}
ENUM!{enum DISPID_SpeechLexiconWord {
    DISPID_SLWLangId = 1,
    DISPID_SLWType,
    DISPID_SLWWord,
    DISPID_SLWPronunciations,
}}
ENUM!{enum DISPID_SpeechLexiconProns {
    DISPID_SLPsCount = 1,
    DISPID_SLPsItem = DISPID_VALUE as u32,
    DISPID_SLPs_NewEnum = DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechLexiconPronunciation {
    DISPID_SLPType = 1,
    DISPID_SLPLangId,
    DISPID_SLPPartOfSpeech,
    DISPID_SLPPhoneIds,
    DISPID_SLPSymbolic,
}}
ENUM!{enum DISPID_SpeechPhoneConverter {
    DISPID_SPCLangId = 1,
    DISPID_SPCPhoneToId,
    DISPID_SPCIdToPhone,
}}
extern {
    pub static LIBID_SpeechLib: IID;
}
RIDL!{#[uuid(0xce17c09b, 0x4efa, 0x44d5, 0xa4, 0xc9, 0x59, 0xd9, 0x58, 0x5a, 0xb0, 0xcd)]
interface ISpeechDataKey(ISpeechDataKeyVtbl): IDispatch(IDispatchVtbl) {
    fn SetBinaryValue(
        ValueName: BSTR,
        Value: VARIANT,
    ) -> HRESULT,
    fn GetBinaryValue(
        ValueName: BSTR,
        Value: *mut VARIANT,
    ) -> HRESULT,
    fn SetStringValue(
        ValueName: BSTR,
        Value: BSTR,
    ) -> HRESULT,
    fn GetStringValue(
        ValueName: BSTR,
        Value: *mut BSTR,
    ) -> HRESULT,
    fn SetLongValue(
        ValueName: BSTR,
        Value: c_long,
    ) -> HRESULT,
    fn GetLongValue(
        ValueName: BSTR,
        Value: *mut c_long,
    ) -> HRESULT,
    fn OpenKey(
        SubKeyName: BSTR,
        SubKey: *mut *mut ISpeechDataKey,
    ) -> HRESULT,
    fn CreateKey(
        SubKeyName: BSTR,
        SubKey: *mut *mut ISpeechDataKey,
    ) -> HRESULT,
    fn DeleteKey(
        SubKeyName: BSTR,
    ) -> HRESULT,
    fn DeleteValue(
        ValueName: BSTR,
    ) -> HRESULT,
    fn EnumKeys(
        Index: c_long,
        SubKeyName: *mut BSTR,
    ) -> HRESULT,
    fn EnumValues(
        Index: c_long,
        ValueName: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc74a3adc, 0xb727, 0x4500, 0xa8, 0x4a, 0xb5, 0x26, 0x72, 0x1c, 0x8b, 0x8c)]
interface ISpeechObjectToken(ISpeechObjectTokenVtbl): IDispatch(IDispatchVtbl) {
    fn get_Id(
        ObjectId: *mut BSTR,
    ) -> HRESULT,
    fn get_DataKey(
        DataKey: *mut *mut ISpeechDataKey,
    ) -> HRESULT,
    fn get_Category(
        Category: *mut *mut ISpeechObjectTokenCategory,
    ) -> HRESULT,
    fn GetDescription(
        Locale: c_long,
        Description: *mut BSTR,
    ) -> HRESULT,
    fn SetId(
        Id: BSTR,
        CategoryId: BSTR,
        CreateIfNotExist: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetAttribute(
        AttributeName: BSTR,
        AttributeValue: *mut BSTR,
    ) -> HRESULT,
    fn CreateInstance(
        pUnkOuter: *mut IUnknown,
        ClsContext: SpeechTokenContext,
        Object: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Remove(
        ObjectStorageCLSID: BSTR,
    ) -> HRESULT,
    fn GetStorageFileName(
        ObjectStorageCLSID: BSTR,
        KeyName: BSTR,
        FileName: BSTR,
        Folder: BSTR,
        FilePath: *mut BSTR,
    ) -> HRESULT,
    fn RemoveStorageFileName(
        ObjectStorageCLSID: BSTR,
        KeyName: BSTR,
        DeleteFile: VARIANT_BOOL,
    ) -> HRESULT,
    fn IsUISupported(
        TypeOfUI: BSTR,
        ExtraData: *const VARIANT,
        Object: *mut IUnknown,
        Supported: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DisplayUI(
        hWnd: c_long,
        Title: BSTR,
        TypeOfUI: BSTR,
        ExtraData: *const VARIANT,
        Object: *mut IUnknown,
    ) -> HRESULT,
    fn MatchesAttributes(
        Attributes: BSTR,
        Matches: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9285b776, 0x2e7b, 0x4bc0, 0xb5, 0x3e, 0x58, 0x0e, 0xb6, 0xfa, 0x96, 0x7f)]
interface ISpeechObjectTokens(ISpeechObjectTokensVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Token: *mut *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn get__NewEnum(
        ppEnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xca7eac50, 0x2d01, 0x4145, 0x86, 0xd4, 0x5a, 0xe7, 0xd7, 0x0f, 0x44, 0x69)]
interface ISpeechObjectTokenCategory(ISpeechObjectTokenCategoryVtbl): IDispatch(IDispatchVtbl) {
    fn get_Id(
        Id: *mut BSTR,
    ) -> HRESULT,
    fn put_Default(
        TokenId: BSTR,
    ) -> HRESULT,
    fn get_Default(
        TokenId: *mut BSTR,
    ) -> HRESULT,
    fn SetId(
        Id: BSTR,
        CreateIfNotExist: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetDataKey(
        Location: SpeechDataKeyLocation,
        DataKey: *mut *mut ISpeechDataKey,
    ) -> HRESULT,
    fn EnumerateTokens(
        RequiredAttributes: BSTR,
        OptionalAttributes: BSTR,
        Tokens: *mut *mut ISpeechObjectTokens,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x11b103d8, 0x1142, 0x4edf, 0xa0, 0x93, 0x82, 0xfb, 0x39, 0x15, 0xf8, 0xcc)]
interface ISpeechAudioBufferInfo(ISpeechAudioBufferInfoVtbl): IDispatch(IDispatchVtbl) {
    fn get_MinNotification(
        MinNotification: *mut c_long,
    ) -> HRESULT,
    fn put_MinNotification(
        MinNotification: c_long,
    ) -> HRESULT,
    fn get_BufferSize(
        BufferSize: *mut c_long,
    ) -> HRESULT,
    fn put_BufferSize(
        BufferSize: c_long,
    ) -> HRESULT,
    fn get_EventBias(
        EventBias: *mut c_long,
    ) -> HRESULT,
    fn put_EventBias(
        EventBias: c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc62d9c91, 0x7458, 0x47f6, 0x86, 0x2d, 0x1e, 0xf8, 0x6f, 0xb0, 0xb2, 0x78)]
interface ISpeechAudioStatus(ISpeechAudioStatusVtbl): IDispatch(IDispatchVtbl) {
    fn get_FreeBufferSpace(
        FreeBufferSpace: *mut c_long,
    ) -> HRESULT,
    fn get_NonBlockingIO(
        NonBlockingIO: *mut c_long,
    ) -> HRESULT,
    fn get_State(
        State: *mut SpeechAudioState,
    ) -> HRESULT,
    fn get_CurrentSeekPosition(
        CurrentSeekPosition: *mut VARIANT,
    ) -> HRESULT,
    fn get_CurrentDevicePosition(
        CurrentDevicePosition: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe6e9c590, 0x3e18, 0x40e3, 0x82, 0x99, 0x06, 0x1f, 0x98, 0xbd, 0xe7, 0xc7)]
interface ISpeechAudioFormat(ISpeechAudioFormatVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        AudioFormat: *mut SpeechAudioFormatType,
    ) -> HRESULT,
    fn put_Type(
        AudioFormat: SpeechAudioFormatType,
    ) -> HRESULT,
    fn get_Guid(
        Guid: *mut BSTR,
    ) -> HRESULT,
    fn put_Guid(
        Guid: BSTR,
    ) -> HRESULT,
    fn GetWaveFormatEx(
        SpeechWaveFormatEx: *mut *mut ISpeechWaveFormatEx,
    ) -> HRESULT,
    fn SetWaveFormatEx(
        SpeechWaveFormatEx: *mut ISpeechWaveFormatEx,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7a1ef0d5, 0x1581, 0x4741, 0x88, 0xe4, 0x20, 0x9a, 0x49, 0xf1, 0x1a, 0x10)]
interface ISpeechWaveFormatEx(ISpeechWaveFormatExVtbl): IDispatch(IDispatchVtbl) {
    fn get_FormatTag(
        FormatTag: *mut c_short,
    ) -> HRESULT,
    fn put_FormatTag(
        FormatTag: c_short,
    ) -> HRESULT,
    fn get_Channels(
        Channels: *mut c_short,
    ) -> HRESULT,
    fn put_Channels(
        Channels: c_short,
    ) -> HRESULT,
    fn get_SamplesPerSec(
        SamplesPerSec: *mut c_long,
    ) -> HRESULT,
    fn put_SamplesPerSec(
        SamplesPerSec: c_long,
    ) -> HRESULT,
    fn get_AvgBytesPerSec(
        AvgBytesPerSec: *mut c_long,
    ) -> HRESULT,
    fn put_AvgBytesPerSec(
        AvgBytesPerSec: c_long,
    ) -> HRESULT,
    fn get_BlockAlign(
        BlockAlign: *mut c_short,
    ) -> HRESULT,
    fn put_BlockAlign(
        BlockAlign: c_short,
    ) -> HRESULT,
    fn get_BitsPerSample(
        BitsPerSample: *mut c_short,
    ) -> HRESULT,
    fn put_BitsPerSample(
        BitsPerSample: c_short,
    ) -> HRESULT,
    fn get_ExtraData(
        ExtraData: *mut VARIANT,
    ) -> HRESULT,
    fn put_ExtraData(
        ExtraData: VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6450336f, 0x7d49, 0x4ced, 0x80, 0x97, 0x49, 0xd6, 0xde, 0xe3, 0x72, 0x94)]
interface ISpeechBaseStream(ISpeechBaseStreamVtbl): IDispatch(IDispatchVtbl) {
    fn get_Format(
        AudioFormat: *mut *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn putref_Format(
        AudioFormat: *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn Read(
        Buffer: *mut VARIANT,
        NumberOfBytes: c_long,
        BytesRead: *mut c_long,
    ) -> HRESULT,
    fn Write(
        Buffer: VARIANT,
        BytesWritten: *mut c_long,
    ) -> HRESULT,
    fn Seek(
        Position: VARIANT,
        Origin: SpeechStreamSeekPositionType,
        NewPosition: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xaf67f125, 0xab39, 0x4e93, 0xb4, 0xa2, 0xcc, 0x2e, 0x66, 0xe1, 0x82, 0xa7)]
interface ISpeechFileStream(ISpeechFileStreamVtbl): ISpeechBaseStream(ISpeechBaseStreamVtbl) {
    fn Open(
        FileName: BSTR,
        FileMode: SpeechStreamFileMode,
        DoEvents: VARIANT_BOOL,
    ) -> HRESULT,
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0xeeb14b68, 0x808b, 0x4abe, 0xa5, 0xea, 0xb5, 0x1d, 0xa7, 0x58, 0x80, 0x08)]
interface ISpeechMemoryStream(ISpeechMemoryStreamVtbl): ISpeechBaseStream(ISpeechBaseStreamVtbl) {
    fn SetData(
        Data: VARIANT,
    ) -> HRESULT,
    fn GetData(
        pData: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1a9e9f4f, 0x104f, 0x4db8, 0xa1, 0x15, 0xef, 0xd7, 0xfd, 0x0c, 0x97, 0xae)]
interface ISpeechCustomStream(ISpeechCustomStreamVtbl): ISpeechBaseStream(ISpeechBaseStreamVtbl) {
    fn get_BaseStream(
        ppUnkStream: *mut *mut IUnknown,
    ) -> HRESULT,
    fn putref_BaseStream(
        pUnkStream: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcff8e175, 0x019e, 0x11d3, 0xa0, 0x8e, 0x00, 0xc0, 0x4f, 0x8e, 0xf9, 0xb5)]
interface ISpeechAudio(ISpeechAudioVtbl): ISpeechBaseStream(ISpeechBaseStreamVtbl) {
    fn get_Status(
        Status: *mut *mut ISpeechAudioStatus,
    ) -> HRESULT,
    fn get_BufferInfo(
        BufferInfo: *mut *mut ISpeechAudioBufferInfo,
    ) -> HRESULT,
    fn get_DefaultFormat(
        StreamFormat: *mut *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn get_Volume(
        Volume: *mut c_long,
    ) -> HRESULT,
    fn put_Volume(
        Volume: c_long,
    ) -> HRESULT,
    fn get_BufferNotifySize(
        BufferNotifySize: *mut c_long,
    ) -> HRESULT,
    fn put_BufferNotifySize(
        BufferNotifySize: c_long,
    ) -> HRESULT,
    fn get_EventHandle(
        EventHandle: *mut c_long,
    ) -> HRESULT,
    fn SetState(
        State: SpeechAudioState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3c76af6d, 0x1fd7, 0x4831, 0x81, 0xd1, 0x3b, 0x71, 0xd5, 0xa1, 0x3c, 0x44)]
interface ISpeechMMSysAudio(ISpeechMMSysAudioVtbl): ISpeechAudio(ISpeechAudioVtbl) {
    fn get_DeviceId(
        DeviceId: *mut c_long,
    ) -> HRESULT,
    fn put_DeviceId(
        DeviceId: c_long,
    ) -> HRESULT,
    fn get_LineId(
        LineId: *mut c_long,
    ) -> HRESULT,
    fn put_LineId(
        LineId: c_long,
    ) -> HRESULT,
    fn get_MMHandle(
        Handle: *mut c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x269316d8, 0x57bd, 0x11d2, 0x9e, 0xee, 0x00, 0xc0, 0x4f, 0x79, 0x73, 0x96)]
interface ISpeechVoice(ISpeechVoiceVtbl): IDispatch(IDispatchVtbl) {
    fn get_Status(
        Status: *mut *mut ISpeechVoiceStatus,
    ) -> HRESULT,
    fn get_Voice(
        Voice: *mut *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn putref_Voice(
        Voice: *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn get_AudioOutput(
        AudioOutput: *mut *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn putref_AudioOutput(
        AudioOutput: *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn get_AudioOutputStream(
        AudioOutputStream: *mut *mut ISpeechBaseStream,
    ) -> HRESULT,
    fn putref_AudioOutputStream(
        AudioOutputStream: *mut ISpeechBaseStream,
    ) -> HRESULT,
    fn get_Rate(
        Rate: *mut c_long,
    ) -> HRESULT,
    fn put_Rate(
        Rate: c_long,
    ) -> HRESULT,
    fn get_Volume(
        Volume: *mut c_long,
    ) -> HRESULT,
    fn put_Volume(
        Volume: c_long,
    ) -> HRESULT,
    fn put_AllowAudioOutputFormatChangesOnNextSet(
        Allow: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllowAudioOutputFormatChangesOnNextSet(
        Allow: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EventInterests(
        EventInterestFlags: *mut SpeechVoiceEvents,
    ) -> HRESULT,
    fn put_EventInterests(
        EventInterestFlags: SpeechVoiceEvents,
    ) -> HRESULT,
    fn put_Priority(
        Priority: SpeechVoicePriority,
    ) -> HRESULT,
    fn get_Priority(
        Priority: *mut SpeechVoicePriority,
    ) -> HRESULT,
    fn put_AlertBoundary(
        Boundary: SpeechVoiceEvents,
    ) -> HRESULT,
    fn get_AlertBoundary(
        Boundary: *mut SpeechVoiceEvents,
    ) -> HRESULT,
    fn put_SynchronousSpeakTimeout(
        msTimeout: c_long,
    ) -> HRESULT,
    fn get_SynchronousSpeakTimeout(
        msTimeOut: *mut c_long,
    ) -> HRESULT,
    fn Speak(
        Text: BSTR,
        Flags: SpeechVoiceSpeakFlags,
        StreamNumber: *mut c_long,
    ) -> HRESULT,
    fn SpeakStream(
        Stream: *mut ISpeechBaseStream,
        Flags: SpeechVoiceSpeakFlags,
        StreamNumber: *mut c_long,
    ) -> HRESULT,
    fn Pause() -> HRESULT,
    fn Resume() -> HRESULT,
    fn Skip(
        Type: BSTR,
        NumItems: c_long,
        NumSkipped: c_long,
    ) -> HRESULT,
    fn GetVoices(
        RequiredAttributes: BSTR,
        OptionalAttributes: BSTR,
        ObjectTokens: *mut *mut ISpeechObjectTokens,
    ) -> HRESULT,
    fn GetAudioOutputs(
        RequiredAttributes: BSTR,
        OptionalAttributes: BSTR,
        ObjectTokens: *mut *mut ISpeechObjectTokens,
    ) -> HRESULT,
    fn WaitUntilDone(
        msTimeout: c_long,
        Done: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SpeakCompleteEvent(
        Handle: *mut c_long,
    ) -> HRESULT,
    fn IsUISupported(
        TypeOfUI: BSTR,
        ExtraData: *const VARIANT,
        Supported: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DisplayUI(
        hWndParent: c_long,
        Title: BSTR,
        TypeOfUI: BSTR,
        ExtraData: *const VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8be47b07, 0x57f6, 0x11d2, 0x9e, 0xee, 0x00, 0xc0, 0x4f, 0x79, 0x73, 0x96)]
interface ISpeechVoiceStatus(ISpeechVoiceStatusVtbl): IDispatch(IDispatchVtbl) {
    fn get_CurrentStreamNumber(
        StreamNumber: *mut c_long,
    ) -> HRESULT,
    fn get_LastStreamNumberQueued(
        StreamNumber: *mut c_long,
    ) -> HRESULT,
    fn get_LastHResult(
        HResult: *mut c_long,
    ) -> HRESULT,
    fn get_RunningState(
        State: *mut SpeechRunState,
    ) -> HRESULT,
    fn get_InputWordPosition(
        Position: *mut c_long,
    ) -> HRESULT,
    fn get_InputWordLength(
        Length: *mut c_long,
    ) -> HRESULT,
    fn get_InputSentencePosition(
        Position: *mut c_long,
    ) -> HRESULT,
    fn get_InputSentenceLength(
        Length: *mut c_long,
    ) -> HRESULT,
    fn get_LastBookmark(
        Bookmark: *mut BSTR,
    ) -> HRESULT,
    fn get_LastBookmarkId(
        BookmarkId: *mut c_long,
    ) -> HRESULT,
    fn get_PhonemeId(
        PhoneId: *mut c_short,
    ) -> HRESULT,
    fn get_VisemeId(
        VisemeId: *mut c_short,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa372acd1, 0x3bef, 0x4bbd, 0x8f, 0xfb, 0xcb, 0x3e, 0x2b, 0x41, 0x6a, 0xf8)]
interface _ISpeechVoiceEvents(_ISpeechVoiceEventsVtbl): IDispatch(IDispatchVtbl) {}}
RIDL!{#[uuid(0x2d5f1c0c, 0xbd75, 0x4b08, 0x94, 0x78, 0x3b, 0x11, 0xfe, 0xa2, 0x58, 0x6c)]
interface ISpeechRecognizer(ISpeechRecognizerVtbl): IDispatch(IDispatchVtbl) {
    fn putref_Recognizer(
        Recognizer: *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn get_Recognizer(
        Recognizer: *mut *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn put_AllowAudioInputFormatChangesOnNextSet(
        Allow: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllowAudioInputFormatChangesOnNextSet(
        Allow: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn putref_AudioInput(
        AudioInput: *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn get_AudioInput(
        AudioInput: *mut *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn putref_AudioInputStream(
        AudioInputStream: *mut ISpeechBaseStream,
    ) -> HRESULT,
    fn get_AudioInputStream(
        AudioInputStream: *mut *mut ISpeechBaseStream,
    ) -> HRESULT,
    fn get_IsShared(
        Shared: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_State(
        State: SpeechRecognizerState,
    ) -> HRESULT,
    fn get_State(
        State: *mut SpeechRecognizerState,
    ) -> HRESULT,
    fn get_Status(
        Status: *mut *mut ISpeechRecognizerStatus,
    ) -> HRESULT,
    fn putref_Profile(
        Profile: *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn get_Profile(
        Profile: *mut *mut ISpeechObjectToken,
    ) -> HRESULT,
    fn EmulateRecognition(
        TextElements: VARIANT,
        ElementDisplayAttributes: *mut VARIANT,
        LanguageId: c_long,
    ) -> HRESULT,
    fn CreateRecoContext(
        NewContext: *mut *mut ISpeechRecoContext,
    ) -> HRESULT,
    fn GetFormat(
        Type: SpeechFormatType,
        Format: *mut *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn SetPropertyNumber(
        Name: BSTR,
        Value: c_long,
        Supported: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPropertyNumber(
        Name: BSTR,
        Value: *mut c_long,
        Supported: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SetPropertyString(
        Name: BSTR,
        Value: BSTR,
        Supported: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPropertyString(
        Name: BSTR,
        Value: *mut BSTR,
        Supported: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IsUISupported(
        TypeOfUI: BSTR,
        ExtraData: *const VARIANT,
        Supported: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DisplayUI(
        hWndParent: c_long,
        Title: BSTR,
        TypeOfUI: BSTR,
        ExtraData: *const VARIANT,
    ) -> HRESULT,
    fn GetRecognizers(
        RequiredAttributes: BSTR,
        OptionalAttributes: BSTR,
        ObjectTokens: *mut *mut ISpeechObjectTokens,
    ) -> HRESULT,
    fn GetAudioInputs(
        RequiredAttributes: BSTR,
        OptionalAttributes: BSTR,
        ObjectTokens: *mut *mut ISpeechObjectTokens,
    ) -> HRESULT,
    fn GetProfiles(
        RequiredAttributes: BSTR,
        OptionalAttributes: BSTR,
        ObjectTokens: *mut *mut ISpeechObjectTokens,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbff9e781, 0x53ec, 0x484e, 0xbb, 0x8a, 0x0e, 0x1b, 0x55, 0x51, 0xe3, 0x5c)]
interface ISpeechRecognizerStatus(ISpeechRecognizerStatusVtbl): IDispatch(IDispatchVtbl) {
    fn get_AudioStatus(
        AudioStatus: *mut *mut ISpeechAudioStatus,
    ) -> HRESULT,
    fn get_CurrentStreamPosition(
        pCurrentStreamPos: *mut VARIANT,
    ) -> HRESULT,
    fn get_CurrentStreamNumber(
        StreamNumber: *mut c_long,
    ) -> HRESULT,
    fn get_NumberOfActiveRules(
        NumberOfActiveRules: *mut c_long,
    ) -> HRESULT,
    fn get_ClsidEngine(
        ClsidEngine: *mut BSTR,
    ) -> HRESULT,
    fn get_SupportedLanguages(
        SupportedLanguages: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x580aa49d, 0x7e1e, 0x4809, 0xb8, 0xe2, 0x57, 0xda, 0x80, 0x61, 0x04, 0xb8)]
interface ISpeechRecoContext(ISpeechRecoContextVtbl): IDispatch(IDispatchVtbl) {
    fn get_Recognizer(
        Recognizer: *mut *mut ISpeechRecognizer,
    ) -> HRESULT,
    fn get_AudioInputInterferenceStatus(
        Interference: *mut SpeechInterference,
    ) -> HRESULT,
    fn get_RequestedUIType(
        UIType: *mut BSTR,
    ) -> HRESULT,
    fn putref_Voice(
        Voice: *mut ISpeechVoice,
    ) -> HRESULT,
    fn get_Voice(
        Voice: *mut *mut ISpeechVoice,
    ) -> HRESULT,
    fn put_AllowVoiceFormatMatchingOnNextSet(
        Allow: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllowVoiceFormatMatchingOnNextSet(
        Allow: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_VoicePurgeEvent(
        EventInterest: SpeechRecoEvents,
    ) -> HRESULT,
    fn get_VoicePurgeEvent(
        EventInterest: *mut SpeechRecoEvents,
    ) -> HRESULT,
    fn put_EventInterests(
        EventInterest: SpeechRecoEvents,
    ) -> HRESULT,
    fn get_EventInterests(
        EventInterest: *mut SpeechRecoEvents,
    ) -> HRESULT,
    fn put_CmdMaxAlternates(
        MaxAlternates: c_long,
    ) -> HRESULT,
    fn get_CmdMaxAlternates(
        MaxAlternates: *mut c_long,
    ) -> HRESULT,
    fn put_State(
        State: SpeechRecoContextState,
    ) -> HRESULT,
    fn get_State(
        State: *mut SpeechRecoContextState,
    ) -> HRESULT,
    fn put_RetainedAudio(
        Option: SpeechRetainedAudioOptions,
    ) -> HRESULT,
    fn get_RetainedAudio(
        Option: *mut SpeechRetainedAudioOptions,
    ) -> HRESULT,
    fn putref_RetainedAudioFormat(
        Format: *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn get_RetainedAudioFormat(
        Format: *mut *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn Pause() -> HRESULT,
    fn Resume() -> HRESULT,
    fn CreateGrammar(
        GrammarId: VARIANT,
        Grammar: *mut *mut ISpeechRecoGrammar,
    ) -> HRESULT,
    fn CreateResultFromMemory(
        ResultBlock: *mut VARIANT,
        Result: *mut *mut ISpeechRecoResult,
    ) -> HRESULT,
    fn Bookmark(
        Options: SpeechBookmarkOptions,
        StreamPos: VARIANT,
        BookmarkId: VARIANT,
    ) -> HRESULT,
    fn SetAdaptationData(
        AdaptationString: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb6d6f79f, 0x2158, 0x4e50, 0xb5, 0xbc, 0x9a, 0x9c, 0xcd, 0x85, 0x2a, 0x09)]
interface ISpeechRecoGrammar(ISpeechRecoGrammarVtbl): IDispatch(IDispatchVtbl) {
    fn get_Id(
        Id: *mut VARIANT,
    ) -> HRESULT,
    fn get_RecoContext(
        RecoContext: *mut *mut ISpeechRecoContext,
    ) -> HRESULT,
    fn put_State(
        State: SpeechGrammarState,
    ) -> HRESULT,
    fn get_State(
        State: *mut SpeechGrammarState,
    ) -> HRESULT,
    fn get_Rules(
        Rules: *mut *mut ISpeechGrammarRules,
    ) -> HRESULT,
    fn Reset(
        NewLanguage: SpeechLanguageId,
    ) -> HRESULT,
    fn CmdLoadFromFile(
        FileName: BSTR,
        LoadOption: SpeechLoadOption,
    ) -> HRESULT,
    fn CmdLoadFromObject(
        ClassId: BSTR,
        GrammarName: BSTR,
        LoadOption: SpeechLoadOption,
    ) -> HRESULT,
    fn CmdLoadFromResource(
        hModule: c_long,
        ResourceName: VARIANT,
        ResourceType: VARIANT,
        LanguageId: SpeechLanguageId,
        LoadOption: SpeechLoadOption,
    ) -> HRESULT,
    fn CmdLoadFromMemory(
        GrammarData: VARIANT,
        LoadOption: SpeechLoadOption,
    ) -> HRESULT,
    fn CmdLoadFromProprietaryGrammar(
        ProprietaryGuid: BSTR,
        PriorietaryString: BSTR,
        ProprietaryData: VARIANT,
        LoadOption: SpeechLoadOption,
    ) -> HRESULT,
    fn CmdSetRuleState(
        Name: BSTR,
        State: SpeechRuleState,
    ) -> HRESULT,
    fn CmdSetRuleIdState(
        RuleId: c_long,
        State: SpeechRuleState,
    ) -> HRESULT,
    fn DictationLoad(
        TopicName: BSTR,
        LoadOption: SpeechLoadOption,
    ) -> HRESULT,
    fn DictationUnload() -> HRESULT,
    fn DictationSetState(
        State: SpeechRuleState,
    ) -> HRESULT,
    fn SetWordSequenceData(
        Text: BSTR,
        TextLength: c_long,
        Info: *mut ISpeechTextSelectionInformation,
    ) -> HRESULT,
    fn SetTextSelection(
        Info: *mut ISpeechTextSelectionInformation,
    ) -> HRESULT,
    fn IsPronounceable(
        Word: BSTR,
        WordPronounceable: *mut SpeechWordPronounceable,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7b8fcb42, 0x0e9d, 0x4f00, 0xa0, 0x48, 0x7b, 0x04, 0xd6, 0x17, 0x9d, 0x3d)]
interface _ISpeechRecoContextEvents(_ISpeechRecoContextEventsVtbl): IDispatch(IDispatchVtbl) {}}
RIDL!{#[uuid(0xafe719cf, 0x5dd1, 0x44f2, 0x99, 0x9c, 0x7a, 0x39, 0x9f, 0x1c, 0xfc, 0xcc)]
interface ISpeechGrammarRule(ISpeechGrammarRuleVtbl): IDispatch(IDispatchVtbl) {
    fn get_Attributes(
        Attributes: *mut SpeechRuleAttributes,
    ) -> HRESULT,
    fn get_InitialState(
        State: *mut *mut ISpeechGrammarRuleState,
    ) -> HRESULT,
    fn get_Name(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn get_Id(
        Id: *mut c_long,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn AddResource(
        ResourceName: BSTR,
        ResourceValue: BSTR,
    ) -> HRESULT,
    fn AddState(
        State: *mut *mut ISpeechGrammarRuleState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6ffa3b44, 0xfc2d, 0x40d1, 0x8a, 0xfc, 0x32, 0x91, 0x1c, 0x7f, 0x1a, 0xd1)]
interface ISpeechGrammarRules(ISpeechGrammarRulesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn FindRule(
        RuleNameOrId: VARIANT,
        Rule: *mut *mut ISpeechGrammarRule,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Rule: *mut *mut ISpeechGrammarRule,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
    fn get_Dynamic(
        Dynamic: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Add(
        RuleName: BSTR,
        Attributes: SpeechRuleAttributes,
        RuleId: c_long,
        Rule: *mut *mut ISpeechGrammarRule,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
    fn CommitAndSave(
        ErrorText: *mut BSTR,
        SaveStream: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd4286f2c, 0xee67, 0x45ae, 0xb9, 0x28, 0x28, 0xd6, 0x95, 0x36, 0x2e, 0xda)]
interface ISpeechGrammarRuleState(ISpeechGrammarRuleStateVtbl): IDispatch(IDispatchVtbl) {
    fn get_Rule(
        Rule: *mut *mut ISpeechGrammarRule,
    ) -> HRESULT,
    fn get_Transitions(
        Transitions: *mut *mut ISpeechGrammarRuleStateTransitions,
    ) -> HRESULT,
    fn AddWordTransition(
        DestState: *mut ISpeechGrammarRuleState,
        Words: BSTR,
        Separators: BSTR,
        Type: SpeechGrammarWordType,
        PropertyName: BSTR,
        PropertyId: c_long,
        PropertyValue: *mut VARIANT,
        Weight: c_float,
    ) -> HRESULT,
    fn AddRuleTransition(
        DestinationState: *mut ISpeechGrammarRuleState,
        Rule: *mut ISpeechGrammarRule,
        PropertyName: BSTR,
        PropertyId: c_long,
        PropertyValue: *mut VARIANT,
        Weight: c_float,
    ) -> HRESULT,
    fn AddSpecialTransition(
        DestinationState: *mut ISpeechGrammarRuleState,
        Type: SpeechSpecialTransitionType,
        PropertyName: BSTR,
        PropertyId: c_long,
        PropertyValue: *mut VARIANT,
        Weight: c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcafd1db1, 0x41d1, 0x4a06, 0x98, 0x63, 0xe2, 0xe8, 0x1d, 0xa1, 0x7a, 0x9a)]
interface ISpeechGrammarRuleStateTransition(ISpeechGrammarRuleStateTransitionVtbl):
    IDispatch(IDispatchVtbl) {
    fn get_Type(
        Type: *mut SpeechGrammarRuleStateTransitionType,
    ) -> HRESULT,
    fn get_Text(
        Text: *mut BSTR,
    ) -> HRESULT,
    fn get_Rule(
        Rule: *mut *mut ISpeechGrammarRule,
    ) -> HRESULT,
    fn get_Weight(
        Weight: *mut VARIANT,
    ) -> HRESULT,
    fn get_PropertyName(
        PropertyName: *mut BSTR,
    ) -> HRESULT,
    fn get_PropertyId(
        PropertyId: *mut c_long,
    ) -> HRESULT,
    fn get_PropertyValue(
        PropertyValue: *mut VARIANT,
    ) -> HRESULT,
    fn get_NextState(
        NextState: *mut *mut ISpeechGrammarRuleState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xeabce657, 0x75bc, 0x44a2, 0xaa, 0x7f, 0xc5, 0x64, 0x76, 0x74, 0x29, 0x63)]
interface ISpeechGrammarRuleStateTransitions(ISpeechGrammarRuleStateTransitionsVtbl):
    IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Transition: *mut *mut ISpeechGrammarRuleStateTransition,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3b9c7e7a, 0x6eee, 0x4ded, 0x90, 0x92, 0x11, 0x65, 0x72, 0x79, 0xad, 0xbe)]
interface ISpeechTextSelectionInformation(ISpeechTextSelectionInformationVtbl):
    IDispatch(IDispatchVtbl) {
    fn put_ActiveOffset(
        ActiveOffset: c_long,
    ) -> HRESULT,
    fn get_ActiveOffset(
        ActiveOffset: *mut c_long,
    ) -> HRESULT,
    fn put_ActiveLength(
        ActiveLength: c_long,
    ) -> HRESULT,
    fn get_ActiveLength(
        ActiveLength: *mut c_long,
    ) -> HRESULT,
    fn put_SelectionOffset(
        SelectionOffset: c_long,
    ) -> HRESULT,
    fn get_SelectionOffset(
        SelectionOffset: *mut c_long,
    ) -> HRESULT,
    fn put_SelectionLength(
        SelectionLength: c_long,
    ) -> HRESULT,
    fn get_SelectionLength(
        SelectionLength: *mut c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xed2879cf, 0xced9, 0x4ee6, 0xa5, 0x34, 0xde, 0x01, 0x91, 0xd5, 0x46, 0x8d)]
interface ISpeechRecoResult(ISpeechRecoResultVtbl): IDispatch(IDispatchVtbl) {
    fn get_RecoContext(
        RecoContext: *mut *mut ISpeechRecoContext,
    ) -> HRESULT,
    fn get_Times(
        Times: *mut *mut ISpeechRecoResultTimes,
    ) -> HRESULT,
    fn putref_AudioFormat(
        Format: *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn get_AudioFormat(
        Format: *mut *mut ISpeechAudioFormat,
    ) -> HRESULT,
    fn get_PhraseInfo(
        PhraseInfo: *mut *mut ISpeechPhraseInfo,
    ) -> HRESULT,
    fn Alternates(
        RequestCount: c_long,
        StartElement: c_long,
        Elements: c_long,
        Alternates: *mut *mut ISpeechPhraseAlternates,
    ) -> HRESULT,
    fn Audio(
        StartElement: c_long,
        Elements: c_long,
        Stream: *mut *mut ISpeechMemoryStream,
    ) -> HRESULT,
    fn SpeakAudio(
        StartElement: c_long,
        Elements: c_long,
        Flags: SpeechVoiceSpeakFlags,
        StreamNumber: *mut c_long,
    ) -> HRESULT,
    fn SaveToMemory(
        ResultBlock: *mut VARIANT,
    ) -> HRESULT,
    fn DiscardResultInfo(
        ValueTypes: SpeechDiscardType,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x62b3b8fb, 0xf6e7, 0x41be, 0xbd, 0xcb, 0x05, 0x6b, 0x1c, 0x29, 0xef, 0xc0)]
interface ISpeechRecoResultTimes(ISpeechRecoResultTimesVtbl): IDispatch(IDispatchVtbl) {
    fn get_StreamTime(
        Time: *mut VARIANT,
    ) -> HRESULT,
    fn get_Length(
        Length: *mut VARIANT,
    ) -> HRESULT,
    fn get_TickCount(
        TickCount: *mut c_long,
    ) -> HRESULT,
    fn get_OffsetFromStart(
        OffsetFromStart: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x27864a2a, 0x2b9f, 0x4cb8, 0x92, 0xd3, 0x0d, 0x27, 0x22, 0xfd, 0x1e, 0x73)]
interface ISpeechPhraseAlternate(ISpeechPhraseAlternateVtbl): IDispatch(IDispatchVtbl) {
    fn get_RecoResult(
        RecoResult: *mut *mut ISpeechRecoResult,
    ) -> HRESULT,
    fn get_StartElementInResult(
        StartElement: *mut c_long,
    ) -> HRESULT,
    fn get_NumberOfElementsInResult(
        NumberOfElements: *mut c_long,
    ) -> HRESULT,
    fn get_PhraseInfo(
        PhraseInfo: *mut *mut ISpeechPhraseInfo,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
}}
RIDL!{#[uuid(0xb238b6d5, 0xf276, 0x4c3d, 0xa6, 0xc1, 0x29, 0x74, 0x80, 0x1c, 0x3c, 0xc2)]
interface ISpeechPhraseAlternates(ISpeechPhraseAlternatesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        PhraseAlternate: *mut *mut ISpeechPhraseAlternate,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x961559cf, 0x4e67, 0x4662, 0x8b, 0xf0, 0xd9, 0x3f, 0x1f, 0xcd, 0x61, 0xb3)]
interface ISpeechPhraseInfo(ISpeechPhraseInfoVtbl): IDispatch(IDispatchVtbl) {
    fn get_LanguageId(
        LanguageId: *mut c_long,
    ) -> HRESULT,
    fn get_GrammarId(
        GrammarId: *mut VARIANT,
    ) -> HRESULT,
    fn get_StartTime(
        StartTime: *mut VARIANT,
    ) -> HRESULT,
    fn get_AudioStreamPosition(
        AudioStreamPosition: *mut VARIANT,
    ) -> HRESULT,
    fn get_AudioSizeBytes(
        pAudioSizeBytes: *mut c_long,
    ) -> HRESULT,
    fn get_RetainedSizeBytes(
        RetainedSizeBytes: *mut c_long,
    ) -> HRESULT,
    fn get_AudioSizeTime(
        AudioSizeTime: *mut c_long,
    ) -> HRESULT,
    fn get_Rule(
        Rule: *mut *mut ISpeechPhraseRule,
    ) -> HRESULT,
    fn get_Properties(
        Properties: *mut *mut ISpeechPhraseProperties,
    ) -> HRESULT,
    fn get_Elements(
        Elements: *mut *mut ISpeechPhraseElements,
    ) -> HRESULT,
    fn get_Replacements(
        Replacements: *mut *mut ISpeechPhraseReplacements,
    ) -> HRESULT,
    fn get_EngineId(
        EngineIdGuid: *mut BSTR,
    ) -> HRESULT,
    fn get_EnginePrivateData(
        PrivateData: *mut VARIANT,
    ) -> HRESULT,
    fn SaveToMemory(
        PhraseBlock: *mut VARIANT,
    ) -> HRESULT,
    fn GetText(
        StartElement: c_long,
        Elements: c_long,
        UseReplacements: VARIANT_BOOL,
        Text: *mut BSTR,
    ) -> HRESULT,
    fn GetDisplayAttributes(
        StartElement: c_long,
        Elements: c_long,
        UseReplacements: VARIANT_BOOL,
        DisplayAttributes: *mut SpeechDisplayAttributes,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe6176f96, 0xe373, 0x4801, 0xb2, 0x23, 0x3b, 0x62, 0xc0, 0x68, 0xc0, 0xb4)]
interface ISpeechPhraseElement(ISpeechPhraseElementVtbl): IDispatch(IDispatchVtbl) {
    fn get_AudioTimeOffset(
        AudioTimeOffset: *mut c_long,
    ) -> HRESULT,
    fn get_AudioSizeTime(
        AudioSizeTime: *mut c_long,
    ) -> HRESULT,
    fn get_AudioStreamOffset(
        AudioStreamOffset: *mut c_long,
    ) -> HRESULT,
    fn get_AudioSizeBytes(
        AudioSizeBytes: *mut c_long,
    ) -> HRESULT,
    fn get_RetainedStreamOffset(
        RetainedStreamOffset: *mut c_long,
    ) -> HRESULT,
    fn get_RetainedSizeBytes(
        RetainedSizeBytes: *mut c_long,
    ) -> HRESULT,
    fn get_DisplayText(
        DisplayText: *mut BSTR,
    ) -> HRESULT,
    fn get_LexicalForm(
        LexicalForm: *mut BSTR,
    ) -> HRESULT,
    fn get_Pronunciation(
        Pronunciation: *mut VARIANT,
    ) -> HRESULT,
    fn get_DisplayAttributes(
        DisplayAttributes: *mut SpeechDisplayAttributes,
    ) -> HRESULT,
    fn get_RequiredConfidence(
        RequiredConfidence: *mut SpeechEngineConfidence,
    ) -> HRESULT,
    fn get_ActualConfidence(
        ActualConfidence: *mut SpeechEngineConfidence,
    ) -> HRESULT,
    fn get_EngineConfidence(
        EngineConfident: *mut c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0626b328, 0x3478, 0x467d, 0xa0, 0xb3, 0xd0, 0x85, 0x3b, 0x93, 0xdd, 0xa3)]
interface ISpeechPhraseElements(ISpeechPhraseElementsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Element: *mut *mut ISpeechPhraseElement,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2890a410, 0x53a7, 0x4fb5, 0x94, 0xec, 0x06, 0xd4, 0x99, 0x8e, 0x3d, 0x02)]
interface ISpeechPhraseReplacement(ISpeechPhraseReplacementVtbl): IDispatch(IDispatchVtbl) {
    fn get_DisplayAttributes(
        DisplayAttributes: *mut SpeechDisplayAttributes,
    ) -> HRESULT,
    fn get_Text(
        Text: *mut BSTR,
    ) -> HRESULT,
    fn get_FirstElement(
        FirstElement: *mut c_long,
    ) -> HRESULT,
    fn get_NumberOfElements(
        NumberOfElements: *mut c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x38bc662f, 0x2257, 0x4525, 0x95, 0x9e, 0x20, 0x69, 0xd2, 0x59, 0x6c, 0x05)]
interface ISpeechPhraseReplacements(ISpeechPhraseReplacementsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Reps: *mut *mut ISpeechPhraseReplacement,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xce563d48, 0x961e, 0x4732, 0xa2, 0xe1, 0x37, 0x8a, 0x42, 0xb4, 0x30, 0xbe)]
interface ISpeechPhraseProperty(ISpeechPhrasePropertyVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn get_Id(
        Id: *mut c_long,
    ) -> HRESULT,
    fn get_Value(
        Value: *mut VARIANT,
    ) -> HRESULT,
    fn get_FirstElement(
        FirstElement: *mut c_long,
    ) -> HRESULT,
    fn get_NumberOfElements(
        NumberOfElements: *mut c_long,
    ) -> HRESULT,
    fn get_EngineConfidence(
        Confidence: *mut c_float,
    ) -> HRESULT,
    fn get_Confidence(
        Confidence: *mut SpeechEngineConfidence,
    ) -> HRESULT,
    fn get_Parent(
        ParentProperty: *mut *mut ISpeechPhraseProperty,
    ) -> HRESULT,
    fn get_Children(
        Children: *mut *mut ISpeechPhraseProperties,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x08166b47, 0x102e, 0x4b23, 0xa5, 0x99, 0xbd, 0xb9, 0x8d, 0xbf, 0xd1, 0xf4)]
interface ISpeechPhraseProperties(ISpeechPhrasePropertiesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Property: *mut *mut ISpeechPhraseProperty,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa7bfe112, 0xa4a0, 0x48d9, 0xb6, 0x02, 0xc3, 0x13, 0x84, 0x3f, 0x69, 0x64)]
interface ISpeechPhraseRule(ISpeechPhraseRuleVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn get_Id(
        Id: *mut c_long,
    ) -> HRESULT,
    fn get_FirstElement(
        FirstElement: *mut c_long,
    ) -> HRESULT,
    fn get_NumberOfElements(
        NumberOfElements: *mut c_long,
    ) -> HRESULT,
    fn get_Parent(
        Parent: *mut *mut ISpeechPhraseRule,
    ) -> HRESULT,
    fn get_Children(
        Children: *mut *mut ISpeechPhraseRules,
    ) -> HRESULT,
    fn get_Confidence(
        ActualConfidence: *mut SpeechEngineConfidence,
    ) -> HRESULT,
    fn get_EngineConfidence(
        Confidence: *mut c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9047d593, 0x01dd, 0x4b72, 0x81, 0xa3, 0xe4, 0xa0, 0xca, 0x69, 0xf4, 0x07)]
interface ISpeechPhraseRules(ISpeechPhraseRulesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Rule: *mut *mut ISpeechPhraseRule,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3da7627a, 0xc7ae, 0x4b23, 0x87, 0x08, 0x63, 0x8c, 0x50, 0x36, 0x2c, 0x25)]
interface ISpeechLexicon(ISpeechLexiconVtbl): IDispatch(IDispatchVtbl) {
    fn get_GenerationId(
        GenerationId: *mut c_long,
    ) -> HRESULT,
    fn GetWords(
        Flags: SpeechLexiconType,
        GenerationID: *mut c_long,
        Words: *mut *mut ISpeechLexiconWords,
    ) -> HRESULT,
    fn AddPronunciation(
        bstrWord: BSTR,
        LangId: SpeechLanguageId,
        PartOfSpeech: SpeechPartOfSpeech,
        bstrPronunciation: BSTR,
    ) -> HRESULT,
    fn AddPronunciationByPhoneIds(
        bstrWord: BSTR,
        LangId: SpeechLanguageId,
        PartOfSpeech: SpeechPartOfSpeech,
        PhoneIds: *mut VARIANT,
    ) -> HRESULT,
    fn RemovePronunciation(
        bstrWord: BSTR,
        LangId: SpeechLanguageId,
        PartOfSpeech: SpeechPartOfSpeech,
        bstrPronunciation: BSTR,
    ) -> HRESULT,
    fn RemovePronunciationByPhoneIds(
        bstrWord: BSTR,
        LangId: SpeechLanguageId,
        PartOfSpeech: SpeechPartOfSpeech,
        PhoneIds: *mut VARIANT,
    ) -> HRESULT,
    fn GetPronunciations(
        bstrWord: BSTR,
        LangId: SpeechLanguageId,
        TypeFlags: SpeechLexiconType,
        ppPronunciations: *mut *mut ISpeechLexiconPronunciations,
    ) -> HRESULT,
    fn GetGenerationChange(
        GenerationID: *mut c_long,
        ppWords: *mut *mut ISpeechLexiconWords,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8d199862, 0x415e, 0x47d5, 0xac, 0x4f, 0xfa, 0xa6, 0x08, 0xb4, 0x24, 0xe6)]
interface ISpeechLexiconWords(ISpeechLexiconWordsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Word: *mut *mut ISpeechLexiconWord,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4e5b933c, 0xc9be, 0x48ed, 0x88, 0x42, 0x1e, 0xe5, 0x1b, 0xb1, 0xd4, 0xff)]
interface ISpeechLexiconWord(ISpeechLexiconWordVtbl): IDispatch(IDispatchVtbl) {
    fn get_LangId(
        LangId: *mut SpeechLanguageId,
    ) -> HRESULT,
    fn get_Type(
        WordType: *mut SpeechWordType,
    ) -> HRESULT,
    fn get_Word(
        Word: *mut BSTR,
    ) -> HRESULT,
    fn get_Pronunciations(
        Pronunciations: *mut *mut ISpeechLexiconPronunciations,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x72829128, 0x5682, 0x4704, 0xa0, 0xd4, 0x3e, 0x2b, 0xb6, 0xf2, 0xea, 0xd3)]
interface ISpeechLexiconPronunciations(ISpeechLexiconPronunciationsVtbl):
    IDispatch(IDispatchVtbl) {
    fn get_Count(
        Count: *mut c_long,
    ) -> HRESULT,
    fn Item(
        Index: c_long,
        Pronunciation: *mut *mut ISpeechLexiconPronunciation,
    ) -> HRESULT,
    fn get__NewEnum(
        EnumVARIANT: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x95252c5d, 0x9e43, 0x4f4a, 0x98, 0x99, 0x48, 0xee, 0x73, 0x35, 0x2f, 0x9f)]
interface ISpeechLexiconPronunciation(ISpeechLexiconPronunciationVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        LexiconType: *mut SpeechLexiconType,
    ) -> HRESULT,
    fn get_LangId(
        LangId: *mut SpeechLanguageId,
    ) -> HRESULT,
    fn get_PartOfSpeech(
        PartOfSpeech: *mut SpeechPartOfSpeech,
    ) -> HRESULT,
    fn get_PhoneIds(
        PhoneIds: *mut VARIANT,
    ) -> HRESULT,
    fn get_Symbolic(
        Symbolic: *mut BSTR,
    ) -> HRESULT,
}}
pub const Speech_Default_Weight: c_float = DEFAULT_WEIGHT;
pub const Speech_Max_Word_Length: LONG = SP_MAX_WORD_LENGTH as i32;
pub const Speech_Max_Pron_Length: LONG = SP_MAX_PRON_LENGTH as i32;
pub const Speech_StreamPos_Asap: LONG = SP_STREAMPOS_ASAP as i32;
pub const Speech_StreamPos_RealTime: LONG = SP_STREAMPOS_REALTIME as i32;
pub const SpeechAllElements: LONG = SPPR_ALL_ELEMENTS as i32;
RIDL!{#[uuid(0x3b151836, 0xdf3a, 0x4e0a, 0x84, 0x6c, 0xd2, 0xad, 0xc9, 0x33, 0x43, 0x33)]
interface ISpeechPhraseInfoBuilder(ISpeechPhraseInfoBuilderVtbl): IDispatch(IDispatchVtbl) {
    fn RestorePhraseFromMemory(
        PhraseInMemory: *mut VARIANT,
        PhraseInfo: *mut *mut ISpeechPhraseInfo,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc3e4f353, 0x433f, 0x43d6, 0x89, 0xa1, 0x6a, 0x62, 0xa7, 0x05, 0x4c, 0x3d)]
interface ISpeechPhoneConverter(ISpeechPhoneConverterVtbl): IDispatch(IDispatchVtbl) {
    fn get_LanguageId(
        LanguageId: *mut SpeechLanguageId,
    ) -> HRESULT,
    fn put_LanguageId(
        LanguageId: SpeechLanguageId,
    ) -> HRESULT,
    fn PhoneToId(
        Phonemes: BSTR,
        IdArray: *mut VARIANT,
    ) -> HRESULT,
    fn IdToPhone(
        IdArray: VARIANT,
        Phonemes: *mut BSTR,
    ) -> HRESULT,
}}
extern {
    pub static CLSID_SpNotifyTranslator: CLSID;
    pub static CLSID_SpObjectTokenCategory: CLSID;
    pub static CLSID_SpObjectToken: CLSID;
    pub static CLSID_SpResourceManager: CLSID;
    pub static CLSID_SpStreamFormatConverter: CLSID;
    pub static CLSID_SpMMAudioEnum: CLSID;
    pub static CLSID_SpMMAudioIn: CLSID;
    pub static CLSID_SpMMAudioOut: CLSID;
    pub static CLSID_SpStream: CLSID;
    pub static CLSID_SpVoice: CLSID;
    pub static CLSID_SpSharedRecoContext: CLSID;
    pub static CLSID_SpInprocRecognizer: CLSID;
    pub static CLSID_SpSharedRecognizer: CLSID;
    pub static CLSID_SpLexicon: CLSID;
    pub static CLSID_SpUnCompressedLexicon: CLSID;
    pub static CLSID_SpCompressedLexicon: CLSID;
    pub static CLSID_SpPhoneConverter: CLSID;
    pub static CLSID_SpNullPhoneConverter: CLSID;
    pub static CLSID_SpTextSelectionInformation: CLSID;
    pub static CLSID_SpPhraseInfoBuilder: CLSID;
    pub static CLSID_SpAudioFormat: CLSID;
    pub static CLSID_SpWaveFormatEx: CLSID;
    pub static CLSID_SpInProcRecoContext: CLSID;
    pub static CLSID_SpCustomStream: CLSID;
    pub static CLSID_SpFileStream: CLSID;
    pub static CLSID_SpMemoryStream: CLSID;
}
