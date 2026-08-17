#[inline]
pub unsafe fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSCoerceToCanonicalValue(key : *const PROPERTYKEY, ppropvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    PSCoerceToCanonicalValue(key, core::mem::transmute(ppropvar)).ok()
}
#[inline]
pub unsafe fn PSCreateAdapterFromPropertyStore<P0>(pps: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<IPropertyStore>,
{
    windows_targets::link!("propsys.dll" "system" fn PSCreateAdapterFromPropertyStore(pps : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreateAdapterFromPropertyStore(pps.param().abi(), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSCreateDelayedMultiplexPropertyStore<P0>(flags: GETPROPERTYSTOREFLAGS, pdpsf: P0, rgstoreids: &[u32], riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<IDelayedPropertyStoreFactory>,
{
    windows_targets::link!("propsys.dll" "system" fn PSCreateDelayedMultiplexPropertyStore(flags : GETPROPERTYSTOREFLAGS, pdpsf : * mut core::ffi::c_void, rgstoreids : *const u32, cstores : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreateDelayedMultiplexPropertyStore(flags, pdpsf.param().abi(), core::mem::transmute(rgstoreids.as_ptr()), rgstoreids.len().try_into().unwrap(), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSCreateMemoryPropertyStore(riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSCreateMemoryPropertyStore(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreateMemoryPropertyStore(riid, ppv).ok()
}
#[inline]
pub unsafe fn PSCreateMultiplexPropertyStore(prgpunkstores: &[Option<windows_core::IUnknown>], riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSCreateMultiplexPropertyStore(prgpunkstores : *const * mut core::ffi::c_void, cstores : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreateMultiplexPropertyStore(core::mem::transmute(prgpunkstores.as_ptr()), prgpunkstores.len().try_into().unwrap(), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSCreatePropertyChangeArray(rgpropkey: Option<*const PROPERTYKEY>, rgflags: Option<*const PKA_FLAGS>, rgpropvar: Option<*const windows_core::PROPVARIANT>, cchanges: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSCreatePropertyChangeArray(rgpropkey : *const PROPERTYKEY, rgflags : *const PKA_FLAGS, rgpropvar : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >, cchanges : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreatePropertyChangeArray(core::mem::transmute(rgpropkey.unwrap_or(std::ptr::null())), core::mem::transmute(rgflags.unwrap_or(std::ptr::null())), core::mem::transmute(rgpropvar.unwrap_or(std::ptr::null())), cchanges, riid, ppv).ok()
}
#[inline]
pub unsafe fn PSCreatePropertyStoreFromObject<P0>(punk: P0, grfmode: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromObject(punk : * mut core::ffi::c_void, grfmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreatePropertyStoreFromObject(punk.param().abi(), grfmode, riid, ppv).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromPropertySetStorage<P0>(ppss: P0, grfmode: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertySetStorage>,
{
    windows_targets::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromPropertySetStorage(ppss : * mut core::ffi::c_void, grfmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreatePropertyStoreFromPropertySetStorage(ppss.param().abi(), grfmode, riid, ppv).ok()
}
#[inline]
pub unsafe fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const windows_core::PROPVARIANT, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSCreateSimplePropertyChange(flags : PKA_FLAGS, key : *const PROPERTYKEY, propvar : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSCreateSimplePropertyChange(flags, key, core::mem::transmute(propvar), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSEnumeratePropertyDescriptions(filteron : PROPDESC_ENUMFILTER, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSEnumeratePropertyDescriptions(filteron, riid, ppv).ok()
}
#[inline]
pub unsafe fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const windows_core::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: &mut [u16]) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSFormatForDisplay(propkey : *const PROPERTYKEY, propvar : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >, pdfflags : PROPDESC_FORMAT_FLAGS, pwsztext : windows_core::PWSTR, cchtext : u32) -> windows_core::HRESULT);
    PSFormatForDisplay(propkey, core::mem::transmute(propvar), pdfflags, core::mem::transmute(pwsztext.as_ptr()), pwsztext.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const windows_core::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("propsys.dll" "system" fn PSFormatForDisplayAlloc(key : *const PROPERTYKEY, propvar : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSFormatForDisplayAlloc(key, core::mem::transmute(propvar), pdff, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PSFormatPropertyValue<P0, P1>(pps: P0, ppd: P1, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<IPropertyStore>,
    P1: windows_core::Param<IPropertyDescription>,
{
    windows_targets::link!("propsys.dll" "system" fn PSFormatPropertyValue(pps : * mut core::ffi::c_void, ppd : * mut core::ffi::c_void, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSFormatPropertyValue(pps.param().abi(), ppd.param().abi(), pdff, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const windows_core::PROPVARIANT) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("propsys.dll" "system" fn PSGetImageReferenceForValue(propkey : *const PROPERTYKEY, propvar : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >, ppszimageres : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSGetImageReferenceForValue(propkey, core::mem::transmute(propvar), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PSGetItemPropertyHandler<P0, P1>(punkitem: P0, freadwrite: P1, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetItemPropertyHandler(punkitem : * mut core::ffi::c_void, freadwrite : super::super::super::Foundation:: BOOL, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSGetItemPropertyHandler(punkitem.param().abi(), freadwrite.param().abi(), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSGetItemPropertyHandlerWithCreateObject<P0, P1, P2>(punkitem: P0, freadwrite: P1, punkcreateobject: P2, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetItemPropertyHandlerWithCreateObject(punkitem : * mut core::ffi::c_void, freadwrite : super::super::super::Foundation:: BOOL, punkcreateobject : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSGetItemPropertyHandlerWithCreateObject(punkitem.param().abi(), freadwrite.param().abi(), punkcreateobject.param().abi(), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("propsys.dll" "system" fn PSGetNameFromPropertyKey(propkey : *const PROPERTYKEY, ppszcanonicalname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSGetNameFromPropertyKey(propkey, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PSGetNamedPropertyFromPropertyStorage<P0, P1>(psps: P0, cb: u32, pszname: P1) -> windows_core::Result<windows_core::PROPVARIANT>
where
    P0: windows_core::Param<PCUSERIALIZEDPROPSTORAGE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetNamedPropertyFromPropertyStorage(psps : PCUSERIALIZEDPROPSTORAGE, cb : u32, pszname : windows_core::PCWSTR, ppropvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSGetNamedPropertyFromPropertyStorage(psps.param().abi(), cb, pszname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSGetPropertyDescription(propkey : *const PROPERTYKEY, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSGetPropertyDescription(propkey, riid, ppv).ok()
}
#[inline]
pub unsafe fn PSGetPropertyDescriptionByName<P0>(pszcanonicalname: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetPropertyDescriptionByName(pszcanonicalname : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSGetPropertyDescriptionByName(pszcanonicalname.param().abi(), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSGetPropertyDescriptionListFromString<P0>(pszproplist: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetPropertyDescriptionListFromString(pszproplist : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSGetPropertyDescriptionListFromString(pszproplist.param().abi(), riid, ppv).ok()
}
#[inline]
pub unsafe fn PSGetPropertyFromPropertyStorage<P0>(psps: P0, cb: u32, rpkey: *const PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>
where
    P0: windows_core::Param<PCUSERIALIZEDPROPSTORAGE>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetPropertyFromPropertyStorage(psps : PCUSERIALIZEDPROPSTORAGE, cb : u32, rpkey : *const PROPERTYKEY, ppropvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSGetPropertyFromPropertyStorage(psps.param().abi(), cb, rpkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn PSGetPropertyKeyFromName<P0>(pszname: P0, ppropkey: *mut PROPERTYKEY) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetPropertyKeyFromName(pszname : windows_core::PCWSTR, ppropkey : *mut PROPERTYKEY) -> windows_core::HRESULT);
    PSGetPropertyKeyFromName(pszname.param().abi(), ppropkey).ok()
}
#[inline]
pub unsafe fn PSGetPropertySystem(riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSGetPropertySystem(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSGetPropertySystem(riid, ppv).ok()
}
#[inline]
pub unsafe fn PSGetPropertyValue<P0, P1>(pps: P0, ppd: P1) -> windows_core::Result<windows_core::PROPVARIANT>
where
    P0: windows_core::Param<IPropertyStore>,
    P1: windows_core::Param<IPropertyDescription>,
{
    windows_targets::link!("propsys.dll" "system" fn PSGetPropertyValue(pps : * mut core::ffi::c_void, ppd : * mut core::ffi::c_void, ppropvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSGetPropertyValue(pps.param().abi(), ppd.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn PSLookupPropertyHandlerCLSID<P0>(pszfilepath: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSLookupPropertyHandlerCLSID(pszfilepath : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSLookupPropertyHandlerCLSID(pszfilepath.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_Delete<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_Delete(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR) -> windows_core::HRESULT);
    PSPropertyBag_Delete(propbag.param().abi(), propname.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadBOOL<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::super::super::Foundation::BOOL>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadBOOL(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadBOOL(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadBSTR<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadBSTR(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadBSTR(propbag.param().abi(), propname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadDWORD<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadDWORD(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadDWORD(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadGUID<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadGUID(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadGUID(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadInt<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadInt(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadInt(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadLONG<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadLONG(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadLONG(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTL<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::super::super::Foundation::POINTL>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTL(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::super::super::Foundation:: POINTL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadPOINTL(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTS<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::super::super::Foundation::POINTS>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTS(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::super::super::Foundation:: POINTS) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadPOINTS(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadPropertyKey<P0, P1>(propbag: P0, propname: P1, value: *mut PROPERTYKEY) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadPropertyKey(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut PROPERTYKEY) -> windows_core::HRESULT);
    PSPropertyBag_ReadPropertyKey(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadRECTL<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::super::super::Foundation::RECTL>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadRECTL(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::super::super::Foundation:: RECTL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadRECTL(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadSHORT<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<i16>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadSHORT(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadSHORT(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStr<P0, P1>(propbag: P0, propname: P1, value: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadStr(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : windows_core::PWSTR, charactercount : i32) -> windows_core::HRESULT);
    PSPropertyBag_ReadStr(propbag.param().abi(), propname.param().abi(), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStrAlloc<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadStrAlloc(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadStrAlloc(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStream<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::super::super::System::Com::IStream>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadStream(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadStream(propbag.param().abi(), propname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadType<P0, P1>(propbag: P0, propname: P1, var: *mut windows_core::VARIANT, r#type: super::super::super::System::Variant::VARENUM) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadType(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, var : *mut core::mem::MaybeUninit < windows_core::VARIANT >, r#type : super::super::super::System::Variant:: VARENUM) -> windows_core::HRESULT);
    PSPropertyBag_ReadType(propbag.param().abi(), propname.param().abi(), core::mem::transmute(var), r#type).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadULONGLONG<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<u64>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadULONGLONG(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PSPropertyBag_ReadULONGLONG(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadUnknown<P0, P1>(propbag: P0, propname: P1, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadUnknown(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PSPropertyBag_ReadUnknown(propbag.param().abi(), propname.param().abi(), riid, ppv).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteBOOL<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteBOOL(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    PSPropertyBag_WriteBOOL(propbag.param().abi(), propname.param().abi(), value.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteBSTR<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::BSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteBSTR(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    PSPropertyBag_WriteBSTR(propbag.param().abi(), propname.param().abi(), value.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteDWORD<P0, P1>(propbag: P0, propname: P1, value: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteDWORD(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : u32) -> windows_core::HRESULT);
    PSPropertyBag_WriteDWORD(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteGUID<P0, P1>(propbag: P0, propname: P1, value: *const windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteGUID(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const windows_core::GUID) -> windows_core::HRESULT);
    PSPropertyBag_WriteGUID(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteInt<P0, P1>(propbag: P0, propname: P1, value: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteInt(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : i32) -> windows_core::HRESULT);
    PSPropertyBag_WriteInt(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteLONG<P0, P1>(propbag: P0, propname: P1, value: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteLONG(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : i32) -> windows_core::HRESULT);
    PSPropertyBag_WriteLONG(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTL<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::POINTL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTL(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const super::super::super::Foundation:: POINTL) -> windows_core::HRESULT);
    PSPropertyBag_WritePOINTL(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTS<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::POINTS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTS(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const super::super::super::Foundation:: POINTS) -> windows_core::HRESULT);
    PSPropertyBag_WritePOINTS(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WritePropertyKey<P0, P1>(propbag: P0, propname: P1, value: *const PROPERTYKEY) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WritePropertyKey(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const PROPERTYKEY) -> windows_core::HRESULT);
    PSPropertyBag_WritePropertyKey(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteRECTL<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::RECTL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteRECTL(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const super::super::super::Foundation:: RECTL) -> windows_core::HRESULT);
    PSPropertyBag_WriteRECTL(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteSHORT<P0, P1>(propbag: P0, propname: P1, value: i16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteSHORT(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : i16) -> windows_core::HRESULT);
    PSPropertyBag_WriteSHORT(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStr<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteStr(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : windows_core::PCWSTR) -> windows_core::HRESULT);
    PSPropertyBag_WriteStr(propbag.param().abi(), propname.param().abi(), value.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStream<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::super::System::Com::IStream>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteStream(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : * mut core::ffi::c_void) -> windows_core::HRESULT);
    PSPropertyBag_WriteStream(propbag.param().abi(), propname.param().abi(), value.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteULONGLONG<P0, P1>(propbag: P0, propname: P1, value: u64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteULONGLONG(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, value : u64) -> windows_core::HRESULT);
    PSPropertyBag_WriteULONGLONG(propbag.param().abi(), propname.param().abi(), value).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteUnknown<P0, P1, P2>(propbag: P0, propname: P1, punk: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteUnknown(propbag : * mut core::ffi::c_void, propname : windows_core::PCWSTR, punk : * mut core::ffi::c_void) -> windows_core::HRESULT);
    PSPropertyBag_WriteUnknown(propbag.param().abi(), propname.param().abi(), punk.param().abi()).ok()
}
#[inline]
pub unsafe fn PSPropertyKeyFromString<P0>(pszstring: P0, pkey: *mut PROPERTYKEY) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSPropertyKeyFromString(pszstring : windows_core::PCWSTR, pkey : *mut PROPERTYKEY) -> windows_core::HRESULT);
    PSPropertyKeyFromString(pszstring.param().abi(), pkey).ok()
}
#[inline]
pub unsafe fn PSRefreshPropertySchema() -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSRefreshPropertySchema() -> windows_core::HRESULT);
    PSRefreshPropertySchema().ok()
}
#[inline]
pub unsafe fn PSRegisterPropertySchema<P0>(pszpath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSRegisterPropertySchema(pszpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    PSRegisterPropertySchema(pszpath.param().abi()).ok()
}
#[inline]
pub unsafe fn PSSetPropertyValue<P0, P1>(pps: P0, ppd: P1, propvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
where
    P0: windows_core::Param<IPropertyStore>,
    P1: windows_core::Param<IPropertyDescription>,
{
    windows_targets::link!("propsys.dll" "system" fn PSSetPropertyValue(pps : * mut core::ffi::c_void, ppd : * mut core::ffi::c_void, propvar : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    PSSetPropertyValue(pps.param().abi(), ppd.param().abi(), core::mem::transmute(propvar)).ok()
}
#[inline]
pub unsafe fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: &mut [u16]) -> windows_core::Result<()> {
    windows_targets::link!("propsys.dll" "system" fn PSStringFromPropertyKey(pkey : *const PROPERTYKEY, psz : windows_core::PWSTR, cch : u32) -> windows_core::HRESULT);
    PSStringFromPropertyKey(pkey, core::mem::transmute(psz.as_ptr()), psz.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn PSUnregisterPropertySchema<P0>(pszpath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("propsys.dll" "system" fn PSUnregisterPropertySchema(pszpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    PSUnregisterPropertySchema(pszpath.param().abi()).ok()
}
#[inline]
pub unsafe fn PifMgr_CloseProperties<P0>(hprops: P0, flopt: u32) -> super::super::super::Foundation::HANDLE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("shell32.dll" "system" fn PifMgr_CloseProperties(hprops : super::super::super::Foundation:: HANDLE, flopt : u32) -> super::super::super::Foundation:: HANDLE);
    PifMgr_CloseProperties(hprops.param().abi(), flopt)
}
#[inline]
pub unsafe fn PifMgr_GetProperties<P0, P1>(hprops: P0, pszgroup: P1, lpprops: Option<*mut core::ffi::c_void>, cbprops: i32, flopt: u32) -> i32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("shell32.dll" "system" fn PifMgr_GetProperties(hprops : super::super::super::Foundation:: HANDLE, pszgroup : windows_core::PCSTR, lpprops : *mut core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
    PifMgr_GetProperties(hprops.param().abi(), pszgroup.param().abi(), core::mem::transmute(lpprops.unwrap_or(std::ptr::null_mut())), cbprops, flopt)
}
#[inline]
pub unsafe fn PifMgr_OpenProperties<P0, P1>(pszapp: P0, pszpif: P1, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("shell32.dll" "system" fn PifMgr_OpenProperties(pszapp : windows_core::PCWSTR, pszpif : windows_core::PCWSTR, hinf : u32, flopt : u32) -> super::super::super::Foundation:: HANDLE);
    PifMgr_OpenProperties(pszapp.param().abi(), pszpif.param().abi(), hinf, flopt)
}
#[inline]
pub unsafe fn PifMgr_SetProperties<P0, P1>(hprops: P0, pszgroup: P1, lpprops: *const core::ffi::c_void, cbprops: i32, flopt: u32) -> i32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("shell32.dll" "system" fn PifMgr_SetProperties(hprops : super::super::super::Foundation:: HANDLE, pszgroup : windows_core::PCSTR, lpprops : *const core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
    PifMgr_SetProperties(hprops.param().abi(), pszgroup.param().abi(), lpprops, cbprops, flopt)
}
#[inline]
pub unsafe fn SHAddDefaultPropertiesByExt<P0, P1>(pszext: P0, ppropstore: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IPropertyStore>,
{
    windows_targets::link!("shell32.dll" "system" fn SHAddDefaultPropertiesByExt(pszext : windows_core::PCWSTR, ppropstore : * mut core::ffi::c_void) -> windows_core::HRESULT);
    SHAddDefaultPropertiesByExt(pszext.param().abi(), ppropstore.param().abi()).ok()
}
#[inline]
pub unsafe fn SHGetPropertyStoreForWindow<P0, T>(hwnd: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    T: windows_core::Interface,
{
    windows_targets::link!("shell32.dll" "system" fn SHGetPropertyStoreForWindow(hwnd : super::super::super::Foundation:: HWND, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    SHGetPropertyStoreForWindow(hwnd.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_UI_Shell_Common")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("shell32.dll" "system" fn SHGetPropertyStoreFromIDList(pidl : *const super::Common:: ITEMIDLIST, flags : GETPROPERTYSTOREFLAGS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    SHGetPropertyStoreFromIDList(pidl, flags, riid, ppv).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromParsingName<P0, P1, T>(pszpath: P0, pbc: P1, flags: GETPROPERTYSTOREFLAGS) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::super::System::Com::IBindCtx>,
    T: windows_core::Interface,
{
    windows_targets::link!("shell32.dll" "system" fn SHGetPropertyStoreFromParsingName(pszpath : windows_core::PCWSTR, pbc : * mut core::ffi::c_void, flags : GETPROPERTYSTOREFLAGS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    SHGetPropertyStoreFromParsingName(pszpath.param().abi(), pbc.param().abi(), flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn SHPropStgCreate<P0>(psstg: P0, fmtid: *const windows_core::GUID, pclsid: Option<*const windows_core::GUID>, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut Option<super::super::super::System::Com::StructuredStorage::IPropertyStorage>, pucodepage: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertySetStorage>,
{
    windows_targets::link!("shell32.dll" "system" fn SHPropStgCreate(psstg : * mut core::ffi::c_void, fmtid : *const windows_core::GUID, pclsid : *const windows_core::GUID, grfflags : u32, grfmode : u32, dwdisposition : u32, ppstg : *mut * mut core::ffi::c_void, pucodepage : *mut u32) -> windows_core::HRESULT);
    SHPropStgCreate(psstg.param().abi(), fmtid, core::mem::transmute(pclsid.unwrap_or(std::ptr::null())), grfflags, grfmode, dwdisposition, core::mem::transmute(ppstg), core::mem::transmute(pucodepage.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn SHPropStgReadMultiple<P0>(pps: P0, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyStorage>,
{
    windows_targets::link!("shell32.dll" "system" fn SHPropStgReadMultiple(pps : * mut core::ffi::c_void, ucodepage : u32, cpspec : u32, rgpspec : *const super::super::super::System::Com::StructuredStorage:: PROPSPEC, rgvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    SHPropStgReadMultiple(pps.param().abi(), ucodepage, cpspec, rgpspec, core::mem::transmute(rgvar)).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn SHPropStgWriteMultiple<P0>(pps: P0, pucodepage: Option<*mut u32>, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut windows_core::PROPVARIANT, propidnamefirst: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::System::Com::StructuredStorage::IPropertyStorage>,
{
    windows_targets::link!("shell32.dll" "system" fn SHPropStgWriteMultiple(pps : * mut core::ffi::c_void, pucodepage : *mut u32, cpspec : u32, rgpspec : *const super::super::super::System::Com::StructuredStorage:: PROPSPEC, rgvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >, propidnamefirst : u32) -> windows_core::HRESULT);
    SHPropStgWriteMultiple(pps.param().abi(), core::mem::transmute(pucodepage.unwrap_or(std::ptr::null_mut())), cpspec, rgpspec, core::mem::transmute(rgvar), propidnamefirst).ok()
}
windows_core::imp::define_interface!(ICreateObject, ICreateObject_Vtbl, 0x75121952_e0d0_43e5_9380_1d80483acf72);
impl core::ops::Deref for ICreateObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateObject, windows_core::IUnknown);
impl ICreateObject {
    pub unsafe fn CreateObject<P0, T>(&self, clsid: *const windows_core::GUID, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), clsid, punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICreateObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDelayedPropertyStoreFactory, IDelayedPropertyStoreFactory_Vtbl, 0x40d4577f_e237_4bdb_bd69_58f089431b6a);
impl core::ops::Deref for IDelayedPropertyStoreFactory {
    type Target = IPropertyStoreFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDelayedPropertyStoreFactory, windows_core::IUnknown, IPropertyStoreFactory);
impl IDelayedPropertyStoreFactory {
    pub unsafe fn GetDelayedPropertyStore<T>(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetDelayedPropertyStore)(windows_core::Interface::as_raw(self), flags, dwstoreid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDelayedPropertyStoreFactory_Vtbl {
    pub base__: IPropertyStoreFactory_Vtbl,
    pub GetDelayedPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, GETPROPERTYSTOREFLAGS, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInitializeWithFile, IInitializeWithFile_Vtbl, 0xb7d14566_0509_4cce_a71f_0a554233bd9b);
impl core::ops::Deref for IInitializeWithFile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInitializeWithFile, windows_core::IUnknown);
impl IInitializeWithFile {
    pub unsafe fn Initialize<P0>(&self, pszfilepath: P0, grfmode: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pszfilepath.param().abi(), grfmode).ok()
    }
}
#[repr(C)]
pub struct IInitializeWithFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInitializeWithStream, IInitializeWithStream_Vtbl, 0xb824b49d_22ac_4161_ac8a_9916e8fa3f7f);
impl core::ops::Deref for IInitializeWithStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInitializeWithStream, windows_core::IUnknown);
impl IInitializeWithStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pstream: P0, grfmode: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), grfmode).ok()
    }
}
#[repr(C)]
pub struct IInitializeWithStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
windows_core::imp::define_interface!(INamedPropertyStore, INamedPropertyStore_Vtbl, 0x71604b0f_97b0_4764_8577_2f13e98a1422);
impl core::ops::Deref for INamedPropertyStore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INamedPropertyStore, windows_core::IUnknown);
impl INamedPropertyStore {
    pub unsafe fn GetNamedValue<P0>(&self, pszname: P0) -> windows_core::Result<windows_core::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamedValue)(windows_core::Interface::as_raw(self), pszname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNamedValue<P0>(&self, pszname: P0, propvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetNamedValue)(windows_core::Interface::as_raw(self), pszname.param().abi(), core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn GetNameCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNameAt(&self, iprop: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameAt)(windows_core::Interface::as_raw(self), iprop, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct INamedPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNamedValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub SetNamedValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNameAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectWithPropertyKey, IObjectWithPropertyKey_Vtbl, 0xfc0ca0a7_c316_4fd2_9031_3e628e6d4f23);
impl core::ops::Deref for IObjectWithPropertyKey {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectWithPropertyKey, windows_core::IUnknown);
impl IObjectWithPropertyKey {
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPropertyKey)(windows_core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyKey)(windows_core::Interface::as_raw(self), pkey).ok()
    }
}
#[repr(C)]
pub struct IObjectWithPropertyKey_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPropertyKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY) -> windows_core::HRESULT,
    pub GetPropertyKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPERTYKEY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersistSerializedPropStorage, IPersistSerializedPropStorage_Vtbl, 0xe318ad57_0aa0_450f_aca5_6fab7103d917);
impl core::ops::Deref for IPersistSerializedPropStorage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistSerializedPropStorage, windows_core::IUnknown);
impl IPersistSerializedPropStorage {
    pub unsafe fn SetFlags(&self, flags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetPropertyStorage<P0>(&self, psps: P0, cb: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PCUSERIALIZEDPROPSTORAGE>,
    {
        (windows_core::Interface::vtable(self).SetPropertyStorage)(windows_core::Interface::as_raw(self), psps.param().abi(), cb).ok()
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyStorage)(windows_core::Interface::as_raw(self), ppsps, pcb).ok()
    }
}
#[repr(C)]
pub struct IPersistSerializedPropStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetPropertyStorage: unsafe extern "system" fn(*mut core::ffi::c_void, PCUSERIALIZEDPROPSTORAGE, u32) -> windows_core::HRESULT,
    pub GetPropertyStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SERIALIZEDPROPSTORAGE, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersistSerializedPropStorage2, IPersistSerializedPropStorage2_Vtbl, 0x77effa68_4f98_4366_ba72_573b3d880571);
impl core::ops::Deref for IPersistSerializedPropStorage2 {
    type Target = IPersistSerializedPropStorage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistSerializedPropStorage2, windows_core::IUnknown, IPersistSerializedPropStorage);
impl IPersistSerializedPropStorage2 {
    pub unsafe fn GetPropertyStorageSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyStorageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyStorageBuffer)(windows_core::Interface::as_raw(self), psps, cb, pcbwritten).ok()
    }
}
#[repr(C)]
pub struct IPersistSerializedPropStorage2_Vtbl {
    pub base__: IPersistSerializedPropStorage_Vtbl,
    pub GetPropertyStorageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyStorageBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SERIALIZEDPROPSTORAGE, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyChange, IPropertyChange_Vtbl, 0xf917bc8a_1bba_4478_a245_1bde03eb9431);
impl core::ops::Deref for IPropertyChange {
    type Target = IObjectWithPropertyKey;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyChange, windows_core::IUnknown, IObjectWithPropertyKey);
impl IPropertyChange {
    pub unsafe fn ApplyToPropVariant(&self, propvarin: *const windows_core::PROPVARIANT) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplyToPropVariant)(windows_core::Interface::as_raw(self), core::mem::transmute(propvarin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPropertyChange_Vtbl {
    pub base__: IObjectWithPropertyKey_Vtbl,
    pub ApplyToPropVariant: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyChangeArray, IPropertyChangeArray_Vtbl, 0x380f5cad_1b5e_42f2_805d_637fd392d31e);
impl core::ops::Deref for IPropertyChangeArray {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyChangeArray, windows_core::IUnknown);
impl IPropertyChangeArray {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt<T>(&self, iindex: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), iindex, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, iindex: u32, ppropchange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPropertyChange>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), iindex, ppropchange.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, ppropchange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPropertyChange>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), ppropchange.param().abi()).ok()
    }
    pub unsafe fn AppendOrReplace<P0>(&self, ppropchange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPropertyChange>,
    {
        (windows_core::Interface::vtable(self).AppendOrReplace)(windows_core::Interface::as_raw(self), ppropchange.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, iindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), iindex).ok()
    }
    pub unsafe fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsKeyInArray)(windows_core::Interface::as_raw(self), key).ok()
    }
}
#[repr(C)]
pub struct IPropertyChangeArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendOrReplace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsKeyInArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyDescription, IPropertyDescription_Vtbl, 0x6f79d558_3e96_4549_a1d1_7d75d2288814);
impl core::ops::Deref for IPropertyDescription {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescription, windows_core::IUnknown);
impl IPropertyDescription {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyKey)(windows_core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCanonicalName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPropertyType(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEditInvitation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeFlags)(windows_core::Interface::as_raw(self), mask, &mut result__).map(|| result__)
    }
    pub unsafe fn GetViewFlags(&self) -> windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultColumnWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDisplayType(&self) -> windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetColumnState(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGroupingRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelativeDescriptionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const windows_core::PROPVARIANT, propvar2: *const windows_core::PROPVARIANT, ppszdesc1: *mut windows_core::PWSTR, ppszdesc2: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRelativeDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar1), core::mem::transmute(propvar2), ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSortDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSortDescriptionLabel)(windows_core::Interface::as_raw(self), fdescending.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAggregationType(&self) -> windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAggregationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConditionType)(windows_core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetEnumTypeList)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CoerceToCanonicalValue)(windows_core::Interface::as_raw(self), core::mem::transmute(ppropvar)).ok()
    }
    pub unsafe fn FormatForDisplay(&self, propvar: *const windows_core::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatForDisplay)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar), pdfflags, &mut result__).map(|| result__)
    }
    pub unsafe fn IsValueCanonical(&self, propvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsValueCanonical)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar)).ok()
    }
}
#[repr(C)]
pub struct IPropertyDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPERTYKEY) -> windows_core::HRESULT,
    pub GetCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPropertyType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetEditInvitation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, PROPDESC_TYPE_FLAGS, *mut PROPDESC_TYPE_FLAGS) -> windows_core::HRESULT,
    pub GetViewFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_VIEW_FLAGS) -> windows_core::HRESULT,
    pub GetDefaultColumnWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_DISPLAYTYPE) -> windows_core::HRESULT,
    pub GetColumnState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGroupingRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_GROUPING_RANGE) -> windows_core::HRESULT,
    pub GetRelativeDescriptionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> windows_core::HRESULT,
    pub GetRelativeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSortDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_SORTDESCRIPTION) -> windows_core::HRESULT,
    pub GetSortDescriptionLabel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetAggregationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_AGGREGATION_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")]
    pub GetConditionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_CONDITION_TYPE, *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))]
    GetConditionType: usize,
    pub GetEnumTypeList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CoerceToCanonicalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub FormatForDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, PROPDESC_FORMAT_FLAGS, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsValueCanonical: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyDescription2, IPropertyDescription2_Vtbl, 0x57d2eded_5062_400e_b107_5dae79fe57a6);
impl core::ops::Deref for IPropertyDescription2 {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescription2, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescription2 {
    pub unsafe fn GetImageReferenceForValue(&self, propvar: *const windows_core::PROPVARIANT) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageReferenceForValue)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPropertyDescription2_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetImageReferenceForValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyDescriptionAliasInfo, IPropertyDescriptionAliasInfo_Vtbl, 0xf67104fc_2af9_46fd_b32d_243c1404f3d1);
impl core::ops::Deref for IPropertyDescriptionAliasInfo {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescriptionAliasInfo, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescriptionAliasInfo {
    pub unsafe fn GetSortByAlias<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetSortByAlias)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAdditionalSortByAliases<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAdditionalSortByAliases)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPropertyDescriptionAliasInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSortByAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdditionalSortByAliases: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyDescriptionList, IPropertyDescriptionList_Vtbl, 0x1f9fc1d0_c39b_4b26_817f_011967d3440e);
impl core::ops::Deref for IPropertyDescriptionList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescriptionList, windows_core::IUnknown);
impl IPropertyDescriptionList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt<T>(&self, ielem: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ielem, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPropertyDescriptionList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyDescriptionRelatedPropertyInfo, IPropertyDescriptionRelatedPropertyInfo_Vtbl, 0x507393f4_2a3d_4a60_b59e_d9c75716c2dd);
impl core::ops::Deref for IPropertyDescriptionRelatedPropertyInfo {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescriptionRelatedPropertyInfo, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescriptionRelatedPropertyInfo {
    pub unsafe fn GetRelatedProperty<P0, T>(&self, pszrelationshipname: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetRelatedProperty)(windows_core::Interface::as_raw(self), pszrelationshipname.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetRelatedProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyDescriptionSearchInfo, IPropertyDescriptionSearchInfo_Vtbl, 0x078f91bd_29a2_440f_924e_46a291524520);
impl core::ops::Deref for IPropertyDescriptionSearchInfo {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescriptionSearchInfo, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescriptionSearchInfo {
    pub unsafe fn GetSearchInfoFlags(&self) -> windows_core::Result<PROPDESC_SEARCHINFO_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSearchInfoFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetColumnIndexType(&self) -> windows_core::Result<PROPDESC_COLUMNINDEX_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnIndexType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProjectionString(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProjectionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPropertyDescriptionSearchInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSearchInfoFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_SEARCHINFO_FLAGS) -> windows_core::HRESULT,
    pub GetColumnIndexType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_COLUMNINDEX_TYPE) -> windows_core::HRESULT,
    pub GetProjectionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyEnumType, IPropertyEnumType_Vtbl, 0x11e1fbf9_2d56_4a6b_8db3_7cd193a471f2);
impl core::ops::Deref for IPropertyEnumType {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyEnumType, windows_core::IUnknown);
impl IPropertyEnumType {
    pub unsafe fn GetEnumType(&self) -> windows_core::Result<PROPENUMTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValue(&self) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRangeMinValue(&self) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRangeMinValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRangeSetValue(&self) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRangeSetValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayText(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPropertyEnumType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnumType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPENUMTYPE) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetRangeMinValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetRangeSetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyEnumType2, IPropertyEnumType2_Vtbl, 0x9b6e051c_5ddd_4321_9070_fe2acb55e794);
impl core::ops::Deref for IPropertyEnumType2 {
    type Target = IPropertyEnumType;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyEnumType2, windows_core::IUnknown, IPropertyEnumType);
impl IPropertyEnumType2 {
    pub unsafe fn GetImageReference(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPropertyEnumType2_Vtbl {
    pub base__: IPropertyEnumType_Vtbl,
    pub GetImageReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyEnumTypeList, IPropertyEnumTypeList_Vtbl, 0xa99400f4_3d84_4557_94ba_1242fb2cc9a6);
impl core::ops::Deref for IPropertyEnumTypeList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyEnumTypeList, windows_core::IUnknown);
impl IPropertyEnumTypeList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt<T>(&self, itype: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), itype, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConditionAt<T>(&self, nindex: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetConditionAt)(windows_core::Interface::as_raw(self), nindex, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindMatchingIndex(&self, propvarcmp: *const windows_core::PROPVARIANT) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindMatchingIndex)(windows_core::Interface::as_raw(self), core::mem::transmute(propvarcmp), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPropertyEnumTypeList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConditionAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindMatchingIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyStore, IPropertyStore_Vtbl, 0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99);
impl core::ops::Deref for IPropertyStore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyStore, windows_core::IUnknown);
impl IPropertyStore {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), iprop, pkey).ok()
    }
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), key, core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut PROPERTYKEY) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyStoreCache, IPropertyStoreCache_Vtbl, 0x3017056d_9a91_4e90_937d_746c72abbf4f);
impl core::ops::Deref for IPropertyStoreCache {
    type Target = IPropertyStore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyStoreCache, windows_core::IUnknown, IPropertyStore);
impl IPropertyStoreCache {
    pub unsafe fn GetState(&self, key: *const PROPERTYKEY) -> windows_core::Result<PSC_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
    }
    pub unsafe fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut windows_core::PROPVARIANT, pstate: *mut PSC_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValueAndState)(windows_core::Interface::as_raw(self), key, core::mem::transmute(ppropvar), pstate).ok()
    }
    pub unsafe fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), key, state).ok()
    }
    pub unsafe fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const windows_core::PROPVARIANT, state: PSC_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValueAndState)(windows_core::Interface::as_raw(self), key, core::mem::transmute(ppropvar), state).ok()
    }
}
#[repr(C)]
pub struct IPropertyStoreCache_Vtbl {
    pub base__: IPropertyStore_Vtbl,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *mut PSC_STATE) -> windows_core::HRESULT,
    pub GetValueAndState: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut PSC_STATE) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, PSC_STATE) -> windows_core::HRESULT,
    pub SetValueAndState: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, PSC_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyStoreCapabilities, IPropertyStoreCapabilities_Vtbl, 0xc8e2d566_186e_4d49_bf41_6909ead56acc);
impl core::ops::Deref for IPropertyStoreCapabilities {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyStoreCapabilities, windows_core::IUnknown);
impl IPropertyStoreCapabilities {
    pub unsafe fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsPropertyWritable)(windows_core::Interface::as_raw(self), key).ok()
    }
}
#[repr(C)]
pub struct IPropertyStoreCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsPropertyWritable: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyStoreFactory, IPropertyStoreFactory_Vtbl, 0xbc110b6d_57e8_4148_a9c6_91015ab2f3a5);
impl core::ops::Deref for IPropertyStoreFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyStoreFactory, windows_core::IUnknown);
impl IPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<P0, T>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetPropertyStore)(windows_core::Interface::as_raw(self), flags, punkfactory.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyStoreForKeys<T>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetPropertyStoreForKeys)(windows_core::Interface::as_raw(self), rgkeys, ckeys, flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPropertyStoreFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, GETPROPERTYSTOREFLAGS, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyStoreForKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, u32, GETPROPERTYSTOREFLAGS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertySystem, IPropertySystem_Vtbl, 0xca724e8a_c3e6_442b_88a4_6fb0db8035a3);
impl core::ops::Deref for IPropertySystem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertySystem, windows_core::IUnknown);
impl IPropertySystem {
    pub unsafe fn GetPropertyDescription<T>(&self, propkey: *const PROPERTYKEY) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetPropertyDescription)(windows_core::Interface::as_raw(self), propkey, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyDescriptionByName<P0, T>(&self, pszcanonicalname: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetPropertyDescriptionByName)(windows_core::Interface::as_raw(self), pszcanonicalname.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyDescriptionListFromString<P0, T>(&self, pszproplist: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetPropertyDescriptionListFromString)(windows_core::Interface::as_raw(self), pszproplist.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePropertyDescriptions<T>(&self, filteron: PROPDESC_ENUMFILTER) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).EnumeratePropertyDescriptions)(windows_core::Interface::as_raw(self), filteron, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const windows_core::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FormatForDisplay)(windows_core::Interface::as_raw(self), key, core::mem::transmute(propvar), pdff, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()).ok()
    }
    pub unsafe fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const windows_core::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatForDisplayAlloc)(windows_core::Interface::as_raw(self), key, core::mem::transmute(propvar), pdff, &mut result__).map(|| result__)
    }
    pub unsafe fn RegisterPropertySchema<P0>(&self, pszpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterPropertySchema)(windows_core::Interface::as_raw(self), pszpath.param().abi()).ok()
    }
    pub unsafe fn UnregisterPropertySchema<P0>(&self, pszpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterPropertySchema)(windows_core::Interface::as_raw(self), pszpath.param().abi()).ok()
    }
    pub unsafe fn RefreshPropertySchema(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefreshPropertySchema)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPropertySystem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyDescriptionByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyDescriptionListFromString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumeratePropertyDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, PROPDESC_ENUMFILTER, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatForDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, PROPDESC_FORMAT_FLAGS, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub FormatForDisplayAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPERTYKEY, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, PROPDESC_FORMAT_FLAGS, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub RegisterPropertySchema: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterPropertySchema: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RefreshPropertySchema: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertySystemChangeNotify, IPropertySystemChangeNotify_Vtbl, 0xfa955fd9_38be_4879_a6ce_824cf52d609f);
impl core::ops::Deref for IPropertySystemChangeNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertySystemChangeNotify, windows_core::IUnknown);
impl IPropertySystemChangeNotify {
    pub unsafe fn SchemaRefreshed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SchemaRefreshed)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPropertySystemChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SchemaRefreshed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyUI, IPropertyUI_Vtbl, 0x757a7d9f_919a_4118_99d7_dbb208c8cc66);
impl core::ops::Deref for IPropertyUI {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyUI, windows_core::IUnknown);
impl IPropertyUI {
    pub unsafe fn ParsePropertyName<P0>(&self, pszname: P0, pfmtid: *mut windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ParsePropertyName)(windows_core::Interface::as_raw(self), pszname.param().abi(), pfmtid, ppid, pcheaten).ok()
    }
    pub unsafe fn GetCannonicalName(&self, fmtid: *const windows_core::GUID, pid: u32, pwsztext: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCannonicalName)(windows_core::Interface::as_raw(self), fmtid, pid, core::mem::transmute(pwsztext.as_ptr()), pwsztext.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDisplayName(&self, fmtid: *const windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), fmtid, pid, flags, core::mem::transmute(pwsztext.as_ptr()), pwsztext.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetPropertyDescription(&self, fmtid: *const windows_core::GUID, pid: u32, pwsztext: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyDescription)(windows_core::Interface::as_raw(self), fmtid, pid, core::mem::transmute(pwsztext.as_ptr()), pwsztext.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDefaultWidth(&self, fmtid: *const windows_core::GUID, pid: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultWidth)(windows_core::Interface::as_raw(self), fmtid, pid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetFlags(&self, fmtid: *const windows_core::GUID, pid: u32) -> windows_core::Result<PROPERTYUI_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), fmtid, pid, &mut result__).map(|| result__)
    }
    pub unsafe fn FormatForDisplay(&self, fmtid: *const windows_core::GUID, pid: u32, ppropvar: *const windows_core::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FormatForDisplay)(windows_core::Interface::as_raw(self), fmtid, pid, core::mem::transmute(ppropvar), puiff, core::mem::transmute(pwsztext.as_ptr()), pwsztext.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetHelpInfo(&self, fmtid: *const windows_core::GUID, pid: u32, pwszhelpfile: &mut [u16], puhelpid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHelpInfo)(windows_core::Interface::as_raw(self), fmtid, pid, core::mem::transmute(pwszhelpfile.as_ptr()), pwszhelpfile.len().try_into().unwrap(), puhelpid).ok()
    }
}
#[repr(C)]
pub struct IPropertyUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ParsePropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::GUID, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCannonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, PROPERTYUI_NAME_FLAGS, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetPropertyDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetDefaultWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut PROPERTYUI_FLAGS) -> windows_core::HRESULT,
    pub FormatForDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, PROPERTYUI_FORMAT_FLAGS, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetHelpInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(0i32);
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(1i32);
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(2i32);
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(64i32);
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(0i32);
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(32i32);
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(512i32);
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1024i32);
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8i32);
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1i32);
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8191i32);
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(128i32);
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(16i32);
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(256i32);
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2i32);
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4i32);
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2048i32);
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4096i32);
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(3i32);
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(4i32);
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(0i32);
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(1i32);
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(6i32);
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(7i32);
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(2i32);
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(5i32);
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(2i32);
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(0i32);
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(3i32);
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(1i32);
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(4i32);
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(5i32);
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(4i32);
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(3i32);
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(0i32);
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(5i32);
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(2i32);
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(1i32);
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(2i32);
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(3i32);
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(4i32);
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(1i32);
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(0i32);
pub const PDEF_ALL: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(0i32);
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(6i32);
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(5i32);
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(2i32);
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(4i32);
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(1i32);
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(3i32);
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4i32);
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(0i32);
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2i32);
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(512i32);
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(64i32);
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(256i32);
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(32i32);
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8192i32);
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1i32);
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4096i32);
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1024i32);
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8i32);
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(128i32);
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(16i32);
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2048i32);
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(1i32);
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(4i32);
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(0i32);
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(3i32);
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(6i32);
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(5i32);
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(2i32);
pub const PDOPS_CANCELLED: PDOPSTATUS = PDOPSTATUS(3i32);
pub const PDOPS_ERRORS: PDOPSTATUS = PDOPSTATUS(5i32);
pub const PDOPS_PAUSED: PDOPSTATUS = PDOPSTATUS(2i32);
pub const PDOPS_RUNNING: PDOPSTATUS = PDOPSTATUS(1i32);
pub const PDOPS_STOPPED: PDOPSTATUS = PDOPSTATUS(4i32);
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(3i32);
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(1i32);
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(6i32);
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(0i32);
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(5i32);
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(10i32);
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(8i32);
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(9i32);
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(4i32);
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(2i32);
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(7i32);
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(1i32);
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(0i32);
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(2i32);
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(4i32);
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(3i32);
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(8i32);
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(0i32);
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(1i32);
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(2i32);
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(4i32);
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(16i32);
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4096u32);
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(512u32);
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(8u32);
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(16u32);
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(0u32);
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2048u32);
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(64u32);
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4u32);
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2u32);
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(256u32);
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147483648u32);
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(32u32);
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(128u32);
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147491839u32);
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1u32);
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1024u32);
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4i32);
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4096i32);
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(1i32);
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(0i32);
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(8i32);
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2048i32);
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(512i32);
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(7167i32);
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2i32);
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(64i32);
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(128i32);
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(256i32);
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(32i32);
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(16i32);
pub const PET_DEFAULTVALUE: PROPENUMTYPE = PROPENUMTYPE(2i32);
pub const PET_DISCRETEVALUE: PROPENUMTYPE = PROPENUMTYPE(0i32);
pub const PET_ENDRANGE: PROPENUMTYPE = PROPENUMTYPE(3i32);
pub const PET_RANGEDVALUE: PROPENUMTYPE = PROPENUMTYPE(1i32);
pub const PKA_APPEND: PKA_FLAGS = PKA_FLAGS(1i32);
pub const PKA_DELETE: PKA_FLAGS = PKA_FLAGS(2i32);
pub const PKA_SET: PKA_FLAGS = PKA_FLAGS(0i32);
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
pub const PSC_DIRTY: PSC_STATE = PSC_STATE(2i32);
pub const PSC_NORMAL: PSC_STATE = PSC_STATE(0i32);
pub const PSC_NOTINSOURCE: PSC_STATE = PSC_STATE(1i32);
pub const PSC_READONLY: PSC_STATE = PSC_STATE(3i32);
pub const PS_ALL: PLACEHOLDER_STATES = PLACEHOLDER_STATES(15i32);
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = PLACEHOLDER_STATES(8i32);
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(4i32);
pub const PS_DEFAULT: PLACEHOLDER_STATES = PLACEHOLDER_STATES(7i32);
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(2i32);
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = PLACEHOLDER_STATES(1i32);
pub const PS_NONE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(0i32);
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(0i32);
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(8i32);
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(4i32);
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(1i32);
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(2i32);
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(0i32);
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(1i32);
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(0i32);
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(2i32);
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(1i32);
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(511i32);
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(4i32);
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(0i32);
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(32i32);
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(16i32);
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(8i32);
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(64i32);
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(256i32);
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(2i32);
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(1i32);
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(128i32);
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(256i32);
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(32i32);
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(16i32);
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(128i32);
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(512i32);
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(2i32);
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1i32);
pub const STS_NONE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(0i32);
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(8i32);
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1024i32);
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(4i32);
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(64i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GETPROPERTYSTOREFLAGS(pub i32);
impl windows_core::TypeKind for GETPROPERTYSTOREFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GETPROPERTYSTOREFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GETPROPERTYSTOREFLAGS").field(&self.0).finish()
    }
}
impl GETPROPERTYSTOREFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GETPROPERTYSTOREFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GETPROPERTYSTOREFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PDOPSTATUS(pub i32);
impl windows_core::TypeKind for PDOPSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PDOPSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDOPSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PKA_FLAGS(pub i32);
impl windows_core::TypeKind for PKA_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PKA_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PKA_FLAGS").field(&self.0).finish()
    }
}
impl PKA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PKA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PKA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PKA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PKA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PKA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PLACEHOLDER_STATES(pub i32);
impl windows_core::TypeKind for PLACEHOLDER_STATES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PLACEHOLDER_STATES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PLACEHOLDER_STATES").field(&self.0).finish()
    }
}
impl PLACEHOLDER_STATES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PLACEHOLDER_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PLACEHOLDER_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PLACEHOLDER_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_AGGREGATION_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_AGGREGATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_AGGREGATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_AGGREGATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_COLUMNINDEX_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_COLUMNINDEX_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_COLUMNINDEX_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_COLUMNINDEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_CONDITION_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_CONDITION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_CONDITION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_CONDITION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_DISPLAYTYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_DISPLAYTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_DISPLAYTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_DISPLAYTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_ENUMFILTER(pub i32);
impl windows_core::TypeKind for PROPDESC_ENUMFILTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_ENUMFILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_ENUMFILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_FORMAT_FLAGS(pub i32);
impl windows_core::TypeKind for PROPDESC_FORMAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_FORMAT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPDESC_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPDESC_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_GROUPING_RANGE(pub i32);
impl windows_core::TypeKind for PROPDESC_GROUPING_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_GROUPING_RANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_GROUPING_RANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_RELATIVEDESCRIPTION_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_RELATIVEDESCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_SEARCHINFO_FLAGS(pub i32);
impl windows_core::TypeKind for PROPDESC_SEARCHINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_SEARCHINFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_SEARCHINFO_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_SEARCHINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_SORTDESCRIPTION(pub i32);
impl windows_core::TypeKind for PROPDESC_SORTDESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_SORTDESCRIPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_SORTDESCRIPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_TYPE_FLAGS(pub u32);
impl windows_core::TypeKind for PROPDESC_TYPE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_TYPE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_TYPE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPDESC_TYPE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPDESC_TYPE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPDESC_VIEW_FLAGS(pub i32);
impl windows_core::TypeKind for PROPDESC_VIEW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPDESC_VIEW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPDESC_VIEW_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_VIEW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPDESC_VIEW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPDESC_VIEW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPENUMTYPE(pub i32);
impl windows_core::TypeKind for PROPENUMTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPENUMTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPENUMTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPERTYUI_FLAGS(pub i32);
impl windows_core::TypeKind for PROPERTYUI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPERTYUI_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPERTYUI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPERTYUI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPERTYUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPERTYUI_FORMAT_FLAGS(pub i32);
impl windows_core::TypeKind for PROPERTYUI_FORMAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPERTYUI_FORMAT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPERTYUI_NAME_FLAGS(pub i32);
impl windows_core::TypeKind for PROPERTYUI_NAME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPERTYUI_NAME_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPERTYUI_NAME_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_NAME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPERTYUI_NAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPERTYUI_NAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSC_STATE(pub i32);
impl windows_core::TypeKind for PSC_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSC_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSC_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_ENGINE_STATE_FLAGS(pub i32);
impl windows_core::TypeKind for SYNC_ENGINE_STATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_ENGINE_STATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_ENGINE_STATE_FLAGS").field(&self.0).finish()
    }
}
impl SYNC_ENGINE_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_TRANSFER_STATUS(pub i32);
impl windows_core::TypeKind for SYNC_TRANSFER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_TRANSFER_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_TRANSFER_STATUS").field(&self.0).finish()
    }
}
impl SYNC_TRANSFER_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SYNC_TRANSFER_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SYNC_TRANSFER_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _PERSIST_SPROPSTORE_FLAGS(pub i32);
impl windows_core::TypeKind for _PERSIST_SPROPSTORE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _PERSIST_SPROPSTORE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_PERSIST_SPROPSTORE_FLAGS").field(&self.0).finish()
    }
}
pub const InMemoryPropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
pub const InMemoryPropertyStoreMarshalByValue: windows_core::GUID = windows_core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUSERIALIZEDPROPSTORAGE(pub isize);
impl Default for PCUSERIALIZEDPROPSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PCUSERIALIZEDPROPSTORAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROPERTYKEY {
    pub fmtid: windows_core::GUID,
    pub pid: u32,
}
impl windows_core::TypeKind for PROPERTYKEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROPERTYKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [i8; 30],
    pub achCmdLine: [i8; 128],
    pub achWorkDir: [i8; 64],
    pub wHotKey: u16,
    pub achIconFile: [i8; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [i8; 80],
    pub achPIFFile: [i8; 260],
}
impl windows_core::TypeKind for PROPPRG {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROPPRG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PropertySystem: windows_core::GUID = windows_core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SERIALIZEDPROPSTORAGE(pub isize);
impl Default for SERIALIZEDPROPSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SERIALIZEDPROPSTORAGE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
