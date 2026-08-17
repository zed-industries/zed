// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! this ALWAYS GENERATED file contains the definitions for the interfaces
use ctypes::c_float;
use shared::basetsd::{UINT32, UINT64};
use shared::guiddef::{LPCGUID, REFIID};
use shared::minwindef::{BYTE, DWORD, LPVOID};
use shared::mmreg::WAVEFORMATEX;
use shared::winerror::{FACILITY_AUDCLNT, SEVERITY_ERROR, SEVERITY_SUCCESS};
use shared::wtypesbase::SCODE;
use um::audiosessiontypes::AUDCLNT_SHAREMODE;
use um::strmif::REFERENCE_TIME;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT};
//1627
pub const AUDCLNT_E_NOT_INITIALIZED: HRESULT = AUDCLNT_ERR!(0x001);
pub const AUDCLNT_E_ALREADY_INITIALIZED: HRESULT = AUDCLNT_ERR!(0x002);
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: HRESULT = AUDCLNT_ERR!(0x003);
pub const AUDCLNT_E_DEVICE_INVALIDATED: HRESULT = AUDCLNT_ERR!(0x004);
pub const AUDCLNT_E_NOT_STOPPED: HRESULT = AUDCLNT_ERR!(0x005);
pub const AUDCLNT_E_BUFFER_TOO_LARGE: HRESULT = AUDCLNT_ERR!(0x006);
pub const AUDCLNT_E_OUT_OF_ORDER: HRESULT = AUDCLNT_ERR!(0x007);
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: HRESULT = AUDCLNT_ERR!(0x008);
pub const AUDCLNT_E_INVALID_SIZE: HRESULT = AUDCLNT_ERR!(0x009);
pub const AUDCLNT_E_DEVICE_IN_USE: HRESULT = AUDCLNT_ERR!(0x00a);
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: HRESULT = AUDCLNT_ERR!(0x00b);
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: HRESULT = AUDCLNT_ERR!(0x00c);
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: HRESULT = AUDCLNT_ERR!(0x00e);
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: HRESULT = AUDCLNT_ERR!(0x00f);
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: HRESULT = AUDCLNT_ERR!(0x010);
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: HRESULT = AUDCLNT_ERR!(0x011);
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: HRESULT = AUDCLNT_ERR!(0x012);
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: HRESULT = AUDCLNT_ERR!(0x013);
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: HRESULT = AUDCLNT_ERR!(0x014);
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: HRESULT = AUDCLNT_ERR!(0x015);
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: HRESULT = AUDCLNT_ERR!(0x016);
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: HRESULT = AUDCLNT_ERR!(0x017);
pub const AUDCLNT_E_BUFFER_ERROR: HRESULT = AUDCLNT_ERR!(0x018);
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: HRESULT = AUDCLNT_ERR!(0x019);
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: HRESULT = AUDCLNT_ERR!(0x020);
pub const AUDCLNT_E_INVALID_STREAM_FLAG: HRESULT = AUDCLNT_ERR!(0x021);
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: HRESULT = AUDCLNT_ERR!(0x022);
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: HRESULT = AUDCLNT_ERR!(0x023);
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: HRESULT = AUDCLNT_ERR!(0x024);
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: HRESULT = AUDCLNT_ERR!(0x025);
pub const AUDCLNT_E_RESOURCES_INVALIDATED: HRESULT = AUDCLNT_ERR!(0x026);
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: HRESULT = AUDCLNT_ERR!(0x027);
pub const AUDCLNT_S_BUFFER_EMPTY: SCODE = AUDCLNT_SUCCESS!(0x001);
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: SCODE = AUDCLNT_SUCCESS!(0x002);
pub const AUDCLNT_S_POSITION_STALLED: SCODE = AUDCLNT_SUCCESS!(0x003);
ENUM!{enum AUDCLNT_BUFFERFLAGS {
    AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY = 0x1,
    AUDCLNT_BUFFERFLAGS_SILENT = 0x2,
    AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR = 0x4,
}}
DEFINE_GUID!{IID_IAudioClient,
    0x1CB9AD4C, 0xDBFA, 0x4c32, 0xB1, 0x78, 0xC2, 0xF5, 0x68, 0xA7, 0x03, 0xB2}
DEFINE_GUID!{IID_IAudioRenderClient,
    0xF294ACFC, 0x3146, 0x4483, 0xA7, 0xBF, 0xAD, 0xDC, 0xA7, 0xC2, 0x60, 0xE2}
DEFINE_GUID!{IID_IAudioCaptureClient,
    0xc8adbd64, 0xe71e, 0x48a0, 0xa4, 0xde, 0x18, 0x5c, 0x39, 0x5c, 0xd3, 0x17}
DEFINE_GUID!{IID_IAudioClock,
    0xcd63314f, 0x3fba, 0x4a1b, 0x81, 0x2c, 0xef, 0x96, 0x35, 0x87, 0x28, 0xe7}
DEFINE_GUID!{IID_IAudioStreamVolume,
    0x93014887, 0x242d, 0x4068, 0x8a, 0x15, 0xcf, 0x5e, 0x93, 0xb9, 0x0f, 0xe3}
RIDL!{#[uuid(0x1cb9ad4c, 0xdbfa, 0x4c32, 0xb1, 0x78, 0xc2, 0xf5, 0x68, 0xa7, 0x03, 0xb2)]
interface IAudioClient(IAudioClientVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        ShareMode: AUDCLNT_SHAREMODE,
        StreamFlags: DWORD,
        hnsBufferDuration: REFERENCE_TIME,
        hnsPeriodicity: REFERENCE_TIME,
        pFormat: *const WAVEFORMATEX,
        AudioSessionGuid: LPCGUID,
    ) -> HRESULT,
    fn GetBufferSize(
        pNumBufferFrames: *mut UINT32,
    ) -> HRESULT,
    fn GetStreamLatency(
        phnsLatency: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn GetCurrentPadding(
        pNumPaddingFrames: *mut UINT32,
    ) -> HRESULT,
    fn IsFormatSupported(
        ShareMode: AUDCLNT_SHAREMODE,
        pFormat: *const WAVEFORMATEX,
        ppClosestMatch: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn GetMixFormat(
        ppDeviceFormat: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn GetDevicePeriod(
        phnsDefaultDevicePeriod: *mut REFERENCE_TIME,
        phnsMinimumDevicePeriod: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn Start() -> HRESULT,
    fn Stop() -> HRESULT,
    fn Reset() -> HRESULT,
    fn SetEventHandle(
        eventHandle: HANDLE,
    ) -> HRESULT,
    fn GetService(
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf294acfc, 0x3146, 0x4483, 0xa7, 0xbf, 0xad, 0xdc, 0xa7, 0xc2, 0x60, 0xe2)]
interface IAudioRenderClient(IAudioRenderClientVtbl): IUnknown(IUnknownVtbl) {
    fn GetBuffer(
        NumFramesRequested: UINT32,
        ppData: *mut *mut BYTE,
    ) -> HRESULT,
    fn ReleaseBuffer(
        NumFramesWritten: UINT32,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc8adbd64, 0xe71e, 0x48a0, 0xa4, 0xde, 0x18, 0x5c, 0x39, 0x5c, 0xd3, 0x17)]
interface IAudioCaptureClient(IAudioCaptureClientVtbl): IUnknown(IUnknownVtbl) {
    fn GetBuffer(
        ppData: *mut *mut BYTE,
        pNumFramesToRead: *mut UINT32,
        pdwFlags: *mut DWORD,
        pu64DevicePosition: *mut UINT64,
        pu64QPCPosition: *mut UINT64,
    ) -> HRESULT,
    fn ReleaseBuffer(
        NumFramesRead: UINT32,
    ) -> HRESULT,
    fn GetNextPacketSize(
        pNumFramesInNextPacket: *mut UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcd63314f, 0x3fba, 0x4a1b, 0x81, 0x2c, 0xef, 0x96, 0x35, 0x87, 0x28, 0xe7)]
interface IAudioClock(IAudioClockVtbl): IUnknown(IUnknownVtbl) {
    fn GetFrequency(
        pu64Frequency: *mut UINT64,
    ) -> HRESULT,
    fn GetPosition(
        pu64Position: *mut UINT64,
        pu64QPCPosition: *mut UINT64,
    ) -> HRESULT,
    fn GetCharacteristics(
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x93014887, 0x242d, 0x4068, 0x8a, 0x15, 0xcf, 0x5e, 0x93, 0xb9, 0x0f, 0xe3)]
interface IAudioStreamVolume(IAudioStreamVolumeVtbl): IUnknown(IUnknownVtbl) {
    fn GetChannelCount(
        pdwCount: *mut UINT32,
    ) -> HRESULT,
    fn SetChannelVolume(
        dwIndex: UINT32,
        fLevel: c_float,
    ) -> HRESULT,
    fn GetChannelVolume(
        dwIndex: UINT32,
        pfLevel: *mut c_float,
    ) -> HRESULT,
    fn SetAllVolumes(
        dwCount: UINT32,
        pfVolumes: *const c_float,
    ) -> HRESULT,
    fn GetAllVolumes(
        dwCount: UINT32,
        pfVolumes: *mut c_float,
    ) -> HRESULT,
}}
