// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! this ALWAYS GENERATED file contains the definitions for the interfaces
use ctypes::c_void;
use shared::guiddef::{GUID, REFIID};
use shared::minwindef::{DWORD, LPARAM, LPVOID, UINT};
// use shared::winerror::{ERROR_NOT_FOUND, ERROR_UNSUPPORTED_TYPE, HRESULT_FROM_WIN32};
use shared::wtypes::PROPERTYKEY;
use um::propidl::PROPVARIANT;
use um::propsys::IPropertyStore;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR};
// pub const E_NOTFOUND: HRESULT = HRESULT_FROM_WIN32(ERROR_NOT_FOUND);
// pub const E_UNSUPPORTED_TYPE: HRESULT = HRESULT_FROM_WIN32(ERROR_UNSUPPORTED_TYPE);
pub const DEVICE_STATE_ACTIVE: DWORD = 0x00000001;
pub const DEVICE_STATE_DISABLED: DWORD = 0x00000002;
pub const DEVICE_STATE_NOTPRESENT: DWORD = 0x00000004;
pub const DEVICE_STATE_UNPLUGGED: DWORD = 0x00000008;
pub const DEVICE_STATEMASK_ALL: DWORD = 0x0000000F;
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_FormFactor,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 0}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_ControlPanelPageProvider,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 1}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_Association,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 2}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_PhysicalSpeakers,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 3}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_GUID,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 4}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_Disable_SysFx,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 5}
pub const ENDPOINT_SYSFX_ENABLED: DWORD = 0x00000000;
pub const ENDPOINT_SYSFX_DISABLED: DWORD = 0x00000001;
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_FullRangeSpeakers,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 6}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_Supports_EventDriven_Mode,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 7}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_JackSubType,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 8}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpoint_Default_VolumeInDb,
    0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 9}
DEFINE_PROPERTYKEY!{PKEY_AudioEngine_DeviceFormat,
    0xf19f064d, 0x82c, 0x4e27, 0xbc, 0x73, 0x68, 0x82, 0xa1, 0xbb, 0x8e, 0x4c, 0}
DEFINE_PROPERTYKEY!{PKEY_AudioEngine_OEMFormat,
    0xe4870e26, 0x3cc5, 0x4cd2, 0xba, 0x46, 0xca, 0xa, 0x9a, 0x70, 0xed, 0x4, 3}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpointLogo_IconEffects,
    0xf1ab780d, 0x2010, 0x4ed3, 0xa3, 0xa6, 0x8b, 0x87, 0xf0, 0xf0, 0xc4, 0x76, 0}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpointLogo_IconPath,
    0xf1ab780d, 0x2010, 0x4ed3, 0xa3, 0xa6, 0x8b, 0x87, 0xf0, 0xf0, 0xc4, 0x76, 1}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpointSettings_MenuText,
    0x14242002, 0x0320, 0x4de4, 0x95, 0x55, 0xa7, 0xd8, 0x2b, 0x73, 0xc2, 0x86, 0}
DEFINE_PROPERTYKEY!{PKEY_AudioEndpointSettings_LaunchContract,
    0x14242002, 0x0320, 0x4de4, 0x95, 0x55, 0xa7, 0xd8, 0x2b, 0x73, 0xc2, 0x86, 1}
STRUCT!{struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    cbDirectXAudioActivationParams: DWORD,
    guidAudioSession: GUID,
    dwAudioStreamFlags: DWORD,
}}
pub type PDIRECTX_AUDIO_ACTIVATION_PARAMS = *mut DIRECTX_AUDIO_ACTIVATION_PARAMS;
ENUM!{enum EDataFlow {
    eRender,
    eCapture,
    eAll,
    EDataFlow_enum_count,
}}
ENUM!{enum ERole {
    eConsole,
    eMultimedia,
    eCommunications,
    ERole_enum_count,
}}
ENUM!{enum EndpointFormFactor {
    RemoteNetworkDevice,
    Speakers,
    LineLevel,
    Headphones,
    Microphone,
    Headset,
    Handset,
    UnknownDigitalPassthrough,
    SPDIF,
    DigitalAudioDisplayDevice,
    UnknownFormFactor,
    EndpointFormFactor_enum_count,
}}
pub const HDMI: EndpointFormFactor = DigitalAudioDisplayDevice;
DEFINE_GUID!{DEVINTERFACE_AUDIO_RENDER,
    0xe6327cad, 0xdcec, 0x4949, 0xae, 0x8a, 0x99, 0x1e, 0x97, 0x6a, 0x79, 0xd2}
DEFINE_GUID!{DEVINTERFACE_AUDIO_CAPTURE,
    0x2eef81be, 0x33fa, 0x4800, 0x96, 0x70, 0x1c, 0xd4, 0x74, 0x97, 0x2c, 0x3f}
DEFINE_GUID!{DEVINTERFACE_MIDI_OUTPUT,
    0x6dc23320, 0xab33, 0x4ce4, 0x80, 0xd4, 0xbb, 0xb3, 0xeb, 0xbf, 0x28, 0x14}
DEFINE_GUID!{DEVINTERFACE_MIDI_INPUT,
    0x504be32c, 0xccf6, 0x4d2c, 0xb7, 0x3f, 0x6f, 0x8b, 0x37, 0x47, 0xe2, 0x2b}
RIDL!{#[uuid(0x7991eec9, 0x7e89, 0x4d85, 0x83, 0x90, 0x6c, 0x70, 0x3c, 0xec, 0x60, 0xc0)]
interface IMMNotificationClient(IMMNotificationClientVtbl): IUnknown(IUnknownVtbl) {
    fn OnDeviceStateChanged(
        pwstrDeviceId: LPCWSTR,
        dwNewState: DWORD,
    ) -> HRESULT,
    fn OnDeviceAdded(
        pwstrDeviceId: LPCWSTR,
    ) -> HRESULT,
    fn OnDeviceRemoved(
        pwstrDeviceId: LPCWSTR,
    ) -> HRESULT,
    fn OnDefaultDeviceChanged(
        flow: EDataFlow,
        role: ERole,
        pwstrDefaultDeviceId: LPCWSTR,
    ) -> HRESULT,
    fn OnPropertyValueChanged(
        pwstrDeviceId: LPCWSTR,
        key: PROPERTYKEY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd666063f, 0x1587, 0x4e43, 0x81, 0xf1, 0xb9, 0x48, 0xe8, 0x07, 0x36, 0x3f)]
interface IMMDevice(IMMDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn Activate(
        iid: REFIID,
        dwClsCtx: DWORD,
        pActivationParams: *mut PROPVARIANT,
        ppInterface: *mut LPVOID,
    ) -> HRESULT,
    fn OpenPropertyStore(
        stgmAccess: DWORD,
        ppProperties: *mut *mut IPropertyStore,
    ) -> HRESULT,
    fn GetId(
        ppstrId: *mut LPWSTR,
    ) -> HRESULT,
    fn GetState(
        pdwState: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0bd7a1be, 0x7a1a, 0x44db, 0x83, 0x97, 0xcc, 0x53, 0x92, 0x38, 0x7b, 0x5e)]
interface IMMDeviceCollection(IMMDeviceCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcDevices: *const UINT,
    ) -> HRESULT,
    fn Item(
        nDevice: UINT,
        ppDevice: *mut *mut IMMDevice,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1be09788, 0x6894, 0x4089, 0x85, 0x86, 0x9a, 0x2a, 0x6c, 0x26, 0x5a, 0xc5)]
interface IMMEndpoint(IMMEndpointVtbl): IUnknown(IUnknownVtbl) {
    fn GetDataFlow(
        pDataFlow: *mut EDataFlow,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa95664d2, 0x9614, 0x4f35, 0xa7, 0x46, 0xde, 0x8d, 0xb6, 0x36, 0x17, 0xe6)]
interface IMMDeviceEnumerator(IMMDeviceEnumeratorVtbl): IUnknown(IUnknownVtbl) {
    fn EnumAudioEndpoints(
        dataFlow: EDataFlow,
        dwStateMask: DWORD,
        ppDevices: *mut *mut IMMDeviceCollection,
    ) -> HRESULT,
    fn GetDefaultAudioEndpoint(
        dataFlow: EDataFlow,
        role: ERole,
        ppEndpoint: *mut *mut IMMDevice,
    ) -> HRESULT,
    fn GetDevice(
        pwstrId: LPCWSTR,
        ppDevices: *mut *mut IMMDevice,
    ) -> HRESULT,
    fn RegisterEndpointNotificationCallback(
        pClient: *mut IMMNotificationClient,
    ) -> HRESULT,
    fn UnregisterEndpointNotificationCallback(
        pClient: *mut IMMNotificationClient,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3b0d0ea4, 0xd0a9, 0x4b0e, 0x93, 0x5b, 0x09, 0x51, 0x67, 0x46, 0xfa, 0xc0)]
interface IMMDeviceActivator(IMMDeviceActivatorVtbl): IUnknown(IUnknownVtbl) {
    fn Activate(
        iid: REFIID,
        pDevice: *mut IMMDevice,
        pActivationParams: *mut PROPVARIANT,
        ppInterface: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x41d949ab, 0x9862, 0x444a, 0x80, 0xf6, 0xc2, 0x61, 0x33, 0x4d, 0xa5, 0xeb)]
interface IActivateAudioInterfaceCompletionHandler(IActivateAudioInterfaceCompletionHandlerVtbl):
    IUnknown(IUnknownVtbl) {
    fn ActivateCompleted(
        activateOperation: *mut IActivateAudioInterfaceAsyncOperation,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x72a22d78, 0xcde4, 0x431d, 0xb8, 0xcc, 0x84, 0x3a, 0x71, 0x19, 0x9b, 0x6d)]
interface IActivateAudioInterfaceAsyncOperation(IActivateAudioInterfaceAsyncOperationVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetActivateResult(
        activateResult: *mut HRESULT,
        activatedInterface: *mut *mut IUnknown,
    ) -> HRESULT,
}}
extern "system" {
    pub fn ActivateAudioInterfaceAsync(
        deviceInterfacePath: LPCWSTR,
        riid: REFIID,
        activationParams: *mut PROPVARIANT,
        completionHandler: *mut IActivateAudioInterfaceCompletionHandler,
        activationOperation: *mut *mut IActivateAudioInterfaceAsyncOperation,
    ) -> HRESULT;
}
STRUCT!{struct AudioExtensionParams {
    AddPageParam: LPARAM,
    pEndpoint: *mut IMMDevice,
    pPnpInterface: *mut IMMDevice,
    pPnpDevnode: *mut IMMDevice,
}}
DEFINE_GUID!{CLSID_MMDeviceEnumerator,
    0xBCDE0395, 0xE52F, 0x467C, 0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E}
RIDL!{#[uuid(0xBCDE0395, 0xE52F, 0x467C, 0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E)]
class MMDeviceEnumerator;}
