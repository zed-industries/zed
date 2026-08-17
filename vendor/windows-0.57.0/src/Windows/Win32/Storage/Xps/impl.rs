#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget_Impl: Sized {
    fn GetXpsOMPackageWriter(&self, documentsequencepartname: Option<&super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
    fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory>;
    fn GetXpsType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsDocumentPackageTarget {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>() -> IXpsDocumentPackageTarget_Vtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsDocumentPackageTarget_Impl::GetXpsOMPackageWriter(this, windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                Ok(ok__) => {
                    core::ptr::write(packagewriter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsDocumentPackageTarget_Impl::GetXpsOMFactory(this) {
                Ok(ok__) => {
                    core::ptr::write(xpsfactory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsDocumentPackageTarget_Impl::GetXpsType(this) {
                Ok(ok__) => {
                    core::ptr::write(documenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter: GetXpsOMPackageWriter::<Identity, Impl, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, Impl, OFFSET>,
            GetXpsType: GetXpsType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget3D_Impl: Sized {
    fn GetXpsOMPackageWriter3D(&self, documentsequencepartname: Option<&super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: Option<&super::Packaging::Opc::IOpcPartUri>, modelpartname: Option<&super::Packaging::Opc::IOpcPartUri>, modeldata: Option<&super::super::System::Com::IStream>) -> windows_core::Result<IXpsOMPackageWriter3D>;
    fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsDocumentPackageTarget3D {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>() -> IXpsDocumentPackageTarget3D_Vtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, modelpartname: *mut core::ffi::c_void, modeldata: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsDocumentPackageTarget3D_Impl::GetXpsOMPackageWriter3D(this, windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&discardcontrolpartname), windows_core::from_raw_borrowed(&modelpartname), windows_core::from_raw_borrowed(&modeldata)) {
                Ok(ok__) => {
                    core::ptr::write(packagewriter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsDocumentPackageTarget3D_Impl::GetXpsOMFactory(this) {
                Ok(ok__) => {
                    core::ptr::write(xpsfactory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter3D: GetXpsOMPackageWriter3D::<Identity, Impl, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget3D as windows_core::Interface>::IID
    }
}
pub trait IXpsOMBrush_Impl: Sized + IXpsOMShareable_Impl {
    fn GetOpacity(&self) -> windows_core::Result<f32>;
    fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMBrush {}
impl IXpsOMBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: isize>() -> IXpsOMBrush_Vtbl {
        unsafe extern "system" fn GetOpacity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMBrush_Impl::GetOpacity(this) {
                Ok(ok__) => {
                    core::ptr::write(opacity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMBrush_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity)).into()
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOpacity: GetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMCanvas_Impl: Sized + IXpsOMVisual_Impl {
    fn GetVisuals(&self) -> windows_core::Result<IXpsOMVisualCollection>;
    fn GetUseAliasedEdgeMode(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUseAliasedEdgeMode(&self, usealiasededgemode: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(&self, shortdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(&self, longdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&self, resourcedictionary: Option<&IXpsOMDictionary>) -> windows_core::Result<()>;
    fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&self, remotedictionaryresource: Option<&IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMCanvas>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMCanvas {}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMCanvas_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>() -> IXpsOMCanvas_Vtbl {
        unsafe extern "system" fn GetVisuals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visuals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::GetVisuals(this) {
                Ok(ok__) => {
                    core::ptr::write(visuals, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::GetUseAliasedEdgeMode(this) {
                Ok(ok__) => {
                    core::ptr::write(usealiasededgemode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCanvas_Impl::SetUseAliasedEdgeMode(this, core::mem::transmute_copy(&usealiasededgemode)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::GetAccessibilityShortDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(shortdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCanvas_Impl::SetAccessibilityShortDescription(this, core::mem::transmute(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::GetAccessibilityLongDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(longdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCanvas_Impl::SetAccessibilityLongDescription(this, core::mem::transmute(&longdescription)).into()
        }
        unsafe extern "system" fn GetDictionary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::GetDictionary(this) {
                Ok(ok__) => {
                    core::ptr::write(resourcedictionary, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::GetDictionaryLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(resourcedictionary, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCanvas_Impl::SetDictionaryLocal(this, windows_core::from_raw_borrowed(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::GetDictionaryResource(this) {
                Ok(ok__) => {
                    core::ptr::write(remotedictionaryresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCanvas_Impl::SetDictionaryResource(this, windows_core::from_raw_borrowed(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, canvas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCanvas_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(canvas, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetVisuals: GetVisuals::<Identity, Impl, OFFSET>,
            GetUseAliasedEdgeMode: GetUseAliasedEdgeMode::<Identity, Impl, OFFSET>,
            SetUseAliasedEdgeMode: SetUseAliasedEdgeMode::<Identity, Impl, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, Impl, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, Impl, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, Impl, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMCanvas as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMVisual as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: Option<&super::super::System::Com::IStream>, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMColorProfileResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>() -> IXpsOMColorProfileResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMColorProfileResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(stream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMColorProfileResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn InsertAt(&self, index: u32, object: Option<&IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: Option<&IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Append(&self, object: Option<&IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMColorProfileResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMColorProfileResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResourceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMColorProfileResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMColorProfileResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMColorProfileResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(object, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMColorProfileResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMColorProfileResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMColorProfileResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMColorProfileResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMColorProfileResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    core::ptr::write(part, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResourceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMCoreProperties_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPackage>;
    fn GetCategory(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCategory(&self, category: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetContentStatus(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetContentStatus(&self, contentstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetContentType(&self, contenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCreated(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCreator(&self, creator: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDescription(&self, description: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIdentifier(&self, identifier: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetKeywords(&self, keywords: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLanguage(&self, language: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLastModifiedBy(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLastModifiedBy(&self, lastmodifiedby: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLastPrinted(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetModified(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetRevision(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRevision(&self, revision: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSubject(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSubject(&self, subject: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTitle(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTitle(&self, title: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetVersion(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetVersion(&self, version: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMCoreProperties>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMCoreProperties {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMCoreProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>() -> IXpsOMCoreProperties_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetCategory(this) {
                Ok(ok__) => {
                    core::ptr::write(category, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetCategory(this, core::mem::transmute(&category)).into()
        }
        unsafe extern "system" fn GetContentStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentstatus: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetContentStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(contentstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentstatus: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetContentStatus(this, core::mem::transmute(&contentstatus)).into()
        }
        unsafe extern "system" fn GetContentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetContentType(this) {
                Ok(ok__) => {
                    core::ptr::write(contenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetContentType(this, core::mem::transmute(&contenttype)).into()
        }
        unsafe extern "system" fn GetCreated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetCreated(this) {
                Ok(ok__) => {
                    core::ptr::write(created, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetCreated(this, core::mem::transmute_copy(&created)).into()
        }
        unsafe extern "system" fn GetCreator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creator: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetCreator(this) {
                Ok(ok__) => {
                    core::ptr::write(creator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creator: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetCreator(this, core::mem::transmute(&creator)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(description, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetDescription(this, core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn GetIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetIdentifier(this) {
                Ok(ok__) => {
                    core::ptr::write(identifier, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetIdentifier(this, core::mem::transmute(&identifier)).into()
        }
        unsafe extern "system" fn GetKeywords<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetKeywords(this) {
                Ok(ok__) => {
                    core::ptr::write(keywords, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetKeywords(this, core::mem::transmute(&keywords)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetLanguage(this) {
                Ok(ok__) => {
                    core::ptr::write(language, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
        }
        unsafe extern "system" fn GetLastModifiedBy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodifiedby: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetLastModifiedBy(this) {
                Ok(ok__) => {
                    core::ptr::write(lastmodifiedby, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedBy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodifiedby: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetLastModifiedBy(this, core::mem::transmute(&lastmodifiedby)).into()
        }
        unsafe extern "system" fn GetLastPrinted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetLastPrinted(this) {
                Ok(ok__) => {
                    core::ptr::write(lastprinted, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastPrinted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetLastPrinted(this, core::mem::transmute_copy(&lastprinted)).into()
        }
        unsafe extern "system" fn GetModified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetModified(this) {
                Ok(ok__) => {
                    core::ptr::write(modified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetModified(this, core::mem::transmute_copy(&modified)).into()
        }
        unsafe extern "system" fn GetRevision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, revision: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetRevision(this) {
                Ok(ok__) => {
                    core::ptr::write(revision, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, revision: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetRevision(this, core::mem::transmute(&revision)).into()
        }
        unsafe extern "system" fn GetSubject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetSubject(this) {
                Ok(ok__) => {
                    core::ptr::write(subject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetSubject(this, core::mem::transmute(&subject)).into()
        }
        unsafe extern "system" fn GetTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetTitle(this) {
                Ok(ok__) => {
                    core::ptr::write(title, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetTitle(this, core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::GetVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(version, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMCoreProperties_Impl::SetVersion(this, core::mem::transmute(&version)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMCoreProperties_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(coreproperties, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            SetCategory: SetCategory::<Identity, Impl, OFFSET>,
            GetContentStatus: GetContentStatus::<Identity, Impl, OFFSET>,
            SetContentStatus: SetContentStatus::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            SetContentType: SetContentType::<Identity, Impl, OFFSET>,
            GetCreated: GetCreated::<Identity, Impl, OFFSET>,
            SetCreated: SetCreated::<Identity, Impl, OFFSET>,
            GetCreator: GetCreator::<Identity, Impl, OFFSET>,
            SetCreator: SetCreator::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET>,
            SetIdentifier: SetIdentifier::<Identity, Impl, OFFSET>,
            GetKeywords: GetKeywords::<Identity, Impl, OFFSET>,
            SetKeywords: SetKeywords::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            GetLastModifiedBy: GetLastModifiedBy::<Identity, Impl, OFFSET>,
            SetLastModifiedBy: SetLastModifiedBy::<Identity, Impl, OFFSET>,
            GetLastPrinted: GetLastPrinted::<Identity, Impl, OFFSET>,
            SetLastPrinted: SetLastPrinted::<Identity, Impl, OFFSET>,
            GetModified: GetModified::<Identity, Impl, OFFSET>,
            SetModified: SetModified::<Identity, Impl, OFFSET>,
            GetRevision: GetRevision::<Identity, Impl, OFFSET>,
            SetRevision: SetRevision::<Identity, Impl, OFFSET>,
            GetSubject: GetSubject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            GetTitle: GetTitle::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMCoreProperties as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
pub trait IXpsOMDashCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<XPS_DASH>;
    fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()>;
    fn Append(&self, dash: *const XPS_DASH) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMDashCollection {}
impl IXpsOMDashCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>() -> IXpsOMDashCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDashCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDashCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(dash, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDashCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDashCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDashCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dash: *const XPS_DASH) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDashCollection_Impl::Append(this, core::mem::transmute_copy(&dash)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDashCollection as windows_core::Interface>::IID
    }
}
pub trait IXpsOMDictionary_Impl: Sized {
    fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32, key: *mut windows_core::PWSTR) -> windows_core::Result<IXpsOMShareable>;
    fn GetByKey(&self, key: &windows_core::PCWSTR, beforeentry: Option<&IXpsOMShareable>) -> windows_core::Result<IXpsOMShareable>;
    fn GetIndex(&self, entry: Option<&IXpsOMShareable>) -> windows_core::Result<u32>;
    fn Append(&self, key: &windows_core::PCWSTR, entry: Option<&IXpsOMShareable>) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, key: &windows_core::PCWSTR, entry: Option<&IXpsOMShareable>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, key: &windows_core::PCWSTR, entry: Option<&IXpsOMShareable>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMDictionary>;
}
impl windows_core::RuntimeName for IXpsOMDictionary {}
impl IXpsOMDictionary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>() -> IXpsOMDictionary_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDictionary_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(owner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDictionary_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: *mut windows_core::PWSTR, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDictionary_Impl::GetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    core::ptr::write(entry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR, beforeentry: *mut core::ffi::c_void, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDictionary_Impl::GetByKey(this, core::mem::transmute(&key), windows_core::from_raw_borrowed(&beforeentry)) {
                Ok(ok__) => {
                    core::ptr::write(entry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entry: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDictionary_Impl::GetIndex(this, windows_core::from_raw_borrowed(&entry)) {
                Ok(ok__) => {
                    core::ptr::write(index, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDictionary_Impl::Append(this, core::mem::transmute(&key), windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDictionary_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute(&key), windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDictionary_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDictionary_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute(&key), windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDictionary_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(dictionary, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetByKey: GetByKey::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDictionary as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocument_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocumentSequence>;
    fn GetPageReferences(&self) -> windows_core::Result<IXpsOMPageReferenceCollection>;
    fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: Option<&IXpsOMPrintTicketResource>) -> windows_core::Result<()>;
    fn GetDocumentStructureResource(&self) -> windows_core::Result<IXpsOMDocumentStructureResource>;
    fn SetDocumentStructureResource(&self, documentstructureresource: Option<&IXpsOMDocumentStructureResource>) -> windows_core::Result<()>;
    fn GetSignatureBlockResources(&self) -> windows_core::Result<IXpsOMSignatureBlockResourceCollection>;
    fn Clone(&self) -> windows_core::Result<IXpsOMDocument>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMDocument {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>() -> IXpsOMDocument_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocument_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(documentsequence, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageReferences<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereferences: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocument_Impl::GetPageReferences(this) {
                Ok(ok__) => {
                    core::ptr::write(pagereferences, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocument_Impl::GetPrintTicketResource(this) {
                Ok(ok__) => {
                    core::ptr::write(printticketresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocument_Impl::SetPrintTicketResource(this, windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        unsafe extern "system" fn GetDocumentStructureResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentstructureresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocument_Impl::GetDocumentStructureResource(this) {
                Ok(ok__) => {
                    core::ptr::write(documentstructureresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentStructureResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentstructureresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocument_Impl::SetDocumentStructureResource(this, windows_core::from_raw_borrowed(&documentstructureresource)).into()
        }
        unsafe extern "system" fn GetSignatureBlockResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblockresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocument_Impl::GetSignatureBlockResources(this) {
                Ok(ok__) => {
                    core::ptr::write(signatureblockresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocument_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(document, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetPageReferences: GetPageReferences::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
            GetDocumentStructureResource: GetDocumentStructureResource::<Identity, Impl, OFFSET>,
            SetDocumentStructureResource: SetDocumentStructureResource::<Identity, Impl, OFFSET>,
            GetSignatureBlockResources: GetSignatureBlockResources::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocument as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
pub trait IXpsOMDocumentCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMDocument>;
    fn InsertAt(&self, index: u32, document: Option<&IXpsOMDocument>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, document: Option<&IXpsOMDocument>) -> windows_core::Result<()>;
    fn Append(&self, document: Option<&IXpsOMDocument>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMDocumentCollection {}
impl IXpsOMDocumentCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>() -> IXpsOMDocumentCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocumentCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocumentCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(document, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocumentCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&document)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocumentCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocumentCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&document)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocumentCollection_Impl::Append(this, windows_core::from_raw_borrowed(&document)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentSequence_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPackage>;
    fn GetDocuments(&self) -> windows_core::Result<IXpsOMDocumentCollection>;
    fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: Option<&IXpsOMPrintTicketResource>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMDocumentSequence {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentSequence_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>() -> IXpsOMDocumentSequence_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocumentSequence_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocuments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocumentSequence_Impl::GetDocuments(this) {
                Ok(ok__) => {
                    core::ptr::write(documents, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocumentSequence_Impl::GetPrintTicketResource(this) {
                Ok(ok__) => {
                    core::ptr::write(printticketresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocumentSequence_Impl::SetPrintTicketResource(this, windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetDocuments: GetDocuments::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentSequence as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentStructureResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: Option<&super::super::System::Com::IStream>, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMDocumentStructureResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentStructureResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>() -> IXpsOMDocumentStructureResource_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocumentStructureResource_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(owner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMDocumentStructureResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(stream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMDocumentStructureResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentStructureResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: Option<&super::super::System::Com::IStream>, embeddingoption: XPS_FONT_EMBEDDING, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetEmbeddingOption(&self) -> windows_core::Result<XPS_FONT_EMBEDDING>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMFontResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>() -> IXpsOMFontResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readerstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMFontResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(readerstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMFontResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), core::mem::transmute_copy(&embeddingoption), windows_core::from_raw_borrowed(&partname)).into()
        }
        unsafe extern "system" fn GetEmbeddingOption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMFontResource_Impl::GetEmbeddingOption(this) {
                Ok(ok__) => {
                    core::ptr::write(embeddingoption, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            GetEmbeddingOption: GetEmbeddingOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMFontResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResourceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMFontResource>;
    fn SetAt(&self, index: u32, value: Option<&IXpsOMFontResource>) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, value: Option<&IXpsOMFontResource>) -> windows_core::Result<()>;
    fn Append(&self, value: Option<&IXpsOMFontResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMFontResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMFontResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResourceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMFontResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMFontResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMFontResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMFontResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMFontResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMFontResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMFontResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMFontResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    core::ptr::write(part, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMFontResourceCollection as windows_core::Interface>::IID
    }
}
pub trait IXpsOMGeometry_Impl: Sized + IXpsOMShareable_Impl {
    fn GetFigures(&self) -> windows_core::Result<IXpsOMGeometryFigureCollection>;
    fn GetFillRule(&self) -> windows_core::Result<XPS_FILL_RULE>;
    fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> windows_core::Result<()>;
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: Option<&IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGeometry>;
}
impl windows_core::RuntimeName for IXpsOMGeometry {}
impl IXpsOMGeometry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>() -> IXpsOMGeometry_Vtbl {
        unsafe extern "system" fn GetFigures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, figures: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometry_Impl::GetFigures(this) {
                Ok(ok__) => {
                    core::ptr::write(figures, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillRule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometry_Impl::GetFillRule(this) {
                Ok(ok__) => {
                    core::ptr::write(fillrule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillrule: XPS_FILL_RULE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometry_Impl::SetFillRule(this, core::mem::transmute_copy(&fillrule)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometry_Impl::GetTransform(this) {
                Ok(ok__) => {
                    core::ptr::write(transform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometry_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(transform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometry_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometry_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(lookup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometry_Impl::SetTransformLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometry_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(geometry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFigures: GetFigures::<Identity, Impl, OFFSET>,
            GetFillRule: GetFillRule::<Identity, Impl, OFFSET>,
            SetFillRule: SetFillRule::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometry as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
pub trait IXpsOMGeometryFigure_Impl: Sized {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> windows_core::Result<()>;
    fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> windows_core::Result<()>;
    fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetStartPoint(&self) -> windows_core::Result<XPS_POINT>;
    fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetIsClosed(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsClosed(&self, isclosed: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetIsFilled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsFilled(&self, isfilled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSegmentCount(&self) -> windows_core::Result<u32>;
    fn GetSegmentDataCount(&self) -> windows_core::Result<u32>;
    fn GetSegmentStrokePattern(&self) -> windows_core::Result<XPS_SEGMENT_STROKE_PATTERN>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGeometryFigure>;
}
impl windows_core::RuntimeName for IXpsOMGeometryFigure {}
impl IXpsOMGeometryFigure_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>() -> IXpsOMGeometryFigure_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(owner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigure_Impl::GetSegmentData(this, core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&segmentdata)).into()
        }
        unsafe extern "system" fn GetSegmentTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigure_Impl::GetSegmentTypes(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmenttypes)).into()
        }
        unsafe extern "system" fn GetSegmentStrokes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigure_Impl::GetSegmentStrokes(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn SetSegments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigure_Impl::SetSegments(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmentdatacount), core::mem::transmute_copy(&segmenttypes), core::mem::transmute_copy(&segmentdata), core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn GetStartPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *mut XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::GetStartPoint(this) {
                Ok(ok__) => {
                    core::ptr::write(startpoint, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigure_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetIsClosed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::GetIsClosed(this) {
                Ok(ok__) => {
                    core::ptr::write(isclosed, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigure_Impl::SetIsClosed(this, core::mem::transmute_copy(&isclosed)).into()
        }
        unsafe extern "system" fn GetIsFilled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::GetIsFilled(this) {
                Ok(ok__) => {
                    core::ptr::write(isfilled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigure_Impl::SetIsFilled(this, core::mem::transmute_copy(&isfilled)).into()
        }
        unsafe extern "system" fn GetSegmentCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::GetSegmentCount(this) {
                Ok(ok__) => {
                    core::ptr::write(segmentcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentdatacount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::GetSegmentDataCount(this) {
                Ok(ok__) => {
                    core::ptr::write(segmentdatacount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::GetSegmentStrokePattern(this) {
                Ok(ok__) => {
                    core::ptr::write(segmentstrokepattern, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryfigure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigure_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(geometryfigure, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, Impl, OFFSET>,
            GetSegmentTypes: GetSegmentTypes::<Identity, Impl, OFFSET>,
            GetSegmentStrokes: GetSegmentStrokes::<Identity, Impl, OFFSET>,
            SetSegments: SetSegments::<Identity, Impl, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            GetIsClosed: GetIsClosed::<Identity, Impl, OFFSET>,
            SetIsClosed: SetIsClosed::<Identity, Impl, OFFSET>,
            GetIsFilled: GetIsFilled::<Identity, Impl, OFFSET>,
            SetIsFilled: SetIsFilled::<Identity, Impl, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, Impl, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, Impl, OFFSET>,
            GetSegmentStrokePattern: GetSegmentStrokePattern::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigure as windows_core::Interface>::IID
    }
}
pub trait IXpsOMGeometryFigureCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGeometryFigure>;
    fn InsertAt(&self, index: u32, geometryfigure: Option<&IXpsOMGeometryFigure>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, geometryfigure: Option<&IXpsOMGeometryFigure>) -> windows_core::Result<()>;
    fn Append(&self, geometryfigure: Option<&IXpsOMGeometryFigure>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMGeometryFigureCollection {}
impl IXpsOMGeometryFigureCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>() -> IXpsOMGeometryFigureCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigureCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGeometryFigureCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(geometryfigure, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigureCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigureCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigureCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGeometryFigureCollection_Impl::Append(this, windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigureCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMGlyphs_Impl: Sized + IXpsOMVisual_Impl {
    fn GetUnicodeString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetGlyphIndexCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()>;
    fn GetGlyphMappingCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32>;
    fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()>;
    fn GetBidiLevel(&self) -> windows_core::Result<u32>;
    fn GetIsSideways(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetStyleSimulations(&self) -> windows_core::Result<XPS_STYLE_SIMULATION>;
    fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> windows_core::Result<()>;
    fn GetOrigin(&self) -> windows_core::Result<XPS_POINT>;
    fn SetOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetFontRenderingEmSize(&self) -> windows_core::Result<f32>;
    fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> windows_core::Result<()>;
    fn GetFontResource(&self) -> windows_core::Result<IXpsOMFontResource>;
    fn SetFontResource(&self, fontresource: Option<&IXpsOMFontResource>) -> windows_core::Result<()>;
    fn GetFontFaceIndex(&self) -> windows_core::Result<i16>;
    fn SetFontFaceIndex(&self, fontfaceindex: i16) -> windows_core::Result<()>;
    fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&self, fillbrush: Option<&IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetFillBrushLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetGlyphsEditor(&self) -> windows_core::Result<IXpsOMGlyphsEditor>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGlyphs>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMGlyphs {}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMGlyphs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>() -> IXpsOMGlyphs_Vtbl {
        unsafe extern "system" fn GetUnicodeString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetUnicodeString(this) {
                Ok(ok__) => {
                    core::ptr::write(unicodestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetGlyphIndexCount(this) {
                Ok(ok__) => {
                    core::ptr::write(indexcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetGlyphMappingCount(this) {
                Ok(ok__) => {
                    core::ptr::write(glyphmappingcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::GetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetProhibitedCaretStopCount(this) {
                Ok(ok__) => {
                    core::ptr::write(prohibitedcaretstopcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::GetProhibitedCaretStops(this, core::mem::transmute_copy(&prohibitedcaretstopcount), core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetBidiLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(bidilevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSideways<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetIsSideways(this) {
                Ok(ok__) => {
                    core::ptr::write(issideways, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetDeviceFontName(this) {
                Ok(ok__) => {
                    core::ptr::write(devicefontname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyleSimulations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetStyleSimulations(this) {
                Ok(ok__) => {
                    core::ptr::write(stylesimulations, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::SetStyleSimulations(this, core::mem::transmute_copy(&stylesimulations)).into()
        }
        unsafe extern "system" fn GetOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *mut XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetOrigin(this) {
                Ok(ok__) => {
                    core::ptr::write(origin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *const XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::SetOrigin(this, core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontrenderingemsize: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetFontRenderingEmSize(this) {
                Ok(ok__) => {
                    core::ptr::write(fontrenderingemsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontrenderingemsize: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::SetFontRenderingEmSize(this, core::mem::transmute_copy(&fontrenderingemsize)).into()
        }
        unsafe extern "system" fn GetFontResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetFontResource(this) {
                Ok(ok__) => {
                    core::ptr::write(fontresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::SetFontResource(this, windows_core::from_raw_borrowed(&fontresource)).into()
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfaceindex: *mut i16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetFontFaceIndex(this) {
                Ok(ok__) => {
                    core::ptr::write(fontfaceindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFaceIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfaceindex: i16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::SetFontFaceIndex(this, core::mem::transmute_copy(&fontfaceindex)).into()
        }
        unsafe extern "system" fn GetFillBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetFillBrush(this) {
                Ok(ok__) => {
                    core::ptr::write(fillbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetFillBrushLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(fillbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::SetFillBrushLocal(this, windows_core::from_raw_borrowed(&fillbrush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetFillBrushLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphs_Impl::SetFillBrushLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetGlyphsEditor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, editor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::GetGlyphsEditor(this) {
                Ok(ok__) => {
                    core::ptr::write(editor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphs_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(glyphs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetUnicodeString: GetUnicodeString::<Identity, Impl, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, Impl, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, Impl, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, Impl, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, Impl, OFFSET>,
            GetStyleSimulations: GetStyleSimulations::<Identity, Impl, OFFSET>,
            SetStyleSimulations: SetStyleSimulations::<Identity, Impl, OFFSET>,
            GetOrigin: GetOrigin::<Identity, Impl, OFFSET>,
            SetOrigin: SetOrigin::<Identity, Impl, OFFSET>,
            GetFontRenderingEmSize: GetFontRenderingEmSize::<Identity, Impl, OFFSET>,
            SetFontRenderingEmSize: SetFontRenderingEmSize::<Identity, Impl, OFFSET>,
            GetFontResource: GetFontResource::<Identity, Impl, OFFSET>,
            SetFontResource: SetFontResource::<Identity, Impl, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, Impl, OFFSET>,
            SetFontFaceIndex: SetFontFaceIndex::<Identity, Impl, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, Impl, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, Impl, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, Impl, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, Impl, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, Impl, OFFSET>,
            GetGlyphsEditor: GetGlyphsEditor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGlyphs as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMVisual as windows_core::Interface>::IID
    }
}
pub trait IXpsOMGlyphsEditor_Impl: Sized {
    fn ApplyEdits(&self) -> windows_core::Result<()>;
    fn GetUnicodeString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUnicodeString(&self, unicodestring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetGlyphIndexCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()>;
    fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> windows_core::Result<()>;
    fn GetGlyphMappingCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()>;
    fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32>;
    fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()>;
    fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> windows_core::Result<()>;
    fn GetBidiLevel(&self) -> windows_core::Result<u32>;
    fn SetBidiLevel(&self, bidilevel: u32) -> windows_core::Result<()>;
    fn GetIsSideways(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsSideways(&self, issideways: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDeviceFontName(&self, devicefontname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMGlyphsEditor {}
impl IXpsOMGlyphsEditor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>() -> IXpsOMGlyphsEditor_Vtbl {
        unsafe extern "system" fn ApplyEdits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::ApplyEdits(this).into()
        }
        unsafe extern "system" fn GetUnicodeString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphsEditor_Impl::GetUnicodeString(this) {
                Ok(ok__) => {
                    core::ptr::write(unicodestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::SetUnicodeString(this, core::mem::transmute(&unicodestring)).into()
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphsEditor_Impl::GetGlyphIndexCount(this) {
                Ok(ok__) => {
                    core::ptr::write(indexcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn SetGlyphIndices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::SetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphsEditor_Impl::GetGlyphMappingCount(this) {
                Ok(ok__) => {
                    core::ptr::write(glyphmappingcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::GetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn SetGlyphMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::SetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphsEditor_Impl::GetProhibitedCaretStopCount(this) {
                Ok(ok__) => {
                    core::ptr::write(prohibitedcaretstopcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::GetProhibitedCaretStops(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::SetProhibitedCaretStops(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphsEditor_Impl::GetBidiLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(bidilevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBidiLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::SetBidiLevel(this, core::mem::transmute_copy(&bidilevel)).into()
        }
        unsafe extern "system" fn GetIsSideways<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphsEditor_Impl::GetIsSideways(this) {
                Ok(ok__) => {
                    core::ptr::write(issideways, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSideways<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::SetIsSideways(this, core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGlyphsEditor_Impl::GetDeviceFontName(this) {
                Ok(ok__) => {
                    core::ptr::write(devicefontname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceFontName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGlyphsEditor_Impl::SetDeviceFontName(this, core::mem::transmute(&devicefontname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplyEdits: ApplyEdits::<Identity, Impl, OFFSET>,
            GetUnicodeString: GetUnicodeString::<Identity, Impl, OFFSET>,
            SetUnicodeString: SetUnicodeString::<Identity, Impl, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            SetGlyphIndices: SetGlyphIndices::<Identity, Impl, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, Impl, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, Impl, OFFSET>,
            SetGlyphMappings: SetGlyphMappings::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            SetProhibitedCaretStops: SetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, Impl, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, Impl, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, Impl, OFFSET>,
            SetIsSideways: SetIsSideways::<Identity, Impl, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, Impl, OFFSET>,
            SetDeviceFontName: SetDeviceFontName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGlyphsEditor as windows_core::Interface>::IID
    }
}
pub trait IXpsOMGradientBrush_Impl: Sized + IXpsOMBrush_Impl {
    fn GetGradientStops(&self) -> windows_core::Result<IXpsOMGradientStopCollection>;
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: Option<&IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSpreadMethod(&self) -> windows_core::Result<XPS_SPREAD_METHOD>;
    fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> windows_core::Result<()>;
    fn GetColorInterpolationMode(&self) -> windows_core::Result<XPS_COLOR_INTERPOLATION>;
    fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMGradientBrush {}
impl IXpsOMGradientBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>() -> IXpsOMGradientBrush_Vtbl {
        unsafe extern "system" fn GetGradientStops<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientBrush_Impl::GetGradientStops(this) {
                Ok(ok__) => {
                    core::ptr::write(gradientstops, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientBrush_Impl::GetTransform(this) {
                Ok(ok__) => {
                    core::ptr::write(transform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientBrush_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(transform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientBrush_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientBrush_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientBrush_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetSpreadMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientBrush_Impl::GetSpreadMethod(this) {
                Ok(ok__) => {
                    core::ptr::write(spreadmethod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientBrush_Impl::SetSpreadMethod(this, core::mem::transmute_copy(&spreadmethod)).into()
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientBrush_Impl::GetColorInterpolationMode(this) {
                Ok(ok__) => {
                    core::ptr::write(colorinterpolationmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientBrush_Impl::SetColorInterpolationMode(this, core::mem::transmute_copy(&colorinterpolationmode)).into()
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGradientStops: GetGradientStops::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetSpreadMethod: GetSpreadMethod::<Identity, Impl, OFFSET>,
            SetSpreadMethod: SetSpreadMethod::<Identity, Impl, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, Impl, OFFSET>,
            SetColorInterpolationMode: SetColorInterpolationMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID
    }
}
pub trait IXpsOMGradientStop_Impl: Sized {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMGradientBrush>;
    fn GetOffset(&self) -> windows_core::Result<f32>;
    fn SetOffset(&self, offset: f32) -> windows_core::Result<()>;
    fn GetColor(&self, color: *mut XPS_COLOR) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColor(&self, color: *const XPS_COLOR, colorprofile: Option<&IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGradientStop>;
}
impl windows_core::RuntimeName for IXpsOMGradientStop {}
impl IXpsOMGradientStop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>() -> IXpsOMGradientStop_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientStop_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(owner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientStop_Impl::GetOffset(this) {
                Ok(ok__) => {
                    core::ptr::write(offset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientStop_Impl::SetOffset(this, core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientStop_Impl::GetColor(this, core::mem::transmute_copy(&color)) {
                Ok(ok__) => {
                    core::ptr::write(colorprofile, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientStop_Impl::SetColor(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientStop_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(gradientstop, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetOffset: GetOffset::<Identity, Impl, OFFSET>,
            SetOffset: SetOffset::<Identity, Impl, OFFSET>,
            GetColor: GetColor::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientStop as windows_core::Interface>::IID
    }
}
pub trait IXpsOMGradientStopCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGradientStop>;
    fn InsertAt(&self, index: u32, stop: Option<&IXpsOMGradientStop>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, stop: Option<&IXpsOMGradientStop>) -> windows_core::Result<()>;
    fn Append(&self, stop: Option<&IXpsOMGradientStop>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMGradientStopCollection {}
impl IXpsOMGradientStopCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>() -> IXpsOMGradientStopCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientStopCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMGradientStopCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(stop, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientStopCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&stop)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientStopCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientStopCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&stop)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMGradientStopCollection_Impl::Append(this, windows_core::from_raw_borrowed(&stop)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientStopCollection as windows_core::Interface>::IID
    }
}
pub trait IXpsOMImageBrush_Impl: Sized + IXpsOMTileBrush_Impl {
    fn GetImageResource(&self) -> windows_core::Result<IXpsOMImageResource>;
    fn SetImageResource(&self, imageresource: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn GetColorProfileResource(&self) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColorProfileResource(&self, colorprofileresource: Option<&IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMImageBrush>;
}
impl windows_core::RuntimeName for IXpsOMImageBrush {}
impl IXpsOMImageBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>() -> IXpsOMImageBrush_Vtbl {
        unsafe extern "system" fn GetImageResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageBrush_Impl::GetImageResource(this) {
                Ok(ok__) => {
                    core::ptr::write(imageresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMImageBrush_Impl::SetImageResource(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn GetColorProfileResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageBrush_Impl::GetColorProfileResource(this) {
                Ok(ok__) => {
                    core::ptr::write(colorprofileresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorProfileResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMImageBrush_Impl::SetColorProfileResource(this, windows_core::from_raw_borrowed(&colorprofileresource)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(imagebrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImageResource: GetImageResource::<Identity, Impl, OFFSET>,
            SetImageResource: SetImageResource::<Identity, Impl, OFFSET>,
            GetColorProfileResource: GetColorProfileResource::<Identity, Impl, OFFSET>,
            SetColorProfileResource: SetColorProfileResource::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMImageBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMTileBrush as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: Option<&super::super::System::Com::IStream>, imagetype: XPS_IMAGE_TYPE, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetImageType(&self) -> windows_core::Result<XPS_IMAGE_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMImageResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>() -> IXpsOMImageResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readerstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(readerstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMImageResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), core::mem::transmute_copy(&imagetype), windows_core::from_raw_borrowed(&partname)).into()
        }
        unsafe extern "system" fn GetImageType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageResource_Impl::GetImageType(this) {
                Ok(ok__) => {
                    core::ptr::write(imagetype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            GetImageType: GetImageType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMImageResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResourceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMImageResource>;
    fn InsertAt(&self, index: u32, object: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn Append(&self, object: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMImageResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResourceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMImageResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(object, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMImageResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMImageResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMImageResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMImageResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMImageResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    core::ptr::write(part, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMImageResourceCollection as windows_core::Interface>::IID
    }
}
pub trait IXpsOMLinearGradientBrush_Impl: Sized + IXpsOMGradientBrush_Impl {
    fn GetStartPoint(&self) -> windows_core::Result<XPS_POINT>;
    fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetEndPoint(&self) -> windows_core::Result<XPS_POINT>;
    fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMLinearGradientBrush>;
}
impl windows_core::RuntimeName for IXpsOMLinearGradientBrush {}
impl IXpsOMLinearGradientBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>() -> IXpsOMLinearGradientBrush_Vtbl {
        unsafe extern "system" fn GetStartPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *mut XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMLinearGradientBrush_Impl::GetStartPoint(this) {
                Ok(ok__) => {
                    core::ptr::write(startpoint, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMLinearGradientBrush_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetEndPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: *mut XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMLinearGradientBrush_Impl::GetEndPoint(this) {
                Ok(ok__) => {
                    core::ptr::write(endpoint, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: *const XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMLinearGradientBrush_Impl::SetEndPoint(this, core::mem::transmute_copy(&endpoint)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMLinearGradientBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(lineargradientbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, Impl, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMLinearGradientBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMGradientBrush as windows_core::Interface>::IID
    }
}
pub trait IXpsOMMatrixTransform_Impl: Sized + IXpsOMShareable_Impl {
    fn GetMatrix(&self) -> windows_core::Result<XPS_MATRIX>;
    fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
}
impl windows_core::RuntimeName for IXpsOMMatrixTransform {}
impl IXpsOMMatrixTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>() -> IXpsOMMatrixTransform_Vtbl {
        unsafe extern "system" fn GetMatrix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut XPS_MATRIX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMMatrixTransform_Impl::GetMatrix(this) {
                Ok(ok__) => {
                    core::ptr::write(matrix, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const XPS_MATRIX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMMatrixTransform_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMMatrixTransform_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(matrixtransform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMatrix: GetMatrix::<Identity, Impl, OFFSET>,
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMMatrixTransform as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
pub trait IXpsOMNameCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IXpsOMNameCollection {}
impl IXpsOMNameCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>() -> IXpsOMNameCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMNameCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMNameCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMNameCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory_Impl: Sized {
    fn CreatePackage(&self) -> windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromFile(&self, filename: &windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL) -> windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromStream(&self, stream: Option<&super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> windows_core::Result<IXpsOMPackage>;
    fn CreateStoryFragmentsResource(&self, acquiredstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn CreateDocumentStructureResource(&self, acquiredstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMDocumentStructureResource>;
    fn CreateSignatureBlockResource(&self, acquiredstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMSignatureBlockResource>;
    fn CreateRemoteDictionaryResource(&self, dictionary: Option<&IXpsOMDictionary>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreateRemoteDictionaryResourceFromStream(&self, dictionarymarkupstream: Option<&super::super::System::Com::IStream>, dictionaryparturi: Option<&super::Packaging::Opc::IOpcPartUri>, resources: Option<&IXpsOMPartResources>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreatePartResources(&self) -> windows_core::Result<IXpsOMPartResources>;
    fn CreateDocumentSequence(&self, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMDocumentSequence>;
    fn CreateDocument(&self, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMDocument>;
    fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> windows_core::Result<IXpsOMPageReference>;
    fn CreatePage(&self, pagedimensions: *const XPS_SIZE, language: &windows_core::PCWSTR, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPage>;
    fn CreatePageFromStream(&self, pagemarkupstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>, resources: Option<&IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> windows_core::Result<IXpsOMPage>;
    fn CreateCanvas(&self) -> windows_core::Result<IXpsOMCanvas>;
    fn CreateGlyphs(&self, fontresource: Option<&IXpsOMFontResource>) -> windows_core::Result<IXpsOMGlyphs>;
    fn CreatePath(&self) -> windows_core::Result<IXpsOMPath>;
    fn CreateGeometry(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMGeometryFigure>;
    fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn CreateSolidColorBrush(&self, color: *const XPS_COLOR, colorprofile: Option<&IXpsOMColorProfileResource>) -> windows_core::Result<IXpsOMSolidColorBrush>;
    fn CreateColorProfileResource(&self, acquiredstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn CreateImageBrush(&self, image: Option<&IXpsOMImageResource>, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMImageBrush>;
    fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMVisualBrush>;
    fn CreateImageResource(&self, acquiredstream: Option<&super::super::System::Com::IStream>, contenttype: XPS_IMAGE_TYPE, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMImageResource>;
    fn CreatePrintTicketResource(&self, acquiredstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn CreateFontResource(&self, acquiredstream: Option<&super::super::System::Com::IStream>, fontembedding: XPS_FONT_EMBEDDING, parturi: Option<&super::Packaging::Opc::IOpcPartUri>, isobfsourcestream: super::super::Foundation::BOOL) -> windows_core::Result<IXpsOMFontResource>;
    fn CreateGradientStop(&self, color: *const XPS_COLOR, colorprofile: Option<&IXpsOMColorProfileResource>, offset: f32) -> windows_core::Result<IXpsOMGradientStop>;
    fn CreateLinearGradientBrush(&self, gradstop1: Option<&IXpsOMGradientStop>, gradstop2: Option<&IXpsOMGradientStop>, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMLinearGradientBrush>;
    fn CreateRadialGradientBrush(&self, gradstop1: Option<&IXpsOMGradientStop>, gradstop2: Option<&IXpsOMGradientStop>, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> windows_core::Result<IXpsOMRadialGradientBrush>;
    fn CreateCoreProperties(&self, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMCoreProperties>;
    fn CreateDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn CreatePartUriCollection(&self) -> windows_core::Result<IXpsOMPartUriCollection>;
    fn CreatePackageWriterOnFile(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: Option<&IXpsOMCoreProperties>, packagethumbnail: Option<&IXpsOMImageResource>, documentsequenceprintticket: Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream(&self, outputstream: Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: Option<&IXpsOMCoreProperties>, packagethumbnail: Option<&IXpsOMImageResource>, documentsequenceprintticket: Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePartUri(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateReadOnlyStreamOnFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMObjectFactory {}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>() -> IXpsOMObjectFactory_Vtbl {
        unsafe extern "system" fn CreatePackage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePackage(this) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePackageFromFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePackageFromStream(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, storyfragmentsresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateStoryFragmentsResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(storyfragmentsresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, documentstructureresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateDocumentStructureResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(documentstructureresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateSignatureBlockResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(signatureblockresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateRemoteDictionaryResource(this, windows_core::from_raw_borrowed(&dictionary), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(remotedictionaryresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarymarkupstream: *mut core::ffi::c_void, dictionaryparturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, dictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateRemoteDictionaryResourceFromStream(this, windows_core::from_raw_borrowed(&dictionarymarkupstream), windows_core::from_raw_borrowed(&dictionaryparturi), windows_core::from_raw_borrowed(&resources)) {
                Ok(ok__) => {
                    core::ptr::write(dictionaryresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePartResources(this) {
                Ok(ok__) => {
                    core::ptr::write(partresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentSequence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateDocumentSequence(this, windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(documentsequence, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateDocument(this, windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(document, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageReference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePageReference(this, core::mem::transmute_copy(&advisorypagedimensions)) {
                Ok(ok__) => {
                    core::ptr::write(pagereference, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: windows_core::PCWSTR, parturi: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePage(this, core::mem::transmute_copy(&pagedimensions), core::mem::transmute(&language), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(page, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagemarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePageFromStream(this, windows_core::from_raw_borrowed(&pagemarkupstream), windows_core::from_raw_borrowed(&parturi), windows_core::from_raw_borrowed(&resources), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    core::ptr::write(page, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCanvas<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, canvas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateCanvas(this) {
                Ok(ok__) => {
                    core::ptr::write(canvas, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut core::ffi::c_void, glyphs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateGlyphs(this, windows_core::from_raw_borrowed(&fontresource)) {
                Ok(ok__) => {
                    core::ptr::write(glyphs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePath(this) {
                Ok(ok__) => {
                    core::ptr::write(path, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateGeometry(this) {
                Ok(ok__) => {
                    core::ptr::write(geometry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryFigure<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateGeometryFigure(this, core::mem::transmute_copy(&startpoint)) {
                Ok(ok__) => {
                    core::ptr::write(figure, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateMatrixTransform(this, core::mem::transmute_copy(&matrix)) {
                Ok(ok__) => {
                    core::ptr::write(transform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateSolidColorBrush(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile)) {
                Ok(ok__) => {
                    core::ptr::write(solidcolorbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorProfileResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, colorprofileresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateColorProfileResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(colorprofileresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateImageBrush(this, windows_core::from_raw_borrowed(&image), core::mem::transmute_copy(&viewbox), core::mem::transmute_copy(&viewport)) {
                Ok(ok__) => {
                    core::ptr::write(imagebrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisualBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateVisualBrush(this, core::mem::transmute_copy(&viewbox), core::mem::transmute_copy(&viewport)) {
                Ok(ok__) => {
                    core::ptr::write(visualbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateImageResource(this, windows_core::from_raw_borrowed(&acquiredstream), core::mem::transmute_copy(&contenttype), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(imageresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintTicketResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePrintTicketResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(printticketresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut core::ffi::c_void, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateFontResource(this, windows_core::from_raw_borrowed(&acquiredstream), core::mem::transmute_copy(&fontembedding), windows_core::from_raw_borrowed(&parturi), core::mem::transmute_copy(&isobfsourcestream)) {
                Ok(ok__) => {
                    core::ptr::write(fontresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void, offset: f32, gradientstop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateGradientStop(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    core::ptr::write(gradientstop, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradstop1: *mut core::ffi::c_void, gradstop2: *mut core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateLinearGradientBrush(this, windows_core::from_raw_borrowed(&gradstop1), windows_core::from_raw_borrowed(&gradstop2), core::mem::transmute_copy(&startpoint), core::mem::transmute_copy(&endpoint)) {
                Ok(ok__) => {
                    core::ptr::write(lineargradientbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradstop1: *mut core::ffi::c_void, gradstop2: *mut core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateRadialGradientBrush(this, windows_core::from_raw_borrowed(&gradstop1), windows_core::from_raw_borrowed(&gradstop2), core::mem::transmute_copy(&centerpoint), core::mem::transmute_copy(&gradientorigin), core::mem::transmute_copy(&radiisizes)) {
                Ok(ok__) => {
                    core::ptr::write(radialgradientbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoreProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateCoreProperties(this, windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(coreproperties, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDictionary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateDictionary(this) {
                Ok(ok__) => {
                    core::ptr::write(dictionary, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUriCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturicollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePartUriCollection(this) {
                Ok(ok__) => {
                    core::ptr::write(parturicollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePackageWriterOnFile(
                this,
                core::mem::transmute(&filename),
                core::mem::transmute_copy(&securityattributes),
                core::mem::transmute_copy(&flagsandattributes),
                core::mem::transmute_copy(&optimizemarkupsize),
                core::mem::transmute_copy(&interleaving),
                windows_core::from_raw_borrowed(&documentsequencepartname),
                windows_core::from_raw_borrowed(&coreproperties),
                windows_core::from_raw_borrowed(&packagethumbnail),
                windows_core::from_raw_borrowed(&documentsequenceprintticket),
                windows_core::from_raw_borrowed(&discardcontrolpartname),
            ) {
                Ok(ok__) => {
                    core::ptr::write(packagewriter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePackageWriterOnStream(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&interleaving), windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&coreproperties), windows_core::from_raw_borrowed(&packagethumbnail), windows_core::from_raw_borrowed(&documentsequenceprintticket), windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                Ok(ok__) => {
                    core::ptr::write(packagewriter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreatePartUri(this, core::mem::transmute(&uri)) {
                Ok(ok__) => {
                    core::ptr::write(parturi, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory_Impl::CreateReadOnlyStreamOnFile(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    core::ptr::write(stream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackage: CreatePackage::<Identity, Impl, OFFSET>,
            CreatePackageFromFile: CreatePackageFromFile::<Identity, Impl, OFFSET>,
            CreatePackageFromStream: CreatePackageFromStream::<Identity, Impl, OFFSET>,
            CreateStoryFragmentsResource: CreateStoryFragmentsResource::<Identity, Impl, OFFSET>,
            CreateDocumentStructureResource: CreateDocumentStructureResource::<Identity, Impl, OFFSET>,
            CreateSignatureBlockResource: CreateSignatureBlockResource::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResource: CreateRemoteDictionaryResource::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream: CreateRemoteDictionaryResourceFromStream::<Identity, Impl, OFFSET>,
            CreatePartResources: CreatePartResources::<Identity, Impl, OFFSET>,
            CreateDocumentSequence: CreateDocumentSequence::<Identity, Impl, OFFSET>,
            CreateDocument: CreateDocument::<Identity, Impl, OFFSET>,
            CreatePageReference: CreatePageReference::<Identity, Impl, OFFSET>,
            CreatePage: CreatePage::<Identity, Impl, OFFSET>,
            CreatePageFromStream: CreatePageFromStream::<Identity, Impl, OFFSET>,
            CreateCanvas: CreateCanvas::<Identity, Impl, OFFSET>,
            CreateGlyphs: CreateGlyphs::<Identity, Impl, OFFSET>,
            CreatePath: CreatePath::<Identity, Impl, OFFSET>,
            CreateGeometry: CreateGeometry::<Identity, Impl, OFFSET>,
            CreateGeometryFigure: CreateGeometryFigure::<Identity, Impl, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, Impl, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, Impl, OFFSET>,
            CreateColorProfileResource: CreateColorProfileResource::<Identity, Impl, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, Impl, OFFSET>,
            CreateVisualBrush: CreateVisualBrush::<Identity, Impl, OFFSET>,
            CreateImageResource: CreateImageResource::<Identity, Impl, OFFSET>,
            CreatePrintTicketResource: CreatePrintTicketResource::<Identity, Impl, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, Impl, OFFSET>,
            CreateGradientStop: CreateGradientStop::<Identity, Impl, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, Impl, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, Impl, OFFSET>,
            CreateCoreProperties: CreateCoreProperties::<Identity, Impl, OFFSET>,
            CreateDictionary: CreateDictionary::<Identity, Impl, OFFSET>,
            CreatePartUriCollection: CreatePartUriCollection::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnFile: CreatePackageWriterOnFile::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnStream: CreatePackageWriterOnStream::<Identity, Impl, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, Impl, OFFSET>,
            CreateReadOnlyStreamOnFile: CreateReadOnlyStreamOnFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory1_Impl: Sized + IXpsOMObjectFactory_Impl {
    fn GetDocumentTypeFromFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn GetDocumentTypeFromStream(&self, xpsdocumentstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn ConvertHDPhotoToJpegXR(&self, imageresource: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn ConvertJpegXRToHDPhoto(&self, imageresource: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn CreatePackageWriterOnFile1(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: Option<&IXpsOMCoreProperties>, packagethumbnail: Option<&IXpsOMImageResource>, documentsequenceprintticket: Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: Option<&super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream1(&self, outputstream: Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: Option<&IXpsOMCoreProperties>, packagethumbnail: Option<&IXpsOMImageResource>, documentsequenceprintticket: Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: Option<&super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackage1(&self) -> windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromStream1(&self, stream: Option<&super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromFile1(&self, filename: &windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL) -> windows_core::Result<IXpsOMPackage1>;
    fn CreatePage1(&self, pagedimensions: *const XPS_SIZE, language: &windows_core::PCWSTR, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPage1>;
    fn CreatePageFromStream1(&self, pagemarkupstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>, resources: Option<&IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> windows_core::Result<IXpsOMPage1>;
    fn CreateRemoteDictionaryResourceFromStream1(&self, dictionarymarkupstream: Option<&super::super::System::Com::IStream>, parturi: Option<&super::Packaging::Opc::IOpcPartUri>, resources: Option<&IXpsOMPartResources>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMObjectFactory1 {}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>() -> IXpsOMObjectFactory1_Vtbl {
        unsafe extern "system" fn GetDocumentTypeFromFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::GetDocumentTypeFromFile(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    core::ptr::write(documenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsdocumentstream: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::GetDocumentTypeFromStream(this, windows_core::from_raw_borrowed(&xpsdocumentstream)) {
                Ok(ok__) => {
                    core::ptr::write(documenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMObjectFactory1_Impl::ConvertHDPhotoToJpegXR(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMObjectFactory1_Impl::ConvertJpegXRToHDPhoto(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreatePackageWriterOnFile1(
                this,
                core::mem::transmute(&filename),
                core::mem::transmute_copy(&securityattributes),
                core::mem::transmute_copy(&flagsandattributes),
                core::mem::transmute_copy(&optimizemarkupsize),
                core::mem::transmute_copy(&interleaving),
                windows_core::from_raw_borrowed(&documentsequencepartname),
                windows_core::from_raw_borrowed(&coreproperties),
                windows_core::from_raw_borrowed(&packagethumbnail),
                windows_core::from_raw_borrowed(&documentsequenceprintticket),
                windows_core::from_raw_borrowed(&discardcontrolpartname),
                core::mem::transmute_copy(&documenttype),
            ) {
                Ok(ok__) => {
                    core::ptr::write(packagewriter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreatePackageWriterOnStream1(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&interleaving), windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&coreproperties), windows_core::from_raw_borrowed(&packagethumbnail), windows_core::from_raw_borrowed(&documentsequenceprintticket), windows_core::from_raw_borrowed(&discardcontrolpartname), core::mem::transmute_copy(&documenttype)) {
                Ok(ok__) => {
                    core::ptr::write(packagewriter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreatePackage1(this) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreatePackageFromStream1(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreatePackageFromFile1(this, core::mem::transmute(&filename), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    core::ptr::write(package, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: windows_core::PCWSTR, parturi: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreatePage1(this, core::mem::transmute_copy(&pagedimensions), core::mem::transmute(&language), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    core::ptr::write(page, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagemarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreatePageFromStream1(this, windows_core::from_raw_borrowed(&pagemarkupstream), windows_core::from_raw_borrowed(&parturi), windows_core::from_raw_borrowed(&resources), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    core::ptr::write(page, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarymarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, dictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMObjectFactory1_Impl::CreateRemoteDictionaryResourceFromStream1(this, windows_core::from_raw_borrowed(&dictionarymarkupstream), windows_core::from_raw_borrowed(&parturi), windows_core::from_raw_borrowed(&resources)) {
                Ok(ok__) => {
                    core::ptr::write(dictionaryresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMObjectFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentTypeFromFile: GetDocumentTypeFromFile::<Identity, Impl, OFFSET>,
            GetDocumentTypeFromStream: GetDocumentTypeFromStream::<Identity, Impl, OFFSET>,
            ConvertHDPhotoToJpegXR: ConvertHDPhotoToJpegXR::<Identity, Impl, OFFSET>,
            ConvertJpegXRToHDPhoto: ConvertJpegXRToHDPhoto::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnFile1: CreatePackageWriterOnFile1::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnStream1: CreatePackageWriterOnStream1::<Identity, Impl, OFFSET>,
            CreatePackage1: CreatePackage1::<Identity, Impl, OFFSET>,
            CreatePackageFromStream1: CreatePackageFromStream1::<Identity, Impl, OFFSET>,
            CreatePackageFromFile1: CreatePackageFromFile1::<Identity, Impl, OFFSET>,
            CreatePage1: CreatePage1::<Identity, Impl, OFFSET>,
            CreatePageFromStream1: CreatePageFromStream1::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream1: CreateRemoteDictionaryResourceFromStream1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory1 as windows_core::Interface>::IID || iid == &<IXpsOMObjectFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage_Impl: Sized {
    fn GetDocumentSequence(&self) -> windows_core::Result<IXpsOMDocumentSequence>;
    fn SetDocumentSequence(&self, documentsequence: Option<&IXpsOMDocumentSequence>) -> windows_core::Result<()>;
    fn GetCoreProperties(&self) -> windows_core::Result<IXpsOMCoreProperties>;
    fn SetCoreProperties(&self, coreproperties: Option<&IXpsOMCoreProperties>) -> windows_core::Result<()>;
    fn GetDiscardControlPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetDiscardControlPartName(&self, discardcontrolparturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&self, imageresource: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn WriteToFile(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteToStream(&self, stream: Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackage {}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>() -> IXpsOMPackage_Vtbl {
        unsafe extern "system" fn GetDocumentSequence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPackage_Impl::GetDocumentSequence(this) {
                Ok(ok__) => {
                    core::ptr::write(documentsequence, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentSequence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage_Impl::SetDocumentSequence(this, windows_core::from_raw_borrowed(&documentsequence)).into()
        }
        unsafe extern "system" fn GetCoreProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPackage_Impl::GetCoreProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(coreproperties, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoreProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage_Impl::SetCoreProperties(this, windows_core::from_raw_borrowed(&coreproperties)).into()
        }
        unsafe extern "system" fn GetDiscardControlPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discardcontrolparturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPackage_Impl::GetDiscardControlPartName(this) {
                Ok(ok__) => {
                    core::ptr::write(discardcontrolparturi, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscardControlPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discardcontrolparturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage_Impl::SetDiscardControlPartName(this, windows_core::from_raw_borrowed(&discardcontrolparturi)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPackage_Impl::GetThumbnailResource(this) {
                Ok(ok__) => {
                    core::ptr::write(imageresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage_Impl::SetThumbnailResource(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn WriteToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage_Impl::WriteToFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes), core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn WriteToStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage_Impl::WriteToStream(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentSequence: GetDocumentSequence::<Identity, Impl, OFFSET>,
            SetDocumentSequence: SetDocumentSequence::<Identity, Impl, OFFSET>,
            GetCoreProperties: GetCoreProperties::<Identity, Impl, OFFSET>,
            SetCoreProperties: SetCoreProperties::<Identity, Impl, OFFSET>,
            GetDiscardControlPartName: GetDiscardControlPartName::<Identity, Impl, OFFSET>,
            SetDiscardControlPartName: SetDiscardControlPartName::<Identity, Impl, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, Impl, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, Impl, OFFSET>,
            WriteToFile: WriteToFile::<Identity, Impl, OFFSET>,
            WriteToStream: WriteToStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage1_Impl: Sized + IXpsOMPackage_Impl {
    fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn WriteToFile1(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
    fn WriteToStream1(&self, outputstream: Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackage1 {}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>() -> IXpsOMPackage1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPackage1_Impl::GetDocumentType(this) {
                Ok(ok__) => {
                    core::ptr::write(documenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToFile1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage1_Impl::WriteToFile1(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
        }
        unsafe extern "system" fn WriteToStream1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackage1_Impl::WriteToStream1(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMPackage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            WriteToFile1: WriteToFile1::<Identity, Impl, OFFSET>,
            WriteToStream1: WriteToStream1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackage1 as windows_core::Interface>::IID || iid == &<IXpsOMPackage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageTarget_Impl: Sized {
    fn CreateXpsOMPackageWriter(&self, documentsequencepartname: Option<&super::Packaging::Opc::IOpcPartUri>, documentsequenceprintticket: Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackageTarget {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageTarget_Impl, const OFFSET: isize>() -> IXpsOMPackageTarget_Vtbl {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPackageTarget_Impl::CreateXpsOMPackageWriter(this, windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&documentsequenceprintticket), windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                Ok(ok__) => {
                    core::ptr::write(packagewriter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateXpsOMPackageWriter: CreateXpsOMPackageWriter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter_Impl: Sized {
    fn StartNewDocument(&self, documentpartname: Option<&super::Packaging::Opc::IOpcPartUri>, documentprintticket: Option<&IXpsOMPrintTicketResource>, documentstructure: Option<&IXpsOMDocumentStructureResource>, signatureblockresources: Option<&IXpsOMSignatureBlockResourceCollection>, restrictedfonts: Option<&IXpsOMPartUriCollection>) -> windows_core::Result<()>;
    fn AddPage(&self, page: Option<&IXpsOMPage>, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: Option<&IXpsOMPartUriCollection>, storyfragments: Option<&IXpsOMStoryFragmentsResource>, pageprintticket: Option<&IXpsOMPrintTicketResource>, pagethumbnail: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn AddResource(&self, resource: Option<&IXpsOMResource>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn IsClosed(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackageWriter {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>() -> IXpsOMPackageWriter_Vtbl {
        unsafe extern "system" fn StartNewDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentpartname: *mut core::ffi::c_void, documentprintticket: *mut core::ffi::c_void, documentstructure: *mut core::ffi::c_void, signatureblockresources: *mut core::ffi::c_void, restrictedfonts: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackageWriter_Impl::StartNewDocument(this, windows_core::from_raw_borrowed(&documentpartname), windows_core::from_raw_borrowed(&documentprintticket), windows_core::from_raw_borrowed(&documentstructure), windows_core::from_raw_borrowed(&signatureblockresources), windows_core::from_raw_borrowed(&restrictedfonts)).into()
        }
        unsafe extern "system" fn AddPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut core::ffi::c_void, storyfragments: *mut core::ffi::c_void, pageprintticket: *mut core::ffi::c_void, pagethumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackageWriter_Impl::AddPage(this, windows_core::from_raw_borrowed(&page), core::mem::transmute_copy(&advisorypagedimensions), windows_core::from_raw_borrowed(&discardableresourceparts), windows_core::from_raw_borrowed(&storyfragments), windows_core::from_raw_borrowed(&pageprintticket), windows_core::from_raw_borrowed(&pagethumbnail)).into()
        }
        unsafe extern "system" fn AddResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackageWriter_Impl::AddResource(this, windows_core::from_raw_borrowed(&resource)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackageWriter_Impl::Close(this).into()
        }
        unsafe extern "system" fn IsClosed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPackageWriter_Impl::IsClosed(this) {
                Ok(ok__) => {
                    core::ptr::write(isclosed, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartNewDocument: StartNewDocument::<Identity, Impl, OFFSET>,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            IsClosed: IsClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter3D_Impl: Sized + IXpsOMPackageWriter_Impl {
    fn AddModelTexture(&self, texturepartname: Option<&super::Packaging::Opc::IOpcPartUri>, texturedata: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetModelPrintTicket(&self, printticketpartname: Option<&super::Packaging::Opc::IOpcPartUri>, printticketdata: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackageWriter3D {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>() -> IXpsOMPackageWriter3D_Vtbl {
        unsafe extern "system" fn AddModelTexture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, texturepartname: *mut core::ffi::c_void, texturedata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackageWriter3D_Impl::AddModelTexture(this, windows_core::from_raw_borrowed(&texturepartname), windows_core::from_raw_borrowed(&texturedata)).into()
        }
        unsafe extern "system" fn SetModelPrintTicket<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketpartname: *mut core::ffi::c_void, printticketdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPackageWriter3D_Impl::SetModelPrintTicket(this, windows_core::from_raw_borrowed(&printticketpartname), windows_core::from_raw_borrowed(&printticketdata)).into()
        }
        Self {
            base__: IXpsOMPackageWriter_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddModelTexture: AddModelTexture::<Identity, Impl, OFFSET>,
            SetModelPrintTicket: SetModelPrintTicket::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter3D as windows_core::Interface>::IID || iid == &<IXpsOMPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPageReference>;
    fn GetVisuals(&self) -> windows_core::Result<IXpsOMVisualCollection>;
    fn GetPageDimensions(&self) -> windows_core::Result<XPS_SIZE>;
    fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()>;
    fn GetContentBox(&self) -> windows_core::Result<XPS_RECT>;
    fn SetContentBox(&self, contentbox: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetBleedBox(&self) -> windows_core::Result<XPS_RECT>;
    fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLanguage(&self, language: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIsHyperlinkTarget(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(&self, ishyperlinktarget: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&self, resourcedictionary: Option<&IXpsOMDictionary>) -> windows_core::Result<()>;
    fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&self, remotedictionaryresource: Option<&IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn Write(&self, stream: Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> windows_core::Result<windows_core::PWSTR>;
    fn Clone(&self) -> windows_core::Result<IXpsOMPage>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPage {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>() -> IXpsOMPage_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(pagereference, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisuals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visuals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetVisuals(this) {
                Ok(ok__) => {
                    core::ptr::write(visuals, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageDimensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetPageDimensions(this) {
                Ok(ok__) => {
                    core::ptr::write(pagedimensions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageDimensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetPageDimensions(this, core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetContentBox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentbox: *mut XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetContentBox(this) {
                Ok(ok__) => {
                    core::ptr::write(contentbox, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentBox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentbox: *const XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetContentBox(this, core::mem::transmute_copy(&contentbox)).into()
        }
        unsafe extern "system" fn GetBleedBox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bleedbox: *mut XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetBleedBox(this) {
                Ok(ok__) => {
                    core::ptr::write(bleedbox, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBleedBox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bleedbox: *const XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetBleedBox(this, core::mem::transmute_copy(&bleedbox)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetLanguage(this) {
                Ok(ok__) => {
                    core::ptr::write(language, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetIsHyperlinkTarget(this) {
                Ok(ok__) => {
                    core::ptr::write(ishyperlinktarget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetIsHyperlinkTarget(this, core::mem::transmute_copy(&ishyperlinktarget)).into()
        }
        unsafe extern "system" fn GetDictionary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetDictionary(this) {
                Ok(ok__) => {
                    core::ptr::write(resourcedictionary, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetDictionaryLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(resourcedictionary, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetDictionaryLocal(this, windows_core::from_raw_borrowed(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GetDictionaryResource(this) {
                Ok(ok__) => {
                    core::ptr::write(remotedictionaryresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::SetDictionaryResource(this, windows_core::from_raw_borrowed(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage_Impl::Write(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::GenerateUnusedLookupKey(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(page, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetVisuals: GetVisuals::<Identity, Impl, OFFSET>,
            GetPageDimensions: GetPageDimensions::<Identity, Impl, OFFSET>,
            SetPageDimensions: SetPageDimensions::<Identity, Impl, OFFSET>,
            GetContentBox: GetContentBox::<Identity, Impl, OFFSET>,
            SetContentBox: SetContentBox::<Identity, Impl, OFFSET>,
            GetBleedBox: GetBleedBox::<Identity, Impl, OFFSET>,
            SetBleedBox: SetBleedBox::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, Impl, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, Impl, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, Impl, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            GenerateUnusedLookupKey: GenerateUnusedLookupKey::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPage as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage1_Impl: Sized + IXpsOMPage_Impl {
    fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&self, stream: Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPage1 {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: isize>() -> IXpsOMPage1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPage1_Impl::GetDocumentType(this) {
                Ok(ok__) => {
                    core::ptr::write(documenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPage1_Impl::Write1(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMPage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            Write1: Write1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPage1 as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMPage as windows_core::Interface>::IID
    }
}
pub trait IXpsOMPageReference_Impl: Sized {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument>;
    fn GetPage(&self) -> windows_core::Result<IXpsOMPage>;
    fn SetPage(&self, page: Option<&IXpsOMPage>) -> windows_core::Result<()>;
    fn DiscardPage(&self) -> windows_core::Result<()>;
    fn IsPageLoaded(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetAdvisoryPageDimensions(&self) -> windows_core::Result<XPS_SIZE>;
    fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()>;
    fn GetStoryFragmentsResource(&self) -> windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn SetStoryFragmentsResource(&self, storyfragmentsresource: Option<&IXpsOMStoryFragmentsResource>) -> windows_core::Result<()>;
    fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: Option<&IXpsOMPrintTicketResource>) -> windows_core::Result<()>;
    fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&self, imageresource: Option<&IXpsOMImageResource>) -> windows_core::Result<()>;
    fn CollectLinkTargets(&self) -> windows_core::Result<IXpsOMNameCollection>;
    fn CollectPartResources(&self) -> windows_core::Result<IXpsOMPartResources>;
    fn HasRestrictedFonts(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Clone(&self) -> windows_core::Result<IXpsOMPageReference>;
}
impl windows_core::RuntimeName for IXpsOMPageReference {}
impl IXpsOMPageReference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>() -> IXpsOMPageReference_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(document, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::GetPage(this) {
                Ok(ok__) => {
                    core::ptr::write(page, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReference_Impl::SetPage(this, windows_core::from_raw_borrowed(&page)).into()
        }
        unsafe extern "system" fn DiscardPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReference_Impl::DiscardPage(this).into()
        }
        unsafe extern "system" fn IsPageLoaded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::IsPageLoaded(this) {
                Ok(ok__) => {
                    core::ptr::write(ispageloaded, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::GetAdvisoryPageDimensions(this) {
                Ok(ok__) => {
                    core::ptr::write(pagedimensions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReference_Impl::SetAdvisoryPageDimensions(this, core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyfragmentsresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::GetStoryFragmentsResource(this) {
                Ok(ok__) => {
                    core::ptr::write(storyfragmentsresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyfragmentsresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReference_Impl::SetStoryFragmentsResource(this, windows_core::from_raw_borrowed(&storyfragmentsresource)).into()
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::GetPrintTicketResource(this) {
                Ok(ok__) => {
                    core::ptr::write(printticketresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReference_Impl::SetPrintTicketResource(this, windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::GetThumbnailResource(this) {
                Ok(ok__) => {
                    core::ptr::write(imageresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReference_Impl::SetThumbnailResource(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn CollectLinkTargets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linktargets: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::CollectLinkTargets(this) {
                Ok(ok__) => {
                    core::ptr::write(linktargets, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectPartResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::CollectPartResources(this) {
                Ok(ok__) => {
                    core::ptr::write(partresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasRestrictedFonts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::HasRestrictedFonts(this) {
                Ok(ok__) => {
                    core::ptr::write(restrictedfonts, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReference_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(pagereference, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetPage: GetPage::<Identity, Impl, OFFSET>,
            SetPage: SetPage::<Identity, Impl, OFFSET>,
            DiscardPage: DiscardPage::<Identity, Impl, OFFSET>,
            IsPageLoaded: IsPageLoaded::<Identity, Impl, OFFSET>,
            GetAdvisoryPageDimensions: GetAdvisoryPageDimensions::<Identity, Impl, OFFSET>,
            SetAdvisoryPageDimensions: SetAdvisoryPageDimensions::<Identity, Impl, OFFSET>,
            GetStoryFragmentsResource: GetStoryFragmentsResource::<Identity, Impl, OFFSET>,
            SetStoryFragmentsResource: SetStoryFragmentsResource::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, Impl, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, Impl, OFFSET>,
            CollectLinkTargets: CollectLinkTargets::<Identity, Impl, OFFSET>,
            CollectPartResources: CollectPartResources::<Identity, Impl, OFFSET>,
            HasRestrictedFonts: HasRestrictedFonts::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPageReference as windows_core::Interface>::IID
    }
}
pub trait IXpsOMPageReferenceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMPageReference>;
    fn InsertAt(&self, index: u32, pagereference: Option<&IXpsOMPageReference>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, pagereference: Option<&IXpsOMPageReference>) -> windows_core::Result<()>;
    fn Append(&self, pagereference: Option<&IXpsOMPageReference>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMPageReferenceCollection {}
impl IXpsOMPageReferenceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>() -> IXpsOMPageReferenceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReferenceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPageReferenceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pagereference, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReferenceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&pagereference)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReferenceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReferenceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&pagereference)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPageReferenceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&pagereference)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPageReferenceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPart_Impl: Sized {
    fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetPartName(&self, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPart {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPart_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: isize>() -> IXpsOMPart_Vtbl {
        unsafe extern "system" fn GetPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPart_Impl::GetPartName(this) {
                Ok(ok__) => {
                    core::ptr::write(parturi, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPart_Impl::SetPartName(this, windows_core::from_raw_borrowed(&parturi)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            SetPartName: SetPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
pub trait IXpsOMPartResources_Impl: Sized {
    fn GetFontResources(&self) -> windows_core::Result<IXpsOMFontResourceCollection>;
    fn GetImageResources(&self) -> windows_core::Result<IXpsOMImageResourceCollection>;
    fn GetColorProfileResources(&self) -> windows_core::Result<IXpsOMColorProfileResourceCollection>;
    fn GetRemoteDictionaryResources(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResourceCollection>;
}
impl windows_core::RuntimeName for IXpsOMPartResources {}
impl IXpsOMPartResources_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>() -> IXpsOMPartResources_Vtbl {
        unsafe extern "system" fn GetFontResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPartResources_Impl::GetFontResources(this) {
                Ok(ok__) => {
                    core::ptr::write(fontresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPartResources_Impl::GetImageResources(this) {
                Ok(ok__) => {
                    core::ptr::write(imageresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProfileResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPartResources_Impl::GetColorProfileResources(this) {
                Ok(ok__) => {
                    core::ptr::write(colorprofileresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPartResources_Impl::GetRemoteDictionaryResources(this) {
                Ok(ok__) => {
                    core::ptr::write(dictionaryresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontResources: GetFontResources::<Identity, Impl, OFFSET>,
            GetImageResources: GetImageResources::<Identity, Impl, OFFSET>,
            GetColorProfileResources: GetColorProfileResources::<Identity, Impl, OFFSET>,
            GetRemoteDictionaryResources: GetRemoteDictionaryResources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPartResources as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPartUriCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn InsertAt(&self, index: u32, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn Append(&self, parturi: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPartUriCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPartUriCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>() -> IXpsOMPartUriCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPartUriCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPartUriCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(parturi, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPartUriCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&parturi)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPartUriCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPartUriCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&parturi)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPartUriCollection_Impl::Append(this, windows_core::from_raw_borrowed(&parturi)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPartUriCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMPath_Impl: Sized + IXpsOMVisual_Impl {
    fn GetGeometry(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn GetGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn SetGeometryLocal(&self, geometry: Option<&IXpsOMGeometry>) -> windows_core::Result<()>;
    fn GetGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetGeometryLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(&self, shortdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(&self, longdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSnapsToPixels(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSnapsToPixels(&self, snapstopixels: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetStrokeBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetStrokeBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetStrokeBrushLocal(&self, brush: Option<&IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetStrokeBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetStrokeBrushLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStrokeDashes(&self) -> windows_core::Result<IXpsOMDashCollection>;
    fn GetStrokeDashCap(&self) -> windows_core::Result<XPS_DASH_CAP>;
    fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> windows_core::Result<()>;
    fn GetStrokeDashOffset(&self) -> windows_core::Result<f32>;
    fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> windows_core::Result<()>;
    fn GetStrokeStartLineCap(&self) -> windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> windows_core::Result<()>;
    fn GetStrokeEndLineCap(&self) -> windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> windows_core::Result<()>;
    fn GetStrokeLineJoin(&self) -> windows_core::Result<XPS_LINE_JOIN>;
    fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> windows_core::Result<()>;
    fn GetStrokeMiterLimit(&self) -> windows_core::Result<f32>;
    fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> windows_core::Result<()>;
    fn GetStrokeThickness(&self) -> windows_core::Result<f32>;
    fn SetStrokeThickness(&self, strokethickness: f32) -> windows_core::Result<()>;
    fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&self, brush: Option<&IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetFillBrushLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMPath>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMPath {}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMPath_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>() -> IXpsOMPath_Vtbl {
        unsafe extern "system" fn GetGeometry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetGeometry(this) {
                Ok(ok__) => {
                    core::ptr::write(geometry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetGeometryLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(geometry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetGeometryLocal(this, windows_core::from_raw_borrowed(&geometry)).into()
        }
        unsafe extern "system" fn GetGeometryLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetGeometryLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(lookup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetGeometryLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetAccessibilityShortDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(shortdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetAccessibilityShortDescription(this, core::mem::transmute(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetAccessibilityLongDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(longdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetAccessibilityLongDescription(this, core::mem::transmute(&longdescription)).into()
        }
        unsafe extern "system" fn GetSnapsToPixels<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetSnapsToPixels(this) {
                Ok(ok__) => {
                    core::ptr::write(snapstopixels, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapsToPixels<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetSnapsToPixels(this, core::mem::transmute_copy(&snapstopixels)).into()
        }
        unsafe extern "system" fn GetStrokeBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeBrush(this) {
                Ok(ok__) => {
                    core::ptr::write(brush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeBrushLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(brush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeBrushLocal(this, windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeBrushLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(lookup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeBrushLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn GetStrokeDashes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeDashes(this) {
                Ok(ok__) => {
                    core::ptr::write(strokedashes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeDashCap(this) {
                Ok(ok__) => {
                    core::ptr::write(strokedashcap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeDashCap(this, core::mem::transmute_copy(&strokedashcap)).into()
        }
        unsafe extern "system" fn GetStrokeDashOffset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashoffset: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeDashOffset(this) {
                Ok(ok__) => {
                    core::ptr::write(strokedashoffset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashoffset: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeDashOffset(this, core::mem::transmute_copy(&strokedashoffset)).into()
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeStartLineCap(this) {
                Ok(ok__) => {
                    core::ptr::write(strokestartlinecap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeStartLineCap(this, core::mem::transmute_copy(&strokestartlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeEndLineCap(this) {
                Ok(ok__) => {
                    core::ptr::write(strokeendlinecap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeEndLineCap(this, core::mem::transmute_copy(&strokeendlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeLineJoin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeLineJoin(this) {
                Ok(ok__) => {
                    core::ptr::write(strokelinejoin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeLineJoin(this, core::mem::transmute_copy(&strokelinejoin)).into()
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokemiterlimit: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeMiterLimit(this) {
                Ok(ok__) => {
                    core::ptr::write(strokemiterlimit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokemiterlimit: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeMiterLimit(this, core::mem::transmute_copy(&strokemiterlimit)).into()
        }
        unsafe extern "system" fn GetStrokeThickness<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokethickness: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetStrokeThickness(this) {
                Ok(ok__) => {
                    core::ptr::write(strokethickness, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokethickness: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetStrokeThickness(this, core::mem::transmute_copy(&strokethickness)).into()
        }
        unsafe extern "system" fn GetFillBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetFillBrush(this) {
                Ok(ok__) => {
                    core::ptr::write(brush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetFillBrushLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(brush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetFillBrushLocal(this, windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::GetFillBrushLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(lookup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPath_Impl::SetFillBrushLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPath_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(path, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGeometry: GetGeometry::<Identity, Impl, OFFSET>,
            GetGeometryLocal: GetGeometryLocal::<Identity, Impl, OFFSET>,
            SetGeometryLocal: SetGeometryLocal::<Identity, Impl, OFFSET>,
            GetGeometryLookup: GetGeometryLookup::<Identity, Impl, OFFSET>,
            SetGeometryLookup: SetGeometryLookup::<Identity, Impl, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            GetSnapsToPixels: GetSnapsToPixels::<Identity, Impl, OFFSET>,
            SetSnapsToPixels: SetSnapsToPixels::<Identity, Impl, OFFSET>,
            GetStrokeBrush: GetStrokeBrush::<Identity, Impl, OFFSET>,
            GetStrokeBrushLocal: GetStrokeBrushLocal::<Identity, Impl, OFFSET>,
            SetStrokeBrushLocal: SetStrokeBrushLocal::<Identity, Impl, OFFSET>,
            GetStrokeBrushLookup: GetStrokeBrushLookup::<Identity, Impl, OFFSET>,
            SetStrokeBrushLookup: SetStrokeBrushLookup::<Identity, Impl, OFFSET>,
            GetStrokeDashes: GetStrokeDashes::<Identity, Impl, OFFSET>,
            GetStrokeDashCap: GetStrokeDashCap::<Identity, Impl, OFFSET>,
            SetStrokeDashCap: SetStrokeDashCap::<Identity, Impl, OFFSET>,
            GetStrokeDashOffset: GetStrokeDashOffset::<Identity, Impl, OFFSET>,
            SetStrokeDashOffset: SetStrokeDashOffset::<Identity, Impl, OFFSET>,
            GetStrokeStartLineCap: GetStrokeStartLineCap::<Identity, Impl, OFFSET>,
            SetStrokeStartLineCap: SetStrokeStartLineCap::<Identity, Impl, OFFSET>,
            GetStrokeEndLineCap: GetStrokeEndLineCap::<Identity, Impl, OFFSET>,
            SetStrokeEndLineCap: SetStrokeEndLineCap::<Identity, Impl, OFFSET>,
            GetStrokeLineJoin: GetStrokeLineJoin::<Identity, Impl, OFFSET>,
            SetStrokeLineJoin: SetStrokeLineJoin::<Identity, Impl, OFFSET>,
            GetStrokeMiterLimit: GetStrokeMiterLimit::<Identity, Impl, OFFSET>,
            SetStrokeMiterLimit: SetStrokeMiterLimit::<Identity, Impl, OFFSET>,
            GetStrokeThickness: GetStrokeThickness::<Identity, Impl, OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Identity, Impl, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, Impl, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, Impl, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, Impl, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, Impl, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPath as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMVisual as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPrintTicketResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: Option<&super::super::System::Com::IStream>, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPrintTicketResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPrintTicketResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>() -> IXpsOMPrintTicketResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMPrintTicketResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(stream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMPrintTicketResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPrintTicketResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
pub trait IXpsOMRadialGradientBrush_Impl: Sized + IXpsOMGradientBrush_Impl {
    fn GetCenter(&self) -> windows_core::Result<XPS_POINT>;
    fn SetCenter(&self, center: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetRadiiSizes(&self) -> windows_core::Result<XPS_SIZE>;
    fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> windows_core::Result<()>;
    fn GetGradientOrigin(&self) -> windows_core::Result<XPS_POINT>;
    fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMRadialGradientBrush>;
}
impl windows_core::RuntimeName for IXpsOMRadialGradientBrush {}
impl IXpsOMRadialGradientBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>() -> IXpsOMRadialGradientBrush_Vtbl {
        unsafe extern "system" fn GetCenter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: *mut XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRadialGradientBrush_Impl::GetCenter(this) {
                Ok(ok__) => {
                    core::ptr::write(center, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: *const XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRadialGradientBrush_Impl::SetCenter(this, core::mem::transmute_copy(&center)).into()
        }
        unsafe extern "system" fn GetRadiiSizes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRadialGradientBrush_Impl::GetRadiiSizes(this) {
                Ok(ok__) => {
                    core::ptr::write(radiisizes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiiSizes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiisizes: *const XPS_SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRadialGradientBrush_Impl::SetRadiiSizes(this, core::mem::transmute_copy(&radiisizes)).into()
        }
        unsafe extern "system" fn GetGradientOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *mut XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRadialGradientBrush_Impl::GetGradientOrigin(this) {
                Ok(ok__) => {
                    core::ptr::write(origin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *const XPS_POINT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRadialGradientBrush_Impl::SetGradientOrigin(this, core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRadialGradientBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(radialgradientbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCenter: GetCenter::<Identity, Impl, OFFSET>,
            SetCenter: SetCenter::<Identity, Impl, OFFSET>,
            GetRadiiSizes: GetRadiiSizes::<Identity, Impl, OFFSET>,
            SetRadiiSizes: SetRadiiSizes::<Identity, Impl, OFFSET>,
            GetGradientOrigin: GetGradientOrigin::<Identity, Impl, OFFSET>,
            SetGradientOrigin: SetGradientOrigin::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRadialGradientBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMGradientBrush as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn SetDictionary(&self, dictionary: Option<&IXpsOMDictionary>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMRemoteDictionaryResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResource_Vtbl {
        unsafe extern "system" fn GetDictionary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRemoteDictionaryResource_Impl::GetDictionary(this) {
                Ok(ok__) => {
                    core::ptr::write(dictionary, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRemoteDictionaryResource_Impl::SetDictionary(this, windows_core::from_raw_borrowed(&dictionary)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            SetDictionary: SetDictionary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource1_Impl: Sized + IXpsOMRemoteDictionaryResource_Impl {
    fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&self, stream: Option<&super::super::System::Com::ISequentialStream>, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMRemoteDictionaryResource1 {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResource1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRemoteDictionaryResource1_Impl::GetDocumentType(this) {
                Ok(ok__) => {
                    core::ptr::write(documenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRemoteDictionaryResource1_Impl::Write1(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMRemoteDictionaryResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            Write1: Write1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource1 as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID || iid == &<IXpsOMRemoteDictionaryResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResourceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn InsertAt(&self, index: u32, object: Option<&IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: Option<&IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn Append(&self, object: Option<&IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMRemoteDictionaryResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRemoteDictionaryResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRemoteDictionaryResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(object, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRemoteDictionaryResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRemoteDictionaryResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRemoteDictionaryResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMRemoteDictionaryResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMRemoteDictionaryResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    core::ptr::write(remotedictionaryresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResourceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMResource_Impl: Sized + IXpsOMPart_Impl {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMResource_Impl, const OFFSET: isize>() -> IXpsOMResource_Vtbl {
        Self { base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
pub trait IXpsOMShareable_Impl: Sized {
    fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetType(&self) -> windows_core::Result<XPS_OBJECT_TYPE>;
}
impl windows_core::RuntimeName for IXpsOMShareable {}
impl IXpsOMShareable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: isize>() -> IXpsOMShareable_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMShareable_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(owner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMShareable_Impl::GetType(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: Option<&super::super::System::Com::IStream>, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMSignatureBlockResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>() -> IXpsOMSignatureBlockResource_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMSignatureBlockResource_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(owner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMSignatureBlockResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(stream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMSignatureBlockResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResourceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMSignatureBlockResource>;
    fn InsertAt(&self, index: u32, signatureblockresource: Option<&IXpsOMSignatureBlockResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, signatureblockresource: Option<&IXpsOMSignatureBlockResource>) -> windows_core::Result<()>;
    fn Append(&self, signatureblockresource: Option<&IXpsOMSignatureBlockResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMSignatureBlockResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMSignatureBlockResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMSignatureBlockResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMSignatureBlockResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMSignatureBlockResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(signatureblockresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMSignatureBlockResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMSignatureBlockResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMSignatureBlockResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMSignatureBlockResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMSignatureBlockResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    core::ptr::write(signatureblockresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResourceCollection as windows_core::Interface>::IID
    }
}
pub trait IXpsOMSolidColorBrush_Impl: Sized + IXpsOMBrush_Impl {
    fn GetColor(&self, color: *mut XPS_COLOR) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColor(&self, color: *const XPS_COLOR, colorprofile: Option<&IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMSolidColorBrush>;
}
impl windows_core::RuntimeName for IXpsOMSolidColorBrush {}
impl IXpsOMSolidColorBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>() -> IXpsOMSolidColorBrush_Vtbl {
        unsafe extern "system" fn GetColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMSolidColorBrush_Impl::GetColor(this, core::mem::transmute_copy(&color)) {
                Ok(ok__) => {
                    core::ptr::write(colorprofile, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMSolidColorBrush_Impl::SetColor(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMSolidColorBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(solidcolorbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetColor: GetColor::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMSolidColorBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMStoryFragmentsResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPageReference>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: Option<&super::super::System::Com::IStream>, partname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMStoryFragmentsResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMStoryFragmentsResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>() -> IXpsOMStoryFragmentsResource_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMStoryFragmentsResource_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(owner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMStoryFragmentsResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(stream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMStoryFragmentsResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMStoryFragmentsResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMThumbnailGenerator_Impl: Sized {
    fn GenerateThumbnail(&self, page: Option<&IXpsOMPage>, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMThumbnailGenerator {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMThumbnailGenerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMThumbnailGenerator_Impl, const OFFSET: isize>() -> IXpsOMThumbnailGenerator_Vtbl {
        unsafe extern "system" fn GenerateThumbnail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMThumbnailGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMThumbnailGenerator_Impl::GenerateThumbnail(this, windows_core::from_raw_borrowed(&page), core::mem::transmute_copy(&thumbnailtype), core::mem::transmute_copy(&thumbnailsize), windows_core::from_raw_borrowed(&imageresourcepartname)) {
                Ok(ok__) => {
                    core::ptr::write(imageresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GenerateThumbnail: GenerateThumbnail::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMThumbnailGenerator as windows_core::Interface>::IID
    }
}
pub trait IXpsOMTileBrush_Impl: Sized + IXpsOMBrush_Impl {
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: Option<&IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetViewbox(&self) -> windows_core::Result<XPS_RECT>;
    fn SetViewbox(&self, viewbox: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetViewport(&self) -> windows_core::Result<XPS_RECT>;
    fn SetViewport(&self, viewport: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetTileMode(&self) -> windows_core::Result<XPS_TILE_MODE>;
    fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMTileBrush {}
impl IXpsOMTileBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>() -> IXpsOMTileBrush_Vtbl {
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMTileBrush_Impl::GetTransform(this) {
                Ok(ok__) => {
                    core::ptr::write(transform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMTileBrush_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(transform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMTileBrush_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMTileBrush_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMTileBrush_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetViewbox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *mut XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMTileBrush_Impl::GetViewbox(this) {
                Ok(ok__) => {
                    core::ptr::write(viewbox, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewbox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *const XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMTileBrush_Impl::SetViewbox(this, core::mem::transmute_copy(&viewbox)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMTileBrush_Impl::GetViewport(this) {
                Ok(ok__) => {
                    core::ptr::write(viewport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *const XPS_RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMTileBrush_Impl::SetViewport(this, core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn GetTileMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMTileBrush_Impl::GetTileMode(this) {
                Ok(ok__) => {
                    core::ptr::write(tilemode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tilemode: XPS_TILE_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMTileBrush_Impl::SetTileMode(this, core::mem::transmute_copy(&tilemode)).into()
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetViewbox: GetViewbox::<Identity, Impl, OFFSET>,
            SetViewbox: SetViewbox::<Identity, Impl, OFFSET>,
            GetViewport: GetViewport::<Identity, Impl, OFFSET>,
            SetViewport: SetViewport::<Identity, Impl, OFFSET>,
            GetTileMode: GetTileMode::<Identity, Impl, OFFSET>,
            SetTileMode: SetTileMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMTileBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMVisual_Impl: Sized + IXpsOMShareable_Impl {
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, matrixtransform: Option<&IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetClipGeometry(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn GetClipGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn SetClipGeometryLocal(&self, clipgeometry: Option<&IXpsOMGeometry>) -> windows_core::Result<()>;
    fn GetClipGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetClipGeometryLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOpacity(&self) -> windows_core::Result<f32>;
    fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()>;
    fn GetOpacityMaskBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetOpacityMaskBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetOpacityMaskBrushLocal(&self, opacitymaskbrush: Option<&IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetOpacityMaskBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetOpacityMaskBrushLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIsHyperlinkTarget(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(&self, ishyperlink: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetHyperlinkNavigateUri(&self) -> windows_core::Result<super::super::System::Com::IUri>;
    fn SetHyperlinkNavigateUri(&self, hyperlinkuri: Option<&super::super::System::Com::IUri>) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLanguage(&self, language: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMVisual {}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMVisual_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>() -> IXpsOMVisual_Vtbl {
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetTransform(this) {
                Ok(ok__) => {
                    core::ptr::write(matrixtransform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(matrixtransform, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&matrixtransform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetClipGeometry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetClipGeometry(this) {
                Ok(ok__) => {
                    core::ptr::write(clipgeometry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetClipGeometryLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(clipgeometry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetClipGeometryLocal(this, windows_core::from_raw_borrowed(&clipgeometry)).into()
        }
        unsafe extern "system" fn GetClipGeometryLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetClipGeometryLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetClipGeometryLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetOpacity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetOpacity(this) {
                Ok(ok__) => {
                    core::ptr::write(opacity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetOpacityMaskBrush(this) {
                Ok(ok__) => {
                    core::ptr::write(opacitymaskbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetOpacityMaskBrushLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(opacitymaskbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetOpacityMaskBrushLocal(this, windows_core::from_raw_borrowed(&opacitymaskbrush)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetOpacityMaskBrushLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetOpacityMaskBrushLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetIsHyperlinkTarget(this) {
                Ok(ok__) => {
                    core::ptr::write(ishyperlink, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetIsHyperlinkTarget(this, core::mem::transmute_copy(&ishyperlink)).into()
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hyperlinkuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetHyperlinkNavigateUri(this) {
                Ok(ok__) => {
                    core::ptr::write(hyperlinkuri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hyperlinkuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetHyperlinkNavigateUri(this, windows_core::from_raw_borrowed(&hyperlinkuri)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisual_Impl::GetLanguage(this) {
                Ok(ok__) => {
                    core::ptr::write(language, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisual_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetClipGeometry: GetClipGeometry::<Identity, Impl, OFFSET>,
            GetClipGeometryLocal: GetClipGeometryLocal::<Identity, Impl, OFFSET>,
            SetClipGeometryLocal: SetClipGeometryLocal::<Identity, Impl, OFFSET>,
            GetClipGeometryLookup: GetClipGeometryLookup::<Identity, Impl, OFFSET>,
            SetClipGeometryLookup: SetClipGeometryLookup::<Identity, Impl, OFFSET>,
            GetOpacity: GetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrush: GetOpacityMaskBrush::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrushLocal: GetOpacityMaskBrushLocal::<Identity, Impl, OFFSET>,
            SetOpacityMaskBrushLocal: SetOpacityMaskBrushLocal::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrushLookup: GetOpacityMaskBrushLookup::<Identity, Impl, OFFSET>,
            SetOpacityMaskBrushLookup: SetOpacityMaskBrushLookup::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            GetHyperlinkNavigateUri: GetHyperlinkNavigateUri::<Identity, Impl, OFFSET>,
            SetHyperlinkNavigateUri: SetHyperlinkNavigateUri::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMVisual as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
pub trait IXpsOMVisualBrush_Impl: Sized + IXpsOMTileBrush_Impl {
    fn GetVisual(&self) -> windows_core::Result<IXpsOMVisual>;
    fn GetVisualLocal(&self) -> windows_core::Result<IXpsOMVisual>;
    fn SetVisualLocal(&self, visual: Option<&IXpsOMVisual>) -> windows_core::Result<()>;
    fn GetVisualLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetVisualLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMVisualBrush>;
}
impl windows_core::RuntimeName for IXpsOMVisualBrush {}
impl IXpsOMVisualBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>() -> IXpsOMVisualBrush_Vtbl {
        unsafe extern "system" fn GetVisual<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisualBrush_Impl::GetVisual(this) {
                Ok(ok__) => {
                    core::ptr::write(visual, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisualLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisualBrush_Impl::GetVisualLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(visual, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisualBrush_Impl::SetVisualLocal(this, windows_core::from_raw_borrowed(&visual)).into()
        }
        unsafe extern "system" fn GetVisualLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisualBrush_Impl::GetVisualLookup(this) {
                Ok(ok__) => {
                    core::ptr::write(lookup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisualBrush_Impl::SetVisualLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visualbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisualBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(visualbrush, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetVisual: GetVisual::<Identity, Impl, OFFSET>,
            GetVisualLocal: GetVisualLocal::<Identity, Impl, OFFSET>,
            SetVisualLocal: SetVisualLocal::<Identity, Impl, OFFSET>,
            GetVisualLookup: GetVisualLookup::<Identity, Impl, OFFSET>,
            SetVisualLookup: SetVisualLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMVisualBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMTileBrush as windows_core::Interface>::IID
    }
}
pub trait IXpsOMVisualCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMVisual>;
    fn InsertAt(&self, index: u32, object: Option<&IXpsOMVisual>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: Option<&IXpsOMVisual>) -> windows_core::Result<()>;
    fn Append(&self, object: Option<&IXpsOMVisual>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsOMVisualCollection {}
impl IXpsOMVisualCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>() -> IXpsOMVisualCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisualCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsOMVisualCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(object, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisualCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisualCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisualCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsOMVisualCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMVisualCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignature_Impl: Sized {
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetCertificateEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateEnumerator>;
    fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<XPS_SIGNATURE_STATUS>;
    fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY>;
    fn GetCustomObjectEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>;
    fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>;
    fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn SetSignatureXml(&self, signaturexml: *const u8, count: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignature {}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignature_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>() -> IXpsSignature_Vtbl {
        unsafe extern "system" fn GetSignatureId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sigid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetSignatureId(this) {
                Ok(ok__) => {
                    core::ptr::write(sigid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignature_Impl::GetSignatureValue(this, core::mem::transmute_copy(&signaturehashvalue), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetCertificateEnumerator(this) {
                Ok(ok__) => {
                    core::ptr::write(certificateenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sigdatetimestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetSigningTime(this) {
                Ok(ok__) => {
                    core::ptr::write(sigdatetimestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetSigningTimeFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(timeformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetSignaturePartName(this) {
                Ok(ok__) => {
                    core::ptr::write(signaturepartname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::Verify(this, core::mem::transmute_copy(&x509certificate)) {
                Ok(ok__) => {
                    core::ptr::write(sigstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(policy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetCustomObjectEnumerator(this) {
                Ok(ok__) => {
                    core::ptr::write(customobjectenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignature_Impl::GetCustomReferenceEnumerator(this) {
                Ok(ok__) => {
                    core::ptr::write(customreferenceenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignature_Impl::GetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetSignatureXml<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *const u8, count: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignature_Impl::SetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, Impl, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, Impl, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, Impl, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            Verify: Verify::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, Impl, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, Impl, OFFSET>,
            SetSignatureXml: SetSignatureXml::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignature as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureBlock_Impl: Sized {
    fn GetRequests(&self) -> windows_core::Result<IXpsSignatureRequestCollection>;
    fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn GetDocumentIndex(&self) -> windows_core::Result<u32>;
    fn GetDocumentName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateRequest(&self, requestid: &windows_core::PCWSTR) -> windows_core::Result<IXpsSignatureRequest>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignatureBlock {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureBlock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>() -> IXpsSignatureBlock_Vtbl {
        unsafe extern "system" fn GetRequests<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requests: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureBlock_Impl::GetRequests(this) {
                Ok(ok__) => {
                    core::ptr::write(requests, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureBlock_Impl::GetPartName(this) {
                Ok(ok__) => {
                    core::ptr::write(partname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixeddocumentindex: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureBlock_Impl::GetDocumentIndex(this) {
                Ok(ok__) => {
                    core::ptr::write(fixeddocumentindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixeddocumentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureBlock_Impl::GetDocumentName(this) {
                Ok(ok__) => {
                    core::ptr::write(fixeddocumentname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::PCWSTR, signaturerequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureBlock_Impl::CreateRequest(this, core::mem::transmute(&requestid)) {
                Ok(ok__) => {
                    core::ptr::write(signaturerequest, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequests: GetRequests::<Identity, Impl, OFFSET>,
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            GetDocumentIndex: GetDocumentIndex::<Identity, Impl, OFFSET>,
            GetDocumentName: GetDocumentName::<Identity, Impl, OFFSET>,
            CreateRequest: CreateRequest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureBlock as windows_core::Interface>::IID
    }
}
pub trait IXpsSignatureBlockCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureBlock>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsSignatureBlockCollection {}
impl IXpsSignatureBlockCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>() -> IXpsSignatureBlockCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureBlockCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureBlockCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(signatureblock, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureBlockCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureBlockCollection as windows_core::Interface>::IID
    }
}
pub trait IXpsSignatureCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignature>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsSignatureCollection {}
impl IXpsSignatureCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>() -> IXpsSignatureCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(signature, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureManager_Impl: Sized {
    fn LoadPackageFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LoadPackageStream(&self, stream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Sign(&self, signoptions: Option<&IXpsSigningOptions>, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<IXpsSignature>;
    fn GetSignatureOriginPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignatureOriginPartName(&self, signatureoriginpartname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetSignatures(&self) -> windows_core::Result<IXpsSignatureCollection>;
    fn AddSignatureBlock(&self, partname: Option<&super::Packaging::Opc::IOpcPartUri>, fixeddocumentindex: u32) -> windows_core::Result<IXpsSignatureBlock>;
    fn GetSignatureBlocks(&self) -> windows_core::Result<IXpsSignatureBlockCollection>;
    fn CreateSigningOptions(&self) -> windows_core::Result<IXpsSigningOptions>;
    fn SavePackageToFile(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> windows_core::Result<()>;
    fn SavePackageToStream(&self, stream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignatureManager {}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>() -> IXpsSignatureManager_Vtbl {
        unsafe extern "system" fn LoadPackageFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureManager_Impl::LoadPackageFile(this, core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn LoadPackageStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureManager_Impl::LoadPackageStream(this, windows_core::from_raw_borrowed(&stream)).into()
        }
        unsafe extern "system" fn Sign<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signoptions: *mut core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureManager_Impl::Sign(this, windows_core::from_raw_borrowed(&signoptions), core::mem::transmute_copy(&x509certificate)) {
                Ok(ok__) => {
                    core::ptr::write(signature, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureManager_Impl::GetSignatureOriginPartName(this) {
                Ok(ok__) => {
                    core::ptr::write(signatureoriginpartname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureManager_Impl::SetSignatureOriginPartName(this, windows_core::from_raw_borrowed(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatures: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureManager_Impl::GetSignatures(this) {
                Ok(ok__) => {
                    core::ptr::write(signatures, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSignatureBlock<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureManager_Impl::AddSignatureBlock(this, windows_core::from_raw_borrowed(&partname), core::mem::transmute_copy(&fixeddocumentindex)) {
                Ok(ok__) => {
                    core::ptr::write(signatureblock, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureBlocks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblocks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureManager_Impl::GetSignatureBlocks(this) {
                Ok(ok__) => {
                    core::ptr::write(signatureblocks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signingoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureManager_Impl::CreateSigningOptions(this) {
                Ok(ok__) => {
                    core::ptr::write(signingoptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePackageToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureManager_Impl::SavePackageToFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes)).into()
        }
        unsafe extern "system" fn SavePackageToStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureManager_Impl::SavePackageToStream(this, windows_core::from_raw_borrowed(&stream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadPackageFile: LoadPackageFile::<Identity, Impl, OFFSET>,
            LoadPackageStream: LoadPackageStream::<Identity, Impl, OFFSET>,
            Sign: Sign::<Identity, Impl, OFFSET>,
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            GetSignatures: GetSignatures::<Identity, Impl, OFFSET>,
            AddSignatureBlock: AddSignatureBlock::<Identity, Impl, OFFSET>,
            GetSignatureBlocks: GetSignatureBlocks::<Identity, Impl, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, Impl, OFFSET>,
            SavePackageToFile: SavePackageToFile::<Identity, Impl, OFFSET>,
            SavePackageToStream: SavePackageToStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureRequest_Impl: Sized {
    fn GetIntent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIntent(&self, intent: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRequestedSigner(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRequestedSigner(&self, signername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRequestSignByDate(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRequestSignByDate(&self, datestring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSigningLocale(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSigningLocale(&self, place: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> windows_core::Result<()>;
    fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> windows_core::Result<()>;
    fn GetRequestId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignature(&self) -> windows_core::Result<IXpsSignature>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignatureRequest {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>() -> IXpsSignatureRequest_Vtbl {
        unsafe extern "system" fn GetIntent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, intent: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequest_Impl::GetIntent(this) {
                Ok(ok__) => {
                    core::ptr::write(intent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, intent: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureRequest_Impl::SetIntent(this, core::mem::transmute(&intent)).into()
        }
        unsafe extern "system" fn GetRequestedSigner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signername: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequest_Impl::GetRequestedSigner(this) {
                Ok(ok__) => {
                    core::ptr::write(signername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSigner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signername: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureRequest_Impl::SetRequestedSigner(this, core::mem::transmute(&signername)).into()
        }
        unsafe extern "system" fn GetRequestSignByDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequest_Impl::GetRequestSignByDate(this) {
                Ok(ok__) => {
                    core::ptr::write(datestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestSignByDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datestring: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureRequest_Impl::SetRequestSignByDate(this, core::mem::transmute(&datestring)).into()
        }
        unsafe extern "system" fn GetSigningLocale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, place: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequest_Impl::GetSigningLocale(this) {
                Ok(ok__) => {
                    core::ptr::write(place, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningLocale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, place: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureRequest_Impl::SetSigningLocale(this, core::mem::transmute(&place)).into()
        }
        unsafe extern "system" fn GetSpotLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut *mut core::ffi::c_void, x: *mut f32, y: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureRequest_Impl::GetSpotLocation(this, core::mem::transmute_copy(&pageindex), core::mem::transmute_copy(&pagepartname), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetSpotLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureRequest_Impl::SetSpotLocation(this, core::mem::transmute_copy(&pageindex), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn GetRequestId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequest_Impl::GetRequestId(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequest_Impl::GetSignature(this) {
                Ok(ok__) => {
                    core::ptr::write(signature, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIntent: GetIntent::<Identity, Impl, OFFSET>,
            SetIntent: SetIntent::<Identity, Impl, OFFSET>,
            GetRequestedSigner: GetRequestedSigner::<Identity, Impl, OFFSET>,
            SetRequestedSigner: SetRequestedSigner::<Identity, Impl, OFFSET>,
            GetRequestSignByDate: GetRequestSignByDate::<Identity, Impl, OFFSET>,
            SetRequestSignByDate: SetRequestSignByDate::<Identity, Impl, OFFSET>,
            GetSigningLocale: GetSigningLocale::<Identity, Impl, OFFSET>,
            SetSigningLocale: SetSigningLocale::<Identity, Impl, OFFSET>,
            GetSpotLocation: GetSpotLocation::<Identity, Impl, OFFSET>,
            SetSpotLocation: SetSpotLocation::<Identity, Impl, OFFSET>,
            GetRequestId: GetRequestId::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureRequest as windows_core::Interface>::IID
    }
}
pub trait IXpsSignatureRequestCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureRequest>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsSignatureRequestCollection {}
impl IXpsSignatureRequestCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>() -> IXpsSignatureRequestCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequestCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signaturerequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSignatureRequestCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(signaturerequest, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSignatureRequestCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureRequestCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSigningOptions_Impl: Sized {
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureId(&self, signatureid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureMethod(&self, signaturemethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDigestMethod(&self, digestmethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignaturePartName(&self, signaturepartname: Option<&super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY>;
    fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> windows_core::Result<()>;
    fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()>;
    fn GetCustomObjects(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet>;
    fn GetCustomReferences(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateSet>;
    fn GetFlags(&self) -> windows_core::Result<XPS_SIGN_FLAGS>;
    fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSigningOptions {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSigningOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>() -> IXpsSigningOptions_Vtbl {
        unsafe extern "system" fn GetSignatureId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetSignatureId(this) {
                Ok(ok__) => {
                    core::ptr::write(signatureid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSigningOptions_Impl::SetSignatureId(this, core::mem::transmute(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetSignatureMethod(this) {
                Ok(ok__) => {
                    core::ptr::write(signaturemethod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSigningOptions_Impl::SetSignatureMethod(this, core::mem::transmute(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDigestMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetDigestMethod(this) {
                Ok(ok__) => {
                    core::ptr::write(digestmethod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigestMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSigningOptions_Impl::SetDigestMethod(this, core::mem::transmute(&digestmethod)).into()
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetSignaturePartName(this) {
                Ok(ok__) => {
                    core::ptr::write(signaturepartname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSigningOptions_Impl::SetSignaturePartName(this, windows_core::from_raw_borrowed(&signaturepartname)).into()
        }
        unsafe extern "system" fn GetPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(policy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: XPS_SIGN_POLICY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSigningOptions_Impl::SetPolicy(this, core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetSigningTimeFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(timeformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningTimeFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSigningOptions_Impl::SetSigningTimeFormat(this, core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetCustomObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetCustomObjects(this) {
                Ok(ok__) => {
                    core::ptr::write(customobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferences<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetCustomReferences(this) {
                Ok(ok__) => {
                    core::ptr::write(customreferenceset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetCertificateSet(this) {
                Ok(ok__) => {
                    core::ptr::write(certificateset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXpsSigningOptions_Impl::GetFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(flags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXpsSigningOptions_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, Impl, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            SetDigestMethod: SetDigestMethod::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            SetPolicy: SetPolicy::<Identity, Impl, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, Impl, OFFSET>,
            SetSigningTimeFormat: SetSigningTimeFormat::<Identity, Impl, OFFSET>,
            GetCustomObjects: GetCustomObjects::<Identity, Impl, OFFSET>,
            GetCustomReferences: GetCustomReferences::<Identity, Impl, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSigningOptions as windows_core::Interface>::IID
    }
}
