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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsDocumentPackageTarget_Vtbl
    where
        Identity: IXpsDocumentPackageTarget_Impl,
    {
        unsafe extern "system" fn GetXpsOMPackageWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentPackageTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsDocumentPackageTarget_Impl::GetXpsOMPackageWriter(this, windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentPackageTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsDocumentPackageTarget_Impl::GetXpsOMFactory(this) {
                Ok(ok__) => {
                    xpsfactory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentPackageTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsDocumentPackageTarget_Impl::GetXpsType(this) {
                Ok(ok__) => {
                    documenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter: GetXpsOMPackageWriter::<Identity, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, OFFSET>,
            GetXpsType: GetXpsType::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsDocumentPackageTarget3D_Vtbl
    where
        Identity: IXpsDocumentPackageTarget3D_Impl,
    {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, modelpartname: *mut core::ffi::c_void, modeldata: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentPackageTarget3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsDocumentPackageTarget3D_Impl::GetXpsOMPackageWriter3D(this, windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&discardcontrolpartname), windows_core::from_raw_borrowed(&modelpartname), windows_core::from_raw_borrowed(&modeldata)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentPackageTarget3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsDocumentPackageTarget3D_Impl::GetXpsOMFactory(this) {
                Ok(ok__) => {
                    xpsfactory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter3D: GetXpsOMPackageWriter3D::<Identity, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMBrush_Vtbl
    where
        Identity: IXpsOMBrush_Impl,
    {
        unsafe extern "system" fn GetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMBrush_Impl::GetOpacity(this) {
                Ok(ok__) => {
                    opacity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMBrush_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity)).into()
        }
        Self { base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(), GetOpacity: GetOpacity::<Identity, OFFSET>, SetOpacity: SetOpacity::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMCanvas_Vtbl
    where
        Identity: IXpsOMCanvas_Impl,
    {
        unsafe extern "system" fn GetVisuals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visuals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::GetVisuals(this) {
                Ok(ok__) => {
                    visuals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::GetUseAliasedEdgeMode(this) {
                Ok(ok__) => {
                    usealiasededgemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCanvas_Impl::SetUseAliasedEdgeMode(this, core::mem::transmute_copy(&usealiasededgemode)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::GetAccessibilityShortDescription(this) {
                Ok(ok__) => {
                    shortdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCanvas_Impl::SetAccessibilityShortDescription(this, core::mem::transmute(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::GetAccessibilityLongDescription(this) {
                Ok(ok__) => {
                    longdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCanvas_Impl::SetAccessibilityLongDescription(this, core::mem::transmute(&longdescription)).into()
        }
        unsafe extern "system" fn GetDictionary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::GetDictionary(this) {
                Ok(ok__) => {
                    resourcedictionary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::GetDictionaryLocal(this) {
                Ok(ok__) => {
                    resourcedictionary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCanvas_Impl::SetDictionaryLocal(this, windows_core::from_raw_borrowed(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::GetDictionaryResource(this) {
                Ok(ok__) => {
                    remotedictionaryresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCanvas_Impl::SetDictionaryResource(this, windows_core::from_raw_borrowed(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, canvas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCanvas_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCanvas_Impl::Clone(this) {
                Ok(ok__) => {
                    canvas.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, OFFSET>(),
            GetVisuals: GetVisuals::<Identity, OFFSET>,
            GetUseAliasedEdgeMode: GetUseAliasedEdgeMode::<Identity, OFFSET>,
            SetUseAliasedEdgeMode: SetUseAliasedEdgeMode::<Identity, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, OFFSET>,
            GetDictionary: GetDictionary::<Identity, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMColorProfileResource_Vtbl
    where
        Identity: IXpsOMColorProfileResource_Impl,
    {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMColorProfileResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMColorProfileResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self { base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(), GetStream: GetStream::<Identity, OFFSET>, SetContent: SetContent::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMColorProfileResourceCollection_Vtbl
    where
        Identity: IXpsOMColorProfileResourceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMColorProfileResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMColorProfileResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    object.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMColorProfileResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMColorProfileResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMColorProfileResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMColorProfileResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMColorProfileResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMColorProfileResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    part.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMCoreProperties_Vtbl
    where
        Identity: IXpsOMCoreProperties_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetOwner(this) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetCategory(this) {
                Ok(ok__) => {
                    category.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetCategory(this, core::mem::transmute(&category)).into()
        }
        unsafe extern "system" fn GetContentStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentstatus: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetContentStatus(this) {
                Ok(ok__) => {
                    contentstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentstatus: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetContentStatus(this, core::mem::transmute(&contentstatus)).into()
        }
        unsafe extern "system" fn GetContentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetContentType(this) {
                Ok(ok__) => {
                    contenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetContentType(this, core::mem::transmute(&contenttype)).into()
        }
        unsafe extern "system" fn GetCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetCreated(this) {
                Ok(ok__) => {
                    created.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetCreated(this, core::mem::transmute_copy(&created)).into()
        }
        unsafe extern "system" fn GetCreator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, creator: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetCreator(this) {
                Ok(ok__) => {
                    creator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, creator: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetCreator(this, core::mem::transmute(&creator)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetDescription(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetDescription(this, core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn GetIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetIdentifier(this) {
                Ok(ok__) => {
                    identifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetIdentifier(this, core::mem::transmute(&identifier)).into()
        }
        unsafe extern "system" fn GetKeywords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetKeywords(this) {
                Ok(ok__) => {
                    keywords.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetKeywords(this, core::mem::transmute(&keywords)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetLanguage(this) {
                Ok(ok__) => {
                    language.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
        }
        unsafe extern "system" fn GetLastModifiedBy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodifiedby: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetLastModifiedBy(this) {
                Ok(ok__) => {
                    lastmodifiedby.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedBy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodifiedby: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetLastModifiedBy(this, core::mem::transmute(&lastmodifiedby)).into()
        }
        unsafe extern "system" fn GetLastPrinted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetLastPrinted(this) {
                Ok(ok__) => {
                    lastprinted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastPrinted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetLastPrinted(this, core::mem::transmute_copy(&lastprinted)).into()
        }
        unsafe extern "system" fn GetModified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetModified(this) {
                Ok(ok__) => {
                    modified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetModified(this, core::mem::transmute_copy(&modified)).into()
        }
        unsafe extern "system" fn GetRevision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, revision: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetRevision(this) {
                Ok(ok__) => {
                    revision.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, revision: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetRevision(this, core::mem::transmute(&revision)).into()
        }
        unsafe extern "system" fn GetSubject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetSubject(this) {
                Ok(ok__) => {
                    subject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetSubject(this, core::mem::transmute(&subject)).into()
        }
        unsafe extern "system" fn GetTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetTitle(this) {
                Ok(ok__) => {
                    title.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetTitle(this, core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::GetVersion(this) {
                Ok(ok__) => {
                    version.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMCoreProperties_Impl::SetVersion(this, core::mem::transmute(&version)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMCoreProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMCoreProperties_Impl::Clone(this) {
                Ok(ok__) => {
                    coreproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
            SetCategory: SetCategory::<Identity, OFFSET>,
            GetContentStatus: GetContentStatus::<Identity, OFFSET>,
            SetContentStatus: SetContentStatus::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            SetContentType: SetContentType::<Identity, OFFSET>,
            GetCreated: GetCreated::<Identity, OFFSET>,
            SetCreated: SetCreated::<Identity, OFFSET>,
            GetCreator: GetCreator::<Identity, OFFSET>,
            SetCreator: SetCreator::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, OFFSET>,
            SetIdentifier: SetIdentifier::<Identity, OFFSET>,
            GetKeywords: GetKeywords::<Identity, OFFSET>,
            SetKeywords: SetKeywords::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            GetLastModifiedBy: GetLastModifiedBy::<Identity, OFFSET>,
            SetLastModifiedBy: SetLastModifiedBy::<Identity, OFFSET>,
            GetLastPrinted: GetLastPrinted::<Identity, OFFSET>,
            SetLastPrinted: SetLastPrinted::<Identity, OFFSET>,
            GetModified: GetModified::<Identity, OFFSET>,
            SetModified: SetModified::<Identity, OFFSET>,
            GetRevision: GetRevision::<Identity, OFFSET>,
            SetRevision: SetRevision::<Identity, OFFSET>,
            GetSubject: GetSubject::<Identity, OFFSET>,
            SetSubject: SetSubject::<Identity, OFFSET>,
            GetTitle: GetTitle::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMDashCollection_Vtbl
    where
        Identity: IXpsOMDashCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMDashCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDashCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> windows_core::HRESULT
        where
            Identity: IXpsOMDashCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDashCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    dash.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> windows_core::HRESULT
        where
            Identity: IXpsOMDashCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDashCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMDashCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDashCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> windows_core::HRESULT
        where
            Identity: IXpsOMDashCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDashCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dash: *const XPS_DASH) -> windows_core::HRESULT
        where
            Identity: IXpsOMDashCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDashCollection_Impl::Append(this, core::mem::transmute_copy(&dash)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMDictionary_Vtbl
    where
        Identity: IXpsOMDictionary_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDictionary_Impl::GetOwner(this) {
                Ok(ok__) => {
                    owner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDictionary_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: *mut windows_core::PWSTR, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDictionary_Impl::GetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    entry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR, beforeentry: *mut core::ffi::c_void, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDictionary_Impl::GetByKey(this, core::mem::transmute(&key), windows_core::from_raw_borrowed(&beforeentry)) {
                Ok(ok__) => {
                    entry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, entry: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDictionary_Impl::GetIndex(this, windows_core::from_raw_borrowed(&entry)) {
                Ok(ok__) => {
                    index.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDictionary_Impl::Append(this, core::mem::transmute(&key), windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDictionary_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute(&key), windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDictionary_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDictionary_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute(&key), windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDictionary_Impl::Clone(this) {
                Ok(ok__) => {
                    dictionary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetByKey: GetByKey::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMDocument_Vtbl
    where
        Identity: IXpsOMDocument_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocument_Impl::GetOwner(this) {
                Ok(ok__) => {
                    documentsequence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereferences: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocument_Impl::GetPageReferences(this) {
                Ok(ok__) => {
                    pagereferences.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocument_Impl::GetPrintTicketResource(this) {
                Ok(ok__) => {
                    printticketresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocument_Impl::SetPrintTicketResource(this, windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        unsafe extern "system" fn GetDocumentStructureResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentstructureresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocument_Impl::GetDocumentStructureResource(this) {
                Ok(ok__) => {
                    documentstructureresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentStructureResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentstructureresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocument_Impl::SetDocumentStructureResource(this, windows_core::from_raw_borrowed(&documentstructureresource)).into()
        }
        unsafe extern "system" fn GetSignatureBlockResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblockresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocument_Impl::GetSignatureBlockResources(this) {
                Ok(ok__) => {
                    signatureblockresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocument_Impl::Clone(this) {
                Ok(ok__) => {
                    document.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetPageReferences: GetPageReferences::<Identity, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, OFFSET>,
            GetDocumentStructureResource: GetDocumentStructureResource::<Identity, OFFSET>,
            SetDocumentStructureResource: SetDocumentStructureResource::<Identity, OFFSET>,
            GetSignatureBlockResources: GetSignatureBlockResources::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMDocumentCollection_Vtbl
    where
        Identity: IXpsOMDocumentCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocumentCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocumentCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    document.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocumentCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&document)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocumentCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocumentCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&document)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocumentCollection_Impl::Append(this, windows_core::from_raw_borrowed(&document)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMDocumentSequence_Vtbl
    where
        Identity: IXpsOMDocumentSequence_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentSequence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocumentSequence_Impl::GetOwner(this) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocuments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentSequence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocumentSequence_Impl::GetDocuments(this) {
                Ok(ok__) => {
                    documents.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentSequence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocumentSequence_Impl::GetPrintTicketResource(this) {
                Ok(ok__) => {
                    printticketresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentSequence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocumentSequence_Impl::SetPrintTicketResource(this, windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetDocuments: GetDocuments::<Identity, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMDocumentStructureResource_Vtbl
    where
        Identity: IXpsOMDocumentStructureResource_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentStructureResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocumentStructureResource_Impl::GetOwner(this) {
                Ok(ok__) => {
                    owner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentStructureResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMDocumentStructureResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMDocumentStructureResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMDocumentStructureResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMFontResource_Vtbl
    where
        Identity: IXpsOMFontResource_Impl,
    {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, readerstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMFontResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    readerstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMFontResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), core::mem::transmute_copy(&embeddingoption), windows_core::from_raw_borrowed(&partname)).into()
        }
        unsafe extern "system" fn GetEmbeddingOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMFontResource_Impl::GetEmbeddingOption(this) {
                Ok(ok__) => {
                    embeddingoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            GetEmbeddingOption: GetEmbeddingOption::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMFontResourceCollection_Vtbl
    where
        Identity: IXpsOMFontResourceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMFontResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMFontResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMFontResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMFontResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMFontResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMFontResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMFontResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMFontResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    part.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGeometry_Vtbl
    where
        Identity: IXpsOMGeometry_Impl,
    {
        unsafe extern "system" fn GetFigures<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, figures: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometry_Impl::GetFigures(this) {
                Ok(ok__) => {
                    figures.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometry_Impl::GetFillRule(this) {
                Ok(ok__) => {
                    fillrule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillrule: XPS_FILL_RULE) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometry_Impl::SetFillRule(this, core::mem::transmute_copy(&fillrule)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometry_Impl::GetTransform(this) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometry_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometry_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometry_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    lookup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometry_Impl::SetTransformLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometry_Impl::Clone(this) {
                Ok(ok__) => {
                    geometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(),
            GetFigures: GetFigures::<Identity, OFFSET>,
            GetFillRule: GetFillRule::<Identity, OFFSET>,
            SetFillRule: SetFillRule::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGeometryFigure_Vtbl
    where
        Identity: IXpsOMGeometryFigure_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::GetOwner(this) {
                Ok(ok__) => {
                    owner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigure_Impl::GetSegmentData(this, core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&segmentdata)).into()
        }
        unsafe extern "system" fn GetSegmentTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigure_Impl::GetSegmentTypes(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmenttypes)).into()
        }
        unsafe extern "system" fn GetSegmentStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigure_Impl::GetSegmentStrokes(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn SetSegments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigure_Impl::SetSegments(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmentdatacount), core::mem::transmute_copy(&segmenttypes), core::mem::transmute_copy(&segmentdata), core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn GetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *mut XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::GetStartPoint(this) {
                Ok(ok__) => {
                    startpoint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigure_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetIsClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::GetIsClosed(this) {
                Ok(ok__) => {
                    isclosed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigure_Impl::SetIsClosed(this, core::mem::transmute_copy(&isclosed)).into()
        }
        unsafe extern "system" fn GetIsFilled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::GetIsFilled(this) {
                Ok(ok__) => {
                    isfilled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigure_Impl::SetIsFilled(this, core::mem::transmute_copy(&isfilled)).into()
        }
        unsafe extern "system" fn GetSegmentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::GetSegmentCount(this) {
                Ok(ok__) => {
                    segmentcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentdatacount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::GetSegmentDataCount(this) {
                Ok(ok__) => {
                    segmentdatacount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::GetSegmentStrokePattern(this) {
                Ok(ok__) => {
                    segmentstrokepattern.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryfigure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigure_Impl::Clone(this) {
                Ok(ok__) => {
                    geometryfigure.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, OFFSET>,
            GetSegmentTypes: GetSegmentTypes::<Identity, OFFSET>,
            GetSegmentStrokes: GetSegmentStrokes::<Identity, OFFSET>,
            SetSegments: SetSegments::<Identity, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            GetIsClosed: GetIsClosed::<Identity, OFFSET>,
            SetIsClosed: SetIsClosed::<Identity, OFFSET>,
            GetIsFilled: GetIsFilled::<Identity, OFFSET>,
            SetIsFilled: SetIsFilled::<Identity, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, OFFSET>,
            GetSegmentStrokePattern: GetSegmentStrokePattern::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGeometryFigureCollection_Vtbl
    where
        Identity: IXpsOMGeometryFigureCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigureCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGeometryFigureCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    geometryfigure.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigureCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigureCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigureCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGeometryFigureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGeometryFigureCollection_Impl::Append(this, windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGlyphs_Vtbl
    where
        Identity: IXpsOMGlyphs_Impl,
    {
        unsafe extern "system" fn GetUnicodeString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetUnicodeString(this) {
                Ok(ok__) => {
                    unicodestring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetGlyphIndexCount(this) {
                Ok(ok__) => {
                    indexcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetGlyphMappingCount(this) {
                Ok(ok__) => {
                    glyphmappingcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::GetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetProhibitedCaretStopCount(this) {
                Ok(ok__) => {
                    prohibitedcaretstopcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::GetProhibitedCaretStops(this, core::mem::transmute_copy(&prohibitedcaretstopcount), core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetBidiLevel(this) {
                Ok(ok__) => {
                    bidilevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSideways<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetIsSideways(this) {
                Ok(ok__) => {
                    issideways.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetDeviceFontName(this) {
                Ok(ok__) => {
                    devicefontname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyleSimulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetStyleSimulations(this) {
                Ok(ok__) => {
                    stylesimulations.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::SetStyleSimulations(this, core::mem::transmute_copy(&stylesimulations)).into()
        }
        unsafe extern "system" fn GetOrigin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *mut XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetOrigin(this) {
                Ok(ok__) => {
                    origin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrigin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *const XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::SetOrigin(this, core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontrenderingemsize: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetFontRenderingEmSize(this) {
                Ok(ok__) => {
                    fontrenderingemsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontrenderingemsize: f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::SetFontRenderingEmSize(this, core::mem::transmute_copy(&fontrenderingemsize)).into()
        }
        unsafe extern "system" fn GetFontResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetFontResource(this) {
                Ok(ok__) => {
                    fontresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::SetFontResource(this, windows_core::from_raw_borrowed(&fontresource)).into()
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfaceindex: *mut i16) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetFontFaceIndex(this) {
                Ok(ok__) => {
                    fontfaceindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFaceIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfaceindex: i16) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::SetFontFaceIndex(this, core::mem::transmute_copy(&fontfaceindex)).into()
        }
        unsafe extern "system" fn GetFillBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetFillBrush(this) {
                Ok(ok__) => {
                    fillbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetFillBrushLocal(this) {
                Ok(ok__) => {
                    fillbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::SetFillBrushLocal(this, windows_core::from_raw_borrowed(&fillbrush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetFillBrushLookup(this) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphs_Impl::SetFillBrushLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetGlyphsEditor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, editor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::GetGlyphsEditor(this) {
                Ok(ok__) => {
                    editor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphs_Impl::Clone(this) {
                Ok(ok__) => {
                    glyphs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, OFFSET>(),
            GetUnicodeString: GetUnicodeString::<Identity, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, OFFSET>,
            GetStyleSimulations: GetStyleSimulations::<Identity, OFFSET>,
            SetStyleSimulations: SetStyleSimulations::<Identity, OFFSET>,
            GetOrigin: GetOrigin::<Identity, OFFSET>,
            SetOrigin: SetOrigin::<Identity, OFFSET>,
            GetFontRenderingEmSize: GetFontRenderingEmSize::<Identity, OFFSET>,
            SetFontRenderingEmSize: SetFontRenderingEmSize::<Identity, OFFSET>,
            GetFontResource: GetFontResource::<Identity, OFFSET>,
            SetFontResource: SetFontResource::<Identity, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, OFFSET>,
            SetFontFaceIndex: SetFontFaceIndex::<Identity, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, OFFSET>,
            GetGlyphsEditor: GetGlyphsEditor::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGlyphsEditor_Vtbl
    where
        Identity: IXpsOMGlyphsEditor_Impl,
    {
        unsafe extern "system" fn ApplyEdits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::ApplyEdits(this).into()
        }
        unsafe extern "system" fn GetUnicodeString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphsEditor_Impl::GetUnicodeString(this) {
                Ok(ok__) => {
                    unicodestring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::SetUnicodeString(this, core::mem::transmute(&unicodestring)).into()
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphsEditor_Impl::GetGlyphIndexCount(this) {
                Ok(ok__) => {
                    indexcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn SetGlyphIndices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::SetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphsEditor_Impl::GetGlyphMappingCount(this) {
                Ok(ok__) => {
                    glyphmappingcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::GetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn SetGlyphMappings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::SetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphsEditor_Impl::GetProhibitedCaretStopCount(this) {
                Ok(ok__) => {
                    prohibitedcaretstopcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::GetProhibitedCaretStops(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::SetProhibitedCaretStops(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphsEditor_Impl::GetBidiLevel(this) {
                Ok(ok__) => {
                    bidilevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBidiLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::SetBidiLevel(this, core::mem::transmute_copy(&bidilevel)).into()
        }
        unsafe extern "system" fn GetIsSideways<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphsEditor_Impl::GetIsSideways(this) {
                Ok(ok__) => {
                    issideways.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSideways<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::SetIsSideways(this, core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGlyphsEditor_Impl::GetDeviceFontName(this) {
                Ok(ok__) => {
                    devicefontname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceFontName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGlyphsEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGlyphsEditor_Impl::SetDeviceFontName(this, core::mem::transmute(&devicefontname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplyEdits: ApplyEdits::<Identity, OFFSET>,
            GetUnicodeString: GetUnicodeString::<Identity, OFFSET>,
            SetUnicodeString: SetUnicodeString::<Identity, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, OFFSET>,
            SetGlyphIndices: SetGlyphIndices::<Identity, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, OFFSET>,
            SetGlyphMappings: SetGlyphMappings::<Identity, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, OFFSET>,
            SetProhibitedCaretStops: SetProhibitedCaretStops::<Identity, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, OFFSET>,
            SetIsSideways: SetIsSideways::<Identity, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, OFFSET>,
            SetDeviceFontName: SetDeviceFontName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGradientBrush_Vtbl
    where
        Identity: IXpsOMGradientBrush_Impl,
    {
        unsafe extern "system" fn GetGradientStops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientBrush_Impl::GetGradientStops(this) {
                Ok(ok__) => {
                    gradientstops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientBrush_Impl::GetTransform(this) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientBrush_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientBrush_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientBrush_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientBrush_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetSpreadMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientBrush_Impl::GetSpreadMethod(this) {
                Ok(ok__) => {
                    spreadmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientBrush_Impl::SetSpreadMethod(this, core::mem::transmute_copy(&spreadmethod)).into()
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientBrush_Impl::GetColorInterpolationMode(this) {
                Ok(ok__) => {
                    colorinterpolationmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientBrush_Impl::SetColorInterpolationMode(this, core::mem::transmute_copy(&colorinterpolationmode)).into()
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, OFFSET>(),
            GetGradientStops: GetGradientStops::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            GetSpreadMethod: GetSpreadMethod::<Identity, OFFSET>,
            SetSpreadMethod: SetSpreadMethod::<Identity, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, OFFSET>,
            SetColorInterpolationMode: SetColorInterpolationMode::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGradientStop_Vtbl
    where
        Identity: IXpsOMGradientStop_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientStop_Impl::GetOwner(this) {
                Ok(ok__) => {
                    owner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientStop_Impl::GetOffset(this) {
                Ok(ok__) => {
                    offset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientStop_Impl::SetOffset(this, core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientStop_Impl::GetColor(this, core::mem::transmute_copy(&color)) {
                Ok(ok__) => {
                    colorprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientStop_Impl::SetColor(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientStop_Impl::Clone(this) {
                Ok(ok__) => {
                    gradientstop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetOffset: GetOffset::<Identity, OFFSET>,
            SetOffset: SetOffset::<Identity, OFFSET>,
            GetColor: GetColor::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMGradientStopCollection_Vtbl
    where
        Identity: IXpsOMGradientStopCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientStopCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMGradientStopCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    stop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientStopCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&stop)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientStopCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientStopCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&stop)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMGradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMGradientStopCollection_Impl::Append(this, windows_core::from_raw_borrowed(&stop)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMImageBrush_Vtbl
    where
        Identity: IXpsOMImageBrush_Impl,
    {
        unsafe extern "system" fn GetImageResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageBrush_Impl::GetImageResource(this) {
                Ok(ok__) => {
                    imageresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMImageBrush_Impl::SetImageResource(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn GetColorProfileResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageBrush_Impl::GetColorProfileResource(this) {
                Ok(ok__) => {
                    colorprofileresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorProfileResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMImageBrush_Impl::SetColorProfileResource(this, windows_core::from_raw_borrowed(&colorprofileresource)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    imagebrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, OFFSET>(),
            GetImageResource: GetImageResource::<Identity, OFFSET>,
            SetImageResource: SetImageResource::<Identity, OFFSET>,
            GetColorProfileResource: GetColorProfileResource::<Identity, OFFSET>,
            SetColorProfileResource: SetColorProfileResource::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMImageResource_Vtbl
    where
        Identity: IXpsOMImageResource_Impl,
    {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, readerstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    readerstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMImageResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), core::mem::transmute_copy(&imagetype), windows_core::from_raw_borrowed(&partname)).into()
        }
        unsafe extern "system" fn GetImageType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageResource_Impl::GetImageType(this) {
                Ok(ok__) => {
                    imagetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            GetImageType: GetImageType::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMImageResourceCollection_Vtbl
    where
        Identity: IXpsOMImageResourceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    object.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMImageResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMImageResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMImageResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMImageResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMImageResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMImageResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    part.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMLinearGradientBrush_Vtbl
    where
        Identity: IXpsOMLinearGradientBrush_Impl,
    {
        unsafe extern "system" fn GetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *mut XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMLinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMLinearGradientBrush_Impl::GetStartPoint(this) {
                Ok(ok__) => {
                    startpoint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMLinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMLinearGradientBrush_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetEndPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: *mut XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMLinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMLinearGradientBrush_Impl::GetEndPoint(this) {
                Ok(ok__) => {
                    endpoint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: *const XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMLinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMLinearGradientBrush_Impl::SetEndPoint(this, core::mem::transmute_copy(&endpoint)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMLinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMLinearGradientBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    lineargradientbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, OFFSET>(),
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMMatrixTransform_Vtbl
    where
        Identity: IXpsOMMatrixTransform_Impl,
    {
        unsafe extern "system" fn GetMatrix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut XPS_MATRIX) -> windows_core::HRESULT
        where
            Identity: IXpsOMMatrixTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMMatrixTransform_Impl::GetMatrix(this) {
                Ok(ok__) => {
                    matrix.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const XPS_MATRIX) -> windows_core::HRESULT
        where
            Identity: IXpsOMMatrixTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMMatrixTransform_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMMatrixTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMMatrixTransform_Impl::Clone(this) {
                Ok(ok__) => {
                    matrixtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(),
            GetMatrix: GetMatrix::<Identity, OFFSET>,
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMNameCollection_Vtbl
    where
        Identity: IXpsOMNameCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMNameCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMNameCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMNameCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMNameCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMObjectFactory_Vtbl
    where
        Identity: IXpsOMObjectFactory_Impl,
    {
        unsafe extern "system" fn CreatePackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePackage(this) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePackageFromFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePackageFromStream(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, storyfragmentsresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateStoryFragmentsResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    storyfragmentsresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, documentstructureresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateDocumentStructureResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    documentstructureresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateSignatureBlockResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    signatureblockresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateRemoteDictionaryResource(this, windows_core::from_raw_borrowed(&dictionary), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    remotedictionaryresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarymarkupstream: *mut core::ffi::c_void, dictionaryparturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, dictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateRemoteDictionaryResourceFromStream(this, windows_core::from_raw_borrowed(&dictionarymarkupstream), windows_core::from_raw_borrowed(&dictionaryparturi), windows_core::from_raw_borrowed(&resources)) {
                Ok(ok__) => {
                    dictionaryresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePartResources(this) {
                Ok(ok__) => {
                    partresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentSequence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateDocumentSequence(this, windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    documentsequence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateDocument(this, windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    document.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePageReference(this, core::mem::transmute_copy(&advisorypagedimensions)) {
                Ok(ok__) => {
                    pagereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: windows_core::PCWSTR, parturi: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePage(this, core::mem::transmute_copy(&pagedimensions), core::mem::transmute(&language), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    page.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagemarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePageFromStream(this, windows_core::from_raw_borrowed(&pagemarkupstream), windows_core::from_raw_borrowed(&parturi), windows_core::from_raw_borrowed(&resources), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    page.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCanvas<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, canvas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateCanvas(this) {
                Ok(ok__) => {
                    canvas.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut core::ffi::c_void, glyphs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateGlyphs(this, windows_core::from_raw_borrowed(&fontresource)) {
                Ok(ok__) => {
                    glyphs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePath(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateGeometry(this) {
                Ok(ok__) => {
                    geometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryFigure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateGeometryFigure(this, core::mem::transmute_copy(&startpoint)) {
                Ok(ok__) => {
                    figure.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateMatrixTransform(this, core::mem::transmute_copy(&matrix)) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateSolidColorBrush(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile)) {
                Ok(ok__) => {
                    solidcolorbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorProfileResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, colorprofileresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateColorProfileResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    colorprofileresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateImageBrush(this, windows_core::from_raw_borrowed(&image), core::mem::transmute_copy(&viewbox), core::mem::transmute_copy(&viewport)) {
                Ok(ok__) => {
                    imagebrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisualBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateVisualBrush(this, core::mem::transmute_copy(&viewbox), core::mem::transmute_copy(&viewport)) {
                Ok(ok__) => {
                    visualbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateImageResource(this, windows_core::from_raw_borrowed(&acquiredstream), core::mem::transmute_copy(&contenttype), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    imageresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintTicketResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePrintTicketResource(this, windows_core::from_raw_borrowed(&acquiredstream), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    printticketresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut core::ffi::c_void, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateFontResource(this, windows_core::from_raw_borrowed(&acquiredstream), core::mem::transmute_copy(&fontembedding), windows_core::from_raw_borrowed(&parturi), core::mem::transmute_copy(&isobfsourcestream)) {
                Ok(ok__) => {
                    fontresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void, offset: f32, gradientstop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateGradientStop(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    gradientstop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradstop1: *mut core::ffi::c_void, gradstop2: *mut core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateLinearGradientBrush(this, windows_core::from_raw_borrowed(&gradstop1), windows_core::from_raw_borrowed(&gradstop2), core::mem::transmute_copy(&startpoint), core::mem::transmute_copy(&endpoint)) {
                Ok(ok__) => {
                    lineargradientbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradstop1: *mut core::ffi::c_void, gradstop2: *mut core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateRadialGradientBrush(this, windows_core::from_raw_borrowed(&gradstop1), windows_core::from_raw_borrowed(&gradstop2), core::mem::transmute_copy(&centerpoint), core::mem::transmute_copy(&gradientorigin), core::mem::transmute_copy(&radiisizes)) {
                Ok(ok__) => {
                    radialgradientbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoreProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateCoreProperties(this, windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    coreproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDictionary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateDictionary(this) {
                Ok(ok__) => {
                    dictionary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUriCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturicollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePartUriCollection(this) {
                Ok(ok__) => {
                    parturicollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
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
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePackageWriterOnStream(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&interleaving), windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&coreproperties), windows_core::from_raw_borrowed(&packagethumbnail), windows_core::from_raw_borrowed(&documentsequenceprintticket), windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreatePartUri(this, core::mem::transmute(&uri)) {
                Ok(ok__) => {
                    parturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory_Impl::CreateReadOnlyStreamOnFile(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackage: CreatePackage::<Identity, OFFSET>,
            CreatePackageFromFile: CreatePackageFromFile::<Identity, OFFSET>,
            CreatePackageFromStream: CreatePackageFromStream::<Identity, OFFSET>,
            CreateStoryFragmentsResource: CreateStoryFragmentsResource::<Identity, OFFSET>,
            CreateDocumentStructureResource: CreateDocumentStructureResource::<Identity, OFFSET>,
            CreateSignatureBlockResource: CreateSignatureBlockResource::<Identity, OFFSET>,
            CreateRemoteDictionaryResource: CreateRemoteDictionaryResource::<Identity, OFFSET>,
            CreateRemoteDictionaryResourceFromStream: CreateRemoteDictionaryResourceFromStream::<Identity, OFFSET>,
            CreatePartResources: CreatePartResources::<Identity, OFFSET>,
            CreateDocumentSequence: CreateDocumentSequence::<Identity, OFFSET>,
            CreateDocument: CreateDocument::<Identity, OFFSET>,
            CreatePageReference: CreatePageReference::<Identity, OFFSET>,
            CreatePage: CreatePage::<Identity, OFFSET>,
            CreatePageFromStream: CreatePageFromStream::<Identity, OFFSET>,
            CreateCanvas: CreateCanvas::<Identity, OFFSET>,
            CreateGlyphs: CreateGlyphs::<Identity, OFFSET>,
            CreatePath: CreatePath::<Identity, OFFSET>,
            CreateGeometry: CreateGeometry::<Identity, OFFSET>,
            CreateGeometryFigure: CreateGeometryFigure::<Identity, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, OFFSET>,
            CreateColorProfileResource: CreateColorProfileResource::<Identity, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, OFFSET>,
            CreateVisualBrush: CreateVisualBrush::<Identity, OFFSET>,
            CreateImageResource: CreateImageResource::<Identity, OFFSET>,
            CreatePrintTicketResource: CreatePrintTicketResource::<Identity, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, OFFSET>,
            CreateGradientStop: CreateGradientStop::<Identity, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, OFFSET>,
            CreateCoreProperties: CreateCoreProperties::<Identity, OFFSET>,
            CreateDictionary: CreateDictionary::<Identity, OFFSET>,
            CreatePartUriCollection: CreatePartUriCollection::<Identity, OFFSET>,
            CreatePackageWriterOnFile: CreatePackageWriterOnFile::<Identity, OFFSET>,
            CreatePackageWriterOnStream: CreatePackageWriterOnStream::<Identity, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, OFFSET>,
            CreateReadOnlyStreamOnFile: CreateReadOnlyStreamOnFile::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMObjectFactory1_Vtbl
    where
        Identity: IXpsOMObjectFactory1_Impl,
    {
        unsafe extern "system" fn GetDocumentTypeFromFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::GetDocumentTypeFromFile(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    documenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsdocumentstream: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::GetDocumentTypeFromStream(this, windows_core::from_raw_borrowed(&xpsdocumentstream)) {
                Ok(ok__) => {
                    documenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMObjectFactory1_Impl::ConvertHDPhotoToJpegXR(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMObjectFactory1_Impl::ConvertJpegXRToHDPhoto(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
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
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::CreatePackageWriterOnStream1(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&interleaving), windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&coreproperties), windows_core::from_raw_borrowed(&packagethumbnail), windows_core::from_raw_borrowed(&documentsequenceprintticket), windows_core::from_raw_borrowed(&discardcontrolpartname), core::mem::transmute_copy(&documenttype)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::CreatePackage1(this) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::CreatePackageFromStream1(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::CreatePackageFromFile1(this, core::mem::transmute(&filename), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: windows_core::PCWSTR, parturi: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::CreatePage1(this, core::mem::transmute_copy(&pagedimensions), core::mem::transmute(&language), windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    page.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagemarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::CreatePageFromStream1(this, windows_core::from_raw_borrowed(&pagemarkupstream), windows_core::from_raw_borrowed(&parturi), windows_core::from_raw_borrowed(&resources), core::mem::transmute_copy(&reuseobjects)) {
                Ok(ok__) => {
                    page.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarymarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, dictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMObjectFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMObjectFactory1_Impl::CreateRemoteDictionaryResourceFromStream1(this, windows_core::from_raw_borrowed(&dictionarymarkupstream), windows_core::from_raw_borrowed(&parturi), windows_core::from_raw_borrowed(&resources)) {
                Ok(ok__) => {
                    dictionaryresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMObjectFactory_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentTypeFromFile: GetDocumentTypeFromFile::<Identity, OFFSET>,
            GetDocumentTypeFromStream: GetDocumentTypeFromStream::<Identity, OFFSET>,
            ConvertHDPhotoToJpegXR: ConvertHDPhotoToJpegXR::<Identity, OFFSET>,
            ConvertJpegXRToHDPhoto: ConvertJpegXRToHDPhoto::<Identity, OFFSET>,
            CreatePackageWriterOnFile1: CreatePackageWriterOnFile1::<Identity, OFFSET>,
            CreatePackageWriterOnStream1: CreatePackageWriterOnStream1::<Identity, OFFSET>,
            CreatePackage1: CreatePackage1::<Identity, OFFSET>,
            CreatePackageFromStream1: CreatePackageFromStream1::<Identity, OFFSET>,
            CreatePackageFromFile1: CreatePackageFromFile1::<Identity, OFFSET>,
            CreatePage1: CreatePage1::<Identity, OFFSET>,
            CreatePageFromStream1: CreatePageFromStream1::<Identity, OFFSET>,
            CreateRemoteDictionaryResourceFromStream1: CreateRemoteDictionaryResourceFromStream1::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPackage_Vtbl
    where
        Identity: IXpsOMPackage_Impl,
    {
        unsafe extern "system" fn GetDocumentSequence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPackage_Impl::GetDocumentSequence(this) {
                Ok(ok__) => {
                    documentsequence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentSequence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage_Impl::SetDocumentSequence(this, windows_core::from_raw_borrowed(&documentsequence)).into()
        }
        unsafe extern "system" fn GetCoreProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPackage_Impl::GetCoreProperties(this) {
                Ok(ok__) => {
                    coreproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoreProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage_Impl::SetCoreProperties(this, windows_core::from_raw_borrowed(&coreproperties)).into()
        }
        unsafe extern "system" fn GetDiscardControlPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, discardcontrolparturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPackage_Impl::GetDiscardControlPartName(this) {
                Ok(ok__) => {
                    discardcontrolparturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscardControlPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, discardcontrolparturi: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage_Impl::SetDiscardControlPartName(this, windows_core::from_raw_borrowed(&discardcontrolparturi)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPackage_Impl::GetThumbnailResource(this) {
                Ok(ok__) => {
                    imageresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage_Impl::SetThumbnailResource(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn WriteToFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage_Impl::WriteToFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes), core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn WriteToStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage_Impl::WriteToStream(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentSequence: GetDocumentSequence::<Identity, OFFSET>,
            SetDocumentSequence: SetDocumentSequence::<Identity, OFFSET>,
            GetCoreProperties: GetCoreProperties::<Identity, OFFSET>,
            SetCoreProperties: SetCoreProperties::<Identity, OFFSET>,
            GetDiscardControlPartName: GetDiscardControlPartName::<Identity, OFFSET>,
            SetDiscardControlPartName: SetDiscardControlPartName::<Identity, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, OFFSET>,
            WriteToFile: WriteToFile::<Identity, OFFSET>,
            WriteToStream: WriteToStream::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPackage1_Vtbl
    where
        Identity: IXpsOMPackage1_Impl,
    {
        unsafe extern "system" fn GetDocumentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPackage1_Impl::GetDocumentType(this) {
                Ok(ok__) => {
                    documenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToFile1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage1_Impl::WriteToFile1(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
        }
        unsafe extern "system" fn WriteToStream1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackage1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackage1_Impl::WriteToStream1(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMPackage_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, OFFSET>,
            WriteToFile1: WriteToFile1::<Identity, OFFSET>,
            WriteToStream1: WriteToStream1::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPackageTarget_Vtbl
    where
        Identity: IXpsOMPackageTarget_Impl,
    {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPackageTarget_Impl::CreateXpsOMPackageWriter(this, windows_core::from_raw_borrowed(&documentsequencepartname), windows_core::from_raw_borrowed(&documentsequenceprintticket), windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateXpsOMPackageWriter: CreateXpsOMPackageWriter::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPackageWriter_Vtbl
    where
        Identity: IXpsOMPackageWriter_Impl,
    {
        unsafe extern "system" fn StartNewDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentpartname: *mut core::ffi::c_void, documentprintticket: *mut core::ffi::c_void, documentstructure: *mut core::ffi::c_void, signatureblockresources: *mut core::ffi::c_void, restrictedfonts: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackageWriter_Impl::StartNewDocument(this, windows_core::from_raw_borrowed(&documentpartname), windows_core::from_raw_borrowed(&documentprintticket), windows_core::from_raw_borrowed(&documentstructure), windows_core::from_raw_borrowed(&signatureblockresources), windows_core::from_raw_borrowed(&restrictedfonts)).into()
        }
        unsafe extern "system" fn AddPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut core::ffi::c_void, storyfragments: *mut core::ffi::c_void, pageprintticket: *mut core::ffi::c_void, pagethumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackageWriter_Impl::AddPage(this, windows_core::from_raw_borrowed(&page), core::mem::transmute_copy(&advisorypagedimensions), windows_core::from_raw_borrowed(&discardableresourceparts), windows_core::from_raw_borrowed(&storyfragments), windows_core::from_raw_borrowed(&pageprintticket), windows_core::from_raw_borrowed(&pagethumbnail)).into()
        }
        unsafe extern "system" fn AddResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackageWriter_Impl::AddResource(this, windows_core::from_raw_borrowed(&resource)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackageWriter_Impl::Close(this).into()
        }
        unsafe extern "system" fn IsClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPackageWriter_Impl::IsClosed(this) {
                Ok(ok__) => {
                    isclosed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartNewDocument: StartNewDocument::<Identity, OFFSET>,
            AddPage: AddPage::<Identity, OFFSET>,
            AddResource: AddResource::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            IsClosed: IsClosed::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPackageWriter3D_Vtbl
    where
        Identity: IXpsOMPackageWriter3D_Impl,
    {
        unsafe extern "system" fn AddModelTexture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, texturepartname: *mut core::ffi::c_void, texturedata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageWriter3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackageWriter3D_Impl::AddModelTexture(this, windows_core::from_raw_borrowed(&texturepartname), windows_core::from_raw_borrowed(&texturedata)).into()
        }
        unsafe extern "system" fn SetModelPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketpartname: *mut core::ffi::c_void, printticketdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPackageWriter3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPackageWriter3D_Impl::SetModelPrintTicket(this, windows_core::from_raw_borrowed(&printticketpartname), windows_core::from_raw_borrowed(&printticketdata)).into()
        }
        Self {
            base__: IXpsOMPackageWriter_Vtbl::new::<Identity, OFFSET>(),
            AddModelTexture: AddModelTexture::<Identity, OFFSET>,
            SetModelPrintTicket: SetModelPrintTicket::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPage_Vtbl
    where
        Identity: IXpsOMPage_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetOwner(this) {
                Ok(ok__) => {
                    pagereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisuals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visuals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetVisuals(this) {
                Ok(ok__) => {
                    visuals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageDimensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetPageDimensions(this) {
                Ok(ok__) => {
                    pagedimensions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageDimensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetPageDimensions(this, core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetContentBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentbox: *mut XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetContentBox(this) {
                Ok(ok__) => {
                    contentbox.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentbox: *const XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetContentBox(this, core::mem::transmute_copy(&contentbox)).into()
        }
        unsafe extern "system" fn GetBleedBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bleedbox: *mut XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetBleedBox(this) {
                Ok(ok__) => {
                    bleedbox.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBleedBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bleedbox: *const XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetBleedBox(this, core::mem::transmute_copy(&bleedbox)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetLanguage(this) {
                Ok(ok__) => {
                    language.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetIsHyperlinkTarget(this) {
                Ok(ok__) => {
                    ishyperlinktarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetIsHyperlinkTarget(this, core::mem::transmute_copy(&ishyperlinktarget)).into()
        }
        unsafe extern "system" fn GetDictionary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetDictionary(this) {
                Ok(ok__) => {
                    resourcedictionary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetDictionaryLocal(this) {
                Ok(ok__) => {
                    resourcedictionary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetDictionaryLocal(this, windows_core::from_raw_borrowed(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GetDictionaryResource(this) {
                Ok(ok__) => {
                    remotedictionaryresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::SetDictionaryResource(this, windows_core::from_raw_borrowed(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage_Impl::Write(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::GenerateUnusedLookupKey(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage_Impl::Clone(this) {
                Ok(ok__) => {
                    page.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetVisuals: GetVisuals::<Identity, OFFSET>,
            GetPageDimensions: GetPageDimensions::<Identity, OFFSET>,
            SetPageDimensions: SetPageDimensions::<Identity, OFFSET>,
            GetContentBox: GetContentBox::<Identity, OFFSET>,
            SetContentBox: SetContentBox::<Identity, OFFSET>,
            GetBleedBox: GetBleedBox::<Identity, OFFSET>,
            SetBleedBox: SetBleedBox::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, OFFSET>,
            GetDictionary: GetDictionary::<Identity, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            GenerateUnusedLookupKey: GenerateUnusedLookupKey::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPage1_Vtbl
    where
        Identity: IXpsOMPage1_Impl,
    {
        unsafe extern "system" fn GetDocumentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPage1_Impl::GetDocumentType(this) {
                Ok(ok__) => {
                    documenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPage1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPage1_Impl::Write1(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
        }
        Self { base__: IXpsOMPage_Vtbl::new::<Identity, OFFSET>(), GetDocumentType: GetDocumentType::<Identity, OFFSET>, Write1: Write1::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPageReference_Vtbl
    where
        Identity: IXpsOMPageReference_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::GetOwner(this) {
                Ok(ok__) => {
                    document.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::GetPage(this) {
                Ok(ok__) => {
                    page.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReference_Impl::SetPage(this, windows_core::from_raw_borrowed(&page)).into()
        }
        unsafe extern "system" fn DiscardPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReference_Impl::DiscardPage(this).into()
        }
        unsafe extern "system" fn IsPageLoaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::IsPageLoaded(this) {
                Ok(ok__) => {
                    ispageloaded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::GetAdvisoryPageDimensions(this) {
                Ok(ok__) => {
                    pagedimensions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReference_Impl::SetAdvisoryPageDimensions(this, core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyfragmentsresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::GetStoryFragmentsResource(this) {
                Ok(ok__) => {
                    storyfragmentsresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyfragmentsresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReference_Impl::SetStoryFragmentsResource(this, windows_core::from_raw_borrowed(&storyfragmentsresource)).into()
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::GetPrintTicketResource(this) {
                Ok(ok__) => {
                    printticketresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReference_Impl::SetPrintTicketResource(this, windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::GetThumbnailResource(this) {
                Ok(ok__) => {
                    imageresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReference_Impl::SetThumbnailResource(this, windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn CollectLinkTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linktargets: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::CollectLinkTargets(this) {
                Ok(ok__) => {
                    linktargets.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectPartResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::CollectPartResources(this) {
                Ok(ok__) => {
                    partresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasRestrictedFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::HasRestrictedFonts(this) {
                Ok(ok__) => {
                    restrictedfonts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReference_Impl::Clone(this) {
                Ok(ok__) => {
                    pagereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetPage: GetPage::<Identity, OFFSET>,
            SetPage: SetPage::<Identity, OFFSET>,
            DiscardPage: DiscardPage::<Identity, OFFSET>,
            IsPageLoaded: IsPageLoaded::<Identity, OFFSET>,
            GetAdvisoryPageDimensions: GetAdvisoryPageDimensions::<Identity, OFFSET>,
            SetAdvisoryPageDimensions: SetAdvisoryPageDimensions::<Identity, OFFSET>,
            GetStoryFragmentsResource: GetStoryFragmentsResource::<Identity, OFFSET>,
            SetStoryFragmentsResource: SetStoryFragmentsResource::<Identity, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, OFFSET>,
            CollectLinkTargets: CollectLinkTargets::<Identity, OFFSET>,
            CollectPartResources: CollectPartResources::<Identity, OFFSET>,
            HasRestrictedFonts: HasRestrictedFonts::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPageReferenceCollection_Vtbl
    where
        Identity: IXpsOMPageReferenceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReferenceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReferenceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReferenceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPageReferenceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pagereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReferenceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReferenceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&pagereference)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReferenceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReferenceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReferenceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReferenceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&pagereference)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPageReferenceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPageReferenceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&pagereference)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPart_Vtbl
    where
        Identity: IXpsOMPart_Impl,
    {
        unsafe extern "system" fn GetPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPart_Impl::GetPartName(this) {
                Ok(ok__) => {
                    parturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPart_Impl::SetPartName(this, windows_core::from_raw_borrowed(&parturi)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartName: GetPartName::<Identity, OFFSET>,
            SetPartName: SetPartName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPartResources_Vtbl
    where
        Identity: IXpsOMPartResources_Impl,
    {
        unsafe extern "system" fn GetFontResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPartResources_Impl::GetFontResources(this) {
                Ok(ok__) => {
                    fontresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPartResources_Impl::GetImageResources(this) {
                Ok(ok__) => {
                    imageresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProfileResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPartResources_Impl::GetColorProfileResources(this) {
                Ok(ok__) => {
                    colorprofileresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPartResources_Impl::GetRemoteDictionaryResources(this) {
                Ok(ok__) => {
                    dictionaryresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontResources: GetFontResources::<Identity, OFFSET>,
            GetImageResources: GetImageResources::<Identity, OFFSET>,
            GetColorProfileResources: GetColorProfileResources::<Identity, OFFSET>,
            GetRemoteDictionaryResources: GetRemoteDictionaryResources::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPartUriCollection_Vtbl
    where
        Identity: IXpsOMPartUriCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartUriCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPartUriCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartUriCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPartUriCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    parturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartUriCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPartUriCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&parturi)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartUriCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPartUriCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartUriCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPartUriCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&parturi)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPartUriCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPartUriCollection_Impl::Append(this, windows_core::from_raw_borrowed(&parturi)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPath_Vtbl
    where
        Identity: IXpsOMPath_Impl,
    {
        unsafe extern "system" fn GetGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetGeometry(this) {
                Ok(ok__) => {
                    geometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetGeometryLocal(this) {
                Ok(ok__) => {
                    geometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetGeometryLocal(this, windows_core::from_raw_borrowed(&geometry)).into()
        }
        unsafe extern "system" fn GetGeometryLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetGeometryLookup(this) {
                Ok(ok__) => {
                    lookup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetGeometryLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetAccessibilityShortDescription(this) {
                Ok(ok__) => {
                    shortdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetAccessibilityShortDescription(this, core::mem::transmute(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetAccessibilityLongDescription(this) {
                Ok(ok__) => {
                    longdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetAccessibilityLongDescription(this, core::mem::transmute(&longdescription)).into()
        }
        unsafe extern "system" fn GetSnapsToPixels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetSnapsToPixels(this) {
                Ok(ok__) => {
                    snapstopixels.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapsToPixels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetSnapsToPixels(this, core::mem::transmute_copy(&snapstopixels)).into()
        }
        unsafe extern "system" fn GetStrokeBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeBrush(this) {
                Ok(ok__) => {
                    brush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeBrushLocal(this) {
                Ok(ok__) => {
                    brush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeBrushLocal(this, windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeBrushLookup(this) {
                Ok(ok__) => {
                    lookup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeBrushLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn GetStrokeDashes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeDashes(this) {
                Ok(ok__) => {
                    strokedashes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeDashCap(this) {
                Ok(ok__) => {
                    strokedashcap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeDashCap(this, core::mem::transmute_copy(&strokedashcap)).into()
        }
        unsafe extern "system" fn GetStrokeDashOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashoffset: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeDashOffset(this) {
                Ok(ok__) => {
                    strokedashoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashoffset: f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeDashOffset(this, core::mem::transmute_copy(&strokedashoffset)).into()
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeStartLineCap(this) {
                Ok(ok__) => {
                    strokestartlinecap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeStartLineCap(this, core::mem::transmute_copy(&strokestartlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeEndLineCap(this) {
                Ok(ok__) => {
                    strokeendlinecap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeEndLineCap(this, core::mem::transmute_copy(&strokeendlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeLineJoin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeLineJoin(this) {
                Ok(ok__) => {
                    strokelinejoin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeLineJoin(this, core::mem::transmute_copy(&strokelinejoin)).into()
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokemiterlimit: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeMiterLimit(this) {
                Ok(ok__) => {
                    strokemiterlimit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokemiterlimit: f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeMiterLimit(this, core::mem::transmute_copy(&strokemiterlimit)).into()
        }
        unsafe extern "system" fn GetStrokeThickness<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokethickness: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetStrokeThickness(this) {
                Ok(ok__) => {
                    strokethickness.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokethickness: f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetStrokeThickness(this, core::mem::transmute_copy(&strokethickness)).into()
        }
        unsafe extern "system" fn GetFillBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetFillBrush(this) {
                Ok(ok__) => {
                    brush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetFillBrushLocal(this) {
                Ok(ok__) => {
                    brush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetFillBrushLocal(this, windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::GetFillBrushLookup(this) {
                Ok(ok__) => {
                    lookup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPath_Impl::SetFillBrushLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPath_Impl::Clone(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, OFFSET>(),
            GetGeometry: GetGeometry::<Identity, OFFSET>,
            GetGeometryLocal: GetGeometryLocal::<Identity, OFFSET>,
            SetGeometryLocal: SetGeometryLocal::<Identity, OFFSET>,
            GetGeometryLookup: GetGeometryLookup::<Identity, OFFSET>,
            SetGeometryLookup: SetGeometryLookup::<Identity, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, OFFSET>,
            GetSnapsToPixels: GetSnapsToPixels::<Identity, OFFSET>,
            SetSnapsToPixels: SetSnapsToPixels::<Identity, OFFSET>,
            GetStrokeBrush: GetStrokeBrush::<Identity, OFFSET>,
            GetStrokeBrushLocal: GetStrokeBrushLocal::<Identity, OFFSET>,
            SetStrokeBrushLocal: SetStrokeBrushLocal::<Identity, OFFSET>,
            GetStrokeBrushLookup: GetStrokeBrushLookup::<Identity, OFFSET>,
            SetStrokeBrushLookup: SetStrokeBrushLookup::<Identity, OFFSET>,
            GetStrokeDashes: GetStrokeDashes::<Identity, OFFSET>,
            GetStrokeDashCap: GetStrokeDashCap::<Identity, OFFSET>,
            SetStrokeDashCap: SetStrokeDashCap::<Identity, OFFSET>,
            GetStrokeDashOffset: GetStrokeDashOffset::<Identity, OFFSET>,
            SetStrokeDashOffset: SetStrokeDashOffset::<Identity, OFFSET>,
            GetStrokeStartLineCap: GetStrokeStartLineCap::<Identity, OFFSET>,
            SetStrokeStartLineCap: SetStrokeStartLineCap::<Identity, OFFSET>,
            GetStrokeEndLineCap: GetStrokeEndLineCap::<Identity, OFFSET>,
            SetStrokeEndLineCap: SetStrokeEndLineCap::<Identity, OFFSET>,
            GetStrokeLineJoin: GetStrokeLineJoin::<Identity, OFFSET>,
            SetStrokeLineJoin: SetStrokeLineJoin::<Identity, OFFSET>,
            GetStrokeMiterLimit: GetStrokeMiterLimit::<Identity, OFFSET>,
            SetStrokeMiterLimit: SetStrokeMiterLimit::<Identity, OFFSET>,
            GetStrokeThickness: GetStrokeThickness::<Identity, OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Identity, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMPrintTicketResource_Vtbl
    where
        Identity: IXpsOMPrintTicketResource_Impl,
    {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPrintTicketResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMPrintTicketResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMPrintTicketResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMPrintTicketResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self { base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(), GetStream: GetStream::<Identity, OFFSET>, SetContent: SetContent::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMRadialGradientBrush_Vtbl
    where
        Identity: IXpsOMRadialGradientBrush_Impl,
    {
        unsafe extern "system" fn GetCenter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: *mut XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMRadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRadialGradientBrush_Impl::GetCenter(this) {
                Ok(ok__) => {
                    center.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: *const XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMRadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRadialGradientBrush_Impl::SetCenter(this, core::mem::transmute_copy(&center)).into()
        }
        unsafe extern "system" fn GetRadiiSizes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> windows_core::HRESULT
        where
            Identity: IXpsOMRadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRadialGradientBrush_Impl::GetRadiiSizes(this) {
                Ok(ok__) => {
                    radiisizes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiiSizes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiisizes: *const XPS_SIZE) -> windows_core::HRESULT
        where
            Identity: IXpsOMRadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRadialGradientBrush_Impl::SetRadiiSizes(this, core::mem::transmute_copy(&radiisizes)).into()
        }
        unsafe extern "system" fn GetGradientOrigin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *mut XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMRadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRadialGradientBrush_Impl::GetGradientOrigin(this) {
                Ok(ok__) => {
                    origin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientOrigin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *const XPS_POINT) -> windows_core::HRESULT
        where
            Identity: IXpsOMRadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRadialGradientBrush_Impl::SetGradientOrigin(this, core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRadialGradientBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    radialgradientbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, OFFSET>(),
            GetCenter: GetCenter::<Identity, OFFSET>,
            SetCenter: SetCenter::<Identity, OFFSET>,
            GetRadiiSizes: GetRadiiSizes::<Identity, OFFSET>,
            SetRadiiSizes: SetRadiiSizes::<Identity, OFFSET>,
            GetGradientOrigin: GetGradientOrigin::<Identity, OFFSET>,
            SetGradientOrigin: SetGradientOrigin::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResource_Vtbl
    where
        Identity: IXpsOMRemoteDictionaryResource_Impl,
    {
        unsafe extern "system" fn GetDictionary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRemoteDictionaryResource_Impl::GetDictionary(this) {
                Ok(ok__) => {
                    dictionary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRemoteDictionaryResource_Impl::SetDictionary(this, windows_core::from_raw_borrowed(&dictionary)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetDictionary: GetDictionary::<Identity, OFFSET>,
            SetDictionary: SetDictionary::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResource1_Vtbl
    where
        Identity: IXpsOMRemoteDictionaryResource1_Impl,
    {
        unsafe extern "system" fn GetDocumentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResource1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRemoteDictionaryResource1_Impl::GetDocumentType(this) {
                Ok(ok__) => {
                    documenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResource1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRemoteDictionaryResource1_Impl::Write1(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMRemoteDictionaryResource_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, OFFSET>,
            Write1: Write1::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceCollection_Vtbl
    where
        Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRemoteDictionaryResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRemoteDictionaryResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    object.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRemoteDictionaryResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRemoteDictionaryResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRemoteDictionaryResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMRemoteDictionaryResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMRemoteDictionaryResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMRemoteDictionaryResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    remotedictionaryresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMResource_Vtbl
    where
        Identity: IXpsOMResource_Impl,
    {
        Self { base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>() }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMShareable_Vtbl
    where
        Identity: IXpsOMShareable_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMShareable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMShareable_Impl::GetOwner(this) {
                Ok(ok__) => {
                    owner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> windows_core::HRESULT
        where
            Identity: IXpsOMShareable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMShareable_Impl::GetType(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOwner: GetOwner::<Identity, OFFSET>, GetType: GetType::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMSignatureBlockResource_Vtbl
    where
        Identity: IXpsOMSignatureBlockResource_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMSignatureBlockResource_Impl::GetOwner(this) {
                Ok(ok__) => {
                    owner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMSignatureBlockResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMSignatureBlockResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMSignatureBlockResourceCollection_Vtbl
    where
        Identity: IXpsOMSignatureBlockResourceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMSignatureBlockResourceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMSignatureBlockResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    signatureblockresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMSignatureBlockResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMSignatureBlockResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMSignatureBlockResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMSignatureBlockResourceCollection_Impl::Append(this, windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSignatureBlockResourceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMSignatureBlockResourceCollection_Impl::GetByPartName(this, windows_core::from_raw_borrowed(&partname)) {
                Ok(ok__) => {
                    signatureblockresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMSolidColorBrush_Vtbl
    where
        Identity: IXpsOMSolidColorBrush_Impl,
    {
        unsafe extern "system" fn GetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSolidColorBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMSolidColorBrush_Impl::GetColor(this, core::mem::transmute_copy(&color)) {
                Ok(ok__) => {
                    colorprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSolidColorBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMSolidColorBrush_Impl::SetColor(this, core::mem::transmute_copy(&color), windows_core::from_raw_borrowed(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMSolidColorBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMSolidColorBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    solidcolorbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, OFFSET>(),
            GetColor: GetColor::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMStoryFragmentsResource_Vtbl
    where
        Identity: IXpsOMStoryFragmentsResource_Impl,
    {
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMStoryFragmentsResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMStoryFragmentsResource_Impl::GetOwner(this) {
                Ok(ok__) => {
                    owner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMStoryFragmentsResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMStoryFragmentsResource_Impl::GetStream(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMStoryFragmentsResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMStoryFragmentsResource_Impl::SetContent(this, windows_core::from_raw_borrowed(&sourcestream), windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMThumbnailGenerator_Vtbl
    where
        Identity: IXpsOMThumbnailGenerator_Impl,
    {
        unsafe extern "system" fn GenerateThumbnail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMThumbnailGenerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMThumbnailGenerator_Impl::GenerateThumbnail(this, windows_core::from_raw_borrowed(&page), core::mem::transmute_copy(&thumbnailtype), core::mem::transmute_copy(&thumbnailsize), windows_core::from_raw_borrowed(&imageresourcepartname)) {
                Ok(ok__) => {
                    imageresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GenerateThumbnail: GenerateThumbnail::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMTileBrush_Vtbl
    where
        Identity: IXpsOMTileBrush_Impl,
    {
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMTileBrush_Impl::GetTransform(this) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMTileBrush_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMTileBrush_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMTileBrush_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMTileBrush_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetViewbox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *mut XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMTileBrush_Impl::GetViewbox(this) {
                Ok(ok__) => {
                    viewbox.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewbox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *const XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMTileBrush_Impl::SetViewbox(this, core::mem::transmute_copy(&viewbox)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMTileBrush_Impl::GetViewport(this) {
                Ok(ok__) => {
                    viewport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *const XPS_RECT) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMTileBrush_Impl::SetViewport(this, core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn GetTileMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMTileBrush_Impl::GetTileMode(this) {
                Ok(ok__) => {
                    tilemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tilemode: XPS_TILE_MODE) -> windows_core::HRESULT
        where
            Identity: IXpsOMTileBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMTileBrush_Impl::SetTileMode(this, core::mem::transmute_copy(&tilemode)).into()
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, OFFSET>(),
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            GetViewbox: GetViewbox::<Identity, OFFSET>,
            SetViewbox: SetViewbox::<Identity, OFFSET>,
            GetViewport: GetViewport::<Identity, OFFSET>,
            SetViewport: SetViewport::<Identity, OFFSET>,
            GetTileMode: GetTileMode::<Identity, OFFSET>,
            SetTileMode: SetTileMode::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMVisual_Vtbl
    where
        Identity: IXpsOMVisual_Impl,
    {
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetTransform(this) {
                Ok(ok__) => {
                    matrixtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetTransformLocal(this) {
                Ok(ok__) => {
                    matrixtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetTransformLocal(this, windows_core::from_raw_borrowed(&matrixtransform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetTransformLookup(this) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetClipGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetClipGeometry(this) {
                Ok(ok__) => {
                    clipgeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetClipGeometryLocal(this) {
                Ok(ok__) => {
                    clipgeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetClipGeometryLocal(this, windows_core::from_raw_borrowed(&clipgeometry)).into()
        }
        unsafe extern "system" fn GetClipGeometryLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetClipGeometryLookup(this) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetClipGeometryLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetOpacity(this) {
                Ok(ok__) => {
                    opacity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetOpacityMaskBrush(this) {
                Ok(ok__) => {
                    opacitymaskbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetOpacityMaskBrushLocal(this) {
                Ok(ok__) => {
                    opacitymaskbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetOpacityMaskBrushLocal(this, windows_core::from_raw_borrowed(&opacitymaskbrush)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetOpacityMaskBrushLookup(this) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetOpacityMaskBrushLookup(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetIsHyperlinkTarget(this) {
                Ok(ok__) => {
                    ishyperlink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetIsHyperlinkTarget(this, core::mem::transmute_copy(&ishyperlink)).into()
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hyperlinkuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetHyperlinkNavigateUri(this) {
                Ok(ok__) => {
                    hyperlinkuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hyperlinkuri: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetHyperlinkNavigateUri(this, windows_core::from_raw_borrowed(&hyperlinkuri)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisual_Impl::GetLanguage(this) {
                Ok(ok__) => {
                    language.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisual_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(),
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            GetClipGeometry: GetClipGeometry::<Identity, OFFSET>,
            GetClipGeometryLocal: GetClipGeometryLocal::<Identity, OFFSET>,
            SetClipGeometryLocal: SetClipGeometryLocal::<Identity, OFFSET>,
            GetClipGeometryLookup: GetClipGeometryLookup::<Identity, OFFSET>,
            SetClipGeometryLookup: SetClipGeometryLookup::<Identity, OFFSET>,
            GetOpacity: GetOpacity::<Identity, OFFSET>,
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            GetOpacityMaskBrush: GetOpacityMaskBrush::<Identity, OFFSET>,
            GetOpacityMaskBrushLocal: GetOpacityMaskBrushLocal::<Identity, OFFSET>,
            SetOpacityMaskBrushLocal: SetOpacityMaskBrushLocal::<Identity, OFFSET>,
            GetOpacityMaskBrushLookup: GetOpacityMaskBrushLookup::<Identity, OFFSET>,
            SetOpacityMaskBrushLookup: SetOpacityMaskBrushLookup::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, OFFSET>,
            GetHyperlinkNavigateUri: GetHyperlinkNavigateUri::<Identity, OFFSET>,
            SetHyperlinkNavigateUri: SetHyperlinkNavigateUri::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMVisualBrush_Vtbl
    where
        Identity: IXpsOMVisualBrush_Impl,
    {
        unsafe extern "system" fn GetVisual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisualBrush_Impl::GetVisual(this) {
                Ok(ok__) => {
                    visual.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisualLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisualBrush_Impl::GetVisualLocal(this) {
                Ok(ok__) => {
                    visual.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisualBrush_Impl::SetVisualLocal(this, windows_core::from_raw_borrowed(&visual)).into()
        }
        unsafe extern "system" fn GetVisualLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisualBrush_Impl::GetVisualLookup(this) {
                Ok(ok__) => {
                    lookup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisualBrush_Impl::SetVisualLookup(this, core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visualbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisualBrush_Impl::Clone(this) {
                Ok(ok__) => {
                    visualbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, OFFSET>(),
            GetVisual: GetVisual::<Identity, OFFSET>,
            GetVisualLocal: GetVisualLocal::<Identity, OFFSET>,
            SetVisualLocal: SetVisualLocal::<Identity, OFFSET>,
            GetVisualLookup: GetVisualLookup::<Identity, OFFSET>,
            SetVisualLookup: SetVisualLookup::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsOMVisualCollection_Vtbl
    where
        Identity: IXpsOMVisualCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisualCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsOMVisualCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    object.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisualCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisualCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisualCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsOMVisualCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsOMVisualCollection_Impl::Append(this, windows_core::from_raw_borrowed(&object)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSignature_Vtbl
    where
        Identity: IXpsSignature_Impl,
    {
        unsafe extern "system" fn GetSignatureId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sigid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetSignatureId(this) {
                Ok(ok__) => {
                    sigid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignature_Impl::GetSignatureValue(this, core::mem::transmute_copy(&signaturehashvalue), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetCertificateEnumerator(this) {
                Ok(ok__) => {
                    certificateenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sigdatetimestring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetSigningTime(this) {
                Ok(ok__) => {
                    sigdatetimestring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetSigningTimeFormat(this) {
                Ok(ok__) => {
                    timeformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetSignaturePartName(this) {
                Ok(ok__) => {
                    signaturepartname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::Verify(this, core::mem::transmute_copy(&x509certificate)) {
                Ok(ok__) => {
                    sigstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetPolicy(this) {
                Ok(ok__) => {
                    policy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetCustomObjectEnumerator(this) {
                Ok(ok__) => {
                    customobjectenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignature_Impl::GetCustomReferenceEnumerator(this) {
                Ok(ok__) => {
                    customreferenceenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignature_Impl::GetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetSignatureXml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *const u8, count: u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignature_Impl::SetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            Verify: Verify::<Identity, OFFSET>,
            GetPolicy: GetPolicy::<Identity, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, OFFSET>,
            SetSignatureXml: SetSignatureXml::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSignatureBlock_Vtbl
    where
        Identity: IXpsSignatureBlock_Impl,
    {
        unsafe extern "system" fn GetRequests<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requests: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureBlock_Impl::GetRequests(this) {
                Ok(ok__) => {
                    requests.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureBlock_Impl::GetPartName(this) {
                Ok(ok__) => {
                    partname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixeddocumentindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureBlock_Impl::GetDocumentIndex(this) {
                Ok(ok__) => {
                    fixeddocumentindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixeddocumentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureBlock_Impl::GetDocumentName(this) {
                Ok(ok__) => {
                    fixeddocumentname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::PCWSTR, signaturerequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureBlock_Impl::CreateRequest(this, core::mem::transmute(&requestid)) {
                Ok(ok__) => {
                    signaturerequest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequests: GetRequests::<Identity, OFFSET>,
            GetPartName: GetPartName::<Identity, OFFSET>,
            GetDocumentIndex: GetDocumentIndex::<Identity, OFFSET>,
            GetDocumentName: GetDocumentName::<Identity, OFFSET>,
            CreateRequest: CreateRequest::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSignatureBlockCollection_Vtbl
    where
        Identity: IXpsSignatureBlockCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlockCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureBlockCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlockCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureBlockCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    signatureblock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureBlockCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureBlockCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSignatureCollection_Vtbl
    where
        Identity: IXpsSignatureCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    signature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSignatureManager_Vtbl
    where
        Identity: IXpsSignatureManager_Impl,
    {
        unsafe extern "system" fn LoadPackageFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureManager_Impl::LoadPackageFile(this, core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn LoadPackageStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureManager_Impl::LoadPackageStream(this, windows_core::from_raw_borrowed(&stream)).into()
        }
        unsafe extern "system" fn Sign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signoptions: *mut core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureManager_Impl::Sign(this, windows_core::from_raw_borrowed(&signoptions), core::mem::transmute_copy(&x509certificate)) {
                Ok(ok__) => {
                    signature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureManager_Impl::GetSignatureOriginPartName(this) {
                Ok(ok__) => {
                    signatureoriginpartname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureManager_Impl::SetSignatureOriginPartName(this, windows_core::from_raw_borrowed(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatures<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatures: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureManager_Impl::GetSignatures(this) {
                Ok(ok__) => {
                    signatures.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSignatureBlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureManager_Impl::AddSignatureBlock(this, windows_core::from_raw_borrowed(&partname), core::mem::transmute_copy(&fixeddocumentindex)) {
                Ok(ok__) => {
                    signatureblock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureBlocks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblocks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureManager_Impl::GetSignatureBlocks(this) {
                Ok(ok__) => {
                    signatureblocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signingoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureManager_Impl::CreateSigningOptions(this) {
                Ok(ok__) => {
                    signingoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePackageToFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureManager_Impl::SavePackageToFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes)).into()
        }
        unsafe extern "system" fn SavePackageToStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureManager_Impl::SavePackageToStream(this, windows_core::from_raw_borrowed(&stream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadPackageFile: LoadPackageFile::<Identity, OFFSET>,
            LoadPackageStream: LoadPackageStream::<Identity, OFFSET>,
            Sign: Sign::<Identity, OFFSET>,
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, OFFSET>,
            GetSignatures: GetSignatures::<Identity, OFFSET>,
            AddSignatureBlock: AddSignatureBlock::<Identity, OFFSET>,
            GetSignatureBlocks: GetSignatureBlocks::<Identity, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, OFFSET>,
            SavePackageToFile: SavePackageToFile::<Identity, OFFSET>,
            SavePackageToStream: SavePackageToStream::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSignatureRequest_Vtbl
    where
        Identity: IXpsSignatureRequest_Impl,
    {
        unsafe extern "system" fn GetIntent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, intent: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequest_Impl::GetIntent(this) {
                Ok(ok__) => {
                    intent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, intent: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureRequest_Impl::SetIntent(this, core::mem::transmute(&intent)).into()
        }
        unsafe extern "system" fn GetRequestedSigner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signername: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequest_Impl::GetRequestedSigner(this) {
                Ok(ok__) => {
                    signername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSigner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signername: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureRequest_Impl::SetRequestedSigner(this, core::mem::transmute(&signername)).into()
        }
        unsafe extern "system" fn GetRequestSignByDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, datestring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequest_Impl::GetRequestSignByDate(this) {
                Ok(ok__) => {
                    datestring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestSignByDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, datestring: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureRequest_Impl::SetRequestSignByDate(this, core::mem::transmute(&datestring)).into()
        }
        unsafe extern "system" fn GetSigningLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, place: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequest_Impl::GetSigningLocale(this) {
                Ok(ok__) => {
                    place.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, place: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureRequest_Impl::SetSigningLocale(this, core::mem::transmute(&place)).into()
        }
        unsafe extern "system" fn GetSpotLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut *mut core::ffi::c_void, x: *mut f32, y: *mut f32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureRequest_Impl::GetSpotLocation(this, core::mem::transmute_copy(&pageindex), core::mem::transmute_copy(&pagepartname), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetSpotLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureRequest_Impl::SetSpotLocation(this, core::mem::transmute_copy(&pageindex), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn GetRequestId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequest_Impl::GetRequestId(this) {
                Ok(ok__) => {
                    requestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequest_Impl::GetSignature(this) {
                Ok(ok__) => {
                    signature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIntent: GetIntent::<Identity, OFFSET>,
            SetIntent: SetIntent::<Identity, OFFSET>,
            GetRequestedSigner: GetRequestedSigner::<Identity, OFFSET>,
            SetRequestedSigner: SetRequestedSigner::<Identity, OFFSET>,
            GetRequestSignByDate: GetRequestSignByDate::<Identity, OFFSET>,
            SetRequestSignByDate: SetRequestSignByDate::<Identity, OFFSET>,
            GetSigningLocale: GetSigningLocale::<Identity, OFFSET>,
            SetSigningLocale: SetSigningLocale::<Identity, OFFSET>,
            GetSpotLocation: GetSpotLocation::<Identity, OFFSET>,
            SetSpotLocation: SetSpotLocation::<Identity, OFFSET>,
            GetRequestId: GetRequestId::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSignatureRequestCollection_Vtbl
    where
        Identity: IXpsSignatureRequestCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequestCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequestCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signaturerequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequestCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSignatureRequestCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    signaturerequest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IXpsSignatureRequestCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSignatureRequestCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsSigningOptions_Vtbl
    where
        Identity: IXpsSigningOptions_Impl,
    {
        unsafe extern "system" fn GetSignatureId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetSignatureId(this) {
                Ok(ok__) => {
                    signatureid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSigningOptions_Impl::SetSignatureId(this, core::mem::transmute(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetSignatureMethod(this) {
                Ok(ok__) => {
                    signaturemethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSigningOptions_Impl::SetSignatureMethod(this, core::mem::transmute(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDigestMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetDigestMethod(this) {
                Ok(ok__) => {
                    digestmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigestMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSigningOptions_Impl::SetDigestMethod(this, core::mem::transmute(&digestmethod)).into()
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetSignaturePartName(this) {
                Ok(ok__) => {
                    signaturepartname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSigningOptions_Impl::SetSignaturePartName(this, windows_core::from_raw_borrowed(&signaturepartname)).into()
        }
        unsafe extern "system" fn GetPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetPolicy(this) {
                Ok(ok__) => {
                    policy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: XPS_SIGN_POLICY) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSigningOptions_Impl::SetPolicy(this, core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetSigningTimeFormat(this) {
                Ok(ok__) => {
                    timeformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningTimeFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSigningOptions_Impl::SetSigningTimeFormat(this, core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetCustomObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetCustomObjects(this) {
                Ok(ok__) => {
                    customobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetCustomReferences(this) {
                Ok(ok__) => {
                    customreferenceset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetCertificateSet(this) {
                Ok(ok__) => {
                    certificateset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsSigningOptions_Impl::GetFlags(this) {
                Ok(ok__) => {
                    flags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> windows_core::HRESULT
        where
            Identity: IXpsSigningOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsSigningOptions_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            SetDigestMethod: SetDigestMethod::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, OFFSET>,
            GetPolicy: GetPolicy::<Identity, OFFSET>,
            SetPolicy: SetPolicy::<Identity, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, OFFSET>,
            SetSigningTimeFormat: SetSigningTimeFormat::<Identity, OFFSET>,
            GetCustomObjects: GetCustomObjects::<Identity, OFFSET>,
            GetCustomReferences: GetCustomReferences::<Identity, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSigningOptions as windows_core::Interface>::IID
    }
}
