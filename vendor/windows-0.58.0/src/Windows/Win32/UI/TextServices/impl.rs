#[cfg(feature = "Win32_System_Com")]
pub trait IAccClientDocMgr_Impl: Sized {
    fn GetDocuments(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn LookupByPoint(&self, pt: &super::super::Foundation::POINT, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetFocused(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAccClientDocMgr {}
#[cfg(feature = "Win32_System_Com")]
impl IAccClientDocMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAccClientDocMgr_Vtbl
    where
        Identity: IAccClientDocMgr_Impl,
    {
        unsafe extern "system" fn GetDocuments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccClientDocMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::GetDocuments(this) {
                Ok(ok__) => {
                    enumunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccClientDocMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::LookupByHWND(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccClientDocMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::LookupByPoint(this, core::mem::transmute(&pt), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocused<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccClientDocMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::GetFocused(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocuments: GetDocuments::<Identity, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, OFFSET>,
            GetFocused: GetFocused::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccClientDocMgr as windows_core::Interface>::IID
    }
}
pub trait IAccDictionary_Impl: Sized {
    fn GetLocalizedString(&self, term: *const windows_core::GUID, lcid: u32, presult: *mut windows_core::BSTR, plcid: *mut u32) -> windows_core::Result<()>;
    fn GetParentTerm(&self, term: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID>;
    fn GetMnemonicString(&self, term: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn LookupMnemonicTerm(&self, bstrmnemonic: &windows_core::BSTR) -> windows_core::Result<windows_core::GUID>;
    fn ConvertValueToString(&self, term: *const windows_core::GUID, lcid: u32, varvalue: &windows_core::VARIANT, pbstrresult: *mut windows_core::BSTR, plcid: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAccDictionary {}
impl IAccDictionary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAccDictionary_Vtbl
    where
        Identity: IAccDictionary_Impl,
    {
        unsafe extern "system" fn GetLocalizedString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, lcid: u32, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>, plcid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAccDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccDictionary_Impl::GetLocalizedString(this, core::mem::transmute_copy(&term), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn GetParentTerm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, pparentterm: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAccDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccDictionary_Impl::GetParentTerm(this, core::mem::transmute_copy(&term)) {
                Ok(ok__) => {
                    pparentterm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMnemonicString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAccDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccDictionary_Impl::GetMnemonicString(this, core::mem::transmute_copy(&term)) {
                Ok(ok__) => {
                    presult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupMnemonicTerm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmnemonic: core::mem::MaybeUninit<windows_core::BSTR>, pterm: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAccDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccDictionary_Impl::LookupMnemonicTerm(this, core::mem::transmute(&bstrmnemonic)) {
                Ok(ok__) => {
                    pterm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertValueToString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, lcid: u32, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>, pbstrresult: *mut core::mem::MaybeUninit<windows_core::BSTR>, plcid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAccDictionary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccDictionary_Impl::ConvertValueToString(this, core::mem::transmute_copy(&term), core::mem::transmute_copy(&lcid), core::mem::transmute(&varvalue), core::mem::transmute_copy(&pbstrresult), core::mem::transmute_copy(&plcid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLocalizedString: GetLocalizedString::<Identity, OFFSET>,
            GetParentTerm: GetParentTerm::<Identity, OFFSET>,
            GetMnemonicString: GetMnemonicString::<Identity, OFFSET>,
            LookupMnemonicTerm: LookupMnemonicTerm::<Identity, OFFSET>,
            ConvertValueToString: ConvertValueToString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccDictionary as windows_core::Interface>::IID
    }
}
pub trait IAccServerDocMgr_Impl: Sized {
    fn NewDocument(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RevokeDocument(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnDocumentFocus(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAccServerDocMgr {}
impl IAccServerDocMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAccServerDocMgr_Vtbl
    where
        Identity: IAccServerDocMgr_Impl,
    {
        unsafe extern "system" fn NewDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccServerDocMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccServerDocMgr_Impl::NewDocument(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RevokeDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccServerDocMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccServerDocMgr_Impl::RevokeDocument(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccServerDocMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccServerDocMgr_Impl::OnDocumentFocus(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NewDocument: NewDocument::<Identity, OFFSET>,
            RevokeDocument: RevokeDocument::<Identity, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccServerDocMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccStore_Impl: Sized {
    fn Register(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Unregister(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDocuments(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn LookupByPoint(&self, pt: &super::super::Foundation::POINT, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn OnDocumentFocus(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetFocused(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAccStore {}
#[cfg(feature = "Win32_System_Com")]
impl IAccStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAccStore_Vtbl
    where
        Identity: IAccStore_Impl,
    {
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccStore_Impl::Register(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn Unregister<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccStore_Impl::Unregister(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetDocuments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::GetDocuments(this) {
                Ok(ok__) => {
                    enumunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::LookupByHWND(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::LookupByPoint(this, core::mem::transmute(&pt), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccStore_Impl::OnDocumentFocus(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetFocused<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::GetFocused(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
            GetDocuments: GetDocuments::<Identity, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, OFFSET>,
            GetFocused: GetFocused::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccStore as windows_core::Interface>::IID
    }
}
pub trait IAnchor_Impl: Sized {
    fn SetGravity(&self, gravity: TsGravity) -> windows_core::Result<()>;
    fn GetGravity(&self) -> windows_core::Result<TsGravity>;
    fn IsEqual(&self, pawith: Option<&IAnchor>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Compare(&self, pawith: Option<&IAnchor>) -> windows_core::Result<i32>;
    fn Shift(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: Option<&IAnchor>) -> windows_core::Result<()>;
    fn ShiftTo(&self, pasite: Option<&IAnchor>) -> windows_core::Result<()>;
    fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetChangeHistoryMask(&self, dwmask: u32) -> windows_core::Result<()>;
    fn GetChangeHistory(&self) -> windows_core::Result<ANCHOR_CHANGE_HISTORY_FLAGS>;
    fn ClearChangeHistory(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IAnchor>;
}
impl windows_core::RuntimeName for IAnchor {}
impl IAnchor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAnchor_Vtbl
    where
        Identity: IAnchor_Impl,
    {
        unsafe extern "system" fn SetGravity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gravity: TsGravity) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::SetGravity(this, core::mem::transmute_copy(&gravity)).into()
        }
        unsafe extern "system" fn GetGravity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgravity: *mut TsGravity) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::GetGravity(this) {
                Ok(ok__) => {
                    pgravity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::IsEqual(this, windows_core::from_raw_borrowed(&pawith)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, plresult: *mut i32) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::Compare(this, windows_core::from_raw_borrowed(&pawith)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shift<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::Shift(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), windows_core::from_raw_borrowed(&pahaltanchor)).into()
        }
        unsafe extern "system" fn ShiftTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasite: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::ShiftTo(this, windows_core::from_raw_borrowed(&pasite)).into()
        }
        unsafe extern "system" fn ShiftRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::ShiftRegion(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dir)) {
                Ok(ok__) => {
                    pfnoregion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::SetChangeHistoryMask(this, core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetChangeHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::GetChangeHistory(this) {
                Ok(ok__) => {
                    pdwhistory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearChangeHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::ClearChangeHistory(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::Clone(this) {
                Ok(ok__) => {
                    ppaclone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGravity: SetGravity::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            Shift: Shift::<Identity, OFFSET>,
            ShiftTo: ShiftTo::<Identity, OFFSET>,
            ShiftRegion: ShiftRegion::<Identity, OFFSET>,
            SetChangeHistoryMask: SetChangeHistoryMask::<Identity, OFFSET>,
            GetChangeHistory: GetChangeHistory::<Identity, OFFSET>,
            ClearChangeHistory: ClearChangeHistory::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnchor as windows_core::Interface>::IID
    }
}
pub trait IClonableWrapper_Impl: Sized {
    fn CloneNewWrapper(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClonableWrapper {}
impl IClonableWrapper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClonableWrapper_Vtbl
    where
        Identity: IClonableWrapper_Impl,
    {
        unsafe extern "system" fn CloneNewWrapper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IClonableWrapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClonableWrapper_Impl::CloneNewWrapper(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CloneNewWrapper: CloneNewWrapper::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClonableWrapper as windows_core::Interface>::IID
    }
}
pub trait ICoCreateLocally_Impl: Sized {
    fn CoCreateLocally(&self, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, punk: *mut Option<windows_core::IUnknown>, riidparam: *const windows_core::GUID, punkparam: Option<&windows_core::IUnknown>, varparam: &windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoCreateLocally {}
impl ICoCreateLocally_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoCreateLocally_Vtbl
    where
        Identity: ICoCreateLocally_Impl,
    {
        unsafe extern "system" fn CoCreateLocally<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, punk: *mut *mut core::ffi::c_void, riidparam: *const windows_core::GUID, punkparam: *mut core::ffi::c_void, varparam: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICoCreateLocally_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoCreateLocally_Impl::CoCreateLocally(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&riidparam), windows_core::from_raw_borrowed(&punkparam), core::mem::transmute(&varparam)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CoCreateLocally: CoCreateLocally::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoCreateLocally as windows_core::Interface>::IID
    }
}
pub trait ICoCreatedLocally_Impl: Sized {
    fn LocalInit(&self, punklocalobject: Option<&windows_core::IUnknown>, riidparam: *const windows_core::GUID, punkparam: Option<&windows_core::IUnknown>, varparam: &windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoCreatedLocally {}
impl ICoCreatedLocally_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoCreatedLocally_Vtbl
    where
        Identity: ICoCreatedLocally_Impl,
    {
        unsafe extern "system" fn LocalInit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punklocalobject: *mut core::ffi::c_void, riidparam: *const windows_core::GUID, punkparam: *mut core::ffi::c_void, varparam: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICoCreatedLocally_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoCreatedLocally_Impl::LocalInit(this, windows_core::from_raw_borrowed(&punklocalobject), core::mem::transmute_copy(&riidparam), windows_core::from_raw_borrowed(&punkparam), core::mem::transmute(&varparam)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LocalInit: LocalInit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoCreatedLocally as windows_core::Interface>::IID
    }
}
pub trait IDocWrap_Impl: Sized {
    fn SetDoc(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetWrappedDoc(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IDocWrap {}
impl IDocWrap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDocWrap_Vtbl
    where
        Identity: IDocWrap_Impl,
    {
        unsafe extern "system" fn SetDoc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDocWrap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDocWrap_Impl::SetDoc(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetWrappedDoc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDocWrap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDocWrap_Impl::GetWrappedDoc(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDoc: SetDoc::<Identity, OFFSET>,
            GetWrappedDoc: GetWrappedDoc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDocWrap as windows_core::Interface>::IID
    }
}
pub trait IEnumITfCompositionView_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumITfCompositionView>;
    fn Next(&self, ulcount: u32, rgcompositionview: *mut Option<ITfCompositionView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumITfCompositionView {}
impl IEnumITfCompositionView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumITfCompositionView_Vtbl
    where
        Identity: IEnumITfCompositionView_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumITfCompositionView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumITfCompositionView_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgcompositionview: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumITfCompositionView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumITfCompositionView_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgcompositionview), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumITfCompositionView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumITfCompositionView_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumITfCompositionView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumITfCompositionView_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumITfCompositionView as windows_core::Interface>::IID
    }
}
pub trait IEnumSpeechCommands_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumSpeechCommands>;
    fn Next(&self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumSpeechCommands {}
impl IEnumSpeechCommands_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSpeechCommands_Vtbl
    where
        Identity: IEnumSpeechCommands_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSpeechCommands_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSpeechCommands_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSpeechCommands_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpeechCommands_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pspcmds), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSpeechCommands_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpeechCommands_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSpeechCommands_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpeechCommands_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSpeechCommands as windows_core::Interface>::IID
    }
}
pub trait IEnumTfCandidates_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfCandidates>;
    fn Next(&self, ulcount: u32, ppcand: *mut Option<ITfCandidateString>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfCandidates {}
impl IEnumTfCandidates_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfCandidates_Vtbl
    where
        Identity: IEnumTfCandidates_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfCandidates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfCandidates_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppcand: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfCandidates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfCandidates_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppcand), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfCandidates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfCandidates_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfCandidates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfCandidates_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfCandidates as windows_core::Interface>::IID
    }
}
pub trait IEnumTfContextViews_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfContextViews>;
    fn Next(&self, ulcount: u32, rgviews: *mut Option<ITfContextView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfContextViews {}
impl IEnumTfContextViews_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfContextViews_Vtbl
    where
        Identity: IEnumTfContextViews_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfContextViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfContextViews_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgviews: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfContextViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContextViews_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgviews), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfContextViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContextViews_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfContextViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContextViews_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfContextViews as windows_core::Interface>::IID
    }
}
pub trait IEnumTfContexts_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfContexts>;
    fn Next(&self, ulcount: u32, rgcontext: *mut Option<ITfContext>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfContexts {}
impl IEnumTfContexts_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfContexts_Vtbl
    where
        Identity: IEnumTfContexts_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfContexts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfContexts_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgcontext: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfContexts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContexts_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgcontext), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfContexts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContexts_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfContexts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContexts_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfContexts as windows_core::Interface>::IID
    }
}
pub trait IEnumTfDisplayAttributeInfo_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn Next(&self, ulcount: u32, rginfo: *mut Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfDisplayAttributeInfo {}
impl IEnumTfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfDisplayAttributeInfo_Vtbl
    where
        Identity: IEnumTfDisplayAttributeInfo_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfDisplayAttributeInfo_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rginfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDisplayAttributeInfo_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rginfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDisplayAttributeInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDisplayAttributeInfo_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfDisplayAttributeInfo as windows_core::Interface>::IID
    }
}
pub trait IEnumTfDocumentMgrs_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn Next(&self, ulcount: u32, rgdocumentmgr: *mut Option<ITfDocumentMgr>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfDocumentMgrs {}
impl IEnumTfDocumentMgrs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfDocumentMgrs_Vtbl
    where
        Identity: IEnumTfDocumentMgrs_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfDocumentMgrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfDocumentMgrs_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfDocumentMgrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDocumentMgrs_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgdocumentmgr), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfDocumentMgrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDocumentMgrs_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfDocumentMgrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDocumentMgrs_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfDocumentMgrs as windows_core::Interface>::IID
    }
}
pub trait IEnumTfFunctionProviders_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn Next(&self, ulcount: u32, ppcmdobj: *mut Option<ITfFunctionProvider>, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfFunctionProviders {}
impl IEnumTfFunctionProviders_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfFunctionProviders_Vtbl
    where
        Identity: IEnumTfFunctionProviders_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfFunctionProviders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfFunctionProviders_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppcmdobj: *mut *mut core::ffi::c_void, pcfetch: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfFunctionProviders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfFunctionProviders_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppcmdobj), core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfFunctionProviders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfFunctionProviders_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfFunctionProviders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfFunctionProviders_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfFunctionProviders as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait IEnumTfInputProcessorProfiles_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for IEnumTfInputProcessorProfiles {}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl IEnumTfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfInputProcessorProfiles_Vtbl
    where
        Identity: IEnumTfInputProcessorProfiles_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfInputProcessorProfiles_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfInputProcessorProfiles_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfInputProcessorProfiles_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfInputProcessorProfiles_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
pub trait IEnumTfLangBarItems_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfLangBarItems>;
    fn Next(&self, ulcount: u32, ppitem: *mut Option<ITfLangBarItem>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfLangBarItems {}
impl IEnumTfLangBarItems_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfLangBarItems_Vtbl
    where
        Identity: IEnumTfLangBarItems_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfLangBarItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfLangBarItems_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppitem: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfLangBarItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLangBarItems_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppitem), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfLangBarItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLangBarItems_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfLangBarItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLangBarItems_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfLangBarItems as windows_core::Interface>::IID
    }
}
pub trait IEnumTfLanguageProfiles_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfLanguageProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfLanguageProfiles {}
impl IEnumTfLanguageProfiles_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfLanguageProfiles_Vtbl
    where
        Identity: IEnumTfLanguageProfiles_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfLanguageProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfLanguageProfiles_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfLanguageProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLanguageProfiles_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfLanguageProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLanguageProfiles_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfLanguageProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLanguageProfiles_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfLanguageProfiles as windows_core::Interface>::IID
    }
}
pub trait IEnumTfLatticeElements_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfLatticeElements>;
    fn Next(&self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfLatticeElements {}
impl IEnumTfLatticeElements_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfLatticeElements_Vtbl
    where
        Identity: IEnumTfLatticeElements_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfLatticeElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfLatticeElements_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfLatticeElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLatticeElements_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgselements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfLatticeElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLatticeElements_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfLatticeElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLatticeElements_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfLatticeElements as windows_core::Interface>::IID
    }
}
pub trait IEnumTfProperties_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfProperties>;
    fn Next(&self, ulcount: u32, ppprop: *mut Option<ITfProperty>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfProperties {}
impl IEnumTfProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfProperties_Vtbl
    where
        Identity: IEnumTfProperties_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfProperties_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppprop: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfProperties_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppprop), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfProperties_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfProperties_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfProperties as windows_core::Interface>::IID
    }
}
pub trait IEnumTfPropertyValue_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfPropertyValue>;
    fn Next(&self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfPropertyValue {}
impl IEnumTfPropertyValue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfPropertyValue_Vtbl
    where
        Identity: IEnumTfPropertyValue_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfPropertyValue_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfPropertyValue_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgvalues), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfPropertyValue_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfPropertyValue_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfPropertyValue as windows_core::Interface>::IID
    }
}
pub trait IEnumTfRanges_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfRanges>;
    fn Next(&self, ulcount: u32, pprange: *mut Option<ITfRange>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfRanges {}
impl IEnumTfRanges_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfRanges_Vtbl
    where
        Identity: IEnumTfRanges_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfRanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfRanges_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprange: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfRanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfRanges_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprange), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfRanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfRanges_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfRanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfRanges_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfRanges as windows_core::Interface>::IID
    }
}
pub trait IEnumTfUIElements_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumTfUIElements>;
    fn Next(&self, ulcount: u32, ppelement: *mut Option<ITfUIElement>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumTfUIElements {}
impl IEnumTfUIElements_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTfUIElements_Vtbl
    where
        Identity: IEnumTfUIElements_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfUIElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfUIElements_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppelement: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfUIElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfUIElements_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppelement), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTfUIElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfUIElements_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTfUIElements_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfUIElements_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfUIElements as windows_core::Interface>::IID
    }
}
pub trait IInternalDocWrap_Impl: Sized {
    fn NotifyRevoke(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternalDocWrap {}
impl IInternalDocWrap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternalDocWrap_Vtbl
    where
        Identity: IInternalDocWrap_Impl,
    {
        unsafe extern "system" fn NotifyRevoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternalDocWrap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternalDocWrap_Impl::NotifyRevoke(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyRevoke: NotifyRevoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternalDocWrap as windows_core::Interface>::IID
    }
}
pub trait ISpeechCommandProvider_Impl: Sized {
    fn EnumSpeechCommands(&self, langid: u16) -> windows_core::Result<IEnumSpeechCommands>;
    fn ProcessCommand(&self, pszcommand: &windows_core::PCWSTR, cch: u32, langid: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpeechCommandProvider {}
impl ISpeechCommandProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpeechCommandProvider_Vtbl
    where
        Identity: ISpeechCommandProvider_Impl,
    {
        unsafe extern "system" fn EnumSpeechCommands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpeechCommandProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechCommandProvider_Impl::EnumSpeechCommands(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcommand: windows_core::PCWSTR, cch: u32, langid: u16) -> windows_core::HRESULT
        where
            Identity: ISpeechCommandProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechCommandProvider_Impl::ProcessCommand(this, core::mem::transmute(&pszcommand), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&langid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumSpeechCommands: EnumSpeechCommands::<Identity, OFFSET>,
            ProcessCommand: ProcessCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechCommandProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreACP_Impl: Sized {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetEndACP(&self) -> windows_core::Result<i32>;
    fn GetActiveView(&self) -> windows_core::Result<u32>;
    fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStoreACP {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreACP_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreACP_Vtbl
    where
        Identity: ITextStoreACP_Impl,
    {
        unsafe extern "system" fn AdviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::UnadviseSink(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetFormattedText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), windows_core::from_raw_borrowed(&pdataobject)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetEndACP(this) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    pvcview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetWnd(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetEndACP: GetEndACP::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACP as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreACP2_Impl: Sized {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetEndACP(&self) -> windows_core::Result<i32>;
    fn GetActiveView(&self) -> windows_core::Result<u32>;
    fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStoreACP2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreACP2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreACP2_Vtbl
    where
        Identity: ITextStoreACP2_Impl,
    {
        unsafe extern "system" fn AdviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::UnadviseSink(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetFormattedText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), windows_core::from_raw_borrowed(&pdataobject)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetEndACP(this) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    pvcview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITextStoreACP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetEndACP: GetEndACP::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACP2 as windows_core::Interface>::IID
    }
}
pub trait ITextStoreACPEx_Impl: Sized {
    fn ScrollToRect(&self, acpstart: i32, acpend: i32, rc: &super::super::Foundation::RECT, dwposition: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextStoreACPEx {}
impl ITextStoreACPEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreACPEx_Vtbl
    where
        Identity: ITextStoreACPEx_Impl,
    {
        unsafe extern "system" fn ScrollToRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPEx_Impl::ScrollToRect(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute(&rc), core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollToRect: ScrollToRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreACPServices_Impl: Sized {
    fn Serialize(&self, pprop: Option<&ITfProperty>, prange: Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Unserialize(&self, pprop: Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>, ploader: Option<&ITfPersistentPropertyLoaderACP>) -> windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: Option<&ITfProperty>) -> windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStoreACPServices {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreACPServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreACPServices_Vtbl
    where
        Identity: ITextStoreACPServices_Impl,
    {
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPServices_Impl::Serialize(this, windows_core::from_raw_borrowed(&pprop), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void, ploader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPServices_Impl::Unserialize(this, windows_core::from_raw_borrowed(&pprop), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPServices_Impl::ForceLoadProperty(this, windows_core::from_raw_borrowed(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACPServices_Impl::CreateRange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Unserialize: Unserialize::<Identity, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, OFFSET>,
            CreateRange: CreateRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPServices as windows_core::Interface>::IID
    }
}
pub trait ITextStoreACPSink_Impl: Sized {
    fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextStoreACPSink {}
impl ITextStoreACPSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreACPSink_Vtbl
    where
        Identity: ITextStoreACPSink_Impl,
    {
        unsafe extern "system" fn OnTextChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnTextChange(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnSelectionChange(this).into()
        }
        unsafe extern "system" fn OnLayoutChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnLayoutChange(this, core::mem::transmute_copy(&lcode), core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnAttrsChange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnLockGranted(this, core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnStartEditTransaction(this).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnEndEditTransaction(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPSink as windows_core::Interface>::IID
    }
}
pub trait ITextStoreACPSinkEx_Impl: Sized + ITextStoreACPSink_Impl {
    fn OnDisconnect(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextStoreACPSinkEx {}
impl ITextStoreACPSinkEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreACPSinkEx_Vtbl
    where
        Identity: ITextStoreACPSinkEx_Impl,
    {
        unsafe extern "system" fn OnDisconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreACPSinkEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSinkEx_Impl::OnDisconnect(this).into()
        }
        Self { base__: ITextStoreACPSink_Vtbl::new::<Identity, OFFSET>(), OnDisconnect: OnDisconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPSinkEx as windows_core::Interface>::IID || iid == &<ITextStoreACPSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreAnchor_Impl: Sized {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, pateststart: Option<&IAnchor>, patestend: Option<&IAnchor>, cch: u32, pparesultstart: *mut Option<IAnchor>, pparesultend: *mut Option<IAnchor>) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::Result<()>;
    fn GetText(&self, dwflags: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, pchtext: windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, pchtext: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn GetFormattedText(&self, pastart: Option<&IAnchor>, paend: Option<&IAnchor>) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, dwflags: u32, papos: Option<&IAnchor>, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn InsertEmbedded(&self, dwflags: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, papos: Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, papos: Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, pastart: Option<&IAnchor>, pahalt: Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetStart(&self) -> windows_core::Result<IAnchor>;
    fn GetEnd(&self) -> windows_core::Result<IAnchor>;
    fn GetActiveView(&self) -> windows_core::Result<u32>;
    fn GetAnchorFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<IAnchor>;
    fn GetTextExt(&self, vcview: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::HWND>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: u32, ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>, ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStoreAnchor {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreAnchor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreAnchor_Vtbl
    where
        Identity: ITextStoreAnchor_Impl,
    {
        unsafe extern "system" fn AdviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::UnadviseSink(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pateststart: *mut core::ffi::c_void, patestend: *mut core::ffi::c_void, cch: u32, pparesultstart: *mut *mut core::ffi::c_void, pparesultend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::QueryInsert(this, windows_core::from_raw_borrowed(&pateststart), windows_core::from_raw_borrowed(&patestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pparesultstart), core::mem::transmute_copy(&pparesultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::GetText(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&fupdateanchor)).into()
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::SetText(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetFormattedText(this, windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, papos: *mut core::ffi::c_void, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetEmbedded(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&papos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RequestAttrsAtPosition(this, windows_core::from_raw_borrowed(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RequestAttrsTransitioningAtPosition(this, windows_core::from_raw_borrowed(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, pahalt: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::FindNextAttrTransition(this, windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&pahalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppastart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetStart(this) {
                Ok(ok__) => {
                    ppastart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetEnd(this) {
                Ok(ok__) => {
                    ppaend.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    pvcview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetAnchorFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppasite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetWnd(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: windows_core::PCWSTR, cch: u32, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&ppastart), core::mem::transmute_copy(&ppaend)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&ppastart), core::mem::transmute_copy(&ppaend)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetAnchorFromPoint: GetAnchorFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchor as windows_core::Interface>::IID
    }
}
pub trait ITextStoreAnchorEx_Impl: Sized {
    fn ScrollToRect(&self, pstart: Option<&IAnchor>, pend: Option<&IAnchor>, rc: &super::super::Foundation::RECT, dwposition: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextStoreAnchorEx {}
impl ITextStoreAnchorEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreAnchorEx_Vtbl
    where
        Identity: ITextStoreAnchorEx_Impl,
    {
        unsafe extern "system" fn ScrollToRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstart: *mut core::ffi::c_void, pend: *mut core::ffi::c_void, rc: super::super::Foundation::RECT, dwposition: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorEx_Impl::ScrollToRect(this, windows_core::from_raw_borrowed(&pstart), windows_core::from_raw_borrowed(&pend), core::mem::transmute(&rc), core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollToRect: ScrollToRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchorEx as windows_core::Interface>::IID
    }
}
pub trait ITextStoreAnchorSink_Impl: Sized {
    fn OnTextChange(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: Option<&IAnchor>, paend: Option<&IAnchor>) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextStoreAnchorSink {}
impl ITextStoreAnchorSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreAnchorSink_Vtbl
    where
        Identity: ITextStoreAnchorSink_Impl,
    {
        unsafe extern "system" fn OnTextChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnTextChange(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnSelectionChange(this).into()
        }
        unsafe extern "system" fn OnLayoutChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnLayoutChange(this, core::mem::transmute_copy(&lcode), core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnAttrsChange(this, windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnLockGranted(this, core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnStartEditTransaction(this).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreAnchorSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnEndEditTransaction(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchorSink as windows_core::Interface>::IID
    }
}
pub trait ITextStoreSinkAnchorEx_Impl: Sized + ITextStoreAnchorSink_Impl {
    fn OnDisconnect(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextStoreSinkAnchorEx {}
impl ITextStoreSinkAnchorEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextStoreSinkAnchorEx_Vtbl
    where
        Identity: ITextStoreSinkAnchorEx_Impl,
    {
        unsafe extern "system" fn OnDisconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextStoreSinkAnchorEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreSinkAnchorEx_Impl::OnDisconnect(this).into()
        }
        Self { base__: ITextStoreAnchorSink_Vtbl::new::<Identity, OFFSET>(), OnDisconnect: OnDisconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreSinkAnchorEx as windows_core::Interface>::IID || iid == &<ITextStoreAnchorSink as windows_core::Interface>::IID
    }
}
pub trait ITfActiveLanguageProfileNotifySink_Impl: Sized {
    fn OnActivated(&self, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfActiveLanguageProfileNotifySink {}
impl ITfActiveLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfActiveLanguageProfileNotifySink_Vtbl
    where
        Identity: ITfActiveLanguageProfileNotifySink_Impl,
    {
        unsafe extern "system" fn OnActivated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfActiveLanguageProfileNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfActiveLanguageProfileNotifySink_Impl::OnActivated(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&factivated)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfActiveLanguageProfileNotifySink as windows_core::Interface>::IID
    }
}
pub trait ITfCandidateList_Impl: Sized {
    fn EnumCandidates(&self) -> windows_core::Result<IEnumTfCandidates>;
    fn GetCandidate(&self, nindex: u32) -> windows_core::Result<ITfCandidateString>;
    fn GetCandidateNum(&self) -> windows_core::Result<u32>;
    fn SetResult(&self, nindex: u32, imcr: TfCandidateResult) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfCandidateList {}
impl ITfCandidateList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCandidateList_Vtbl
    where
        Identity: ITfCandidateList_Impl,
    {
        unsafe extern "system" fn EnumCandidates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCandidateList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateList_Impl::EnumCandidates(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppcand: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCandidateList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateList_Impl::GetCandidate(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    ppcand.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateNum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncnt: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateList_Impl::GetCandidateNum(this) {
                Ok(ok__) => {
                    pncnt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> windows_core::HRESULT
        where
            Identity: ITfCandidateList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateList_Impl::SetResult(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&imcr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumCandidates: EnumCandidates::<Identity, OFFSET>,
            GetCandidate: GetCandidate::<Identity, OFFSET>,
            GetCandidateNum: GetCandidateNum::<Identity, OFFSET>,
            SetResult: SetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateList as windows_core::Interface>::IID
    }
}
pub trait ITfCandidateListUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> windows_core::Result<u32>;
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> windows_core::Result<()>;
    fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> windows_core::Result<()>;
    fn GetCurrentPage(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ITfCandidateListUIElement {}
impl ITfCandidateListUIElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCandidateListUIElement_Vtbl
    where
        Identity: ITfCandidateListUIElement_Impl,
    {
        unsafe extern "system" fn GetUpdatedFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetUpdatedFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetSelection(this) {
                Ok(ok__) => {
                    puindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetString(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    pstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElement_Impl::GetPageIndex(this, core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&usize), core::mem::transmute_copy(&pupagecnt)).into()
        }
        unsafe extern "system" fn SetPageIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElement_Impl::SetPageIndex(this, core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&upagecnt)).into()
        }
        unsafe extern "system" fn GetCurrentPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupage: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetCurrentPage(this) {
                Ok(ok__) => {
                    pupage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetPageIndex: GetPageIndex::<Identity, OFFSET>,
            SetPageIndex: SetPageIndex::<Identity, OFFSET>,
            GetCurrentPage: GetCurrentPage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
pub trait ITfCandidateListUIElementBehavior_Impl: Sized + ITfCandidateListUIElement_Impl {
    fn SetSelection(&self, nindex: u32) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfCandidateListUIElementBehavior {}
impl ITfCandidateListUIElementBehavior_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCandidateListUIElementBehavior_Vtbl
    where
        Identity: ITfCandidateListUIElementBehavior_Impl,
    {
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElementBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElementBehavior_Impl::SetSelection(this, core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn Finalize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElementBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElementBehavior_Impl::Finalize(this).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCandidateListUIElementBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElementBehavior_Impl::Abort(this).into()
        }
        Self {
            base__: ITfCandidateListUIElement_Vtbl::new::<Identity, OFFSET>(),
            SetSelection: SetSelection::<Identity, OFFSET>,
            Finalize: Finalize::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElementBehavior as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID || iid == &<ITfCandidateListUIElement as windows_core::Interface>::IID
    }
}
pub trait ITfCandidateString_Impl: Sized {
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIndex(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ITfCandidateString {}
impl ITfCandidateString_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCandidateString_Vtbl
    where
        Identity: ITfCandidateString_Impl,
    {
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfCandidateString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateString_Impl::GetString(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCandidateString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateString_Impl::GetIndex(this) {
                Ok(ok__) => {
                    pnindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetString: GetString::<Identity, OFFSET>, GetIndex: GetIndex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateString as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCategoryMgr_Impl: Sized {
    fn RegisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumCategoriesInItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn EnumItemsInCategory(&self, rcatid: *const windows_core::GUID) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn FindClosestCategory(&self, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: *const *const windows_core::GUID, ulcount: u32) -> windows_core::Result<()>;
    fn RegisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn UnregisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGUIDDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::Result<()>;
    fn UnregisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGUIDDWORD(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn RegisterGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetGUID(&self, guidatom: u32) -> windows_core::Result<windows_core::GUID>;
    fn IsEqualTfGuidAtom(&self, guidatom: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfCategoryMgr {}
#[cfg(feature = "Win32_System_Com")]
impl ITfCategoryMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCategoryMgr_Vtbl
    where
        Identity: ITfCategoryMgr_Impl,
    {
        unsafe extern "system" fn RegisterCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::RegisterCategory(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn UnregisterCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::UnregisterCategory(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCategoriesInItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::EnumCategoriesInItem(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsInCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcatid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::EnumItemsInCategory(this, core::mem::transmute_copy(&rcatid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: *const *const windows_core::GUID, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::FindClosestCategory(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pcatid), core::mem::transmute_copy(&ppcatidlist), core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn RegisterGUIDDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::RegisterGUIDDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::UnregisterGUIDDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pbstrdesc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::GetGUIDDescription(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::RegisterGUIDDWORD(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::UnregisterGUIDDWORD(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDWORD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pdw: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::GetGUIDDWORD(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pdw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pguidatom: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::RegisterGUID(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pguidatom.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidatom: u32, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::GetGUID(this, core::mem::transmute_copy(&guidatom)) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidatom: u32, rguid: *const windows_core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfCategoryMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::IsEqualTfGuidAtom(this, core::mem::transmute_copy(&guidatom), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCategory: RegisterCategory::<Identity, OFFSET>,
            UnregisterCategory: UnregisterCategory::<Identity, OFFSET>,
            EnumCategoriesInItem: EnumCategoriesInItem::<Identity, OFFSET>,
            EnumItemsInCategory: EnumItemsInCategory::<Identity, OFFSET>,
            FindClosestCategory: FindClosestCategory::<Identity, OFFSET>,
            RegisterGUIDDescription: RegisterGUIDDescription::<Identity, OFFSET>,
            UnregisterGUIDDescription: UnregisterGUIDDescription::<Identity, OFFSET>,
            GetGUIDDescription: GetGUIDDescription::<Identity, OFFSET>,
            RegisterGUIDDWORD: RegisterGUIDDWORD::<Identity, OFFSET>,
            UnregisterGUIDDWORD: UnregisterGUIDDWORD::<Identity, OFFSET>,
            GetGUIDDWORD: GetGUIDDWORD::<Identity, OFFSET>,
            RegisterGUID: RegisterGUID::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            IsEqualTfGuidAtom: IsEqualTfGuidAtom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCategoryMgr as windows_core::Interface>::IID
    }
}
pub trait ITfCleanupContextDurationSink_Impl: Sized {
    fn OnStartCleanupContext(&self) -> windows_core::Result<()>;
    fn OnEndCleanupContext(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfCleanupContextDurationSink {}
impl ITfCleanupContextDurationSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCleanupContextDurationSink_Vtbl
    where
        Identity: ITfCleanupContextDurationSink_Impl,
    {
        unsafe extern "system" fn OnStartCleanupContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCleanupContextDurationSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCleanupContextDurationSink_Impl::OnStartCleanupContext(this).into()
        }
        unsafe extern "system" fn OnEndCleanupContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCleanupContextDurationSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCleanupContextDurationSink_Impl::OnEndCleanupContext(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartCleanupContext: OnStartCleanupContext::<Identity, OFFSET>,
            OnEndCleanupContext: OnEndCleanupContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextDurationSink as windows_core::Interface>::IID
    }
}
pub trait ITfCleanupContextSink_Impl: Sized {
    fn OnCleanupContext(&self, ecwrite: u32, pic: Option<&ITfContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfCleanupContextSink {}
impl ITfCleanupContextSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCleanupContextSink_Vtbl
    where
        Identity: ITfCleanupContextSink_Impl,
    {
        unsafe extern "system" fn OnCleanupContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pic: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCleanupContextSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCleanupContextSink_Impl::OnCleanupContext(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pic)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCleanupContext: OnCleanupContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextSink as windows_core::Interface>::IID
    }
}
pub trait ITfClientId_Impl: Sized {
    fn GetClientId(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ITfClientId {}
impl ITfClientId_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfClientId_Vtbl
    where
        Identity: ITfClientId_Impl,
    {
        unsafe extern "system" fn GetClientId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ptid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfClientId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfClientId_Impl::GetClientId(this, core::mem::transmute_copy(&rclsid)) {
                Ok(ok__) => {
                    ptid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClientId: GetClientId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfClientId as windows_core::Interface>::IID
    }
}
pub trait ITfCompartment_Impl: Sized {
    fn SetValue(&self, tid: u32, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetValue(&self) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for ITfCompartment {}
impl ITfCompartment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCompartment_Vtbl
    where
        Identity: ITfCompartment_Impl,
    {
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, pvarvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITfCompartment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompartment_Impl::SetValue(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITfCompartment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompartment_Impl::GetValue(this) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetValue: SetValue::<Identity, OFFSET>, GetValue: GetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartment as windows_core::Interface>::IID
    }
}
pub trait ITfCompartmentEventSink_Impl: Sized {
    fn OnChange(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfCompartmentEventSink {}
impl ITfCompartmentEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCompartmentEventSink_Vtbl
    where
        Identity: ITfCompartmentEventSink_Impl,
    {
        unsafe extern "system" fn OnChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCompartmentEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompartmentEventSink_Impl::OnChange(this, core::mem::transmute_copy(&rguid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartmentEventSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCompartmentMgr_Impl: Sized {
    fn GetCompartment(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfCompartment>;
    fn ClearCompartment(&self, tid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumCompartments(&self) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfCompartmentMgr {}
#[cfg(feature = "Win32_System_Com")]
impl ITfCompartmentMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCompartmentMgr_Vtbl
    where
        Identity: ITfCompartmentMgr_Impl,
    {
        unsafe extern "system" fn GetCompartment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCompartmentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompartmentMgr_Impl::GetCompartment(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    ppcomp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCompartment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCompartmentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompartmentMgr_Impl::ClearCompartment(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCompartments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCompartmentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompartmentMgr_Impl::EnumCompartments(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCompartment: GetCompartment::<Identity, OFFSET>,
            ClearCompartment: ClearCompartment::<Identity, OFFSET>,
            EnumCompartments: EnumCompartments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartmentMgr as windows_core::Interface>::IID
    }
}
pub trait ITfComposition_Impl: Sized {
    fn GetRange(&self) -> windows_core::Result<ITfRange>;
    fn ShiftStart(&self, ecwrite: u32, pnewstart: Option<&ITfRange>) -> windows_core::Result<()>;
    fn ShiftEnd(&self, ecwrite: u32, pnewend: Option<&ITfRange>) -> windows_core::Result<()>;
    fn EndComposition(&self, ecwrite: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfComposition {}
impl ITfComposition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfComposition_Vtbl
    where
        Identity: ITfComposition_Impl,
    {
        unsafe extern "system" fn GetRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfComposition_Impl::GetRange(this) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pnewstart: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfComposition_Impl::ShiftStart(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pnewstart)).into()
        }
        unsafe extern "system" fn ShiftEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pnewend: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfComposition_Impl::ShiftEnd(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pnewend)).into()
        }
        unsafe extern "system" fn EndComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32) -> windows_core::HRESULT
        where
            Identity: ITfComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfComposition_Impl::EndComposition(this, core::mem::transmute_copy(&ecwrite)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRange: GetRange::<Identity, OFFSET>,
            ShiftStart: ShiftStart::<Identity, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, OFFSET>,
            EndComposition: EndComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfComposition as windows_core::Interface>::IID
    }
}
pub trait ITfCompositionSink_Impl: Sized {
    fn OnCompositionTerminated(&self, ecwrite: u32, pcomposition: Option<&ITfComposition>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfCompositionSink {}
impl ITfCompositionSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCompositionSink_Vtbl
    where
        Identity: ITfCompositionSink_Impl,
    {
        unsafe extern "system" fn OnCompositionTerminated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCompositionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompositionSink_Impl::OnCompositionTerminated(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCompositionTerminated: OnCompositionTerminated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompositionSink as windows_core::Interface>::IID
    }
}
pub trait ITfCompositionView_Impl: Sized {
    fn GetOwnerClsid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetRange(&self) -> windows_core::Result<ITfRange>;
}
impl windows_core::RuntimeName for ITfCompositionView {}
impl ITfCompositionView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCompositionView_Vtbl
    where
        Identity: ITfCompositionView_Impl,
    {
        unsafe extern "system" fn GetOwnerClsid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfCompositionView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompositionView_Impl::GetOwnerClsid(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCompositionView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompositionView_Impl::GetRange(this) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerClsid: GetOwnerClsid::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompositionView as windows_core::Interface>::IID
    }
}
pub trait ITfConfigureSystemKeystrokeFeed_Impl: Sized {
    fn DisableSystemKeystrokeFeed(&self) -> windows_core::Result<()>;
    fn EnableSystemKeystrokeFeed(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfConfigureSystemKeystrokeFeed {}
impl ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfConfigureSystemKeystrokeFeed_Vtbl
    where
        Identity: ITfConfigureSystemKeystrokeFeed_Impl,
    {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfConfigureSystemKeystrokeFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfConfigureSystemKeystrokeFeed_Impl::DisableSystemKeystrokeFeed(this).into()
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfConfigureSystemKeystrokeFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfConfigureSystemKeystrokeFeed_Impl::EnableSystemKeystrokeFeed(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DisableSystemKeystrokeFeed: DisableSystemKeystrokeFeed::<Identity, OFFSET>,
            EnableSystemKeystrokeFeed: EnableSystemKeystrokeFeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfConfigureSystemKeystrokeFeed as windows_core::Interface>::IID
    }
}
pub trait ITfContext_Impl: Sized {
    fn RequestEditSession(&self, tid: u32, pes: Option<&ITfEditSession>, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> windows_core::Result<windows_core::HRESULT>;
    fn InWriteSession(&self, tid: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetSelection(&self, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> windows_core::Result<()>;
    fn GetStart(&self, ec: u32) -> windows_core::Result<ITfRange>;
    fn GetEnd(&self, ec: u32) -> windows_core::Result<ITfRange>;
    fn GetActiveView(&self) -> windows_core::Result<ITfContextView>;
    fn EnumViews(&self) -> windows_core::Result<IEnumTfContextViews>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn GetProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfProperty>;
    fn GetAppProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfReadOnlyProperty>;
    fn TrackProperties(&self, prgprop: *const *const windows_core::GUID, cprop: u32, prgappprop: *const *const windows_core::GUID, cappprop: u32) -> windows_core::Result<ITfReadOnlyProperty>;
    fn EnumProperties(&self) -> windows_core::Result<IEnumTfProperties>;
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn CreateRangeBackup(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<ITfRangeBackup>;
}
impl windows_core::RuntimeName for ITfContext {}
impl ITfContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContext_Vtbl
    where
        Identity: ITfContext_Impl,
    {
        unsafe extern "system" fn RequestEditSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, pes: *mut core::ffi::c_void, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::RequestEditSession(this, core::mem::transmute_copy(&tid), windows_core::from_raw_borrowed(&pes), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InWriteSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::InWriteSession(this, core::mem::transmute_copy(&tid)) {
                Ok(ok__) => {
                    pfwritesession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContext_Impl::GetSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContext_Impl::SetSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppstart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetStart(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    ppstart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetEnd(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    ppend.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    ppview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::EnumViews(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetProperty(this, core::mem::transmute_copy(&guidprop)) {
                Ok(ok__) => {
                    ppprop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetAppProperty(this, core::mem::transmute_copy(&guidprop)) {
                Ok(ok__) => {
                    ppprop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prgprop: *const *const windows_core::GUID, cprop: u32, prgappprop: *const *const windows_core::GUID, cappprop: u32, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::TrackProperties(this, core::mem::transmute_copy(&prgprop), core::mem::transmute_copy(&cprop), core::mem::transmute_copy(&prgappprop), core::mem::transmute_copy(&cappprop)) {
                Ok(ok__) => {
                    ppproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::EnumProperties(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetDocumentMgr(this) {
                Ok(ok__) => {
                    ppdm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeBackup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, ppbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::CreateRangeBackup(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppbackup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestEditSession: RequestEditSession::<Identity, OFFSET>,
            InWriteSession: InWriteSession::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            EnumViews: EnumViews::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetAppProperty: GetAppProperty::<Identity, OFFSET>,
            TrackProperties: TrackProperties::<Identity, OFFSET>,
            EnumProperties: EnumProperties::<Identity, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET>,
            CreateRangeBackup: CreateRangeBackup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContext as windows_core::Interface>::IID
    }
}
pub trait ITfContextComposition_Impl: Sized {
    fn StartComposition(&self, ecwrite: u32, pcompositionrange: Option<&ITfRange>, psink: Option<&ITfCompositionSink>) -> windows_core::Result<ITfComposition>;
    fn EnumCompositions(&self) -> windows_core::Result<IEnumITfCompositionView>;
    fn FindComposition(&self, ecread: u32, ptestrange: Option<&ITfRange>) -> windows_core::Result<IEnumITfCompositionView>;
    fn TakeOwnership(&self, ecwrite: u32, pcomposition: Option<&ITfCompositionView>, psink: Option<&ITfCompositionSink>) -> windows_core::Result<ITfComposition>;
}
impl windows_core::RuntimeName for ITfContextComposition {}
impl ITfContextComposition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContextComposition_Vtbl
    where
        Identity: ITfContextComposition_Impl,
    {
        unsafe extern "system" fn StartComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pcompositionrange: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, ppcomposition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::StartComposition(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pcompositionrange), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    ppcomposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCompositions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::EnumCompositions(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecread: u32, ptestrange: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::FindComposition(this, core::mem::transmute_copy(&ecread), windows_core::from_raw_borrowed(&ptestrange)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pcomposition: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, ppcomposition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextComposition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::TakeOwnership(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pcomposition), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    ppcomposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartComposition: StartComposition::<Identity, OFFSET>,
            EnumCompositions: EnumCompositions::<Identity, OFFSET>,
            FindComposition: FindComposition::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextComposition as windows_core::Interface>::IID
    }
}
pub trait ITfContextKeyEventSink_Impl: Sized {
    fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfContextKeyEventSink {}
impl ITfContextKeyEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContextKeyEventSink_Vtbl
    where
        Identity: ITfContextKeyEventSink_Impl,
    {
        unsafe extern "system" fn OnKeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContextKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContextKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContextKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnTestKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContextKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnTestKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextKeyEventSink as windows_core::Interface>::IID
    }
}
pub trait ITfContextOwner_Impl: Sized {
    fn GetACPFromPoint(&self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn GetWnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn GetAttribute(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for ITfContextOwner {}
impl ITfContextOwner_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContextOwner_Vtbl
    where
        Identity: ITfContextOwner_Impl,
    {
        unsafe extern "system" fn GetACPFromPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITfContextOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContextOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwner_Impl::GetTextExt(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfContextOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetScreenExt(this) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT
        where
            Identity: ITfContextOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ITfContextOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetWnd(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidattribute: *const windows_core::GUID, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITfContextOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetAttribute(this, core::mem::transmute_copy(&rguidattribute)) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
            GetAttribute: GetAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwner as windows_core::Interface>::IID
    }
}
pub trait ITfContextOwnerCompositionServices_Impl: Sized + ITfContextComposition_Impl {
    fn TerminateComposition(&self, pcomposition: Option<&ITfCompositionView>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfContextOwnerCompositionServices {}
impl ITfContextOwnerCompositionServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContextOwnerCompositionServices_Vtbl
    where
        Identity: ITfContextOwnerCompositionServices_Impl,
    {
        unsafe extern "system" fn TerminateComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerCompositionServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerCompositionServices_Impl::TerminateComposition(this, windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self { base__: ITfContextComposition_Vtbl::new::<Identity, OFFSET>(), TerminateComposition: TerminateComposition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionServices as windows_core::Interface>::IID || iid == &<ITfContextComposition as windows_core::Interface>::IID
    }
}
pub trait ITfContextOwnerCompositionSink_Impl: Sized {
    fn OnStartComposition(&self, pcomposition: Option<&ITfCompositionView>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnUpdateComposition(&self, pcomposition: Option<&ITfCompositionView>, prangenew: Option<&ITfRange>) -> windows_core::Result<()>;
    fn OnEndComposition(&self, pcomposition: Option<&ITfCompositionView>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfContextOwnerCompositionSink {}
impl ITfContextOwnerCompositionSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContextOwnerCompositionSink_Vtbl
    where
        Identity: ITfContextOwnerCompositionSink_Impl,
    {
        unsafe extern "system" fn OnStartComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void, pfok: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerCompositionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwnerCompositionSink_Impl::OnStartComposition(this, windows_core::from_raw_borrowed(&pcomposition)) {
                Ok(ok__) => {
                    pfok.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUpdateComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerCompositionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerCompositionSink_Impl::OnUpdateComposition(this, windows_core::from_raw_borrowed(&pcomposition), windows_core::from_raw_borrowed(&prangenew)).into()
        }
        unsafe extern "system" fn OnEndComposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerCompositionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerCompositionSink_Impl::OnEndComposition(this, windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartComposition: OnStartComposition::<Identity, OFFSET>,
            OnUpdateComposition: OnUpdateComposition::<Identity, OFFSET>,
            OnEndComposition: OnEndComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfContextOwnerServices_Impl: Sized {
    fn OnLayoutChange(&self) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttributeChange(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Serialize(&self, pprop: Option<&ITfProperty>, prange: Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Unserialize(&self, pprop: Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>, ploader: Option<&ITfPersistentPropertyLoaderACP>) -> windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: Option<&ITfProperty>) -> windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfContextOwnerServices {}
#[cfg(feature = "Win32_System_Com")]
impl ITfContextOwnerServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContextOwnerServices_Vtbl
    where
        Identity: ITfContextOwnerServices_Impl,
    {
        unsafe extern "system" fn OnLayoutChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::OnLayoutChange(this).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttributeChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidattribute: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::OnAttributeChange(this, core::mem::transmute_copy(&rguidattribute)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::Serialize(this, windows_core::from_raw_borrowed(&pprop), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void, ploader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::Unserialize(this, windows_core::from_raw_borrowed(&pprop), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::ForceLoadProperty(this, windows_core::from_raw_borrowed(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextOwnerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwnerServices_Impl::CreateRange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttributeChange: OnAttributeChange::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            Unserialize: Unserialize::<Identity, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, OFFSET>,
            CreateRange: CreateRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerServices as windows_core::Interface>::IID
    }
}
pub trait ITfContextView_Impl: Sized {
    fn GetRangeFromPoint(&self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<ITfRange>;
    fn GetTextExt(&self, ec: u32, prange: Option<&ITfRange>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
}
impl windows_core::RuntimeName for ITfContextView {}
impl ITfContextView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfContextView_Vtbl
    where
        Identity: ITfContextView_Impl,
    {
        unsafe extern "system" fn GetRangeFromPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfContextView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextView_Impl::GetRangeFromPoint(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ppt), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfContextView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextView_Impl::GetTextExt(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfContextView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextView_Impl::GetScreenExt(this) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ITfContextView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextView_Impl::GetWnd(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRangeFromPoint: GetRangeFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextView as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCreatePropertyStore_Impl: Sized {
    fn IsStoreSerializable(&self, guidprop: *const windows_core::GUID, prange: Option<&ITfRange>, ppropstore: Option<&ITfPropertyStore>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CreatePropertyStore(&self, guidprop: *const windows_core::GUID, prange: Option<&ITfRange>, cb: u32, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<ITfPropertyStore>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfCreatePropertyStore {}
#[cfg(feature = "Win32_System_Com")]
impl ITfCreatePropertyStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfCreatePropertyStore_Vtbl
    where
        Identity: ITfCreatePropertyStore_Impl,
    {
        unsafe extern "system" fn IsStoreSerializable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, prange: *mut core::ffi::c_void, ppropstore: *mut core::ffi::c_void, pfserializable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfCreatePropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCreatePropertyStore_Impl::IsStoreSerializable(this, core::mem::transmute_copy(&guidprop), windows_core::from_raw_borrowed(&prange), windows_core::from_raw_borrowed(&ppropstore)) {
                Ok(ok__) => {
                    pfserializable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, prange: *mut core::ffi::c_void, cb: u32, pstream: *mut core::ffi::c_void, ppstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfCreatePropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCreatePropertyStore_Impl::CreatePropertyStore(this, core::mem::transmute_copy(&guidprop), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&cb), windows_core::from_raw_borrowed(&pstream)) {
                Ok(ok__) => {
                    ppstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStoreSerializable: IsStoreSerializable::<Identity, OFFSET>,
            CreatePropertyStore: CreatePropertyStore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCreatePropertyStore as windows_core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeInfo_Impl: Sized {
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::Result<()>;
    fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfDisplayAttributeInfo {}
impl ITfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfDisplayAttributeInfo_Vtbl
    where
        Identity: ITfDisplayAttributeInfo_Impl,
    {
        unsafe extern "system" fn GetGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeInfo_Impl::GetGUID(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdesc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeInfo_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeInfo_Impl::GetAttributeInfo(this, core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn SetAttributeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeInfo_Impl::SetAttributeInfo(this, core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeInfo_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, OFFSET>,
            SetAttributeInfo: SetAttributeInfo::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeInfo as windows_core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeMgr_Impl: Sized {
    fn OnUpdateInfo(&self) -> windows_core::Result<()>;
    fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID, ppinfo: *mut Option<ITfDisplayAttributeInfo>, pclsidowner: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfDisplayAttributeMgr {}
impl ITfDisplayAttributeMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfDisplayAttributeMgr_Vtbl
    where
        Identity: ITfDisplayAttributeMgr_Impl,
    {
        unsafe extern "system" fn OnUpdateInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeMgr_Impl::OnUpdateInfo(this).into()
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeMgr_Impl::EnumDisplayAttributeInfo(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, ppinfo: *mut *mut core::ffi::c_void, pclsidowner: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeMgr_Impl::GetDisplayAttributeInfo(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&ppinfo), core::mem::transmute_copy(&pclsidowner)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdateInfo: OnUpdateInfo::<Identity, OFFSET>,
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeMgr as windows_core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeNotifySink_Impl: Sized {
    fn OnUpdateInfo(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfDisplayAttributeNotifySink {}
impl ITfDisplayAttributeNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfDisplayAttributeNotifySink_Vtbl
    where
        Identity: ITfDisplayAttributeNotifySink_Impl,
    {
        unsafe extern "system" fn OnUpdateInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeNotifySink_Impl::OnUpdateInfo(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdateInfo: OnUpdateInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeNotifySink as windows_core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeProvider_Impl: Sized {
    fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITfDisplayAttributeInfo>;
}
impl windows_core::RuntimeName for ITfDisplayAttributeProvider {}
impl ITfDisplayAttributeProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfDisplayAttributeProvider_Vtbl
    where
        Identity: ITfDisplayAttributeProvider_Impl,
    {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeProvider_Impl::EnumDisplayAttributeInfo(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDisplayAttributeProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeProvider_Impl::GetDisplayAttributeInfo(this, core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    ppinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeProvider as windows_core::Interface>::IID
    }
}
pub trait ITfDocumentMgr_Impl: Sized {
    fn CreateContext(&self, tidowner: u32, dwflags: u32, punk: Option<&windows_core::IUnknown>, ppic: *mut Option<ITfContext>, pectextstore: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
    fn Pop(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetTop(&self) -> windows_core::Result<ITfContext>;
    fn GetBase(&self) -> windows_core::Result<ITfContext>;
    fn EnumContexts(&self) -> windows_core::Result<IEnumTfContexts>;
}
impl windows_core::RuntimeName for ITfDocumentMgr {}
impl ITfDocumentMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfDocumentMgr_Vtbl
    where
        Identity: ITfDocumentMgr_Impl,
    {
        unsafe extern "system" fn CreateContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void, pectextstore: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfDocumentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDocumentMgr_Impl::CreateContext(this, core::mem::transmute_copy(&tidowner), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&ppic), core::mem::transmute_copy(&pectextstore)).into()
        }
        unsafe extern "system" fn Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDocumentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDocumentMgr_Impl::Push(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn Pop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfDocumentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDocumentMgr_Impl::Pop(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDocumentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDocumentMgr_Impl::GetTop(this) {
                Ok(ok__) => {
                    ppic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDocumentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDocumentMgr_Impl::GetBase(this) {
                Ok(ok__) => {
                    ppic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumContexts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfDocumentMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDocumentMgr_Impl::EnumContexts(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContext: CreateContext::<Identity, OFFSET>,
            Push: Push::<Identity, OFFSET>,
            Pop: Pop::<Identity, OFFSET>,
            GetTop: GetTop::<Identity, OFFSET>,
            GetBase: GetBase::<Identity, OFFSET>,
            EnumContexts: EnumContexts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDocumentMgr as windows_core::Interface>::IID
    }
}
pub trait ITfEditRecord_Impl: Sized {
    fn GetSelectionStatus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetTextAndPropertyUpdates(&self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const windows_core::GUID, cproperties: u32) -> windows_core::Result<IEnumTfRanges>;
}
impl windows_core::RuntimeName for ITfEditRecord {}
impl ITfEditRecord_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfEditRecord_Vtbl
    where
        Identity: ITfEditRecord_Impl,
    {
        unsafe extern "system" fn GetSelectionStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfEditRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfEditRecord_Impl::GetSelectionStatus(this) {
                Ok(ok__) => {
                    pfchanged.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const windows_core::GUID, cproperties: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfEditRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfEditRecord_Impl::GetTextAndPropertyUpdates(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&prgproperties), core::mem::transmute_copy(&cproperties)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelectionStatus: GetSelectionStatus::<Identity, OFFSET>,
            GetTextAndPropertyUpdates: GetTextAndPropertyUpdates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditRecord as windows_core::Interface>::IID
    }
}
pub trait ITfEditSession_Impl: Sized {
    fn DoEditSession(&self, ec: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfEditSession {}
impl ITfEditSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfEditSession_Vtbl
    where
        Identity: ITfEditSession_Impl,
    {
        unsafe extern "system" fn DoEditSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32) -> windows_core::HRESULT
        where
            Identity: ITfEditSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfEditSession_Impl::DoEditSession(this, core::mem::transmute_copy(&ec)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoEditSession: DoEditSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditSession as windows_core::Interface>::IID
    }
}
pub trait ITfEditTransactionSink_Impl: Sized {
    fn OnStartEditTransaction(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfEditTransactionSink {}
impl ITfEditTransactionSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfEditTransactionSink_Vtbl
    where
        Identity: ITfEditTransactionSink_Impl,
    {
        unsafe extern "system" fn OnStartEditTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfEditTransactionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfEditTransactionSink_Impl::OnStartEditTransaction(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfEditTransactionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfEditTransactionSink_Impl::OnEndEditTransaction(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditTransactionSink as windows_core::Interface>::IID
    }
}
pub trait ITfFnAdviseText_Impl: Sized + ITfFunction_Impl {
    fn OnTextUpdate(&self, prange: Option<&ITfRange>, pchtext: &windows_core::PCWSTR, cch: i32) -> windows_core::Result<()>;
    fn OnLatticeUpdate(&self, prange: Option<&ITfRange>, plattice: Option<&ITfLMLattice>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnAdviseText {}
impl ITfFnAdviseText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnAdviseText_Vtbl
    where
        Identity: ITfFnAdviseText_Impl,
    {
        unsafe extern "system" fn OnTextUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pchtext: windows_core::PCWSTR, cch: i32) -> windows_core::HRESULT
        where
            Identity: ITfFnAdviseText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnAdviseText_Impl::OnTextUpdate(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn OnLatticeUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, plattice: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnAdviseText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnAdviseText_Impl::OnLatticeUpdate(this, windows_core::from_raw_borrowed(&prange), windows_core::from_raw_borrowed(&plattice)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            OnTextUpdate: OnTextUpdate::<Identity, OFFSET>,
            OnLatticeUpdate: OnLatticeUpdate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnAdviseText as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnBalloon_Impl: Sized {
    fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnBalloon {}
impl ITfFnBalloon_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnBalloon_Vtbl
    where
        Identity: ITfFnBalloon_Impl,
    {
        unsafe extern "system" fn UpdateBalloon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, style: TfLBBalloonStyle, pch: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT
        where
            Identity: ITfFnBalloon_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnBalloon_Impl::UpdateBalloon(this, core::mem::transmute_copy(&style), core::mem::transmute(&pch), core::mem::transmute_copy(&cch)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), UpdateBalloon: UpdateBalloon::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnBalloon as windows_core::Interface>::IID
    }
}
pub trait ITfFnConfigure_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnConfigure {}
impl ITfFnConfigure_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnConfigure_Vtbl
    where
        Identity: ITfFnConfigure_Impl,
    {
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfFnConfigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnConfigure_Impl::Show(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rguidprofile)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnConfigure as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnConfigureRegisterEudc_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnConfigureRegisterEudc {}
impl ITfFnConfigureRegisterEudc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnConfigureRegisterEudc_Vtbl
    where
        Identity: ITfFnConfigureRegisterEudc_Impl,
    {
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfFnConfigureRegisterEudc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnConfigureRegisterEudc_Impl::Show(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rguidprofile), core::mem::transmute(&bstrregistered)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterEudc as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnConfigureRegisterWord_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnConfigureRegisterWord {}
impl ITfFnConfigureRegisterWord_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnConfigureRegisterWord_Vtbl
    where
        Identity: ITfFnConfigureRegisterWord_Impl,
    {
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfFnConfigureRegisterWord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnConfigureRegisterWord_Impl::Show(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rguidprofile), core::mem::transmute(&bstrregistered)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterWord as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnCustomSpeechCommand_Impl: Sized + ITfFunction_Impl {
    fn SetSpeechCommandProvider(&self, pspcmdprovider: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnCustomSpeechCommand {}
impl ITfFnCustomSpeechCommand_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnCustomSpeechCommand_Vtbl
    where
        Identity: ITfFnCustomSpeechCommand_Impl,
    {
        unsafe extern "system" fn SetSpeechCommandProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspcmdprovider: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnCustomSpeechCommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnCustomSpeechCommand_Impl::SetSpeechCommandProvider(this, windows_core::from_raw_borrowed(&pspcmdprovider)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), SetSpeechCommandProvider: SetSpeechCommandProvider::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnCustomSpeechCommand as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnGetLinguisticAlternates_Impl: Sized + ITfFunction_Impl {
    fn GetAlternates(&self, prange: Option<&ITfRange>) -> windows_core::Result<ITfCandidateList>;
}
impl windows_core::RuntimeName for ITfFnGetLinguisticAlternates {}
impl ITfFnGetLinguisticAlternates_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnGetLinguisticAlternates_Vtbl
    where
        Identity: ITfFnGetLinguisticAlternates_Impl,
    {
        unsafe extern "system" fn GetAlternates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppcandidatelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnGetLinguisticAlternates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnGetLinguisticAlternates_Impl::GetAlternates(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppcandidatelist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), GetAlternates: GetAlternates::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnGetLinguisticAlternates as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnGetPreferredTouchKeyboardLayout_Impl: Sized + ITfFunction_Impl {
    fn GetLayout(&self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnGetPreferredTouchKeyboardLayout {}
impl ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnGetPreferredTouchKeyboardLayout_Vtbl
    where
        Identity: ITfFnGetPreferredTouchKeyboardLayout_Impl,
    {
        unsafe extern "system" fn GetLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> windows_core::HRESULT
        where
            Identity: ITfFnGetPreferredTouchKeyboardLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnGetPreferredTouchKeyboardLayout_Impl::GetLayout(this, core::mem::transmute_copy(&ptkblayouttype), core::mem::transmute_copy(&pwpreferredlayoutid)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), GetLayout: GetLayout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnGetPreferredTouchKeyboardLayout as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnGetSAPIObject_Impl: Sized + ITfFunction_Impl {
    fn Get(&self, sobj: TfSapiObject) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ITfFnGetSAPIObject {}
impl ITfFnGetSAPIObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnGetSAPIObject_Vtbl
    where
        Identity: ITfFnGetSAPIObject_Impl,
    {
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnGetSAPIObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnGetSAPIObject_Impl::Get(this, core::mem::transmute_copy(&sobj)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnGetSAPIObject as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnLMInternal_Impl: Sized + ITfFnLMProcessor_Impl {
    fn ProcessLattice(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnLMInternal {}
impl ITfFnLMInternal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnLMInternal_Vtbl
    where
        Identity: ITfFnLMInternal_Impl,
    {
        unsafe extern "system" fn ProcessLattice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnLMInternal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMInternal_Impl::ProcessLattice(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: ITfFnLMProcessor_Vtbl::new::<Identity, OFFSET>(), ProcessLattice: ProcessLattice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnLMInternal as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID || iid == &<ITfFnLMProcessor as windows_core::Interface>::IID
    }
}
pub trait ITfFnLMProcessor_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&self, prange: Option<&ITfRange>, ppnewrange: *mut Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn QueryLangID(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetReconversion(&self, prange: Option<&ITfRange>) -> windows_core::Result<ITfCandidateList>;
    fn Reconvert(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
    fn QueryKey(&self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InvokeKey(&self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn InvokeFunc(&self, pic: Option<&ITfContext>, refguidfunc: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnLMProcessor {}
impl ITfFnLMProcessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnLMProcessor_Vtbl
    where
        Identity: ITfFnLMProcessor_Impl,
    {
        unsafe extern "system" fn QueryRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppnewrange: *mut *mut core::ffi::c_void, pfaccepted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfFnLMProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::QueryRange(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&ppnewrange), core::mem::transmute_copy(&pfaccepted)).into()
        }
        unsafe extern "system" fn QueryLangID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfFnLMProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLMProcessor_Impl::QueryLangID(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    pfaccepted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReconversion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppcandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnLMProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLMProcessor_Impl::GetReconversion(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppcandlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnLMProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::Reconvert(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn QueryKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfFnLMProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLMProcessor_Impl::QueryKey(this, core::mem::transmute_copy(&fup), core::mem::transmute_copy(&vkey), core::mem::transmute_copy(&lparamkeydata)) {
                Ok(ok__) => {
                    pfinterested.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: ITfFnLMProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::InvokeKey(this, core::mem::transmute_copy(&fup), core::mem::transmute_copy(&vkey), core::mem::transmute_copy(&lparamkeydata)).into()
        }
        unsafe extern "system" fn InvokeFunc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, refguidfunc: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfFnLMProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::InvokeFunc(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&refguidfunc)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            QueryRange: QueryRange::<Identity, OFFSET>,
            QueryLangID: QueryLangID::<Identity, OFFSET>,
            GetReconversion: GetReconversion::<Identity, OFFSET>,
            Reconvert: Reconvert::<Identity, OFFSET>,
            QueryKey: QueryKey::<Identity, OFFSET>,
            InvokeKey: InvokeKey::<Identity, OFFSET>,
            InvokeFunc: InvokeFunc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnLMProcessor as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnLangProfileUtil_Impl: Sized + ITfFunction_Impl {
    fn RegisterActiveProfiles(&self) -> windows_core::Result<()>;
    fn IsProfileAvailableForLang(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfFnLangProfileUtil {}
impl ITfFnLangProfileUtil_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnLangProfileUtil_Vtbl
    where
        Identity: ITfFnLangProfileUtil_Impl,
    {
        unsafe extern "system" fn RegisterActiveProfiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnLangProfileUtil_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLangProfileUtil_Impl::RegisterActiveProfiles(this).into()
        }
        unsafe extern "system" fn IsProfileAvailableForLang<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfFnLangProfileUtil_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLangProfileUtil_Impl::IsProfileAvailableForLang(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    pfavailable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            RegisterActiveProfiles: RegisterActiveProfiles::<Identity, OFFSET>,
            IsProfileAvailableForLang: IsProfileAvailableForLang::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnLangProfileUtil as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnPlayBack_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&self, prange: Option<&ITfRange>, ppnewrange: *mut Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Play(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnPlayBack {}
impl ITfFnPlayBack_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnPlayBack_Vtbl
    where
        Identity: ITfFnPlayBack_Impl,
    {
        unsafe extern "system" fn QueryRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppnewrange: *mut *mut core::ffi::c_void, pfplayable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfFnPlayBack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnPlayBack_Impl::QueryRange(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&ppnewrange), core::mem::transmute_copy(&pfplayable)).into()
        }
        unsafe extern "system" fn Play<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnPlayBack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnPlayBack_Impl::Play(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), QueryRange: QueryRange::<Identity, OFFSET>, Play: Play::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnPlayBack as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnPropertyUIStatus_Impl: Sized + ITfFunction_Impl {
    fn GetStatus(&self, refguidprop: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn SetStatus(&self, refguidprop: *const windows_core::GUID, dw: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnPropertyUIStatus {}
impl ITfFnPropertyUIStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnPropertyUIStatus_Vtbl
    where
        Identity: ITfFnPropertyUIStatus_Impl,
    {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguidprop: *const windows_core::GUID, pdw: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfFnPropertyUIStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnPropertyUIStatus_Impl::GetStatus(this, core::mem::transmute_copy(&refguidprop)) {
                Ok(ok__) => {
                    pdw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguidprop: *const windows_core::GUID, dw: u32) -> windows_core::HRESULT
        where
            Identity: ITfFnPropertyUIStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnPropertyUIStatus_Impl::SetStatus(this, core::mem::transmute_copy(&refguidprop), core::mem::transmute_copy(&dw)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), GetStatus: GetStatus::<Identity, OFFSET>, SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnPropertyUIStatus as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnReconversion_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&self, prange: Option<&ITfRange>, ppnewrange: *mut Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetReconversion(&self, prange: Option<&ITfRange>) -> windows_core::Result<ITfCandidateList>;
    fn Reconvert(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnReconversion {}
impl ITfFnReconversion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnReconversion_Vtbl
    where
        Identity: ITfFnReconversion_Impl,
    {
        unsafe extern "system" fn QueryRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppnewrange: *mut *mut core::ffi::c_void, pfconvertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfFnReconversion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnReconversion_Impl::QueryRange(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&ppnewrange), core::mem::transmute_copy(&pfconvertable)).into()
        }
        unsafe extern "system" fn GetReconversion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppcandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnReconversion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnReconversion_Impl::GetReconversion(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppcandlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnReconversion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnReconversion_Impl::Reconvert(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            QueryRange: QueryRange::<Identity, OFFSET>,
            GetReconversion: GetReconversion::<Identity, OFFSET>,
            Reconvert: Reconvert::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnReconversion as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnSearchCandidateProvider_Impl: Sized + ITfFunction_Impl {
    fn GetSearchCandidates(&self, bstrquery: &windows_core::BSTR, bstrapplicationid: &windows_core::BSTR) -> windows_core::Result<ITfCandidateList>;
    fn SetResult(&self, bstrquery: &windows_core::BSTR, bstrapplicationid: &windows_core::BSTR, bstrresult: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnSearchCandidateProvider {}
impl ITfFnSearchCandidateProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnSearchCandidateProvider_Vtbl
    where
        Identity: ITfFnSearchCandidateProvider_Impl,
    {
        unsafe extern "system" fn GetSearchCandidates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquery: core::mem::MaybeUninit<windows_core::BSTR>, bstrapplicationid: core::mem::MaybeUninit<windows_core::BSTR>, pplist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFnSearchCandidateProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnSearchCandidateProvider_Impl::GetSearchCandidates(this, core::mem::transmute(&bstrquery), core::mem::transmute(&bstrapplicationid)) {
                Ok(ok__) => {
                    pplist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquery: core::mem::MaybeUninit<windows_core::BSTR>, bstrapplicationid: core::mem::MaybeUninit<windows_core::BSTR>, bstrresult: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfFnSearchCandidateProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnSearchCandidateProvider_Impl::SetResult(this, core::mem::transmute(&bstrquery), core::mem::transmute(&bstrapplicationid), core::mem::transmute(&bstrresult)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            GetSearchCandidates: GetSearchCandidates::<Identity, OFFSET>,
            SetResult: SetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnSearchCandidateProvider as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFnShowHelp_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfFnShowHelp {}
impl ITfFnShowHelp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFnShowHelp_Vtbl
    where
        Identity: ITfFnShowHelp_Impl,
    {
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ITfFnShowHelp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnShowHelp_Impl::Show(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnShowHelp as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFunction_Impl: Sized {
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITfFunction {}
impl ITfFunction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFunction_Vtbl
    where
        Identity: ITfFunction_Impl,
    {
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfFunction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunction_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDisplayName: GetDisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
pub trait ITfFunctionProvider_Impl: Sized {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFunction(&self, rguid: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ITfFunctionProvider {}
impl ITfFunctionProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfFunctionProvider_Vtbl
    where
        Identity: ITfFunctionProvider_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfFunctionProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunctionProvider_Impl::GetType(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdesc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfFunctionProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunctionProvider_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfFunctionProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunctionProvider_Impl::GetFunction(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFunctionProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait ITfInputProcessorProfileActivationSink_Impl: Sized {
    fn OnActivated(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for ITfInputProcessorProfileActivationSink {}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl ITfInputProcessorProfileActivationSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInputProcessorProfileActivationSink_Vtbl
    where
        Identity: ITfInputProcessorProfileActivationSink_Impl,
    {
        unsafe extern "system" fn OnActivated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileActivationSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileActivationSink_Impl::OnActivated(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&catid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileActivationSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait ITfInputProcessorProfileMgr_Impl: Sized {
    fn ActivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()>;
    fn DeactivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()>;
    fn GetProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()>;
    fn EnumProfiles(&self, langid: u16) -> windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn ReleaseInputProcessor(&self, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RegisterProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cchdesc: u32, pchiconfile: &windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: super::Input::KeyboardAndMouse::HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> windows_core::Result<()>;
    fn UnregisterProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveProfile(&self, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for ITfInputProcessorProfileMgr {}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl ITfInputProcessorProfileMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInputProcessorProfileMgr_Vtbl
    where
        Identity: ITfInputProcessorProfileMgr_Impl,
    {
        unsafe extern "system" fn ActivateProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::ActivateProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeactivateProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::DeactivateProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::GetProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&pprofile)).into()
        }
        unsafe extern "system" fn EnumProfiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfileMgr_Impl::EnumProfiles(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseInputProcessor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::ReleaseInputProcessor(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RegisterProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cchdesc: u32, pchiconfile: windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: super::Input::KeyboardAndMouse::HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::RegisterProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc), core::mem::transmute(&pchiconfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uiconindex), core::mem::transmute_copy(&hklsubstitute), core::mem::transmute_copy(&dwpreferredlayout), core::mem::transmute_copy(&benabledbydefault), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn UnregisterProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::UnregisterProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::GetActiveProfile(this, core::mem::transmute_copy(&catid), core::mem::transmute_copy(&pprofile)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ActivateProfile: ActivateProfile::<Identity, OFFSET>,
            DeactivateProfile: DeactivateProfile::<Identity, OFFSET>,
            GetProfile: GetProfile::<Identity, OFFSET>,
            EnumProfiles: EnumProfiles::<Identity, OFFSET>,
            ReleaseInputProcessor: ReleaseInputProcessor::<Identity, OFFSET>,
            RegisterProfile: RegisterProfile::<Identity, OFFSET>,
            UnregisterProfile: UnregisterProfile::<Identity, OFFSET>,
            GetActiveProfile: GetActiveProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait ITfInputProcessorProfileSubstituteLayout_Impl: Sized {
    fn GetSubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::Input::KeyboardAndMouse::HKL>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for ITfInputProcessorProfileSubstituteLayout {}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInputProcessorProfileSubstituteLayout_Vtbl
    where
        Identity: ITfInputProcessorProfileSubstituteLayout_Impl,
    {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, phkl: *mut super::Input::KeyboardAndMouse::HKL) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfileSubstituteLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfileSubstituteLayout_Impl::GetSubstituteKeyboardLayout(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                Ok(ok__) => {
                    phkl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSubstituteKeyboardLayout: GetSubstituteKeyboardLayout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileSubstituteLayout as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
pub trait ITfInputProcessorProfiles_Impl: Sized {
    fn Register(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Unregister(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn AddLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cchdesc: u32, pchiconfile: &windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> windows_core::Result<()>;
    fn RemoveLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumInputProcessorInfo(&self) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()>;
    fn ActivateLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetActiveLanguageProfile(&self, rclsid: *const windows_core::GUID, plangid: *mut u16, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetLanguageProfileDescription(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentLanguage(&self) -> windows_core::Result<u16>;
    fn ChangeCurrentLanguage(&self, langid: u16) -> windows_core::Result<()>;
    fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> windows_core::Result<()>;
    fn EnumLanguageProfiles(&self, langid: u16) -> windows_core::Result<IEnumTfLanguageProfiles>;
    fn EnableLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsEnabledLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EnableLanguageProfileByDefault(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl windows_core::RuntimeName for ITfInputProcessorProfiles {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl ITfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInputProcessorProfiles_Vtbl
    where
        Identity: ITfInputProcessorProfiles_Impl,
    {
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::Register(this, core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn Unregister<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::Unregister(this, core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn AddLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cchdesc: u32, pchiconfile: windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::AddLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc), core::mem::transmute(&pchiconfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uiconindex)).into()
        }
        unsafe extern "system" fn RemoveLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::RemoveLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::EnumInputProcessorInfo(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::GetDefaultLanguageProfile(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&catid), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::SetDefaultLanguageProfile(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn ActivateLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::ActivateLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, plangid: *mut u16, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::GetActiveLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&plangid), core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pbstrprofile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::GetLanguageProfileDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                Ok(ok__) => {
                    pbstrprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plangid: *mut u16) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::GetCurrentLanguage(this) {
                Ok(ok__) => {
                    plangid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::ChangeCurrentLanguage(this, core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetLanguageList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::GetLanguageList(this, core::mem::transmute_copy(&pplangid), core::mem::transmute_copy(&pulcount)).into()
        }
        unsafe extern "system" fn EnumLanguageProfiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::EnumLanguageProfiles(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::EnableLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::IsEnabledLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                Ok(ok__) => {
                    pfenable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::EnableLanguageProfileByDefault(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::SubstituteKeyboardLayout(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
            AddLanguageProfile: AddLanguageProfile::<Identity, OFFSET>,
            RemoveLanguageProfile: RemoveLanguageProfile::<Identity, OFFSET>,
            EnumInputProcessorInfo: EnumInputProcessorInfo::<Identity, OFFSET>,
            GetDefaultLanguageProfile: GetDefaultLanguageProfile::<Identity, OFFSET>,
            SetDefaultLanguageProfile: SetDefaultLanguageProfile::<Identity, OFFSET>,
            ActivateLanguageProfile: ActivateLanguageProfile::<Identity, OFFSET>,
            GetActiveLanguageProfile: GetActiveLanguageProfile::<Identity, OFFSET>,
            GetLanguageProfileDescription: GetLanguageProfileDescription::<Identity, OFFSET>,
            GetCurrentLanguage: GetCurrentLanguage::<Identity, OFFSET>,
            ChangeCurrentLanguage: ChangeCurrentLanguage::<Identity, OFFSET>,
            GetLanguageList: GetLanguageList::<Identity, OFFSET>,
            EnumLanguageProfiles: EnumLanguageProfiles::<Identity, OFFSET>,
            EnableLanguageProfile: EnableLanguageProfile::<Identity, OFFSET>,
            IsEnabledLanguageProfile: IsEnabledLanguageProfile::<Identity, OFFSET>,
            EnableLanguageProfileByDefault: EnableLanguageProfileByDefault::<Identity, OFFSET>,
            SubstituteKeyboardLayout: SubstituteKeyboardLayout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
pub trait ITfInputProcessorProfilesEx_Impl: Sized + ITfInputProcessorProfiles_Impl {
    fn SetLanguageProfileDisplayName(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchfile: &windows_core::PCWSTR, cchfile: u32, uresid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl windows_core::RuntimeName for ITfInputProcessorProfilesEx {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl ITfInputProcessorProfilesEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInputProcessorProfilesEx_Vtbl
    where
        Identity: ITfInputProcessorProfilesEx_Impl,
    {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchfile: windows_core::PCWSTR, cchfile: u32, uresid: u32) -> windows_core::HRESULT
        where
            Identity: ITfInputProcessorProfilesEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfilesEx_Impl::SetLanguageProfileDisplayName(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute(&pchfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uresid)).into()
        }
        Self {
            base__: ITfInputProcessorProfiles_Vtbl::new::<Identity, OFFSET>(),
            SetLanguageProfileDisplayName: SetLanguageProfileDisplayName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfilesEx as windows_core::Interface>::IID || iid == &<ITfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
pub trait ITfInputScope_Impl: Sized {
    fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> windows_core::Result<()>;
    fn GetPhrase(&self, ppbstrphrases: *mut *mut windows_core::BSTR, pccount: *mut u32) -> windows_core::Result<()>;
    fn GetRegularExpression(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSRGS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetXML(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITfInputScope {}
impl ITfInputScope_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInputScope_Vtbl
    where
        Identity: ITfInputScope_Impl,
    {
        unsafe extern "system" fn GetInputScopes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfInputScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputScope_Impl::GetInputScopes(this, core::mem::transmute_copy(&pprginputscopes), core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetPhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbstrphrases: *mut *mut windows_core::BSTR, pccount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfInputScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputScope_Impl::GetPhrase(this, core::mem::transmute_copy(&ppbstrphrases), core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetRegularExpression<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrregexp: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfInputScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope_Impl::GetRegularExpression(this) {
                Ok(ok__) => {
                    pbstrregexp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSRGS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsrgs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfInputScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope_Impl::GetSRGS(this) {
                Ok(ok__) => {
                    pbstrsrgs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfInputScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope_Impl::GetXML(this) {
                Ok(ok__) => {
                    pbstrxml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputScopes: GetInputScopes::<Identity, OFFSET>,
            GetPhrase: GetPhrase::<Identity, OFFSET>,
            GetRegularExpression: GetRegularExpression::<Identity, OFFSET>,
            GetSRGS: GetSRGS::<Identity, OFFSET>,
            GetXML: GetXML::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputScope as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInputScope2_Impl: Sized + ITfInputScope_Impl {
    fn EnumWordList(&self) -> windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfInputScope2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITfInputScope2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInputScope2_Vtbl
    where
        Identity: ITfInputScope2_Impl,
    {
        unsafe extern "system" fn EnumWordList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfInputScope2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope2_Impl::EnumWordList(this) {
                Ok(ok__) => {
                    ppenumstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfInputScope_Vtbl::new::<Identity, OFFSET>(), EnumWordList: EnumWordList::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputScope2 as windows_core::Interface>::IID || iid == &<ITfInputScope as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInsertAtSelection_Impl: Sized {
    fn InsertTextAtSelection(&self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: &windows_core::PCWSTR, cch: i32) -> windows_core::Result<ITfRange>;
    fn InsertEmbeddedAtSelection(&self, ec: u32, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<ITfRange>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfInsertAtSelection {}
#[cfg(feature = "Win32_System_Com")]
impl ITfInsertAtSelection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfInsertAtSelection_Vtbl
    where
        Identity: ITfInsertAtSelection_Impl,
    {
        unsafe extern "system" fn InsertTextAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: windows_core::PCWSTR, cch: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfInsertAtSelection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInsertAtSelection_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfInsertAtSelection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInsertAtSelection_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInsertAtSelection as windows_core::Interface>::IID
    }
}
pub trait ITfIntegratableCandidateListUIElement_Impl: Sized {
    fn SetIntegrationStyle(&self, guidintegrationstyle: &windows_core::GUID) -> windows_core::Result<()>;
    fn GetSelectionStyle(&self) -> windows_core::Result<TfIntegratableCandidateListSelectionStyle>;
    fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ShowCandidateNumbers(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn FinalizeExactCompositionString(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfIntegratableCandidateListUIElement {}
impl ITfIntegratableCandidateListUIElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfIntegratableCandidateListUIElement_Vtbl
    where
        Identity: ITfIntegratableCandidateListUIElement_Impl,
    {
        unsafe extern "system" fn SetIntegrationStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidintegrationstyle: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfIntegratableCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfIntegratableCandidateListUIElement_Impl::SetIntegrationStyle(this, core::mem::transmute(&guidintegrationstyle)).into()
        }
        unsafe extern "system" fn GetSelectionStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> windows_core::HRESULT
        where
            Identity: ITfIntegratableCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfIntegratableCandidateListUIElement_Impl::GetSelectionStyle(this) {
                Ok(ok__) => {
                    ptfselectionstyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfIntegratableCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfIntegratableCandidateListUIElement_Impl::OnKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCandidateNumbers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfIntegratableCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfIntegratableCandidateListUIElement_Impl::ShowCandidateNumbers(this) {
                Ok(ok__) => {
                    pfshow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalizeExactCompositionString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfIntegratableCandidateListUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfIntegratableCandidateListUIElement_Impl::FinalizeExactCompositionString(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIntegrationStyle: SetIntegrationStyle::<Identity, OFFSET>,
            GetSelectionStyle: GetSelectionStyle::<Identity, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            ShowCandidateNumbers: ShowCandidateNumbers::<Identity, OFFSET>,
            FinalizeExactCompositionString: FinalizeExactCompositionString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfIntegratableCandidateListUIElement as windows_core::Interface>::IID
    }
}
pub trait ITfKeyEventSink_Impl: Sized {
    fn OnSetFocus(&self, fforeground: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnTestKeyDown(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyDown(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnPreservedKey(&self, pic: Option<&ITfContext>, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfKeyEventSink {}
impl ITfKeyEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfKeyEventSink_Vtbl
    where
        Identity: ITfKeyEventSink_Impl,
    {
        unsafe extern "system" fn OnSetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeyEventSink_Impl::OnSetFocus(this, core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnTestKeyDown(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnTestKeyUp(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnKeyDown(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnKeyUp(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreservedKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeyEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnPreservedKey(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, OFFSET>,
            OnPreservedKey: OnPreservedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeyEventSink as windows_core::Interface>::IID
    }
}
pub trait ITfKeyTraceEventSink_Impl: Sized {
    fn OnKeyTraceDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn OnKeyTraceUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfKeyTraceEventSink {}
impl ITfKeyTraceEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfKeyTraceEventSink_Vtbl
    where
        Identity: ITfKeyTraceEventSink_Impl,
    {
        unsafe extern "system" fn OnKeyTraceDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: ITfKeyTraceEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeyTraceEventSink_Impl::OnKeyTraceDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn OnKeyTraceUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: ITfKeyTraceEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeyTraceEventSink_Impl::OnKeyTraceUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyTraceDown: OnKeyTraceDown::<Identity, OFFSET>,
            OnKeyTraceUp: OnKeyTraceUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeyTraceEventSink as windows_core::Interface>::IID
    }
}
pub trait ITfKeystrokeMgr_Impl: Sized {
    fn AdviseKeyEventSink(&self, tid: u32, psink: Option<&ITfKeyEventSink>, fforeground: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UnadviseKeyEventSink(&self, tid: u32) -> windows_core::Result<()>;
    fn GetForeground(&self) -> windows_core::Result<windows_core::GUID>;
    fn TestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn TestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetPreservedKey(&self, pic: Option<&ITfContext>, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<windows_core::GUID>;
    fn IsPreservedKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn PreserveKey(&self, tid: u32, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: &windows_core::PCWSTR, cchdesc: u32) -> windows_core::Result<()>;
    fn UnpreserveKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()>;
    fn SetPreservedKeyDescription(&self, rguid: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cchdesc: u32) -> windows_core::Result<()>;
    fn GetPreservedKeyDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn SimulatePreservedKey(&self, pic: Option<&ITfContext>, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfKeystrokeMgr {}
impl ITfKeystrokeMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfKeystrokeMgr_Vtbl
    where
        Identity: ITfKeystrokeMgr_Impl,
    {
        unsafe extern "system" fn AdviseKeyEventSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, psink: *mut core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::AdviseKeyEventSink(this, core::mem::transmute_copy(&tid), windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::UnadviseKeyEventSink(this, core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn GetForeground<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::GetForeground(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::TestKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::TestKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::KeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::KeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreservedKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::GetPreservedKey(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&pprekey)) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreservedKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::IsPreservedKey(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pprekey)) {
                Ok(ok__) => {
                    pfregistered.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreserveKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: windows_core::PCWSTR, cchdesc: u32) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::PreserveKey(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&prekey), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn UnpreserveKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::UnpreserveKey(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pprekey)).into()
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cchdesc: u32) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::SetPreservedKeyDescription(this, core::mem::transmute_copy(&rguid), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pbstrdesc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::GetPreservedKeyDescription(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulatePreservedKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfKeystrokeMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::SimulatePreservedKey(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseKeyEventSink: AdviseKeyEventSink::<Identity, OFFSET>,
            UnadviseKeyEventSink: UnadviseKeyEventSink::<Identity, OFFSET>,
            GetForeground: GetForeground::<Identity, OFFSET>,
            TestKeyDown: TestKeyDown::<Identity, OFFSET>,
            TestKeyUp: TestKeyUp::<Identity, OFFSET>,
            KeyDown: KeyDown::<Identity, OFFSET>,
            KeyUp: KeyUp::<Identity, OFFSET>,
            GetPreservedKey: GetPreservedKey::<Identity, OFFSET>,
            IsPreservedKey: IsPreservedKey::<Identity, OFFSET>,
            PreserveKey: PreserveKey::<Identity, OFFSET>,
            UnpreserveKey: UnpreserveKey::<Identity, OFFSET>,
            SetPreservedKeyDescription: SetPreservedKeyDescription::<Identity, OFFSET>,
            GetPreservedKeyDescription: GetPreservedKeyDescription::<Identity, OFFSET>,
            SimulatePreservedKey: SimulatePreservedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeystrokeMgr as windows_core::Interface>::IID
    }
}
pub trait ITfLMLattice_Impl: Sized {
    fn QueryType(&self, rguidtype: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EnumLatticeElements(&self, dwframestart: u32, rguidtype: *const windows_core::GUID) -> windows_core::Result<IEnumTfLatticeElements>;
}
impl windows_core::RuntimeName for ITfLMLattice {}
impl ITfLMLattice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLMLattice_Vtbl
    where
        Identity: ITfLMLattice_Impl,
    {
        unsafe extern "system" fn QueryType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidtype: *const windows_core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfLMLattice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLMLattice_Impl::QueryType(this, core::mem::transmute_copy(&rguidtype)) {
                Ok(ok__) => {
                    pfsupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumLatticeElements<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwframestart: u32, rguidtype: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLMLattice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLMLattice_Impl::EnumLatticeElements(this, core::mem::transmute_copy(&dwframestart), core::mem::transmute_copy(&rguidtype)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryType: QueryType::<Identity, OFFSET>,
            EnumLatticeElements: EnumLatticeElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLMLattice as windows_core::Interface>::IID
    }
}
pub trait ITfLangBarEventSink_Impl: Sized {
    fn OnSetFocus(&self, dwthreadid: u32) -> windows_core::Result<()>;
    fn OnThreadTerminate(&self, dwthreadid: u32) -> windows_core::Result<()>;
    fn OnThreadItemChange(&self, dwthreadid: u32) -> windows_core::Result<()>;
    fn OnModalInput(&self, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::RECT>;
}
impl windows_core::RuntimeName for ITfLangBarEventSink {}
impl ITfLangBarEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarEventSink_Vtbl
    where
        Identity: ITfLangBarEventSink_Impl,
    {
        unsafe extern "system" fn OnSetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnSetFocus(this, core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnThreadTerminate(this, core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadItemChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnThreadItemChange(this, core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnModalInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: ITfLangBarEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnModalInput(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn ShowFloating<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::ShowFloating(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, rguid: *const windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfLangBarEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarEventSink_Impl::GetItemFloatingRect(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, OFFSET>,
            OnThreadItemChange: OnThreadItemChange::<Identity, OFFSET>,
            OnModalInput: OnModalInput::<Identity, OFFSET>,
            ShowFloating: ShowFloating::<Identity, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarEventSink as windows_core::Interface>::IID
    }
}
pub trait ITfLangBarItem_Impl: Sized {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Show(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetTooltipString(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITfLangBarItem {}
impl ITfLangBarItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarItem_Vtbl
    where
        Identity: ITfLangBarItem_Impl,
    {
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItem_Impl::GetInfo(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItem_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItem_Impl::Show(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetTooltipString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtooltip: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItem_Impl::GetTooltipString(this) {
                Ok(ok__) => {
                    pbstrtooltip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            GetTooltipString: GetTooltipString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
pub trait ITfLangBarItemBalloon_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn GetBalloonInfo(&self) -> windows_core::Result<TF_LBBALLOONINFO>;
}
impl windows_core::RuntimeName for ITfLangBarItemBalloon {}
impl ITfLangBarItemBalloon_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarItemBalloon_Vtbl
    where
        Identity: ITfLangBarItemBalloon_Impl,
    {
        unsafe extern "system" fn OnClick<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBalloon_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBalloon_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBalloon_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBalloon_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault)) {
                Ok(ok__) => {
                    psz.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBalloonInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBalloon_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBalloon_Impl::GetBalloonInfo(this) {
                Ok(ok__) => {
                    pinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            GetBalloonInfo: GetBalloonInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBalloon as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfLangBarItemBitmap_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for ITfLangBarItemBitmap {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ITfLangBarItemBitmap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarItemBitmap_Vtbl
    where
        Identity: ITfLangBarItemBitmap_Impl,
    {
        unsafe extern "system" fn OnClick<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmap_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBitmap_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault)) {
                Ok(ok__) => {
                    psz.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmap_Impl::DrawBitmap(this, core::mem::transmute_copy(&bmwidth), core::mem::transmute_copy(&bmheight), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&phbmp), core::mem::transmute_copy(&phbmpmask)).into()
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmap as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfLangBarItemBitmapButton_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for ITfLangBarItemBitmapButton {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ITfLangBarItemBitmapButton_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarItemBitmapButton_Vtbl
    where
        Identity: ITfLangBarItemBitmapButton_Impl,
    {
        unsafe extern "system" fn OnClick<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmapButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmapButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::InitMenu(this, windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmapButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmapButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBitmapButton_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault)) {
                Ok(ok__) => {
                    psz.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmapButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::DrawBitmap(this, core::mem::transmute_copy(&bmwidth), core::mem::transmute_copy(&bmheight), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&phbmp), core::mem::transmute_copy(&phbmpmask)).into()
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemBitmapButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBitmapButton_Impl::GetText(this) {
                Ok(ok__) => {
                    pbstrtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmapButton as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfLangBarItemButton_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()>;
    fn GetIcon(&self) -> windows_core::Result<super::WindowsAndMessaging::HICON>;
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ITfLangBarItemButton {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ITfLangBarItemButton_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarItemButton_Vtbl
    where
        Identity: ITfLangBarItemButton_Impl,
    {
        unsafe extern "system" fn OnClick<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemButton_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemButton_Impl::InitMenu(this, windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemButton_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemButton_Impl::GetIcon(this) {
                Ok(ok__) => {
                    phicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemButton_Impl::GetText(this) {
                Ok(ok__) => {
                    pbstrtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemButton as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
pub trait ITfLangBarItemMgr_Impl: Sized {
    fn EnumItems(&self) -> windows_core::Result<IEnumTfLangBarItems>;
    fn GetItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfLangBarItem>;
    fn AddItem(&self, punk: Option<&ITfLangBarItem>) -> windows_core::Result<()>;
    fn RemoveItem(&self, punk: Option<&ITfLangBarItem>) -> windows_core::Result<()>;
    fn AdviseItemSink(&self, punk: Option<&ITfLangBarItemSink>, pdwcookie: *mut u32, rguiditem: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnadviseItemSink(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetItemsStatus(&self, ulcount: u32, prgguid: *const windows_core::GUID, pdwstatus: *mut u32) -> windows_core::Result<()>;
    fn GetItemNum(&self) -> windows_core::Result<u32>;
    fn GetItems(&self, ulcount: u32, ppitem: *mut Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn AdviseItemsSink(&self, ulcount: u32, ppunk: *const Option<ITfLangBarItemSink>, pguiditem: *const windows_core::GUID, pdwcookie: *mut u32) -> windows_core::Result<()>;
    fn UnadviseItemsSink(&self, ulcount: u32, pdwcookie: *const u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfLangBarItemMgr {}
impl ITfLangBarItemMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarItemMgr_Vtbl
    where
        Identity: ITfLangBarItemMgr_Impl,
    {
        unsafe extern "system" fn EnumItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::EnumItems(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::GetItem(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::AddItem(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::RemoveItem(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn AdviseItemSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, pdwcookie: *mut u32, rguiditem: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::AdviseItemSink(this, windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&pdwcookie), core::mem::transmute_copy(&rguiditem)).into()
        }
        unsafe extern "system" fn UnadviseItemSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::UnadviseItemSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, rguid: *const windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::GetItemFloatingRect(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, prgguid: *const windows_core::GUID, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::GetItemsStatus(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&prgguid), core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn GetItemNum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::GetItemNum(this) {
                Ok(ok__) => {
                    pulcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppitem: *mut *mut core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::GetItems(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppitem), core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn AdviseItemsSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppunk: *const *mut core::ffi::c_void, pguiditem: *const windows_core::GUID, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::AdviseItemsSink(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppunk), core::mem::transmute_copy(&pguiditem), core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseItemsSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::UnadviseItemsSink(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pdwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumItems: EnumItems::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            AdviseItemSink: AdviseItemSink::<Identity, OFFSET>,
            UnadviseItemSink: UnadviseItemSink::<Identity, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, OFFSET>,
            GetItemsStatus: GetItemsStatus::<Identity, OFFSET>,
            GetItemNum: GetItemNum::<Identity, OFFSET>,
            GetItems: GetItems::<Identity, OFFSET>,
            AdviseItemsSink: AdviseItemsSink::<Identity, OFFSET>,
            UnadviseItemsSink: UnadviseItemsSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemMgr as windows_core::Interface>::IID
    }
}
pub trait ITfLangBarItemSink_Impl: Sized {
    fn OnUpdate(&self, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfLangBarItemSink {}
impl ITfLangBarItemSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarItemSink_Vtbl
    where
        Identity: ITfLangBarItemSink_Impl,
    {
        unsafe extern "system" fn OnUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarItemSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemSink_Impl::OnUpdate(this, core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdate: OnUpdate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemSink as windows_core::Interface>::IID
    }
}
pub trait ITfLangBarMgr_Impl: Sized {
    fn AdviseEventSink(&self, psink: Option<&ITfLangBarEventSink>, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> windows_core::Result<()>;
    fn UnadviseEventSink(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn GetThreadMarshalInterface(&self, dwthreadid: u32, dwtype: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetThreadLangBarItemMgr(&self, dwthreadid: u32, pplbi: *mut Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> windows_core::Result<()>;
    fn GetInputProcessorProfiles(&self, dwthreadid: u32, ppaip: *mut Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> windows_core::Result<()>;
    fn RestoreLastFocus(&self, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetModalInput(&self, psink: Option<&ITfLangBarEventSink>, dwthreadid: u32, dwflags: u32) -> windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetShowFloatingStatus(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ITfLangBarMgr {}
impl ITfLangBarMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLangBarMgr_Vtbl
    where
        Identity: ITfLangBarMgr_Impl,
    {
        unsafe extern "system" fn AdviseEventSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::AdviseEventSink(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseEventSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::UnadviseEventSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarMgr_Impl::GetThreadMarshalInterface(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, pplbi: *mut *mut core::ffi::c_void, pdwthreadid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::GetThreadLangBarItemMgr(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&pplbi), core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, ppaip: *mut *mut core::ffi::c_void, pdwthreadid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::GetInputProcessorProfiles(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&ppaip), core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn RestoreLastFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::RestoreLastFocus(this, core::mem::transmute_copy(&pdwthreadid), core::mem::transmute_copy(&fprev)).into()
        }
        unsafe extern "system" fn SetModalInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, dwthreadid: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::SetModalInput(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ShowFloating<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::ShowFloating(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetShowFloatingStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfLangBarMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarMgr_Impl::GetShowFloatingStatus(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseEventSink: AdviseEventSink::<Identity, OFFSET>,
            UnadviseEventSink: UnadviseEventSink::<Identity, OFFSET>,
            GetThreadMarshalInterface: GetThreadMarshalInterface::<Identity, OFFSET>,
            GetThreadLangBarItemMgr: GetThreadLangBarItemMgr::<Identity, OFFSET>,
            GetInputProcessorProfiles: GetInputProcessorProfiles::<Identity, OFFSET>,
            RestoreLastFocus: RestoreLastFocus::<Identity, OFFSET>,
            SetModalInput: SetModalInput::<Identity, OFFSET>,
            ShowFloating: ShowFloating::<Identity, OFFSET>,
            GetShowFloatingStatus: GetShowFloatingStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarMgr as windows_core::Interface>::IID
    }
}
pub trait ITfLanguageProfileNotifySink_Impl: Sized {
    fn OnLanguageChange(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnLanguageChanged(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfLanguageProfileNotifySink {}
impl ITfLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfLanguageProfileNotifySink_Vtbl
    where
        Identity: ITfLanguageProfileNotifySink_Impl,
    {
        unsafe extern "system" fn OnLanguageChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfLanguageProfileNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLanguageProfileNotifySink_Impl::OnLanguageChange(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    pfaccept.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLanguageChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfLanguageProfileNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLanguageProfileNotifySink_Impl::OnLanguageChanged(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLanguageChange: OnLanguageChange::<Identity, OFFSET>,
            OnLanguageChanged: OnLanguageChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLanguageProfileNotifySink as windows_core::Interface>::IID
    }
}
pub trait ITfMSAAControl_Impl: Sized {
    fn SystemEnableMSAA(&self) -> windows_core::Result<()>;
    fn SystemDisableMSAA(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfMSAAControl {}
impl ITfMSAAControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfMSAAControl_Vtbl
    where
        Identity: ITfMSAAControl_Impl,
    {
        unsafe extern "system" fn SystemEnableMSAA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfMSAAControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMSAAControl_Impl::SystemEnableMSAA(this).into()
        }
        unsafe extern "system" fn SystemDisableMSAA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfMSAAControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMSAAControl_Impl::SystemDisableMSAA(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SystemEnableMSAA: SystemEnableMSAA::<Identity, OFFSET>,
            SystemDisableMSAA: SystemDisableMSAA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMSAAControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfMenu_Impl: Sized {
    fn AddMenuItem(&self, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: &windows_core::PCWSTR, cch: u32, ppmenu: *mut Option<ITfMenu>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for ITfMenu {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ITfMenu_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfMenu_Vtbl
    where
        Identity: ITfMenu_Impl,
    {
        unsafe extern "system" fn AddMenuItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: windows_core::PCWSTR, cch: u32, ppmenu: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfMenu_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMenu_Impl::AddMenuItem(this, core::mem::transmute_copy(&uid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&hbmpmask), core::mem::transmute(&pch), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&ppmenu)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddMenuItem: AddMenuItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMenu as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfMessagePump_Impl: Sized {
    fn PeekMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn PeekMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ITfMessagePump {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ITfMessagePump_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfMessagePump_Vtbl
    where
        Identity: ITfMessagePump_Impl,
    {
        unsafe extern "system" fn PeekMessageA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfMessagePump_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::PeekMessageA(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&wremovemsg), core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfMessagePump_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::GetMessageA(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn PeekMessageW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfMessagePump_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::PeekMessageW(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&wremovemsg), core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfMessagePump_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::GetMessageW(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&pfresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PeekMessageA: PeekMessageA::<Identity, OFFSET>,
            GetMessageA: GetMessageA::<Identity, OFFSET>,
            PeekMessageW: PeekMessageW::<Identity, OFFSET>,
            GetMessageW: GetMessageW::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMessagePump as windows_core::Interface>::IID
    }
}
pub trait ITfMouseSink_Impl: Sized {
    fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfMouseSink {}
impl ITfMouseSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfMouseSink_Vtbl
    where
        Identity: ITfMouseSink_Impl,
    {
        unsafe extern "system" fn OnMouseEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfMouseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfMouseSink_Impl::OnMouseEvent(this, core::mem::transmute_copy(&uedge), core::mem::transmute_copy(&uquadrant), core::mem::transmute_copy(&dwbtnstatus)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMouseEvent: OnMouseEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseSink as windows_core::Interface>::IID
    }
}
pub trait ITfMouseTracker_Impl: Sized {
    fn AdviseMouseSink(&self, range: Option<&ITfRange>, psink: Option<&ITfMouseSink>) -> windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfMouseTracker {}
impl ITfMouseTracker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfMouseTracker_Vtbl
    where
        Identity: ITfMouseTracker_Impl,
    {
        unsafe extern "system" fn AdviseMouseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfMouseTracker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfMouseTracker_Impl::AdviseMouseSink(this, windows_core::from_raw_borrowed(&range), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ITfMouseTracker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMouseTracker_Impl::UnadviseMouseSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseTracker as windows_core::Interface>::IID
    }
}
pub trait ITfMouseTrackerACP_Impl: Sized {
    fn AdviseMouseSink(&self, range: Option<&ITfRangeACP>, psink: Option<&ITfMouseSink>) -> windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfMouseTrackerACP {}
impl ITfMouseTrackerACP_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfMouseTrackerACP_Vtbl
    where
        Identity: ITfMouseTrackerACP_Impl,
    {
        unsafe extern "system" fn AdviseMouseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfMouseTrackerACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfMouseTrackerACP_Impl::AdviseMouseSink(this, windows_core::from_raw_borrowed(&range), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ITfMouseTrackerACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMouseTrackerACP_Impl::UnadviseMouseSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseTrackerACP as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfPersistentPropertyLoaderACP_Impl: Sized {
    fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfPersistentPropertyLoaderACP {}
#[cfg(feature = "Win32_System_Com")]
impl ITfPersistentPropertyLoaderACP_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfPersistentPropertyLoaderACP_Vtbl
    where
        Identity: ITfPersistentPropertyLoaderACP_Impl,
    {
        unsafe extern "system" fn LoadProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfPersistentPropertyLoaderACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPersistentPropertyLoaderACP_Impl::LoadProperty(this, core::mem::transmute_copy(&phdr)) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadProperty: LoadProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPersistentPropertyLoaderACP as windows_core::Interface>::IID
    }
}
pub trait ITfPreservedKeyNotifySink_Impl: Sized {
    fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfPreservedKeyNotifySink {}
impl ITfPreservedKeyNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfPreservedKeyNotifySink_Vtbl
    where
        Identity: ITfPreservedKeyNotifySink_Impl,
    {
        unsafe extern "system" fn OnUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT
        where
            Identity: ITfPreservedKeyNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfPreservedKeyNotifySink_Impl::OnUpdated(this, core::mem::transmute_copy(&pprekey)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdated: OnUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPreservedKeyNotifySink as windows_core::Interface>::IID
    }
}
pub trait ITfProperty_Impl: Sized + ITfReadOnlyProperty_Impl {
    fn FindRange(&self, ec: u32, prange: Option<&ITfRange>, pprange: *mut Option<ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn SetValueStore(&self, ec: u32, prange: Option<&ITfRange>, ppropstore: Option<&ITfPropertyStore>) -> windows_core::Result<()>;
    fn SetValue(&self, ec: u32, prange: Option<&ITfRange>, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfProperty {}
impl ITfProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfProperty_Vtbl
    where
        Identity: ITfProperty_Impl,
    {
        unsafe extern "system" fn FindRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT
        where
            Identity: ITfProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::FindRange(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&pprange), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn SetValueStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, ppropstore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::SetValueStore(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), windows_core::from_raw_borrowed(&ppropstore)).into()
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, pvarvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITfProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::SetValue(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::Clear(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)).into()
        }
        Self {
            base__: ITfReadOnlyProperty_Vtbl::new::<Identity, OFFSET>(),
            FindRange: FindRange::<Identity, OFFSET>,
            SetValueStore: SetValueStore::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfProperty as windows_core::Interface>::IID || iid == &<ITfReadOnlyProperty as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfPropertyStore_Impl: Sized {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDataType(&self) -> windows_core::Result<u32>;
    fn GetData(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn OnTextUpdated(&self, dwflags: u32, prangenew: Option<&ITfRange>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Shrink(&self, prangenew: Option<&ITfRange>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Divide(&self, prangethis: Option<&ITfRange>, prangenew: Option<&ITfRange>) -> windows_core::Result<ITfPropertyStore>;
    fn Clone(&self) -> windows_core::Result<ITfPropertyStore>;
    fn GetPropertyRangeCreator(&self) -> windows_core::Result<windows_core::GUID>;
    fn Serialize(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfPropertyStore {}
#[cfg(feature = "Win32_System_Com")]
impl ITfPropertyStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfPropertyStore_Vtbl
    where
        Identity: ITfPropertyStore_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetType(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetDataType(this) {
                Ok(ok__) => {
                    pdwreserved.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetData(this) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTextUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, prangenew: *mut core::ffi::c_void, pfaccept: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::OnTextUpdated(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&prangenew)) {
                Ok(ok__) => {
                    pfaccept.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void, pffree: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Shrink(this, windows_core::from_raw_borrowed(&prangenew)) {
                Ok(ok__) => {
                    pffree.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Divide<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangethis: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void, pppropstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Divide(this, windows_core::from_raw_borrowed(&prangethis), windows_core::from_raw_borrowed(&prangenew)) {
                Ok(ok__) => {
                    pppropstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Clone(this) {
                Ok(ok__) => {
                    ppropstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetPropertyRangeCreator(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Serialize(this, windows_core::from_raw_borrowed(&pstream)) {
                Ok(ok__) => {
                    pcb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDataType: GetDataType::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            OnTextUpdated: OnTextUpdated::<Identity, OFFSET>,
            Shrink: Shrink::<Identity, OFFSET>,
            Divide: Divide::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetPropertyRangeCreator: GetPropertyRangeCreator::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfQueryEmbedded_Impl: Sized {
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfQueryEmbedded {}
#[cfg(feature = "Win32_System_Com")]
impl ITfQueryEmbedded_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfQueryEmbedded_Vtbl
    where
        Identity: ITfQueryEmbedded_Impl,
    {
        unsafe extern "system" fn QueryInsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfQueryEmbedded_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfQueryEmbedded_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfQueryEmbedded as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfRange_Impl: Sized {
    fn GetText(&self, ec: u32, dwflags: u32, pchtext: windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> windows_core::Result<()>;
    fn SetText(&self, ec: u32, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: i32) -> windows_core::Result<()>;
    fn GetFormattedText(&self, ec: u32) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, ec: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn InsertEmbedded(&self, ec: u32, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()>;
    fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()>;
    fn ShiftStartToRange(&self, ec: u32, prange: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn ShiftEndToRange(&self, ec: u32, prange: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEmpty(&self, ec: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Collapse(&self, ec: u32, apos: TfAnchor) -> windows_core::Result<()>;
    fn IsEqualStart(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEqualEnd(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareStart(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<i32>;
    fn CompareEnd(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<i32>;
    fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::Result<()>;
    fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ITfRange>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfRange {}
#[cfg(feature = "Win32_System_Com")]
impl ITfRange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfRange_Vtbl
    where
        Identity: ITfRange_Impl,
    {
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pchtext: windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::GetText(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cchmax), core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pchtext: windows_core::PCWSTR, cch: i32) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::SetText(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::GetFormattedText(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::GetEmbedded(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::InsertEmbedded(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn ShiftStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftStart(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftEnd(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftStartToRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftStartToRange(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftEndToRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftEndToRange(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftStartRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::ShiftStartRegion(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dir)) {
                Ok(ok__) => {
                    pfnoregion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEndRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::ShiftEndRegion(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dir)) {
                Ok(ok__) => {
                    pfnoregion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEmpty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::IsEmpty(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    pfempty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, apos: TfAnchor) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::Collapse(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn IsEqualStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::IsEqualStart(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::IsEqualEnd(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::CompareStart(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::CompareEnd(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustForInsert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::AdjustForInsert(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchinsert)) {
                Ok(ok__) => {
                    pfinsertok.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::GetGravity(this, core::mem::transmute_copy(&pgstart), core::mem::transmute_copy(&pgend)).into()
        }
        unsafe extern "system" fn SetGravity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::SetGravity(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&gstart), core::mem::transmute_copy(&gend)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::Clone(this) {
                Ok(ok__) => {
                    ppclone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfRange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::GetContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            ShiftStart: ShiftStart::<Identity, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, OFFSET>,
            ShiftStartToRange: ShiftStartToRange::<Identity, OFFSET>,
            ShiftEndToRange: ShiftEndToRange::<Identity, OFFSET>,
            ShiftStartRegion: ShiftStartRegion::<Identity, OFFSET>,
            ShiftEndRegion: ShiftEndRegion::<Identity, OFFSET>,
            IsEmpty: IsEmpty::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            IsEqualStart: IsEqualStart::<Identity, OFFSET>,
            IsEqualEnd: IsEqualEnd::<Identity, OFFSET>,
            CompareStart: CompareStart::<Identity, OFFSET>,
            CompareEnd: CompareEnd::<Identity, OFFSET>,
            AdjustForInsert: AdjustForInsert::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            SetGravity: SetGravity::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfRangeACP_Impl: Sized + ITfRange_Impl {
    fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::Result<()>;
    fn SetExtent(&self, acpanchor: i32, cch: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfRangeACP {}
#[cfg(feature = "Win32_System_Com")]
impl ITfRangeACP_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfRangeACP_Vtbl
    where
        Identity: ITfRangeACP_Impl,
    {
        unsafe extern "system" fn GetExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITfRangeACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRangeACP_Impl::GetExtent(this, core::mem::transmute_copy(&pacpanchor), core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpanchor: i32, cch: i32) -> windows_core::HRESULT
        where
            Identity: ITfRangeACP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRangeACP_Impl::SetExtent(this, core::mem::transmute_copy(&acpanchor), core::mem::transmute_copy(&cch)).into()
        }
        Self { base__: ITfRange_Vtbl::new::<Identity, OFFSET>(), GetExtent: GetExtent::<Identity, OFFSET>, SetExtent: SetExtent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRangeACP as windows_core::Interface>::IID || iid == &<ITfRange as windows_core::Interface>::IID
    }
}
pub trait ITfRangeBackup_Impl: Sized {
    fn Restore(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfRangeBackup {}
impl ITfRangeBackup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfRangeBackup_Vtbl
    where
        Identity: ITfRangeBackup_Impl,
    {
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfRangeBackup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRangeBackup_Impl::Restore(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Restore: Restore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRangeBackup as windows_core::Interface>::IID
    }
}
pub trait ITfReadOnlyProperty_Impl: Sized {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn EnumRanges(&self, ec: u32, ppenum: *mut Option<IEnumTfRanges>, ptargetrange: Option<&ITfRange>) -> windows_core::Result<()>;
    fn GetValue(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<windows_core::VARIANT>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
}
impl windows_core::RuntimeName for ITfReadOnlyProperty {}
impl ITfReadOnlyProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfReadOnlyProperty_Vtbl
    where
        Identity: ITfReadOnlyProperty_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfReadOnlyProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadOnlyProperty_Impl::GetType(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppenum: *mut *mut core::ffi::c_void, ptargetrange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfReadOnlyProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfReadOnlyProperty_Impl::EnumRanges(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ppenum), windows_core::from_raw_borrowed(&ptargetrange)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITfReadOnlyProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadOnlyProperty_Impl::GetValue(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfReadOnlyProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadOnlyProperty_Impl::GetContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            EnumRanges: EnumRanges::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReadOnlyProperty as windows_core::Interface>::IID
    }
}
pub trait ITfReadingInformationUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> windows_core::Result<u32>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetMaxReadingStringLength(&self) -> windows_core::Result<u32>;
    fn GetErrorIndex(&self) -> windows_core::Result<u32>;
    fn IsVerticalOrderPreferred(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfReadingInformationUIElement {}
impl ITfReadingInformationUIElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfReadingInformationUIElement_Vtbl
    where
        Identity: ITfReadingInformationUIElement_Impl,
    {
        unsafe extern "system" fn GetUpdatedFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfReadingInformationUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetUpdatedFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfReadingInformationUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetContext(this) {
                Ok(ok__) => {
                    ppic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfReadingInformationUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetString(this) {
                Ok(ok__) => {
                    pstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchmax: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfReadingInformationUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetMaxReadingStringLength(this) {
                Ok(ok__) => {
                    pcchmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfReadingInformationUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetErrorIndex(this) {
                Ok(ok__) => {
                    perrorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfReadingInformationUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::IsVerticalOrderPreferred(this) {
                Ok(ok__) => {
                    pfvertical.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetMaxReadingStringLength: GetMaxReadingStringLength::<Identity, OFFSET>,
            GetErrorIndex: GetErrorIndex::<Identity, OFFSET>,
            IsVerticalOrderPreferred: IsVerticalOrderPreferred::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReadingInformationUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
pub trait ITfReverseConversion_Impl: Sized {
    fn DoReverseConversion(&self, lpstr: &windows_core::PCWSTR) -> windows_core::Result<ITfReverseConversionList>;
}
impl windows_core::RuntimeName for ITfReverseConversion {}
impl ITfReverseConversion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfReverseConversion_Vtbl
    where
        Identity: ITfReverseConversion_Impl,
    {
        unsafe extern "system" fn DoReverseConversion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstr: windows_core::PCWSTR, pplist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfReverseConversion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversion_Impl::DoReverseConversion(this, core::mem::transmute(&lpstr)) {
                Ok(ok__) => {
                    pplist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoReverseConversion: DoReverseConversion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversion as windows_core::Interface>::IID
    }
}
pub trait ITfReverseConversionList_Impl: Sized {
    fn GetLength(&self) -> windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITfReverseConversionList {}
impl ITfReverseConversionList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfReverseConversionList_Vtbl
    where
        Identity: ITfReverseConversionList_Impl,
    {
        unsafe extern "system" fn GetLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfReverseConversionList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversionList_Impl::GetLength(this) {
                Ok(ok__) => {
                    puindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfReverseConversionList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversionList_Impl::GetString(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionList as windows_core::Interface>::IID
    }
}
pub trait ITfReverseConversionMgr_Impl: Sized {
    fn GetReverseConversion(&self, langid: u16, guidprofile: *const windows_core::GUID, dwflag: u32) -> windows_core::Result<ITfReverseConversion>;
}
impl windows_core::RuntimeName for ITfReverseConversionMgr {}
impl ITfReverseConversionMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfReverseConversionMgr_Vtbl
    where
        Identity: ITfReverseConversionMgr_Impl,
    {
        unsafe extern "system" fn GetReverseConversion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, guidprofile: *const windows_core::GUID, dwflag: u32, ppreverseconversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfReverseConversionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversionMgr_Impl::GetReverseConversion(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&dwflag)) {
                Ok(ok__) => {
                    ppreverseconversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetReverseConversion: GetReverseConversion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionMgr as windows_core::Interface>::IID
    }
}
pub trait ITfSource_Impl: Sized {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn UnadviseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfSource {}
impl ITfSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfSource_Vtbl
    where
        Identity: ITfSource_Impl,
    {
        unsafe extern "system" fn AdviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfSource_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ITfSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSource_Impl::UnadviseSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSource as windows_core::Interface>::IID
    }
}
pub trait ITfSourceSingle_Impl: Sized {
    fn AdviseSingleSink(&self, tid: u32, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UnadviseSingleSink(&self, tid: u32, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfSourceSingle {}
impl ITfSourceSingle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfSourceSingle_Vtbl
    where
        Identity: ITfSourceSingle_Impl,
    {
        unsafe extern "system" fn AdviseSingleSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfSourceSingle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSourceSingle_Impl::AdviseSingleSink(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn UnadviseSingleSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, riid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfSourceSingle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSourceSingle_Impl::UnadviseSingleSink(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSingleSink: AdviseSingleSink::<Identity, OFFSET>,
            UnadviseSingleSink: UnadviseSingleSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSourceSingle as windows_core::Interface>::IID
    }
}
pub trait ITfSpeechUIServer_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn ShowUI(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfSpeechUIServer {}
impl ITfSpeechUIServer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfSpeechUIServer_Vtbl
    where
        Identity: ITfSpeechUIServer_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfSpeechUIServer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSpeechUIServer_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn ShowUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfSpeechUIServer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSpeechUIServer_Impl::ShowUI(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UpdateBalloon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, style: TfLBBalloonStyle, pch: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT
        where
            Identity: ITfSpeechUIServer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSpeechUIServer_Impl::UpdateBalloon(this, core::mem::transmute_copy(&style), core::mem::transmute(&pch), core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ShowUI: ShowUI::<Identity, OFFSET>,
            UpdateBalloon: UpdateBalloon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSpeechUIServer as windows_core::Interface>::IID
    }
}
pub trait ITfStatusSink_Impl: Sized {
    fn OnStatusChange(&self, pic: Option<&ITfContext>, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfStatusSink {}
impl ITfStatusSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfStatusSink_Vtbl
    where
        Identity: ITfStatusSink_Impl,
    {
        unsafe extern "system" fn OnStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfStatusSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfStatusSink_Impl::OnStatusChange(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatusChange: OnStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfStatusSink as windows_core::Interface>::IID
    }
}
pub trait ITfSystemDeviceTypeLangBarItem_Impl: Sized {
    fn SetIconMode(&self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> windows_core::Result<()>;
    fn GetIconMode(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ITfSystemDeviceTypeLangBarItem {}
impl ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfSystemDeviceTypeLangBarItem_Vtbl
    where
        Identity: ITfSystemDeviceTypeLangBarItem_Impl,
    {
        unsafe extern "system" fn SetIconMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> windows_core::HRESULT
        where
            Identity: ITfSystemDeviceTypeLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemDeviceTypeLangBarItem_Impl::SetIconMode(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetIconMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfSystemDeviceTypeLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfSystemDeviceTypeLangBarItem_Impl::GetIconMode(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIconMode: SetIconMode::<Identity, OFFSET>,
            GetIconMode: GetIconMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemDeviceTypeLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfSystemLangBarItem_Impl: Sized {
    fn SetIcon(&self, hicon: super::WindowsAndMessaging::HICON) -> windows_core::Result<()>;
    fn SetTooltipString(&self, pchtooltip: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ITfSystemLangBarItem {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ITfSystemLangBarItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfSystemLangBarItem_Vtbl
    where
        Identity: ITfSystemLangBarItem_Impl,
    {
        unsafe extern "system" fn SetIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> windows_core::HRESULT
        where
            Identity: ITfSystemLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItem_Impl::SetIcon(this, core::mem::transmute_copy(&hicon)).into()
        }
        unsafe extern "system" fn SetTooltipString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchtooltip: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT
        where
            Identity: ITfSystemLangBarItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItem_Impl::SetTooltipString(this, core::mem::transmute(&pchtooltip), core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIcon: SetIcon::<Identity, OFFSET>,
            SetTooltipString: SetTooltipString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItem as windows_core::Interface>::IID
    }
}
pub trait ITfSystemLangBarItemSink_Impl: Sized {
    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfSystemLangBarItemSink {}
impl ITfSystemLangBarItemSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfSystemLangBarItemSink_Vtbl
    where
        Identity: ITfSystemLangBarItemSink_Impl,
    {
        unsafe extern "system" fn InitMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfSystemLangBarItemSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItemSink_Impl::InitMenu(this, windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: u32) -> windows_core::HRESULT
        where
            Identity: ITfSystemLangBarItemSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItemSink_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemSink as windows_core::Interface>::IID
    }
}
pub trait ITfSystemLangBarItemText_Impl: Sized {
    fn SetItemText(&self, pch: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn GetItemText(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITfSystemLangBarItemText {}
impl ITfSystemLangBarItemText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfSystemLangBarItemText_Vtbl
    where
        Identity: ITfSystemLangBarItemText_Impl,
    {
        unsafe extern "system" fn SetItemText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pch: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT
        where
            Identity: ITfSystemLangBarItemText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItemText_Impl::SetItemText(this, core::mem::transmute(&pch), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetItemText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfSystemLangBarItemText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfSystemLangBarItemText_Impl::GetItemText(this) {
                Ok(ok__) => {
                    pbstrtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetItemText: SetItemText::<Identity, OFFSET>,
            GetItemText: GetItemText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemText as windows_core::Interface>::IID
    }
}
pub trait ITfTextEditSink_Impl: Sized {
    fn OnEndEdit(&self, pic: Option<&ITfContext>, ecreadonly: u32, peditrecord: Option<&ITfEditRecord>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfTextEditSink {}
impl ITfTextEditSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfTextEditSink_Vtbl
    where
        Identity: ITfTextEditSink_Impl,
    {
        unsafe extern "system" fn OnEndEdit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, ecreadonly: u32, peditrecord: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfTextEditSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextEditSink_Impl::OnEndEdit(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&ecreadonly), windows_core::from_raw_borrowed(&peditrecord)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEndEdit: OnEndEdit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextEditSink as windows_core::Interface>::IID
    }
}
pub trait ITfTextInputProcessor_Impl: Sized {
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfTextInputProcessor {}
impl ITfTextInputProcessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfTextInputProcessor_Vtbl
    where
        Identity: ITfTextInputProcessor_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptim: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT
        where
            Identity: ITfTextInputProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextInputProcessor_Impl::Activate(this, windows_core::from_raw_borrowed(&ptim), core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfTextInputProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextInputProcessor_Impl::Deactivate(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessor as windows_core::Interface>::IID
    }
}
pub trait ITfTextInputProcessorEx_Impl: Sized + ITfTextInputProcessor_Impl {
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfTextInputProcessorEx {}
impl ITfTextInputProcessorEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfTextInputProcessorEx_Vtbl
    where
        Identity: ITfTextInputProcessorEx_Impl,
    {
        unsafe extern "system" fn ActivateEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptim: *mut core::ffi::c_void, tid: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfTextInputProcessorEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextInputProcessorEx_Impl::ActivateEx(this, windows_core::from_raw_borrowed(&ptim), core::mem::transmute_copy(&tid), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ITfTextInputProcessor_Vtbl::new::<Identity, OFFSET>(), ActivateEx: ActivateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessorEx as windows_core::Interface>::IID || iid == &<ITfTextInputProcessor as windows_core::Interface>::IID
    }
}
pub trait ITfTextLayoutSink_Impl: Sized {
    fn OnLayoutChange(&self, pic: Option<&ITfContext>, lcode: TfLayoutCode, pview: Option<&ITfContextView>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfTextLayoutSink {}
impl ITfTextLayoutSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfTextLayoutSink_Vtbl
    where
        Identity: ITfTextLayoutSink_Impl,
    {
        unsafe extern "system" fn OnLayoutChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, lcode: TfLayoutCode, pview: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfTextLayoutSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextLayoutSink_Impl::OnLayoutChange(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&lcode), windows_core::from_raw_borrowed(&pview)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLayoutChange: OnLayoutChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextLayoutSink as windows_core::Interface>::IID
    }
}
pub trait ITfThreadFocusSink_Impl: Sized {
    fn OnSetThreadFocus(&self) -> windows_core::Result<()>;
    fn OnKillThreadFocus(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfThreadFocusSink {}
impl ITfThreadFocusSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfThreadFocusSink_Vtbl
    where
        Identity: ITfThreadFocusSink_Impl,
    {
        unsafe extern "system" fn OnSetThreadFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadFocusSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadFocusSink_Impl::OnSetThreadFocus(this).into()
        }
        unsafe extern "system" fn OnKillThreadFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadFocusSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadFocusSink_Impl::OnKillThreadFocus(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetThreadFocus: OnSetThreadFocus::<Identity, OFFSET>,
            OnKillThreadFocus: OnKillThreadFocus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadFocusSink as windows_core::Interface>::IID
    }
}
pub trait ITfThreadMgr_Impl: Sized {
    fn Activate(&self) -> windows_core::Result<u32>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn AssociateFocus(&self, hwnd: super::super::Foundation::HWND, pdimnew: Option<&ITfDocumentMgr>) -> windows_core::Result<ITfDocumentMgr>;
    fn IsThreadFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr>;
}
impl windows_core::RuntimeName for ITfThreadMgr {}
impl ITfThreadMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfThreadMgr_Vtbl
    where
        Identity: ITfThreadMgr_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::Activate(this) {
                Ok(ok__) => {
                    ptid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr_Impl::Deactivate(this).into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::CreateDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::EnumDocumentMgrs(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdimfocus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::GetFocus(this) {
                Ok(ok__) => {
                    ppdimfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr_Impl::SetFocus(this, windows_core::from_raw_borrowed(&pdimfocus)).into()
        }
        unsafe extern "system" fn AssociateFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: *mut core::ffi::c_void, ppdimprev: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::AssociateFocus(this, core::mem::transmute_copy(&hwnd), windows_core::from_raw_borrowed(&pdimnew)) {
                Ok(ok__) => {
                    ppdimprev.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::IsThreadFocus(this) {
                Ok(ok__) => {
                    pfthreadfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, ppfuncprov: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::GetFunctionProvider(this, core::mem::transmute_copy(&clsid)) {
                Ok(ok__) => {
                    ppfuncprov.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::EnumFunctionProviders(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcompmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::GetGlobalCompartment(this) {
                Ok(ok__) => {
                    ppcompmgr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            AssociateFocus: AssociateFocus::<Identity, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr as windows_core::Interface>::IID
    }
}
pub trait ITfThreadMgr2_Impl: Sized {
    fn Activate(&self) -> windows_core::Result<u32>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn IsThreadFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr>;
    fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveFlags(&self) -> windows_core::Result<u32>;
    fn SuspendKeystrokeHandling(&self) -> windows_core::Result<()>;
    fn ResumeKeystrokeHandling(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfThreadMgr2 {}
impl ITfThreadMgr2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfThreadMgr2_Vtbl
    where
        Identity: ITfThreadMgr2_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::Activate(this) {
                Ok(ok__) => {
                    ptid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::Deactivate(this).into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::CreateDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::EnumDocumentMgrs(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdimfocus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetFocus(this) {
                Ok(ok__) => {
                    ppdimfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::SetFocus(this, windows_core::from_raw_borrowed(&pdimfocus)).into()
        }
        unsafe extern "system" fn IsThreadFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::IsThreadFocus(this) {
                Ok(ok__) => {
                    pfthreadfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, ppfuncprov: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetFunctionProvider(this, core::mem::transmute_copy(&clsid)) {
                Ok(ok__) => {
                    ppfuncprov.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::EnumFunctionProviders(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcompmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetGlobalCompartment(this) {
                Ok(ok__) => {
                    ppcompmgr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::ActivateEx(this, core::mem::transmute_copy(&ptid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetActiveFlags(this) {
                Ok(ok__) => {
                    lpdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::SuspendKeystrokeHandling(this).into()
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::ResumeKeystrokeHandling(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, OFFSET>,
            ActivateEx: ActivateEx::<Identity, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, OFFSET>,
            SuspendKeystrokeHandling: SuspendKeystrokeHandling::<Identity, OFFSET>,
            ResumeKeystrokeHandling: ResumeKeystrokeHandling::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr2 as windows_core::Interface>::IID
    }
}
pub trait ITfThreadMgrEventSink_Impl: Sized {
    fn OnInitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnUninitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnSetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>, pdimprevfocus: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnPushContext(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
    fn OnPopContext(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfThreadMgrEventSink {}
impl ITfThreadMgrEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfThreadMgrEventSink_Vtbl
    where
        Identity: ITfThreadMgrEventSink_Impl,
    {
        unsafe extern "system" fn OnInitDocumentMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdim: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgrEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnInitDocumentMgr(this, windows_core::from_raw_borrowed(&pdim)).into()
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdim: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgrEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnUninitDocumentMgr(this, windows_core::from_raw_borrowed(&pdim)).into()
        }
        unsafe extern "system" fn OnSetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void, pdimprevfocus: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgrEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnSetFocus(this, windows_core::from_raw_borrowed(&pdimfocus), windows_core::from_raw_borrowed(&pdimprevfocus)).into()
        }
        unsafe extern "system" fn OnPushContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgrEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnPushContext(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn OnPopContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgrEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnPopContext(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInitDocumentMgr: OnInitDocumentMgr::<Identity, OFFSET>,
            OnUninitDocumentMgr: OnUninitDocumentMgr::<Identity, OFFSET>,
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnPushContext: OnPushContext::<Identity, OFFSET>,
            OnPopContext: OnPopContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEventSink as windows_core::Interface>::IID
    }
}
pub trait ITfThreadMgrEx_Impl: Sized + ITfThreadMgr_Impl {
    fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveFlags(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ITfThreadMgrEx {}
impl ITfThreadMgrEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfThreadMgrEx_Vtbl
    where
        Identity: ITfThreadMgrEx_Impl,
    {
        unsafe extern "system" fn ActivateEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgrEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEx_Impl::ActivateEx(this, core::mem::transmute_copy(&ptid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfThreadMgrEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgrEx_Impl::GetActiveFlags(this) {
                Ok(ok__) => {
                    lpdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfThreadMgr_Vtbl::new::<Identity, OFFSET>(),
            ActivateEx: ActivateEx::<Identity, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEx as windows_core::Interface>::IID || iid == &<ITfThreadMgr as windows_core::Interface>::IID
    }
}
pub trait ITfToolTipUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITfToolTipUIElement {}
impl ITfToolTipUIElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfToolTipUIElement_Vtbl
    where
        Identity: ITfToolTipUIElement_Impl,
    {
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfToolTipUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfToolTipUIElement_Impl::GetString(this) {
                Ok(ok__) => {
                    pstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(), GetString: GetString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfToolTipUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
pub trait ITfTransitoryExtensionSink_Impl: Sized {
    fn OnTransitoryExtensionUpdated(&self, pic: Option<&ITfContext>, ecreadonly: u32, presultrange: Option<&ITfRange>, pcompositionrange: Option<&ITfRange>) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfTransitoryExtensionSink {}
impl ITfTransitoryExtensionSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfTransitoryExtensionSink_Vtbl
    where
        Identity: ITfTransitoryExtensionSink_Impl,
    {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, ecreadonly: u32, presultrange: *mut core::ffi::c_void, pcompositionrange: *mut core::ffi::c_void, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfTransitoryExtensionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfTransitoryExtensionSink_Impl::OnTransitoryExtensionUpdated(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&ecreadonly), windows_core::from_raw_borrowed(&presultrange), windows_core::from_raw_borrowed(&pcompositionrange)) {
                Ok(ok__) => {
                    pfdeleteresultrange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTransitoryExtensionUpdated: OnTransitoryExtensionUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionSink as windows_core::Interface>::IID
    }
}
pub trait ITfTransitoryExtensionUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
}
impl windows_core::RuntimeName for ITfTransitoryExtensionUIElement {}
impl ITfTransitoryExtensionUIElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfTransitoryExtensionUIElement_Vtbl
    where
        Identity: ITfTransitoryExtensionUIElement_Impl,
    {
        unsafe extern "system" fn GetDocumentMgr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfTransitoryExtensionUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfTransitoryExtensionUIElement_Impl::GetDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(), GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
pub trait ITfUIElement_Impl: Sized {
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn Show(&self, bshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsShown(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITfUIElement {}
impl ITfUIElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfUIElement_Vtbl
    where
        Identity: ITfUIElement_Impl,
    {
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITfUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElement_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITfUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElement_Impl::GetGUID(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElement_Impl::Show(this, core::mem::transmute_copy(&bshow)).into()
        }
        unsafe extern "system" fn IsShown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfUIElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElement_Impl::IsShown(this) {
                Ok(ok__) => {
                    pbshow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            IsShown: IsShown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
pub trait ITfUIElementMgr_Impl: Sized {
    fn BeginUIElement(&self, pelement: Option<&ITfUIElement>, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn GetUIElement(&self, dwuielementid: u32) -> windows_core::Result<ITfUIElement>;
    fn EnumUIElements(&self) -> windows_core::Result<IEnumTfUIElements>;
}
impl windows_core::RuntimeName for ITfUIElementMgr {}
impl ITfUIElementMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfUIElementMgr_Vtbl
    where
        Identity: ITfUIElementMgr_Impl,
    {
        unsafe extern "system" fn BeginUIElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITfUIElementMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementMgr_Impl::BeginUIElement(this, windows_core::from_raw_borrowed(&pelement), core::mem::transmute_copy(&pbshow), core::mem::transmute_copy(&pdwuielementid)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT
        where
            Identity: ITfUIElementMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementMgr_Impl::UpdateUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT
        where
            Identity: ITfUIElementMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementMgr_Impl::EndUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn GetUIElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32, ppelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfUIElementMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElementMgr_Impl::GetUIElement(this, core::mem::transmute_copy(&dwuielementid)) {
                Ok(ok__) => {
                    ppelement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumUIElements<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITfUIElementMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElementMgr_Impl::EnumUIElements(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUIElement: BeginUIElement::<Identity, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, OFFSET>,
            EndUIElement: EndUIElement::<Identity, OFFSET>,
            GetUIElement: GetUIElement::<Identity, OFFSET>,
            EnumUIElements: EnumUIElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElementMgr as windows_core::Interface>::IID
    }
}
pub trait ITfUIElementSink_Impl: Sized {
    fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITfUIElementSink {}
impl ITfUIElementSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITfUIElementSink_Vtbl
    where
        Identity: ITfUIElementSink_Impl,
    {
        unsafe extern "system" fn BeginUIElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITfUIElementSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementSink_Impl::BeginUIElement(this, core::mem::transmute_copy(&dwuielementid), core::mem::transmute_copy(&pbshow)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT
        where
            Identity: ITfUIElementSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementSink_Impl::UpdateUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT
        where
            Identity: ITfUIElementSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementSink_Impl::EndUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUIElement: BeginUIElement::<Identity, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, OFFSET>,
            EndUIElement: EndUIElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElementSink as windows_core::Interface>::IID
    }
}
pub trait IUIManagerEventSink_Impl: Sized {
    fn OnWindowOpening(&self, prcbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowOpened(&self, prcbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowUpdating(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowUpdated(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowClosing(&self) -> windows_core::Result<()>;
    fn OnWindowClosed(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIManagerEventSink {}
impl IUIManagerEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUIManagerEventSink_Vtbl
    where
        Identity: IUIManagerEventSink_Impl,
    {
        unsafe extern "system" fn OnWindowOpening<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IUIManagerEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowOpening(this, core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowOpened<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IUIManagerEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowOpened(this, core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdating<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IUIManagerEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowUpdating(this, core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IUIManagerEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowUpdated(this, core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowClosing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUIManagerEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowClosing(this).into()
        }
        unsafe extern "system" fn OnWindowClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUIManagerEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowClosed(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnWindowOpening: OnWindowOpening::<Identity, OFFSET>,
            OnWindowOpened: OnWindowOpened::<Identity, OFFSET>,
            OnWindowUpdating: OnWindowUpdating::<Identity, OFFSET>,
            OnWindowUpdated: OnWindowUpdated::<Identity, OFFSET>,
            OnWindowClosing: OnWindowClosing::<Identity, OFFSET>,
            OnWindowClosed: OnWindowClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIManagerEventSink as windows_core::Interface>::IID
    }
}
pub trait IVersionInfo_Impl: Sized {
    fn GetSubcomponentCount(&self, ulsub: u32) -> windows_core::Result<u32>;
    fn GetImplementationID(&self, ulsub: u32) -> windows_core::Result<windows_core::GUID>;
    fn GetBuildVersion(&self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::Result<()>;
    fn GetComponentDescription(&self, ulsub: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetInstanceDescription(&self, ulsub: u32) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IVersionInfo {}
impl IVersionInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVersionInfo_Vtbl
    where
        Identity: IVersionInfo_Impl,
    {
        unsafe extern "system" fn GetSubcomponentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVersionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetSubcomponentCount(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    ulcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplementationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, implid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVersionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetImplementationID(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    implid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuildVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVersionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVersionInfo_Impl::GetBuildVersion(this, core::mem::transmute_copy(&ulsub), core::mem::transmute_copy(&pdwmajor), core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetComponentDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, pimplstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVersionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetComponentDescription(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    pimplstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, pimplstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVersionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetInstanceDescription(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    pimplstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSubcomponentCount: GetSubcomponentCount::<Identity, OFFSET>,
            GetImplementationID: GetImplementationID::<Identity, OFFSET>,
            GetBuildVersion: GetBuildVersion::<Identity, OFFSET>,
            GetComponentDescription: GetComponentDescription::<Identity, OFFSET>,
            GetInstanceDescription: GetInstanceDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVersionInfo as windows_core::Interface>::IID
    }
}
