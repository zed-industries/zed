// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_float;
use shared::basetsd::UINT32;
use shared::guiddef::{GUID, LPCGUID};
use shared::minwindef::{BOOL, DWORD, UINT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
STRUCT!{struct AUDIO_VOLUME_NOTIFICATION_DATA {
    guidEventContext: GUID,
    bMuted: BOOL,
    fMasterVolume: c_float,
    nChannels: UINT,
    afChannelVolumes: [c_float; 1],
}}
pub type PAUDIO_VOLUME_NOTIFICATION_DATA = *mut AUDIO_VOLUME_NOTIFICATION_DATA;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: DWORD = 0x00000001;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: DWORD = 0x00000002;
pub const ENDPOINT_HARDWARE_SUPPORT_METER: DWORD = 0x00000004;
RIDL!{#[uuid(0x657804fa, 0xd6ad, 0x4496, 0x8a, 0x60, 0x35, 0x27, 0x52, 0xaf, 0x4f, 0x89)]
interface IAudioEndpointVolumeCallback(IAudioEndpointVolumeCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnNotify(
        pNotify: PAUDIO_VOLUME_NOTIFICATION_DATA,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5cdf2c82, 0x841e, 0x4546, 0x97, 0x22, 0x0c, 0xf7, 0x40, 0x78, 0x22, 0x9a)]
interface IAudioEndpointVolume(IAudioEndpointVolumeVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterControlChangeNotify(
        pNotify: *mut IAudioEndpointVolumeCallback,
    ) -> HRESULT,
    fn UnregisterControlChangeNotify(
        pNotify: *mut IAudioEndpointVolumeCallback,
    ) -> HRESULT,
    fn GetChannelCount(
        pnChannelCount: *mut UINT,
    ) -> HRESULT,
    fn SetMasterVolumeLevel(
        fLevelDB: c_float,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn SetMasterVolumeLevelScalar(
        fLevel: c_float,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn GetMasterVolumeLevel(
        pfLevelDB: *mut c_float,
    ) -> HRESULT,
    fn GetMasterVolumeLevelScalar(
        pfLevel: *mut c_float,
    ) -> HRESULT,
    fn SetChannelVolumeLevel(
        nChannel: UINT,
        fLevelDB: c_float,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn SetChannelVolumeLevelScalar(
        nChannel: UINT,
        fLevel: c_float,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn GetChannelVolumeLevel(
        nChannel: UINT,
        pfLevelDB: *mut c_float,
    ) -> HRESULT,
    fn GetChannelVolumeLevelScalar(
        nChannel: UINT,
        pfLevel: *mut c_float,
    ) -> HRESULT,
    fn SetMute(
        bMute: BOOL,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn GetMute(
        pbMute: *mut BOOL,
    ) -> HRESULT,
    fn GetVolumeStepInfo(
        pnStep: *mut UINT,
        pnStepCount: *mut UINT,
    ) -> HRESULT,
    fn VolumeStepUp(
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn VolumeStepDown(
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn QueryHardwareSupport(
        pdwHardwareSupportMask: *mut DWORD,
    ) -> HRESULT,
    fn GetVolumeRange(
        pflVolumeMindB: *mut c_float,
        pflVolumeMaxdB: *mut c_float,
        pflVolumeIncrementdB: *mut c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x66e11784, 0xf695, 0x4f28, 0xa5, 0x05, 0xa7, 0x08, 0x00, 0x81, 0xa7, 0x8f)]
interface IAudioEndpointVolumeEx(IAudioEndpointVolumeExVtbl):
    IAudioEndpointVolume(IAudioEndpointVolumeVtbl) {
    fn GetVolumeRangeChannel(
        iChannel: UINT,
        pflVolumeMindB: *mut c_float,
        pflVolumeMaxdB: *mut c_float,
        pflVolumeIncrementdB: *mut c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc02216f6, 0x8c67, 0x4b5b, 0x9d, 0x00, 0xd0, 0x08, 0xe7, 0x3e, 0x00, 0x64)]
interface IAudioMeterInformation(IAudioMeterInformationVtbl): IUnknown(IUnknownVtbl) {
    fn GetPeakValue(
        pfPeak: *mut c_float,
    ) -> HRESULT,
    fn GetMeteringChannelCount(
        pnChannelCount: *mut UINT,
    ) -> HRESULT,
    fn GetChannelsPeakValues(
        u32ChannelCount: UINT32,
        afPeakValues: *mut c_float,
    ) -> HRESULT,
    fn QueryHardwareSupport(
        pdwHardwareSupportMask: *mut DWORD,
    ) -> HRESULT,
}}
