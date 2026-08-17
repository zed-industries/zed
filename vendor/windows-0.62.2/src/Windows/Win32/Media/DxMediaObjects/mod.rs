#[inline]
pub unsafe fn DMOEnum(guidcategory: *const windows_core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> windows_core::Result<IEnumDMO> {
    windows_core::link!("msdmo.dll" "system" fn DMOEnum(guidcategory : *const windows_core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE, ppenum : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DMOEnum(guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn DMOGetName(clsiddmo: *const windows_core::GUID, szname: &mut [u16; 80]) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn DMOGetName(clsiddmo : *const windows_core::GUID, szname : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DMOGetName(clsiddmo, core::mem::transmute(szname.as_ptr())).ok() }
}
#[inline]
pub unsafe fn DMOGetTypes(clsiddmo: *const windows_core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn DMOGetTypes(clsiddmo : *const windows_core::GUID, ulinputtypesrequested : u32, pulinputtypessupplied : *mut u32, pinputtypes : *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested : u32, puloutputtypessupplied : *mut u32, poutputtypes : *mut DMO_PARTIAL_MEDIATYPE) -> windows_core::HRESULT);
    unsafe { DMOGetTypes(clsiddmo, ulinputtypesrequested, pulinputtypessupplied as _, pinputtypes as _, uloutputtypesrequested, puloutputtypessupplied as _, poutputtypes as _).ok() }
}
#[inline]
pub unsafe fn DMORegister<P0>(szname: P0, clsiddmo: *const windows_core::GUID, guidcategory: *const windows_core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdmo.dll" "system" fn DMORegister(szname : windows_core::PCWSTR, clsiddmo : *const windows_core::GUID, guidcategory : *const windows_core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE) -> windows_core::HRESULT);
    unsafe { DMORegister(szname.param().abi(), clsiddmo, guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes).ok() }
}
#[inline]
pub unsafe fn DMOUnregister(clsiddmo: *const windows_core::GUID, guidcategory: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn DMOUnregister(clsiddmo : *const windows_core::GUID, guidcategory : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { DMOUnregister(clsiddmo, guidcategory).ok() }
}
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoCopyMediaType(core::mem::transmute(pmtdest), core::mem::transmute(pmtsrc)).ok() }
}
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut DMO_MEDIA_TYPE, cbformat : u32) -> windows_core::HRESULT);
    unsafe { MoCreateMediaType(ppmt as _, cbformat).ok() }
}
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoDeleteMediaType(core::mem::transmute(pmt)).ok() }
}
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoDuplicateMediaType(ppmtdest as _, core::mem::transmute(pmtsrc)).ok() }
}
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MoFreeMediaType(core::mem::transmute(pmt)).ok() }
}
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> windows_core::Result<()> {
    windows_core::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut DMO_MEDIA_TYPE, cbformat : u32) -> windows_core::HRESULT);
    unsafe { MoInitMediaType(core::mem::transmute(pmt), cbformat).ok() }
}
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: windows_core::GUID = windows_core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AGC: windows_core::GUID = windows_core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0xf665aaba_3e09_4920_aa5f_219811148f09);
pub const DMOCATEGORY_AUDIO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x57f2db8b_e6bb_4513_9d43_dcd2a6593125);
pub const DMOCATEGORY_AUDIO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0xf3602b3f_0592_48df_a4cd_674721e7ebeb);
pub const DMOCATEGORY_AUDIO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: windows_core::GUID = windows_core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const DMOCATEGORY_VIDEO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x4a69b442_28be_4991_969c_b500adf5d8a8);
pub const DMOCATEGORY_VIDEO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0xd990ee14_776c_4723_be46_3da2f56f10b9);
pub const DMOCATEGORY_VIDEO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = DMO_ENUM_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DMO_ENUM_FLAGS(pub i32);
pub const DMO_E_INVALIDSTREAMINDEX: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const DMO_E_INVALIDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040202_u32 as _);
pub const DMO_E_NOTACCEPTING: windows_core::HRESULT = windows_core::HRESULT(0x80040204_u32 as _);
pub const DMO_E_NO_MORE_ITEMS: windows_core::HRESULT = windows_core::HRESULT(0x80040206_u32 as _);
pub const DMO_E_TYPE_NOT_ACCEPTED: windows_core::HRESULT = windows_core::HRESULT(0x80040205_u32 as _);
pub const DMO_E_TYPE_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x80040203_u32 as _);
pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(0i32);
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(1i32);
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(8i32);
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(1i32);
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(2i32);
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(4i32);
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = _DMO_INPUT_STATUS_FLAGS(1i32);
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(4i32);
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(8i32);
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(2i32);
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DMO_MEDIA_TYPE {
    pub majortype: windows_core::GUID,
    pub subtype: windows_core::GUID,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: core::mem::ManuallyDrop<Option<IMediaBuffer>>,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(8i32);
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(16777216i32);
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(1i32);
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(2i32);
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(4i32);
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(8i32);
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(4i32);
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(16i32);
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(2i32);
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: windows_core::GUID,
    pub subtype: windows_core::GUID,
}
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = _DMO_PROCESS_OUTPUT_FLAGS(1i32);
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = _DMO_QUALITY_STATUS_FLAGS(1i32);
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = DMO_REGISTER_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DMO_REGISTER_FLAGS(pub i32);
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(2i32);
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(1i32);
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = _DMO_VIDEO_OUTPUT_STREAM_FLAGS(1i32);
windows_core::imp::define_interface!(IDMOQualityControl, IDMOQualityControl_Vtbl, 0x65abea96_cf36_453f_af8a_705e98f16260);
windows_core::imp::interface_hierarchy!(IDMOQualityControl, windows_core::IUnknown);
impl IDMOQualityControl {
    pub unsafe fn SetNow(&self, rtnow: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNow)(windows_core::Interface::as_raw(self), rtnow).ok() }
    }
    pub unsafe fn SetStatus(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOQualityControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetNow: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDMOQualityControl_Impl: windows_core::IUnknownImpl {
    fn SetNow(&self, rtnow: i64) -> windows_core::Result<()>;
    fn SetStatus(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
}
impl IDMOQualityControl_Vtbl {
    pub const fn new<Identity: IDMOQualityControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNow<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtnow: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDMOQualityControl_Impl::SetNow(this, core::mem::transmute_copy(&rtnow)).into()
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDMOQualityControl_Impl::SetStatus(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOQualityControl_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNow: SetNow::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMOQualityControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDMOQualityControl {}
windows_core::imp::define_interface!(IDMOVideoOutputOptimizations, IDMOVideoOutputOptimizations_Vtbl, 0xbe8f4f4e_5b16_4d29_b350_7f6b5d9298ac);
windows_core::imp::interface_hierarchy!(IDMOVideoOutputOptimizations, windows_core::IUnknown);
impl IDMOVideoOutputOptimizations {
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryOperationModePreferences)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOperationMode)(windows_core::Interface::as_raw(self), uloutputstreamindex, dwenabledfeatures).ok() }
    }
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentOperationMode)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSampleRequirements)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOVideoOutputOptimizations_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryOperationModePreferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetOperationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetCurrentOperationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentSampleRequirements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IDMOVideoOutputOptimizations_Impl: windows_core::IUnknownImpl {
    fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
    fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::Result<()>;
    fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
}
impl IDMOVideoOutputOptimizations_Vtbl {
    pub const fn new<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryOperationModePreferences<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOVideoOutputOptimizations_Impl::QueryOperationModePreferences(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                    Ok(ok__) => {
                        pdwrequestedcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOperationMode<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDMOVideoOutputOptimizations_Impl::SetOperationMode(this, core::mem::transmute_copy(&uloutputstreamindex), core::mem::transmute_copy(&dwenabledfeatures)).into()
            }
        }
        unsafe extern "system" fn GetCurrentOperationMode<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOVideoOutputOptimizations_Impl::GetCurrentOperationMode(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                    Ok(ok__) => {
                        pdwenabledfeatures.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDMOVideoOutputOptimizations_Impl::GetCurrentSampleRequirements(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                    Ok(ok__) => {
                        pdwrequestedfeatures.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryOperationModePreferences: QueryOperationModePreferences::<Identity, OFFSET>,
            SetOperationMode: SetOperationMode::<Identity, OFFSET>,
            GetCurrentOperationMode: GetCurrentOperationMode::<Identity, OFFSET>,
            GetCurrentSampleRequirements: GetCurrentSampleRequirements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMOVideoOutputOptimizations as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDMOVideoOutputOptimizations {}
windows_core::imp::define_interface!(IEnumDMO, IEnumDMO_Vtbl, 0x2c3cd98a_2bfa_4a53_9c27_5249ba64ba0f);
windows_core::imp::interface_hierarchy!(IEnumDMO, windows_core::IUnknown);
impl IEnumDMO {
    pub unsafe fn Next(&self, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), citemstofetch, pclsid as _, names as _, pcitemsfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, citemstoskip: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), citemstoskip).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDMO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDMO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDMO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, citemstoskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDMO>;
}
impl IEnumDMO_Vtbl {
    pub const fn new<Identity: IEnumDMO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDMO_Impl::Next(this, core::mem::transmute_copy(&citemstofetch), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&names), core::mem::transmute_copy(&pcitemsfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, citemstoskip: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDMO_Impl::Skip(this, core::mem::transmute_copy(&citemstoskip)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDMO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDMO_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDMO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDMO {}
windows_core::imp::define_interface!(IMediaBuffer, IMediaBuffer_Vtbl, 0x59eff8b9_938c_4a26_82f2_95cb84cdc837);
windows_core::imp::interface_hierarchy!(IMediaBuffer, windows_core::IUnknown);
impl IMediaBuffer {
    pub unsafe fn SetLength(&self, cblength: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), cblength).ok() }
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: Option<*mut *mut u8>, pcblength: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBufferAndLength)(windows_core::Interface::as_raw(self), ppbuffer.unwrap_or(core::mem::zeroed()) as _, pcblength.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IMediaBuffer_Impl: windows_core::IUnknownImpl {
    fn SetLength(&self, cblength: u32) -> windows_core::Result<()>;
    fn GetMaxLength(&self) -> windows_core::Result<u32>;
    fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>;
}
impl IMediaBuffer_Vtbl {
    pub const fn new<Identity: IMediaBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cblength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaBuffer_Impl::SetLength(this, core::mem::transmute_copy(&cblength)).into()
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbmaxlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaBuffer_Impl::GetMaxLength(this) {
                    Ok(ok__) => {
                        pcbmaxlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaBuffer_Impl::GetBufferAndLength(this, core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLength: SetLength::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMediaBuffer {}
windows_core::imp::define_interface!(IMediaObject, IMediaObject_Vtbl, 0xd8ad0f58_5494_4102_97c5_ec798e59bcf4);
windows_core::imp::interface_hierarchy!(IMediaObject, windows_core::IUnknown);
impl IMediaObject {
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pcinputstreams as _, pcoutputstreams as _).ok() }
    }
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamInfo)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamInfo)(windows_core::Interface::as_raw(self), dwoutputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: Option<*mut DMO_MEDIA_TYPE>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputType)(windows_core::Interface::as_raw(self), dwinputstreamindex, dwtypeindex, pmt.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: Option<*mut DMO_MEDIA_TYPE>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, dwtypeindex, pmt.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInputType)(windows_core::Interface::as_raw(self), dwinputstreamindex, pmt.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
    }
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, pmt.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
    }
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputCurrentType)(windows_core::Interface::as_raw(self), dwinputstreamindex, core::mem::transmute(pmt)).ok() }
    }
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputCurrentType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, core::mem::transmute(pmt)).ok() }
    }
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputSizeInfo)(windows_core::Interface::as_raw(self), dwinputstreamindex, pcbsize as _, pcbmaxlookahead as _, pcbalignment as _).ok() }
    }
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputSizeInfo)(windows_core::Interface::as_raw(self), dwoutputstreamindex, pcbsize as _, pcbalignment as _).ok() }
    }
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputMaxLatency)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInputMaxLatency)(windows_core::Interface::as_raw(self), dwinputstreamindex, rtmaxlatency).ok() }
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Discontinuity(&self, dwinputstreamindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Discontinuity)(windows_core::Interface::as_raw(self), dwinputstreamindex).ok() }
    }
    pub unsafe fn AllocateStreamingResources(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllocateStreamingResources)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FreeStreamingResources(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeStreamingResources)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetInputStatus(&self, dwinputstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStatus)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProcessInput<P1>(&self, dwinputstreamindex: u32, pbuffer: P1, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IMediaBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), dwinputstreamindex, pbuffer.param().abi(), dwflags, rttimestamp, rttimelength).ok() }
    }
    pub unsafe fn ProcessOutput(&self, dwflags: u32, poutputbuffers: &mut [DMO_OUTPUT_DATA_BUFFER], pdwstatus: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessOutput)(windows_core::Interface::as_raw(self), dwflags, poutputbuffers.len().try_into().unwrap(), core::mem::transmute(poutputbuffers.as_ptr()), pdwstatus as _).ok() }
    }
    pub unsafe fn Lock(&self, block: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), block).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub GetOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub SetInputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DMO_MEDIA_TYPE, u32) -> windows_core::HRESULT,
    pub SetOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DMO_MEDIA_TYPE, u32) -> windows_core::HRESULT,
    pub GetInputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub GetOutputCurrentType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    pub GetInputSizeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputSizeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputMaxLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i64) -> windows_core::HRESULT,
    pub SetInputMaxLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i64) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AllocateStreamingResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeStreamingResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, i64, i64) -> windows_core::HRESULT,
    pub ProcessOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DMO_OUTPUT_DATA_BUFFER, *mut u32) -> windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IMediaObject_Impl: windows_core::IUnknownImpl {
    fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::Result<()>;
    fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn SetInputType(&self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::Result<()>;
    fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::Result<()>;
    fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()>;
    fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()>;
    fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> windows_core::Result<i64>;
    fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn Discontinuity(&self, dwinputstreamindex: u32) -> windows_core::Result<()>;
    fn AllocateStreamingResources(&self) -> windows_core::Result<()>;
    fn FreeStreamingResources(&self) -> windows_core::Result<()>;
    fn GetInputStatus(&self, dwinputstreamindex: u32) -> windows_core::Result<u32>;
    fn ProcessInput(&self, dwinputstreamindex: u32, pbuffer: windows_core::Ref<IMediaBuffer>, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> windows_core::Result<()>;
    fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::Result<()>;
    fn Lock(&self, block: i32) -> windows_core::Result<()>;
}
impl IMediaObject_Vtbl {
    pub const fn new<Identity: IMediaObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamCount<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetStreamCount(this, core::mem::transmute_copy(&pcinputstreams), core::mem::transmute_copy(&pcoutputstreams)).into()
            }
        }
        unsafe extern "system" fn GetInputStreamInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetInputStreamInfo(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetOutputStreamInfo(this, core::mem::transmute_copy(&dwoutputstreamindex)) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetInputType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&dwtypeindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn GetOutputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetOutputType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&dwtypeindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn SetInputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::SetInputType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetOutputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::SetOutputType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetInputCurrentType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetOutputCurrentType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn GetInputSizeInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetInputSizeInfo(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pcbsize), core::mem::transmute_copy(&pcbmaxlookahead), core::mem::transmute_copy(&pcbalignment)).into()
            }
        }
        unsafe extern "system" fn GetOutputSizeInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::GetOutputSizeInfo(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pcbsize), core::mem::transmute_copy(&pcbalignment)).into()
            }
        }
        unsafe extern "system" fn GetInputMaxLatency<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetInputMaxLatency(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                    Ok(ok__) => {
                        prtmaxlatency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputMaxLatency<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::SetInputMaxLatency(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&rtmaxlatency)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn Discontinuity<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::Discontinuity(this, core::mem::transmute_copy(&dwinputstreamindex)).into()
            }
        }
        unsafe extern "system" fn AllocateStreamingResources<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::AllocateStreamingResources(this).into()
            }
        }
        unsafe extern "system" fn FreeStreamingResources<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::FreeStreamingResources(this).into()
            }
        }
        unsafe extern "system" fn GetInputStatus<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObject_Impl::GetInputStatus(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                    Ok(ok__) => {
                        dwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pbuffer: *mut core::ffi::c_void, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::ProcessInput(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&rttimestamp), core::mem::transmute_copy(&rttimelength)).into()
            }
        }
        unsafe extern "system" fn ProcessOutput<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::ProcessOutput(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&coutputbuffercount), core::mem::transmute_copy(&poutputbuffers), core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObject_Impl::Lock(this, core::mem::transmute_copy(&block)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Identity, OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Identity, OFFSET>,
            GetInputType: GetInputType::<Identity, OFFSET>,
            GetOutputType: GetOutputType::<Identity, OFFSET>,
            SetInputType: SetInputType::<Identity, OFFSET>,
            SetOutputType: SetOutputType::<Identity, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, OFFSET>,
            GetInputSizeInfo: GetInputSizeInfo::<Identity, OFFSET>,
            GetOutputSizeInfo: GetOutputSizeInfo::<Identity, OFFSET>,
            GetInputMaxLatency: GetInputMaxLatency::<Identity, OFFSET>,
            SetInputMaxLatency: SetInputMaxLatency::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            Discontinuity: Discontinuity::<Identity, OFFSET>,
            AllocateStreamingResources: AllocateStreamingResources::<Identity, OFFSET>,
            FreeStreamingResources: FreeStreamingResources::<Identity, OFFSET>,
            GetInputStatus: GetInputStatus::<Identity, OFFSET>,
            ProcessInput: ProcessInput::<Identity, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMediaObject {}
windows_core::imp::define_interface!(IMediaObjectInPlace, IMediaObjectInPlace_Vtbl, 0x651b9ad0_0fc7_4aa9_9538_d89931010741);
windows_core::imp::interface_hierarchy!(IMediaObjectInPlace, windows_core::IUnknown);
impl IMediaObjectInPlace {
    pub unsafe fn Process(&self, pdata: &mut [u8], reftimestart: i64, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), pdata.len().try_into().unwrap(), core::mem::transmute(pdata.as_ptr()), reftimestart, dwflags).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IMediaObjectInPlace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLatency(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObjectInPlace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, i64, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
pub trait IMediaObjectInPlace_Impl: windows_core::IUnknownImpl {
    fn Process(&self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMediaObjectInPlace>;
    fn GetLatency(&self) -> windows_core::Result<i64>;
}
impl IMediaObjectInPlace_Vtbl {
    pub const fn new<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Process<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaObjectInPlace_Impl::Process(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&reftimestart), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediaobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObjectInPlace_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppmediaobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLatency<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, platencytime: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaObjectInPlace_Impl::GetLatency(this) {
                    Ok(ok__) => {
                        platencytime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Process: Process::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetLatency: GetLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaObjectInPlace as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMediaObjectInPlace {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_INPLACE_PROCESS_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_INPUT_DATA_BUFFER_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_INPUT_STATUS_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_INPUT_STREAM_INFO_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_OUTPUT_DATA_BUFFER_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_OUTPUT_STREAM_INFO_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_PROCESS_OUTPUT_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_QUALITY_STATUS_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_SET_TYPE_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DMO_VIDEO_OUTPUT_STREAM_FLAGS(pub i32);
