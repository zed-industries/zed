windows_core::imp::define_interface!(IOpcCertificateEnumerator, IOpcCertificateEnumerator_Vtbl, 0x85131937_8f24_421f_b439_59ab24d140b8);
impl core::ops::Deref for IOpcCertificateEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcCertificateEnumerator, windows_core::IUnknown);
impl IOpcCertificateEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<*mut super::super::super::Security::Cryptography::CERT_CONTEXT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcCertificateEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcCertificateEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    GetCurrent: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcCertificateSet, IOpcCertificateSet_Vtbl, 0x56ea4325_8e2d_4167_b1a4_e486d24c8fa7);
impl core::ops::Deref for IOpcCertificateSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcCertificateSet, windows_core::IUnknown);
impl IOpcCertificateSet {
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), certificate).ok()
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), certificate).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcCertificateSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    Add: usize,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    Remove: usize,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcDigitalSignature, IOpcDigitalSignature_Vtbl, 0x52ab21dd_1cd0_4949_bc80_0c1232d00cb4);
impl core::ops::Deref for IOpcDigitalSignature {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcDigitalSignature, windows_core::IUnknown);
impl IOpcDigitalSignature {
    pub unsafe fn GetNamespaces(&self, prefixes: *mut *mut windows_core::PWSTR, namespaces: *mut *mut windows_core::PWSTR, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNamespaces)(windows_core::Interface::as_raw(self), prefixes, namespaces, count).ok()
    }
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCanonicalizationMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCanonicalizationMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSignatureValue)(windows_core::Interface::as_raw(self), signaturevalue, count).ok()
    }
    pub unsafe fn GetSignaturePartReferenceEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignaturePartReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSignatureRelationshipReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureRelationshipReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSigningTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPackageObjectReference(&self) -> windows_core::Result<IOpcSignatureReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageObjectReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCustomObjectEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomObjectEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSignatureXml)(windows_core::Interface::as_raw(self), signaturexml, count).ok()
    }
}
#[repr(C)]
pub struct IOpcDigitalSignature_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::PWSTR, *mut *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetSignatureId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    pub GetSignatureMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetCanonicalizationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetSignaturePartReferenceEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSignatureRelationshipReferenceEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSigningTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT,
    pub GetPackageObjectReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCertificateEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSignatureXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcDigitalSignatureEnumerator, IOpcDigitalSignatureEnumerator_Vtbl, 0x967b6882_0ba3_4358_b9e7_b64c75063c5e);
impl core::ops::Deref for IOpcDigitalSignatureEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcDigitalSignatureEnumerator, windows_core::IUnknown);
impl IOpcDigitalSignatureEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcDigitalSignature> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcDigitalSignatureEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcDigitalSignatureManager, IOpcDigitalSignatureManager_Vtbl, 0xd5e62a0b_696d_462f_94df_72e33cef2659);
impl core::ops::Deref for IOpcDigitalSignatureManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcDigitalSignatureManager, windows_core::IUnknown);
impl IOpcDigitalSignatureManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignatureOriginPartName(&self) -> windows_core::Result<IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureOriginPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetSignatureOriginPartName)(windows_core::Interface::as_raw(self), signatureoriginpartname.param().abi()).ok()
    }
    pub unsafe fn GetSignatureEnumerator(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveSignature<P0>(&self, signaturepartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).RemoveSignature)(windows_core::Interface::as_raw(self), signaturepartname.param().abi()).ok()
    }
    pub unsafe fn CreateSigningOptions(&self) -> windows_core::Result<IOpcSigningOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSigningOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Validate<P0>(&self, signature: P0, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<OPC_SIGNATURE_VALIDATION_RESULT>
    where
        P0: windows_core::Param<IOpcDigitalSignature>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), signature.param().abi(), certificate, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Sign<P0>(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: P0) -> windows_core::Result<IOpcDigitalSignature>
    where
        P0: windows_core::Param<IOpcSigningOptions>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Sign)(windows_core::Interface::as_raw(self), certificate, signingoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReplaceSignatureXml<P0>(&self, signaturepartname: P0, newsignaturexml: &[u8]) -> windows_core::Result<IOpcDigitalSignature>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReplaceSignatureXml)(windows_core::Interface::as_raw(self), signaturepartname.param().abi(), core::mem::transmute(newsignaturexml.as_ptr()), newsignaturexml.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcDigitalSignatureManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignatureOriginPartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignatureOriginPartName: usize,
    pub GetSignatureEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveSignature: usize,
    pub CreateSigningOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Security::Cryptography::CERT_CONTEXT, *mut OPC_SIGNATURE_VALIDATION_RESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    Validate: usize,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub Sign: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Security::Cryptography::CERT_CONTEXT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    Sign: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReplaceSignatureXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReplaceSignatureXml: usize,
}
windows_core::imp::define_interface!(IOpcFactory, IOpcFactory_Vtbl, 0x6d0b4446_cd73_4ab3_94f4_8ccdf6116154);
impl core::ops::Deref for IOpcFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcFactory, windows_core::IUnknown);
impl IOpcFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageRootUri(&self) -> windows_core::Result<IOpcUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageRootUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePartUri<P0>(&self, pwzuri: P0) -> windows_core::Result<IOpcPartUri>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePartUri)(windows_core::Interface::as_raw(self), pwzuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStreamOnFile<P0>(&self, filename: P0, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> windows_core::Result<super::super::super::System::Com::IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStreamOnFile)(windows_core::Interface::as_raw(self), filename.param().abi(), iomode, securityattributes, dwflagsandattributes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePackage(&self) -> windows_core::Result<IOpcPackage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReadPackageFromStream<P0>(&self, stream: P0, flags: OPC_READ_FLAGS) -> windows_core::Result<IOpcPackage>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadPackageFromStream)(windows_core::Interface::as_raw(self), stream.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WritePackageToStream<P0, P1>(&self, package: P0, flags: OPC_WRITE_FLAGS, stream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPackage>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).WritePackageToStream)(windows_core::Interface::as_raw(self), package.param().abi(), flags, stream.param().abi()).ok()
    }
    pub unsafe fn CreateDigitalSignatureManager<P0>(&self, package: P0) -> windows_core::Result<IOpcDigitalSignatureManager>
    where
        P0: windows_core::Param<IOpcPackage>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDigitalSignatureManager)(windows_core::Interface::as_raw(self), package.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageRootUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageRootUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePartUri: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePartUri: usize,
    #[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub CreateStreamOnFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OPC_STREAM_IO_MODE, *const super::super::super::Security::SECURITY_ATTRIBUTES, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security", feature = "Win32_System_Com")))]
    CreateStreamOnFile: usize,
    pub CreatePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReadPackageFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, OPC_READ_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReadPackageFromStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WritePackageToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, OPC_WRITE_FLAGS, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WritePackageToStream: usize,
    pub CreateDigitalSignatureManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcPackage, IOpcPackage_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee70);
impl core::ops::Deref for IOpcPackage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcPackage, windows_core::IUnknown);
impl IOpcPackage {
    pub unsafe fn GetPartSet(&self) -> windows_core::Result<IOpcPartSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationshipSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcPackage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPartSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelationshipSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcPart, IOpcPart_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee71);
impl core::ops::Deref for IOpcPart {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcPart, windows_core::IUnknown);
impl IOpcPart {
    pub unsafe fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationshipSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetName(&self) -> windows_core::Result<IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCompressionOptions(&self) -> windows_core::Result<OPC_COMPRESSION_OPTIONS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCompressionOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpcPart_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRelationshipSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetContentStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetContentStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetName: usize,
    pub GetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetCompressionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_COMPRESSION_OPTIONS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcPartEnumerator, IOpcPartEnumerator_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee75);
impl core::ops::Deref for IOpcPartEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcPartEnumerator, windows_core::IUnknown);
impl IOpcPartEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcPart> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcPartEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcPartEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcPartSet, IOpcPartSet_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee73);
impl core::ops::Deref for IOpcPartSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcPartSet, windows_core::IUnknown);
impl IOpcPartSet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPart<P0>(&self, name: P0) -> windows_core::Result<IOpcPart>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPart)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePart<P0, P1>(&self, name: P0, contenttype: P1, compressionoptions: OPC_COMPRESSION_OPTIONS) -> windows_core::Result<IOpcPart>
    where
        P0: windows_core::Param<IOpcPartUri>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePart)(windows_core::Interface::as_raw(self), name.param().abi(), contenttype.param().abi(), compressionoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeletePart<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).DeletePart)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PartExists<P0>(&self, name: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PartExists)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcPartEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcPartSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, OPC_COMPRESSION_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeletePart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeletePart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PartExists: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PartExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IOpcPartUri, IOpcPartUri_Vtbl, 0x7d3babe7_88b2_46ba_85cb_4203cb016c87);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IOpcPartUri {
    type Target = IOpcUri;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IOpcPartUri, windows_core::IUnknown, super::super::super::System::Com::IUri, IOpcUri);
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartUri {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ComparePartUri<P0>(&self, parturi: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComparePartUri)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> windows_core::Result<IOpcUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsRelationshipsPartUri(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRelationshipsPartUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOpcPartUri_Vtbl {
    pub base__: IOpcUri_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ComparePartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ComparePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    pub IsRelationshipsPartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcRelationship, IOpcRelationship_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee72);
impl core::ops::Deref for IOpcRelationship {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcRelationship, windows_core::IUnknown);
impl IOpcRelationship {
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRelationshipType(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationshipType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> windows_core::Result<IOpcUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTargetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTargetMode(&self) -> windows_core::Result<OPC_URI_TARGET_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpcRelationship_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetRelationshipType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTargetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTargetUri: usize,
    pub GetTargetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_URI_TARGET_MODE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcRelationshipEnumerator, IOpcRelationshipEnumerator_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee76);
impl core::ops::Deref for IOpcRelationshipEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcRelationshipEnumerator, windows_core::IUnknown);
impl IOpcRelationshipEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcRelationship> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcRelationshipEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcRelationshipEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcRelationshipSelector, IOpcRelationshipSelector_Vtbl, 0xf8f26c7f_b28f_4899_84c8_5d5639ede75f);
impl core::ops::Deref for IOpcRelationshipSelector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcRelationshipSelector, windows_core::IUnknown);
impl IOpcRelationshipSelector {
    pub unsafe fn GetSelectorType(&self) -> windows_core::Result<OPC_RELATIONSHIP_SELECTOR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSelectionCriterion(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelectionCriterion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpcRelationshipSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_RELATIONSHIP_SELECTOR) -> windows_core::HRESULT,
    pub GetSelectionCriterion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcRelationshipSelectorEnumerator, IOpcRelationshipSelectorEnumerator_Vtbl, 0x5e50a181_a91b_48ac_88d2_bca3d8f8c0b1);
impl core::ops::Deref for IOpcRelationshipSelectorEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcRelationshipSelectorEnumerator, windows_core::IUnknown);
impl IOpcRelationshipSelectorEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcRelationshipSelector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcRelationshipSelectorEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcRelationshipSelectorSet, IOpcRelationshipSelectorSet_Vtbl, 0x6e34c269_a4d3_47c0_b5c4_87ff2b3b6136);
impl core::ops::Deref for IOpcRelationshipSelectorSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcRelationshipSelectorSet, windows_core::IUnknown);
impl IOpcRelationshipSelectorSet {
    pub unsafe fn Create<P0>(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: P0) -> windows_core::Result<IOpcRelationshipSelector>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), selector, selectioncriterion.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, relationshipselector: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcRelationshipSelector>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), relationshipselector.param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcRelationshipSelectorSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, OPC_RELATIONSHIP_SELECTOR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcRelationshipSet, IOpcRelationshipSet_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee74);
impl core::ops::Deref for IOpcRelationshipSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcRelationshipSet, windows_core::IUnknown);
impl IOpcRelationshipSet {
    pub unsafe fn GetRelationship<P0>(&self, relationshipidentifier: P0) -> windows_core::Result<IOpcRelationship>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationship)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRelationship<P0, P1, P2>(&self, relationshipidentifier: P0, relationshiptype: P1, targeturi: P2, targetmode: OPC_URI_TARGET_MODE) -> windows_core::Result<IOpcRelationship>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::super::System::Com::IUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRelationship)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi(), relationshiptype.param().abi(), targeturi.param().abi(), targetmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRelationship<P0>(&self, relationshipidentifier: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteRelationship)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi()).ok()
    }
    pub unsafe fn RelationshipExists<P0>(&self, relationshipidentifier: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RelationshipExists)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEnumeratorForType<P0>(&self, relationshiptype: P0) -> windows_core::Result<IOpcRelationshipEnumerator>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumeratorForType)(windows_core::Interface::as_raw(self), relationshiptype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationshipsContentStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcRelationshipSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, OPC_URI_TARGET_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRelationship: usize,
    pub DeleteRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RelationshipExists: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumeratorForType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsContentStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsContentStream: usize,
}
windows_core::imp::define_interface!(IOpcSignatureCustomObject, IOpcSignatureCustomObject_Vtbl, 0x5d77a19e_62c1_44e7_becd_45da5ae51a56);
impl core::ops::Deref for IOpcSignatureCustomObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObject, windows_core::IUnknown);
impl IOpcSignatureCustomObject {
    pub unsafe fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetXml)(windows_core::Interface::as_raw(self), xmlmarkup, count).ok()
    }
}
#[repr(C)]
pub struct IOpcSignatureCustomObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureCustomObjectEnumerator, IOpcSignatureCustomObjectEnumerator_Vtbl, 0x5ee4fe1d_e1b0_4683_8079_7ea0fcf80b4c);
impl core::ops::Deref for IOpcSignatureCustomObjectEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObjectEnumerator, windows_core::IUnknown);
impl IOpcSignatureCustomObjectEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureCustomObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureCustomObjectSet, IOpcSignatureCustomObjectSet_Vtbl, 0x8f792ac5_7947_4e11_bc3d_2659ff046ae1);
impl core::ops::Deref for IOpcSignatureCustomObjectSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObjectSet, windows_core::IUnknown);
impl IOpcSignatureCustomObjectSet {
    pub unsafe fn Create(&self, xmlmarkup: &[u8]) -> windows_core::Result<IOpcSignatureCustomObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(xmlmarkup.as_ptr()), xmlmarkup.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, customobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignatureCustomObject>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), customobject.param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignatureCustomObjectSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignaturePartReference, IOpcSignaturePartReference_Vtbl, 0xe24231ca_59f4_484e_b64b_36eeda36072c);
impl core::ops::Deref for IOpcSignaturePartReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignaturePartReference, windows_core::IUnknown);
impl IOpcSignaturePartReference {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPartName(&self) -> windows_core::Result<IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDigestValue)(windows_core::Interface::as_raw(self), digestvalue, count).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpcSignaturePartReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPartName: usize,
    pub GetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignaturePartReferenceEnumerator, IOpcSignaturePartReferenceEnumerator_Vtbl, 0x80eb1561_8c77_49cf_8266_459b356ee99a);
impl core::ops::Deref for IOpcSignaturePartReferenceEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignaturePartReferenceEnumerator, windows_core::IUnknown);
impl IOpcSignaturePartReferenceEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignaturePartReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignaturePartReferenceSet, IOpcSignaturePartReferenceSet_Vtbl, 0x6c9fe28c_ecd9_4b22_9d36_7fdde670fec0);
impl core::ops::Deref for IOpcSignaturePartReferenceSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignaturePartReferenceSet, windows_core::IUnknown);
impl IOpcSignaturePartReferenceSet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1>(&self, parturi: P0, digestmethod: P1, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignaturePartReference>
    where
        P0: windows_core::Param<IOpcPartUri>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), parturi.param().abi(), digestmethod.param().abi(), transformmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, partreference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignaturePartReference>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), partreference.param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignaturePartReferenceSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, OPC_CANONICALIZATION_METHOD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureReference, IOpcSignatureReference_Vtbl, 0x1b47005e_3011_4edc_be6f_0f65e5ab0342);
impl core::ops::Deref for IOpcSignatureReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureReference, windows_core::IUnknown);
impl IOpcSignatureReference {
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDigestValue)(windows_core::Interface::as_raw(self), digestvalue, count).ok()
    }
}
#[repr(C)]
pub struct IOpcSignatureReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUri: usize,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureReferenceEnumerator, IOpcSignatureReferenceEnumerator_Vtbl, 0xcfa59a45_28b1_4868_969e_fa8097fdc12a);
impl core::ops::Deref for IOpcSignatureReferenceEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureReferenceEnumerator, windows_core::IUnknown);
impl IOpcSignatureReferenceEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignatureReferenceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureReferenceSet, IOpcSignatureReferenceSet_Vtbl, 0xf3b02d31_ab12_42dd_9e2f_2b16761c3c1e);
impl core::ops::Deref for IOpcSignatureReferenceSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureReferenceSet, windows_core::IUnknown);
impl IOpcSignatureReferenceSet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1, P2, P3>(&self, referenceuri: P0, referenceid: P1, r#type: P2, digestmethod: P3, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignatureReference>
    where
        P0: windows_core::Param<super::super::super::System::Com::IUri>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), referenceuri.param().abi(), referenceid.param().abi(), r#type.param().abi(), digestmethod.param().abi(), transformmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, reference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignatureReference>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), reference.param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignatureReferenceSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, OPC_CANONICALIZATION_METHOD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureRelationshipReference, IOpcSignatureRelationshipReference_Vtbl, 0x57babac6_9d4a_4e50_8b86_e5d4051eae7c);
impl core::ops::Deref for IOpcSignatureRelationshipReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReference, windows_core::IUnknown);
impl IOpcSignatureRelationshipReference {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> windows_core::Result<IOpcUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDigestValue)(windows_core::Interface::as_raw(self), digestvalue, count).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRelationshipSigningOption(&self) -> windows_core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationshipSigningOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRelationshipSelectorEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationshipSelectorEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignatureRelationshipReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    pub GetDigestMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT,
    pub GetRelationshipSigningOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> windows_core::HRESULT,
    pub GetRelationshipSelectorEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureRelationshipReferenceEnumerator, IOpcSignatureRelationshipReferenceEnumerator_Vtbl, 0x773ba3e4_f021_48e4_aa04_9816db5d3495);
impl core::ops::Deref for IOpcSignatureRelationshipReferenceEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceEnumerator, windows_core::IUnknown);
impl IOpcSignatureRelationshipReferenceEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureRelationshipReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSignatureRelationshipReferenceSet, IOpcSignatureRelationshipReferenceSet_Vtbl, 0x9f863ca5_3631_404c_828d_807e0715069b);
impl core::ops::Deref for IOpcSignatureRelationshipReferenceSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceSet, windows_core::IUnknown);
impl IOpcSignatureRelationshipReferenceSet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1, P2>(&self, sourceuri: P0, digestmethod: P1, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: P2, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignatureRelationshipReference>
    where
        P0: windows_core::Param<IOpcUri>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IOpcRelationshipSelectorSet>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), sourceuri.param().abi(), digestmethod.param().abi(), relationshipsigningoption, selectorset.param().abi(), transformmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRelationshipSelectorSet(&self) -> windows_core::Result<IOpcRelationshipSelectorSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRelationshipSelectorSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, relationshipreference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignatureRelationshipReference>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), relationshipreference.param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpcSignatureRelationshipReferenceSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, OPC_RELATIONSHIPS_SIGNING_OPTION, *mut core::ffi::c_void, OPC_CANONICALIZATION_METHOD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub CreateRelationshipSelectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpcSigningOptions, IOpcSigningOptions_Vtbl, 0x50d2d6a5_7aeb_46c0_b241_43ab0e9b407e);
impl core::ops::Deref for IOpcSigningOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpcSigningOptions, windows_core::IUnknown);
impl IOpcSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSignatureId)(windows_core::Interface::as_raw(self), signatureid.param().abi()).ok()
    }
    pub unsafe fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSignatureMethod)(windows_core::Interface::as_raw(self), signaturemethod.param().abi()).ok()
    }
    pub unsafe fn GetDefaultDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDefaultDigestMethod<P0>(&self, digestmethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefaultDigestMethod)(windows_core::Interface::as_raw(self), digestmethod.param().abi()).ok()
    }
    pub unsafe fn GetCertificateEmbeddingOption(&self) -> windows_core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateEmbeddingOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCertificateEmbeddingOption)(windows_core::Interface::as_raw(self), embeddingoption).ok()
    }
    pub unsafe fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTimeFormat)(windows_core::Interface::as_raw(self), timeformat).ok()
    }
    pub unsafe fn GetSignaturePartReferenceSet(&self) -> windows_core::Result<IOpcSignaturePartReferenceSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignaturePartReferenceSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSignatureRelationshipReferenceSet(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureRelationshipReferenceSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCustomObjectSet(&self) -> windows_core::Result<IOpcSignatureCustomObjectSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomObjectSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCustomReferenceSet(&self) -> windows_core::Result<IOpcSignatureReferenceSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomReferenceSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateSet(&self) -> windows_core::Result<IOpcCertificateSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetSignaturePartName)(windows_core::Interface::as_raw(self), signaturepartname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOpcSigningOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDefaultDigestMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDefaultDigestMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCertificateEmbeddingOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::HRESULT,
    pub SetCertificateEmbeddingOption: unsafe extern "system" fn(*mut core::ffi::c_void, OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT,
    pub SetTimeFormat: unsafe extern "system" fn(*mut core::ffi::c_void, OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT,
    pub GetSignaturePartReferenceSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSignatureRelationshipReferenceSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCustomObjectSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCustomReferenceSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCertificateSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignaturePartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignaturePartName: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IOpcUri, IOpcUri_Vtbl, 0xbc9c1b9b_d62c_49eb_aef0_3b4e0b28ebed);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IOpcUri {
    type Target = super::super::super::System::Com::IUri;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IOpcUri, windows_core::IUnknown, super::super::super::System::Com::IUri);
#[cfg(feature = "Win32_System_Com")]
impl IOpcUri {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsPartUri(&self) -> windows_core::Result<IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationshipsPartUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelativeUri<P0>(&self, targetparturi: P0) -> windows_core::Result<super::super::super::System::Com::IUri>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelativeUri)(windows_core::Interface::as_raw(self), targetparturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CombinePartUri<P0>(&self, relativeuri: P0) -> windows_core::Result<IOpcPartUri>
    where
        P0: windows_core::Param<super::super::super::System::Com::IUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CombinePartUri)(windows_core::Interface::as_raw(self), relativeuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOpcUri_Vtbl {
    pub base__: super::super::super::System::Com::IUri_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsPartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsPartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelativeUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelativeUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CombinePartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CombinePartUri: usize,
}
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = OPC_READ_FLAGS(2i32);
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(1i32);
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(2i32);
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(0i32);
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(0i32);
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(1i32);
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(2i32);
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(2i32);
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(1i32);
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(-1i32);
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(0i32);
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(3i32);
pub const OPC_E_CONFLICTING_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x80510014_u32 as _);
pub const OPC_E_COULD_NOT_RECOVER: windows_core::HRESULT = windows_core::HRESULT(0x8051004E_u32 as _);
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x80510047_u32 as _);
pub const OPC_E_DS_DIGEST_VALUE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8051001A_u32 as _);
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: windows_core::HRESULT = windows_core::HRESULT(0x8051002D_u32 as _);
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x8051001B_u32 as _);
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: windows_core::HRESULT = windows_core::HRESULT(0x80510028_u32 as _);
pub const OPC_E_DS_EXTERNAL_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x8051001E_u32 as _);
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x8051002F_u32 as _);
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: windows_core::HRESULT = windows_core::HRESULT(0x80510022_u32 as _);
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x8051001D_u32 as _);
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80510024_u32 as _);
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: windows_core::HRESULT = windows_core::HRESULT(0x80510023_u32 as _);
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: windows_core::HRESULT = windows_core::HRESULT(0x80510021_u32 as _);
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: windows_core::HRESULT = windows_core::HRESULT(0x8051002B_u32 as _);
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x8051001C_u32 as _);
pub const OPC_E_DS_INVALID_SIGNATURE_XML: windows_core::HRESULT = windows_core::HRESULT(0x8051002A_u32 as _);
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x80510032_u32 as _);
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: windows_core::HRESULT = windows_core::HRESULT(0x80510056_u32 as _);
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x8051002E_u32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: windows_core::HRESULT = windows_core::HRESULT(0x8051002C_u32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: windows_core::HRESULT = windows_core::HRESULT(0x8051001F_u32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_PART: windows_core::HRESULT = windows_core::HRESULT(0x80510020_u32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: windows_core::HRESULT = windows_core::HRESULT(0x80510026_u32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: windows_core::HRESULT = windows_core::HRESULT(0x80510027_u32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80510029_u32 as _);
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: windows_core::HRESULT = windows_core::HRESULT(0x80510031_u32 as _);
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: windows_core::HRESULT = windows_core::HRESULT(0x80510025_u32 as _);
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80510030_u32 as _);
pub const OPC_E_DS_SIGNATURE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80510019_u32 as _);
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x80510046_u32 as _);
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80510054_u32 as _);
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x80510045_u32 as _);
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: windows_core::HRESULT = windows_core::HRESULT(0x80510043_u32 as _);
pub const OPC_E_DS_UNSIGNED_PACKAGE: windows_core::HRESULT = windows_core::HRESULT(0x80510055_u32 as _);
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: windows_core::HRESULT = windows_core::HRESULT(0x8051000F_u32 as _);
pub const OPC_E_DUPLICATE_OVERRIDE_PART: windows_core::HRESULT = windows_core::HRESULT(0x8051000D_u32 as _);
pub const OPC_E_DUPLICATE_PART: windows_core::HRESULT = windows_core::HRESULT(0x8051000B_u32 as _);
pub const OPC_E_DUPLICATE_PIECE: windows_core::HRESULT = windows_core::HRESULT(0x80510015_u32 as _);
pub const OPC_E_DUPLICATE_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x80510013_u32 as _);
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: windows_core::HRESULT = windows_core::HRESULT(0x80510051_u32 as _);
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: windows_core::HRESULT = windows_core::HRESULT(0x80510052_u32 as _);
pub const OPC_E_ENUM_COLLECTION_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x80510050_u32 as _);
pub const OPC_E_ENUM_INVALID_POSITION: windows_core::HRESULT = windows_core::HRESULT(0x80510053_u32 as _);
pub const OPC_E_INVALID_CONTENT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80510044_u32 as _);
pub const OPC_E_INVALID_CONTENT_TYPE_XML: windows_core::HRESULT = windows_core::HRESULT(0x80510006_u32 as _);
pub const OPC_E_INVALID_DEFAULT_EXTENSION: windows_core::HRESULT = windows_core::HRESULT(0x8051000E_u32 as _);
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8051000C_u32 as _);
pub const OPC_E_INVALID_PIECE: windows_core::HRESULT = windows_core::HRESULT(0x80510016_u32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_ID: windows_core::HRESULT = windows_core::HRESULT(0x80510010_u32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x80510012_u32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: windows_core::HRESULT = windows_core::HRESULT(0x8051004D_u32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80510011_u32 as _);
pub const OPC_E_INVALID_RELS_XML: windows_core::HRESULT = windows_core::HRESULT(0x8051000A_u32 as _);
pub const OPC_E_INVALID_XML_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x80510042_u32 as _);
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: windows_core::HRESULT = windows_core::HRESULT(0x8051004B_u32 as _);
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: windows_core::HRESULT = windows_core::HRESULT(0x8051004C_u32 as _);
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: windows_core::HRESULT = windows_core::HRESULT(0x8051004A_u32 as _);
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: windows_core::HRESULT = windows_core::HRESULT(0x80510040_u32 as _);
pub const OPC_E_MC_INVALID_ENUM_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8051003C_u32 as _);
pub const OPC_E_MC_INVALID_PREFIX_LIST: windows_core::HRESULT = windows_core::HRESULT(0x80510037_u32 as _);
pub const OPC_E_MC_INVALID_QNAME_LIST: windows_core::HRESULT = windows_core::HRESULT(0x80510038_u32 as _);
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: windows_core::HRESULT = windows_core::HRESULT(0x80510041_u32 as _);
pub const OPC_E_MC_MISSING_CHOICE: windows_core::HRESULT = windows_core::HRESULT(0x8051003B_u32 as _);
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: windows_core::HRESULT = windows_core::HRESULT(0x80510035_u32 as _);
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: windows_core::HRESULT = windows_core::HRESULT(0x80510049_u32 as _);
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: windows_core::HRESULT = windows_core::HRESULT(0x80510039_u32 as _);
pub const OPC_E_MC_UNEXPECTED_ATTR: windows_core::HRESULT = windows_core::HRESULT(0x80510036_u32 as _);
pub const OPC_E_MC_UNEXPECTED_CHOICE: windows_core::HRESULT = windows_core::HRESULT(0x8051003A_u32 as _);
pub const OPC_E_MC_UNEXPECTED_ELEMENT: windows_core::HRESULT = windows_core::HRESULT(0x80510033_u32 as _);
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: windows_core::HRESULT = windows_core::HRESULT(0x80510034_u32 as _);
pub const OPC_E_MC_UNKNOWN_NAMESPACE: windows_core::HRESULT = windows_core::HRESULT(0x8051003E_u32 as _);
pub const OPC_E_MC_UNKNOWN_PREFIX: windows_core::HRESULT = windows_core::HRESULT(0x8051003F_u32 as _);
pub const OPC_E_MISSING_CONTENT_TYPES: windows_core::HRESULT = windows_core::HRESULT(0x80510007_u32 as _);
pub const OPC_E_MISSING_PIECE: windows_core::HRESULT = windows_core::HRESULT(0x80510017_u32 as _);
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: windows_core::HRESULT = windows_core::HRESULT(0x80510008_u32 as _);
pub const OPC_E_NONCONFORMING_RELS_XML: windows_core::HRESULT = windows_core::HRESULT(0x80510009_u32 as _);
pub const OPC_E_NONCONFORMING_URI: windows_core::HRESULT = windows_core::HRESULT(0x80510001_u32 as _);
pub const OPC_E_NO_SUCH_PART: windows_core::HRESULT = windows_core::HRESULT(0x80510018_u32 as _);
pub const OPC_E_NO_SUCH_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x80510048_u32 as _);
pub const OPC_E_NO_SUCH_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x80510057_u32 as _);
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: windows_core::HRESULT = windows_core::HRESULT(0x80510004_u32 as _);
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80510003_u32 as _);
pub const OPC_E_RELATIVE_URI_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80510002_u32 as _);
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80510005_u32 as _);
pub const OPC_E_UNSUPPORTED_PACKAGE: windows_core::HRESULT = windows_core::HRESULT(0x8051004F_u32 as _);
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80511009_u32 as _);
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x8051100C_u32 as _);
pub const OPC_E_ZIP_COMPRESSION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80511003_u32 as _);
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: windows_core::HRESULT = windows_core::HRESULT(0x80511002_u32 as _);
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80511004_u32 as _);
pub const OPC_E_ZIP_DUPLICATE_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8051100B_u32 as _);
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x8051100D_u32 as _);
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x8051100E_u32 as _);
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: windows_core::HRESULT = windows_core::HRESULT(0x80511006_u32 as _);
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: windows_core::HRESULT = windows_core::HRESULT(0x80511005_u32 as _);
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80511001_u32 as _);
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: windows_core::HRESULT = windows_core::HRESULT(0x80511007_u32 as _);
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: windows_core::HRESULT = windows_core::HRESULT(0x8051100F_u32 as _);
pub const OPC_E_ZIP_NAME_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x8051100A_u32 as _);
pub const OPC_E_ZIP_REQUIRES_64_BIT: windows_core::HRESULT = windows_core::HRESULT(0x80511010_u32 as _);
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: windows_core::HRESULT = windows_core::HRESULT(0x80511008_u32 as _);
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = OPC_READ_FLAGS(0i32);
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(0i32);
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(1i32);
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(1i32);
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(0i32);
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(-1i32);
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(3i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(0i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(2i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(4i32);
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(1i32);
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(5i32);
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(0i32);
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(1i32);
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(2i32);
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(1i32);
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(0i32);
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = OPC_READ_FLAGS(1i32);
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(0i32);
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_CANONICALIZATION_METHOD(pub i32);
impl windows_core::TypeKind for OPC_CANONICALIZATION_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_CANONICALIZATION_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_CANONICALIZATION_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_CERTIFICATE_EMBEDDING_OPTION(pub i32);
impl windows_core::TypeKind for OPC_CERTIFICATE_EMBEDDING_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_CERTIFICATE_EMBEDDING_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_CERTIFICATE_EMBEDDING_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_COMPRESSION_OPTIONS(pub i32);
impl windows_core::TypeKind for OPC_COMPRESSION_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_COMPRESSION_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_COMPRESSION_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_READ_FLAGS(pub i32);
impl windows_core::TypeKind for OPC_READ_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_READ_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_READ_FLAGS").field(&self.0).finish()
    }
}
impl OPC_READ_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPC_READ_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPC_READ_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPC_READ_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPC_READ_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPC_READ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_RELATIONSHIPS_SIGNING_OPTION(pub i32);
impl windows_core::TypeKind for OPC_RELATIONSHIPS_SIGNING_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_RELATIONSHIPS_SIGNING_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_RELATIONSHIPS_SIGNING_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_RELATIONSHIP_SELECTOR(pub i32);
impl windows_core::TypeKind for OPC_RELATIONSHIP_SELECTOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_RELATIONSHIP_SELECTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_RELATIONSHIP_SELECTOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_SIGNATURE_TIME_FORMAT(pub i32);
impl windows_core::TypeKind for OPC_SIGNATURE_TIME_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_SIGNATURE_TIME_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_SIGNATURE_TIME_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_SIGNATURE_VALIDATION_RESULT(pub i32);
impl windows_core::TypeKind for OPC_SIGNATURE_VALIDATION_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_SIGNATURE_VALIDATION_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_SIGNATURE_VALIDATION_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_STREAM_IO_MODE(pub i32);
impl windows_core::TypeKind for OPC_STREAM_IO_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_STREAM_IO_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_STREAM_IO_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_URI_TARGET_MODE(pub i32);
impl windows_core::TypeKind for OPC_URI_TARGET_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_URI_TARGET_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_URI_TARGET_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPC_WRITE_FLAGS(pub i32);
impl windows_core::TypeKind for OPC_WRITE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPC_WRITE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPC_WRITE_FLAGS").field(&self.0).finish()
    }
}
impl OPC_WRITE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPC_WRITE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPC_WRITE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPC_WRITE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const OpcFactory: windows_core::GUID = windows_core::GUID::from_u128(0x6b2d6ba0_9f3e_4f27_920b_313cc426a39e);
#[cfg(feature = "implement")]
core::include!("impl.rs");
