// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{ULONG, USHORT};
use shared::winerror::HRESULT;
use um::bthledef::{
    BLUETOOTH_GATT_EVENT_HANDLE, BTH_LE_GATT_EVENT_TYPE, BTH_LE_GATT_RELIABLE_WRITE_CONTEXT,
    PBTH_LE_GATT_CHARACTERISTIC, PBTH_LE_GATT_CHARACTERISTIC_VALUE, PBTH_LE_GATT_DESCRIPTOR,
    PBTH_LE_GATT_DESCRIPTOR_VALUE, PBTH_LE_GATT_RELIABLE_WRITE_CONTEXT, PBTH_LE_GATT_SERVICE,
    PFNBLUETOOTH_GATT_EVENT_CALLBACK,
};
use um::winnt::{HANDLE, PVOID};
extern "system" {
    pub fn BluetoothGATTGetServices(
        hDevice: HANDLE,
        ServicesBufferCount: USHORT,
        ServicesBuffer: PBTH_LE_GATT_SERVICE,
        ServicesBufferActual: *mut USHORT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTGetIncludedServices(
        hDevice: HANDLE,
        ParentService: PBTH_LE_GATT_SERVICE,
        IncludedServicesBufferCount: USHORT,
        IncludedServicesBuffer: PBTH_LE_GATT_SERVICE,
        IncludedServicesBufferActual: *mut USHORT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTGetCharacteristics(
        hDevice: HANDLE,
        Service: PBTH_LE_GATT_SERVICE,
        CharacteristicsBufferCount: USHORT,
        CharacteristicsBuffer: PBTH_LE_GATT_CHARACTERISTIC,
        CharacteristicsBufferActual: *mut USHORT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTGetDescriptors(
        hDevice: HANDLE,
        Characteristic: PBTH_LE_GATT_CHARACTERISTIC,
        DescriptorsBufferCount: USHORT,
        DescriptorsBuffer: PBTH_LE_GATT_DESCRIPTOR,
        DescriptorsBufferActual: *mut USHORT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTGetCharacteristicValue(
        hDevice: HANDLE,
        Characteristic: PBTH_LE_GATT_CHARACTERISTIC,
        CharacteristicValueDataSize: ULONG,
        CharacteristicValue: PBTH_LE_GATT_CHARACTERISTIC_VALUE,
        CharacteristicValueSizeRequired: *mut USHORT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTGetDescriptorValue(
        hDevice: HANDLE,
        Descriptor: PBTH_LE_GATT_DESCRIPTOR,
        DescriptorValueDataSize: ULONG,
        DescriptorValue: PBTH_LE_GATT_DESCRIPTOR_VALUE,
        DescriptorValueSizeRequired: *mut USHORT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTBeginReliableWrite(
        hDevice: HANDLE,
        ReliableWriteContext: PBTH_LE_GATT_RELIABLE_WRITE_CONTEXT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTSetCharacteristicValue(
        hDevice: HANDLE,
        Characteristic: PBTH_LE_GATT_CHARACTERISTIC,
        CharacteristicValue: PBTH_LE_GATT_CHARACTERISTIC_VALUE,
        ReliableWriteContext: BTH_LE_GATT_RELIABLE_WRITE_CONTEXT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTEndReliableWrite(
        hDevice: HANDLE,
        ReliableWriteContext: BTH_LE_GATT_RELIABLE_WRITE_CONTEXT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTAbortReliableWrite(
        hDevice: HANDLE,
        ReliableWriteContext: BTH_LE_GATT_RELIABLE_WRITE_CONTEXT,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTSetDescriptorValue(
        hDevice: HANDLE,
        Descriptor: PBTH_LE_GATT_DESCRIPTOR,
        DescriptorValue: PBTH_LE_GATT_DESCRIPTOR_VALUE,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTRegisterEvent(
        hService: HANDLE,
        EventType: BTH_LE_GATT_EVENT_TYPE,
        EventParameterIn: PVOID,
        Callback: PFNBLUETOOTH_GATT_EVENT_CALLBACK,
        CallbackContext: PVOID,
        pEventHandle: *mut BLUETOOTH_GATT_EVENT_HANDLE,
        Flags: ULONG,
    ) -> HRESULT;
    pub fn BluetoothGATTUnregisterEvent(
        EventHandle: BLUETOOTH_GATT_EVENT_HANDLE,
        Flags: ULONG,
    ) -> HRESULT;
}
