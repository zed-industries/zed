#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IRichEditOle_Impl: Sized {
    fn GetClientSite(&self) -> windows_core::Result<super::super::super::System::Ole::IOleClientSite>;
    fn GetObjectCount(&self) -> i32;
    fn GetLinkCount(&self) -> i32;
    fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> windows_core::Result<()>;
    fn InsertObject(&self, lpreobject: *mut REOBJECT) -> windows_core::Result<()>;
    fn ConvertObject(&self, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn ActivateAs(&self, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetHostNames(&self, lpstrcontainerapp: &windows_core::PCSTR, lpstrcontainerobj: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetLinkAvailable(&self, iob: i32, favailable: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> windows_core::Result<()>;
    fn HandsOffStorage(&self, iob: i32) -> windows_core::Result<()>;
    fn SaveCompleted(&self, iob: i32, lpstg: Option<&super::super::super::System::Com::StructuredStorage::IStorage>) -> windows_core::Result<()>;
    fn InPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut Option<super::super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn ImportDataObject(&self, lpdataobj: Option<&super::super::super::System::Com::IDataObject>, cf: u16, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IRichEditOle {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IRichEditOle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>() -> IRichEditOle_Vtbl {
        unsafe extern "system" fn GetClientSite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpolesite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRichEditOle_Impl::GetClientSite(this) {
                Ok(ok__) => {
                    core::ptr::write(lplpolesite, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::GetObjectCount(this)
        }
        unsafe extern "system" fn GetLinkCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::GetLinkCount(this)
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::GetObject(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&lpreobject), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn InsertObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpreobject: *mut REOBJECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::InsertObject(this, core::mem::transmute_copy(&lpreobject)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::ConvertObject(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&rclsidnew), core::mem::transmute(&lpstrusertypenew)).into()
        }
        unsafe extern "system" fn ActivateAs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::ActivateAs(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rclsidas)).into()
        }
        unsafe extern "system" fn SetHostNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstrcontainerapp: windows_core::PCSTR, lpstrcontainerobj: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::SetHostNames(this, core::mem::transmute(&lpstrcontainerapp), core::mem::transmute(&lpstrcontainerobj)).into()
        }
        unsafe extern "system" fn SetLinkAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, favailable: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::SetLinkAvailable(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&favailable)).into()
        }
        unsafe extern "system" fn SetDvaspect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, dvaspect: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::SetDvaspect(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&dvaspect)).into()
        }
        unsafe extern "system" fn HandsOffStorage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::HandsOffStorage(this, core::mem::transmute_copy(&iob)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, lpstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::SaveCompleted(this, core::mem::transmute_copy(&iob), windows_core::from_raw_borrowed(&lpstg)).into()
        }
        unsafe extern "system" fn InPlaceDeactivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::InPlaceDeactivate(this).into()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
        }
        unsafe extern "system" fn GetClipboardData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
        }
        unsafe extern "system" fn ImportDataObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, cf: u16, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOle_Impl::ImportDataObject(this, windows_core::from_raw_borrowed(&lpdataobj), core::mem::transmute_copy(&cf), core::mem::transmute_copy(&hmetapict)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientSite: GetClientSite::<Identity, Impl, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, Impl, OFFSET>,
            GetLinkCount: GetLinkCount::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            InsertObject: InsertObject::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            ActivateAs: ActivateAs::<Identity, Impl, OFFSET>,
            SetHostNames: SetHostNames::<Identity, Impl, OFFSET>,
            SetLinkAvailable: SetLinkAvailable::<Identity, Impl, OFFSET>,
            SetDvaspect: SetDvaspect::<Identity, Impl, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            InPlaceDeactivate: InPlaceDeactivate::<Identity, Impl, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, Impl, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, Impl, OFFSET>,
            ImportDataObject: ImportDataObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditOle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IRichEditOleCallback_Impl: Sized {
    fn GetNewStorage(&self) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::IStorage>;
    fn GetInPlaceContext(&self, lplpframe: *mut Option<super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: *mut Option<super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> windows_core::Result<()>;
    fn ShowContainerUI(&self, fshow: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn QueryInsertObject(&self, lpclsid: *mut windows_core::GUID, lpstg: Option<&super::super::super::System::Com::StructuredStorage::IStorage>, cp: i32) -> windows_core::Result<()>;
    fn DeleteObject(&self, lpoleobj: Option<&super::super::super::System::Ole::IOleObject>) -> windows_core::Result<()>;
    fn QueryAcceptData(&self, lpdataobj: Option<&super::super::super::System::Com::IDataObject>, lpcfformat: *mut u16, reco: super::super::super::System::SystemServices::RECO_FLAGS, freally: super::super::super::Foundation::BOOL, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut Option<super::super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn GetDragDropEffect(&self, fdrag: super::super::super::Foundation::BOOL, grfkeystate: super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, pdweffect: *mut super::super::super::System::Ole::DROPEFFECT) -> windows_core::Result<()>;
    fn GetContextMenu(&self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: Option<&super::super::super::System::Ole::IOleObject>, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IRichEditOleCallback {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices", feature = "Win32_UI_WindowsAndMessaging"))]
impl IRichEditOleCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>() -> IRichEditOleCallback_Vtbl {
        unsafe extern "system" fn GetNewStorage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRichEditOleCallback_Impl::GetNewStorage(this) {
                Ok(ok__) => {
                    core::ptr::write(lplpstg, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInPlaceContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpframe: *mut *mut core::ffi::c_void, lplpdoc: *mut *mut core::ffi::c_void, lpframeinfo: *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::GetInPlaceContext(this, core::mem::transmute_copy(&lplpframe), core::mem::transmute_copy(&lplpdoc), core::mem::transmute_copy(&lpframeinfo)).into()
        }
        unsafe extern "system" fn ShowContainerUI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::ShowContainerUI(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn QueryInsertObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpclsid: *mut windows_core::GUID, lpstg: *mut core::ffi::c_void, cp: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::QueryInsertObject(this, core::mem::transmute_copy(&lpclsid), windows_core::from_raw_borrowed(&lpstg), core::mem::transmute_copy(&cp)).into()
        }
        unsafe extern "system" fn DeleteObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpoleobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::DeleteObject(this, windows_core::from_raw_borrowed(&lpoleobj)).into()
        }
        unsafe extern "system" fn QueryAcceptData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, lpcfformat: *mut u16, reco: super::super::super::System::SystemServices::RECO_FLAGS, freally: super::super::super::Foundation::BOOL, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::QueryAcceptData(this, windows_core::from_raw_borrowed(&lpdataobj), core::mem::transmute_copy(&lpcfformat), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&freally), core::mem::transmute_copy(&hmetapict)).into()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
        }
        unsafe extern "system" fn GetClipboardData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
        }
        unsafe extern "system" fn GetDragDropEffect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdrag: super::super::super::Foundation::BOOL, grfkeystate: super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, pdweffect: *mut super::super::super::System::Ole::DROPEFFECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::GetDragDropEffect(this, core::mem::transmute_copy(&fdrag), core::mem::transmute_copy(&grfkeystate), core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn GetContextMenu<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: *mut core::ffi::c_void, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRichEditOleCallback_Impl::GetContextMenu(this, core::mem::transmute_copy(&seltype), windows_core::from_raw_borrowed(&lpoleobj), core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&lphmenu)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNewStorage: GetNewStorage::<Identity, Impl, OFFSET>,
            GetInPlaceContext: GetInPlaceContext::<Identity, Impl, OFFSET>,
            ShowContainerUI: ShowContainerUI::<Identity, Impl, OFFSET>,
            QueryInsertObject: QueryInsertObject::<Identity, Impl, OFFSET>,
            DeleteObject: DeleteObject::<Identity, Impl, OFFSET>,
            QueryAcceptData: QueryAcceptData::<Identity, Impl, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, Impl, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, Impl, OFFSET>,
            GetDragDropEffect: GetDragDropEffect::<Identity, Impl, OFFSET>,
            GetContextMenu: GetContextMenu::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditOleCallback as windows_core::Interface>::IID
    }
}
pub trait IRicheditUiaOverrides_Impl: Sized {
    fn GetPropertyOverrideValue(&self, propertyid: i32, pretvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRicheditUiaOverrides {}
impl IRicheditUiaOverrides_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRicheditUiaOverrides_Impl, const OFFSET: isize>() -> IRicheditUiaOverrides_Vtbl {
        unsafe extern "system" fn GetPropertyOverrideValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRicheditUiaOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: i32, pretvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRicheditUiaOverrides_Impl::GetPropertyOverrideValue(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&pretvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPropertyOverrideValue: GetPropertyOverrideValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRicheditUiaOverrides as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDisplays_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextDisplays {}
#[cfg(feature = "Win32_System_Com")]
impl ITextDisplays_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDisplays_Impl, const OFFSET: isize>() -> ITextDisplays_Vtbl {
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDisplays as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSelection(&self) -> windows_core::Result<ITextSelection>;
    fn GetStoryCount(&self) -> windows_core::Result<i32>;
    fn GetStoryRanges(&self) -> windows_core::Result<ITextStoryRanges>;
    fn GetSaved(&self) -> windows_core::Result<i32>;
    fn SetSaved(&self, value: tomConstants) -> windows_core::Result<()>;
    fn GetDefaultTabStop(&self) -> windows_core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()>;
    fn New(&self) -> windows_core::Result<()>;
    fn Open(&self, pvar: *const windows_core::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::Result<()>;
    fn Save(&self, pvar: *const windows_core::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::Result<()>;
    fn Freeze(&self) -> windows_core::Result<i32>;
    fn Unfreeze(&self) -> windows_core::Result<i32>;
    fn BeginEditCollection(&self) -> windows_core::Result<()>;
    fn EndEditCollection(&self) -> windows_core::Result<()>;
    fn Undo(&self, count: i32) -> windows_core::Result<i32>;
    fn Redo(&self, count: i32) -> windows_core::Result<i32>;
    fn Range(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange>;
    fn RangeFromPoint(&self, x: i32, y: i32) -> windows_core::Result<ITextRange>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>() -> ITextDocument_Vtbl {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(pname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::GetSelection(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::GetStoryCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::GetStoryRanges(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstories, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSaved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::GetSaved(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument_Impl::SetSaved(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDefaultTabStop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::GetDefaultTabStop(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument_Impl::SetDefaultTabStop(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn New<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument_Impl::New(this).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>, flags: tomConstants, codepage: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument_Impl::Open(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&codepage)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>, flags: tomConstants, codepage: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument_Impl::Save(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&codepage)).into()
        }
        unsafe extern "system" fn Freeze<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::Freeze(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unfreeze<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::Unfreeze(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEditCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument_Impl::BeginEditCollection(this).into()
        }
        unsafe extern "system" fn EndEditCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument_Impl::EndEditCollection(this).into()
        }
        unsafe extern "system" fn Undo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::Undo(this, core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Redo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::Redo(this, core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Range<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::Range(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument_Impl::RangeFromPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetStoryCount: GetStoryCount::<Identity, Impl, OFFSET>,
            GetStoryRanges: GetStoryRanges::<Identity, Impl, OFFSET>,
            GetSaved: GetSaved::<Identity, Impl, OFFSET>,
            SetSaved: SetSaved::<Identity, Impl, OFFSET>,
            GetDefaultTabStop: GetDefaultTabStop::<Identity, Impl, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, Impl, OFFSET>,
            New: New::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Freeze: Freeze::<Identity, Impl, OFFSET>,
            Unfreeze: Unfreeze::<Identity, Impl, OFFSET>,
            BeginEditCollection: BeginEditCollection::<Identity, Impl, OFFSET>,
            EndEditCollection: EndEditCollection::<Identity, Impl, OFFSET>,
            Undo: Undo::<Identity, Impl, OFFSET>,
            Redo: Redo::<Identity, Impl, OFFSET>,
            Range: Range::<Identity, Impl, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocument2_Impl: Sized + ITextDocument_Impl {
    fn GetCaretType(&self) -> windows_core::Result<i32>;
    fn SetCaretType(&self, value: i32) -> windows_core::Result<()>;
    fn GetDisplays(&self) -> windows_core::Result<ITextDisplays>;
    fn GetDocumentFont(&self) -> windows_core::Result<ITextFont2>;
    fn SetDocumentFont(&self, pfont: Option<&ITextFont2>) -> windows_core::Result<()>;
    fn GetDocumentPara(&self) -> windows_core::Result<ITextPara2>;
    fn SetDocumentPara(&self, ppara: Option<&ITextPara2>) -> windows_core::Result<()>;
    fn GetEastAsianFlags(&self) -> windows_core::Result<tomConstants>;
    fn GetGenerator(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetIMEInProgress(&self, value: i32) -> windows_core::Result<()>;
    fn GetNotificationMode(&self) -> windows_core::Result<i32>;
    fn SetNotificationMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetSelection2(&self) -> windows_core::Result<ITextSelection2>;
    fn GetStoryRanges2(&self) -> windows_core::Result<ITextStoryRanges2>;
    fn GetTypographyOptions(&self) -> windows_core::Result<i32>;
    fn GetVersion(&self) -> windows_core::Result<i32>;
    fn GetWindow(&self) -> windows_core::Result<i64>;
    fn AttachMsgFilter(&self, pfilter: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> windows_core::Result<()>;
    fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetClientRect(&self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> windows_core::Result<i32>;
    fn GetImmContext(&self) -> windows_core::Result<i64>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetStrings(&self) -> windows_core::Result<ITextStrings>;
    fn Notify(&self, notify: i32) -> windows_core::Result<()>;
    fn Range2(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2>;
    fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> windows_core::Result<ITextRange2>;
    fn ReleaseCallManager(&self, pvoid: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()>;
    fn SetEffectColor(&self, index: i32, value: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetTypographyOptions(&self, options: i32, mask: i32) -> windows_core::Result<()>;
    fn SysBeep(&self) -> windows_core::Result<()>;
    fn Update(&self, value: i32) -> windows_core::Result<()>;
    fn UpdateWindow(&self) -> windows_core::Result<()>;
    fn GetMathProperties(&self) -> windows_core::Result<i32>;
    fn SetMathProperties(&self, options: i32, mask: i32) -> windows_core::Result<()>;
    fn GetActiveStory(&self) -> windows_core::Result<ITextStory>;
    fn SetActiveStory(&self, pstory: Option<&ITextStory>) -> windows_core::Result<()>;
    fn GetMainStory(&self) -> windows_core::Result<ITextStory>;
    fn GetNewStory(&self) -> windows_core::Result<ITextStory>;
    fn GetStory(&self, index: i32) -> windows_core::Result<ITextStory>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>() -> ITextDocument2_Vtbl {
        unsafe extern "system" fn GetCaretType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetCaretType(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetCaretType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDisplays<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplays: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetDisplays(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdisplays, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentFont<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetDocumentFont(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfont, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentFont<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetDocumentFont(this, windows_core::from_raw_borrowed(&pfont)).into()
        }
        unsafe extern "system" fn GetDocumentPara<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetDocumentPara(this) {
                Ok(ok__) => {
                    core::ptr::write(pppara, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentPara<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetDocumentPara(this, windows_core::from_raw_borrowed(&ppara)).into()
        }
        unsafe extern "system" fn GetEastAsianFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetEastAsianFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetGenerator(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIMEInProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetIMEInProgress(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNotificationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetNotificationMode(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetNotificationMode(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSelection2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetSelection2(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetStoryRanges2(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstories, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypographyOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetTypographyOptions(this) {
                Ok(ok__) => {
                    core::ptr::write(poptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetWindow(this) {
                Ok(ok__) => {
                    core::ptr::write(phwnd, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachMsgFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::AttachMsgFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        unsafe extern "system" fn CheckTextLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: i32, pcch: *const i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::CheckTextLimit(this, core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn GetCallManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetCallManager(this) {
                Ok(ok__) => {
                    core::ptr::write(ppvoid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::GetClientRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
        }
        unsafe extern "system" fn GetEffectColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetEffectColor(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetImmContext(this) {
                Ok(ok__) => {
                    core::ptr::write(pcontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredFont<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::GetPreferredFont(this, core::mem::transmute_copy(&cp), core::mem::transmute_copy(&charrep), core::mem::transmute_copy(&options), core::mem::transmute_copy(&curcharrep), core::mem::transmute_copy(&curfontsize), core::mem::transmute_copy(&pbstr), core::mem::transmute_copy(&ppitchandfamily), core::mem::transmute_copy(&pnewfontsize)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetStrings(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::Notify(this, core::mem::transmute_copy(&notify)).into()
        }
        unsafe extern "system" fn Range2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::Range2(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::RangeFromPoint2(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::ReleaseCallManager(this, windows_core::from_raw_borrowed(&pvoid)).into()
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::ReleaseImmContext(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn SetEffectColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetEffectColor(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetTypographyOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: i32, mask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetTypographyOptions(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SysBeep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SysBeep(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::Update(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UpdateWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::UpdateWindow(this).into()
        }
        unsafe extern "system" fn GetMathProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetMathProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(poptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: i32, mask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetMathProperties(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetActiveStory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetActiveStory(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveStory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2_Impl::SetActiveStory(this, windows_core::from_raw_borrowed(&pstory)).into()
        }
        unsafe extern "system" fn GetMainStory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetMainStory(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNewStory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetNewStory(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2_Impl::GetStory(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(ppstory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITextDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCaretType: GetCaretType::<Identity, Impl, OFFSET>,
            SetCaretType: SetCaretType::<Identity, Impl, OFFSET>,
            GetDisplays: GetDisplays::<Identity, Impl, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, Impl, OFFSET>,
            SetDocumentFont: SetDocumentFont::<Identity, Impl, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, Impl, OFFSET>,
            SetDocumentPara: SetDocumentPara::<Identity, Impl, OFFSET>,
            GetEastAsianFlags: GetEastAsianFlags::<Identity, Impl, OFFSET>,
            GetGenerator: GetGenerator::<Identity, Impl, OFFSET>,
            SetIMEInProgress: SetIMEInProgress::<Identity, Impl, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, Impl, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, Impl, OFFSET>,
            GetSelection2: GetSelection2::<Identity, Impl, OFFSET>,
            GetStoryRanges2: GetStoryRanges2::<Identity, Impl, OFFSET>,
            GetTypographyOptions: GetTypographyOptions::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            AttachMsgFilter: AttachMsgFilter::<Identity, Impl, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, Impl, OFFSET>,
            GetCallManager: GetCallManager::<Identity, Impl, OFFSET>,
            GetClientRect: GetClientRect::<Identity, Impl, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, Impl, OFFSET>,
            GetImmContext: GetImmContext::<Identity, Impl, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetStrings: GetStrings::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Range2: Range2::<Identity, Impl, OFFSET>,
            RangeFromPoint2: RangeFromPoint2::<Identity, Impl, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, Impl, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, Impl, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetTypographyOptions: SetTypographyOptions::<Identity, Impl, OFFSET>,
            SysBeep: SysBeep::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, Impl, OFFSET>,
            GetMathProperties: GetMathProperties::<Identity, Impl, OFFSET>,
            SetMathProperties: SetMathProperties::<Identity, Impl, OFFSET>,
            GetActiveStory: GetActiveStory::<Identity, Impl, OFFSET>,
            SetActiveStory: SetActiveStory::<Identity, Impl, OFFSET>,
            GetMainStory: GetMainStory::<Identity, Impl, OFFSET>,
            GetNewStory: GetNewStory::<Identity, Impl, OFFSET>,
            GetStory: GetStory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocument2Old_Impl: Sized + ITextDocument_Impl {
    fn AttachMsgFilter(&self, pfilter: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetEffectColor(&self, index: i32, cr: super::super::super::Foundation::COLORREF) -> windows_core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> windows_core::Result<super::super::super::Foundation::COLORREF>;
    fn GetCaretType(&self) -> windows_core::Result<i32>;
    fn SetCaretType(&self, carettype: i32) -> windows_core::Result<()>;
    fn GetImmContext(&self) -> windows_core::Result<i64>;
    fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()>;
    fn GetNotificationMode(&self) -> windows_core::Result<i32>;
    fn SetNotificationMode(&self, mode: i32) -> windows_core::Result<()>;
    fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
    fn GetSelection2(&self) -> windows_core::Result<ITextSelection>;
    fn GetWindow(&self) -> windows_core::Result<i32>;
    fn GetFEFlags(&self) -> windows_core::Result<i32>;
    fn UpdateWindow(&self) -> windows_core::Result<()>;
    fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> windows_core::Result<()>;
    fn IMEInProgress(&self, value: i32) -> windows_core::Result<()>;
    fn SysBeep(&self) -> windows_core::Result<()>;
    fn Update(&self, mode: i32) -> windows_core::Result<()>;
    fn Notify(&self, notify: i32) -> windows_core::Result<()>;
    fn GetDocumentFont(&self) -> windows_core::Result<ITextFont>;
    fn GetDocumentPara(&self) -> windows_core::Result<ITextPara>;
    fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ReleaseCallManager(&self, pvoid: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextDocument2Old {}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2Old_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>() -> ITextDocument2Old_Vtbl {
        unsafe extern "system" fn AttachMsgFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::AttachMsgFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        unsafe extern "system" fn SetEffectColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, cr: super::super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::SetEffectColor(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&cr)).into()
        }
        unsafe extern "system" fn GetEffectColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pcr: *mut super::super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetEffectColor(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pcr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcarettype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetCaretType(this) {
                Ok(ok__) => {
                    core::ptr::write(pcarettype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, carettype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::SetCaretType(this, core::mem::transmute_copy(&carettype)).into()
        }
        unsafe extern "system" fn GetImmContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetImmContext(this) {
                Ok(ok__) => {
                    core::ptr::write(pcontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::ReleaseImmContext(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetPreferredFont<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::GetPreferredFont(this, core::mem::transmute_copy(&cp), core::mem::transmute_copy(&charrep), core::mem::transmute_copy(&option), core::mem::transmute_copy(&charrepcur), core::mem::transmute_copy(&curfontsize), core::mem::transmute_copy(&pbstr), core::mem::transmute_copy(&ppitchandfamily), core::mem::transmute_copy(&pnewfontsize)).into()
        }
        unsafe extern "system" fn GetNotificationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetNotificationMode(this) {
                Ok(ok__) => {
                    core::ptr::write(pmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::SetNotificationMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn GetClientRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::GetClientRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
        }
        unsafe extern "system" fn GetSelection2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetSelection2(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetWindow(this) {
                Ok(ok__) => {
                    core::ptr::write(phwnd, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFEFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetFEFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::UpdateWindow(this).into()
        }
        unsafe extern "system" fn CheckTextLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: i32, pcch: *const i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::CheckTextLimit(this, core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn IMEInProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::IMEInProgress(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SysBeep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::SysBeep(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::Update(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::Notify(this, core::mem::transmute_copy(&notify)).into()
        }
        unsafe extern "system" fn GetDocumentFont<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitextfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetDocumentFont(this) {
                Ok(ok__) => {
                    core::ptr::write(ppitextfont, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentPara<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitextpara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetDocumentPara(this) {
                Ok(ok__) => {
                    core::ptr::write(ppitextpara, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextDocument2Old_Impl::GetCallManager(this) {
                Ok(ok__) => {
                    core::ptr::write(ppvoid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextDocument2Old_Impl::ReleaseCallManager(this, windows_core::from_raw_borrowed(&pvoid)).into()
        }
        Self {
            base__: ITextDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            AttachMsgFilter: AttachMsgFilter::<Identity, Impl, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, Impl, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, Impl, OFFSET>,
            GetCaretType: GetCaretType::<Identity, Impl, OFFSET>,
            SetCaretType: SetCaretType::<Identity, Impl, OFFSET>,
            GetImmContext: GetImmContext::<Identity, Impl, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, Impl, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, Impl, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, Impl, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, Impl, OFFSET>,
            GetClientRect: GetClientRect::<Identity, Impl, OFFSET>,
            GetSelection2: GetSelection2::<Identity, Impl, OFFSET>,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            GetFEFlags: GetFEFlags::<Identity, Impl, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, Impl, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, Impl, OFFSET>,
            IMEInProgress: IMEInProgress::<Identity, Impl, OFFSET>,
            SysBeep: SysBeep::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, Impl, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, Impl, OFFSET>,
            GetCallManager: GetCallManager::<Identity, Impl, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument2Old as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextFont_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetDuplicate(&self) -> windows_core::Result<ITextFont>;
    fn SetDuplicate(&self, pfont: Option<&ITextFont>) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn IsEqual(&self, pfont: Option<&ITextFont>) -> windows_core::Result<i32>;
    fn Reset(&self, value: tomConstants) -> windows_core::Result<()>;
    fn GetStyle(&self) -> windows_core::Result<i32>;
    fn SetStyle(&self, value: i32) -> windows_core::Result<()>;
    fn GetAllCaps(&self) -> windows_core::Result<i32>;
    fn SetAllCaps(&self, value: i32) -> windows_core::Result<()>;
    fn GetAnimation(&self) -> windows_core::Result<i32>;
    fn SetAnimation(&self, value: i32) -> windows_core::Result<()>;
    fn GetBackColor(&self) -> windows_core::Result<i32>;
    fn SetBackColor(&self, value: i32) -> windows_core::Result<()>;
    fn GetBold(&self) -> windows_core::Result<i32>;
    fn SetBold(&self, value: i32) -> windows_core::Result<()>;
    fn GetEmboss(&self) -> windows_core::Result<i32>;
    fn SetEmboss(&self, value: i32) -> windows_core::Result<()>;
    fn GetForeColor(&self) -> windows_core::Result<i32>;
    fn SetForeColor(&self, value: i32) -> windows_core::Result<()>;
    fn GetHidden(&self) -> windows_core::Result<i32>;
    fn SetHidden(&self, value: i32) -> windows_core::Result<()>;
    fn GetEngrave(&self) -> windows_core::Result<i32>;
    fn SetEngrave(&self, value: i32) -> windows_core::Result<()>;
    fn GetItalic(&self) -> windows_core::Result<i32>;
    fn SetItalic(&self, value: i32) -> windows_core::Result<()>;
    fn GetKerning(&self) -> windows_core::Result<f32>;
    fn SetKerning(&self, value: f32) -> windows_core::Result<()>;
    fn GetLanguageID(&self) -> windows_core::Result<i32>;
    fn SetLanguageID(&self, value: i32) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetOutline(&self) -> windows_core::Result<i32>;
    fn SetOutline(&self, value: i32) -> windows_core::Result<()>;
    fn GetPosition(&self) -> windows_core::Result<f32>;
    fn SetPosition(&self, value: f32) -> windows_core::Result<()>;
    fn GetProtected(&self) -> windows_core::Result<i32>;
    fn SetProtected(&self, value: i32) -> windows_core::Result<()>;
    fn GetShadow(&self) -> windows_core::Result<i32>;
    fn SetShadow(&self, value: i32) -> windows_core::Result<()>;
    fn GetSize(&self) -> windows_core::Result<f32>;
    fn SetSize(&self, value: f32) -> windows_core::Result<()>;
    fn GetSmallCaps(&self) -> windows_core::Result<i32>;
    fn SetSmallCaps(&self, value: i32) -> windows_core::Result<()>;
    fn GetSpacing(&self) -> windows_core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> windows_core::Result<()>;
    fn GetStrikeThrough(&self) -> windows_core::Result<i32>;
    fn SetStrikeThrough(&self, value: i32) -> windows_core::Result<()>;
    fn GetSubscript(&self) -> windows_core::Result<i32>;
    fn SetSubscript(&self, value: i32) -> windows_core::Result<()>;
    fn GetSuperscript(&self) -> windows_core::Result<i32>;
    fn SetSuperscript(&self, value: i32) -> windows_core::Result<()>;
    fn GetUnderline(&self) -> windows_core::Result<i32>;
    fn SetUnderline(&self, value: i32) -> windows_core::Result<()>;
    fn GetWeight(&self) -> windows_core::Result<i32>;
    fn SetWeight(&self, value: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextFont {}
#[cfg(feature = "Win32_System_Com")]
impl ITextFont_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>() -> ITextFont_Vtbl {
        unsafe extern "system" fn GetDuplicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetDuplicate(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfont, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetDuplicate(this, windows_core::from_raw_borrowed(&pfont)).into()
        }
        unsafe extern "system" fn CanChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::CanChange(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::IsEqual(this, windows_core::from_raw_borrowed(&pfont)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStyle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetStyle(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetStyle(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAllCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetAllCaps(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetAllCaps(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAnimation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetAnimation(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetAnimation(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBackColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetBackColor(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetBackColor(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetBold(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetBold(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEmboss<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetEmboss(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmboss<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetEmboss(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetForeColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetForeColor(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetForeColor(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHidden<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetHidden(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetHidden(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEngrave<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetEngrave(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEngrave<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetEngrave(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetItalic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetItalic(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetItalic(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKerning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetKerning(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetKerning(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLanguageID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetLanguageID(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetLanguageID(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetName(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn GetOutline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetOutline(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetOutline(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetPosition(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetPosition(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetProtected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetProtected(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetProtected(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetShadow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetShadow(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetShadow(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetSize(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSmallCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetSmallCaps(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetSmallCaps(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpacing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetSpacing(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetSpacing(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStrikeThrough<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetStrikeThrough(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikeThrough<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetStrikeThrough(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSubscript<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetSubscript(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetSubscript(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSuperscript<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetSuperscript(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetSuperscript(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnderline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetUnderline(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetUnderline(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetWeight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont_Impl::GetWeight(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont_Impl::SetWeight(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, Impl, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, Impl, OFFSET>,
            CanChange: CanChange::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            SetStyle: SetStyle::<Identity, Impl, OFFSET>,
            GetAllCaps: GetAllCaps::<Identity, Impl, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, Impl, OFFSET>,
            GetAnimation: GetAnimation::<Identity, Impl, OFFSET>,
            SetAnimation: SetAnimation::<Identity, Impl, OFFSET>,
            GetBackColor: GetBackColor::<Identity, Impl, OFFSET>,
            SetBackColor: SetBackColor::<Identity, Impl, OFFSET>,
            GetBold: GetBold::<Identity, Impl, OFFSET>,
            SetBold: SetBold::<Identity, Impl, OFFSET>,
            GetEmboss: GetEmboss::<Identity, Impl, OFFSET>,
            SetEmboss: SetEmboss::<Identity, Impl, OFFSET>,
            GetForeColor: GetForeColor::<Identity, Impl, OFFSET>,
            SetForeColor: SetForeColor::<Identity, Impl, OFFSET>,
            GetHidden: GetHidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            GetEngrave: GetEngrave::<Identity, Impl, OFFSET>,
            SetEngrave: SetEngrave::<Identity, Impl, OFFSET>,
            GetItalic: GetItalic::<Identity, Impl, OFFSET>,
            SetItalic: SetItalic::<Identity, Impl, OFFSET>,
            GetKerning: GetKerning::<Identity, Impl, OFFSET>,
            SetKerning: SetKerning::<Identity, Impl, OFFSET>,
            GetLanguageID: GetLanguageID::<Identity, Impl, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetOutline: GetOutline::<Identity, Impl, OFFSET>,
            SetOutline: SetOutline::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            GetProtected: GetProtected::<Identity, Impl, OFFSET>,
            SetProtected: SetProtected::<Identity, Impl, OFFSET>,
            GetShadow: GetShadow::<Identity, Impl, OFFSET>,
            SetShadow: SetShadow::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            GetSmallCaps: GetSmallCaps::<Identity, Impl, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, Impl, OFFSET>,
            GetSpacing: GetSpacing::<Identity, Impl, OFFSET>,
            SetSpacing: SetSpacing::<Identity, Impl, OFFSET>,
            GetStrikeThrough: GetStrikeThrough::<Identity, Impl, OFFSET>,
            SetStrikeThrough: SetStrikeThrough::<Identity, Impl, OFFSET>,
            GetSubscript: GetSubscript::<Identity, Impl, OFFSET>,
            SetSubscript: SetSubscript::<Identity, Impl, OFFSET>,
            GetSuperscript: GetSuperscript::<Identity, Impl, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, Impl, OFFSET>,
            GetUnderline: GetUnderline::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            GetWeight: GetWeight::<Identity, Impl, OFFSET>,
            SetWeight: SetWeight::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextFont as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextFont2_Impl: Sized + ITextFont_Impl {
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetAutoLigatures(&self) -> windows_core::Result<i32>;
    fn SetAutoLigatures(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceAlpha(&self) -> windows_core::Result<i32>;
    fn SetAutospaceAlpha(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceNumeric(&self) -> windows_core::Result<i32>;
    fn SetAutospaceNumeric(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceParens(&self) -> windows_core::Result<i32>;
    fn SetAutospaceParens(&self, value: i32) -> windows_core::Result<()>;
    fn GetCharRep(&self) -> windows_core::Result<i32>;
    fn SetCharRep(&self, value: i32) -> windows_core::Result<()>;
    fn GetCompressionMode(&self) -> windows_core::Result<i32>;
    fn SetCompressionMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetCookie(&self) -> windows_core::Result<i32>;
    fn SetCookie(&self, value: i32) -> windows_core::Result<()>;
    fn GetDoubleStrike(&self) -> windows_core::Result<i32>;
    fn SetDoubleStrike(&self, value: i32) -> windows_core::Result<()>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextFont2>;
    fn SetDuplicate2(&self, pfont: Option<&ITextFont2>) -> windows_core::Result<()>;
    fn GetLinkType(&self) -> windows_core::Result<i32>;
    fn GetMathZone(&self) -> windows_core::Result<i32>;
    fn SetMathZone(&self, value: i32) -> windows_core::Result<()>;
    fn GetModWidthPairs(&self) -> windows_core::Result<i32>;
    fn SetModWidthPairs(&self, value: i32) -> windows_core::Result<()>;
    fn GetModWidthSpace(&self) -> windows_core::Result<i32>;
    fn SetModWidthSpace(&self, value: i32) -> windows_core::Result<()>;
    fn GetOldNumbers(&self) -> windows_core::Result<i32>;
    fn SetOldNumbers(&self, value: i32) -> windows_core::Result<()>;
    fn GetOverlapping(&self) -> windows_core::Result<i32>;
    fn SetOverlapping(&self, value: i32) -> windows_core::Result<()>;
    fn GetPositionSubSuper(&self) -> windows_core::Result<i32>;
    fn SetPositionSubSuper(&self, value: i32) -> windows_core::Result<()>;
    fn GetScaling(&self) -> windows_core::Result<i32>;
    fn SetScaling(&self, value: i32) -> windows_core::Result<()>;
    fn GetSpaceExtension(&self) -> windows_core::Result<f32>;
    fn SetSpaceExtension(&self, value: f32) -> windows_core::Result<()>;
    fn GetUnderlinePositionMode(&self) -> windows_core::Result<i32>;
    fn SetUnderlinePositionMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::Result<()>;
    fn IsEqual2(&self, pfont: Option<&ITextFont2>) -> windows_core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetEffects2(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextFont2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextFont2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>() -> ITextFont2_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoLigatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetAutoLigatures(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoLigatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetAutoLigatures(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceAlpha<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetAutospaceAlpha(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceAlpha<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetAutospaceAlpha(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceNumeric<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetAutospaceNumeric(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceNumeric<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetAutospaceNumeric(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceParens<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetAutospaceParens(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceParens<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetAutospaceParens(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCharRep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetCharRep(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharRep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetCharRep(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCompressionMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetCompressionMode(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompressionMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetCompressionMode(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCookie<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetCookie(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCookie<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetCookie(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDoubleStrike<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetDoubleStrike(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleStrike<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetDoubleStrike(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDuplicate2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetDuplicate2(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfont, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetDuplicate2(this, windows_core::from_raw_borrowed(&pfont)).into()
        }
        unsafe extern "system" fn GetLinkType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetLinkType(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMathZone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetMathZone(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathZone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetMathZone(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetModWidthPairs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetModWidthPairs(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthPairs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetModWidthPairs(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetModWidthSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetModWidthSpace(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetModWidthSpace(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetOldNumbers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetOldNumbers(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldNumbers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetOldNumbers(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetOverlapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetOverlapping(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetOverlapping(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPositionSubSuper<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetPositionSubSuper(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionSubSuper<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetPositionSubSuper(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetScaling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetScaling(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetScaling(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpaceExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetSpaceExtension(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetSpaceExtension(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnderlinePositionMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetUnderlinePositionMode(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderlinePositionMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetUnderlinePositionMode(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEffects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::GetEffects(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetEffects2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::GetEffects2(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn IsEqual2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextFont2_Impl::IsEqual2(this, windows_core::from_raw_borrowed(&pfont)) {
                Ok(ok__) => {
                    core::ptr::write(pb, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetEffects(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetEffects2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetEffects2(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextFont2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ITextFont_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAutoLigatures: GetAutoLigatures::<Identity, Impl, OFFSET>,
            SetAutoLigatures: SetAutoLigatures::<Identity, Impl, OFFSET>,
            GetAutospaceAlpha: GetAutospaceAlpha::<Identity, Impl, OFFSET>,
            SetAutospaceAlpha: SetAutospaceAlpha::<Identity, Impl, OFFSET>,
            GetAutospaceNumeric: GetAutospaceNumeric::<Identity, Impl, OFFSET>,
            SetAutospaceNumeric: SetAutospaceNumeric::<Identity, Impl, OFFSET>,
            GetAutospaceParens: GetAutospaceParens::<Identity, Impl, OFFSET>,
            SetAutospaceParens: SetAutospaceParens::<Identity, Impl, OFFSET>,
            GetCharRep: GetCharRep::<Identity, Impl, OFFSET>,
            SetCharRep: SetCharRep::<Identity, Impl, OFFSET>,
            GetCompressionMode: GetCompressionMode::<Identity, Impl, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, Impl, OFFSET>,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            SetCookie: SetCookie::<Identity, Impl, OFFSET>,
            GetDoubleStrike: GetDoubleStrike::<Identity, Impl, OFFSET>,
            SetDoubleStrike: SetDoubleStrike::<Identity, Impl, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, Impl, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, Impl, OFFSET>,
            GetLinkType: GetLinkType::<Identity, Impl, OFFSET>,
            GetMathZone: GetMathZone::<Identity, Impl, OFFSET>,
            SetMathZone: SetMathZone::<Identity, Impl, OFFSET>,
            GetModWidthPairs: GetModWidthPairs::<Identity, Impl, OFFSET>,
            SetModWidthPairs: SetModWidthPairs::<Identity, Impl, OFFSET>,
            GetModWidthSpace: GetModWidthSpace::<Identity, Impl, OFFSET>,
            SetModWidthSpace: SetModWidthSpace::<Identity, Impl, OFFSET>,
            GetOldNumbers: GetOldNumbers::<Identity, Impl, OFFSET>,
            SetOldNumbers: SetOldNumbers::<Identity, Impl, OFFSET>,
            GetOverlapping: GetOverlapping::<Identity, Impl, OFFSET>,
            SetOverlapping: SetOverlapping::<Identity, Impl, OFFSET>,
            GetPositionSubSuper: GetPositionSubSuper::<Identity, Impl, OFFSET>,
            SetPositionSubSuper: SetPositionSubSuper::<Identity, Impl, OFFSET>,
            GetScaling: GetScaling::<Identity, Impl, OFFSET>,
            SetScaling: SetScaling::<Identity, Impl, OFFSET>,
            GetSpaceExtension: GetSpaceExtension::<Identity, Impl, OFFSET>,
            SetSpaceExtension: SetSpaceExtension::<Identity, Impl, OFFSET>,
            GetUnderlinePositionMode: GetUnderlinePositionMode::<Identity, Impl, OFFSET>,
            SetUnderlinePositionMode: SetUnderlinePositionMode::<Identity, Impl, OFFSET>,
            GetEffects: GetEffects::<Identity, Impl, OFFSET>,
            GetEffects2: GetEffects2::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            IsEqual2: IsEqual2::<Identity, Impl, OFFSET>,
            SetEffects: SetEffects::<Identity, Impl, OFFSET>,
            SetEffects2: SetEffects2::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextFont2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextFont as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost_Impl: Sized {
    fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC;
    fn TxReleaseDC(&self, hdc: super::super::super::Graphics::Gdi::HDC) -> i32;
    fn TxShowScrollBar(&self, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> super::super::super::Foundation::BOOL;
    fn TxSetScrollRange(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxSetScrollPos(&self, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxInvalidateRect(&self, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL);
    fn TxViewChange(&self, fupdate: super::super::super::Foundation::BOOL);
    fn TxCreateCaret(&self, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL;
    fn TxShowCaret(&self, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL;
    fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL;
    fn TxKillTimer(&self, idtimer: u32);
    fn TxScrollWindowEx(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS);
    fn TxSetCapture(&self, fcapture: super::super::super::Foundation::BOOL);
    fn TxSetFocus(&self);
    fn TxSetCursor(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL);
    fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    fn TxActivate(&self, ploldstate: *mut i32) -> windows_core::Result<()>;
    fn TxDeactivate(&self, lnewstate: i32) -> windows_core::Result<()>;
    fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> windows_core::Result<()>;
    fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> windows_core::Result<()>;
    fn TxGetSysColor(&self, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF;
    fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> windows_core::Result<()>;
    fn TxGetMaxLength(&self, plength: *mut u32) -> windows_core::Result<()>;
    fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> windows_core::Result<()>;
    fn TxGetPasswordChar(&self) -> windows_core::Result<i8>;
    fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> windows_core::Result<()>;
    fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> windows_core::Result<()>;
    fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> windows_core::Result<()>;
    fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> windows_core::Result<()>;
    fn TxNotify(&self, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC;
    fn TxImmReleaseContext(&self, himc: super::super::super::Globalization::HIMC);
    fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for ITextHost {}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>() -> ITextHost_Vtbl {
        unsafe extern "system" fn TxGetDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetDC(this)
        }
        unsafe extern "system" fn TxReleaseDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxReleaseDC(this, core::mem::transmute_copy(&hdc))
        }
        unsafe extern "system" fn TxShowScrollBar<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxShowScrollBar(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&fshow))
        }
        unsafe extern "system" fn TxEnableScrollBar<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxEnableScrollBar(this, core::mem::transmute_copy(&fusbflags), core::mem::transmute_copy(&fuarrowflags))
        }
        unsafe extern "system" fn TxSetScrollRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxSetScrollRange(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&nminpos), core::mem::transmute_copy(&nmaxpos), core::mem::transmute_copy(&fredraw))
        }
        unsafe extern "system" fn TxSetScrollPos<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxSetScrollPos(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&npos), core::mem::transmute_copy(&fredraw))
        }
        unsafe extern "system" fn TxInvalidateRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxInvalidateRect(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&fmode))
        }
        unsafe extern "system" fn TxViewChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxViewChange(this, core::mem::transmute_copy(&fupdate))
        }
        unsafe extern "system" fn TxCreateCaret<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxCreateCaret(this, core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&xwidth), core::mem::transmute_copy(&yheight))
        }
        unsafe extern "system" fn TxShowCaret<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxShowCaret(this, core::mem::transmute_copy(&fshow))
        }
        unsafe extern "system" fn TxSetCaretPos<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxSetCaretPos(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y))
        }
        unsafe extern "system" fn TxSetTimer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxSetTimer(this, core::mem::transmute_copy(&idtimer), core::mem::transmute_copy(&utimeout))
        }
        unsafe extern "system" fn TxKillTimer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idtimer: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxKillTimer(this, core::mem::transmute_copy(&idtimer))
        }
        unsafe extern "system" fn TxScrollWindowEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxScrollWindowEx(this, core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&lprcscroll), core::mem::transmute_copy(&lprcclip), core::mem::transmute_copy(&hrgnupdate), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&fuscroll))
        }
        unsafe extern "system" fn TxSetCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxSetCapture(this, core::mem::transmute_copy(&fcapture))
        }
        unsafe extern "system" fn TxSetFocus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxSetFocus(this)
        }
        unsafe extern "system" fn TxSetCursor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxSetCursor(this, core::mem::transmute_copy(&hcur), core::mem::transmute_copy(&ftext))
        }
        unsafe extern "system" fn TxScreenToClient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxScreenToClient(this, core::mem::transmute_copy(&lppt))
        }
        unsafe extern "system" fn TxClientToScreen<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxClientToScreen(this, core::mem::transmute_copy(&lppt))
        }
        unsafe extern "system" fn TxActivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploldstate: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxActivate(this, core::mem::transmute_copy(&ploldstate)).into()
        }
        unsafe extern "system" fn TxDeactivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewstate: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxDeactivate(this, core::mem::transmute_copy(&lnewstate)).into()
        }
        unsafe extern "system" fn TxGetClientRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetClientRect(this, core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxGetViewInset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetViewInset(this, core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxGetCharFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetCharFormat(this, core::mem::transmute_copy(&ppcf)).into()
        }
        unsafe extern "system" fn TxGetParaFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppf: *const *const PARAFORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetParaFormat(this, core::mem::transmute_copy(&pppf)).into()
        }
        unsafe extern "system" fn TxGetSysColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetSysColor(this, core::mem::transmute_copy(&nindex))
        }
        unsafe extern "system" fn TxGetBackStyle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetBackStyle(this, core::mem::transmute_copy(&pstyle)).into()
        }
        unsafe extern "system" fn TxGetMaxLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetMaxLength(this, core::mem::transmute_copy(&plength)).into()
        }
        unsafe extern "system" fn TxGetScrollBars<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscrollbar: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetScrollBars(this, core::mem::transmute_copy(&pdwscrollbar)).into()
        }
        unsafe extern "system" fn TxGetPasswordChar<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pch: *mut i8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextHost_Impl::TxGetPasswordChar(this) {
                Ok(ok__) => {
                    core::ptr::write(pch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetAcceleratorPos<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcp: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetAcceleratorPos(this, core::mem::transmute_copy(&pcp)).into()
        }
        unsafe extern "system" fn TxGetExtent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetExtent(this, core::mem::transmute_copy(&lpextent)).into()
        }
        unsafe extern "system" fn OnTxCharFormatChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *const CHARFORMATW) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::OnTxCharFormatChange(this, core::mem::transmute_copy(&pcf)).into()
        }
        unsafe extern "system" fn OnTxParaFormatChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppf: *const PARAFORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::OnTxParaFormatChange(this, core::mem::transmute_copy(&ppf)).into()
        }
        unsafe extern "system" fn TxGetPropertyBits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetPropertyBits(this, core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&pdwbits)).into()
        }
        unsafe extern "system" fn TxNotify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxNotify(this, core::mem::transmute_copy(&inotify), core::mem::transmute_copy(&pv)).into()
        }
        unsafe extern "system" fn TxImmGetContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Globalization::HIMC {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxImmGetContext(this)
        }
        unsafe extern "system" fn TxImmReleaseContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: super::super::super::Globalization::HIMC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxImmReleaseContext(this, core::mem::transmute_copy(&himc))
        }
        unsafe extern "system" fn TxGetSelectionBarWidth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lselbarwidth: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost_Impl::TxGetSelectionBarWidth(this, core::mem::transmute_copy(&lselbarwidth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TxGetDC: TxGetDC::<Identity, Impl, OFFSET>,
            TxReleaseDC: TxReleaseDC::<Identity, Impl, OFFSET>,
            TxShowScrollBar: TxShowScrollBar::<Identity, Impl, OFFSET>,
            TxEnableScrollBar: TxEnableScrollBar::<Identity, Impl, OFFSET>,
            TxSetScrollRange: TxSetScrollRange::<Identity, Impl, OFFSET>,
            TxSetScrollPos: TxSetScrollPos::<Identity, Impl, OFFSET>,
            TxInvalidateRect: TxInvalidateRect::<Identity, Impl, OFFSET>,
            TxViewChange: TxViewChange::<Identity, Impl, OFFSET>,
            TxCreateCaret: TxCreateCaret::<Identity, Impl, OFFSET>,
            TxShowCaret: TxShowCaret::<Identity, Impl, OFFSET>,
            TxSetCaretPos: TxSetCaretPos::<Identity, Impl, OFFSET>,
            TxSetTimer: TxSetTimer::<Identity, Impl, OFFSET>,
            TxKillTimer: TxKillTimer::<Identity, Impl, OFFSET>,
            TxScrollWindowEx: TxScrollWindowEx::<Identity, Impl, OFFSET>,
            TxSetCapture: TxSetCapture::<Identity, Impl, OFFSET>,
            TxSetFocus: TxSetFocus::<Identity, Impl, OFFSET>,
            TxSetCursor: TxSetCursor::<Identity, Impl, OFFSET>,
            TxScreenToClient: TxScreenToClient::<Identity, Impl, OFFSET>,
            TxClientToScreen: TxClientToScreen::<Identity, Impl, OFFSET>,
            TxActivate: TxActivate::<Identity, Impl, OFFSET>,
            TxDeactivate: TxDeactivate::<Identity, Impl, OFFSET>,
            TxGetClientRect: TxGetClientRect::<Identity, Impl, OFFSET>,
            TxGetViewInset: TxGetViewInset::<Identity, Impl, OFFSET>,
            TxGetCharFormat: TxGetCharFormat::<Identity, Impl, OFFSET>,
            TxGetParaFormat: TxGetParaFormat::<Identity, Impl, OFFSET>,
            TxGetSysColor: TxGetSysColor::<Identity, Impl, OFFSET>,
            TxGetBackStyle: TxGetBackStyle::<Identity, Impl, OFFSET>,
            TxGetMaxLength: TxGetMaxLength::<Identity, Impl, OFFSET>,
            TxGetScrollBars: TxGetScrollBars::<Identity, Impl, OFFSET>,
            TxGetPasswordChar: TxGetPasswordChar::<Identity, Impl, OFFSET>,
            TxGetAcceleratorPos: TxGetAcceleratorPos::<Identity, Impl, OFFSET>,
            TxGetExtent: TxGetExtent::<Identity, Impl, OFFSET>,
            OnTxCharFormatChange: OnTxCharFormatChange::<Identity, Impl, OFFSET>,
            OnTxParaFormatChange: OnTxParaFormatChange::<Identity, Impl, OFFSET>,
            TxGetPropertyBits: TxGetPropertyBits::<Identity, Impl, OFFSET>,
            TxNotify: TxNotify::<Identity, Impl, OFFSET>,
            TxImmGetContext: TxImmGetContext::<Identity, Impl, OFFSET>,
            TxImmReleaseContext: TxImmReleaseContext::<Identity, Impl, OFFSET>,
            TxGetSelectionBarWidth: TxGetSelectionBarWidth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost2_Impl: Sized + ITextHost_Impl {
    fn TxIsDoubleClickPending(&self) -> super::super::super::Foundation::BOOL;
    fn TxGetWindow(&self, phwnd: *mut super::super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn TxSetForegroundWindow(&self) -> windows_core::Result<()>;
    fn TxGetPalette(&self) -> super::super::super::Graphics::Gdi::HPALETTE;
    fn TxGetEastAsianFlags(&self, pflags: *mut i32) -> windows_core::Result<()>;
    fn TxSetCursor2(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR;
    fn TxFreeTextServicesNotification(&self);
    fn TxGetEditStyle(&self, dwitem: u32, pdwdata: *mut u32) -> windows_core::Result<()>;
    fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::Result<()>;
    fn TxShowDropCaret(&self, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn TxDestroyCaret(&self) -> windows_core::Result<()>;
    fn TxGetHorzExtent(&self, plhorzextent: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for ITextHost2 {}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>() -> ITextHost2_Vtbl {
        unsafe extern "system" fn TxIsDoubleClickPending<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxIsDoubleClickPending(this)
        }
        unsafe extern "system" fn TxGetWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxGetWindow(this, core::mem::transmute_copy(&phwnd)).into()
        }
        unsafe extern "system" fn TxSetForegroundWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxSetForegroundWindow(this).into()
        }
        unsafe extern "system" fn TxGetPalette<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxGetPalette(this)
        }
        unsafe extern "system" fn TxGetEastAsianFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxGetEastAsianFlags(this, core::mem::transmute_copy(&pflags)).into()
        }
        unsafe extern "system" fn TxSetCursor2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxSetCursor2(this, core::mem::transmute_copy(&hcur), core::mem::transmute_copy(&btext))
        }
        unsafe extern "system" fn TxFreeTextServicesNotification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxFreeTextServicesNotification(this)
        }
        unsafe extern "system" fn TxGetEditStyle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxGetEditStyle(this, core::mem::transmute_copy(&dwitem), core::mem::transmute_copy(&pdwdata)).into()
        }
        unsafe extern "system" fn TxGetWindowStyles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxGetWindowStyles(this, core::mem::transmute_copy(&pdwstyle), core::mem::transmute_copy(&pdwexstyle)).into()
        }
        unsafe extern "system" fn TxShowDropCaret<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxShowDropCaret(this, core::mem::transmute_copy(&fshow), core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxDestroyCaret<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxDestroyCaret(this).into()
        }
        unsafe extern "system" fn TxGetHorzExtent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhorzextent: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextHost2_Impl::TxGetHorzExtent(this, core::mem::transmute_copy(&plhorzextent)).into()
        }
        Self {
            base__: ITextHost_Vtbl::new::<Identity, Impl, OFFSET>(),
            TxIsDoubleClickPending: TxIsDoubleClickPending::<Identity, Impl, OFFSET>,
            TxGetWindow: TxGetWindow::<Identity, Impl, OFFSET>,
            TxSetForegroundWindow: TxSetForegroundWindow::<Identity, Impl, OFFSET>,
            TxGetPalette: TxGetPalette::<Identity, Impl, OFFSET>,
            TxGetEastAsianFlags: TxGetEastAsianFlags::<Identity, Impl, OFFSET>,
            TxSetCursor2: TxSetCursor2::<Identity, Impl, OFFSET>,
            TxFreeTextServicesNotification: TxFreeTextServicesNotification::<Identity, Impl, OFFSET>,
            TxGetEditStyle: TxGetEditStyle::<Identity, Impl, OFFSET>,
            TxGetWindowStyles: TxGetWindowStyles::<Identity, Impl, OFFSET>,
            TxShowDropCaret: TxShowDropCaret::<Identity, Impl, OFFSET>,
            TxDestroyCaret: TxDestroyCaret::<Identity, Impl, OFFSET>,
            TxGetHorzExtent: TxGetHorzExtent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHost2 as windows_core::Interface>::IID || iid == &<ITextHost as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextPara_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetDuplicate(&self) -> windows_core::Result<ITextPara>;
    fn SetDuplicate(&self, ppara: Option<&ITextPara>) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn IsEqual(&self, ppara: Option<&ITextPara>) -> windows_core::Result<i32>;
    fn Reset(&self, value: i32) -> windows_core::Result<()>;
    fn GetStyle(&self) -> windows_core::Result<i32>;
    fn SetStyle(&self, value: i32) -> windows_core::Result<()>;
    fn GetAlignment(&self) -> windows_core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetHyphenation(&self) -> windows_core::Result<tomConstants>;
    fn SetHyphenation(&self, value: i32) -> windows_core::Result<()>;
    fn GetFirstLineIndent(&self) -> windows_core::Result<f32>;
    fn GetKeepTogether(&self) -> windows_core::Result<tomConstants>;
    fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepWithNext(&self) -> windows_core::Result<tomConstants>;
    fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()>;
    fn GetLeftIndent(&self) -> windows_core::Result<f32>;
    fn GetLineSpacing(&self) -> windows_core::Result<f32>;
    fn GetLineSpacingRule(&self) -> windows_core::Result<i32>;
    fn GetListAlignment(&self) -> windows_core::Result<i32>;
    fn SetListAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetListLevelIndex(&self) -> windows_core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()>;
    fn GetListStart(&self) -> windows_core::Result<i32>;
    fn SetListStart(&self, value: i32) -> windows_core::Result<()>;
    fn GetListTab(&self) -> windows_core::Result<f32>;
    fn SetListTab(&self, value: f32) -> windows_core::Result<()>;
    fn GetListType(&self) -> windows_core::Result<i32>;
    fn SetListType(&self, value: i32) -> windows_core::Result<()>;
    fn GetNoLineNumber(&self) -> windows_core::Result<i32>;
    fn SetNoLineNumber(&self, value: i32) -> windows_core::Result<()>;
    fn GetPageBreakBefore(&self) -> windows_core::Result<i32>;
    fn SetPageBreakBefore(&self, value: i32) -> windows_core::Result<()>;
    fn GetRightIndent(&self) -> windows_core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> windows_core::Result<()>;
    fn SetIndents(&self, first: f32, left: f32, right: f32) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, rule: i32, spacing: f32) -> windows_core::Result<()>;
    fn GetSpaceAfter(&self) -> windows_core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()>;
    fn GetSpaceBefore(&self) -> windows_core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()>;
    fn GetWidowControl(&self) -> windows_core::Result<i32>;
    fn SetWidowControl(&self, value: i32) -> windows_core::Result<()>;
    fn GetTabCount(&self) -> windows_core::Result<i32>;
    fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::Result<()>;
    fn ClearAllTabs(&self) -> windows_core::Result<()>;
    fn DeleteTab(&self, tbpos: f32) -> windows_core::Result<()>;
    fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextPara {}
#[cfg(feature = "Win32_System_Com")]
impl ITextPara_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>() -> ITextPara_Vtbl {
        unsafe extern "system" fn GetDuplicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetDuplicate(this) {
                Ok(ok__) => {
                    core::ptr::write(pppara, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetDuplicate(this, windows_core::from_raw_borrowed(&ppara)).into()
        }
        unsafe extern "system" fn CanChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::CanChange(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::IsEqual(this, windows_core::from_raw_borrowed(&ppara)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStyle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetStyle(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetStyle(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetAlignment(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetAlignment(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHyphenation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetHyphenation(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyphenation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetHyphenation(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFirstLineIndent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetFirstLineIndent(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeepTogether<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetKeepTogether(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetKeepTogether(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetKeepWithNext(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetKeepWithNext(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLeftIndent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetLeftIndent(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetLineSpacing(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacingRule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetLineSpacingRule(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetListAlignment(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetListAlignment(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListLevelIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetListLevelIndex(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetListLevelIndex(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetListStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetListStart(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListTab<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetListTab(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetListTab(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetListType(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetListType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNoLineNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetNoLineNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetNoLineNumber(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPageBreakBefore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetPageBreakBefore(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetPageBreakBefore(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetRightIndent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetRightIndent(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetRightIndent(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetIndents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, first: f32, left: f32, right: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetIndents(this, core::mem::transmute_copy(&first), core::mem::transmute_copy(&left), core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: i32, spacing: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetLineSpacing(this, core::mem::transmute_copy(&rule), core::mem::transmute_copy(&spacing)).into()
        }
        unsafe extern "system" fn GetSpaceAfter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetSpaceAfter(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetSpaceAfter(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpaceBefore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetSpaceBefore(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetSpaceBefore(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetWidowControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetWidowControl(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::SetWidowControl(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTabCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara_Impl::GetTabCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::AddTab(this, core::mem::transmute_copy(&tbpos), core::mem::transmute_copy(&tbalign), core::mem::transmute_copy(&tbleader)).into()
        }
        unsafe extern "system" fn ClearAllTabs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::ClearAllTabs(this).into()
        }
        unsafe extern "system" fn DeleteTab<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tbpos: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::DeleteTab(this, core::mem::transmute_copy(&tbpos)).into()
        }
        unsafe extern "system" fn GetTab<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara_Impl::GetTab(this, core::mem::transmute_copy(&itab), core::mem::transmute_copy(&ptbpos), core::mem::transmute_copy(&ptbalign), core::mem::transmute_copy(&ptbleader)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, Impl, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, Impl, OFFSET>,
            CanChange: CanChange::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            SetStyle: SetStyle::<Identity, Impl, OFFSET>,
            GetAlignment: GetAlignment::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            GetHyphenation: GetHyphenation::<Identity, Impl, OFFSET>,
            SetHyphenation: SetHyphenation::<Identity, Impl, OFFSET>,
            GetFirstLineIndent: GetFirstLineIndent::<Identity, Impl, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, Impl, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, Impl, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, Impl, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, Impl, OFFSET>,
            GetLeftIndent: GetLeftIndent::<Identity, Impl, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, Impl, OFFSET>,
            GetLineSpacingRule: GetLineSpacingRule::<Identity, Impl, OFFSET>,
            GetListAlignment: GetListAlignment::<Identity, Impl, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, Impl, OFFSET>,
            GetListLevelIndex: GetListLevelIndex::<Identity, Impl, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, Impl, OFFSET>,
            GetListStart: GetListStart::<Identity, Impl, OFFSET>,
            SetListStart: SetListStart::<Identity, Impl, OFFSET>,
            GetListTab: GetListTab::<Identity, Impl, OFFSET>,
            SetListTab: SetListTab::<Identity, Impl, OFFSET>,
            GetListType: GetListType::<Identity, Impl, OFFSET>,
            SetListType: SetListType::<Identity, Impl, OFFSET>,
            GetNoLineNumber: GetNoLineNumber::<Identity, Impl, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, Impl, OFFSET>,
            GetPageBreakBefore: GetPageBreakBefore::<Identity, Impl, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, Impl, OFFSET>,
            GetRightIndent: GetRightIndent::<Identity, Impl, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, Impl, OFFSET>,
            SetIndents: SetIndents::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
            GetSpaceAfter: GetSpaceAfter::<Identity, Impl, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, Impl, OFFSET>,
            GetSpaceBefore: GetSpaceBefore::<Identity, Impl, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, Impl, OFFSET>,
            GetWidowControl: GetWidowControl::<Identity, Impl, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, Impl, OFFSET>,
            GetTabCount: GetTabCount::<Identity, Impl, OFFSET>,
            AddTab: AddTab::<Identity, Impl, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, Impl, OFFSET>,
            DeleteTab: DeleteTab::<Identity, Impl, OFFSET>,
            GetTab: GetTab::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextPara as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextPara2_Impl: Sized + ITextPara_Impl {
    fn GetBorders(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextPara2>;
    fn SetDuplicate2(&self, ppara: Option<&ITextPara2>) -> windows_core::Result<()>;
    fn GetFontAlignment(&self) -> windows_core::Result<i32>;
    fn SetFontAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetHangingPunctuation(&self) -> windows_core::Result<i32>;
    fn SetHangingPunctuation(&self, value: i32) -> windows_core::Result<()>;
    fn GetSnapToGrid(&self) -> windows_core::Result<i32>;
    fn SetSnapToGrid(&self, value: i32) -> windows_core::Result<()>;
    fn GetTrimPunctuationAtStart(&self) -> windows_core::Result<i32>;
    fn SetTrimPunctuationAtStart(&self, value: i32) -> windows_core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn IsEqual2(&self, ppara: Option<&ITextPara2>) -> windows_core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextPara2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextPara2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>() -> ITextPara2_Vtbl {
        unsafe extern "system" fn GetBorders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppborders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::GetBorders(this) {
                Ok(ok__) => {
                    core::ptr::write(ppborders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::GetDuplicate2(this) {
                Ok(ok__) => {
                    core::ptr::write(pppara, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::SetDuplicate2(this, windows_core::from_raw_borrowed(&ppara)).into()
        }
        unsafe extern "system" fn GetFontAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::GetFontAlignment(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::SetFontAlignment(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHangingPunctuation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::GetHangingPunctuation(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHangingPunctuation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::SetHangingPunctuation(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSnapToGrid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::GetSnapToGrid(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapToGrid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::SetSnapToGrid(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTrimPunctuationAtStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::GetTrimPunctuationAtStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimPunctuationAtStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::SetTrimPunctuationAtStart(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEffects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::GetEffects(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextPara2_Impl::IsEqual2(this, windows_core::from_raw_borrowed(&ppara)) {
                Ok(ok__) => {
                    core::ptr::write(pb, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::SetEffects(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextPara2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ITextPara_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBorders: GetBorders::<Identity, Impl, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, Impl, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, Impl, OFFSET>,
            GetFontAlignment: GetFontAlignment::<Identity, Impl, OFFSET>,
            SetFontAlignment: SetFontAlignment::<Identity, Impl, OFFSET>,
            GetHangingPunctuation: GetHangingPunctuation::<Identity, Impl, OFFSET>,
            SetHangingPunctuation: SetHangingPunctuation::<Identity, Impl, OFFSET>,
            GetSnapToGrid: GetSnapToGrid::<Identity, Impl, OFFSET>,
            SetSnapToGrid: SetSnapToGrid::<Identity, Impl, OFFSET>,
            GetTrimPunctuationAtStart: GetTrimPunctuationAtStart::<Identity, Impl, OFFSET>,
            SetTrimPunctuationAtStart: SetTrimPunctuationAtStart::<Identity, Impl, OFFSET>,
            GetEffects: GetEffects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            IsEqual2: IsEqual2::<Identity, Impl, OFFSET>,
            SetEffects: SetEffects::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextPara2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextPara as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRange_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetChar(&self) -> windows_core::Result<i32>;
    fn SetChar(&self, char: i32) -> windows_core::Result<()>;
    fn GetDuplicate(&self) -> windows_core::Result<ITextRange>;
    fn GetFormattedText(&self) -> windows_core::Result<ITextRange>;
    fn SetFormattedText(&self, prange: Option<&ITextRange>) -> windows_core::Result<()>;
    fn GetStart(&self) -> windows_core::Result<i32>;
    fn SetStart(&self, cpfirst: i32) -> windows_core::Result<()>;
    fn GetEnd(&self) -> windows_core::Result<i32>;
    fn SetEnd(&self, cplim: i32) -> windows_core::Result<()>;
    fn GetFont(&self) -> windows_core::Result<ITextFont>;
    fn SetFont(&self, pfont: Option<&ITextFont>) -> windows_core::Result<()>;
    fn GetPara(&self) -> windows_core::Result<ITextPara>;
    fn SetPara(&self, ppara: Option<&ITextPara>) -> windows_core::Result<()>;
    fn GetStoryLength(&self) -> windows_core::Result<i32>;
    fn GetStoryType(&self) -> windows_core::Result<i32>;
    fn Collapse(&self, bstart: i32) -> windows_core::Result<()>;
    fn Expand(&self, unit: i32) -> windows_core::Result<i32>;
    fn GetIndex(&self, unit: i32) -> windows_core::Result<i32>;
    fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> windows_core::Result<()>;
    fn SetRange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()>;
    fn InRange(&self, prange: Option<&ITextRange>) -> windows_core::Result<i32>;
    fn InStory(&self, prange: Option<&ITextRange>) -> windows_core::Result<i32>;
    fn IsEqual(&self, prange: Option<&ITextRange>) -> windows_core::Result<i32>;
    fn Select(&self) -> windows_core::Result<()>;
    fn StartOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn EndOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn Move(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveStart(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveEnd(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveWhile(&self, cset: *const windows_core::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveStartWhile(&self, cset: *const windows_core::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveEndWhile(&self, cset: *const windows_core::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveUntil(&self, cset: *const windows_core::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveStartUntil(&self, cset: *const windows_core::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveEndUntil(&self, cset: *const windows_core::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn FindText(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32>;
    fn FindTextStart(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32>;
    fn FindTextEnd(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32>;
    fn Delete(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn Cut(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Copy(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Paste(&self, pvar: *const windows_core::VARIANT, format: i32) -> windows_core::Result<()>;
    fn CanPaste(&self, pvar: *const windows_core::VARIANT, format: i32) -> windows_core::Result<i32>;
    fn CanEdit(&self) -> windows_core::Result<i32>;
    fn ChangeCase(&self, r#type: tomConstants) -> windows_core::Result<()>;
    fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> windows_core::Result<()>;
    fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, value: i32) -> windows_core::Result<()>;
    fn GetEmbeddedObject(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextRange {}
#[cfg(feature = "Win32_System_Com")]
impl ITextRange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>() -> ITextRange_Vtbl {
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetText(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetText(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn GetChar<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchar: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetChar(this) {
                Ok(ok__) => {
                    core::ptr::write(pchar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChar<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, char: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetChar(this, core::mem::transmute_copy(&char)).into()
        }
        unsafe extern "system" fn GetDuplicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetDuplicate(this) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetFormattedText(this) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetFormattedText(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn GetStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpfirst: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pcpfirst, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpfirst: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetStart(this, core::mem::transmute_copy(&cpfirst)).into()
        }
        unsafe extern "system" fn GetEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcplim: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pcplim, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cplim: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetEnd(this, core::mem::transmute_copy(&cplim)).into()
        }
        unsafe extern "system" fn GetFont<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetFont(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfont, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetFont(this, windows_core::from_raw_borrowed(&pfont)).into()
        }
        unsafe extern "system" fn GetPara<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetPara(this) {
                Ok(ok__) => {
                    core::ptr::write(pppara, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetPara(this, windows_core::from_raw_borrowed(&ppara)).into()
        }
        unsafe extern "system" fn GetStoryLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetStoryLength(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetStoryType(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstart: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::Collapse(this, core::mem::transmute_copy(&bstart)).into()
        }
        unsafe extern "system" fn Expand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::Expand(this, core::mem::transmute_copy(&unit)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, pindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetIndex(this, core::mem::transmute_copy(&unit)) {
                Ok(ok__) => {
                    core::ptr::write(pindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, index: i32, extend: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetIndex(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&index), core::mem::transmute_copy(&extend)).into()
        }
        unsafe extern "system" fn SetRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetRange(this, core::mem::transmute_copy(&cpanchor), core::mem::transmute_copy(&cpactive)).into()
        }
        unsafe extern "system" fn InRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::InRange(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InStory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::InStory(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::IsEqual(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::Select(this).into()
        }
        unsafe extern "system" fn StartOf<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::StartOf(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::EndOf(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::Move(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveStart(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveEnd(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveWhile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const core::mem::MaybeUninit<windows_core::VARIANT>, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartWhile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const core::mem::MaybeUninit<windows_core::VARIANT>, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveStartWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndWhile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const core::mem::MaybeUninit<windows_core::VARIANT>, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveEndWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUntil<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const core::mem::MaybeUninit<windows_core::VARIANT>, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartUntil<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const core::mem::MaybeUninit<windows_core::VARIANT>, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveStartUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndUntil<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const core::mem::MaybeUninit<windows_core::VARIANT>, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::MoveEndUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>, count: i32, flags: tomConstants, plength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::FindText(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(plength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>, count: i32, flags: tomConstants, plength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::FindTextStart(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(plength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>, count: i32, flags: tomConstants, plength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::FindTextEnd(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(plength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::Delete(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cut<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::Cut(this) {
                Ok(ok__) => {
                    core::ptr::write(pvar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::Copy(this) {
                Ok(ok__) => {
                    core::ptr::write(pvar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>, format: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::Paste(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn CanPaste<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>, format: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::CanPaste(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanEdit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::CanEdit(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::ChangeCase(this, core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn GetPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: tomConstants, px: *mut i32, py: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::GetPoint(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&px), core::mem::transmute_copy(&py)).into()
        }
        unsafe extern "system" fn SetPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, r#type: tomConstants, extend: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::SetPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&extend)).into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange_Impl::ScrollIntoView(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEmbeddedObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange_Impl::GetEmbeddedObject(this) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetChar: GetChar::<Identity, Impl, OFFSET>,
            SetChar: SetChar::<Identity, Impl, OFFSET>,
            GetDuplicate: GetDuplicate::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            GetStart: GetStart::<Identity, Impl, OFFSET>,
            SetStart: SetStart::<Identity, Impl, OFFSET>,
            GetEnd: GetEnd::<Identity, Impl, OFFSET>,
            SetEnd: SetEnd::<Identity, Impl, OFFSET>,
            GetFont: GetFont::<Identity, Impl, OFFSET>,
            SetFont: SetFont::<Identity, Impl, OFFSET>,
            GetPara: GetPara::<Identity, Impl, OFFSET>,
            SetPara: SetPara::<Identity, Impl, OFFSET>,
            GetStoryLength: GetStoryLength::<Identity, Impl, OFFSET>,
            GetStoryType: GetStoryType::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
            InRange: InRange::<Identity, Impl, OFFSET>,
            InStory: InStory::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            StartOf: StartOf::<Identity, Impl, OFFSET>,
            EndOf: EndOf::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveStart: MoveStart::<Identity, Impl, OFFSET>,
            MoveEnd: MoveEnd::<Identity, Impl, OFFSET>,
            MoveWhile: MoveWhile::<Identity, Impl, OFFSET>,
            MoveStartWhile: MoveStartWhile::<Identity, Impl, OFFSET>,
            MoveEndWhile: MoveEndWhile::<Identity, Impl, OFFSET>,
            MoveUntil: MoveUntil::<Identity, Impl, OFFSET>,
            MoveStartUntil: MoveStartUntil::<Identity, Impl, OFFSET>,
            MoveEndUntil: MoveEndUntil::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            FindTextStart: FindTextStart::<Identity, Impl, OFFSET>,
            FindTextEnd: FindTextEnd::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cut: Cut::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            CanPaste: CanPaste::<Identity, Impl, OFFSET>,
            CanEdit: CanEdit::<Identity, Impl, OFFSET>,
            ChangeCase: ChangeCase::<Identity, Impl, OFFSET>,
            GetPoint: GetPoint::<Identity, Impl, OFFSET>,
            SetPoint: SetPoint::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetEmbeddedObject: GetEmbeddedObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRange2_Impl: Sized + ITextSelection_Impl {
    fn GetCch(&self) -> windows_core::Result<i32>;
    fn GetCells(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetColumn(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextRange2>;
    fn GetFont2(&self) -> windows_core::Result<ITextFont2>;
    fn SetFont2(&self, pfont: Option<&ITextFont2>) -> windows_core::Result<()>;
    fn GetFormattedText2(&self) -> windows_core::Result<ITextRange2>;
    fn SetFormattedText2(&self, prange: Option<&ITextRange2>) -> windows_core::Result<()>;
    fn GetGravity(&self) -> windows_core::Result<i32>;
    fn SetGravity(&self, value: i32) -> windows_core::Result<()>;
    fn GetPara2(&self) -> windows_core::Result<ITextPara2>;
    fn SetPara2(&self, ppara: Option<&ITextPara2>) -> windows_core::Result<()>;
    fn GetRow(&self) -> windows_core::Result<ITextRow>;
    fn GetStartPara(&self) -> windows_core::Result<i32>;
    fn GetTable(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetURL(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> windows_core::Result<()>;
    fn BuildUpMath(&self, flags: i32) -> windows_core::Result<()>;
    fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> windows_core::Result<()>;
    fn Find(&self, prange: Option<&ITextRange2>, count: i32, flags: i32) -> windows_core::Result<i32>;
    fn GetChar2(&self, pchar: *mut i32, offset: i32) -> windows_core::Result<()>;
    fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> windows_core::Result<()>;
    fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::Result<()>;
    fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::Result<()>;
    fn GetText2(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn HexToUnicode(&self) -> windows_core::Result<()>;
    fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> windows_core::Result<()>;
    fn Linearize(&self, flags: i32) -> windows_core::Result<()>;
    fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()>;
    fn SetDropCap(&self, cline: i32, position: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetText2(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UnicodeToHex(&self) -> windows_core::Result<()>;
    fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::Result<()>;
    fn GetMathFunctionType(&self, bstr: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: &windows_core::BSTR, pstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextRange2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextRange2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>() -> ITextRange2_Vtbl {
        unsafe extern "system" fn GetCch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcch: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetCch(this) {
                Ok(ok__) => {
                    core::ptr::write(pcch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCells<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcells: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetCells(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcells, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolumn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetColumn(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcolumn, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetDuplicate2(this) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetFont2(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfont, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetFont2(this, windows_core::from_raw_borrowed(&pfont)).into()
        }
        unsafe extern "system" fn GetFormattedText2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetFormattedText2(this) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetFormattedText2(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn GetGravity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetGravity(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetGravity(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPara2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetPara2(this) {
                Ok(ok__) => {
                    core::ptr::write(pppara, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetPara2(this, windows_core::from_raw_borrowed(&ppara)).into()
        }
        unsafe extern "system" fn GetRow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetRow(this) {
                Ok(ok__) => {
                    core::ptr::write(pprow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStartPara<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetStartPara(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetTable(this) {
                Ok(ok__) => {
                    core::ptr::write(pptable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetURL(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetURL(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn AddSubrange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::AddSubrange(this, core::mem::transmute_copy(&cp1), core::mem::transmute_copy(&cp2), core::mem::transmute_copy(&activate)).into()
        }
        unsafe extern "system" fn BuildUpMath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::BuildUpMath(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn DeleteSubrange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpfirst: i32, cplim: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::DeleteSubrange(this, core::mem::transmute_copy(&cpfirst), core::mem::transmute_copy(&cplim)).into()
        }
        unsafe extern "system" fn Find<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, count: i32, flags: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::Find(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchar: *mut i32, offset: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::GetChar2(this, core::mem::transmute_copy(&pchar), core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetDropCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::GetDropCap(this, core::mem::transmute_copy(&pcline), core::mem::transmute_copy(&pposition)).into()
        }
        unsafe extern "system" fn GetInlineObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::GetInlineObject(this, core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&palign), core::mem::transmute_copy(&pchar), core::mem::transmute_copy(&pchar1), core::mem::transmute_copy(&pchar2), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&ptexstyle), core::mem::transmute_copy(&pccol), core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::GetRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom), core::mem::transmute_copy(&phit)).into()
        }
        unsafe extern "system" fn GetSubrange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::GetSubrange(this, core::mem::transmute_copy(&isubrange), core::mem::transmute_copy(&pcpfirst), core::mem::transmute_copy(&pcplim)).into()
        }
        unsafe extern "system" fn GetText2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetText2(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(pbstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HexToUnicode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::HexToUnicode(this).into()
        }
        unsafe extern "system" fn InsertTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::InsertTable(this, core::mem::transmute_copy(&ccol), core::mem::transmute_copy(&crow), core::mem::transmute_copy(&autofit)).into()
        }
        unsafe extern "system" fn Linearize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::Linearize(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetActiveSubrange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetActiveSubrange(this, core::mem::transmute_copy(&cpanchor), core::mem::transmute_copy(&cpactive)).into()
        }
        unsafe extern "system" fn SetDropCap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cline: i32, position: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetDropCap(this, core::mem::transmute_copy(&cline), core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetText2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetText2(this, core::mem::transmute_copy(&flags), core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn UnicodeToHex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::UnicodeToHex(this).into()
        }
        unsafe extern "system" fn SetInlineObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::SetInlineObject(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&align), core::mem::transmute_copy(&char), core::mem::transmute_copy(&char1), core::mem::transmute_copy(&char2), core::mem::transmute_copy(&count), core::mem::transmute_copy(&texstyle), core::mem::transmute_copy(&ccol)).into()
        }
        unsafe extern "system" fn GetMathFunctionType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRange2_Impl::GetMathFunctionType(this, core::mem::transmute(&bstr)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: core::mem::MaybeUninit<windows_core::BSTR>, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRange2_Impl::InsertImage(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&ascent), core::mem::transmute_copy(&r#type), core::mem::transmute(&bstralttext), windows_core::from_raw_borrowed(&pstream)).into()
        }
        Self {
            base__: ITextSelection_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCch: GetCch::<Identity, Impl, OFFSET>,
            GetCells: GetCells::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, Impl, OFFSET>,
            GetFont2: GetFont2::<Identity, Impl, OFFSET>,
            SetFont2: SetFont2::<Identity, Impl, OFFSET>,
            GetFormattedText2: GetFormattedText2::<Identity, Impl, OFFSET>,
            SetFormattedText2: SetFormattedText2::<Identity, Impl, OFFSET>,
            GetGravity: GetGravity::<Identity, Impl, OFFSET>,
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            GetPara2: GetPara2::<Identity, Impl, OFFSET>,
            SetPara2: SetPara2::<Identity, Impl, OFFSET>,
            GetRow: GetRow::<Identity, Impl, OFFSET>,
            GetStartPara: GetStartPara::<Identity, Impl, OFFSET>,
            GetTable: GetTable::<Identity, Impl, OFFSET>,
            GetURL: GetURL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            AddSubrange: AddSubrange::<Identity, Impl, OFFSET>,
            BuildUpMath: BuildUpMath::<Identity, Impl, OFFSET>,
            DeleteSubrange: DeleteSubrange::<Identity, Impl, OFFSET>,
            Find: Find::<Identity, Impl, OFFSET>,
            GetChar2: GetChar2::<Identity, Impl, OFFSET>,
            GetDropCap: GetDropCap::<Identity, Impl, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetSubrange: GetSubrange::<Identity, Impl, OFFSET>,
            GetText2: GetText2::<Identity, Impl, OFFSET>,
            HexToUnicode: HexToUnicode::<Identity, Impl, OFFSET>,
            InsertTable: InsertTable::<Identity, Impl, OFFSET>,
            Linearize: Linearize::<Identity, Impl, OFFSET>,
            SetActiveSubrange: SetActiveSubrange::<Identity, Impl, OFFSET>,
            SetDropCap: SetDropCap::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetText2: SetText2::<Identity, Impl, OFFSET>,
            UnicodeToHex: UnicodeToHex::<Identity, Impl, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, Impl, OFFSET>,
            GetMathFunctionType: GetMathFunctionType::<Identity, Impl, OFFSET>,
            InsertImage: InsertImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID || iid == &<ITextSelection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRow_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetAlignment(&self) -> windows_core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellCount(&self) -> windows_core::Result<i32>;
    fn SetCellCount(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellCountCache(&self) -> windows_core::Result<i32>;
    fn SetCellCountCache(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellIndex(&self) -> windows_core::Result<i32>;
    fn SetCellIndex(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellMargin(&self) -> windows_core::Result<i32>;
    fn SetCellMargin(&self, value: i32) -> windows_core::Result<()>;
    fn GetHeight(&self) -> windows_core::Result<i32>;
    fn SetHeight(&self, value: i32) -> windows_core::Result<()>;
    fn GetIndent(&self) -> windows_core::Result<i32>;
    fn SetIndent(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepTogether(&self) -> windows_core::Result<i32>;
    fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepWithNext(&self) -> windows_core::Result<i32>;
    fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()>;
    fn GetNestLevel(&self) -> windows_core::Result<i32>;
    fn GetRTL(&self) -> windows_core::Result<i32>;
    fn SetRTL(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellAlignment(&self) -> windows_core::Result<i32>;
    fn SetCellAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellColorBack(&self) -> windows_core::Result<i32>;
    fn SetCellColorBack(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellColorFore(&self) -> windows_core::Result<i32>;
    fn SetCellColorFore(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellMergeFlags(&self) -> windows_core::Result<i32>;
    fn SetCellMergeFlags(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellShading(&self) -> windows_core::Result<i32>;
    fn SetCellShading(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellVerticalText(&self) -> windows_core::Result<i32>;
    fn SetCellVerticalText(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellWidth(&self) -> windows_core::Result<i32>;
    fn SetCellWidth(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::Result<()>;
    fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::Result<()>;
    fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::Result<()>;
    fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::Result<()>;
    fn Apply(&self, crow: i32, flags: tomConstants) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn Insert(&self, crow: i32) -> windows_core::Result<()>;
    fn IsEqual(&self, prow: Option<&ITextRow>) -> windows_core::Result<i32>;
    fn Reset(&self, value: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextRow {}
#[cfg(feature = "Win32_System_Com")]
impl ITextRow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>() -> ITextRow_Vtbl {
        unsafe extern "system" fn GetAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetAlignment(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetAlignment(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellCount(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellCountCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellCountCache(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCountCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellCountCache(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellIndex(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellIndex(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellMargin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellMargin(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMargin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellMargin(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHeight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetHeight(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetHeight(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetIndent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetIndent(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetIndent(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepTogether<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetKeepTogether(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetKeepTogether(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetKeepWithNext(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetKeepWithNext(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNestLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetNestLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRTL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetRTL(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRTL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetRTL(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellAlignment(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellAlignment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellAlignment(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellColorBack<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellColorBack(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorBack<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellColorBack(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellColorFore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellColorFore(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorFore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellColorFore(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellMergeFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellMergeFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMergeFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellMergeFlags(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellShading<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellShading(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellShading<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellShading(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellVerticalText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellVerticalText(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellVerticalText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellVerticalText(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellWidth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetCellWidth(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellWidth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellWidth(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellBorderColors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::GetCellBorderColors(this, core::mem::transmute_copy(&pcrleft), core::mem::transmute_copy(&pcrtop), core::mem::transmute_copy(&pcrright), core::mem::transmute_copy(&pcrbottom)).into()
        }
        unsafe extern "system" fn GetCellBorderWidths<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::GetCellBorderWidths(this, core::mem::transmute_copy(&pduleft), core::mem::transmute_copy(&pdutop), core::mem::transmute_copy(&pduright), core::mem::transmute_copy(&pdubottom)).into()
        }
        unsafe extern "system" fn SetCellBorderColors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellBorderColors(this, core::mem::transmute_copy(&crleft), core::mem::transmute_copy(&crtop), core::mem::transmute_copy(&crright), core::mem::transmute_copy(&crbottom)).into()
        }
        unsafe extern "system" fn SetCellBorderWidths<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetCellBorderWidths(this, core::mem::transmute_copy(&duleft), core::mem::transmute_copy(&dutop), core::mem::transmute_copy(&duright), core::mem::transmute_copy(&dubottom)).into()
        }
        unsafe extern "system" fn Apply<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crow: i32, flags: tomConstants) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::Apply(this, core::mem::transmute_copy(&crow), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CanChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::CanChange(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crow: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::Insert(this, core::mem::transmute_copy(&crow)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prow: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextRow_Impl::IsEqual(this, windows_core::from_raw_borrowed(&prow)) {
                Ok(ok__) => {
                    core::ptr::write(pb, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextRow_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAlignment: GetAlignment::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            GetCellCount: GetCellCount::<Identity, Impl, OFFSET>,
            SetCellCount: SetCellCount::<Identity, Impl, OFFSET>,
            GetCellCountCache: GetCellCountCache::<Identity, Impl, OFFSET>,
            SetCellCountCache: SetCellCountCache::<Identity, Impl, OFFSET>,
            GetCellIndex: GetCellIndex::<Identity, Impl, OFFSET>,
            SetCellIndex: SetCellIndex::<Identity, Impl, OFFSET>,
            GetCellMargin: GetCellMargin::<Identity, Impl, OFFSET>,
            SetCellMargin: SetCellMargin::<Identity, Impl, OFFSET>,
            GetHeight: GetHeight::<Identity, Impl, OFFSET>,
            SetHeight: SetHeight::<Identity, Impl, OFFSET>,
            GetIndent: GetIndent::<Identity, Impl, OFFSET>,
            SetIndent: SetIndent::<Identity, Impl, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, Impl, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, Impl, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, Impl, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, Impl, OFFSET>,
            GetNestLevel: GetNestLevel::<Identity, Impl, OFFSET>,
            GetRTL: GetRTL::<Identity, Impl, OFFSET>,
            SetRTL: SetRTL::<Identity, Impl, OFFSET>,
            GetCellAlignment: GetCellAlignment::<Identity, Impl, OFFSET>,
            SetCellAlignment: SetCellAlignment::<Identity, Impl, OFFSET>,
            GetCellColorBack: GetCellColorBack::<Identity, Impl, OFFSET>,
            SetCellColorBack: SetCellColorBack::<Identity, Impl, OFFSET>,
            GetCellColorFore: GetCellColorFore::<Identity, Impl, OFFSET>,
            SetCellColorFore: SetCellColorFore::<Identity, Impl, OFFSET>,
            GetCellMergeFlags: GetCellMergeFlags::<Identity, Impl, OFFSET>,
            SetCellMergeFlags: SetCellMergeFlags::<Identity, Impl, OFFSET>,
            GetCellShading: GetCellShading::<Identity, Impl, OFFSET>,
            SetCellShading: SetCellShading::<Identity, Impl, OFFSET>,
            GetCellVerticalText: GetCellVerticalText::<Identity, Impl, OFFSET>,
            SetCellVerticalText: SetCellVerticalText::<Identity, Impl, OFFSET>,
            GetCellWidth: GetCellWidth::<Identity, Impl, OFFSET>,
            SetCellWidth: SetCellWidth::<Identity, Impl, OFFSET>,
            GetCellBorderColors: GetCellBorderColors::<Identity, Impl, OFFSET>,
            GetCellBorderWidths: GetCellBorderWidths::<Identity, Impl, OFFSET>,
            SetCellBorderColors: SetCellBorderColors::<Identity, Impl, OFFSET>,
            SetCellBorderWidths: SetCellBorderWidths::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            CanChange: CanChange::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRow as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextSelection_Impl: Sized + ITextRange_Impl {
    fn GetFlags(&self) -> windows_core::Result<i32>;
    fn SetFlags(&self, flags: i32) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<i32>;
    fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn HomeKey(&self, unit: tomConstants, extend: i32) -> windows_core::Result<i32>;
    fn EndKey(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn TypeText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextSelection {}
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>() -> ITextSelection_Vtbl {
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::GetFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextSelection_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::GetType(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::MoveLeft(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::MoveRight(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::MoveUp(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::MoveDown(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::HomeKey(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextSelection_Impl::EndKey(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                Ok(ok__) => {
                    core::ptr::write(pdelta, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextSelection_Impl::TypeText(this, core::mem::transmute(&bstr)).into()
        }
        Self {
            base__: ITextRange_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            MoveLeft: MoveLeft::<Identity, Impl, OFFSET>,
            MoveRight: MoveRight::<Identity, Impl, OFFSET>,
            MoveUp: MoveUp::<Identity, Impl, OFFSET>,
            MoveDown: MoveDown::<Identity, Impl, OFFSET>,
            HomeKey: HomeKey::<Identity, Impl, OFFSET>,
            EndKey: EndKey::<Identity, Impl, OFFSET>,
            TypeText: TypeText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextSelection2_Impl: Sized + ITextRange2_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextSelection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection2_Impl, const OFFSET: isize>() -> ITextSelection2_Vtbl {
        Self { base__: ITextRange2_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID || iid == &<ITextSelection as windows_core::Interface>::IID || iid == &<ITextRange2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices_Impl: Sized {
    fn TxSendMessage(&self, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn TxDraw(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> windows_core::Result<()>;
    fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnTxSetCursor(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> windows_core::Result<()>;
    fn TxQueryHitPoint(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::Result<()>;
    fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnTxInPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn OnTxUIActivate(&self) -> windows_core::Result<()>;
    fn OnTxUIDeactivate(&self) -> windows_core::Result<()>;
    fn TxGetText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TxSetText(&self, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn TxGetCurTargetX(&self, param0: *mut i32) -> windows_core::Result<()>;
    fn TxGetBaseLinePos(&self, param0: *mut i32) -> windows_core::Result<()>;
    fn TxGetNaturalSize(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn TxGetDropTarget(&self) -> windows_core::Result<super::super::super::System::Ole::IDropTarget>;
    fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> windows_core::Result<()>;
    fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ITextServices {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>() -> ITextServices_Vtbl {
        unsafe extern "system" fn TxSendMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxSendMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn TxDraw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxDraw(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcwbounds), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&pfncontinue), core::mem::transmute_copy(&dwcontinue), core::mem::transmute_copy(&lviewid)).into()
        }
        unsafe extern "system" fn TxGetHScroll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxGetHScroll(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plpos), core::mem::transmute_copy(&plpage), core::mem::transmute_copy(&pfenabled)).into()
        }
        unsafe extern "system" fn TxGetVScroll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxGetVScroll(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plpos), core::mem::transmute_copy(&plpage), core::mem::transmute_copy(&pfenabled)).into()
        }
        unsafe extern "system" fn OnTxSetCursor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::OnTxSetCursor(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcclient), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn TxQueryHitPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxQueryHitPoint(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcclient), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&phitresult)).into()
        }
        unsafe extern "system" fn OnTxInPlaceActivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::OnTxInPlaceActivate(this, core::mem::transmute_copy(&prcclient)).into()
        }
        unsafe extern "system" fn OnTxInPlaceDeactivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::OnTxInPlaceDeactivate(this).into()
        }
        unsafe extern "system" fn OnTxUIActivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::OnTxUIActivate(this).into()
        }
        unsafe extern "system" fn OnTxUIDeactivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::OnTxUIDeactivate(this).into()
        }
        unsafe extern "system" fn TxGetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxGetText(this, core::mem::transmute_copy(&pbstrtext)).into()
        }
        unsafe extern "system" fn TxSetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxSetText(this, core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn TxGetCurTargetX<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxGetCurTargetX(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn TxGetBaseLinePos<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxGetBaseLinePos(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn TxGetNaturalSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxGetNaturalSize(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&psizelextent), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn TxGetDropTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdroptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextServices_Impl::TxGetDropTarget(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdroptarget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxPropertyBitsChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32, dwbits: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::OnTxPropertyBitsChange(this, core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&dwbits)).into()
        }
        unsafe extern "system" fn TxGetCachedSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices_Impl::TxGetCachedSize(this, core::mem::transmute_copy(&pdwwidth), core::mem::transmute_copy(&pdwheight)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TxSendMessage: TxSendMessage::<Identity, Impl, OFFSET>,
            TxDraw: TxDraw::<Identity, Impl, OFFSET>,
            TxGetHScroll: TxGetHScroll::<Identity, Impl, OFFSET>,
            TxGetVScroll: TxGetVScroll::<Identity, Impl, OFFSET>,
            OnTxSetCursor: OnTxSetCursor::<Identity, Impl, OFFSET>,
            TxQueryHitPoint: TxQueryHitPoint::<Identity, Impl, OFFSET>,
            OnTxInPlaceActivate: OnTxInPlaceActivate::<Identity, Impl, OFFSET>,
            OnTxInPlaceDeactivate: OnTxInPlaceDeactivate::<Identity, Impl, OFFSET>,
            OnTxUIActivate: OnTxUIActivate::<Identity, Impl, OFFSET>,
            OnTxUIDeactivate: OnTxUIDeactivate::<Identity, Impl, OFFSET>,
            TxGetText: TxGetText::<Identity, Impl, OFFSET>,
            TxSetText: TxSetText::<Identity, Impl, OFFSET>,
            TxGetCurTargetX: TxGetCurTargetX::<Identity, Impl, OFFSET>,
            TxGetBaseLinePos: TxGetBaseLinePos::<Identity, Impl, OFFSET>,
            TxGetNaturalSize: TxGetNaturalSize::<Identity, Impl, OFFSET>,
            TxGetDropTarget: TxGetDropTarget::<Identity, Impl, OFFSET>,
            OnTxPropertyBitsChange: OnTxPropertyBitsChange::<Identity, Impl, OFFSET>,
            TxGetCachedSize: TxGetCachedSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices2_Impl: Sized + ITextServices_Impl {
    fn TxGetNaturalSize2(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::Result<()>;
    fn TxDrawD2D(&self, prendertarget: Option<&super::super::super::Graphics::Direct2D::ID2D1RenderTarget>, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ITextServices2 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices2_Impl, const OFFSET: isize>() -> ITextServices2_Vtbl {
        unsafe extern "system" fn TxGetNaturalSize2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices2_Impl::TxGetNaturalSize2(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&psizelextent), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight), core::mem::transmute_copy(&pascent)).into()
        }
        unsafe extern "system" fn TxDrawD2D<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextServices2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextServices2_Impl::TxDrawD2D(this, windows_core::from_raw_borrowed(&prendertarget), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&lviewid)).into()
        }
        Self {
            base__: ITextServices_Vtbl::new::<Identity, Impl, OFFSET>(),
            TxGetNaturalSize2: TxGetNaturalSize2::<Identity, Impl, OFFSET>,
            TxDrawD2D: TxDrawD2D::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextServices2 as windows_core::Interface>::IID || iid == &<ITextServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStory_Impl: Sized {
    fn GetActive(&self) -> windows_core::Result<i32>;
    fn SetActive(&self, value: i32) -> windows_core::Result<()>;
    fn GetDisplay(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetIndex(&self) -> windows_core::Result<i32>;
    fn GetType(&self) -> windows_core::Result<i32>;
    fn SetType(&self, value: i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetRange(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2>;
    fn GetText(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormattedText(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetText(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStory {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>() -> ITextStory_Vtbl {
        unsafe extern "system" fn GetActive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStory_Impl::GetActive(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStory_Impl::SetActive(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDisplay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStory_Impl::GetDisplay(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdisplay, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStory_Impl::GetIndex(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStory_Impl::GetType(this) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStory_Impl::SetType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStory_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStory_Impl::GetRange(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStory_Impl::GetText(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(pbstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStory_Impl::SetFormattedText(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStory_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStory_Impl::SetText(this, core::mem::transmute_copy(&flags), core::mem::transmute(&bstr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActive: GetActive::<Identity, Impl, OFFSET>,
            SetActive: SetActive::<Identity, Impl, OFFSET>,
            GetDisplay: GetDisplay::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoryRanges_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<ITextRange>;
    fn GetCount(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStoryRanges {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoryRanges_Impl, const OFFSET: isize>() -> ITextStoryRanges_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStoryRanges_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunkenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStoryRanges_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStoryRanges_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoryRanges as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoryRanges2_Impl: Sized + ITextStoryRanges_Impl {
    fn Item2(&self, index: i32) -> windows_core::Result<ITextRange2>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStoryRanges2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoryRanges2_Impl, const OFFSET: isize>() -> ITextStoryRanges2_Vtbl {
        unsafe extern "system" fn Item2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoryRanges2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStoryRanges2_Impl::Item2(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITextStoryRanges_Vtbl::new::<Identity, Impl, OFFSET>(), Item2: Item2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoryRanges2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextStoryRanges as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStrings_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<ITextRange2>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Append(&self, prange: Option<&ITextRange2>, istring: i32) -> windows_core::Result<()>;
    fn Cat2(&self, istring: i32) -> windows_core::Result<()>;
    fn CatTop2(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteRange(&self, prange: Option<&ITextRange2>) -> windows_core::Result<()>;
    fn EncodeFunction(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: Option<&ITextRange2>) -> windows_core::Result<()>;
    fn GetCch(&self, istring: i32) -> windows_core::Result<i32>;
    fn InsertNullStr(&self, istring: i32) -> windows_core::Result<()>;
    fn MoveBoundary(&self, istring: i32, cch: i32) -> windows_core::Result<()>;
    fn PrefixTop(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, istring: i32, cstring: i32) -> windows_core::Result<()>;
    fn SetFormattedText(&self, pranged: Option<&ITextRange2>, pranges: Option<&ITextRange2>) -> windows_core::Result<()>;
    fn SetOpCp(&self, istring: i32, cp: i32) -> windows_core::Result<()>;
    fn SuffixTop(&self, bstr: &windows_core::BSTR, prange: Option<&ITextRange2>) -> windows_core::Result<()>;
    fn Swap(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStrings {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStrings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>() -> ITextStrings_Vtbl {
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStrings_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pprange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStrings_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::Add(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::Append(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn Cat2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::Cat2(this, core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn CatTop2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::CatTop2(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn DeleteRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::DeleteRange(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn EncodeFunction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::EncodeFunction(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&align), core::mem::transmute_copy(&char), core::mem::transmute_copy(&char1), core::mem::transmute_copy(&char2), core::mem::transmute_copy(&count), core::mem::transmute_copy(&texstyle), core::mem::transmute_copy(&ccol), windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn GetCch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, pcch: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITextStrings_Impl::GetCch(this, core::mem::transmute_copy(&istring)) {
                Ok(ok__) => {
                    core::ptr::write(pcch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertNullStr<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::InsertNullStr(this, core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn MoveBoundary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cch: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::MoveBoundary(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn PrefixTop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::PrefixTop(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cstring: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::Remove(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cstring)).into()
        }
        unsafe extern "system" fn SetFormattedText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pranged: *mut core::ffi::c_void, pranges: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::SetFormattedText(this, windows_core::from_raw_borrowed(&pranged), windows_core::from_raw_borrowed(&pranges)).into()
        }
        unsafe extern "system" fn SetOpCp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cp: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::SetOpCp(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cp)).into()
        }
        unsafe extern "system" fn SuffixTop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::SuffixTop(this, core::mem::transmute(&bstr), windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn Swap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITextStrings_Impl::Swap(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Cat2: Cat2::<Identity, Impl, OFFSET>,
            CatTop2: CatTop2::<Identity, Impl, OFFSET>,
            DeleteRange: DeleteRange::<Identity, Impl, OFFSET>,
            EncodeFunction: EncodeFunction::<Identity, Impl, OFFSET>,
            GetCch: GetCch::<Identity, Impl, OFFSET>,
            InsertNullStr: InsertNullStr::<Identity, Impl, OFFSET>,
            MoveBoundary: MoveBoundary::<Identity, Impl, OFFSET>,
            PrefixTop: PrefixTop::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            SetOpCp: SetOpCp::<Identity, Impl, OFFSET>,
            SuffixTop: SuffixTop::<Identity, Impl, OFFSET>,
            Swap: Swap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStrings as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
