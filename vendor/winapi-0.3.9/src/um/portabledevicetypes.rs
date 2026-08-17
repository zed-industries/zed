// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of PortableDeviceTypes.h
use shared::guiddef::{GUID, REFGUID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, ULONG};
use shared::wtypes::{PROPERTYKEY, VARTYPE};
use um::propidl::PROPVARIANT;
use um::propkeydef::REFPROPERTYKEY;
use um::propsys::IPropertyStore;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LONGLONG, LPCWSTR, LPWSTR, ULONGLONG};
//330
RIDL!{#[uuid(0x6848f6f2, 0x3155, 0x4f86, 0xb6, 0xf5, 0x26, 0x3e, 0xee, 0xab, 0x31, 0x43)]
interface IPortableDeviceValues(IPortableDeviceValuesVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcelt: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        index: DWORD,
        pKey: *mut PROPERTYKEY,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetValue(
        key: REFPROPERTYKEY,
        pValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn GetValue(
        key: REFPROPERTYKEY,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetStringValue(
        key: REFPROPERTYKEY,
        Value: LPCWSTR,
    ) -> HRESULT,
    fn GetStringValue(
        key: REFPROPERTYKEY,
        pValue: *mut LPWSTR,
    ) -> HRESULT,
    fn SetUnsignedIntegerValue(
        key: REFPROPERTYKEY,
        Value: ULONG,
    ) -> HRESULT,
    fn GetUnsignedIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut ULONG,
    ) -> HRESULT,
    fn SetSignedIntegerValue(
        key: REFPROPERTYKEY,
        Value: LONG,
    ) -> HRESULT,
    fn GetSignedIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut LONG,
    ) -> HRESULT,
    fn SetUnsignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        Value: ULONGLONG,
    ) -> HRESULT,
    fn GetUnsignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut ULONGLONG,
    ) -> HRESULT,
    fn SetSignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        Value: LONGLONG,
    ) -> HRESULT,
    fn GetSignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut LONGLONG,
    ) -> HRESULT,
    fn SetFloatValue(
        key: REFPROPERTYKEY,
        Value: FLOAT,
    ) -> HRESULT,
    fn GetFloatValue(
        key: REFPROPERTYKEY,
        pValue: *mut FLOAT,
    ) -> HRESULT,
    fn SetErrorValue(
        key: REFPROPERTYKEY,
        Value: HRESULT,
    ) -> HRESULT,
    fn GetErrorValue(
        key: REFPROPERTYKEY,
        pValue: *mut HRESULT,
    ) -> HRESULT,
    fn SetKeyValue(
        key: REFPROPERTYKEY,
        Value: REFPROPERTYKEY,
    ) -> HRESULT,
    fn GetKeyValue(
        key: REFPROPERTYKEY,
        pValue: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn SetBoolValue(
        key: REFPROPERTYKEY,
        Value: BOOL,
    ) -> HRESULT,
    fn GetBoolValue(
        key: REFPROPERTYKEY,
        pValue: *mut BOOL,
    ) -> HRESULT,
    fn SetIUnknownValue(
        key: REFPROPERTYKEY,
        pValue: *mut IUnknown,
    ) -> HRESULT,
    fn GetIUnknownValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT,
    fn SetGuidValue(
        key: REFPROPERTYKEY,
        Value: REFGUID,
    ) -> HRESULT,
    fn GetGuidValue(
        key: REFPROPERTYKEY,
        pValue: *mut GUID,
    ) -> HRESULT,
    fn SetBufferValue(
        key: REFPROPERTYKEY,
        pValue: *mut BYTE,
        cbValue: DWORD,
    ) -> HRESULT,
    fn GetBufferValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut BYTE,
        pcbValue: *mut DWORD,
    ) -> HRESULT,
    fn SetIPortableDeviceValuesValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn GetIPortableDeviceValuesValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn SetIPortableDevicePropVariantCollectionValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetIPortableDevicePropVariantCollectionValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn SetIPortableDeviceKeyCollectionValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn GetIPortableDeviceKeyCollectionValue(
        key: REFPROPERTYKEY,
         ppValue: *mut *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn SetIPortableDeviceValuesCollectionValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDeviceValuesCollection,
    ) -> HRESULT,
    fn GetIPortableDeviceValuesCollectionValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IPortableDeviceValuesCollection,
    ) -> HRESULT,
    fn RemoveValue(
        key: REFPROPERTYKEY,
    ) -> HRESULT,
    fn CopyValuesFromPropertyStore(
        pStore: *mut IPropertyStore,
    ) -> HRESULT,
    fn CopyValuesToPropertyStore(
        pStore: *mut IPropertyStore,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
}}
RIDL!{#[uuid(0xdada2357, 0xe0ad, 0x492e, 0x98, 0xdb, 0xdd, 0x61, 0xc5, 0x3b, 0xa3, 0x53)]
interface IPortableDeviceKeyCollection(IPortableDeviceKeyCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcElems: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        dwIndex: DWORD,
        pKey: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn Add(
        Key: REFPROPERTYKEY,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn RemoveAt(
        dwIndex: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x89b2e422, 0x4f1b, 0x4316, 0xbc, 0xef, 0xa4, 0x4a, 0xfe, 0xa8, 0x3e, 0xb3)]
interface IPortableDevicePropVariantCollection(IPortableDevicePropVariantCollectionVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcElems: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        dwIndex: DWORD,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn Add(
        pValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn GetType(
        pvt: *mut VARTYPE,
    ) -> HRESULT,
    fn ChangeType(
        vt: VARTYPE,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn RemoveAt(
        dwIndex: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6e3f2d79, 0x4e07, 0x48c4, 0x82, 0x08, 0xd8, 0xc2, 0xe5, 0xaf, 0x4a, 0x99)]
interface IPortableDeviceValuesCollection(IPortableDeviceValuesCollectionVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcElems: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        dwIndex: DWORD,
        ppValues: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn Add(
        pValues: *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn RemoveAt(
        dwIndex: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{LIBID_PortableDeviceTypesLib,
    0x2B00BA2F, 0xE750, 0x4beb, 0x92, 0x35, 0x97, 0x14, 0x2E, 0xDE, 0x1D, 0x3E}
DEFINE_GUID!{CLSID_WpdSerializer,
    0x0b91a74b, 0xad7c, 0x4a9d, 0xb5, 0x63, 0x29, 0xee, 0xf9, 0x16, 0x71, 0x72}
RIDL!{#[uuid(0x0b91a74b, 0xad7c, 0x4a9d, 0xb5, 0x63, 0x29, 0xee, 0xf9, 0x16, 0x71, 0x72)]
class WpdSerializer;}
DEFINE_GUID!{CLSID_PortableDeviceValues,
    0x0c15d503, 0xd017, 0x47ce, 0x90, 0x16, 0x7b, 0x3f, 0x97, 0x87, 0x21, 0xcc}
RIDL!{#[uuid(0x0c15d503, 0xd017, 0x47ce, 0x90, 0x16, 0x7b, 0x3f, 0x97, 0x87, 0x21, 0xcc)]
class PortableDeviceValues;}
DEFINE_GUID!{CLSID_PortableDeviceKeyCollection,
    0xde2d022d, 0x2480, 0x43be, 0x97, 0xf0, 0xd1, 0xfa, 0x2c, 0xf9, 0x8f, 0x4f}
RIDL!{#[uuid(0xde2d022d, 0x2480, 0x43be, 0x97, 0xf0, 0xd1, 0xfa, 0x2c, 0xf9, 0x8f, 0x4f)]
class PortableDeviceKeyCollection;}
DEFINE_GUID!{CLSID_PortableDevicePropVariantCollection,
    0x08a99e2f, 0x6d6d, 0x4b80, 0xaf, 0x5a, 0xba, 0xf2, 0xbc, 0xbe, 0x4c, 0xb9}
RIDL!{#[uuid(0x08a99e2f, 0x6d6d, 0x4b80, 0xaf, 0x5a, 0xba, 0xf2, 0xbc, 0xbe, 0x4c, 0xb9)]
class PortableDevicePropVariantCollection;}
DEFINE_GUID!{CLSID_PortableDeviceValuesCollection,
    0x3882134d, 0x14cf, 0x4220, 0x9c, 0xb4, 0x43, 0x5f, 0x86, 0xd8, 0x3f, 0x60}
RIDL!{#[uuid(0x3882134d, 0x14cf, 0x4220, 0x9c, 0xb4, 0x43, 0x5f, 0x86, 0xd8, 0x3f, 0x60)]
class PortableDeviceValuesCollection;}
