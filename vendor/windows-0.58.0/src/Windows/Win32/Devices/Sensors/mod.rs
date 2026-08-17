#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListAllocateBufferAndSerialize(sourcecollection: *const SENSOR_COLLECTION_LIST, ptargetbuffersizeinbytes: *mut u32, ptargetbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListAllocateBufferAndSerialize(sourcecollection : *const SENSOR_COLLECTION_LIST, ptargetbuffersizeinbytes : *mut u32, ptargetbuffer : *mut *mut u8) -> super::super::Foundation:: NTSTATUS);
    CollectionsListAllocateBufferAndSerialize(sourcecollection, ptargetbuffersizeinbytes, ptargetbuffer)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListCopyAndMarshall(target: *mut SENSOR_COLLECTION_LIST, source: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListCopyAndMarshall(target : *mut SENSOR_COLLECTION_LIST, source : *const SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    CollectionsListCopyAndMarshall(target, source)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListDeserializeFromBuffer(sourcebuffer: &[u8], targetcollection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListDeserializeFromBuffer(sourcebuffersizeinbytes : u32, sourcebuffer : *const u8, targetcollection : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    CollectionsListDeserializeFromBuffer(sourcebuffer.len().try_into().unwrap(), core::mem::transmute(sourcebuffer.as_ptr()), targetcollection)
}
#[inline]
pub unsafe fn CollectionsListGetFillableCount(buffersizebytes: u32) -> u32 {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetFillableCount(buffersizebytes : u32) -> u32);
    CollectionsListGetFillableCount(buffersizebytes)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetMarshalledSize(collection : *const SENSOR_COLLECTION_LIST) -> u32);
    CollectionsListGetMarshalledSize(collection)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSizeWithoutSerialization(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetMarshalledSizeWithoutSerialization(collection : *const SENSOR_COLLECTION_LIST) -> u32);
    CollectionsListGetMarshalledSizeWithoutSerialization(collection)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListGetSerializedSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetSerializedSize(collection : *const SENSOR_COLLECTION_LIST) -> u32);
    CollectionsListGetSerializedSize(collection)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListMarshall(target: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListMarshall(target : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    CollectionsListMarshall(target)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListSerializeToBuffer(sourcecollection: *const SENSOR_COLLECTION_LIST, targetbuffer: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListSerializeToBuffer(sourcecollection : *const SENSOR_COLLECTION_LIST, targetbuffersizeinbytes : u32, targetbuffer : *mut u8) -> super::super::Foundation:: NTSTATUS);
    CollectionsListSerializeToBuffer(sourcecollection, targetbuffer.len().try_into().unwrap(), core::mem::transmute(targetbuffer.as_ptr()))
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds: *const SENSOR_COLLECTION_LIST, pcollection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds : *const SENSOR_COLLECTION_LIST, pcollection : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    CollectionsListSortSubscribedActivitiesByConfidence(thresholds, pcollection)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn CollectionsListUpdateMarshalledPointer(collection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn CollectionsListUpdateMarshalledPointer(collection : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    CollectionsListUpdateMarshalledPointer(collection)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn EvaluateActivityThresholds(newsample: *const SENSOR_COLLECTION_LIST, oldsample: *const SENSOR_COLLECTION_LIST, thresholds: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn EvaluateActivityThresholds(newsample : *const SENSOR_COLLECTION_LIST, oldsample : *const SENSOR_COLLECTION_LIST, thresholds : *const SENSOR_COLLECTION_LIST) -> super::super::Foundation:: BOOLEAN);
    EvaluateActivityThresholds(newsample, oldsample, thresholds)
}
#[inline]
pub unsafe fn GetPerformanceTime(timems: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn GetPerformanceTime(timems : *mut u32) -> super::super::Foundation:: NTSTATUS);
    GetPerformanceTime(timems)
}
#[inline]
pub unsafe fn InitPropVariantFromCLSIDArray(members: &[windows_core::GUID]) -> windows_core::Result<windows_core::PROPVARIANT> {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn InitPropVariantFromCLSIDArray(members : *const windows_core::GUID, size : u32, ppropvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    InitPropVariantFromCLSIDArray(core::mem::transmute(members.as_ptr()), members.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromFloat(fltval: f32) -> windows_core::Result<windows_core::PROPVARIANT> {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn InitPropVariantFromFloat(fltval : f32, ppropvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    InitPropVariantFromFloat(fltval, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn IsCollectionListSame(lista: *const SENSOR_COLLECTION_LIST, listb: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn IsCollectionListSame(lista : *const SENSOR_COLLECTION_LIST, listb : *const SENSOR_COLLECTION_LIST) -> super::super::Foundation:: BOOLEAN);
    IsCollectionListSame(lista, listb)
}
#[inline]
pub unsafe fn IsGUIDPresentInList(guidarray: &[windows_core::GUID], guidelem: *const windows_core::GUID) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn IsGUIDPresentInList(guidarray : *const windows_core::GUID, arraylength : u32, guidelem : *const windows_core::GUID) -> super::super::Foundation:: BOOLEAN);
    IsGUIDPresentInList(core::mem::transmute(guidarray.as_ptr()), guidarray.len().try_into().unwrap(), guidelem)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn IsKeyPresentInCollectionList(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn IsKeyPresentInCollectionList(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY) -> super::super::Foundation:: BOOLEAN);
    IsKeyPresentInCollectionList(plist, pkey)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn IsKeyPresentInPropertyList(plist: *const SENSOR_PROPERTY_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn IsKeyPresentInPropertyList(plist : *const SENSOR_PROPERTY_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY) -> super::super::Foundation:: BOOLEAN);
    IsKeyPresentInPropertyList(plist, pkey)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn IsSensorSubscribed(subscriptionlist: *const SENSOR_COLLECTION_LIST, currenttype: windows_core::GUID) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn IsSensorSubscribed(subscriptionlist : *const SENSOR_COLLECTION_LIST, currenttype : windows_core::GUID) -> super::super::Foundation:: BOOLEAN);
    IsSensorSubscribed(subscriptionlist, core::mem::transmute(currenttype))
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetBool(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetBool(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetBool(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetDouble(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f64) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetDouble(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut f64) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetDouble(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetFileTime(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetFileTime(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetFileTime(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetFloat(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut f32) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetFloat(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut f32) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetFloat(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetGuid(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut windows_core::GUID) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetGuid(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut windows_core::GUID) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetGuid(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt32(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i32) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetInt32(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut i32) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetInt32(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetInt64(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut i64) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetInt64(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetNthInt64(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, occurrence : u32, pretvalue : *mut i64) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetNthInt64(plist, pkey, occurrence, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetNthUlong(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, occurrence : u32, pretvalue : *mut u32) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetNthUlong(plist, pkey, occurrence, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, occurrence: u32, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetNthUshort(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, occurrence : u32, pretvalue : *mut u16) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetNthUshort(plist, pkey, occurrence, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetPropVariant<P0>(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: P0, pvalue: *mut windows_core::PROPVARIANT) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetPropVariant(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, typecheck : super::super::Foundation:: BOOLEAN, pvalue : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetPropVariant(plist, pkey, typecheck.param().abi(), core::mem::transmute(pvalue))
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetUlong(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut u32) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetUlong(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeyGetUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetUshort(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pretvalue : *mut u16) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeyGetUshort(plist, pkey, pretvalue)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropKeyFindKeySetPropVariant<P0>(plist: *mut SENSOR_COLLECTION_LIST, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, typecheck: P0, pvalue: *const windows_core::PROPVARIANT) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeySetPropVariant(plist : *mut SENSOR_COLLECTION_LIST, pkey : *const super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, typecheck : super::super::Foundation:: BOOLEAN, pvalue : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> super::super::Foundation:: NTSTATUS);
    PropKeyFindKeySetPropVariant(plist, pkey, typecheck.param().abi(), core::mem::transmute(pvalue))
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn PropVariantGetInformation(propvariantvalue: *const windows_core::PROPVARIANT, propvariantoffset: Option<*mut u32>, propvariantsize: Option<*mut u32>, propvariantpointer: Option<*mut *mut core::ffi::c_void>, remappedtype: Option<*mut super::Properties::DEVPROPTYPE>) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropVariantGetInformation(propvariantvalue : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >, propvariantoffset : *mut u32, propvariantsize : *mut u32, propvariantpointer : *mut *mut core::ffi::c_void, remappedtype : *mut super::Properties:: DEVPROPTYPE) -> super::super::Foundation:: NTSTATUS);
    PropVariantGetInformation(core::mem::transmute(propvariantvalue), core::mem::transmute(propvariantoffset.unwrap_or(std::ptr::null_mut())), core::mem::transmute(propvariantsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(propvariantpointer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(remappedtype.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn PropertiesListCopy(target: *mut SENSOR_PROPERTY_LIST, source: *const SENSOR_PROPERTY_LIST) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropertiesListCopy(target : *mut SENSOR_PROPERTY_LIST, source : *const SENSOR_PROPERTY_LIST) -> super::super::Foundation:: NTSTATUS);
    PropertiesListCopy(target, source)
}
#[inline]
pub unsafe fn PropertiesListGetFillableCount(buffersizebytes: u32) -> u32 {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn PropertiesListGetFillableCount(buffersizebytes : u32) -> u32);
    PropertiesListGetFillableCount(buffersizebytes)
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[inline]
pub unsafe fn SensorCollectionGetAt(index: u32, psensorslist: *const SENSOR_COLLECTION_LIST, pkey: Option<*mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pvalue: Option<*mut windows_core::PROPVARIANT>) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn SensorCollectionGetAt(index : u32, psensorslist : *const SENSOR_COLLECTION_LIST, pkey : *mut super::super::UI::Shell::PropertiesSystem:: PROPERTYKEY, pvalue : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> super::super::Foundation:: NTSTATUS);
    SensorCollectionGetAt(index, psensorslist, core::mem::transmute(pkey.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SerializationBufferAllocate(sizeinbytes: u32, pbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn SerializationBufferAllocate(sizeinbytes : u32, pbuffer : *mut *mut u8) -> super::super::Foundation:: NTSTATUS);
    SerializationBufferAllocate(sizeinbytes, pbuffer)
}
#[inline]
pub unsafe fn SerializationBufferFree(buffer: Option<*const u8>) {
    windows_targets::link!("sensorsutilsv2.dll" "system" fn SerializationBufferFree(buffer : *const u8));
    SerializationBufferFree(core::mem::transmute(buffer.unwrap_or(std::ptr::null())))
}
windows_core::imp::define_interface!(ILocationPermissions, ILocationPermissions_Vtbl, 0xd5fb0a7f_e74e_44f5_8e02_4806863a274f);
impl core::ops::Deref for ILocationPermissions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILocationPermissions, windows_core::IUnknown);
impl ILocationPermissions {
    pub unsafe fn GetGlobalLocationPermission(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlobalLocationPermission)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CheckLocationCapability(&self, dwclientthreadid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckLocationCapability)(windows_core::Interface::as_raw(self), dwclientthreadid).ok()
    }
}
#[repr(C)]
pub struct ILocationPermissions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGlobalLocationPermission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CheckLocationCapability: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensor, ISensor_Vtbl, 0x5fa08f80_2657_458e_af75_46f73fa6ac5c);
impl core::ops::Deref for ISensor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISensor, windows_core::IUnknown);
impl ISensor {
    pub unsafe fn GetID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetProperties<P0>(&self, pkeys: P0) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::PortableDevices::IPortableDeviceKeyCollection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetSupportedDataFields(&self) -> windows_core::Result<super::PortableDevices::IPortableDeviceKeyCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedDataFields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn SetProperties<P0>(&self, pproperties: P0) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::PortableDevices::IPortableDeviceValues>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), pproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SupportsDataField(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportsDataField)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<SensorState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetData(&self) -> windows_core::Result<ISensorDataReport> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SupportsEvent(&self, eventguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportsEvent)(windows_core::Interface::as_raw(self), eventguid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetEventInterest(&self, ppvalues: *mut *mut windows_core::GUID, pcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetEventInterest)(windows_core::Interface::as_raw(self), ppvalues, pcount).ok()
    }
    pub unsafe fn SetEventInterest(&self, pvalues: Option<&[windows_core::GUID]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventInterest)(windows_core::Interface::as_raw(self), core::mem::transmute(pvalues.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pvalues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetEventSink<P0>(&self, pevents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensorEvents>,
    {
        (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), pevents.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISensor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetProperties: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetSupportedDataFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetSupportedDataFields: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    SetProperties: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SupportsDataField: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SupportsDataField: usize,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorState) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportsEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetEventInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorCollection, ISensorCollection_Vtbl, 0x23571e11_e545_4dd8_a337_b89bf44b10df);
impl core::ops::Deref for ISensorCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISensorCollection, windows_core::IUnknown);
impl ISensorCollection {
    pub unsafe fn GetAt(&self, ulindex: u32) -> windows_core::Result<ISensor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ulindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Add<P0>(&self, psensor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), psensor.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, psensor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), psensor.param().abi()).ok()
    }
    pub unsafe fn RemoveByID(&self, sensorid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveByID)(windows_core::Interface::as_raw(self), sensorid).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISensorCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorDataReport, ISensorDataReport_Vtbl, 0x0ab9df9b_c4b5_4796_8898_0470706a2e1d);
impl core::ops::Deref for ISensorDataReport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISensorDataReport, windows_core::IUnknown);
impl ISensorDataReport {
    pub unsafe fn GetTimestamp(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSensorValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSensorValue)(windows_core::Interface::as_raw(self), pkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetSensorValues<P0>(&self, pkeys: P0) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::PortableDevices::IPortableDeviceKeyCollection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSensorValues)(windows_core::Interface::as_raw(self), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISensorDataReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSensorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSensorValue: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetSensorValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetSensorValues: usize,
}
windows_core::imp::define_interface!(ISensorEvents, ISensorEvents_Vtbl, 0x5d8dcc91_4641_47e7_b7c3_b74f48a6c391);
impl core::ops::Deref for ISensorEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISensorEvents, windows_core::IUnknown);
impl ISensorEvents {
    pub unsafe fn OnStateChanged<P0>(&self, psensor: P0, state: SensorState) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), psensor.param().abi(), state).ok()
    }
    pub unsafe fn OnDataUpdated<P0, P1>(&self, psensor: P0, pnewdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
        P1: windows_core::Param<ISensorDataReport>,
    {
        (windows_core::Interface::vtable(self).OnDataUpdated)(windows_core::Interface::as_raw(self), psensor.param().abi(), pnewdata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn OnEvent<P0, P1>(&self, psensor: P0, eventid: *const windows_core::GUID, peventdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
        P1: windows_core::Param<super::PortableDevices::IPortableDeviceValues>,
    {
        (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), psensor.param().abi(), eventid, peventdata.param().abi()).ok()
    }
    pub unsafe fn OnLeave(&self, id: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLeave)(windows_core::Interface::as_raw(self), id).ok()
    }
}
#[repr(C)]
pub struct ISensorEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SensorState) -> windows_core::HRESULT,
    pub OnDataUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    OnEvent: usize,
    pub OnLeave: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorManager, ISensorManager_Vtbl, 0xbd77db67_45a8_42dc_8d00_6dcf15f8377a);
impl core::ops::Deref for ISensorManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISensorManager, windows_core::IUnknown);
impl ISensorManager {
    pub unsafe fn GetSensorsByCategory(&self, sensorcategory: *const windows_core::GUID) -> windows_core::Result<ISensorCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSensorsByCategory)(windows_core::Interface::as_raw(self), sensorcategory, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSensorsByType(&self, sensortype: *const windows_core::GUID) -> windows_core::Result<ISensorCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSensorsByType)(windows_core::Interface::as_raw(self), sensortype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSensorByID(&self, sensorid: *const windows_core::GUID) -> windows_core::Result<ISensor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSensorByID)(windows_core::Interface::as_raw(self), sensorid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventSink<P0>(&self, pevents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensorManagerEvents>,
    {
        (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), pevents.param().abi()).ok()
    }
    pub unsafe fn RequestPermissions<P0, P1, P2>(&self, hparent: P0, psensors: P1, fmodal: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<ISensorCollection>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hparent.param().abi(), psensors.param().abi(), fmodal.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISensorManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSensorsByCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSensorsByType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSensorByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorManagerEvents, ISensorManagerEvents_Vtbl, 0x9b3b0b86_266a_4aad_b21f_fde5501001b7);
impl core::ops::Deref for ISensorManagerEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISensorManagerEvents, windows_core::IUnknown);
impl ISensorManagerEvents {
    pub unsafe fn OnSensorEnter<P0>(&self, psensor: P0, state: SensorState) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        (windows_core::Interface::vtable(self).OnSensorEnter)(windows_core::Interface::as_raw(self), psensor.param().abi(), state).ok()
    }
}
#[repr(C)]
pub struct ISensorManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSensorEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SensorState) -> windows_core::HRESULT,
}
pub const AXIS_MAX: AXIS = AXIS(3i32);
pub const AXIS_X: AXIS = AXIS(0i32);
pub const AXIS_Y: AXIS = AXIS(1i32);
pub const AXIS_Z: AXIS = AXIS(2i32);
pub const ActivityStateCount: ACTIVITY_STATE_COUNT = ACTIVITY_STATE_COUNT(8i32);
pub const ActivityState_Biking: ACTIVITY_STATE = ACTIVITY_STATE(64i32);
pub const ActivityState_Fidgeting: ACTIVITY_STATE = ACTIVITY_STATE(4i32);
pub const ActivityState_Force_Dword: ACTIVITY_STATE = ACTIVITY_STATE(-1i32);
pub const ActivityState_Idle: ACTIVITY_STATE = ACTIVITY_STATE(128i32);
pub const ActivityState_InVehicle: ACTIVITY_STATE = ACTIVITY_STATE(32i32);
pub const ActivityState_Max: ACTIVITY_STATE = ACTIVITY_STATE(256i32);
pub const ActivityState_Running: ACTIVITY_STATE = ACTIVITY_STATE(16i32);
pub const ActivityState_Stationary: ACTIVITY_STATE = ACTIVITY_STATE(2i32);
pub const ActivityState_Unknown: ACTIVITY_STATE = ACTIVITY_STATE(1i32);
pub const ActivityState_Walking: ACTIVITY_STATE = ACTIVITY_STATE(8i32);
pub const ElevationChangeMode_Elevator: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(1i32);
pub const ElevationChangeMode_Force_Dword: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(-1i32);
pub const ElevationChangeMode_Max: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(3i32);
pub const ElevationChangeMode_Stepping: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(2i32);
pub const ElevationChangeMode_Unknown: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(0i32);
pub const GNSS_CLEAR_ALL_ASSISTANCE_DATA: u32 = 1u32;
pub const GUID_DEVINTERFACE_SENSOR: windows_core::GUID = windows_core::GUID::from_u128(0xba1bb692_9b7a_4833_9a1e_525ed134e7e2);
pub const GUID_SensorCategory_All: windows_core::GUID = windows_core::GUID::from_u128(0xc317c286_c468_4288_9975_d4c4587c442c);
pub const GUID_SensorCategory_Biometric: windows_core::GUID = windows_core::GUID::from_u128(0xca19690f_a2c7_477d_a99e_99ec6e2b5648);
pub const GUID_SensorCategory_Electrical: windows_core::GUID = windows_core::GUID::from_u128(0xfb73fcd8_fc4a_483c_ac58_27b691c6beff);
pub const GUID_SensorCategory_Environmental: windows_core::GUID = windows_core::GUID::from_u128(0x323439aa_7f66_492b_ba0c_73e9aa0a65d5);
pub const GUID_SensorCategory_Light: windows_core::GUID = windows_core::GUID::from_u128(0x17a665c0_9063_4216_b202_5c7a255e18ce);
pub const GUID_SensorCategory_Location: windows_core::GUID = windows_core::GUID::from_u128(0xbfa794e4_f964_4fdb_90f6_51056bfe4b44);
pub const GUID_SensorCategory_Mechanical: windows_core::GUID = windows_core::GUID::from_u128(0x8d131d68_8ef7_4656_80b5_cccbd93791c5);
pub const GUID_SensorCategory_Motion: windows_core::GUID = windows_core::GUID::from_u128(0xcd09daf1_3b2e_4c3d_b598_b5e5ff93fd46);
pub const GUID_SensorCategory_Orientation: windows_core::GUID = windows_core::GUID::from_u128(0x9e6c04b6_96fe_4954_b726_68682a473f69);
pub const GUID_SensorCategory_Other: windows_core::GUID = windows_core::GUID::from_u128(0x2c90e7a9_f4c9_4fa2_af37_56d471fe5a3d);
pub const GUID_SensorCategory_PersonalActivity: windows_core::GUID = windows_core::GUID::from_u128(0xf1609081_1e12_412b_a14d_cbb0e95bd2e5);
pub const GUID_SensorCategory_Scanner: windows_core::GUID = windows_core::GUID::from_u128(0xb000e77e_f5b5_420f_815d_0270a726f270);
pub const GUID_SensorCategory_Unsupported: windows_core::GUID = windows_core::GUID::from_u128(0x2beae7fa_19b0_48c5_a1f6_b5480dc206b0);
pub const GUID_SensorType_Accelerometer3D: windows_core::GUID = windows_core::GUID::from_u128(0xc2fb0f5f_e2d2_4c78_bcd0_352a9582819d);
pub const GUID_SensorType_ActivityDetection: windows_core::GUID = windows_core::GUID::from_u128(0x9d9e0118_1807_4f2e_96e4_2ce57142e196);
pub const GUID_SensorType_AmbientLight: windows_core::GUID = windows_core::GUID::from_u128(0x97f115c8_599a_4153_8894_d2d12899918a);
pub const GUID_SensorType_Barometer: windows_core::GUID = windows_core::GUID::from_u128(0x0e903829_ff8a_4a93_97df_3dcbde402288);
pub const GUID_SensorType_Custom: windows_core::GUID = windows_core::GUID::from_u128(0xe83af229_8640_4d18_a213_e22675ebb2c3);
pub const GUID_SensorType_FloorElevation: windows_core::GUID = windows_core::GUID::from_u128(0xade4987f_7ac4_4dfa_9722_0a027181c747);
pub const GUID_SensorType_GeomagneticOrientation: windows_core::GUID = windows_core::GUID::from_u128(0xe77195f8_2d1f_4823_971b_1c4467556c9d);
pub const GUID_SensorType_GravityVector: windows_core::GUID = windows_core::GUID::from_u128(0x03b52c73_bb76_463f_9524_38de76eb700b);
pub const GUID_SensorType_Gyrometer3D: windows_core::GUID = windows_core::GUID::from_u128(0x09485f5a_759e_42c2_bd4b_a349b75c8643);
pub const GUID_SensorType_HingeAngle: windows_core::GUID = windows_core::GUID::from_u128(0x82358065_f4c4_4da1_b272_13c23332a207);
pub const GUID_SensorType_Humidity: windows_core::GUID = windows_core::GUID::from_u128(0x5c72bf67_bd7e_4257_990b_98a3ba3b400a);
pub const GUID_SensorType_LinearAccelerometer: windows_core::GUID = windows_core::GUID::from_u128(0x038b0283_97b4_41c8_bc24_5ff1aa48fec7);
pub const GUID_SensorType_Magnetometer3D: windows_core::GUID = windows_core::GUID::from_u128(0x55e5effb_15c7_40df_8698_a84b7c863c53);
pub const GUID_SensorType_Orientation: windows_core::GUID = windows_core::GUID::from_u128(0xcdb5d8f7_3cfd_41c8_8542_cce622cf5d6e);
pub const GUID_SensorType_Pedometer: windows_core::GUID = windows_core::GUID::from_u128(0xb19f89af_e3eb_444b_8dea_202575a71599);
pub const GUID_SensorType_Proximity: windows_core::GUID = windows_core::GUID::from_u128(0x5220dae9_3179_4430_9f90_06266d2a34de);
pub const GUID_SensorType_RelativeOrientation: windows_core::GUID = windows_core::GUID::from_u128(0x40993b51_4706_44dc_98d5_c920c037ffab);
pub const GUID_SensorType_SimpleDeviceOrientation: windows_core::GUID = windows_core::GUID::from_u128(0x86a19291_0482_402c_bf4c_addac52b1c39);
pub const GUID_SensorType_Temperature: windows_core::GUID = windows_core::GUID::from_u128(0x04fd0ec4_d5da_45fa_95a9_5db38ee19306);
pub const HumanPresenceDetectionTypeCount: HUMAN_PRESENCE_DETECTION_TYPE_COUNT = HUMAN_PRESENCE_DETECTION_TYPE_COUNT(4i32);
pub const HumanPresenceDetectionType_AudioBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(8i32);
pub const HumanPresenceDetectionType_FacialBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(4i32);
pub const HumanPresenceDetectionType_Force_Dword: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(-1i32);
pub const HumanPresenceDetectionType_Undefined: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(0i32);
pub const HumanPresenceDetectionType_VendorDefinedBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(2i32);
pub const HumanPresenceDetectionType_VendorDefinedNonBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(1i32);
pub const LOCATION_DESIRED_ACCURACY_DEFAULT: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(0i32);
pub const LOCATION_DESIRED_ACCURACY_HIGH: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(1i32);
pub const LOCATION_POSITION_SOURCE_CELLULAR: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(0i32);
pub const LOCATION_POSITION_SOURCE_IPADDRESS: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(3i32);
pub const LOCATION_POSITION_SOURCE_SATELLITE: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(1i32);
pub const LOCATION_POSITION_SOURCE_UNKNOWN: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(4i32);
pub const LOCATION_POSITION_SOURCE_WIFI: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(2i32);
pub const MAGNETOMETER_ACCURACY_APPROXIMATE: MagnetometerAccuracy = MagnetometerAccuracy(2i32);
pub const MAGNETOMETER_ACCURACY_HIGH: MagnetometerAccuracy = MagnetometerAccuracy(3i32);
pub const MAGNETOMETER_ACCURACY_UNKNOWN: MagnetometerAccuracy = MagnetometerAccuracy(0i32);
pub const MAGNETOMETER_ACCURACY_UNRELIABLE: MagnetometerAccuracy = MagnetometerAccuracy(1i32);
pub const MagnetometerAccuracy_Approximate: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(2i32);
pub const MagnetometerAccuracy_High: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(3i32);
pub const MagnetometerAccuracy_Unknown: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(0i32);
pub const MagnetometerAccuracy_Unreliable: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(1i32);
pub const PedometerStepTypeCount: PEDOMETER_STEP_TYPE_COUNT = PEDOMETER_STEP_TYPE_COUNT(3i32);
pub const PedometerStepType_Force_Dword: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(-1i32);
pub const PedometerStepType_Max: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(8i32);
pub const PedometerStepType_Running: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(4i32);
pub const PedometerStepType_Unknown: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(1i32);
pub const PedometerStepType_Walking: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(2i32);
pub const ProximityType_Force_Dword: PROXIMITY_TYPE = PROXIMITY_TYPE(-1i32);
pub const ProximityType_HumanProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(1i32);
pub const ProximityType_ObjectProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(0i32);
pub const Proximity_Sensor_Human_Engagement_Capable: PROXIMITY_SENSOR_CAPABILITIES = PROXIMITY_SENSOR_CAPABILITIES(2i32);
pub const Proximity_Sensor_Human_Presence_Capable: PROXIMITY_SENSOR_CAPABILITIES = PROXIMITY_SENSOR_CAPABILITIES(1i32);
pub const Proximity_Sensor_Supported_Capabilities: PROXIMITY_SENSOR_CAPABILITIES = PROXIMITY_SENSOR_CAPABILITIES(3i32);
pub const SENSOR_CATEGORY_ALL: windows_core::GUID = windows_core::GUID::from_u128(0xc317c286_c468_4288_9975_d4c4587c442c);
pub const SENSOR_CATEGORY_BIOMETRIC: windows_core::GUID = windows_core::GUID::from_u128(0xca19690f_a2c7_477d_a99e_99ec6e2b5648);
pub const SENSOR_CATEGORY_ELECTRICAL: windows_core::GUID = windows_core::GUID::from_u128(0xfb73fcd8_fc4a_483c_ac58_27b691c6beff);
pub const SENSOR_CATEGORY_ENVIRONMENTAL: windows_core::GUID = windows_core::GUID::from_u128(0x323439aa_7f66_492b_ba0c_73e9aa0a65d5);
pub const SENSOR_CATEGORY_LIGHT: windows_core::GUID = windows_core::GUID::from_u128(0x17a665c0_9063_4216_b202_5c7a255e18ce);
pub const SENSOR_CATEGORY_LOCATION: windows_core::GUID = windows_core::GUID::from_u128(0xbfa794e4_f964_4fdb_90f6_51056bfe4b44);
pub const SENSOR_CATEGORY_MECHANICAL: windows_core::GUID = windows_core::GUID::from_u128(0x8d131d68_8ef7_4656_80b5_cccbd93791c5);
pub const SENSOR_CATEGORY_MOTION: windows_core::GUID = windows_core::GUID::from_u128(0xcd09daf1_3b2e_4c3d_b598_b5e5ff93fd46);
pub const SENSOR_CATEGORY_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0x9e6c04b6_96fe_4954_b726_68682a473f69);
pub const SENSOR_CATEGORY_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x2c90e7a9_f4c9_4fa2_af37_56d471fe5a3d);
pub const SENSOR_CATEGORY_SCANNER: windows_core::GUID = windows_core::GUID::from_u128(0xb000e77e_f5b5_420f_815d_0270a726f270);
pub const SENSOR_CATEGORY_UNSUPPORTED: windows_core::GUID = windows_core::GUID::from_u128(0x2beae7fa_19b0_48c5_a1f6_b5480dc206b0);
pub const SENSOR_CONNECTION_TYPE_PC_ATTACHED: SensorConnectionType = SensorConnectionType(1i32);
pub const SENSOR_CONNECTION_TYPE_PC_EXTERNAL: SensorConnectionType = SensorConnectionType(2i32);
pub const SENSOR_CONNECTION_TYPE_PC_INTEGRATED: SensorConnectionType = SensorConnectionType(0i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ABSOLUTE_PRESSURE_PASCAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ACCELERATION_X_G: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ACCELERATION_Y_G: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ACCELERATION_Z_G: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ADDRESS1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 23 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ADDRESS2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 24 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_ANTENNA_SEALEVEL_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 36 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_ELLIPSOID_ERROR_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 29 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_ELLIPSOID_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_ERROR_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 30 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_X_DEGREES_PER_SECOND_SQUARED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_Y_DEGREES_PER_SECOND_SQUARED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_Z_DEGREES_PER_SECOND_SQUARED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_X_DEGREES_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_Y_DEGREES_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_Z_DEGREES_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ATMOSPHERIC_PRESSURE_BAR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 4 };
pub const SENSOR_DATA_TYPE_BIOMETRIC_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_BOOLEAN_SWITCH_ARRAY_STATES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_BOOLEAN_SWITCH_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CAPACITANCE_FARAD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 25 };
pub const SENSOR_DATA_TYPE_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdb5e0cf2_cf1f_4c18_b46c_d86011d62150);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_COUNTRY_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 28 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CURRENT_AMPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_BOOLEAN_ARRAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 6 };
pub const SENSOR_DATA_TYPE_CUSTOM_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE10: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE11: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE12: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE13: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE14: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 20 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE15: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 21 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE16: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 22 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE17: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 23 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE18: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 24 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE19: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 25 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE20: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 26 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE21: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 27 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE22: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 28 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE23: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 29 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE24: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 30 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE25: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 31 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE26: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE27: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 33 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE28: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 34 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE4: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE5: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE6: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE7: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE8: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE9: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DGPS_DATA_AGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 35 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DIFFERENTIAL_REFERENCE_STATION_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 37 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DISTANCE_X_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DISTANCE_Y_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_DISTANCE_Z_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ELECTRICAL_FREQUENCY_HERTZ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 9 };
pub const SENSOR_DATA_TYPE_ELECTRICAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ELECTRICAL_PERCENT_OF_RANGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ELECTRICAL_POWER_WATTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 7 };
pub const SENSOR_DATA_TYPE_ENVIRONMENTAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ERROR_RADIUS_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 22 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_FIX_QUALITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_FIX_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_FORCE_NEWTONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GAUGE_PRESSURE_PASCAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GEOIDAL_SEPARATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 34 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GPS_OPERATION_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GPS_SELECTION_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 31 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_GPS_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 33 };
pub const SENSOR_DATA_TYPE_GUID_MECHANICAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_HORIZONAL_DILUTION_OF_PRECISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_HUMAN_PRESENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_HUMAN_PROXIMITY_METERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_INDUCTANCE_HENRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LATITUDE_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LIGHT_CHROMACITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 4 };
pub const SENSOR_DATA_TYPE_LIGHT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LIGHT_TEMPERATURE_KELVIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 3 };
pub const SENSOR_DATA_TYPE_LOCATION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LOCATION_SOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 40 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_LONGITUDE_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_X_MILLIGAUSS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_Y_MILLIGAUSS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 20 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_Z_MILLIGAUSS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 21 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_COMPENSATED_MAGNETIC_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_COMPENSATED_TRUE_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_MAGNETIC_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_TRUE_NORTH_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_X_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_Y_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_Z_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETIC_VARIATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MAGNETOMETER_ACCURACY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 22 };
pub const SENSOR_DATA_TYPE_MOTION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MOTION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_MULTIVALUE_SWITCH_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_NMEA_SENTENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 38 };
pub const SENSOR_DATA_TYPE_ORIENTATION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_POSITION_DILUTION_OF_PRECISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_POSTALCODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 27 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_QUADRANT_ANGLE_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_QUATERNION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_RELATIVE_HUMIDITY_PERCENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_RESISTANCE_OHMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_RFID_TAG_40_BIT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd7a59a3c_3421_44ab_8d3a_9de8ab6c4cae), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_ROTATION_MATRIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_AZIMUTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 20 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_ELEVATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 39 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_PRNS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_STN_RATIO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 21 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_USED_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_USED_PRNS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SATELLITES_USED_PRNS_AND_CONSTELLATIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 41 };
pub const SENSOR_DATA_TYPE_SCANNER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd7a59a3c_3421_44ab_8d3a_9de8ab6c4cae);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SIMPLE_DEVICE_ORIENTATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SPEED_KNOTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_SPEED_METERS_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_STATE_PROVINCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 26 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_STRAIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TEMPERATURE_CELSIUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TILT_X_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TILT_Y_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TILT_Z_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TIMESTAMP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdb5e0cf2_cf1f_4c18_b46c_d86011d62150), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TOUCH_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_TRUE_HEADING_DEGREES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_VERTICAL_DILUTION_OF_PRECISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_VOLTAGE_VOLTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_WEIGHT_KILOGRAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_WIND_DIRECTION_DEGREES_ANTICLOCKWISE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_DATA_TYPE_WIND_SPEED_METERS_PER_SECOND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 6 };
pub const SENSOR_ERROR_PARAMETER_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x77112bcd_fce1_4f43_b8b8_a88256adb4b3);
pub const SENSOR_EVENT_ACCELEROMETER_SHAKE: windows_core::GUID = windows_core::GUID::from_u128(0x825f5a94_0f48_4396_9ca0_6ecb5c99d915);
pub const SENSOR_EVENT_DATA_UPDATED: windows_core::GUID = windows_core::GUID::from_u128(0x2ed0f2a4_0087_41d3_87db_6773370b3c88);
pub const SENSOR_EVENT_PARAMETER_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_EVENT_PARAMETER_EVENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_EVENT_PARAMETER_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28), pid: 3 };
pub const SENSOR_EVENT_PROPERTY_CHANGED: windows_core::GUID = windows_core::GUID::from_u128(0x2358f099_84c9_4d3d_90df_c2421e2b2045);
pub const SENSOR_EVENT_STATE_CHANGED: windows_core::GUID = windows_core::GUID::from_u128(0xbfd96016_6bd7_4560_ad34_f2f6607e8f81);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_ACCURACY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CHANGE_SENSITIVITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CLEAR_ASSISTANCE_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34), pid: 2 };
pub const SENSOR_PROPERTY_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CONNECTION_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_CURRENT_REPORT_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_DEVICE_PATH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_HID_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 22 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_LIGHT_RESPONSE_CURVE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 16 };
pub const SENSOR_PROPERTY_LIST_HEADER_SIZE: u32 = 8u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_LOCATION_DESIRED_ACCURACY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_MIN_REPORT_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RADIO_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 23 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RADIO_STATE_PREVIOUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 24 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RANGE_MAXIMUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 21 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RANGE_MINIMUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 20 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 3 };
pub const SENSOR_PROPERTY_TEST_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_TURN_ON_OFF_NMEA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SENSOR_PROPERTY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 2 };
pub const SENSOR_STATE_ACCESS_DENIED: SensorState = SensorState(4i32);
pub const SENSOR_STATE_ERROR: SensorState = SensorState(5i32);
pub const SENSOR_STATE_INITIALIZING: SensorState = SensorState(3i32);
pub const SENSOR_STATE_MAX: SensorState = SensorState(5i32);
pub const SENSOR_STATE_MIN: SensorState = SensorState(0i32);
pub const SENSOR_STATE_NOT_AVAILABLE: SensorState = SensorState(1i32);
pub const SENSOR_STATE_NO_DATA: SensorState = SensorState(2i32);
pub const SENSOR_STATE_READY: SensorState = SensorState(0i32);
pub const SENSOR_TYPE_ACCELEROMETER_1D: windows_core::GUID = windows_core::GUID::from_u128(0xc04d2387_7340_4cc2_991e_3b18cb8ef2f4);
pub const SENSOR_TYPE_ACCELEROMETER_2D: windows_core::GUID = windows_core::GUID::from_u128(0xb2c517a8_f6b5_4ba6_a423_5df560b4cc07);
pub const SENSOR_TYPE_ACCELEROMETER_3D: windows_core::GUID = windows_core::GUID::from_u128(0xc2fb0f5f_e2d2_4c78_bcd0_352a9582819d);
pub const SENSOR_TYPE_AGGREGATED_DEVICE_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0xcdb5d8f7_3cfd_41c8_8542_cce622cf5d6e);
pub const SENSOR_TYPE_AGGREGATED_QUADRANT_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0x9f81f1af_c4ab_4307_9904_c828bfb90829);
pub const SENSOR_TYPE_AGGREGATED_SIMPLE_DEVICE_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0x86a19291_0482_402c_bf4c_addac52b1c39);
pub const SENSOR_TYPE_AMBIENT_LIGHT: windows_core::GUID = windows_core::GUID::from_u128(0x97f115c8_599a_4153_8894_d2d12899918a);
pub const SENSOR_TYPE_BARCODE_SCANNER: windows_core::GUID = windows_core::GUID::from_u128(0x990b3d8f_85bb_45ff_914d_998c04f372df);
pub const SENSOR_TYPE_BOOLEAN_SWITCH: windows_core::GUID = windows_core::GUID::from_u128(0x9c7e371f_1041_460b_8d5c_71e4752e350c);
pub const SENSOR_TYPE_BOOLEAN_SWITCH_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0x545c8ba5_b143_4545_868f_ca7fd986b4f6);
pub const SENSOR_TYPE_CAPACITANCE: windows_core::GUID = windows_core::GUID::from_u128(0xca2ffb1c_2317_49c0_a0b4_b63ce63461a0);
pub const SENSOR_TYPE_COMPASS_1D: windows_core::GUID = windows_core::GUID::from_u128(0xa415f6c5_cb50_49d0_8e62_a8270bd7a26c);
pub const SENSOR_TYPE_COMPASS_2D: windows_core::GUID = windows_core::GUID::from_u128(0x15655cc0_997a_4d30_84db_57caba3648bb);
pub const SENSOR_TYPE_COMPASS_3D: windows_core::GUID = windows_core::GUID::from_u128(0x76b5ce0d_17dd_414d_93a1_e127f40bdf6e);
pub const SENSOR_TYPE_CURRENT: windows_core::GUID = windows_core::GUID::from_u128(0x5adc9fce_15a0_4bbe_a1ad_2d38a9ae831c);
pub const SENSOR_TYPE_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0xe83af229_8640_4d18_a213_e22675ebb2c3);
pub const SENSOR_TYPE_DISTANCE_1D: windows_core::GUID = windows_core::GUID::from_u128(0x5f14ab2f_1407_4306_a93f_b1dbabe4f9c0);
pub const SENSOR_TYPE_DISTANCE_2D: windows_core::GUID = windows_core::GUID::from_u128(0x5cf9a46c_a9a2_4e55_b6a1_a04aafa95a92);
pub const SENSOR_TYPE_DISTANCE_3D: windows_core::GUID = windows_core::GUID::from_u128(0xa20cae31_0e25_4772_9fe5_96608a1354b2);
pub const SENSOR_TYPE_ELECTRICAL_POWER: windows_core::GUID = windows_core::GUID::from_u128(0x212f10f5_14ab_4376_9a43_a7794098c2fe);
pub const SENSOR_TYPE_ENVIRONMENTAL_ATMOSPHERIC_PRESSURE: windows_core::GUID = windows_core::GUID::from_u128(0x0e903829_ff8a_4a93_97df_3dcbde402288);
pub const SENSOR_TYPE_ENVIRONMENTAL_HUMIDITY: windows_core::GUID = windows_core::GUID::from_u128(0x5c72bf67_bd7e_4257_990b_98a3ba3b400a);
pub const SENSOR_TYPE_ENVIRONMENTAL_TEMPERATURE: windows_core::GUID = windows_core::GUID::from_u128(0x04fd0ec4_d5da_45fa_95a9_5db38ee19306);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_DIRECTION: windows_core::GUID = windows_core::GUID::from_u128(0x9ef57a35_9306_434d_af09_37fa5a9c00bd);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_SPEED: windows_core::GUID = windows_core::GUID::from_u128(0xdd50607b_a45f_42cd_8efd_ec61761c4226);
pub const SENSOR_TYPE_FORCE: windows_core::GUID = windows_core::GUID::from_u128(0xc2ab2b02_1a1c_4778_a81b_954a1788cc75);
pub const SENSOR_TYPE_FREQUENCY: windows_core::GUID = windows_core::GUID::from_u128(0x8cd2cbb6_73e6_4640_a709_72ae8fb60d7f);
pub const SENSOR_TYPE_GYROMETER_1D: windows_core::GUID = windows_core::GUID::from_u128(0xfa088734_f552_4584_8324_edfaf649652c);
pub const SENSOR_TYPE_GYROMETER_2D: windows_core::GUID = windows_core::GUID::from_u128(0x31ef4f83_919b_48bf_8de0_5d7a9d240556);
pub const SENSOR_TYPE_GYROMETER_3D: windows_core::GUID = windows_core::GUID::from_u128(0x09485f5a_759e_42c2_bd4b_a349b75c8643);
pub const SENSOR_TYPE_HUMAN_PRESENCE: windows_core::GUID = windows_core::GUID::from_u128(0xc138c12b_ad52_451c_9375_87f518ff10c6);
pub const SENSOR_TYPE_HUMAN_PROXIMITY: windows_core::GUID = windows_core::GUID::from_u128(0x5220dae9_3179_4430_9f90_06266d2a34de);
pub const SENSOR_TYPE_INCLINOMETER_1D: windows_core::GUID = windows_core::GUID::from_u128(0xb96f98c5_7a75_4ba7_94e9_ac868c966dd8);
pub const SENSOR_TYPE_INCLINOMETER_2D: windows_core::GUID = windows_core::GUID::from_u128(0xab140f6d_83eb_4264_b70b_b16a5b256a01);
pub const SENSOR_TYPE_INCLINOMETER_3D: windows_core::GUID = windows_core::GUID::from_u128(0xb84919fb_ea85_4976_8444_6f6f5c6d31db);
pub const SENSOR_TYPE_INDUCTANCE: windows_core::GUID = windows_core::GUID::from_u128(0xdc1d933f_c435_4c7d_a2fe_607192a524d3);
pub const SENSOR_TYPE_LOCATION_BROADCAST: windows_core::GUID = windows_core::GUID::from_u128(0xd26988cf_5162_4039_bb17_4c58b698e44a);
pub const SENSOR_TYPE_LOCATION_DEAD_RECKONING: windows_core::GUID = windows_core::GUID::from_u128(0x1a37d538_f28b_42da_9fce_a9d0a2a6d829);
pub const SENSOR_TYPE_LOCATION_GPS: windows_core::GUID = windows_core::GUID::from_u128(0xed4ca589_327a_4ff9_a560_91da4b48275e);
pub const SENSOR_TYPE_LOCATION_LOOKUP: windows_core::GUID = windows_core::GUID::from_u128(0x3b2eae4a_72ce_436d_96d2_3c5b8570e987);
pub const SENSOR_TYPE_LOCATION_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x9b2d0566_0368_4f71_b88d_533f132031de);
pub const SENSOR_TYPE_LOCATION_STATIC: windows_core::GUID = windows_core::GUID::from_u128(0x095f8184_0fa9_4445_8e6e_b70f320b6b4c);
pub const SENSOR_TYPE_LOCATION_TRIANGULATION: windows_core::GUID = windows_core::GUID::from_u128(0x691c341a_5406_4fe1_942f_2246cbeb39e0);
pub const SENSOR_TYPE_MOTION_DETECTOR: windows_core::GUID = windows_core::GUID::from_u128(0x5c7c1a12_30a5_43b9_a4b2_cf09ec5b7be8);
pub const SENSOR_TYPE_MULTIVALUE_SWITCH: windows_core::GUID = windows_core::GUID::from_u128(0xb3ee4d76_37a4_4402_b25e_99c60a775fa1);
pub const SENSOR_TYPE_POTENTIOMETER: windows_core::GUID = windows_core::GUID::from_u128(0x2b3681a9_cadc_45aa_a6ff_54957c8bb440);
pub const SENSOR_TYPE_PRESSURE: windows_core::GUID = windows_core::GUID::from_u128(0x26d31f34_6352_41cf_b793_ea0713d53d77);
pub const SENSOR_TYPE_RESISTANCE: windows_core::GUID = windows_core::GUID::from_u128(0x9993d2c8_c157_4a52_a7b5_195c76037231);
pub const SENSOR_TYPE_RFID_SCANNER: windows_core::GUID = windows_core::GUID::from_u128(0x44328ef5_02dd_4e8d_ad5d_9249832b2eca);
pub const SENSOR_TYPE_SCALE: windows_core::GUID = windows_core::GUID::from_u128(0xc06dd92c_7feb_438e_9bf6_82207fff5bb8);
pub const SENSOR_TYPE_SPEEDOMETER: windows_core::GUID = windows_core::GUID::from_u128(0x6bd73c1f_0bb4_4310_81b2_dfc18a52bf94);
pub const SENSOR_TYPE_STRAIN: windows_core::GUID = windows_core::GUID::from_u128(0xc6d1ec0e_6803_4361_ad3d_85bcc58c6d29);
pub const SENSOR_TYPE_TOUCH: windows_core::GUID = windows_core::GUID::from_u128(0x17db3018_06c4_4f7d_81af_9274b7599c27);
pub const SENSOR_TYPE_UNKNOWN: windows_core::GUID = windows_core::GUID::from_u128(0x10ba83e3_ef4f_41ed_9885_a87d6435a8e1);
pub const SENSOR_TYPE_VOLTAGE: windows_core::GUID = windows_core::GUID::from_u128(0xc5484637_4fb7_4953_98b8_a56d8aa1fb1e);
pub const SIMPLE_DEVICE_ORIENTATION_NOT_ROTATED: SimpleDeviceOrientation = SimpleDeviceOrientation(0i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_180: SimpleDeviceOrientation = SimpleDeviceOrientation(2i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_270: SimpleDeviceOrientation = SimpleDeviceOrientation(3i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_90: SimpleDeviceOrientation = SimpleDeviceOrientation(1i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_DOWN: SimpleDeviceOrientation = SimpleDeviceOrientation(5i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_UP: SimpleDeviceOrientation = SimpleDeviceOrientation(4i32);
pub const SensorConnectionType_Attached: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(1i32);
pub const SensorConnectionType_External: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(2i32);
pub const SensorConnectionType_Integrated: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(0i32);
pub const SensorState_Active: SENSOR_STATE = SENSOR_STATE(2i32);
pub const SensorState_Error: SENSOR_STATE = SENSOR_STATE(3i32);
pub const SensorState_Idle: SENSOR_STATE = SENSOR_STATE(1i32);
pub const SensorState_Initializing: SENSOR_STATE = SENSOR_STATE(0i32);
pub const SimpleDeviceOrientation_Facedown: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(5i32);
pub const SimpleDeviceOrientation_Faceup: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(4i32);
pub const SimpleDeviceOrientation_NotRotated: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(0i32);
pub const SimpleDeviceOrientation_Rotated180DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(2i32);
pub const SimpleDeviceOrientation_Rotated270DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(3i32);
pub const SimpleDeviceOrientation_Rotated90DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACTIVITY_STATE(pub i32);
impl windows_core::TypeKind for ACTIVITY_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACTIVITY_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACTIVITY_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACTIVITY_STATE_COUNT(pub i32);
impl windows_core::TypeKind for ACTIVITY_STATE_COUNT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACTIVITY_STATE_COUNT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACTIVITY_STATE_COUNT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AXIS(pub i32);
impl windows_core::TypeKind for AXIS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AXIS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AXIS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ELEVATION_CHANGE_MODE(pub i32);
impl windows_core::TypeKind for ELEVATION_CHANGE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ELEVATION_CHANGE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ELEVATION_CHANGE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE(pub i32);
impl windows_core::TypeKind for HUMAN_PRESENCE_DETECTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HUMAN_PRESENCE_DETECTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HUMAN_PRESENCE_DETECTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE_COUNT(pub i32);
impl windows_core::TypeKind for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HUMAN_PRESENCE_DETECTION_TYPE_COUNT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOCATION_DESIRED_ACCURACY(pub i32);
impl windows_core::TypeKind for LOCATION_DESIRED_ACCURACY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOCATION_DESIRED_ACCURACY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOCATION_DESIRED_ACCURACY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOCATION_POSITION_SOURCE(pub i32);
impl windows_core::TypeKind for LOCATION_POSITION_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOCATION_POSITION_SOURCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOCATION_POSITION_SOURCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MAGNETOMETER_ACCURACY(pub i32);
impl windows_core::TypeKind for MAGNETOMETER_ACCURACY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MAGNETOMETER_ACCURACY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MAGNETOMETER_ACCURACY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagnetometerAccuracy(pub i32);
impl windows_core::TypeKind for MagnetometerAccuracy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagnetometerAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagnetometerAccuracy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEDOMETER_STEP_TYPE(pub i32);
impl windows_core::TypeKind for PEDOMETER_STEP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEDOMETER_STEP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEDOMETER_STEP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEDOMETER_STEP_TYPE_COUNT(pub i32);
impl windows_core::TypeKind for PEDOMETER_STEP_TYPE_COUNT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEDOMETER_STEP_TYPE_COUNT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEDOMETER_STEP_TYPE_COUNT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROXIMITY_SENSOR_CAPABILITIES(pub i32);
impl windows_core::TypeKind for PROXIMITY_SENSOR_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROXIMITY_SENSOR_CAPABILITIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROXIMITY_SENSOR_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROXIMITY_TYPE(pub i32);
impl windows_core::TypeKind for PROXIMITY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROXIMITY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROXIMITY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SENSOR_CONNECTION_TYPES(pub i32);
impl windows_core::TypeKind for SENSOR_CONNECTION_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SENSOR_CONNECTION_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SENSOR_CONNECTION_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SENSOR_STATE(pub i32);
impl windows_core::TypeKind for SENSOR_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SENSOR_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SENSOR_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SIMPLE_DEVICE_ORIENTATION(pub i32);
impl windows_core::TypeKind for SIMPLE_DEVICE_ORIENTATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SIMPLE_DEVICE_ORIENTATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SIMPLE_DEVICE_ORIENTATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SensorConnectionType(pub i32);
impl windows_core::TypeKind for SensorConnectionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SensorConnectionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SensorConnectionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SensorState(pub i32);
impl windows_core::TypeKind for SensorState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SensorState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SensorState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SimpleDeviceOrientation(pub i32);
impl windows_core::TypeKind for SimpleDeviceOrientation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SimpleDeviceOrientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SimpleDeviceOrientation").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MATRIX3X3 {
    pub Anonymous: MATRIX3X3_0,
}
impl windows_core::TypeKind for MATRIX3X3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MATRIX3X3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MATRIX3X3_0 {
    pub Anonymous1: MATRIX3X3_0_0,
    pub Anonymous2: MATRIX3X3_0_1,
    pub M: [f32; 9],
}
impl windows_core::TypeKind for MATRIX3X3_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MATRIX3X3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MATRIX3X3_0_0 {
    pub A11: f32,
    pub A12: f32,
    pub A13: f32,
    pub A21: f32,
    pub A22: f32,
    pub A23: f32,
    pub A31: f32,
    pub A32: f32,
    pub A33: f32,
}
impl windows_core::TypeKind for MATRIX3X3_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MATRIX3X3_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MATRIX3X3_0_1 {
    pub V1: VEC3D,
    pub V2: VEC3D,
    pub V3: VEC3D,
}
impl windows_core::TypeKind for MATRIX3X3_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MATRIX3X3_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QUATERNION {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl windows_core::TypeKind for QUATERNION {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUATERNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Debug, Eq, PartialEq)]
pub struct SENSOR_COLLECTION_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [SENSOR_VALUE_PAIR; 1],
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for SENSOR_COLLECTION_LIST {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for SENSOR_COLLECTION_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for SENSOR_COLLECTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SENSOR_PROPERTY_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [super::super::UI::Shell::PropertiesSystem::PROPERTYKEY; 1],
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for SENSOR_PROPERTY_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for SENSOR_PROPERTY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Debug, Eq, PartialEq)]
pub struct SENSOR_VALUE_PAIR {
    pub Key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    pub Value: core::mem::ManuallyDrop<windows_core::PROPVARIANT>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for SENSOR_VALUE_PAIR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for SENSOR_VALUE_PAIR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for SENSOR_VALUE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Sensor: windows_core::GUID = windows_core::GUID::from_u128(0xe97ced00_523a_4133_bf6f_d3a2dae7f6ba);
pub const SensorCollection: windows_core::GUID = windows_core::GUID::from_u128(0x79c43adb_a429_469f_aa39_2f2b74b75937);
pub const SensorDataReport: windows_core::GUID = windows_core::GUID::from_u128(0x4ea9d6ef_694b_4218_8816_ccda8da74bba);
pub const SensorManager: windows_core::GUID = windows_core::GUID::from_u128(0x77a1c827_fcd2_4689_8915_9d613cc5fa3e);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VEC3D {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl windows_core::TypeKind for VEC3D {
    type TypeKind = windows_core::CopyType;
}
impl Default for VEC3D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
