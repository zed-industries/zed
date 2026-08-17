pub const CAccessiblityWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0x6e18f9c6_a3eb_495a_89b7_956482e19f7a);
pub const CInitiateWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0x489331dc_f5e0_4528_9fda_45331bf4a571);
pub const CProvideWinSATVisuals: windows_core::GUID = windows_core::GUID::from_u128(0x9f377d7e_e551_44f8_9f94_9db392b03b7b);
pub const CQueryAllWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0x05df8d13_c355_47f4_a11e_851b338cefb8);
pub const CQueryOEMWinSATCustomization: windows_core::GUID = windows_core::GUID::from_u128(0xc47a41b7_b729_424f_9af9_5cb3934f2dfa);
pub const CQueryWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0xf3bdfad3_f276_49e9_9b17_c474f48f0764);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
windows_core::imp::define_interface!(IAccessibleWinSAT, IAccessibleWinSAT_Vtbl, 0x30e6018a_94a8_4ff8_a69a_71b67413f07b);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl core::ops::Deref for IAccessibleWinSAT {
    type Target = super::super::UI::Accessibility::IAccessible;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
windows_core::imp::interface_hierarchy!(IAccessibleWinSAT, windows_core::IUnknown, super::Com::IDispatch, super::super::UI::Accessibility::IAccessible);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSAT {
    pub unsafe fn SetAccessiblityData<P0, P1, P2>(&self, wsname: P0, wsvalue: P1, wsdesc: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccessiblityData)(windows_core::Interface::as_raw(self), wsname.param().abi(), wsvalue.param().abi(), wsdesc.param().abi()).ok() }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleWinSAT_Vtbl {
    pub base__: super::super::UI::Accessibility::IAccessible_Vtbl,
    pub SetAccessiblityData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
pub trait IAccessibleWinSAT_Impl: super::super::UI::Accessibility::IAccessible_Impl {
    fn SetAccessiblityData(&self, wsname: &windows_core::PCWSTR, wsvalue: &windows_core::PCWSTR, wsdesc: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSAT_Vtbl {
    pub const fn new<Identity: IAccessibleWinSAT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAccessiblityData<Identity: IAccessibleWinSAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsname: windows_core::PCWSTR, wsvalue: windows_core::PCWSTR, wsdesc: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibleWinSAT_Impl::SetAccessiblityData(this, core::mem::transmute(&wsname), core::mem::transmute(&wsvalue), core::mem::transmute(&wsdesc)).into()
            }
        }
        Self {
            base__: super::super::UI::Accessibility::IAccessible_Vtbl::new::<Identity, OFFSET>(),
            SetAccessiblityData: SetAccessiblityData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibleWinSAT as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<super::super::UI::Accessibility::IAccessible as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
impl windows_core::RuntimeName for IAccessibleWinSAT {}
windows_core::imp::define_interface!(IInitiateWinSATAssessment, IInitiateWinSATAssessment_Vtbl, 0xd983fc50_f5bf_49d5_b5ed_cccb18aa7fc1);
windows_core::imp::interface_hierarchy!(IInitiateWinSATAssessment, windows_core::IUnknown);
impl IInitiateWinSATAssessment {
    pub unsafe fn InitiateAssessment<P0, P1>(&self, cmdline: P0, pcallbacks: P1, callerhwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWinSATInitiateEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitiateAssessment)(windows_core::Interface::as_raw(self), cmdline.param().abi(), pcallbacks.param().abi(), callerhwnd.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn InitiateFormalAssessment<P0>(&self, pcallbacks: P0, callerhwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWinSATInitiateEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitiateFormalAssessment)(windows_core::Interface::as_raw(self), pcallbacks.param().abi(), callerhwnd.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CancelAssessment(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelAssessment)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitiateWinSATAssessment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitiateAssessment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub InitiateFormalAssessment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub CancelAssessment: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInitiateWinSATAssessment_Impl: windows_core::IUnknownImpl {
    fn InitiateAssessment(&self, cmdline: &windows_core::PCWSTR, pcallbacks: windows_core::Ref<IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn InitiateFormalAssessment(&self, pcallbacks: windows_core::Ref<IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn CancelAssessment(&self) -> windows_core::Result<()>;
}
impl IInitiateWinSATAssessment_Vtbl {
    pub const fn new<Identity: IInitiateWinSATAssessment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitiateAssessment<Identity: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmdline: windows_core::PCWSTR, pcallbacks: *mut core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitiateWinSATAssessment_Impl::InitiateAssessment(this, core::mem::transmute(&cmdline), core::mem::transmute_copy(&pcallbacks), core::mem::transmute_copy(&callerhwnd)).into()
            }
        }
        unsafe extern "system" fn InitiateFormalAssessment<Identity: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbacks: *mut core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitiateWinSATAssessment_Impl::InitiateFormalAssessment(this, core::mem::transmute_copy(&pcallbacks), core::mem::transmute_copy(&callerhwnd)).into()
            }
        }
        unsafe extern "system" fn CancelAssessment<Identity: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitiateWinSATAssessment_Impl::CancelAssessment(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitiateAssessment: InitiateAssessment::<Identity, OFFSET>,
            InitiateFormalAssessment: InitiateFormalAssessment::<Identity, OFFSET>,
            CancelAssessment: CancelAssessment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitiateWinSATAssessment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInitiateWinSATAssessment {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IProvideWinSATAssessmentInfo, IProvideWinSATAssessmentInfo_Vtbl, 0x0cd1c380_52d3_4678_ac6f_e929e480be9e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IProvideWinSATAssessmentInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IProvideWinSATAssessmentInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATAssessmentInfo {
    pub unsafe fn Score(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Score)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATAssessmentInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Score: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProvideWinSATAssessmentInfo_Impl: super::Com::IDispatch_Impl {
    fn Score(&self) -> windows_core::Result<f32>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProvideWinSATAssessmentInfo_Vtbl {
    pub const fn new<Identity: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Score<Identity: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, score: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATAssessmentInfo_Impl::Score(this) {
                    Ok(ok__) => {
                        score.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Title<Identity: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATAssessmentInfo_Impl::Title(this) {
                    Ok(ok__) => {
                        title.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATAssessmentInfo_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Score: Score::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideWinSATAssessmentInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IProvideWinSATAssessmentInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IProvideWinSATResultsInfo, IProvideWinSATResultsInfo_Vtbl, 0xf8334d5d_568e_4075_875f_9df341506640);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IProvideWinSATResultsInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IProvideWinSATResultsInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATResultsInfo {
    pub unsafe fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> windows_core::Result<IProvideWinSATAssessmentInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAssessmentInfo)(windows_core::Interface::as_raw(self), assessment, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AssessmentState(&self) -> windows_core::Result<WINSAT_ASSESSMENT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssessmentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AssessmentDateTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssessmentDateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SystemRating(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemRating)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RatingStateDesc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RatingStateDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATResultsInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetAssessmentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, WINSAT_ASSESSMENT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssessmentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WINSAT_ASSESSMENT_STATE) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AssessmentDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AssessmentDateTime: usize,
    pub SystemRating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub RatingStateDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProvideWinSATResultsInfo_Impl: super::Com::IDispatch_Impl {
    fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> windows_core::Result<IProvideWinSATAssessmentInfo>;
    fn AssessmentState(&self) -> windows_core::Result<WINSAT_ASSESSMENT_STATE>;
    fn AssessmentDateTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SystemRating(&self) -> windows_core::Result<f32>;
    fn RatingStateDesc(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProvideWinSATResultsInfo_Vtbl {
    pub const fn new<Identity: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAssessmentInfo<Identity: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATResultsInfo_Impl::GetAssessmentInfo(this, core::mem::transmute_copy(&assessment)) {
                    Ok(ok__) => {
                        ppinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssessmentState<Identity: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATResultsInfo_Impl::AssessmentState(this) {
                    Ok(ok__) => {
                        state.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssessmentDateTime<Identity: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATResultsInfo_Impl::AssessmentDateTime(this) {
                    Ok(ok__) => {
                        filetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SystemRating<Identity: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATResultsInfo_Impl::SystemRating(this) {
                    Ok(ok__) => {
                        level.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RatingStateDesc<Identity: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATResultsInfo_Impl::RatingStateDesc(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetAssessmentInfo: GetAssessmentInfo::<Identity, OFFSET>,
            AssessmentState: AssessmentState::<Identity, OFFSET>,
            AssessmentDateTime: AssessmentDateTime::<Identity, OFFSET>,
            SystemRating: SystemRating::<Identity, OFFSET>,
            RatingStateDesc: RatingStateDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideWinSATResultsInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IProvideWinSATResultsInfo {}
windows_core::imp::define_interface!(IProvideWinSATVisuals, IProvideWinSATVisuals_Vtbl, 0xa9f4ade0_871a_42a3_b813_3078d25162c9);
windows_core::imp::interface_hierarchy!(IProvideWinSATVisuals, windows_core::IUnknown);
impl IProvideWinSATVisuals {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn get_Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Bitmap)(windows_core::Interface::as_raw(self), bitmapsize, state, rating, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATVisuals_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub get_Bitmap: unsafe extern "system" fn(*mut core::ffi::c_void, WINSAT_BITMAP_SIZE, WINSAT_ASSESSMENT_STATE, f32, *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    get_Bitmap: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IProvideWinSATVisuals_Impl: windows_core::IUnknownImpl {
    fn get_Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IProvideWinSATVisuals_Vtbl {
    pub const fn new<Identity: IProvideWinSATVisuals_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Bitmap<Identity: IProvideWinSATVisuals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideWinSATVisuals_Impl::get_Bitmap(this, core::mem::transmute_copy(&bitmapsize), core::mem::transmute_copy(&state), core::mem::transmute_copy(&rating)) {
                    Ok(ok__) => {
                        pbitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_Bitmap: get_Bitmap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideWinSATVisuals as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IProvideWinSATVisuals {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IQueryAllWinSATAssessments, IQueryAllWinSATAssessments_Vtbl, 0x0b89ed1d_6398_4fea_87fc_567d8d19176f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IQueryAllWinSATAssessments {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IQueryAllWinSATAssessments, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IQueryAllWinSATAssessments {
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub unsafe fn get_AllXML(&self, xpath: &windows_core::BSTR, namespaces: &windows_core::BSTR) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllXML)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xpath), core::mem::transmute_copy(namespaces), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryAllWinSATAssessments_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub get_AllXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))]
    get_AllXML: usize,
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IQueryAllWinSATAssessments_Impl: super::Com::IDispatch_Impl {
    fn get_AllXML(&self, xpath: &windows_core::BSTR, namespaces: &windows_core::BSTR) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IQueryAllWinSATAssessments_Vtbl {
    pub const fn new<Identity: IQueryAllWinSATAssessments_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AllXML<Identity: IQueryAllWinSATAssessments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: *mut core::ffi::c_void, namespaces: *mut core::ffi::c_void, ppdomnodelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryAllWinSATAssessments_Impl::get_AllXML(this, core::mem::transmute(&xpath), core::mem::transmute(&namespaces)) {
                    Ok(ok__) => {
                        ppdomnodelist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_AllXML: get_AllXML::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryAllWinSATAssessments as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IQueryAllWinSATAssessments {}
windows_core::imp::define_interface!(IQueryOEMWinSATCustomization, IQueryOEMWinSATCustomization_Vtbl, 0xbc9a6a9f_ad4e_420e_9953_b34671e9df22);
windows_core::imp::interface_hierarchy!(IQueryOEMWinSATCustomization, windows_core::IUnknown);
impl IQueryOEMWinSATCustomization {
    pub unsafe fn GetOEMPrePopulationInfo(&self) -> windows_core::Result<WINSAT_OEM_CUSTOMIZATION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOEMPrePopulationInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOEMWinSATCustomization_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOEMPrePopulationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WINSAT_OEM_CUSTOMIZATION_STATE) -> windows_core::HRESULT,
}
pub trait IQueryOEMWinSATCustomization_Impl: windows_core::IUnknownImpl {
    fn GetOEMPrePopulationInfo(&self) -> windows_core::Result<WINSAT_OEM_CUSTOMIZATION_STATE>;
}
impl IQueryOEMWinSATCustomization_Vtbl {
    pub const fn new<Identity: IQueryOEMWinSATCustomization_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOEMPrePopulationInfo<Identity: IQueryOEMWinSATCustomization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut WINSAT_OEM_CUSTOMIZATION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryOEMWinSATCustomization_Impl::GetOEMPrePopulationInfo(this) {
                    Ok(ok__) => {
                        state.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOEMPrePopulationInfo: GetOEMPrePopulationInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryOEMWinSATCustomization as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IQueryOEMWinSATCustomization {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IQueryRecentWinSATAssessment, IQueryRecentWinSATAssessment_Vtbl, 0xf8ad5d1f_3b47_4bdc_9375_7c6b1da4eca7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IQueryRecentWinSATAssessment {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IQueryRecentWinSATAssessment, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IQueryRecentWinSATAssessment {
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub unsafe fn get_XML(&self, xpath: &windows_core::BSTR, namespaces: &windows_core::BSTR) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_XML)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xpath), core::mem::transmute_copy(namespaces), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Info(&self) -> windows_core::Result<IProvideWinSATResultsInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Info)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryRecentWinSATAssessment_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub get_XML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))]
    get_XML: usize,
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IQueryRecentWinSATAssessment_Impl: super::Com::IDispatch_Impl {
    fn get_XML(&self, xpath: &windows_core::BSTR, namespaces: &windows_core::BSTR) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
    fn Info(&self) -> windows_core::Result<IProvideWinSATResultsInfo>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IQueryRecentWinSATAssessment_Vtbl {
    pub const fn new<Identity: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_XML<Identity: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: *mut core::ffi::c_void, namespaces: *mut core::ffi::c_void, ppdomnodelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryRecentWinSATAssessment_Impl::get_XML(this, core::mem::transmute(&xpath), core::mem::transmute(&namespaces)) {
                    Ok(ok__) => {
                        ppdomnodelist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Info<Identity: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwinsatassessmentinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryRecentWinSATAssessment_Impl::Info(this) {
                    Ok(ok__) => {
                        ppwinsatassessmentinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_XML: get_XML::<Identity, OFFSET>, Info: Info::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryRecentWinSATAssessment as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IQueryRecentWinSATAssessment {}
windows_core::imp::define_interface!(IWinSATInitiateEvents, IWinSATInitiateEvents_Vtbl, 0x262a1918_ba0d_41d5_92c2_fab4633ee74f);
windows_core::imp::interface_hierarchy!(IWinSATInitiateEvents, windows_core::IUnknown);
impl IWinSATInitiateEvents {
    pub unsafe fn WinSATComplete<P1>(&self, hresult: windows_core::HRESULT, strdescription: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WinSATComplete)(windows_core::Interface::as_raw(self), hresult, strdescription.param().abi()).ok() }
    }
    pub unsafe fn WinSATUpdate<P2>(&self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WinSATUpdate)(windows_core::Interface::as_raw(self), ucurrenttick, uticktotal, strcurrentstate.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinSATInitiateEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WinSATComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WinSATUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWinSATInitiateEvents_Impl: windows_core::IUnknownImpl {
    fn WinSATComplete(&self, hresult: windows_core::HRESULT, strdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WinSATUpdate(&self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWinSATInitiateEvents_Vtbl {
    pub const fn new<Identity: IWinSATInitiateEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WinSATComplete<Identity: IWinSATInitiateEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, strdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinSATInitiateEvents_Impl::WinSATComplete(this, core::mem::transmute_copy(&hresult), core::mem::transmute(&strdescription)).into()
            }
        }
        unsafe extern "system" fn WinSATUpdate<Identity: IWinSATInitiateEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinSATInitiateEvents_Impl::WinSATUpdate(this, core::mem::transmute_copy(&ucurrenttick), core::mem::transmute_copy(&uticktotal), core::mem::transmute(&strcurrentstate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WinSATComplete: WinSATComplete::<Identity, OFFSET>,
            WinSATUpdate: WinSATUpdate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinSATInitiateEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinSATInitiateEvents {}
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(2i32);
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(3i32);
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINSAT_OEM_CUSTOMIZATION_STATE(pub i32);
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(2i32);
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(1i32);
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(0i32);
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(3i32);
