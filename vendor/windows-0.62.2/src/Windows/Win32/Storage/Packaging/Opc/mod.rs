windows_core::imp::define_interface!(IOpcCertificateEnumerator, IOpcCertificateEnumerator_Vtbl, 0x85131937_8f24_421f_b439_59ab24d140b8);
windows_core::imp::interface_hierarchy!(IOpcCertificateEnumerator, windows_core::IUnknown);
impl IOpcCertificateEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<*mut super::super::super::Security::Cryptography::CERT_CONTEXT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcCertificateEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    GetCurrent: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Security_Cryptography")]
pub trait IOpcCertificateEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<*mut super::super::super::Security::Cryptography::CERT_CONTEXT>;
    fn Clone(&self) -> windows_core::Result<IOpcCertificateEnumerator>;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl IOpcCertificateEnumerator_Vtbl {
    pub const fn new<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcCertificateEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcCertificateEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *mut *mut super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcCertificateEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        certificate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcCertificateEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcCertificateEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::RuntimeName for IOpcCertificateEnumerator {}
windows_core::imp::define_interface!(IOpcCertificateSet, IOpcCertificateSet_Vtbl, 0x56ea4325_8e2d_4167_b1a4_e486d24c8fa7);
windows_core::imp::interface_hierarchy!(IOpcCertificateSet, windows_core::IUnknown);
impl IOpcCertificateSet {
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), certificate).ok() }
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), certificate).ok() }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Security_Cryptography")]
pub trait IOpcCertificateSet_Impl: windows_core::IUnknownImpl {
    fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()>;
    fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator>;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl IOpcCertificateSet_Vtbl {
    pub const fn new<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcCertificateSet_Impl::Add(this, core::mem::transmute_copy(&certificate)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcCertificateSet_Impl::Remove(this, core::mem::transmute_copy(&certificate)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcCertificateSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        certificateenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcCertificateSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::RuntimeName for IOpcCertificateSet {}
windows_core::imp::define_interface!(IOpcDigitalSignature, IOpcDigitalSignature_Vtbl, 0x52ab21dd_1cd0_4949_bc80_0c1232d00cb4);
windows_core::imp::interface_hierarchy!(IOpcDigitalSignature, windows_core::IUnknown);
impl IOpcDigitalSignature {
    pub unsafe fn GetNamespaces(&self, prefixes: *mut *mut windows_core::PWSTR, namespaces: *mut *mut windows_core::PWSTR, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNamespaces)(windows_core::Interface::as_raw(self), prefixes as _, namespaces as _, count as _).ok() }
    }
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCanonicalizationMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCanonicalizationMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSignatureValue)(windows_core::Interface::as_raw(self), signaturevalue as _, count as _).ok() }
    }
    pub unsafe fn GetSignaturePartReferenceEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignaturePartReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSignatureRelationshipReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureRelationshipReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPackageObjectReference(&self) -> windows_core::Result<IOpcSignatureReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageObjectReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCertificateEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCustomObjectEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomObjectEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSignatureXml)(windows_core::Interface::as_raw(self), signaturexml as _, count as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcDigitalSignature_Impl: windows_core::IUnknownImpl {
    fn GetNamespaces(&self, prefixes: *mut *mut windows_core::PWSTR, namespaces: *mut *mut windows_core::PWSTR, count: *mut u32) -> windows_core::Result<()>;
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetCanonicalizationMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetSignaturePartReferenceEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
    fn GetSignatureRelationshipReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
    fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn GetPackageObjectReference(&self) -> windows_core::Result<IOpcSignatureReference>;
    fn GetCertificateEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator>;
    fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator>;
    fn GetCustomObjectEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
    fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcDigitalSignature_Vtbl {
    pub const fn new<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNamespaces<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefixes: *mut *mut windows_core::PWSTR, namespaces: *mut *mut windows_core::PWSTR, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcDigitalSignature_Impl::GetNamespaces(this, core::mem::transmute_copy(&prefixes), core::mem::transmute_copy(&namespaces), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetSignatureId<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetSignatureId(this) {
                    Ok(ok__) => {
                        signatureid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetSignaturePartName(this) {
                    Ok(ok__) => {
                        signaturepartname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetSignatureMethod(this) {
                    Ok(ok__) => {
                        signaturemethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCanonicalizationMethod<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetCanonicalizationMethod(this) {
                    Ok(ok__) => {
                        canonicalizationmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureValue<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcDigitalSignature_Impl::GetSignatureValue(this, core::mem::transmute_copy(&signaturevalue), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetSignaturePartReferenceEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetSignaturePartReferenceEnumerator(this) {
                    Ok(ok__) => {
                        partreferenceenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetSignatureRelationshipReferenceEnumerator(this) {
                    Ok(ok__) => {
                        relationshipreferenceenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSigningTime<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signingtime: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetSigningTime(this) {
                    Ok(ok__) => {
                        signingtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimeFormat<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetTimeFormat(this) {
                    Ok(ok__) => {
                        timeformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageObjectReference<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageobjectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetPackageObjectReference(this) {
                    Ok(ok__) => {
                        packageobjectreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetCertificateEnumerator(this) {
                    Ok(ok__) => {
                        certificateenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetCustomReferenceEnumerator(this) {
                    Ok(ok__) => {
                        customreferenceenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignature_Impl::GetCustomObjectEnumerator(this) {
                    Ok(ok__) => {
                        customobjectenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureXml<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcDigitalSignature_Impl::GetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamespaces: GetNamespaces::<Identity, OFFSET>,
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, OFFSET>,
            GetCanonicalizationMethod: GetCanonicalizationMethod::<Identity, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, OFFSET>,
            GetSignaturePartReferenceEnumerator: GetSignaturePartReferenceEnumerator::<Identity, OFFSET>,
            GetSignatureRelationshipReferenceEnumerator: GetSignatureRelationshipReferenceEnumerator::<Identity, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, OFFSET>,
            GetPackageObjectReference: GetPackageObjectReference::<Identity, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcDigitalSignature as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcDigitalSignature {}
windows_core::imp::define_interface!(IOpcDigitalSignatureEnumerator, IOpcDigitalSignatureEnumerator_Vtbl, 0x967b6882_0ba3_4358_b9e7_b64c75063c5e);
windows_core::imp::interface_hierarchy!(IOpcDigitalSignatureEnumerator, windows_core::IUnknown);
impl IOpcDigitalSignatureEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcDigitalSignature> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcDigitalSignatureEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcDigitalSignature>;
    fn Clone(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator>;
}
impl IOpcDigitalSignatureEnumerator_Vtbl {
    pub const fn new<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digitalsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        digitalsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcDigitalSignatureEnumerator {}
windows_core::imp::define_interface!(IOpcDigitalSignatureManager, IOpcDigitalSignatureManager_Vtbl, 0xd5e62a0b_696d_462f_94df_72e33cef2659);
windows_core::imp::interface_hierarchy!(IOpcDigitalSignatureManager, windows_core::IUnknown);
impl IOpcDigitalSignatureManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignatureOriginPartName(&self) -> windows_core::Result<IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureOriginPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignatureOriginPartName)(windows_core::Interface::as_raw(self), signatureoriginpartname.param().abi()).ok() }
    }
    pub unsafe fn GetSignatureEnumerator(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveSignature<P0>(&self, signaturepartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveSignature)(windows_core::Interface::as_raw(self), signaturepartname.param().abi()).ok() }
    }
    pub unsafe fn CreateSigningOptions(&self) -> windows_core::Result<IOpcSigningOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSigningOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Validate<P0>(&self, signature: P0, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<OPC_SIGNATURE_VALIDATION_RESULT>
    where
        P0: windows_core::Param<IOpcDigitalSignature>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), signature.param().abi(), certificate, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Sign<P1>(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: P1) -> windows_core::Result<IOpcDigitalSignature>
    where
        P1: windows_core::Param<IOpcSigningOptions>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sign)(windows_core::Interface::as_raw(self), certificate, signingoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReplaceSignatureXml<P0>(&self, signaturepartname: P0, newsignaturexml: &[u8]) -> windows_core::Result<IOpcDigitalSignature>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReplaceSignatureXml)(windows_core::Interface::as_raw(self), signaturepartname.param().abi(), core::mem::transmute(newsignaturexml.as_ptr()), newsignaturexml.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
pub trait IOpcDigitalSignatureManager_Impl: windows_core::IUnknownImpl {
    fn GetSignatureOriginPartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn SetSignatureOriginPartName(&self, signatureoriginpartname: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<()>;
    fn GetSignatureEnumerator(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator>;
    fn RemoveSignature(&self, signaturepartname: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<()>;
    fn CreateSigningOptions(&self) -> windows_core::Result<IOpcSigningOptions>;
    fn Validate(&self, signature: windows_core::Ref<IOpcDigitalSignature>, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<OPC_SIGNATURE_VALIDATION_RESULT>;
    fn Sign(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: windows_core::Ref<IOpcSigningOptions>) -> windows_core::Result<IOpcDigitalSignature>;
    fn ReplaceSignatureXml(&self, signaturepartname: windows_core::Ref<IOpcPartUri>, newsignaturexml: *const u8, count: u32) -> windows_core::Result<IOpcDigitalSignature>;
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl IOpcDigitalSignatureManager_Vtbl {
    pub const fn new<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureManager_Impl::GetSignatureOriginPartName(this) {
                    Ok(ok__) => {
                        signatureoriginpartname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcDigitalSignatureManager_Impl::SetSignatureOriginPartName(this, core::mem::transmute_copy(&signatureoriginpartname)).into()
            }
        }
        unsafe extern "system" fn GetSignatureEnumerator<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureManager_Impl::GetSignatureEnumerator(this) {
                    Ok(ok__) => {
                        signatureenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSignature<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcDigitalSignatureManager_Impl::RemoveSignature(this, core::mem::transmute_copy(&signaturepartname)).into()
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signingoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureManager_Impl::CreateSigningOptions(this) {
                    Ok(ok__) => {
                        signingoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Validate<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureManager_Impl::Validate(this, core::mem::transmute_copy(&signature), core::mem::transmute_copy(&certificate)) {
                    Ok(ok__) => {
                        validationresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Sign<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: *mut core::ffi::c_void, digitalsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureManager_Impl::Sign(this, core::mem::transmute_copy(&certificate), core::mem::transmute_copy(&signingoptions)) {
                    Ok(ok__) => {
                        digitalsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReplaceSignatureXml<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void, newsignaturexml: *const u8, count: u32, digitalsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcDigitalSignatureManager_Impl::ReplaceSignatureXml(this, core::mem::transmute_copy(&signaturepartname), core::mem::transmute_copy(&newsignaturexml), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        digitalsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, OFFSET>,
            GetSignatureEnumerator: GetSignatureEnumerator::<Identity, OFFSET>,
            RemoveSignature: RemoveSignature::<Identity, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
            Sign: Sign::<Identity, OFFSET>,
            ReplaceSignatureXml: ReplaceSignatureXml::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IOpcDigitalSignatureManager {}
windows_core::imp::define_interface!(IOpcFactory, IOpcFactory_Vtbl, 0x6d0b4446_cd73_4ab3_94f4_8ccdf6116154);
windows_core::imp::interface_hierarchy!(IOpcFactory, windows_core::IUnknown);
impl IOpcFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageRootUri(&self) -> windows_core::Result<IOpcUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageRootUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePartUri<P0>(&self, pwzuri: P0) -> windows_core::Result<IOpcPartUri>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePartUri)(windows_core::Interface::as_raw(self), pwzuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStreamOnFile<P0>(&self, filename: P0, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> windows_core::Result<super::super::super::System::Com::IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStreamOnFile)(windows_core::Interface::as_raw(self), filename.param().abi(), iomode, securityattributes, dwflagsandattributes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePackage(&self) -> windows_core::Result<IOpcPackage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReadPackageFromStream<P0>(&self, stream: P0, flags: OPC_READ_FLAGS) -> windows_core::Result<IOpcPackage>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadPackageFromStream)(windows_core::Interface::as_raw(self), stream.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WritePackageToStream<P0, P2>(&self, package: P0, flags: OPC_WRITE_FLAGS, stream: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPackage>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WritePackageToStream)(windows_core::Interface::as_raw(self), package.param().abi(), flags, stream.param().abi()).ok() }
    }
    pub unsafe fn CreateDigitalSignatureManager<P0>(&self, package: P0) -> windows_core::Result<IOpcDigitalSignatureManager>
    where
        P0: windows_core::Param<IOpcPackage>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDigitalSignatureManager)(windows_core::Interface::as_raw(self), package.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
pub trait IOpcFactory_Impl: windows_core::IUnknownImpl {
    fn CreatePackageRootUri(&self) -> windows_core::Result<IOpcUri>;
    fn CreatePartUri(&self, pwzuri: &windows_core::PCWSTR) -> windows_core::Result<IOpcPartUri>;
    fn CreateStreamOnFile(&self, filename: &windows_core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn CreatePackage(&self) -> windows_core::Result<IOpcPackage>;
    fn ReadPackageFromStream(&self, stream: windows_core::Ref<super::super::super::System::Com::IStream>, flags: OPC_READ_FLAGS) -> windows_core::Result<IOpcPackage>;
    fn WritePackageToStream(&self, package: windows_core::Ref<IOpcPackage>, flags: OPC_WRITE_FLAGS, stream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CreateDigitalSignatureManager(&self, package: windows_core::Ref<IOpcPackage>) -> windows_core::Result<IOpcDigitalSignatureManager>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
impl IOpcFactory_Vtbl {
    pub const fn new<Identity: IOpcFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePackageRootUri<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rooturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcFactory_Impl::CreatePackageRootUri(this) {
                    Ok(ok__) => {
                        rooturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePartUri<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzuri: windows_core::PCWSTR, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcFactory_Impl::CreatePartUri(this, core::mem::transmute(&pwzuri)) {
                    Ok(ok__) => {
                        parturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStreamOnFile<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcFactory_Impl::CreateStreamOnFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&iomode), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&dwflagsandattributes)) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackage<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcFactory_Impl::CreatePackage(this) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadPackageFromStream<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, flags: OPC_READ_FLAGS, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcFactory_Impl::ReadPackageFromStream(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WritePackageToStream<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut core::ffi::c_void, flags: OPC_WRITE_FLAGS, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcFactory_Impl::WritePackageToStream(this, core::mem::transmute_copy(&package), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&stream)).into()
            }
        }
        unsafe extern "system" fn CreateDigitalSignatureManager<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut core::ffi::c_void, signaturemanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcFactory_Impl::CreateDigitalSignatureManager(this, core::mem::transmute_copy(&package)) {
                    Ok(ok__) => {
                        signaturemanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackageRootUri: CreatePackageRootUri::<Identity, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, OFFSET>,
            CreateStreamOnFile: CreateStreamOnFile::<Identity, OFFSET>,
            CreatePackage: CreatePackage::<Identity, OFFSET>,
            ReadPackageFromStream: ReadPackageFromStream::<Identity, OFFSET>,
            WritePackageToStream: WritePackageToStream::<Identity, OFFSET>,
            CreateDigitalSignatureManager: CreateDigitalSignatureManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IOpcFactory {}
windows_core::imp::define_interface!(IOpcPackage, IOpcPackage_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee70);
windows_core::imp::interface_hierarchy!(IOpcPackage, windows_core::IUnknown);
impl IOpcPackage {
    pub unsafe fn GetPartSet(&self) -> windows_core::Result<IOpcPartSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationshipSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPackage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPartSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelationshipSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcPackage_Impl: windows_core::IUnknownImpl {
    fn GetPartSet(&self) -> windows_core::Result<IOpcPartSet>;
    fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet>;
}
impl IOpcPackage_Vtbl {
    pub const fn new<Identity: IOpcPackage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPartSet<Identity: IOpcPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPackage_Impl::GetPartSet(this) {
                    Ok(ok__) => {
                        partset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelationshipSet<Identity: IOpcPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPackage_Impl::GetRelationshipSet(this) {
                    Ok(ok__) => {
                        relationshipset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartSet: GetPartSet::<Identity, OFFSET>,
            GetRelationshipSet: GetRelationshipSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPackage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcPackage {}
windows_core::imp::define_interface!(IOpcPart, IOpcPart_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee71);
windows_core::imp::interface_hierarchy!(IOpcPart, windows_core::IUnknown);
impl IOpcPart {
    pub unsafe fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationshipSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetName(&self) -> windows_core::Result<IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCompressionOptions(&self) -> windows_core::Result<OPC_COMPRESSION_OPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCompressionOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPart_Impl: windows_core::IUnknownImpl {
    fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet>;
    fn GetContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn GetName(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetCompressionOptions(&self) -> windows_core::Result<OPC_COMPRESSION_OPTIONS>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPart_Vtbl {
    pub const fn new<Identity: IOpcPart_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRelationshipSet<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPart_Impl::GetRelationshipSet(this) {
                    Ok(ok__) => {
                        relationshipset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContentStream<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPart_Impl::GetContentStream(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPart_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContentType<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPart_Impl::GetContentType(this) {
                    Ok(ok__) => {
                        contenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCompressionOptions<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPart_Impl::GetCompressionOptions(this) {
                    Ok(ok__) => {
                        compressionoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRelationshipSet: GetRelationshipSet::<Identity, OFFSET>,
            GetContentStream: GetContentStream::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            GetCompressionOptions: GetCompressionOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPart as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcPart {}
windows_core::imp::define_interface!(IOpcPartEnumerator, IOpcPartEnumerator_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee75);
windows_core::imp::interface_hierarchy!(IOpcPartEnumerator, windows_core::IUnknown);
impl IOpcPartEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcPart> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcPartEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcPartEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcPart>;
    fn Clone(&self) -> windows_core::Result<IOpcPartEnumerator>;
}
impl IOpcPartEnumerator_Vtbl {
    pub const fn new<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        part.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPartEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcPartEnumerator {}
windows_core::imp::define_interface!(IOpcPartSet, IOpcPartSet_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee73);
windows_core::imp::interface_hierarchy!(IOpcPartSet, windows_core::IUnknown);
impl IOpcPartSet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPart<P0>(&self, name: P0) -> windows_core::Result<IOpcPart>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPart)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePart<P0, P1>(&self, name: P0, contenttype: P1, compressionoptions: OPC_COMPRESSION_OPTIONS) -> windows_core::Result<IOpcPart>
    where
        P0: windows_core::Param<IOpcPartUri>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePart)(windows_core::Interface::as_raw(self), name.param().abi(), contenttype.param().abi(), compressionoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeletePart<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeletePart)(windows_core::Interface::as_raw(self), name.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PartExists<P0>(&self, name: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PartExists)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcPartEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub PartExists: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PartExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPartSet_Impl: windows_core::IUnknownImpl {
    fn GetPart(&self, name: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<IOpcPart>;
    fn CreatePart(&self, name: windows_core::Ref<IOpcPartUri>, contenttype: &windows_core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS) -> windows_core::Result<IOpcPart>;
    fn DeletePart(&self, name: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<()>;
    fn PartExists(&self, name: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<windows_core::BOOL>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcPartEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartSet_Vtbl {
    pub const fn new<Identity: IOpcPartSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPart<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartSet_Impl::GetPart(this, core::mem::transmute_copy(&name)) {
                    Ok(ok__) => {
                        part.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePart<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, contenttype: windows_core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartSet_Impl::CreatePart(this, core::mem::transmute_copy(&name), core::mem::transmute(&contenttype), core::mem::transmute_copy(&compressionoptions)) {
                    Ok(ok__) => {
                        part.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeletePart<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcPartSet_Impl::DeletePart(this, core::mem::transmute_copy(&name)).into()
            }
        }
        unsafe extern "system" fn PartExists<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, partexists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartSet_Impl::PartExists(this, core::mem::transmute_copy(&name)) {
                    Ok(ok__) => {
                        partexists.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        partenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPart: GetPart::<Identity, OFFSET>,
            CreatePart: CreatePart::<Identity, OFFSET>,
            DeletePart: DeletePart::<Identity, OFFSET>,
            PartExists: PartExists::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPartSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcPartSet {}
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
    pub unsafe fn ComparePartUri<P0>(&self, parturi: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComparePartUri)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSourceUri(&self) -> windows_core::Result<IOpcUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsRelationshipsPartUri(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRelationshipsPartUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartUri_Vtbl {
    pub base__: IOpcUri_Vtbl,
    pub ComparePartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRelationshipsPartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPartUri_Impl: IOpcUri_Impl {
    fn ComparePartUri(&self, parturi: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<i32>;
    fn GetSourceUri(&self) -> windows_core::Result<IOpcUri>;
    fn IsRelationshipsPartUri(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartUri_Vtbl {
    pub const fn new<Identity: IOpcPartUri_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ComparePartUri<Identity: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, comparisonresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartUri_Impl::ComparePartUri(this, core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        comparisonresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceUri<Identity: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartUri_Impl::GetSourceUri(this) {
                    Ok(ok__) => {
                        sourceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRelationshipsPartUri<Identity: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isrelationshipuri: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcPartUri_Impl::IsRelationshipsPartUri(this) {
                    Ok(ok__) => {
                        isrelationshipuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IOpcUri_Vtbl::new::<Identity, OFFSET>(),
            ComparePartUri: ComparePartUri::<Identity, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, OFFSET>,
            IsRelationshipsPartUri: IsRelationshipsPartUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPartUri as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IUri as windows_core::Interface>::IID || iid == &<IOpcUri as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcPartUri {}
windows_core::imp::define_interface!(IOpcRelationship, IOpcRelationship_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee72);
windows_core::imp::interface_hierarchy!(IOpcRelationship, windows_core::IUnknown);
impl IOpcRelationship {
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRelationshipType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationshipType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> windows_core::Result<IOpcUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTargetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTargetMode(&self) -> windows_core::Result<OPC_URI_TARGET_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcRelationship_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRelationshipType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSourceUri(&self) -> windows_core::Result<IOpcUri>;
    fn GetTargetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetTargetMode(&self) -> windows_core::Result<OPC_URI_TARGET_MODE>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcRelationship_Vtbl {
    pub const fn new<Identity: IOpcRelationship_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationship_Impl::GetId(this) {
                    Ok(ok__) => {
                        relationshipidentifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelationshipType<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshiptype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationship_Impl::GetRelationshipType(this) {
                    Ok(ok__) => {
                        relationshiptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceUri<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationship_Impl::GetSourceUri(this) {
                    Ok(ok__) => {
                        sourceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetUri<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationship_Impl::GetTargetUri(this) {
                    Ok(ok__) => {
                        targeturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetMode<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationship_Impl::GetTargetMode(this) {
                    Ok(ok__) => {
                        targetmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetRelationshipType: GetRelationshipType::<Identity, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, OFFSET>,
            GetTargetUri: GetTargetUri::<Identity, OFFSET>,
            GetTargetMode: GetTargetMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationship as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcRelationship {}
windows_core::imp::define_interface!(IOpcRelationshipEnumerator, IOpcRelationshipEnumerator_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee76);
windows_core::imp::interface_hierarchy!(IOpcRelationshipEnumerator, windows_core::IUnknown);
impl IOpcRelationshipEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcRelationship> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcRelationshipEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcRelationshipEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcRelationship>;
    fn Clone(&self) -> windows_core::Result<IOpcRelationshipEnumerator>;
}
impl IOpcRelationshipEnumerator_Vtbl {
    pub const fn new<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        relationship.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcRelationshipEnumerator {}
windows_core::imp::define_interface!(IOpcRelationshipSelector, IOpcRelationshipSelector_Vtbl, 0xf8f26c7f_b28f_4899_84c8_5d5639ede75f);
windows_core::imp::interface_hierarchy!(IOpcRelationshipSelector, windows_core::IUnknown);
impl IOpcRelationshipSelector {
    pub unsafe fn GetSelectorType(&self) -> windows_core::Result<OPC_RELATIONSHIP_SELECTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSelectionCriterion(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectionCriterion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPC_RELATIONSHIP_SELECTOR) -> windows_core::HRESULT,
    pub GetSelectionCriterion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IOpcRelationshipSelector_Impl: windows_core::IUnknownImpl {
    fn GetSelectorType(&self) -> windows_core::Result<OPC_RELATIONSHIP_SELECTOR>;
    fn GetSelectionCriterion(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IOpcRelationshipSelector_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelectorType<Identity: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelector_Impl::GetSelectorType(this) {
                    Ok(ok__) => {
                        selector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelectionCriterion<Identity: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectioncriterion: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelector_Impl::GetSelectionCriterion(this) {
                    Ok(ok__) => {
                        selectioncriterion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelectorType: GetSelectorType::<Identity, OFFSET>,
            GetSelectionCriterion: GetSelectionCriterion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSelector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcRelationshipSelector {}
windows_core::imp::define_interface!(IOpcRelationshipSelectorEnumerator, IOpcRelationshipSelectorEnumerator_Vtbl, 0x5e50a181_a91b_48ac_88d2_bca3d8f8c0b1);
windows_core::imp::interface_hierarchy!(IOpcRelationshipSelectorEnumerator, windows_core::IUnknown);
impl IOpcRelationshipSelectorEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcRelationshipSelector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcRelationshipSelectorEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcRelationshipSelector>;
    fn Clone(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
impl IOpcRelationshipSelectorEnumerator_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelectorEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelectorEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipselector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelectorEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        relationshipselector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelectorEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcRelationshipSelectorEnumerator {}
windows_core::imp::define_interface!(IOpcRelationshipSelectorSet, IOpcRelationshipSelectorSet_Vtbl, 0x6e34c269_a4d3_47c0_b5c4_87ff2b3b6136);
windows_core::imp::interface_hierarchy!(IOpcRelationshipSelectorSet, windows_core::IUnknown);
impl IOpcRelationshipSelectorSet {
    pub unsafe fn Create<P1>(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: P1) -> windows_core::Result<IOpcRelationshipSelector>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), selector, selectioncriterion.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P0>(&self, relationshipselector: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcRelationshipSelector>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), relationshipselector.param().abi()).ok() }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, OPC_RELATIONSHIP_SELECTOR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcRelationshipSelectorSet_Impl: windows_core::IUnknownImpl {
    fn Create(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: &windows_core::PCWSTR) -> windows_core::Result<IOpcRelationshipSelector>;
    fn Delete(&self, relationshipselector: windows_core::Ref<IOpcRelationshipSelector>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
impl IOpcRelationshipSelectorSet_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: windows_core::PCWSTR, relationshipselector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelectorSet_Impl::Create(this, core::mem::transmute_copy(&selector), core::mem::transmute(&selectioncriterion)) {
                    Ok(ok__) => {
                        relationshipselector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipselector: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcRelationshipSelectorSet_Impl::Delete(this, core::mem::transmute_copy(&relationshipselector)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipselectorenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSelectorSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        relationshipselectorenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorSet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcRelationshipSelectorSet {}
windows_core::imp::define_interface!(IOpcRelationshipSet, IOpcRelationshipSet_Vtbl, 0x42195949_3b79_4fc8_89c6_fc7fb979ee74);
windows_core::imp::interface_hierarchy!(IOpcRelationshipSet, windows_core::IUnknown);
impl IOpcRelationshipSet {
    pub unsafe fn GetRelationship<P0>(&self, relationshipidentifier: P0) -> windows_core::Result<IOpcRelationship>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationship)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRelationship<P0, P1, P2>(&self, relationshipidentifier: P0, relationshiptype: P1, targeturi: P2, targetmode: OPC_URI_TARGET_MODE) -> windows_core::Result<IOpcRelationship>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::super::System::Com::IUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRelationship)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi(), relationshiptype.param().abi(), targeturi.param().abi(), targetmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteRelationship<P0>(&self, relationshipidentifier: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteRelationship)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi()).ok() }
    }
    pub unsafe fn RelationshipExists<P0>(&self, relationshipidentifier: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelationshipExists)(windows_core::Interface::as_raw(self), relationshipidentifier.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEnumeratorForType<P0>(&self, relationshiptype: P0) -> windows_core::Result<IOpcRelationshipEnumerator>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumeratorForType)(windows_core::Interface::as_raw(self), relationshiptype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationshipsContentStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, OPC_URI_TARGET_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRelationship: usize,
    pub DeleteRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RelationshipExists: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumeratorForType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsContentStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsContentStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcRelationshipSet_Impl: windows_core::IUnknownImpl {
    fn GetRelationship(&self, relationshipidentifier: &windows_core::PCWSTR) -> windows_core::Result<IOpcRelationship>;
    fn CreateRelationship(&self, relationshipidentifier: &windows_core::PCWSTR, relationshiptype: &windows_core::PCWSTR, targeturi: windows_core::Ref<super::super::super::System::Com::IUri>, targetmode: OPC_URI_TARGET_MODE) -> windows_core::Result<IOpcRelationship>;
    fn DeleteRelationship(&self, relationshipidentifier: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RelationshipExists(&self, relationshipidentifier: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipEnumerator>;
    fn GetEnumeratorForType(&self, relationshiptype: &windows_core::PCWSTR) -> windows_core::Result<IOpcRelationshipEnumerator>;
    fn GetRelationshipsContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcRelationshipSet_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRelationship<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR, relationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSet_Impl::GetRelationship(this, core::mem::transmute(&relationshipidentifier)) {
                    Ok(ok__) => {
                        relationship.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRelationship<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR, relationshiptype: windows_core::PCWSTR, targeturi: *mut core::ffi::c_void, targetmode: OPC_URI_TARGET_MODE, relationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSet_Impl::CreateRelationship(this, core::mem::transmute(&relationshipidentifier), core::mem::transmute(&relationshiptype), core::mem::transmute_copy(&targeturi), core::mem::transmute_copy(&targetmode)) {
                    Ok(ok__) => {
                        relationship.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRelationship<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcRelationshipSet_Impl::DeleteRelationship(this, core::mem::transmute(&relationshipidentifier)).into()
            }
        }
        unsafe extern "system" fn RelationshipExists<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR, relationshipexists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSet_Impl::RelationshipExists(this, core::mem::transmute(&relationshipidentifier)) {
                    Ok(ok__) => {
                        relationshipexists.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        relationshipenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnumeratorForType<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshiptype: windows_core::PCWSTR, relationshipenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSet_Impl::GetEnumeratorForType(this, core::mem::transmute(&relationshiptype)) {
                    Ok(ok__) => {
                        relationshipenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelationshipsContentStream<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcRelationshipSet_Impl::GetRelationshipsContentStream(this) {
                    Ok(ok__) => {
                        contents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRelationship: GetRelationship::<Identity, OFFSET>,
            CreateRelationship: CreateRelationship::<Identity, OFFSET>,
            DeleteRelationship: DeleteRelationship::<Identity, OFFSET>,
            RelationshipExists: RelationshipExists::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
            GetEnumeratorForType: GetEnumeratorForType::<Identity, OFFSET>,
            GetRelationshipsContentStream: GetRelationshipsContentStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcRelationshipSet {}
windows_core::imp::define_interface!(IOpcSignatureCustomObject, IOpcSignatureCustomObject_Vtbl, 0x5d77a19e_62c1_44e7_becd_45da5ae51a56);
windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObject, windows_core::IUnknown);
impl IOpcSignatureCustomObject {
    pub unsafe fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetXml)(windows_core::Interface::as_raw(self), xmlmarkup as _, count as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IOpcSignatureCustomObject_Impl: windows_core::IUnknownImpl {
    fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
}
impl IOpcSignatureCustomObject_Vtbl {
    pub const fn new<Identity: IOpcSignatureCustomObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetXml<Identity: IOpcSignatureCustomObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignatureCustomObject_Impl::GetXml(this, core::mem::transmute_copy(&xmlmarkup), core::mem::transmute_copy(&count)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetXml: GetXml::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcSignatureCustomObject {}
windows_core::imp::define_interface!(IOpcSignatureCustomObjectEnumerator, IOpcSignatureCustomObjectEnumerator_Vtbl, 0x5ee4fe1d_e1b0_4683_8079_7ea0fcf80b4c);
windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObjectEnumerator, windows_core::IUnknown);
impl IOpcSignatureCustomObjectEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureCustomObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcSignatureCustomObjectEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureCustomObject>;
    fn Clone(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
}
impl IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureCustomObjectEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureCustomObjectEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureCustomObjectEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        customobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureCustomObjectEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcSignatureCustomObjectEnumerator {}
windows_core::imp::define_interface!(IOpcSignatureCustomObjectSet, IOpcSignatureCustomObjectSet_Vtbl, 0x8f792ac5_7947_4e11_bc3d_2659ff046ae1);
windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObjectSet, windows_core::IUnknown);
impl IOpcSignatureCustomObjectSet {
    pub unsafe fn Create(&self, xmlmarkup: &[u8]) -> windows_core::Result<IOpcSignatureCustomObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(xmlmarkup.as_ptr()), xmlmarkup.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P0>(&self, customobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignatureCustomObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), customobject.param().abi()).ok() }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcSignatureCustomObjectSet_Impl: windows_core::IUnknownImpl {
    fn Create(&self, xmlmarkup: *const u8, count: u32) -> windows_core::Result<IOpcSignatureCustomObject>;
    fn Delete(&self, customobject: windows_core::Ref<IOpcSignatureCustomObject>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
}
impl IOpcSignatureCustomObjectSet_Vtbl {
    pub const fn new<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureCustomObjectSet_Impl::Create(this, core::mem::transmute_copy(&xmlmarkup), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        customobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignatureCustomObjectSet_Impl::Delete(this, core::mem::transmute_copy(&customobject)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureCustomObjectSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        customobjectenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectSet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcSignatureCustomObjectSet {}
windows_core::imp::define_interface!(IOpcSignaturePartReference, IOpcSignaturePartReference_Vtbl, 0xe24231ca_59f4_484e_b64b_36eeda36072c);
windows_core::imp::interface_hierarchy!(IOpcSignaturePartReference, windows_core::IUnknown);
impl IOpcSignaturePartReference {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPartName(&self) -> windows_core::Result<IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDigestValue)(windows_core::Interface::as_raw(self), digestvalue as _, count as _).ok() }
    }
    pub unsafe fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReference_Impl: windows_core::IUnknownImpl {
    fn GetPartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignaturePartReference_Vtbl {
    pub const fn new<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPartName<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReference_Impl::GetPartName(this) {
                    Ok(ok__) => {
                        partname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContentType<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReference_Impl::GetContentType(this) {
                    Ok(ok__) => {
                        contenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReference_Impl::GetDigestMethod(this) {
                    Ok(ok__) => {
                        digestmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignaturePartReference_Impl::GetDigestValue(this, core::mem::transmute_copy(&digestvalue), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetTransformMethod<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReference_Impl::GetTransformMethod(this) {
                    Ok(ok__) => {
                        transformmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartName: GetPartName::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignaturePartReference as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignaturePartReference {}
windows_core::imp::define_interface!(IOpcSignaturePartReferenceEnumerator, IOpcSignaturePartReferenceEnumerator_Vtbl, 0x80eb1561_8c77_49cf_8266_459b356ee99a);
windows_core::imp::interface_hierarchy!(IOpcSignaturePartReferenceEnumerator, windows_core::IUnknown);
impl IOpcSignaturePartReferenceEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignaturePartReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcSignaturePartReferenceEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignaturePartReference>;
    fn Clone(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
}
impl IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReferenceEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReferenceEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReferenceEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        partreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReferenceEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcSignaturePartReferenceEnumerator {}
windows_core::imp::define_interface!(IOpcSignaturePartReferenceSet, IOpcSignaturePartReferenceSet_Vtbl, 0x6c9fe28c_ecd9_4b22_9d36_7fdde670fec0);
windows_core::imp::interface_hierarchy!(IOpcSignaturePartReferenceSet, windows_core::IUnknown);
impl IOpcSignaturePartReferenceSet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1>(&self, parturi: P0, digestmethod: P1, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignaturePartReference>
    where
        P0: windows_core::Param<IOpcPartUri>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), parturi.param().abi(), digestmethod.param().abi(), transformmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P0>(&self, partreference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignaturePartReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), partreference.param().abi()).ok() }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, OPC_CANONICALIZATION_METHOD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReferenceSet_Impl: windows_core::IUnknownImpl {
    fn Create(&self, parturi: windows_core::Ref<IOpcPartUri>, digestmethod: &windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignaturePartReference>;
    fn Delete(&self, partreference: windows_core::Ref<IOpcSignaturePartReference>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignaturePartReferenceSet_Vtbl {
    pub const fn new<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReferenceSet_Impl::Create(this, core::mem::transmute_copy(&parturi), core::mem::transmute(&digestmethod), core::mem::transmute_copy(&transformmethod)) {
                    Ok(ok__) => {
                        partreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignaturePartReferenceSet_Impl::Delete(this, core::mem::transmute_copy(&partreference)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignaturePartReferenceSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        partreferenceenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignaturePartReferenceSet {}
windows_core::imp::define_interface!(IOpcSignatureReference, IOpcSignatureReference_Vtbl, 0x1b47005e_3011_4edc_be6f_0f65e5ab0342);
windows_core::imp::interface_hierarchy!(IOpcSignatureReference, windows_core::IUnknown);
impl IOpcSignatureReference {
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDigestValue)(windows_core::Interface::as_raw(self), digestvalue as _, count as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReference_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureReference_Vtbl {
    pub const fn new<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReference_Impl::GetId(this) {
                    Ok(ok__) => {
                        referenceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUri<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReference_Impl::GetUri(this) {
                    Ok(ok__) => {
                        referenceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReference_Impl::GetType(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransformMethod<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReference_Impl::GetTransformMethod(this) {
                    Ok(ok__) => {
                        transformmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReference_Impl::GetDigestMethod(this) {
                    Ok(ok__) => {
                        digestmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignatureReference_Impl::GetDigestValue(this, core::mem::transmute_copy(&digestvalue), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetUri: GetUri::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureReference as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureReference {}
windows_core::imp::define_interface!(IOpcSignatureReferenceEnumerator, IOpcSignatureReferenceEnumerator_Vtbl, 0xcfa59a45_28b1_4868_969e_fa8097fdc12a);
windows_core::imp::interface_hierarchy!(IOpcSignatureReferenceEnumerator, windows_core::IUnknown);
impl IOpcSignatureReferenceEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcSignatureReferenceEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureReference>;
    fn Clone(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator>;
}
impl IOpcSignatureReferenceEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReferenceEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReferenceEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReferenceEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        reference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReferenceEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcSignatureReferenceEnumerator {}
windows_core::imp::define_interface!(IOpcSignatureReferenceSet, IOpcSignatureReferenceSet_Vtbl, 0xf3b02d31_ab12_42dd_9e2f_2b16761c3c1e);
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), referenceuri.param().abi(), referenceid.param().abi(), r#type.param().abi(), digestmethod.param().abi(), transformmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P0>(&self, reference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignatureReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), reference.param().abi()).ok() }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, OPC_CANONICALIZATION_METHOD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReferenceSet_Impl: windows_core::IUnknownImpl {
    fn Create(&self, referenceuri: windows_core::Ref<super::super::super::System::Com::IUri>, referenceid: &windows_core::PCWSTR, r#type: &windows_core::PCWSTR, digestmethod: &windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignatureReference>;
    fn Delete(&self, reference: windows_core::Ref<IOpcSignatureReference>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureReferenceSet_Vtbl {
    pub const fn new<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceuri: *mut core::ffi::c_void, referenceid: windows_core::PCWSTR, r#type: windows_core::PCWSTR, digestmethod: windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReferenceSet_Impl::Create(this, core::mem::transmute_copy(&referenceuri), core::mem::transmute(&referenceid), core::mem::transmute(&r#type), core::mem::transmute(&digestmethod), core::mem::transmute_copy(&transformmethod)) {
                    Ok(ok__) => {
                        reference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignatureReferenceSet_Impl::Delete(this, core::mem::transmute_copy(&reference)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureReferenceSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        referenceenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureReferenceSet {}
windows_core::imp::define_interface!(IOpcSignatureRelationshipReference, IOpcSignatureRelationshipReference_Vtbl, 0x57babac6_9d4a_4e50_8b86_e5d4051eae7c);
windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReference, windows_core::IUnknown);
impl IOpcSignatureRelationshipReference {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> windows_core::Result<IOpcUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDigestValue)(windows_core::Interface::as_raw(self), digestvalue as _, count as _).ok() }
    }
    pub unsafe fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRelationshipSigningOption(&self) -> windows_core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationshipSigningOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRelationshipSelectorEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationshipSelectorEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReference_Impl: windows_core::IUnknownImpl {
    fn GetSourceUri(&self) -> windows_core::Result<IOpcUri>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetRelationshipSigningOption(&self) -> windows_core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION>;
    fn GetRelationshipSelectorEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureRelationshipReference_Vtbl {
    pub const fn new<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSourceUri<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReference_Impl::GetSourceUri(this) {
                    Ok(ok__) => {
                        sourceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReference_Impl::GetDigestMethod(this) {
                    Ok(ok__) => {
                        digestmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignatureRelationshipReference_Impl::GetDigestValue(this, core::mem::transmute_copy(&digestvalue), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetTransformMethod<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReference_Impl::GetTransformMethod(this) {
                    Ok(ok__) => {
                        transformmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelationshipSigningOption<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReference_Impl::GetRelationshipSigningOption(this) {
                    Ok(ok__) => {
                        relationshipsigningoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelationshipSelectorEnumerator<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectorenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReference_Impl::GetRelationshipSelectorEnumerator(this) {
                    Ok(ok__) => {
                        selectorenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSourceUri: GetSourceUri::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, OFFSET>,
            GetRelationshipSigningOption: GetRelationshipSigningOption::<Identity, OFFSET>,
            GetRelationshipSelectorEnumerator: GetRelationshipSelectorEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReference as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureRelationshipReference {}
windows_core::imp::define_interface!(IOpcSignatureRelationshipReferenceEnumerator, IOpcSignatureRelationshipReferenceEnumerator_Vtbl, 0x773ba3e4_f021_48e4_aa04_9816db5d3495);
windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceEnumerator, windows_core::IUnknown);
impl IOpcSignatureRelationshipReferenceEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MovePrevious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureRelationshipReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpcSignatureRelationshipReferenceEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureRelationshipReference>;
    fn Clone(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
impl IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReferenceEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReferenceEnumerator_Impl::MovePrevious(this) {
                    Ok(ok__) => {
                        hasprevious.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReferenceEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        relationshipreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReferenceEnumerator_Impl::Clone(this) {
                    Ok(ok__) => {
                        copy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpcSignatureRelationshipReferenceEnumerator {}
windows_core::imp::define_interface!(IOpcSignatureRelationshipReferenceSet, IOpcSignatureRelationshipReferenceSet_Vtbl, 0x9f863ca5_3631_404c_828d_807e0715069b);
windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceSet, windows_core::IUnknown);
impl IOpcSignatureRelationshipReferenceSet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1, P3>(&self, sourceuri: P0, digestmethod: P1, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: P3, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignatureRelationshipReference>
    where
        P0: windows_core::Param<IOpcUri>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IOpcRelationshipSelectorSet>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), sourceuri.param().abi(), digestmethod.param().abi(), relationshipsigningoption, selectorset.param().abi(), transformmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRelationshipSelectorSet(&self) -> windows_core::Result<IOpcRelationshipSelectorSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRelationshipSelectorSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P0>(&self, relationshipreference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcSignatureRelationshipReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), relationshipreference.param().abi()).ok() }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReferenceSet_Impl: windows_core::IUnknownImpl {
    fn Create(&self, sourceuri: windows_core::Ref<IOpcUri>, digestmethod: &windows_core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: windows_core::Ref<IOpcRelationshipSelectorSet>, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignatureRelationshipReference>;
    fn CreateRelationshipSelectorSet(&self) -> windows_core::Result<IOpcRelationshipSelectorSet>;
    fn Delete(&self, relationshipreference: windows_core::Ref<IOpcSignatureRelationshipReference>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureRelationshipReferenceSet_Vtbl {
    pub const fn new<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: *mut core::ffi::c_void, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReferenceSet_Impl::Create(this, core::mem::transmute_copy(&sourceuri), core::mem::transmute(&digestmethod), core::mem::transmute_copy(&relationshipsigningoption), core::mem::transmute_copy(&selectorset), core::mem::transmute_copy(&transformmethod)) {
                    Ok(ok__) => {
                        relationshipreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRelationshipSelectorSet<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectorset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReferenceSet_Impl::CreateRelationshipSelectorSet(this) {
                    Ok(ok__) => {
                        selectorset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSignatureRelationshipReferenceSet_Impl::Delete(this, core::mem::transmute_copy(&relationshipreference)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSignatureRelationshipReferenceSet_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        relationshipreferenceenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            CreateRelationshipSelectorSet: CreateRelationshipSelectorSet::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureRelationshipReferenceSet {}
windows_core::imp::define_interface!(IOpcSigningOptions, IOpcSigningOptions_Vtbl, 0x50d2d6a5_7aeb_46c0_b241_43ab0e9b407e);
windows_core::imp::interface_hierarchy!(IOpcSigningOptions, windows_core::IUnknown);
impl IOpcSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignatureId)(windows_core::Interface::as_raw(self), signatureid.param().abi()).ok() }
    }
    pub unsafe fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignatureMethod)(windows_core::Interface::as_raw(self), signaturemethod.param().abi()).ok() }
    }
    pub unsafe fn GetDefaultDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultDigestMethod<P0>(&self, digestmethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultDigestMethod)(windows_core::Interface::as_raw(self), digestmethod.param().abi()).ok() }
    }
    pub unsafe fn GetCertificateEmbeddingOption(&self) -> windows_core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateEmbeddingOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCertificateEmbeddingOption)(windows_core::Interface::as_raw(self), embeddingoption).ok() }
    }
    pub unsafe fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimeFormat)(windows_core::Interface::as_raw(self), timeformat).ok() }
    }
    pub unsafe fn GetSignaturePartReferenceSet(&self) -> windows_core::Result<IOpcSignaturePartReferenceSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignaturePartReferenceSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSignatureRelationshipReferenceSet(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureRelationshipReferenceSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCustomObjectSet(&self) -> windows_core::Result<IOpcSignatureCustomObjectSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomObjectSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCustomReferenceSet(&self) -> windows_core::Result<IOpcSignatureReferenceSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomReferenceSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCertificateSet(&self) -> windows_core::Result<IOpcCertificateSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignaturePartName)(windows_core::Interface::as_raw(self), signaturepartname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IOpcSigningOptions_Impl: windows_core::IUnknownImpl {
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureId(&self, signatureid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureMethod(&self, signaturemethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDefaultDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDefaultDigestMethod(&self, digestmethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCertificateEmbeddingOption(&self) -> windows_core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION>;
    fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::Result<()>;
    fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()>;
    fn GetSignaturePartReferenceSet(&self) -> windows_core::Result<IOpcSignaturePartReferenceSet>;
    fn GetSignatureRelationshipReferenceSet(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceSet>;
    fn GetCustomObjectSet(&self) -> windows_core::Result<IOpcSignatureCustomObjectSet>;
    fn GetCustomReferenceSet(&self) -> windows_core::Result<IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&self) -> windows_core::Result<IOpcCertificateSet>;
    fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn SetSignaturePartName(&self, signaturepartname: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSigningOptions_Vtbl {
    pub const fn new<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSignatureId<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetSignatureId(this) {
                    Ok(ok__) => {
                        signatureid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignatureId<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSigningOptions_Impl::SetSignatureId(this, core::mem::transmute(&signatureid)).into()
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetSignatureMethod(this) {
                    Ok(ok__) => {
                        signaturemethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSigningOptions_Impl::SetSignatureMethod(this, core::mem::transmute(&signaturemethod)).into()
            }
        }
        unsafe extern "system" fn GetDefaultDigestMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetDefaultDigestMethod(this) {
                    Ok(ok__) => {
                        digestmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultDigestMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSigningOptions_Impl::SetDefaultDigestMethod(this, core::mem::transmute(&digestmethod)).into()
            }
        }
        unsafe extern "system" fn GetCertificateEmbeddingOption<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetCertificateEmbeddingOption(this) {
                    Ok(ok__) => {
                        embeddingoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCertificateEmbeddingOption<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSigningOptions_Impl::SetCertificateEmbeddingOption(this, core::mem::transmute_copy(&embeddingoption)).into()
            }
        }
        unsafe extern "system" fn GetTimeFormat<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetTimeFormat(this) {
                    Ok(ok__) => {
                        timeformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimeFormat<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSigningOptions_Impl::SetTimeFormat(this, core::mem::transmute_copy(&timeformat)).into()
            }
        }
        unsafe extern "system" fn GetSignaturePartReferenceSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetSignaturePartReferenceSet(this) {
                    Ok(ok__) => {
                        partreferenceset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetSignatureRelationshipReferenceSet(this) {
                    Ok(ok__) => {
                        relationshipreferenceset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustomObjectSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetCustomObjectSet(this) {
                    Ok(ok__) => {
                        customobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustomReferenceSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetCustomReferenceSet(this) {
                    Ok(ok__) => {
                        customreferenceset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetCertificateSet(this) {
                    Ok(ok__) => {
                        certificateset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcSigningOptions_Impl::GetSignaturePartName(this) {
                    Ok(ok__) => {
                        signaturepartname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpcSigningOptions_Impl::SetSignaturePartName(this, core::mem::transmute_copy(&signaturepartname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, OFFSET>,
            GetDefaultDigestMethod: GetDefaultDigestMethod::<Identity, OFFSET>,
            SetDefaultDigestMethod: SetDefaultDigestMethod::<Identity, OFFSET>,
            GetCertificateEmbeddingOption: GetCertificateEmbeddingOption::<Identity, OFFSET>,
            SetCertificateEmbeddingOption: SetCertificateEmbeddingOption::<Identity, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, OFFSET>,
            SetTimeFormat: SetTimeFormat::<Identity, OFFSET>,
            GetSignaturePartReferenceSet: GetSignaturePartReferenceSet::<Identity, OFFSET>,
            GetSignatureRelationshipReferenceSet: GetSignatureRelationshipReferenceSet::<Identity, OFFSET>,
            GetCustomObjectSet: GetCustomObjectSet::<Identity, OFFSET>,
            GetCustomReferenceSet: GetCustomReferenceSet::<Identity, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSigningOptions as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSigningOptions {}
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
    pub unsafe fn GetRelationshipsPartUri(&self) -> windows_core::Result<IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationshipsPartUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRelativeUri<P0>(&self, targetparturi: P0) -> windows_core::Result<super::super::super::System::Com::IUri>
    where
        P0: windows_core::Param<IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelativeUri)(windows_core::Interface::as_raw(self), targetparturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CombinePartUri<P0>(&self, relativeuri: P0) -> windows_core::Result<IOpcPartUri>
    where
        P0: windows_core::Param<super::super::super::System::Com::IUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CombinePartUri)(windows_core::Interface::as_raw(self), relativeuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IOpcUri_Vtbl {
    pub base__: super::super::super::System::Com::IUri_Vtbl,
    pub GetRelationshipsPartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelativeUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CombinePartUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcUri_Impl: super::super::super::System::Com::IUri_Impl {
    fn GetRelationshipsPartUri(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetRelativeUri(&self, targetparturi: windows_core::Ref<IOpcPartUri>) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn CombinePartUri(&self, relativeuri: windows_core::Ref<super::super::super::System::Com::IUri>) -> windows_core::Result<IOpcPartUri>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcUri_Vtbl {
    pub const fn new<Identity: IOpcUri_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRelationshipsPartUri<Identity: IOpcUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipparturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcUri_Impl::GetRelationshipsPartUri(this) {
                    Ok(ok__) => {
                        relationshipparturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelativeUri<Identity: IOpcUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetparturi: *mut core::ffi::c_void, relativeuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcUri_Impl::GetRelativeUri(this, core::mem::transmute_copy(&targetparturi)) {
                    Ok(ok__) => {
                        relativeuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CombinePartUri<Identity: IOpcUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeuri: *mut core::ffi::c_void, combineduri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpcUri_Impl::CombinePartUri(this, core::mem::transmute_copy(&relativeuri)) {
                    Ok(ok__) => {
                        combineduri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IUri_Vtbl::new::<Identity, OFFSET>(),
            GetRelationshipsPartUri: GetRelationshipsPartUri::<Identity, OFFSET>,
            GetRelativeUri: GetRelativeUri::<Identity, OFFSET>,
            CombinePartUri: CombinePartUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcUri as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IUri as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcUri {}
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = OPC_READ_FLAGS(2i32);
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(1i32);
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_CANONICALIZATION_METHOD(pub i32);
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_CERTIFICATE_EMBEDDING_OPTION(pub i32);
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(0i32);
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(1i32);
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(2i32);
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(2i32);
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(1i32);
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(-1i32);
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_COMPRESSION_OPTIONS(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_READ_FLAGS(pub i32);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_RELATIONSHIPS_SIGNING_OPTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_RELATIONSHIP_SELECTOR(pub i32);
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(0i32);
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(1i32);
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(1i32);
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(0i32);
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_SIGNATURE_TIME_FORMAT(pub i32);
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(3i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(0i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(2i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(4i32);
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(1i32);
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(5i32);
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_SIGNATURE_VALIDATION_RESULT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_STREAM_IO_MODE(pub i32);
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(1i32);
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_URI_TARGET_MODE(pub i32);
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(1i32);
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(0i32);
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = OPC_READ_FLAGS(1i32);
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPC_WRITE_FLAGS(pub i32);
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
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(1i32);
pub const OpcFactory: windows_core::GUID = windows_core::GUID::from_u128(0x6b2d6ba0_9f3e_4f27_920b_313cc426a39e);
