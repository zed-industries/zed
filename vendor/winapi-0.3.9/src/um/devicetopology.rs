// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! The DeviceTopology API gives clients control over a variety of internal functions of audio
//! adapters that they cannot access through the MMDevice API, WASAPI, or the EndpointVolume API.
use ctypes::{c_float, c_void};
use shared::guiddef::{GUID, LPCGUID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, UCHAR, UINT, ULONG, WORD};
use shared::windef::COLORREF;
use shared::wtypes::VARTYPE;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LONGLONG, LPWSTR, WCHAR};
DEFINE_GUID!{EVENTCONTEXT_VOLUMESLIDER,
    0xe2c2e9de, 0x09b1, 0x4b04, 0x84, 0xe5, 0x07, 0x93, 0x12, 0x25, 0xee, 0x04}
STRUCT!{struct KSDATAFORMAT {
    FormatSize: ULONG,
    Flags: ULONG,
    SampleSize: ULONG,
    Reserved: ULONG,
    MajorFormat: GUID,
    SubFormat: GUID,
    Specifier: GUID,
}}
pub type PKSDATAFORMAT = *mut KSDATAFORMAT;
STRUCT!{struct KSIDENTIFIER_s {
    Set: GUID,
    Id: ULONG,
    Flags: ULONG,
}}
UNION!{union KSIDENTIFIER {
    [u64; 3],
    s s_mut: KSIDENTIFIER_s,
    Alignment Alignment_mut: LONGLONG,
}}
pub type KSPROPERTY = KSIDENTIFIER;
pub type PKSPROPERTY = *mut KSIDENTIFIER;
pub type KSMETHOD = KSIDENTIFIER;
pub type PKSMETHOD = *mut KSIDENTIFIER;
pub type KSEVENT = KSIDENTIFIER;
pub type PKSEVENT = *mut KSIDENTIFIER;
ENUM!{enum EPcxConnectionType {
    eConnTypeUnknown = 0,
    eConnType3Point5mm = 1,
    eConnTypeQuarter = 2,
    eConnTypeAtapiInternal = 3,
    eConnTypeRCA = 4,
    eConnTypeOptical = 5,
    eConnTypeOtherDigital = 6,
    eConnTypeOtherAnalog = 7,
    eConnTypeMultichannelAnalogDIN = 8,
    eConnTypeXlrProfessional = 9,
    eConnTypeRJ11Modem = 10,
    eConnTypeCombination = 11,
}}
ENUM!{enum EPcxGeoLocation {
    eGeoLocRear = 1,
    eGeoLocFront = 2,
    eGeoLocLeft = 3,
    eGeoLocRight = 4,
    eGeoLocTop = 5,
    eGeoLocBottom = 6,
    eGeoLocRearPanel = 7,
    eGeoLocRiser = 8,
    eGeoLocInsideMobileLid = 9,
    eGeoLocDrivebay = 10,
    eGeoLocHDMI = 11,
    eGeoLocOutsideMobileLid = 12,
    eGeoLocATAPI = 13,
    eGeoLocNotApplicable = 14,
    eGeoLocReserved6 = 15,
}}
ENUM!{enum EPcxGenLocation {
    eGenLocPrimaryBox = 0,
    eGenLocInternal = 1,
    eGenLocSeparate = 2,
    eGenLocOther = 3,
}}
ENUM!{enum EPxcPortConnection {
    ePortConnJack = 0,
    ePortConnIntegratedDevice = 1,
    ePortConnBothIntegratedAndJack = 2,
    ePortConnUnknown = 3,
}}
STRUCT!{struct KSJACK_DESCRIPTION {
    ChannelMapping: DWORD,
    Color: COLORREF,
    ConnectionType: EPcxConnectionType,
    GeoLocation: EPcxGeoLocation,
    GenLocation: EPcxGenLocation,
    PortConnection: EPxcPortConnection,
    IsConnected: BOOL,
}}
pub type PKSJACK_DESCRIPTION = *mut KSJACK_DESCRIPTION;
STRUCT!{struct LUID {
    LowPart: DWORD,
    HighPart: LONG,
}}
pub type PLUID = *mut LUID;
ENUM!{enum KSJACK_SINK_CONNECTIONTYPE {
    KSJACK_SINK_CONNECTIONTYPE_HDMI = 0,
    KSJACK_SINK_CONNECTIONTYPE_DISPLAYPORT = 1,
}}
STRUCT!{struct KSJACK_SINK_INFORMATION {
    ConnType: KSJACK_SINK_CONNECTIONTYPE,
    ManufacturerId: WORD,
    ProductId: WORD,
    AudioLatency: WORD,
    HDCPCapable: BOOL,
    AICapable: BOOL,
    SinkDescriptionLength: UCHAR,
    SinkDescription: [WCHAR; 32],
    PortId: LUID,
}}
STRUCT!{struct KSJACK_DESCRIPTION2 {
    DeviceStateInfo: DWORD,
    JackCapabilities: DWORD,
}}
pub type PKSJACK_DESCRIPTION2 = *mut KSJACK_DESCRIPTION2;
ENUM!{enum DataFlow {
    In = 0,
    Out = 1,
}}
ENUM!{enum PartType {
    Connector = 0,
    Subunit = 1,
}}
ENUM!{enum ConnectorType {
    Unknown_Connector = 0,
    Physical_Internal = 1,
    Physical_External = 2,
    Software_IO = 3,
    Software_Fixed = 4,
    Network = 5,
}}
RIDL!{#[uuid(0x28f54685, 0x06fd, 0x11d2, 0xb2, 0x7a, 0x00, 0xa0, 0xc9, 0x22, 0x31, 0x96)]
interface IKsControl(IKsControlVtbl): IUnknown(IUnknownVtbl) {
    fn KsProperty(
        Property: PKSPROPERTY,
        PropertyLength: ULONG,
        PropertyData: *mut c_void,
        DataLength: ULONG,
        BytesReturned: *mut ULONG,
    ) -> HRESULT,
    fn KsMethod(
        Method: PKSMETHOD,
        MethodLength: ULONG,
        MethodData: *mut c_void,
        DataLength: ULONG,
        BytesReturned: *mut ULONG,
    ) -> HRESULT,
    fn KsEvent(
        Event: PKSEVENT,
        EventLength: ULONG,
        EventData: *mut c_void,
        DataLength: ULONG,
        BytesReturned: *mut ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc2f8e001, 0xf205, 0x4bc9, 0x99, 0xbc, 0xc1, 0x3b, 0x1e, 0x04, 0x8c, 0xcb)]
interface IPerChannelDbLevel(IPerChannelDbLevelVtbl): IUnknown(IUnknownVtbl) {
    fn GetChannelCount(
        pcChannels: *mut UINT,
    ) -> HRESULT,
    fn GetLevelRange(
        nChannel: UINT,
        pfMinLevelDB: *mut c_float,
        pfMaxLevelDB: *mut c_float,
        pfStepping: *mut c_float,
    ) -> HRESULT,
    fn GetLevel(
        nChannel: UINT,
        pfLevelDB: *mut c_float,
    ) -> HRESULT,
    fn SetLevel(
        nChannel: UINT,
        fLevelDB: c_float,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn SetLevelUniform(
        fLevelDB: c_float,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn SetLevelAllChannels(
        aLevelsDB: *mut c_float,
        cChannels: ULONG,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7fb7b48f, 0x531d, 0x44a2, 0xbc, 0xb3, 0x5a, 0xd5, 0xa1, 0x34, 0xb3, 0xdc)]
interface IAudioVolumeLevel(IAudioVolumeLevelVtbl): IPerChannelDbLevel(IPerChannelDbLevelVtbl) {}}
RIDL!{#[uuid(0xbb11c46f, 0xec28, 0x493c, 0xb8, 0x8a, 0x5d, 0xb8, 0x80, 0x62, 0xce, 0x98)]
interface IAudioChannelConfig(IAudioChannelConfigVtbl): IUnknown(IUnknownVtbl) {
    fn SetChannelConfig(
        dwConfig: DWORD,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn GetChannelConfig(
        pdwConfig: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7d8b1437, 0xdd53, 0x4350, 0x9c, 0x1b, 0x1e, 0xe2, 0x89, 0x0b, 0xd9, 0x38)]
interface IAudioLoudness(IAudioLoudnessVtbl): IUnknown(IUnknownVtbl) {
    fn GetEnabled(
        pbEnabled: *mut BOOL,
    ) -> HRESULT,
    fn SetEnabled(
        bEnable: BOOL,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4f03dc02, 0x5e6e, 0x4653, 0x8f, 0x72, 0xa0, 0x30, 0xc1, 0x23, 0xd5, 0x98)]
interface IAudioInputSelector(IAudioInputSelectorVtbl): IUnknown(IUnknownVtbl) {
    fn GetSelection(
        pnIdSelected: *mut UINT,
    ) -> HRESULT,
    fn SetSelection(
        nIdSelect: UINT,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbb515f69, 0x94a7, 0x429e, 0x8b, 0x9c, 0x27, 0x1b, 0x3f, 0x11, 0xa3, 0xab)]
interface IAudioOutputSelector(IAudioOutputSelectorVtbl): IUnknown(IUnknownVtbl) {
    fn GetSelection(
        pnIdSelected: *mut UINT,
    ) -> HRESULT,
    fn SetSelection(
        nIdSelect: UINT,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdf45aeea, 0xb74a, 0x4b6b, 0xaf, 0xad, 0x23, 0x66, 0xb6, 0xaa, 0x01, 0x2e)]
interface IAudioMute(IAudioMuteVtbl): IUnknown(IUnknownVtbl) {
    fn SetMute(
        bMuted: BOOL,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn GetMute(
        pbMuted: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa2b1a1d9, 0x4db3, 0x425d, 0xa2, 0xb2, 0xbd, 0x33, 0x5c, 0xb3, 0xe2, 0xe5)]
interface IAudioBass(IAudioBassVtbl): IPerChannelDbLevel(IPerChannelDbLevelVtbl) {}}
RIDL!{#[uuid(0x5e54b6d7, 0xb44b, 0x40d9, 0x9a, 0x9e, 0xe6, 0x91, 0xd9, 0xce, 0x6e, 0xdf)]
interface IAudioMidrange(IAudioMidrangeVtbl): IPerChannelDbLevel(IPerChannelDbLevelVtbl) {}}
RIDL!{#[uuid(0x0a717812, 0x694e, 0x4907, 0xb7, 0x4b, 0xba, 0xfa, 0x5c, 0xfd, 0xca, 0x7b)]
interface IAudioTreble(IAudioTrebleVtbl): IPerChannelDbLevel(IPerChannelDbLevelVtbl) {}}
RIDL!{#[uuid(0x85401fd4, 0x6de4, 0x4b9d, 0x98, 0x69, 0x2d, 0x67, 0x53, 0xa8, 0x2f, 0x3c)]
interface IAudioAutoGainControl(IAudioAutoGainControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetEnabled(
        pbEnabled: *mut BOOL,
    ) -> HRESULT,
    fn SetEnabled(
        bEnable: BOOL,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdd79923c, 0x0599, 0x45e0, 0xb8, 0xb6, 0xc8, 0xdf, 0x7d, 0xb6, 0xe7, 0x96)]
interface IAudioPeakMeter(IAudioPeakMeterVtbl): IUnknown(IUnknownVtbl) {
    fn GetChannelCount(
        pcChannels: *mut UINT,
    ) -> HRESULT,
    fn GetLevel(
        nChannel: UINT,
        pfLevel: *mut c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3b22bcbf, 0x2586, 0x4af0, 0x85, 0x83, 0x20, 0x5d, 0x39, 0x1b, 0x80, 0x7c)]
interface IDeviceSpecificProperty(IDeviceSpecificPropertyVtbl): IUnknown(IUnknownVtbl) {
    fn GetType(
        pVType: *mut VARTYPE,
    ) -> HRESULT,
    fn GetValue(
        pvValue: *mut c_void,
        pcbValue: *mut DWORD,
    ) -> HRESULT,
    fn SetValue(
        pvValue: *mut c_void,
        cbValue: DWORD,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
    fn Get4BRange(
        plMin: *mut LONG,
        plMax: *mut LONG,
        plStepping: *mut LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3cb4a69d, 0xbb6f, 0x4d2b, 0x95, 0xb7, 0x45, 0x2d, 0x2c, 0x15, 0x5d, 0xb5)]
interface IKsFormatSupport(IKsFormatSupportVtbl): IUnknown(IUnknownVtbl) {
    fn IsFormatSupported(
        pKsFormat: PKSDATAFORMAT,
        cbFormat: DWORD,
        pbSupported: *mut BOOL,
    ) -> HRESULT,
    fn GetDevicePreferredFormat(
        ppKsFormat: *mut PKSDATAFORMAT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4509f757, 0x2d46, 0x4637, 0x8e, 0x62, 0xce, 0x7d, 0xb9, 0x44, 0xf5, 0x7b)]
interface IKsJackDescription(IKsJackDescriptionVtbl): IUnknown(IUnknownVtbl) {
    fn GetJackCount(
        pcJacks: *mut UINT,
    ) -> HRESULT,
    fn GetJackDescription(
        nJack: UINT,
        pDescription: *mut KSJACK_DESCRIPTION,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x478f3a9b, 0xe0c9, 0x4827, 0x92, 0x28, 0x6f, 0x55, 0x05, 0xff, 0xe7, 0x6a)]
interface IKsJackDescription2(IKsJackDescription2Vtbl): IUnknown(IUnknownVtbl) {
    fn GetJackCount(
        pcJacks: *mut UINT,
    ) -> HRESULT,
    fn GetJackDescription2(
        nJack: UINT,
        pDescription2: *mut KSJACK_DESCRIPTION2,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd9bd72ed, 0x290f, 0x4581, 0x9f, 0xf3, 0x61, 0x02, 0x7a, 0x8f, 0xe5, 0x32)]
interface IKsJackSinkInformation(IKsJackSinkInformationVtbl): IUnknown(IUnknownVtbl) {
    fn GetJackSinkInformation(
        pJackSinkInformation: *mut KSJACK_SINK_INFORMATION,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc99af463, 0xd629, 0x4ec4, 0x8c, 0x00, 0xe5, 0x4d, 0x68, 0x15, 0x42, 0x48)]
interface IKsJackContainerId(IKsJackContainerIdVtbl): IUnknown(IUnknownVtbl) {
    fn GetJackContainerId(
        pJackContainerId: *mut GUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6daa848c, 0x5eb0, 0x45cc, 0xae, 0xa5, 0x99, 0x8a, 0x2c, 0xda, 0x1f, 0xfb)]
interface IPartsList(IPartsListVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pCount: *mut UINT,
    ) -> HRESULT,
    fn GetPart(
        nIndex: UINT,
        ppPart: *mut *mut IPart,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xae2de0e4, 0x5bca, 0x4f2d, 0xaa, 0x46, 0x5d, 0x13, 0xf8, 0xfd, 0xb3, 0xa9)]
interface IPart(IPartVtbl): IUnknown(IUnknownVtbl) {
    fn GetName(
        ppwstrName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetLocalId(
        pnId: *mut UINT,
    ) -> HRESULT,
    fn GetGlobalId(
        ppwstrGlobalId: *mut LPWSTR,
    ) -> HRESULT,
    fn GetPartType(
        pPartType: *mut PartType,
    ) -> HRESULT,
    fn GetSubType(
        pSubType: *mut GUID,
    ) -> HRESULT,
    fn GetControlInterfaceCount(
        pCount: *mut UINT,
    ) -> HRESULT,
    fn GetControlInterface(
        nIndex: UINT,
        ppInterfaceDesc: *mut *mut IControlInterface,
    ) -> HRESULT,
    fn EnumPartsIncoming(
        ppParts: *mut *mut IPartsList,
    ) -> HRESULT,
    fn EnumPartsOutgoing(
        ppParts: *mut *mut IPartsList,
    ) -> HRESULT,
    fn GetTopologyObject(
        ppTopology: *mut *mut IDeviceTopology,
    ) -> HRESULT,
    fn Activate(
        dwClsContext: DWORD,
        refiid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn RegisterControlChangeCallback(
        riid: REFGUID,
        pNotify: *mut IControlChangeNotify,
    ) -> HRESULT,
    fn UnregisterControlChangeCallback(
        pNotify: *mut IControlChangeNotify,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9c2c4058, 0x23f5, 0x41de, 0x87, 0x7a, 0xdf, 0x3a, 0xf2, 0x36, 0xa0, 0x9e)]
interface IConnector(IConnectorVtbl): IUnknown(IUnknownVtbl) {
    fn GetType(
        pType: *mut ConnectorType,
    ) -> HRESULT,
    fn GetDataFlow(
        pFlow: *mut DataFlow,
    ) -> HRESULT,
    fn ConnectTo(
        pConnectTo: *mut IConnector,
    ) -> HRESULT,
    fn Disconnect() -> HRESULT,
    fn IsConnected(
        pbConnected: *mut BOOL,
    ) -> HRESULT,
    fn GetConnectedTo(
        ppConTo: *mut *mut IConnector,
    ) -> HRESULT,
    fn GetConnectorIdConnectedTo(
        ppwstrConnectorId: *mut LPWSTR,
    ) -> HRESULT,
    fn GetDeviceIdConnectedTo(
        ppwstrDeviceId: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x82149a85, 0xdba6, 0x4487, 0x86, 0xbb, 0xea, 0x8f, 0x7f, 0xef, 0xcc, 0x71)]
interface ISubunit(ISubunitVtbl): IUnknown(IUnknownVtbl) {}}
RIDL!{#[uuid(0x45d37c3f, 0x5140, 0x444a, 0xae, 0x24, 0x40, 0x07, 0x89, 0xf3, 0xcb, 0xf3)]
interface IControlInterface(IControlInterfaceVtbl): IUnknown(IUnknownVtbl) {
    fn GetName(
        ppwstrName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetIID(
        pIID: *mut GUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa09513ed, 0xc709, 0x4d21, 0xbd, 0x7b, 0x5f, 0x34, 0xc4, 0x7f, 0x39, 0x47)]
interface IControlChangeNotify(IControlChangeNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn OnNotify(
        dwSenderProcessId: DWORD,
        pguidEventContext: LPCGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2a07407e, 0x6497, 0x4a18, 0x97, 0x87, 0x32, 0xf7, 0x9b, 0xd0, 0xd9, 0x8f)]
interface IDeviceTopology(IDeviceTopologyVtbl): IUnknown(IUnknownVtbl) {
    fn GetConnectorCount(
        pCount: *mut UINT,
    ) -> HRESULT,
    fn GetConnector(
        nIndex: UINT,
        ppConnector: *mut *mut IConnector,
    ) -> HRESULT,
    fn GetSubunitCount(
        pCount: *mut UINT,
    ) -> HRESULT,
    fn GetSubunit(
        nIndex: UINT,
        ppSubunit: *mut *mut ISubunit,
    ) -> HRESULT,
    fn GetPartById(
        nId: UINT,
        ppPart: *mut *mut IPart,
    ) -> HRESULT,
    fn GetDeviceId(
        ppwstrDeviceId: *mut LPWSTR,
    ) -> HRESULT,
    fn GetSignalPath(
        pIPartFrom: *mut IPart,
        pIPartTo: *mut IPart,
        bRejectMixedPaths: BOOL,
        ppParts: *mut *mut IPartsList,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1df639d0, 0x5ec1, 0x47aa, 0x93, 0x79, 0x82, 0x8d, 0xc1, 0xaa, 0x8c, 0x59)]
class DeviceTopology;}
