// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of PortableDeviceApi.h
use shared::guiddef::REFGUID;
use shared::minwindef::{BYTE, DWORD, ULONG};
use um::objidlbase::IStream;
use um::portabledevicetypes::{
    IPortableDeviceKeyCollection, IPortableDevicePropVariantCollection, IPortableDeviceValues,
};
use um::propkeydef::REFPROPERTYKEY;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR, WCHAR};
// 328
RIDL!{#[uuid(0xa1567595, 0x4c2f, 0x4574, 0xa6, 0xfa, 0xec, 0xef, 0x91, 0x7b, 0x9a, 0x40)]
interface IPortableDeviceManager(IPortableDeviceManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetDevices(
        pPnPDeviceIDs: *mut LPWSTR,
        pcPnPDeviceIDs: *mut DWORD,
    ) -> HRESULT,
    fn RefreshDeviceList() -> HRESULT,
    fn GetDeviceFriendlyName(
        pszPnPDeviceID: LPCWSTR,
        pDeviceFriendlyName: *mut WCHAR,
        pcchDeviceFriendlyName: *mut DWORD,
    ) -> HRESULT,
    fn GetDeviceDescription(
        pszPnPDeviceID: LPCWSTR,
        pDeviceDescription: *mut WCHAR,
        pcchDeviceDescription: *mut DWORD,
    ) -> HRESULT,
    fn GetDeviceManufacturer(
        pszPnPDeviceID: LPCWSTR,
        pDeviceManufacturer: *mut WCHAR,
        pcchDeviceManufacturer: *mut DWORD,
    ) -> HRESULT,
    fn GetDeviceProperty(
        pszPnPDeviceID: LPCWSTR,
        pszDevicePropertyName: LPCWSTR,
        pData: *mut BYTE,
        pcbData: *mut DWORD,
        pdwType: *mut DWORD,
    ) -> HRESULT,
    fn GetPrivateDevices(
        pPnPDeviceIDs: *mut LPWSTR,
        pcPnPDeviceIDs: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x625e2df8, 0x6392, 0x4cf0, 0x9a, 0xd1, 0x3c, 0xfa, 0x5f, 0x17, 0x77, 0x5c)]
interface IPortableDevice(IPortableDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn Open(
        pszPnPDeviceID: LPCWSTR,
        pClientInfo: *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn SendCommand(
        dwFlags: DWORD,
        pParameters: *mut IPortableDeviceValues,
        ppResults: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn Content(
        ppContent: *mut *mut IPortableDeviceContent,
    ) -> HRESULT,
    fn Capabilities(
        ppCapabilities: *mut *mut IPortableDeviceCapabilities,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
    fn Close() -> HRESULT,
    fn Advise(
        dwFlags: DWORD,
        pCallback: *mut IPortableDeviceEventCallback,
        pParameters: *mut IPortableDeviceValues,
        ppszCookie: *mut LPWSTR,
    ) -> HRESULT,
    fn Unadvise(
        pszCookie: LPCWSTR,
    ) -> HRESULT,
    fn GetPnPDeviceID(
        ppszPnPDeviceID: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6a96ed84, 0x7c73, 0x4480, 0x99, 0x38, 0xbf, 0x5a, 0xf4, 0x77, 0xd4, 0x26)]
interface IPortableDeviceContent(IPortableDeviceContentVtbl): IUnknown(IUnknownVtbl) {
    fn EnumObjects(
        dwFlags: DWORD,
        pszParentObjectID: LPCWSTR,
        pFilter: *mut IPortableDeviceValues,
        ppEnum: *mut *mut IEnumPortableDeviceObjectIDs,
    ) -> HRESULT,
    fn Properties(
        ppProperties: *mut *mut IPortableDeviceProperties,
    ) -> HRESULT,
    fn Transfer(
        ppResources: *mut *mut IPortableDeviceResources,
    ) -> HRESULT,
    fn CreateObjectWithPropertiesOnly(
        pValues: *mut IPortableDeviceValues,
        ppszObjectID: *mut LPWSTR,
    ) -> HRESULT,
    fn CreateObjectWithPropertiesAndData(
        pValues: *mut IPortableDeviceValues,
        ppData: *mut *mut IStream,
        pdwOptimalWriteBufferSize: *mut DWORD,
        ppszCookie: *mut LPWSTR,
    ) -> HRESULT,
    fn Delete(
        dwOptions: DWORD,
        pObjectIDs: *mut IPortableDevicePropVariantCollection,
        ppResults: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetObjectIDsFromPersistentUniqueIDs(
        pPersistentUniqueIDs: *mut IPortableDevicePropVariantCollection,
        ppObjectIDs: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
    fn Move(
        pObjectIDs: *mut IPortableDevicePropVariantCollection,
        pszDestinationFolderObjectID: LPCWSTR,
        ppResults: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn Copy(
        pObjectIDs: *mut IPortableDevicePropVariantCollection,
        pszDestinationFolderObjectID: LPCWSTR,
        ppResults: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9b4add96, 0xf6bf, 0x4034, 0x87, 0x08, 0xec, 0xa7, 0x2b, 0xf1, 0x05, 0x54)]
interface IPortableDeviceContent2(IPortableDeviceContent2Vtbl):
    IPortableDeviceContent(IPortableDeviceContentVtbl) {
    fn UpdateObjectWithPropertiesAndData(
        pszObjectID: LPCWSTR,
        pProperties: *mut IPortableDeviceValues,
        ppData: *mut *mut IStream,
        pdwOptimalWriteBufferSize: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x10ece955, 0xcf41, 0x4728, 0xbf, 0xa0, 0x41, 0xee, 0xdf, 0x1b, 0xbf, 0x19)]
interface IEnumPortableDeviceObjectIDs(IEnumPortableDeviceObjectIDsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cObjects: ULONG,
        pObjIDs: *mut LPWSTR,
        pcFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cObjects: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumPortableDeviceObjectIDs,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
}}
RIDL!{#[uuid(0x7f6d695c, 0x03df, 0x4439, 0xa8, 0x09, 0x59, 0x26, 0x6b, 0xee, 0xe3, 0xa6)]
interface IPortableDeviceProperties(IPortableDevicePropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn GetSupportedProperties(
        pszObjectID: LPCWSTR,
        ppKeys: *mut *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn GetPropertyAttributes(
        pszObjectID: LPCWSTR,
        Key: REFPROPERTYKEY,
        ppAttributes: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn GetValues(
        pszObjectID: LPCWSTR,
        pKeys: *mut IPortableDeviceKeyCollection,
        ppValues: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn SetValues(
        pszObjectID: LPCWSTR,
        pValues: *mut IPortableDeviceValues,
        ppResults: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn Delete(
        pszObjectID: LPCWSTR,
        pKeys: *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
}}
RIDL!{#[uuid(0xfd8878ac, 0xd841, 0x4d17, 0x89, 0x1c, 0xe6, 0x82, 0x9c, 0xdb, 0x69, 0x34)]
interface IPortableDeviceResources(IPortableDeviceResourcesVtbl): IUnknown(IUnknownVtbl) {
    fn GetSupportedResources(
        pszObjectID: LPCWSTR,
        ppKeys: *mut *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn GetResourceAttributes(
        pszObjectID: LPCWSTR,
        Key: REFPROPERTYKEY,
        ppResourceAttributes: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn GetStream(
        pszObjectID: LPCWSTR,
        Key: REFPROPERTYKEY,
        dwMode: DWORD,
        pdwOptimalBufferSize: *mut DWORD,
        ppStream: *mut *mut IStream,
    ) -> HRESULT,
    fn Delete(
        pszObjectID: LPCWSTR,
        pKeys: *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
    fn CreateResource(
        pResourceAttributes: *mut IPortableDeviceValues,
        ppData: *mut *mut IStream,
        pdwOptimalWriteBufferSize: *mut DWORD,
        ppszCookie: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2c8c6dbf, 0xe3dc, 0x4061, 0xbe, 0xcc, 0x85, 0x42, 0xe8, 0x10, 0xd1, 0x26)]
interface IPortableDeviceCapabilities(IPortableDeviceCapabilitiesVtbl): IUnknown(IUnknownVtbl) {
    fn GetSupportedCommands(
        ppCommands: *mut *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn GetCommandOptions(
        Command: REFPROPERTYKEY,
        ppOptions: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn GetFunctionalCategories(
        ppCategories: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetFunctionalObjects(
        Category: REFGUID,
        ppObjectIDs: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetSupportedContentTypes(
        Category: REFGUID,
        ppContentTypes: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetSupportedFormats(
        ContentType: REFGUID,
        ppFormats: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetSupportedFormatProperties(
        Format: REFGUID,
        ppKeys: *mut *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn GetFixedPropertyAttributes(
        Format: REFGUID,
        Key: REFPROPERTYKEY,
        ppAttributes: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
    fn GetSupportedEvents(
        ppEvents: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetEventOptions(
        Event: REFGUID,
        ppOptions: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa8792a31, 0xf385, 0x493c, 0xa8, 0x93, 0x40, 0xf6, 0x4e, 0xb4, 0x5f, 0x6e)]
interface IPortableDeviceEventCallback(IPortableDeviceEventCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnEvent(
        pEventParameters: *mut IPortableDeviceValues,
    ) -> HRESULT,
}}
DEFINE_GUID!{LIBID_PortableDeviceApiLib,
    0x1f001332, 0x1a57, 0x4934, 0xbe, 0x31, 0xaf, 0xfc, 0x99, 0xf4, 0xee, 0x0a}
DEFINE_GUID!{CLSID_PortableDevice,
    0x728a21c5, 0x3d9e, 0x48d7, 0x98, 0x10, 0x86, 0x48, 0x48, 0xf0, 0xf4, 0x04}
RIDL!{#[uuid(0x728a21c5, 0x3d9e, 0x48d7, 0x98, 0x10, 0x86, 0x48, 0x48, 0xf0, 0xf4, 0x04)]
class PortableDevice;}
DEFINE_GUID!{CLSID_PortableDeviceManager,
    0x0af10cec, 0x2ecd, 0x4b92, 0x95, 0x81, 0x34, 0xf6, 0xae, 0x06, 0x37, 0xf3}
RIDL!{#[uuid(0x0af10cec, 0x2ecd, 0x4b92, 0x95, 0x81, 0x34, 0xf6, 0xae, 0x06, 0x37, 0xf3)]
class PortableDeviceManager;}
DEFINE_GUID!{CLSID_PortableDeviceService,
    0xef5db4c2, 0x9312, 0x422c, 0x91, 0x52, 0x41, 0x1c, 0xd9, 0xc4, 0xdd, 0x84}
RIDL!{#[uuid(0xef5db4c2, 0x9312, 0x422c, 0x91, 0x52, 0x41, 0x1c, 0xd9, 0xc4, 0xdd, 0x84)]
class PortableDeviceService;}
DEFINE_GUID!{CLSID_PortableDeviceDispatchFactory,
    0x43232233, 0x8338, 0x4658, 0xae, 0x01, 0x0b, 0x4a, 0xe8, 0x30, 0xb6, 0xb0}
RIDL!{#[uuid(0x43232233, 0x8338, 0x4658, 0xae, 0x01, 0x0b, 0x4a, 0xe8, 0x30, 0xb6, 0xb0)]
class PortableDeviceDispatchFactory;}
DEFINE_GUID!{CLSID_PortableDeviceFTM,
    0xf7c0039a, 0x4762, 0x488a, 0xb4, 0xb3, 0x76, 0x0e, 0xf9, 0xa1, 0xba, 0x9b}
RIDL!{#[uuid(0xf7c0039a, 0x4762, 0x488a, 0xb4, 0xb3, 0x76, 0x0e, 0xf9, 0xa1, 0xba, 0x9b)]
class PortableDeviceFTM;}
DEFINE_GUID!{CLSID_PortableDeviceServiceFTM,
    0x1649b154, 0xc794, 0x497a, 0x9b, 0x03, 0xf3, 0xf0, 0x12, 0x13, 0x02, 0xf3}
RIDL!{#[uuid(0x1649b154, 0xc794, 0x497a, 0x9b, 0x03, 0xf3, 0xf0, 0x12, 0x13, 0x02, 0xf3)]
class PortableDeviceServiceFTM;}
DEFINE_GUID!{CLSID_PortableDeviceWebControl,
    0x186dd02c, 0x2dec, 0x41b5, 0xa7, 0xd4, 0xb5, 0x90, 0x56, 0xfa, 0xde, 0x51}
RIDL!{#[uuid(0x186dd02c, 0x2dec, 0x41b5, 0xa7, 0xd4, 0xb5, 0x90, 0x56, 0xfa, 0xde, 0x51)]
class PortableDeviceWebControl;}
