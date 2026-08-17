#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
pub trait IAccessibleWinSAT_Impl: Sized + super::super::UI::Accessibility::IAccessible_Impl {
    fn SetAccessiblityData(&self, wsname: &windows_core::PCWSTR, wsvalue: &windows_core::PCWSTR, wsdesc: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl windows_core::RuntimeName for IAccessibleWinSAT {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSAT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWinSAT_Impl, const OFFSET: isize>() -> IAccessibleWinSAT_Vtbl {
        unsafe extern "system" fn SetAccessiblityData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWinSAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsname: windows_core::PCWSTR, wsvalue: windows_core::PCWSTR, wsdesc: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAccessibleWinSAT_Impl::SetAccessiblityData(this, core::mem::transmute(&wsname), core::mem::transmute(&wsvalue), core::mem::transmute(&wsdesc)).into()
        }
        Self {
            base__: super::super::UI::Accessibility::IAccessible_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetAccessiblityData: SetAccessiblityData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibleWinSAT as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<super::super::UI::Accessibility::IAccessible as windows_core::Interface>::IID
    }
}
pub trait IInitiateWinSATAssessment_Impl: Sized {
    fn InitiateAssessment(&self, cmdline: &windows_core::PCWSTR, pcallbacks: Option<&IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn InitiateFormalAssessment(&self, pcallbacks: Option<&IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn CancelAssessment(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInitiateWinSATAssessment {}
impl IInitiateWinSATAssessment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>() -> IInitiateWinSATAssessment_Vtbl {
        unsafe extern "system" fn InitiateAssessment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmdline: windows_core::PCWSTR, pcallbacks: *mut core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IInitiateWinSATAssessment_Impl::InitiateAssessment(this, core::mem::transmute(&cmdline), windows_core::from_raw_borrowed(&pcallbacks), core::mem::transmute_copy(&callerhwnd)).into()
        }
        unsafe extern "system" fn InitiateFormalAssessment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbacks: *mut core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IInitiateWinSATAssessment_Impl::InitiateFormalAssessment(this, windows_core::from_raw_borrowed(&pcallbacks), core::mem::transmute_copy(&callerhwnd)).into()
        }
        unsafe extern "system" fn CancelAssessment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IInitiateWinSATAssessment_Impl::CancelAssessment(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitiateAssessment: InitiateAssessment::<Identity, Impl, OFFSET>,
            InitiateFormalAssessment: InitiateFormalAssessment::<Identity, Impl, OFFSET>,
            CancelAssessment: CancelAssessment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitiateWinSATAssessment as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideWinSATAssessmentInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Score(&self) -> windows_core::Result<f32>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProvideWinSATAssessmentInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATAssessmentInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>() -> IProvideWinSATAssessmentInfo_Vtbl {
        unsafe extern "system" fn Score<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, score: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATAssessmentInfo_Impl::Score(this) {
                Ok(ok__) => {
                    core::ptr::write(score, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATAssessmentInfo_Impl::Title(this) {
                Ok(ok__) => {
                    core::ptr::write(title, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATAssessmentInfo_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(description, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Score: Score::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideWinSATAssessmentInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideWinSATResultsInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> windows_core::Result<IProvideWinSATAssessmentInfo>;
    fn AssessmentState(&self) -> windows_core::Result<WINSAT_ASSESSMENT_STATE>;
    fn AssessmentDateTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SystemRating(&self) -> windows_core::Result<f32>;
    fn RatingStateDesc(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProvideWinSATResultsInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATResultsInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>() -> IProvideWinSATResultsInfo_Vtbl {
        unsafe extern "system" fn GetAssessmentInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATResultsInfo_Impl::GetAssessmentInfo(this, core::mem::transmute_copy(&assessment)) {
                Ok(ok__) => {
                    core::ptr::write(ppinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATResultsInfo_Impl::AssessmentState(this) {
                Ok(ok__) => {
                    core::ptr::write(state, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentDateTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATResultsInfo_Impl::AssessmentDateTime(this) {
                Ok(ok__) => {
                    core::ptr::write(filetime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRating<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATResultsInfo_Impl::SystemRating(this) {
                Ok(ok__) => {
                    core::ptr::write(level, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RatingStateDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATResultsInfo_Impl::RatingStateDesc(this) {
                Ok(ok__) => {
                    core::ptr::write(description, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAssessmentInfo: GetAssessmentInfo::<Identity, Impl, OFFSET>,
            AssessmentState: AssessmentState::<Identity, Impl, OFFSET>,
            AssessmentDateTime: AssessmentDateTime::<Identity, Impl, OFFSET>,
            SystemRating: SystemRating::<Identity, Impl, OFFSET>,
            RatingStateDesc: RatingStateDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideWinSATResultsInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IProvideWinSATVisuals_Impl: Sized {
    fn get_Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IProvideWinSATVisuals {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IProvideWinSATVisuals_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATVisuals_Impl, const OFFSET: isize>() -> IProvideWinSATVisuals_Vtbl {
        unsafe extern "system" fn get_Bitmap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATVisuals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IProvideWinSATVisuals_Impl::get_Bitmap(this, core::mem::transmute_copy(&bitmapsize), core::mem::transmute_copy(&state), core::mem::transmute_copy(&rating)) {
                Ok(ok__) => {
                    core::ptr::write(pbitmap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_Bitmap: get_Bitmap::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideWinSATVisuals as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IQueryAllWinSATAssessments_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_AllXML(&self, xpath: &windows_core::BSTR, namespaces: &windows_core::BSTR) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IQueryAllWinSATAssessments {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IQueryAllWinSATAssessments_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryAllWinSATAssessments_Impl, const OFFSET: isize>() -> IQueryAllWinSATAssessments_Vtbl {
        unsafe extern "system" fn get_AllXML<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryAllWinSATAssessments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: core::mem::MaybeUninit<windows_core::BSTR>, namespaces: core::mem::MaybeUninit<windows_core::BSTR>, ppdomnodelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IQueryAllWinSATAssessments_Impl::get_AllXML(this, core::mem::transmute(&xpath), core::mem::transmute(&namespaces)) {
                Ok(ok__) => {
                    core::ptr::write(ppdomnodelist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), get_AllXML: get_AllXML::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryAllWinSATAssessments as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IQueryOEMWinSATCustomization_Impl: Sized {
    fn GetOEMPrePopulationInfo(&self) -> windows_core::Result<WINSAT_OEM_CUSTOMIZATION_STATE>;
}
impl windows_core::RuntimeName for IQueryOEMWinSATCustomization {}
impl IQueryOEMWinSATCustomization_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryOEMWinSATCustomization_Impl, const OFFSET: isize>() -> IQueryOEMWinSATCustomization_Vtbl {
        unsafe extern "system" fn GetOEMPrePopulationInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryOEMWinSATCustomization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut WINSAT_OEM_CUSTOMIZATION_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IQueryOEMWinSATCustomization_Impl::GetOEMPrePopulationInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(state, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOEMPrePopulationInfo: GetOEMPrePopulationInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryOEMWinSATCustomization as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IQueryRecentWinSATAssessment_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_XML(&self, xpath: &windows_core::BSTR, namespaces: &windows_core::BSTR) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
    fn Info(&self) -> windows_core::Result<IProvideWinSATResultsInfo>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IQueryRecentWinSATAssessment {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IQueryRecentWinSATAssessment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>() -> IQueryRecentWinSATAssessment_Vtbl {
        unsafe extern "system" fn get_XML<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: core::mem::MaybeUninit<windows_core::BSTR>, namespaces: core::mem::MaybeUninit<windows_core::BSTR>, ppdomnodelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IQueryRecentWinSATAssessment_Impl::get_XML(this, core::mem::transmute(&xpath), core::mem::transmute(&namespaces)) {
                Ok(ok__) => {
                    core::ptr::write(ppdomnodelist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwinsatassessmentinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IQueryRecentWinSATAssessment_Impl::Info(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwinsatassessmentinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_XML: get_XML::<Identity, Impl, OFFSET>,
            Info: Info::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryRecentWinSATAssessment as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWinSATInitiateEvents_Impl: Sized {
    fn WinSATComplete(&self, hresult: windows_core::HRESULT, strdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WinSATUpdate(&self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWinSATInitiateEvents {}
impl IWinSATInitiateEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: isize>() -> IWinSATInitiateEvents_Vtbl {
        unsafe extern "system" fn WinSATComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, strdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWinSATInitiateEvents_Impl::WinSATComplete(this, core::mem::transmute_copy(&hresult), core::mem::transmute(&strdescription)).into()
        }
        unsafe extern "system" fn WinSATUpdate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWinSATInitiateEvents_Impl::WinSATUpdate(this, core::mem::transmute_copy(&ucurrenttick), core::mem::transmute_copy(&uticktotal), core::mem::transmute(&strcurrentstate)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WinSATComplete: WinSATComplete::<Identity, Impl, OFFSET>,
            WinSATUpdate: WinSATUpdate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinSATInitiateEvents as windows_core::Interface>::IID
    }
}
