// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::LPGUID;
use shared::hidpi::PHIDP_PREPARSED_DATA;
use shared::minwindef::{PULONG, ULONG, USHORT};
use um::winnt::{BOOLEAN, HANDLE, PVOID};
STRUCT!{struct HIDD_CONFIGURATION {
    cookie: PVOID,
    size: ULONG,
    RingBufferSize: ULONG,
}}
pub type PHIDD_CONFIGURATION = *mut HIDD_CONFIGURATION;
STRUCT!{struct HIDD_ATTRIBUTES {
    Size: ULONG,
    VendorID: USHORT,
    ProductID: USHORT,
    VersionNumber: USHORT,
}}
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;
extern "system" {
    pub fn HidD_GetAttributes(
        HidDeviceObject: HANDLE,
        Attributes: PHIDD_ATTRIBUTES,
    ) -> BOOLEAN;
    pub fn HidD_GetHidGuid(
        HidGuid: LPGUID,
    );
    pub fn HidD_GetPreparsedData(
        HidDeviceObject: HANDLE,
        PreparsedData: *mut PHIDP_PREPARSED_DATA,
    ) -> BOOLEAN;
    pub fn HidD_FreePreparsedData(
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> BOOLEAN;
    pub fn HidD_FlushQueue(
        HidDeviceObject: HANDLE,
    ) -> BOOLEAN;
    pub fn HidD_GetConfiguration(
        HidDeviceObject: HANDLE,
        Configuration: PHIDD_CONFIGURATION,
        ConfigurationLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_SetConfiguration(
        HidDeviceObject: HANDLE,
        Configuration: PHIDD_CONFIGURATION,
        ConfigurationLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetFeature(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_SetFeature(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetInputReport(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_SetOutputReport(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetNumInputBuffers(
        HidDeviceObject: HANDLE,
        NumberBuffers: PULONG,
    ) -> BOOLEAN;
    pub fn HidD_SetNumInputBuffers(
        HidDeviceObject: HANDLE,
        NumberBuffers: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetPhysicalDescriptor(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetManufacturerString(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetProductString(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetIndexedString(
        HidDeviceObject: HANDLE,
        StringIndex: ULONG,
        Buffer: PVOID,
        BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetSerialNumberString(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetMsGenreDescriptor(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG,
    ) -> BOOLEAN;
}
