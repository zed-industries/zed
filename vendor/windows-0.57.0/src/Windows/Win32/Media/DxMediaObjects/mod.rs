#[inline]
pub unsafe fn DMOEnum(guidcategory: *const windows_core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> windows_core::Result<IEnumDMO> {
    windows_targets::link!("msdmo.dll" "system" fn DMOEnum(guidcategory : *const windows_core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE, ppenum : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DMOEnum(guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn DMOGetName(clsiddmo: *const windows_core::GUID, szname: &mut [u16; 80]) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn DMOGetName(clsiddmo : *const windows_core::GUID, szname : windows_core::PWSTR) -> windows_core::HRESULT);
    DMOGetName(clsiddmo, core::mem::transmute(szname.as_ptr())).ok()
}
#[inline]
pub unsafe fn DMOGetTypes(clsiddmo: *const windows_core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn DMOGetTypes(clsiddmo : *const windows_core::GUID, ulinputtypesrequested : u32, pulinputtypessupplied : *mut u32, pinputtypes : *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested : u32, puloutputtypessupplied : *mut u32, poutputtypes : *mut DMO_PARTIAL_MEDIATYPE) -> windows_core::HRESULT);
    DMOGetTypes(clsiddmo, ulinputtypesrequested, pulinputtypessupplied, pinputtypes, uloutputtypesrequested, puloutputtypessupplied, poutputtypes).ok()
}
#[inline]
pub unsafe fn DMORegister<P0>(szname: P0, clsiddmo: *const windows_core::GUID, guidcategory: *const windows_core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdmo.dll" "system" fn DMORegister(szname : windows_core::PCWSTR, clsiddmo : *const windows_core::GUID, guidcategory : *const windows_core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE) -> windows_core::HRESULT);
    DMORegister(szname.param().abi(), clsiddmo, guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes).ok()
}
#[inline]
pub unsafe fn DMOUnregister(clsiddmo: *const windows_core::GUID, guidcategory: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn DMOUnregister(clsiddmo : *const windows_core::GUID, guidcategory : *const windows_core::GUID) -> windows_core::HRESULT);
    DMOUnregister(clsiddmo, guidcategory).ok()
}
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    MoCopyMediaType(pmtdest, pmtsrc).ok()
}
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut DMO_MEDIA_TYPE, cbformat : u32) -> windows_core::HRESULT);
    MoCreateMediaType(ppmt, cbformat).ok()
}
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    MoDeleteMediaType(pmt).ok()
}
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    MoDuplicateMediaType(ppmtdest, pmtsrc).ok()
}
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT);
    MoFreeMediaType(pmt).ok()
}
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut DMO_MEDIA_TYPE, cbformat : u32) -> windows_core::HRESULT);
    MoInitMediaType(pmt, cbformat).ok()
}
windows_core::imp::define_interface!(IDMOQualityControl, IDMOQualityControl_Vtbl, 0x65abea96_cf36_453f_af8a_705e98f16260);
impl core::ops::Deref for IDMOQualityControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDMOQualityControl, windows_core::IUnknown);
impl IDMOQualityControl {
    pub unsafe fn SetNow(&self, rtnow: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNow)(windows_core::Interface::as_raw(self), rtnow).ok()
    }
    pub unsafe fn SetStatus(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDMOQualityControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetNow: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDMOVideoOutputOptimizations, IDMOVideoOutputOptimizations_Vtbl, 0xbe8f4f4e_5b16_4d29_b350_7f6b5d9298ac);
impl core::ops::Deref for IDMOVideoOutputOptimizations {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDMOVideoOutputOptimizations, windows_core::IUnknown);
impl IDMOVideoOutputOptimizations {
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryOperationModePreferences)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
    }
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOperationMode)(windows_core::Interface::as_raw(self), uloutputstreamindex, dwenabledfeatures).ok()
    }
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentOperationMode)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentSampleRequirements)(windows_core::Interface::as_raw(self), uloutputstreamindex, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDMOVideoOutputOptimizations_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryOperationModePreferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetOperationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetCurrentOperationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentSampleRequirements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumDMO, IEnumDMO_Vtbl, 0x2c3cd98a_2bfa_4a53_9c27_5249ba64ba0f);
impl core::ops::Deref for IEnumDMO {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDMO, windows_core::IUnknown);
impl IEnumDMO {
    pub unsafe fn Next(&self, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), citemstofetch, pclsid, names, pcitemsfetched).ok()
    }
    pub unsafe fn Skip(&self, citemstoskip: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), citemstoskip).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDMO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumDMO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBuffer, IMediaBuffer_Vtbl, 0x59eff8b9_938c_4a26_82f2_95cb84cdc837);
impl core::ops::Deref for IMediaBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaBuffer, windows_core::IUnknown);
impl IMediaBuffer {
    pub unsafe fn SetLength(&self, cblength: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), cblength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: Option<*mut *mut u8>, pcblength: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBufferAndLength)(windows_core::Interface::as_raw(self), core::mem::transmute(ppbuffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcblength.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMediaBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaObject, IMediaObject_Vtbl, 0xd8ad0f58_5494_4102_97c5_ec798e59bcf4);
impl core::ops::Deref for IMediaObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaObject, windows_core::IUnknown);
impl IMediaObject {
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pcinputstreams, pcoutputstreams).ok()
    }
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputStreamInfo)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputStreamInfo)(windows_core::Interface::as_raw(self), dwoutputstreamindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: Option<*mut DMO_MEDIA_TYPE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputType)(windows_core::Interface::as_raw(self), dwinputstreamindex, dwtypeindex, core::mem::transmute(pmt.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: Option<*mut DMO_MEDIA_TYPE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, dwtypeindex, core::mem::transmute(pmt.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInputType)(windows_core::Interface::as_raw(self), dwinputstreamindex, core::mem::transmute(pmt.unwrap_or(std::ptr::null())), dwflags).ok()
    }
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOutputType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, core::mem::transmute(pmt.unwrap_or(std::ptr::null())), dwflags).ok()
    }
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputCurrentType)(windows_core::Interface::as_raw(self), dwinputstreamindex, pmt).ok()
    }
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputCurrentType)(windows_core::Interface::as_raw(self), dwoutputstreamindex, pmt).ok()
    }
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputSizeInfo)(windows_core::Interface::as_raw(self), dwinputstreamindex, pcbsize, pcbmaxlookahead, pcbalignment).ok()
    }
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputSizeInfo)(windows_core::Interface::as_raw(self), dwoutputstreamindex, pcbsize, pcbalignment).ok()
    }
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputMaxLatency)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
    }
    pub unsafe fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInputMaxLatency)(windows_core::Interface::as_raw(self), dwinputstreamindex, rtmaxlatency).ok()
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Discontinuity(&self, dwinputstreamindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Discontinuity)(windows_core::Interface::as_raw(self), dwinputstreamindex).ok()
    }
    pub unsafe fn AllocateStreamingResources(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateStreamingResources)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FreeStreamingResources(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeStreamingResources)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetInputStatus(&self, dwinputstreamindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputStatus)(windows_core::Interface::as_raw(self), dwinputstreamindex, &mut result__).map(|| result__)
    }
    pub unsafe fn ProcessInput<P0>(&self, dwinputstreamindex: u32, pbuffer: P0, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMediaBuffer>,
    {
        (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), dwinputstreamindex, pbuffer.param().abi(), dwflags, rttimestamp, rttimelength).ok()
    }
    pub unsafe fn ProcessOutput(&self, dwflags: u32, poutputbuffers: &mut [DMO_OUTPUT_DATA_BUFFER], pdwstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessOutput)(windows_core::Interface::as_raw(self), dwflags, poutputbuffers.len().try_into().unwrap(), core::mem::transmute(poutputbuffers.as_ptr()), pdwstatus).ok()
    }
    pub unsafe fn Lock(&self, block: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), block).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMediaObjectInPlace, IMediaObjectInPlace_Vtbl, 0x651b9ad0_0fc7_4aa9_9538_d89931010741);
impl core::ops::Deref for IMediaObjectInPlace {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaObjectInPlace, windows_core::IUnknown);
impl IMediaObjectInPlace {
    pub unsafe fn Process(&self, pdata: &mut [u8], reftimestart: i64, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), pdata.len().try_into().unwrap(), core::mem::transmute(pdata.as_ptr()), reftimestart, dwflags).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IMediaObjectInPlace> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLatency(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMediaObjectInPlace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, i64, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
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
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = _DMO_PROCESS_OUTPUT_FLAGS(1i32);
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = _DMO_QUALITY_STATUS_FLAGS(1i32);
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = DMO_REGISTER_FLAGS(1i32);
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(2i32);
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(1i32);
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = _DMO_VIDEO_OUTPUT_STREAM_FLAGS(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DMO_ENUM_FLAGS(pub i32);
impl windows_core::TypeKind for DMO_ENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DMO_ENUM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DMO_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DMO_REGISTER_FLAGS(pub i32);
impl windows_core::TypeKind for DMO_REGISTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DMO_REGISTER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DMO_REGISTER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_INPLACE_PROCESS_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_INPLACE_PROCESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_INPLACE_PROCESS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_INPLACE_PROCESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_INPUT_DATA_BUFFER_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_INPUT_DATA_BUFFER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_INPUT_STATUS_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_INPUT_STATUS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_INPUT_STATUS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_INPUT_STREAM_INFO_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_INPUT_STREAM_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_OUTPUT_DATA_BUFFER_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_OUTPUT_STREAM_INFO_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_PROCESS_OUTPUT_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_PROCESS_OUTPUT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_PROCESS_OUTPUT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_PROCESS_OUTPUT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_QUALITY_STATUS_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_QUALITY_STATUS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_QUALITY_STATUS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_QUALITY_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_SET_TYPE_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_SET_TYPE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_SET_TYPE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_SET_TYPE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DMO_VIDEO_OUTPUT_STREAM_FLAGS(pub i32);
impl windows_core::TypeKind for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DMO_VIDEO_OUTPUT_STREAM_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct DMO_MEDIA_TYPE {
    pub majortype: windows_core::GUID,
    pub subtype: windows_core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl Clone for DMO_MEDIA_TYPE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for DMO_MEDIA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: core::mem::ManuallyDrop<Option<IMediaBuffer>>,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl Clone for DMO_OUTPUT_DATA_BUFFER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for DMO_OUTPUT_DATA_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DMO_OUTPUT_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: windows_core::GUID,
    pub subtype: windows_core::GUID,
}
impl windows_core::TypeKind for DMO_PARTIAL_MEDIATYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DMO_PARTIAL_MEDIATYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
