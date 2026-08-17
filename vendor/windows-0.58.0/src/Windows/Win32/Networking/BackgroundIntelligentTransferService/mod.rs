windows_core::imp::define_interface!(AsyncIBackgroundCopyCallback, AsyncIBackgroundCopyCallback_Vtbl, 0xca29d251_b4bb_4679_a3d9_ae8006119d54);
impl core::ops::Deref for AsyncIBackgroundCopyCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIBackgroundCopyCallback, windows_core::IUnknown);
impl AsyncIBackgroundCopyCallback {
    pub unsafe fn Begin_JobTransferred<P0>(&self, pjob: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        (windows_core::Interface::vtable(self).Begin_JobTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi()).ok()
    }
    pub unsafe fn Finish_JobTransferred(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_JobTransferred)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_JobError<P0, P1>(&self, pjob: P0, perror: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyError>,
    {
        (windows_core::Interface::vtable(self).Begin_JobError)(windows_core::Interface::as_raw(self), pjob.param().abi(), perror.param().abi()).ok()
    }
    pub unsafe fn Finish_JobError(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_JobError)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_JobModification<P0>(&self, pjob: P0, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        (windows_core::Interface::vtable(self).Begin_JobModification)(windows_core::Interface::as_raw(self), pjob.param().abi(), dwreserved).ok()
    }
    pub unsafe fn Finish_JobModification(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_JobModification)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct AsyncIBackgroundCopyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_JobError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_JobError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_JobModification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_JobModification: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBITSExtensionSetup, IBITSExtensionSetup_Vtbl, 0x29cfbbf7_09e4_4b97_b0bc_f2287e3d8eb3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBITSExtensionSetup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBITSExtensionSetup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetup {
    pub unsafe fn EnableBITSUploads(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableBITSUploads)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableBITSUploads(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableBITSUploads)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCleanupTaskName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCleanupTaskName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCleanupTask(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCleanupTask)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBITSExtensionSetup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EnableBITSUploads: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableBITSUploads: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCleanupTaskName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCleanupTask: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBITSExtensionSetupFactory, IBITSExtensionSetupFactory_Vtbl, 0xd5d2d542_5503_4e64_8b48_72ef91a32ee1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBITSExtensionSetupFactory {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBITSExtensionSetupFactory, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetupFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObject<P0>(&self, path: P0) -> windows_core::Result<IBITSExtensionSetup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBITSExtensionSetupFactory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObject: usize,
}
windows_core::imp::define_interface!(IBackgroundCopyCallback, IBackgroundCopyCallback_Vtbl, 0x97ea99c7_0186_4ad4_8df9_c5b4e0ed6b22);
impl core::ops::Deref for IBackgroundCopyCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback, windows_core::IUnknown);
impl IBackgroundCopyCallback {
    pub unsafe fn JobTransferred<P0>(&self, pjob: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        (windows_core::Interface::vtable(self).JobTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi()).ok()
    }
    pub unsafe fn JobError<P0, P1>(&self, pjob: P0, perror: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyError>,
    {
        (windows_core::Interface::vtable(self).JobError)(windows_core::Interface::as_raw(self), pjob.param().abi(), perror.param().abi()).ok()
    }
    pub unsafe fn JobModification<P0>(&self, pjob: P0, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        (windows_core::Interface::vtable(self).JobModification)(windows_core::Interface::as_raw(self), pjob.param().abi(), dwreserved).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobModification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyCallback1, IBackgroundCopyCallback1_Vtbl, 0x084f6593_3800_4e08_9b59_99fa59addf82);
impl core::ops::Deref for IBackgroundCopyCallback1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback1, windows_core::IUnknown);
impl IBackgroundCopyCallback1 {
    pub unsafe fn OnStatus<P0, P1>(&self, pgroup: P0, pjob: P1, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyGroup>,
        P1: windows_core::Param<IBackgroundCopyJob1>,
    {
        (windows_core::Interface::vtable(self).OnStatus)(windows_core::Interface::as_raw(self), pgroup.param().abi(), pjob.param().abi(), dwfileindex, dwstatus, dwnumofretries, dwwin32result, dwtransportresult).ok()
    }
    pub unsafe fn OnProgress<P0, P1>(&self, progresstype: u32, pgroup: P0, pjob: P1, dwfileindex: u32, dwprogressvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyGroup>,
        P1: windows_core::Param<IBackgroundCopyJob1>,
    {
        (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), progresstype, pgroup.param().abi(), pjob.param().abi(), dwfileindex, dwprogressvalue).ok()
    }
    pub unsafe fn OnProgressEx<P0, P1>(&self, progresstype: u32, pgroup: P0, pjob: P1, dwfileindex: u32, dwprogressvalue: u32, pbyte: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyGroup>,
        P1: windows_core::Param<IBackgroundCopyJob1>,
    {
        (windows_core::Interface::vtable(self).OnProgressEx)(windows_core::Interface::as_raw(self), progresstype, pgroup.param().abi(), pjob.param().abi(), dwfileindex, dwprogressvalue, pbyte.len().try_into().unwrap(), core::mem::transmute(pbyte.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyCallback1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub OnProgressEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyCallback2, IBackgroundCopyCallback2_Vtbl, 0x659cdeac_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBackgroundCopyCallback2 {
    type Target = IBackgroundCopyCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback2, windows_core::IUnknown, IBackgroundCopyCallback);
impl IBackgroundCopyCallback2 {
    pub unsafe fn FileTransferred<P0, P1>(&self, pjob: P0, pfile: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyFile>,
    {
        (windows_core::Interface::vtable(self).FileTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi(), pfile.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyCallback2_Vtbl {
    pub base__: IBackgroundCopyCallback_Vtbl,
    pub FileTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyCallback3, IBackgroundCopyCallback3_Vtbl, 0x98c97bd2_e32b_4ad8_a528_95fd8b16bd42);
impl core::ops::Deref for IBackgroundCopyCallback3 {
    type Target = IBackgroundCopyCallback2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback3, windows_core::IUnknown, IBackgroundCopyCallback, IBackgroundCopyCallback2);
impl IBackgroundCopyCallback3 {
    pub unsafe fn FileRangesTransferred<P0, P1>(&self, job: P0, file: P1, ranges: &[BG_FILE_RANGE]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyFile>,
    {
        (windows_core::Interface::vtable(self).FileRangesTransferred)(windows_core::Interface::as_raw(self), job.param().abi(), file.param().abi(), ranges.len().try_into().unwrap(), core::mem::transmute(ranges.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyCallback3_Vtbl {
    pub base__: IBackgroundCopyCallback2_Vtbl,
    pub FileRangesTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const BG_FILE_RANGE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyError, IBackgroundCopyError_Vtbl, 0x19c613a0_fcb8_4f28_81ae_897c3d078f81);
impl core::ops::Deref for IBackgroundCopyError {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyError, windows_core::IUnknown);
impl IBackgroundCopyError {
    pub unsafe fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), pcontext, pcode).ok()
    }
    pub unsafe fn GetFile(&self) -> windows_core::Result<IBackgroundCopyFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetErrorDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), languageid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetErrorContextDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorContextDescription)(windows_core::Interface::as_raw(self), languageid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetProtocol(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProtocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_ERROR_CONTEXT, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetErrorContextDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyFile, IBackgroundCopyFile_Vtbl, 0x01b7bd23_fb88_4a77_8490_5891d3e4653a);
impl core::ops::Deref for IBackgroundCopyFile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile, windows_core::IUnknown);
impl IBackgroundCopyFile {
    pub unsafe fn GetRemoteName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRemoteName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLocalName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProgress(&self, pval: *mut BG_FILE_PROGRESS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), pval).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_FILE_PROGRESS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyFile2, IBackgroundCopyFile2_Vtbl, 0x83e81b93_0873_474d_8a8c_f2018b1a939c);
impl core::ops::Deref for IBackgroundCopyFile2 {
    type Target = IBackgroundCopyFile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile2, windows_core::IUnknown, IBackgroundCopyFile);
impl IBackgroundCopyFile2 {
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileRanges)(windows_core::Interface::as_raw(self), rangecount, ranges).ok()
    }
    pub unsafe fn SetRemoteName<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteName)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyFile2_Vtbl {
    pub base__: IBackgroundCopyFile_Vtbl,
    pub GetFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT,
    pub SetRemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyFile3, IBackgroundCopyFile3_Vtbl, 0x659cdeaa_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBackgroundCopyFile3 {
    type Target = IBackgroundCopyFile2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile3, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2);
impl IBackgroundCopyFile3 {
    pub unsafe fn GetTemporaryName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTemporaryName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetValidationState<P0>(&self, state: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetValidationState)(windows_core::Interface::as_raw(self), state.param().abi()).ok()
    }
    pub unsafe fn GetValidationState(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValidationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDownloadedFromPeer(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDownloadedFromPeer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyFile3_Vtbl {
    pub base__: IBackgroundCopyFile2_Vtbl,
    pub GetTemporaryName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetValidationState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetValidationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsDownloadedFromPeer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyFile4, IBackgroundCopyFile4_Vtbl, 0xef7e0655_7888_4960_b0e5_730846e03492);
impl core::ops::Deref for IBackgroundCopyFile4 {
    type Target = IBackgroundCopyFile3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile4, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2, IBackgroundCopyFile3);
impl IBackgroundCopyFile4 {
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPeerDownloadStats)(windows_core::Interface::as_raw(self), pfromorigin, pfrompeers).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyFile4_Vtbl {
    pub base__: IBackgroundCopyFile3_Vtbl,
    pub GetPeerDownloadStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyFile5, IBackgroundCopyFile5_Vtbl, 0x85c1657f_dafc_40e8_8834_df18ea25717e);
impl core::ops::Deref for IBackgroundCopyFile5 {
    type Target = IBackgroundCopyFile4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile5, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2, IBackgroundCopyFile3, IBackgroundCopyFile4);
impl IBackgroundCopyFile5 {
    pub unsafe fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> windows_core::Result<BITS_FILE_PROPERTY_VALUE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyFile5_Vtbl {
    pub base__: IBackgroundCopyFile4_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_FILE_PROPERTY_ID, BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_FILE_PROPERTY_ID, *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyFile6, IBackgroundCopyFile6_Vtbl, 0xcf6784f7_d677_49fd_9368_cb47aee9d1ad);
impl core::ops::Deref for IBackgroundCopyFile6 {
    type Target = IBackgroundCopyFile5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile6, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2, IBackgroundCopyFile3, IBackgroundCopyFile4, IBackgroundCopyFile5);
impl IBackgroundCopyFile6 {
    pub unsafe fn UpdateDownloadPosition(&self, offset: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateDownloadPosition)(windows_core::Interface::as_raw(self), offset).ok()
    }
    pub unsafe fn RequestFileRanges(&self, ranges: &[BG_FILE_RANGE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestFileRanges)(windows_core::Interface::as_raw(self), ranges.len().try_into().unwrap(), core::mem::transmute(ranges.as_ptr())).ok()
    }
    pub unsafe fn GetFilledFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFilledFileRanges)(windows_core::Interface::as_raw(self), rangecount, ranges).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyFile6_Vtbl {
    pub base__: IBackgroundCopyFile5_Vtbl,
    pub UpdateDownloadPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub RequestFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BG_FILE_RANGE) -> windows_core::HRESULT,
    pub GetFilledFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyGroup, IBackgroundCopyGroup_Vtbl, 0x1ded80a7_53ea_424f_8a04_17fea9adc4f5);
impl core::ops::Deref for IBackgroundCopyGroup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyGroup, windows_core::IUnknown);
impl IBackgroundCopyGroup {
    pub unsafe fn GetProp(&self, propid: GROUPPROP) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProp)(windows_core::Interface::as_raw(self), propid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProp(&self, propid: GROUPPROP, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProp)(windows_core::Interface::as_raw(self), propid, core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn GetProgress(&self, dwflags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdwstatus, pdwjobindex).ok()
    }
    pub unsafe fn GetJob(&self, jobid: windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), core::mem::transmute(jobid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SuspendGroup(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspendGroup)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeGroup(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResumeGroup)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelGroup(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelGroup)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Size(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GroupID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GroupID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateJob(&self, guidjobid: windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateJob)(windows_core::Interface::as_raw(self), core::mem::transmute(guidjobid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumJobs(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyJobs1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumJobs)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SwitchToForeground(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SwitchToForeground)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryNewJobInterface(&self, iid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryNewJobInterface)(windows_core::Interface::as_raw(self), iid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNotificationPointer<P0>(&self, iid: *const windows_core::GUID, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetNotificationPointer)(windows_core::Interface::as_raw(self), iid, punk.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProp: unsafe extern "system" fn(*mut core::ffi::c_void, GROUPPROP, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProp: unsafe extern "system" fn(*mut core::ffi::c_void, GROUPPROP, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspendGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GroupID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumJobs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SwitchToForeground: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryNewJobInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNotificationPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJob, IBackgroundCopyJob_Vtbl, 0x37668d37_507e_4160_9316_26306d150b12);
impl core::ops::Deref for IBackgroundCopyJob {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob, windows_core::IUnknown);
impl IBackgroundCopyJob {
    pub unsafe fn AddFileSet(&self, pfileset: &[BG_FILE_INFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddFileSet)(windows_core::Interface::as_raw(self), pfileset.len().try_into().unwrap(), core::mem::transmute(pfileset.as_ptr())).ok()
    }
    pub unsafe fn AddFile<P0, P1>(&self, remoteurl: P0, localname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddFile)(windows_core::Interface::as_raw(self), remoteurl.param().abi(), localname.param().abi()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> windows_core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Suspend(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Complete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<BG_JOB_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProgress(&self, pval: *mut BG_JOB_PROGRESS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), pval).ok()
    }
    pub unsafe fn GetTimes(&self, pval: *mut BG_JOB_TIMES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTimes)(windows_core::Interface::as_raw(self), pval).ok()
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<BG_JOB_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetError(&self) -> windows_core::Result<IBackgroundCopyError> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOwner(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDescription<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<BG_JOB_PRIORITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNotifyFlags)(windows_core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNotifyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotifyInterface<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetNotifyInterface)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNotifyInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinimumRetryDelay)(windows_core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinimumRetryDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNoProgressTimeout)(windows_core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNoProgressTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetErrorCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProxySettings<P0, P1>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: P0, proxybypasslist: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProxySettings)(windows_core::Interface::as_raw(self), proxyusage, proxylist.param().abi(), proxybypasslist.param().abi()).ok()
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProxySettings)(windows_core::Interface::as_raw(self), pproxyusage, pproxylist, pproxybypasslist).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFileSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BG_FILE_INFO) -> windows_core::HRESULT,
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_TYPE) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PROGRESS) -> windows_core::HRESULT,
    pub GetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_TIMES) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_STATE) -> windows_core::HRESULT,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, BG_JOB_PRIORITY) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PRIORITY) -> windows_core::HRESULT,
    pub SetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNotifyInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNotifyInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMinimumRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMinimumRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNoProgressTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNoProgressTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetErrorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, BG_JOB_PROXY_USAGE, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PROXY_USAGE, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJob1, IBackgroundCopyJob1_Vtbl, 0x59f5553c_2031_4629_bb18_2645a6970947);
impl core::ops::Deref for IBackgroundCopyJob1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob1, windows_core::IUnknown);
impl IBackgroundCopyJob1 {
    pub unsafe fn CancelJob(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelJob)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProgress(&self, dwflags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdwstatus, pdwwin32result, pdwtransportresult, pdwnumofretries).ok()
    }
    pub unsafe fn AddFiles(&self, ppfileset: &[*const FILESETINFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddFiles)(windows_core::Interface::as_raw(self), ppfileset.len().try_into().unwrap(), core::mem::transmute(ppfileset.as_ptr())).ok()
    }
    pub unsafe fn GetFile(&self, cfileindex: u32) -> windows_core::Result<FILESETINFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), cfileindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetFileCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SwitchToForeground(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SwitchToForeground)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn JobID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JobID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyJob1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CancelJob: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub AddFiles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const FILESETINFO) -> windows_core::HRESULT,
    pub GetFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut FILESETINFO) -> windows_core::HRESULT,
    pub GetFileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SwitchToForeground: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJob2, IBackgroundCopyJob2_Vtbl, 0x54b50739_686f_45eb_9dff_d6a9a0faa9af);
impl core::ops::Deref for IBackgroundCopyJob2 {
    type Target = IBackgroundCopyJob;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob2, windows_core::IUnknown, IBackgroundCopyJob);
impl IBackgroundCopyJob2 {
    pub unsafe fn SetNotifyCmdLine<P0, P1>(&self, program: P0, parameters: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetNotifyCmdLine)(windows_core::Interface::as_raw(self), program.param().abi(), parameters.param().abi()).ok()
    }
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNotifyCmdLine)(windows_core::Interface::as_raw(self), pprogram, pparameters).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReplyProgress)(windows_core::Interface::as_raw(self), pprogress).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReplyData)(windows_core::Interface::as_raw(self), ppbuffer, plength).ok()
    }
    pub unsafe fn SetReplyFileName<P0>(&self, replyfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetReplyFileName)(windows_core::Interface::as_raw(self), replyfilename.param().abi()).ok()
    }
    pub unsafe fn GetReplyFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReplyFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), credentials).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveCredentials)(windows_core::Interface::as_raw(self), target, scheme).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyJob2_Vtbl {
    pub base__: IBackgroundCopyJob_Vtbl,
    pub SetNotifyCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetNotifyCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetReplyProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT,
    pub GetReplyData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u64) -> windows_core::HRESULT,
    pub SetReplyFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetReplyFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *const BG_AUTH_CREDENTIALS) -> windows_core::HRESULT,
    pub RemoveCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, BG_AUTH_TARGET, BG_AUTH_SCHEME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJob3, IBackgroundCopyJob3_Vtbl, 0x443c8934_90ff_48ed_bcde_26f5c7450042);
impl core::ops::Deref for IBackgroundCopyJob3 {
    type Target = IBackgroundCopyJob2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob3, windows_core::IUnknown, IBackgroundCopyJob, IBackgroundCopyJob2);
impl IBackgroundCopyJob3 {
    pub unsafe fn ReplaceRemotePrefix<P0, P1>(&self, oldprefix: P0, newprefix: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReplaceRemotePrefix)(windows_core::Interface::as_raw(self), oldprefix.param().abi(), newprefix.param().abi()).ok()
    }
    pub unsafe fn AddFileWithRanges<P0, P1>(&self, remoteurl: P0, localname: P1, ranges: &[BG_FILE_RANGE]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddFileWithRanges)(windows_core::Interface::as_raw(self), remoteurl.param().abi(), localname.param().abi(), ranges.len().try_into().unwrap(), core::mem::transmute(ranges.as_ptr())).ok()
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileACLFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetFileACLFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileACLFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyJob3_Vtbl {
    pub base__: IBackgroundCopyJob2_Vtbl,
    pub ReplaceRemotePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddFileWithRanges: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *const BG_FILE_RANGE) -> windows_core::HRESULT,
    pub SetFileACLFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFileACLFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJob4, IBackgroundCopyJob4_Vtbl, 0x659cdeae_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBackgroundCopyJob4 {
    type Target = IBackgroundCopyJob3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob4, windows_core::IUnknown, IBackgroundCopyJob, IBackgroundCopyJob2, IBackgroundCopyJob3);
impl IBackgroundCopyJob4 {
    pub unsafe fn SetPeerCachingFlags(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPeerCachingFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetPeerCachingFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPeerCachingFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOwnerIntegrityLevel(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwnerIntegrityLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOwnerElevationState(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwnerElevationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaximumDownloadTime(&self, timeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaximumDownloadTime)(windows_core::Interface::as_raw(self), timeout).ok()
    }
    pub unsafe fn GetMaximumDownloadTime(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaximumDownloadTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyJob4_Vtbl {
    pub base__: IBackgroundCopyJob3_Vtbl,
    pub SetPeerCachingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPeerCachingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOwnerIntegrityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOwnerElevationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetMaximumDownloadTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumDownloadTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJob5, IBackgroundCopyJob5_Vtbl, 0xe847030c_bbba_4657_af6d_484aa42bf1fe);
impl core::ops::Deref for IBackgroundCopyJob5 {
    type Target = IBackgroundCopyJob4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob5, windows_core::IUnknown, IBackgroundCopyJob, IBackgroundCopyJob2, IBackgroundCopyJob3, IBackgroundCopyJob4);
impl IBackgroundCopyJob5 {
    pub unsafe fn SetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID) -> windows_core::Result<BITS_JOB_PROPERTY_VALUE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyJob5_Vtbl {
    pub base__: IBackgroundCopyJob4_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_JOB_PROPERTY_ID, BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_JOB_PROPERTY_ID, *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions, IBackgroundCopyJobHttpOptions_Vtbl, 0xf1bd1079_9f01_4bdc_8036_f09b70095066);
impl core::ops::Deref for IBackgroundCopyJobHttpOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions, windows_core::IUnknown);
impl IBackgroundCopyJobHttpOptions {
    pub unsafe fn SetClientCertificateByID<P0>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, pcerthashblob: &[u8; 20]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientCertificateByID)(windows_core::Interface::as_raw(self), storelocation, storename.param().abi(), core::mem::transmute(pcerthashblob.as_ptr())).ok()
    }
    pub unsafe fn SetClientCertificateByName<P0, P1>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, subjectname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientCertificateByName)(windows_core::Interface::as_raw(self), storelocation, storename.param().abi(), subjectname.param().abi()).ok()
    }
    pub unsafe fn RemoveClientCertificate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveClientCertificate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClientCertificate)(windows_core::Interface::as_raw(self), pstorelocation, pstorename, ppcerthashblob, psubjectname).ok()
    }
    pub unsafe fn SetCustomHeaders<P0>(&self, requestheaders: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCustomHeaders)(windows_core::Interface::as_raw(self), requestheaders.param().abi()).ok()
    }
    pub unsafe fn GetCustomHeaders(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSecurityFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetSecurityFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurityFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyJobHttpOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetClientCertificateByID: unsafe extern "system" fn(*mut core::ffi::c_void, BG_CERT_STORE_LOCATION, windows_core::PCWSTR, *const u8) -> windows_core::HRESULT,
    pub SetClientCertificateByName: unsafe extern "system" fn(*mut core::ffi::c_void, BG_CERT_STORE_LOCATION, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_CERT_STORE_LOCATION, *mut windows_core::PWSTR, *mut *mut u8, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCustomHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCustomHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSecurityFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetSecurityFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions2, IBackgroundCopyJobHttpOptions2_Vtbl, 0xb591a192_a405_4fc3_8323_4c5c542578fc);
impl core::ops::Deref for IBackgroundCopyJobHttpOptions2 {
    type Target = IBackgroundCopyJobHttpOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions2, windows_core::IUnknown, IBackgroundCopyJobHttpOptions);
impl IBackgroundCopyJobHttpOptions2 {
    pub unsafe fn SetHttpMethod<P0>(&self, method: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHttpMethod)(windows_core::Interface::as_raw(self), method.param().abi()).ok()
    }
    pub unsafe fn GetHttpMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHttpMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyJobHttpOptions2_Vtbl {
    pub base__: IBackgroundCopyJobHttpOptions_Vtbl,
    pub SetHttpMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetHttpMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions3, IBackgroundCopyJobHttpOptions3_Vtbl, 0x8a9263d3_fd4c_4eda_9b28_30132a4d4e3c);
impl core::ops::Deref for IBackgroundCopyJobHttpOptions3 {
    type Target = IBackgroundCopyJobHttpOptions2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions3, windows_core::IUnknown, IBackgroundCopyJobHttpOptions, IBackgroundCopyJobHttpOptions2);
impl IBackgroundCopyJobHttpOptions3 {
    pub unsafe fn SetServerCertificateValidationInterface<P0>(&self, certvalidationcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetServerCertificateValidationInterface)(windows_core::Interface::as_raw(self), certvalidationcallback.param().abi()).ok()
    }
    pub unsafe fn MakeCustomHeadersWriteOnly(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MakeCustomHeadersWriteOnly)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyJobHttpOptions3_Vtbl {
    pub base__: IBackgroundCopyJobHttpOptions2_Vtbl,
    pub SetServerCertificateValidationInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MakeCustomHeadersWriteOnly: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyManager, IBackgroundCopyManager_Vtbl, 0x5ce34c0d_0dc9_4c1f_897c_daa1b78cee7c);
impl core::ops::Deref for IBackgroundCopyManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyManager, windows_core::IUnknown);
impl IBackgroundCopyManager {
    pub unsafe fn CreateJob<P0>(&self, displayname: P0, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut Option<IBackgroundCopyJob>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateJob)(windows_core::Interface::as_raw(self), displayname.param().abi(), r#type, pjobid, core::mem::transmute(ppjob)).ok()
    }
    pub unsafe fn GetJob(&self, jobid: *const windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), jobid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumJobs(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyJobs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumJobs)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetErrorDescription(&self, hresult: windows_core::HRESULT, languageid: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), hresult, languageid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBackgroundCopyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, BG_JOB_TYPE, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumJobs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyQMgr, IBackgroundCopyQMgr_Vtbl, 0x16f41c69_09f5_41d2_8cd8_3c08c47bc8a8);
impl core::ops::Deref for IBackgroundCopyQMgr {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyQMgr, windows_core::IUnknown);
impl IBackgroundCopyQMgr {
    pub unsafe fn CreateGroup(&self, guidgroupid: windows_core::GUID) -> windows_core::Result<IBackgroundCopyGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(guidgroupid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGroup(&self, groupid: windows_core::GUID) -> windows_core::Result<IBackgroundCopyGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGroup)(windows_core::Interface::as_raw(self), core::mem::transmute(groupid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumGroups(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumGroups)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IBackgroundCopyQMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumGroups: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundCopyServerCertificateValidationCallback, IBackgroundCopyServerCertificateValidationCallback_Vtbl, 0x4cec0d02_def7_4158_813a_c32a46945ff7);
impl core::ops::Deref for IBackgroundCopyServerCertificateValidationCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyServerCertificateValidationCallback, windows_core::IUnknown);
impl IBackgroundCopyServerCertificateValidationCallback {
    pub unsafe fn ValidateServerCertificate<P0, P1>(&self, job: P0, file: P1, certdata: &[u8], certencodingtype: u32, certstoredata: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyFile>,
    {
        (windows_core::Interface::vtable(self).ValidateServerCertificate)(windows_core::Interface::as_raw(self), job.param().abi(), file.param().abi(), certdata.len().try_into().unwrap(), core::mem::transmute(certdata.as_ptr()), certencodingtype, certstoredata.len().try_into().unwrap(), core::mem::transmute(certstoredata.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ValidateServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitsPeer, IBitsPeer_Vtbl, 0x659cdea2_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBitsPeer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBitsPeer, windows_core::IUnknown);
impl IBitsPeer {
    pub unsafe fn GetPeerName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPeerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsAvailable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBitsPeer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPeerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitsPeerCacheAdministration, IBitsPeerCacheAdministration_Vtbl, 0x659cdead_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBitsPeerCacheAdministration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBitsPeerCacheAdministration, windows_core::IUnknown);
impl IBitsPeerCacheAdministration {
    pub unsafe fn GetMaximumCacheSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaximumCacheSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaximumCacheSize(&self, bytes: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaximumCacheSize)(windows_core::Interface::as_raw(self), bytes).ok()
    }
    pub unsafe fn GetMaximumContentAge(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaximumContentAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaximumContentAge(&self, seconds: u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).SetMaximumContentAge)(windows_core::Interface::as_raw(self), seconds)
    }
    pub unsafe fn GetConfigurationFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConfigurationFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetConfigurationFlags(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConfigurationFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn EnumRecords(&self) -> windows_core::Result<IEnumBitsPeerCacheRecords> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRecords)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<IBitsPeerCacheRecord> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecord)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClearRecords(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearRecords)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteRecord)(windows_core::Interface::as_raw(self), id).ok()
    }
    pub unsafe fn DeleteUrl<P0>(&self, url: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteUrl)(windows_core::Interface::as_raw(self), url.param().abi()).ok()
    }
    pub unsafe fn EnumPeers(&self) -> windows_core::Result<IEnumBitsPeers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumPeers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClearPeers(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearPeers)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DiscoverPeers(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DiscoverPeers)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IBitsPeerCacheAdministration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaximumCacheSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumCacheSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumContentAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumContentAge: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetConfigurationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetConfigurationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearRecords: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeleteUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumPeers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearPeers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscoverPeers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitsPeerCacheRecord, IBitsPeerCacheRecord_Vtbl, 0x659cdeaf_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBitsPeerCacheRecord {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBitsPeerCacheRecord, windows_core::IUnknown);
impl IBitsPeerCacheRecord {
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOriginUrl(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOriginUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFileModificationTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileModificationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLastAccessTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastAccessTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsFileValidated(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsFileValidated)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetFileRanges(&self, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileRanges)(windows_core::Interface::as_raw(self), prangecount, ppranges).ok()
    }
}
#[repr(C)]
pub struct IBitsPeerCacheRecord_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetOriginUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetFileModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetLastAccessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub IsFileValidated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitsTokenOptions, IBitsTokenOptions_Vtbl, 0x9a2584c3_f7d2_457a_9a5e_22b67bffc7d2);
impl core::ops::Deref for IBitsTokenOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBitsTokenOptions, windows_core::IUnknown);
impl IBitsTokenOptions {
    pub unsafe fn SetHelperTokenFlags(&self, usageflags: BG_TOKEN) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelperTokenFlags)(windows_core::Interface::as_raw(self), usageflags).ok()
    }
    pub unsafe fn GetHelperTokenFlags(&self) -> windows_core::Result<BG_TOKEN> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHelperTokenFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHelperToken(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelperToken)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ClearHelperToken(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearHelperToken)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHelperTokenSid(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHelperTokenSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBitsTokenOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHelperTokenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, BG_TOKEN) -> windows_core::HRESULT,
    pub GetHelperTokenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_TOKEN) -> windows_core::HRESULT,
    pub SetHelperToken: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearHelperToken: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHelperTokenSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumBackgroundCopyFiles, IEnumBackgroundCopyFiles_Vtbl, 0xca51e165_c365_424c_8d41_24aaa4ff3c40);
impl core::ops::Deref for IEnumBackgroundCopyFiles {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyFiles, windows_core::IUnknown);
impl IEnumBackgroundCopyFiles {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IBackgroundCopyFile>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumBackgroundCopyFiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumBackgroundCopyGroups, IEnumBackgroundCopyGroups_Vtbl, 0xd993e603_4aa4_47c5_8665_c20d39c2ba4f);
impl core::ops::Deref for IEnumBackgroundCopyGroups {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyGroups, windows_core::IUnknown);
impl IEnumBackgroundCopyGroups {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::GUID], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumBackgroundCopyGroups_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumBackgroundCopyJobs, IEnumBackgroundCopyJobs_Vtbl, 0x1af4f612_3b71_466f_8f58_7b6f73ac57ad);
impl core::ops::Deref for IEnumBackgroundCopyJobs {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyJobs, windows_core::IUnknown);
impl IEnumBackgroundCopyJobs {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IBackgroundCopyJob>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyJobs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumBackgroundCopyJobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumBackgroundCopyJobs1, IEnumBackgroundCopyJobs1_Vtbl, 0x8baeba9d_8f1c_42c4_b82c_09ae79980d25);
impl core::ops::Deref for IEnumBackgroundCopyJobs1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyJobs1, windows_core::IUnknown);
impl IEnumBackgroundCopyJobs1 {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::GUID], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyJobs1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumBackgroundCopyJobs1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumBitsPeerCacheRecords, IEnumBitsPeerCacheRecords_Vtbl, 0x659cdea4_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IEnumBitsPeerCacheRecords {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumBitsPeerCacheRecords, windows_core::IUnknown);
impl IEnumBitsPeerCacheRecords {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IBitsPeerCacheRecord>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumBitsPeerCacheRecords> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumBitsPeerCacheRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumBitsPeers, IEnumBitsPeers_Vtbl, 0x659cdea5_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IEnumBitsPeers {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumBitsPeers, windows_core::IUnknown);
impl IEnumBitsPeers {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IBitsPeer>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumBitsPeers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumBitsPeers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub const BG_AUTH_SCHEME_BASIC: BG_AUTH_SCHEME = BG_AUTH_SCHEME(1i32);
pub const BG_AUTH_SCHEME_DIGEST: BG_AUTH_SCHEME = BG_AUTH_SCHEME(2i32);
pub const BG_AUTH_SCHEME_NEGOTIATE: BG_AUTH_SCHEME = BG_AUTH_SCHEME(4i32);
pub const BG_AUTH_SCHEME_NTLM: BG_AUTH_SCHEME = BG_AUTH_SCHEME(3i32);
pub const BG_AUTH_SCHEME_PASSPORT: BG_AUTH_SCHEME = BG_AUTH_SCHEME(5i32);
pub const BG_AUTH_TARGET_PROXY: BG_AUTH_TARGET = BG_AUTH_TARGET(2i32);
pub const BG_AUTH_TARGET_SERVER: BG_AUTH_TARGET = BG_AUTH_TARGET(1i32);
pub const BG_CERT_STORE_LOCATION_CURRENT_SERVICE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(2i32);
pub const BG_CERT_STORE_LOCATION_CURRENT_USER: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(0i32);
pub const BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(5i32);
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(1i32);
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(7i32);
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(6i32);
pub const BG_CERT_STORE_LOCATION_SERVICES: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(3i32);
pub const BG_CERT_STORE_LOCATION_USERS: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(4i32);
pub const BG_COPY_FILE_ALL: u32 = 15u32;
pub const BG_COPY_FILE_DACL: u32 = 4u32;
pub const BG_COPY_FILE_GROUP: u32 = 2u32;
pub const BG_COPY_FILE_OWNER: u32 = 1u32;
pub const BG_COPY_FILE_SACL: u32 = 8u32;
pub const BG_DISABLE_BRANCH_CACHE: u32 = 4u32;
pub const BG_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
pub const BG_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
pub const BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(2i32);
pub const BG_ERROR_CONTEXT_GENERAL_TRANSPORT: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(6i32);
pub const BG_ERROR_CONTEXT_LOCAL_FILE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(4i32);
pub const BG_ERROR_CONTEXT_NONE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(0i32);
pub const BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(3i32);
pub const BG_ERROR_CONTEXT_REMOTE_APPLICATION: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(7i32);
pub const BG_ERROR_CONTEXT_REMOTE_FILE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(5i32);
pub const BG_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(8i32);
pub const BG_ERROR_CONTEXT_UNKNOWN: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(1i32);
pub const BG_E_APP_PACKAGE_NOT_FOUND: i32 = -2145386390i32;
pub const BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED: i32 = -2145386389i32;
pub const BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY: i32 = -2145386386i32;
pub const BG_E_BLOCKED_BY_BATTERY_POLICY: i32 = -2145386393i32;
pub const BG_E_BLOCKED_BY_BATTERY_SAVER: i32 = -2145386392i32;
pub const BG_E_BLOCKED_BY_COST_TRANSFER_POLICY: i32 = -2145386407i32;
pub const BG_E_BLOCKED_BY_GAME_MODE: i32 = -2145386385i32;
pub const BG_E_BLOCKED_BY_POLICY: i32 = -2145386434i32;
pub const BG_E_BLOCKED_BY_SYSTEM_POLICY: i32 = -2145386384i32;
pub const BG_E_BUSYCACHERECORD: i32 = -2145386424i32;
pub const BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH: i32 = -2145386462i32;
pub const BG_E_COMMIT_IN_PROGRESS: i32 = -2145386429i32;
pub const BG_E_CONNECTION_CLOSED: i32 = -2145386450i32;
pub const BG_E_CONNECT_FAILURE: i32 = -2145386451i32;
pub const BG_E_DATABASE_CORRUPT: i32 = -2145386388i32;
pub const BG_E_DESTINATION_LOCKED: i32 = -2145386483i32;
pub const BG_E_DISCOVERY_IN_PROGRESS: i32 = -2145386428i32;
pub const BG_E_EMPTY: i32 = -2145386493i32;
pub const BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: i32 = -2145386488i32;
pub const BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT: i32 = -2145386485i32;
pub const BG_E_ERROR_CONTEXT_LOCAL_FILE: i32 = -2145386487i32;
pub const BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: i32 = -2145386484i32;
pub const BG_E_ERROR_CONTEXT_REMOTE_APPLICATION: i32 = -2145386466i32;
pub const BG_E_ERROR_CONTEXT_REMOTE_FILE: i32 = -2145386486i32;
pub const BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: i32 = -2145386378i32;
pub const BG_E_ERROR_CONTEXT_UNKNOWN: i32 = -2145386489i32;
pub const BG_E_ERROR_INFORMATION_UNAVAILABLE: i32 = -2145386481i32;
pub const BG_E_FILE_NOT_AVAILABLE: i32 = -2145386492i32;
pub const BG_E_FILE_NOT_FOUND: i32 = -2145386455i32;
pub const BG_E_HTTP_ERROR_100: i32 = -2145845148i32;
pub const BG_E_HTTP_ERROR_101: i32 = -2145845147i32;
pub const BG_E_HTTP_ERROR_200: i32 = -2145845048i32;
pub const BG_E_HTTP_ERROR_201: i32 = -2145845047i32;
pub const BG_E_HTTP_ERROR_202: i32 = -2145845046i32;
pub const BG_E_HTTP_ERROR_203: i32 = -2145845045i32;
pub const BG_E_HTTP_ERROR_204: i32 = -2145845044i32;
pub const BG_E_HTTP_ERROR_205: i32 = -2145845043i32;
pub const BG_E_HTTP_ERROR_206: i32 = -2145845042i32;
pub const BG_E_HTTP_ERROR_300: i32 = -2145844948i32;
pub const BG_E_HTTP_ERROR_301: i32 = -2145844947i32;
pub const BG_E_HTTP_ERROR_302: i32 = -2145844946i32;
pub const BG_E_HTTP_ERROR_303: i32 = -2145844945i32;
pub const BG_E_HTTP_ERROR_304: i32 = -2145844944i32;
pub const BG_E_HTTP_ERROR_305: i32 = -2145844943i32;
pub const BG_E_HTTP_ERROR_307: i32 = -2145844941i32;
pub const BG_E_HTTP_ERROR_400: i32 = -2145844848i32;
pub const BG_E_HTTP_ERROR_401: i32 = -2145844847i32;
pub const BG_E_HTTP_ERROR_402: i32 = -2145844846i32;
pub const BG_E_HTTP_ERROR_403: i32 = -2145844845i32;
pub const BG_E_HTTP_ERROR_404: i32 = -2145844844i32;
pub const BG_E_HTTP_ERROR_405: i32 = -2145844843i32;
pub const BG_E_HTTP_ERROR_406: i32 = -2145844842i32;
pub const BG_E_HTTP_ERROR_407: i32 = -2145844841i32;
pub const BG_E_HTTP_ERROR_408: i32 = -2145844840i32;
pub const BG_E_HTTP_ERROR_409: i32 = -2145844839i32;
pub const BG_E_HTTP_ERROR_410: i32 = -2145844838i32;
pub const BG_E_HTTP_ERROR_411: i32 = -2145844837i32;
pub const BG_E_HTTP_ERROR_412: i32 = -2145844836i32;
pub const BG_E_HTTP_ERROR_413: i32 = -2145844835i32;
pub const BG_E_HTTP_ERROR_414: i32 = -2145844834i32;
pub const BG_E_HTTP_ERROR_415: i32 = -2145844833i32;
pub const BG_E_HTTP_ERROR_416: i32 = -2145844832i32;
pub const BG_E_HTTP_ERROR_417: i32 = -2145844831i32;
pub const BG_E_HTTP_ERROR_449: i32 = -2145844799i32;
pub const BG_E_HTTP_ERROR_500: i32 = -2145844748i32;
pub const BG_E_HTTP_ERROR_501: i32 = -2145844747i32;
pub const BG_E_HTTP_ERROR_502: i32 = -2145844746i32;
pub const BG_E_HTTP_ERROR_503: i32 = -2145844745i32;
pub const BG_E_HTTP_ERROR_504: i32 = -2145844744i32;
pub const BG_E_HTTP_ERROR_505: i32 = -2145844743i32;
pub const BG_E_INSUFFICIENT_HTTP_SUPPORT: i32 = -2145386478i32;
pub const BG_E_INSUFFICIENT_RANGE_SUPPORT: i32 = -2145386477i32;
pub const BG_E_INVALID_AUTH_SCHEME: i32 = -2145386456i32;
pub const BG_E_INVALID_AUTH_TARGET: i32 = -2145386457i32;
pub const BG_E_INVALID_CREDENTIALS: i32 = -2145386432i32;
pub const BG_E_INVALID_HASH_ALGORITHM: i32 = -2145386431i32;
pub const BG_E_INVALID_PROXY_INFO: i32 = -2145386433i32;
pub const BG_E_INVALID_RANGE: i32 = -2145386453i32;
pub const BG_E_INVALID_SERVER_RESPONSE: i32 = -2145386469i32;
pub const BG_E_INVALID_STATE: i32 = -2145386494i32;
pub const BG_E_LOCAL_FILE_CHANGED: i32 = -2145386467i32;
pub const BG_E_MAXDOWNLOAD_TIMEOUT: i32 = -2145386412i32;
pub const BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE: i32 = -2145386397i32;
pub const BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED: i32 = -2145386396i32;
pub const BG_E_MISSING_FILE_SIZE: i32 = -2145386479i32;
pub const BG_E_NETWORK_DISCONNECTED: i32 = -2145386480i32;
pub const BG_E_NEW_OWNER_DIFF_MAPPING: i32 = -2145386475i32;
pub const BG_E_NEW_OWNER_NO_FILE_ACCESS: i32 = -2145386474i32;
pub const BG_E_NOT_FOUND: i32 = -2145386495i32;
pub const BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD: i32 = -2145386383i32;
pub const BG_E_NO_PROGRESS: i32 = -2145386460i32;
pub const BG_E_OVERLAPPING_RANGES: i32 = -2145386452i32;
pub const BG_E_PASSWORD_TOO_LARGE: i32 = -2145386458i32;
pub const BG_E_PEERCACHING_DISABLED: i32 = -2145386425i32;
pub const BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY: i32 = -2145386400i32;
pub const BG_E_PROTOCOL_NOT_AVAILABLE: i32 = -2145386491i32;
pub const BG_E_PROXY_BYPASS_LIST_TOO_LARGE: i32 = -2145386471i32;
pub const BG_E_PROXY_LIST_TOO_LARGE: i32 = -2145386472i32;
pub const BG_E_RANDOM_ACCESS_NOT_SUPPORTED: i32 = -2145386387i32;
pub const BG_E_READ_ONLY_PROPERTY: i32 = -2145386408i32;
pub const BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE: i32 = -2145386399i32;
pub const BG_E_READ_ONLY_PROPERTY_AFTER_RESUME: i32 = -2145386398i32;
pub const BG_E_READ_ONLY_WHEN_JOB_ACTIVE: i32 = -2145386379i32;
pub const BG_E_RECORD_DELETED: i32 = -2145386430i32;
pub const BG_E_REMOTE_FILE_CHANGED: i32 = -2145386381i32;
pub const BG_E_REMOTE_NOT_SUPPORTED: i32 = -2145386476i32;
pub const BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED: i32 = -2145386380i32;
pub const BG_E_SERVER_EXECUTE_ENABLE: i32 = -2145386461i32;
pub const BG_E_SESSION_NOT_FOUND: i32 = -2145386465i32;
pub const BG_E_STANDBY_MODE: i32 = -2145386395i32;
pub const BG_E_STRING_TOO_LONG: i32 = -2145386463i32;
pub const BG_E_TEST_OPTION_BLOCKED_DOWNLOAD: i32 = -2145386426i32;
pub const BG_E_TOKEN_REQUIRED: i32 = -2145386410i32;
pub const BG_E_TOO_LARGE: i32 = -2145386464i32;
pub const BG_E_TOO_MANY_FILES: i32 = -2145386468i32;
pub const BG_E_TOO_MANY_FILES_IN_JOB: i32 = -2145386415i32;
pub const BG_E_TOO_MANY_JOBS_PER_MACHINE: i32 = -2145386416i32;
pub const BG_E_TOO_MANY_JOBS_PER_USER: i32 = -2145386423i32;
pub const BG_E_TOO_MANY_RANGES_IN_FILE: i32 = -2145386414i32;
pub const BG_E_UNKNOWN_PROPERTY_ID: i32 = -2145386409i32;
pub const BG_E_UNSUPPORTED_JOB_CONFIGURATION: i32 = -2145386382i32;
pub const BG_E_UPNP_ERROR: i32 = -2145386427i32;
pub const BG_E_USERNAME_TOO_LARGE: i32 = -2145386459i32;
pub const BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED: i32 = -2145386394i32;
pub const BG_E_VALIDATION_FAILED: i32 = -2145386413i32;
pub const BG_E_VOLUME_CHANGED: i32 = -2145386482i32;
pub const BG_E_WATCHDOG_TIMEOUT: i32 = -2145386391i32;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: u32 = 2048u32;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: u32 = 256u32;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: u32 = 0u32;
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: u32 = 512u32;
pub const BG_HTTP_REDIRECT_POLICY_MASK: u32 = 1792u32;
pub const BG_JOB_DISABLE_BRANCH_CACHE: u32 = 4u32;
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
pub const BG_JOB_ENUM_ALL_USERS: u32 = 1u32;
pub const BG_JOB_PRIORITY_FOREGROUND: BG_JOB_PRIORITY = BG_JOB_PRIORITY(0i32);
pub const BG_JOB_PRIORITY_HIGH: BG_JOB_PRIORITY = BG_JOB_PRIORITY(1i32);
pub const BG_JOB_PRIORITY_LOW: BG_JOB_PRIORITY = BG_JOB_PRIORITY(3i32);
pub const BG_JOB_PRIORITY_NORMAL: BG_JOB_PRIORITY = BG_JOB_PRIORITY(2i32);
pub const BG_JOB_PROXY_USAGE_AUTODETECT: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(3i32);
pub const BG_JOB_PROXY_USAGE_NO_PROXY: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(1i32);
pub const BG_JOB_PROXY_USAGE_OVERRIDE: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(2i32);
pub const BG_JOB_PROXY_USAGE_PRECONFIG: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(0i32);
pub const BG_JOB_STATE_ACKNOWLEDGED: BG_JOB_STATE = BG_JOB_STATE(7i32);
pub const BG_JOB_STATE_CANCELLED: BG_JOB_STATE = BG_JOB_STATE(8i32);
pub const BG_JOB_STATE_CONNECTING: BG_JOB_STATE = BG_JOB_STATE(1i32);
pub const BG_JOB_STATE_ERROR: BG_JOB_STATE = BG_JOB_STATE(4i32);
pub const BG_JOB_STATE_QUEUED: BG_JOB_STATE = BG_JOB_STATE(0i32);
pub const BG_JOB_STATE_SUSPENDED: BG_JOB_STATE = BG_JOB_STATE(3i32);
pub const BG_JOB_STATE_TRANSFERRED: BG_JOB_STATE = BG_JOB_STATE(6i32);
pub const BG_JOB_STATE_TRANSFERRING: BG_JOB_STATE = BG_JOB_STATE(2i32);
pub const BG_JOB_STATE_TRANSIENT_ERROR: BG_JOB_STATE = BG_JOB_STATE(5i32);
pub const BG_JOB_TYPE_DOWNLOAD: BG_JOB_TYPE = BG_JOB_TYPE(0i32);
pub const BG_JOB_TYPE_UPLOAD: BG_JOB_TYPE = BG_JOB_TYPE(1i32);
pub const BG_JOB_TYPE_UPLOAD_REPLY: BG_JOB_TYPE = BG_JOB_TYPE(2i32);
pub const BG_NOTIFY_DISABLE: u32 = 4u32;
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: u32 = 32u32;
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 16u32;
pub const BG_NOTIFY_JOB_ERROR: u32 = 2u32;
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 8u32;
pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 1u32;
pub const BG_SSL_ENABLE_CRL_CHECK: u32 = 1u32;
pub const BG_SSL_IGNORE_CERT_CN_INVALID: u32 = 2u32;
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: u32 = 4u32;
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: u32 = 16u32;
pub const BG_SSL_IGNORE_UNKNOWN_CA: u32 = 8u32;
pub const BG_S_ERROR_CONTEXT_NONE: i32 = 2097158i32;
pub const BG_S_OVERRIDDEN_BY_POLICY: i32 = 2097237i32;
pub const BG_S_PARTIAL_COMPLETE: i32 = 2097175i32;
pub const BG_S_PROXY_CHANGED: i32 = 2097194i32;
pub const BG_S_UNABLE_TO_DELETE_FILES: i32 = 2097178i32;
pub const BG_TOKEN_LOCAL_FILE: BG_TOKEN = BG_TOKEN(1u32);
pub const BG_TOKEN_NETWORK: BG_TOKEN = BG_TOKEN(2u32);
pub const BITS_COST_OPTION_IGNORE_CONGESTION: u32 = 2147483648u32;
pub const BITS_COST_STATE_BELOW_CAP: u32 = 4u32;
pub const BITS_COST_STATE_CAPPED_USAGE_UNKNOWN: u32 = 2u32;
pub const BITS_COST_STATE_NEAR_CAP: u32 = 8u32;
pub const BITS_COST_STATE_OVERCAP_CHARGED: u32 = 16u32;
pub const BITS_COST_STATE_OVERCAP_THROTTLED: u32 = 32u32;
pub const BITS_COST_STATE_RESERVED: u32 = 1073741824u32;
pub const BITS_COST_STATE_ROAMING: u32 = 128u32;
pub const BITS_COST_STATE_UNRESTRICTED: u32 = 1u32;
pub const BITS_COST_STATE_USAGE_BASED: u32 = 64u32;
pub const BITS_FILE_PROPERTY_ID_HTTP_RESPONSE_HEADERS: BITS_FILE_PROPERTY_ID = BITS_FILE_PROPERTY_ID(1i32);
pub const BITS_JOB_PROPERTY_DYNAMIC_CONTENT: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(3i32);
pub const BITS_JOB_PROPERTY_HIGH_PERFORMANCE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(4i32);
pub const BITS_JOB_PROPERTY_ID_COST_FLAGS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(1i32);
pub const BITS_JOB_PROPERTY_MAX_DOWNLOAD_SIZE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(5i32);
pub const BITS_JOB_PROPERTY_MINIMUM_NOTIFICATION_INTERVAL_MS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(9i32);
pub const BITS_JOB_PROPERTY_NOTIFICATION_CLSID: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(2i32);
pub const BITS_JOB_PROPERTY_ON_DEMAND_MODE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(10i32);
pub const BITS_JOB_PROPERTY_USE_STORED_CREDENTIALS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(7i32);
pub const BITS_JOB_TRANSFER_POLICY_ALWAYS: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483393i32);
pub const BITS_JOB_TRANSFER_POLICY_NOT_ROAMING: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483521i32);
pub const BITS_JOB_TRANSFER_POLICY_NO_SURCHARGE: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483537i32);
pub const BITS_JOB_TRANSFER_POLICY_STANDARD: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483545i32);
pub const BITS_JOB_TRANSFER_POLICY_UNRESTRICTED: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483615i32);
pub const BITS_MC_FAILED_TO_START: i32 = -2145828856i32;
pub const BITS_MC_FATAL_IGD_ERROR: i32 = -2145828855i32;
pub const BITS_MC_FILE_DELETION_FAILED: i32 = -2145828863i32;
pub const BITS_MC_FILE_DELETION_FAILED_MORE: i32 = -2145828862i32;
pub const BITS_MC_JOB_CANCELLED: i32 = -2145828864i32;
pub const BITS_MC_JOB_NOTIFICATION_FAILURE: i32 = -2145828858i32;
pub const BITS_MC_JOB_PROPERTY_CHANGE: i32 = -2145828861i32;
pub const BITS_MC_JOB_SCAVENGED: i32 = -2145828859i32;
pub const BITS_MC_JOB_TAKE_OWNERSHIP: i32 = -2145828860i32;
pub const BITS_MC_PEERCACHING_PORT: i32 = -2145828854i32;
pub const BITS_MC_STATE_FILE_CORRUPT: i32 = -2145828857i32;
pub const BITS_MC_WSD_PORT: i32 = -2145828853i32;
pub const GROUPPROP_DESCRIPTION: GROUPPROP = GROUPPROP(12i32);
pub const GROUPPROP_DISPLAYNAME: GROUPPROP = GROUPPROP(11i32);
pub const GROUPPROP_LOCALUSERID: GROUPPROP = GROUPPROP(3i32);
pub const GROUPPROP_LOCALUSERPWD: GROUPPROP = GROUPPROP(4i32);
pub const GROUPPROP_NOTIFYCLSID: GROUPPROP = GROUPPROP(7i32);
pub const GROUPPROP_NOTIFYFLAGS: GROUPPROP = GROUPPROP(6i32);
pub const GROUPPROP_PRIORITY: GROUPPROP = GROUPPROP(0i32);
pub const GROUPPROP_PROGRESSPERCENT: GROUPPROP = GROUPPROP(9i32);
pub const GROUPPROP_PROGRESSSIZE: GROUPPROP = GROUPPROP(8i32);
pub const GROUPPROP_PROGRESSTIME: GROUPPROP = GROUPPROP(10i32);
pub const GROUPPROP_PROTOCOLFLAGS: GROUPPROP = GROUPPROP(5i32);
pub const GROUPPROP_REMOTEUSERID: GROUPPROP = GROUPPROP(1i32);
pub const GROUPPROP_REMOTEUSERPWD: GROUPPROP = GROUPPROP(2i32);
pub const QM_E_DOWNLOADER_UNAVAILABLE: u32 = 2164264963u32;
pub const QM_E_INVALID_STATE: u32 = 2164264961u32;
pub const QM_E_ITEM_NOT_FOUND: u32 = 2164264964u32;
pub const QM_E_SERVICE_UNAVAILABLE: u32 = 2164264962u32;
pub const QM_NOTIFY_DISABLE_NOTIFY: u32 = 64u32;
pub const QM_NOTIFY_FILE_DONE: u32 = 1u32;
pub const QM_NOTIFY_GROUP_DONE: u32 = 4u32;
pub const QM_NOTIFY_JOB_DONE: u32 = 2u32;
pub const QM_NOTIFY_USE_PROGRESSEX: u32 = 128u32;
pub const QM_PROGRESS_PERCENT_DONE: u32 = 1u32;
pub const QM_PROGRESS_SIZE_DONE: u32 = 3u32;
pub const QM_PROGRESS_TIME_DONE: u32 = 2u32;
pub const QM_PROTOCOL_CUSTOM: u32 = 4u32;
pub const QM_PROTOCOL_FTP: u32 = 2u32;
pub const QM_PROTOCOL_HTTP: u32 = 1u32;
pub const QM_PROTOCOL_SMB: u32 = 3u32;
pub const QM_STATUS_FILE_COMPLETE: u32 = 1u32;
pub const QM_STATUS_FILE_INCOMPLETE: u32 = 2u32;
pub const QM_STATUS_GROUP_COMPLETE: u32 = 64u32;
pub const QM_STATUS_GROUP_ERROR: u32 = 512u32;
pub const QM_STATUS_GROUP_FOREGROUND: u32 = 1024u32;
pub const QM_STATUS_GROUP_INCOMPLETE: u32 = 128u32;
pub const QM_STATUS_GROUP_SUSPENDED: u32 = 256u32;
pub const QM_STATUS_JOB_COMPLETE: u32 = 4u32;
pub const QM_STATUS_JOB_ERROR: u32 = 16u32;
pub const QM_STATUS_JOB_FOREGROUND: u32 = 32u32;
pub const QM_STATUS_JOB_INCOMPLETE: u32 = 8u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_AUTH_SCHEME(pub i32);
impl windows_core::TypeKind for BG_AUTH_SCHEME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_AUTH_SCHEME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_AUTH_SCHEME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_AUTH_TARGET(pub i32);
impl windows_core::TypeKind for BG_AUTH_TARGET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_AUTH_TARGET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_AUTH_TARGET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_CERT_STORE_LOCATION(pub i32);
impl windows_core::TypeKind for BG_CERT_STORE_LOCATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_CERT_STORE_LOCATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_CERT_STORE_LOCATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_ERROR_CONTEXT(pub i32);
impl windows_core::TypeKind for BG_ERROR_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_ERROR_CONTEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_ERROR_CONTEXT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_JOB_PRIORITY(pub i32);
impl windows_core::TypeKind for BG_JOB_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_JOB_PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_JOB_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_JOB_PROXY_USAGE(pub i32);
impl windows_core::TypeKind for BG_JOB_PROXY_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_JOB_PROXY_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_JOB_PROXY_USAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_JOB_STATE(pub i32);
impl windows_core::TypeKind for BG_JOB_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_JOB_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_JOB_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_JOB_TYPE(pub i32);
impl windows_core::TypeKind for BG_JOB_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_JOB_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_JOB_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BG_TOKEN(pub u32);
impl windows_core::TypeKind for BG_TOKEN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BG_TOKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BG_TOKEN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BITS_FILE_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for BITS_FILE_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BITS_FILE_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BITS_FILE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BITS_JOB_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for BITS_JOB_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BITS_JOB_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BITS_JOB_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BITS_JOB_TRANSFER_POLICY(pub i32);
impl windows_core::TypeKind for BITS_JOB_TRANSFER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BITS_JOB_TRANSFER_POLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BITS_JOB_TRANSFER_POLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GROUPPROP(pub i32);
impl windows_core::TypeKind for GROUPPROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GROUPPROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GROUPPROP").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BG_AUTH_CREDENTIALS {
    pub Target: BG_AUTH_TARGET,
    pub Scheme: BG_AUTH_SCHEME,
    pub Credentials: BG_AUTH_CREDENTIALS_UNION,
}
impl windows_core::TypeKind for BG_AUTH_CREDENTIALS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_AUTH_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BG_AUTH_CREDENTIALS_UNION {
    pub Basic: BG_BASIC_CREDENTIALS,
}
impl windows_core::TypeKind for BG_AUTH_CREDENTIALS_UNION {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_AUTH_CREDENTIALS_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BG_BASIC_CREDENTIALS {
    pub UserName: windows_core::PWSTR,
    pub Password: windows_core::PWSTR,
}
impl windows_core::TypeKind for BG_BASIC_CREDENTIALS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_BASIC_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BG_FILE_INFO {
    pub RemoteName: windows_core::PWSTR,
    pub LocalName: windows_core::PWSTR,
}
impl windows_core::TypeKind for BG_FILE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BG_FILE_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub Completed: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for BG_FILE_PROGRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_FILE_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BG_FILE_RANGE {
    pub InitialOffset: u64,
    pub Length: u64,
}
impl windows_core::TypeKind for BG_FILE_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_FILE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BG_JOB_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub FilesTotal: u32,
    pub FilesTransferred: u32,
}
impl windows_core::TypeKind for BG_JOB_PROGRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_JOB_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BG_JOB_REPLY_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
}
impl windows_core::TypeKind for BG_JOB_REPLY_PROGRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_JOB_REPLY_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BG_JOB_TIMES {
    pub CreationTime: super::super::Foundation::FILETIME,
    pub ModificationTime: super::super::Foundation::FILETIME,
    pub TransferCompletionTime: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for BG_JOB_TIMES {
    type TypeKind = windows_core::CopyType;
}
impl Default for BG_JOB_TIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BITSExtensionSetupFactory: windows_core::GUID = windows_core::GUID::from_u128(0xefbbab68_7286_4783_94bf_9461d8b7e7e9);
#[repr(C)]
#[derive(Clone, Copy)]
pub union BITS_FILE_PROPERTY_VALUE {
    pub String: windows_core::PWSTR,
}
impl windows_core::TypeKind for BITS_FILE_PROPERTY_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for BITS_FILE_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BITS_JOB_PROPERTY_VALUE {
    pub Dword: u32,
    pub ClsID: windows_core::GUID,
    pub Enable: super::super::Foundation::BOOL,
    pub Uint64: u64,
    pub Target: BG_AUTH_TARGET,
}
impl windows_core::TypeKind for BITS_JOB_PROPERTY_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for BITS_JOB_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BackgroundCopyManager: windows_core::GUID = windows_core::GUID::from_u128(0x4991d34b_80a1_4291_83b6_3328366b9097);
pub const BackgroundCopyManager10_1: windows_core::GUID = windows_core::GUID::from_u128(0x4bd3e4e1_7bd4_4a2b_9964_496400de5193);
pub const BackgroundCopyManager10_2: windows_core::GUID = windows_core::GUID::from_u128(0x4575438f_a6c8_4976_b0fe_2f26b80d959e);
pub const BackgroundCopyManager10_3: windows_core::GUID = windows_core::GUID::from_u128(0x5fd42ad5_c04e_4d36_adc7_e08ff15737ad);
pub const BackgroundCopyManager1_5: windows_core::GUID = windows_core::GUID::from_u128(0xf087771f_d74f_4c1a_bb8a_e16aca9124ea);
pub const BackgroundCopyManager2_0: windows_core::GUID = windows_core::GUID::from_u128(0x6d18ad12_bde3_4393_b311_099c346e6df9);
pub const BackgroundCopyManager2_5: windows_core::GUID = windows_core::GUID::from_u128(0x03ca98d6_ff5d_49b8_abc6_03dd84127020);
pub const BackgroundCopyManager3_0: windows_core::GUID = windows_core::GUID::from_u128(0x659cdea7_489e_11d9_a9cd_000d56965251);
pub const BackgroundCopyManager4_0: windows_core::GUID = windows_core::GUID::from_u128(0xbb6df56b_cace_11dc_9992_0019b93a3a84);
pub const BackgroundCopyManager5_0: windows_core::GUID = windows_core::GUID::from_u128(0x1ecca34c_e88a_44e3_8d6a_8921bde9e452);
pub const BackgroundCopyQMgr: windows_core::GUID = windows_core::GUID::from_u128(0x69ad4aee_51be_439b_a92c_86ae490e8b30);
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct FILESETINFO {
    pub bstrRemoteFile: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrLocalFile: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dwSizeHint: u32,
}
impl Clone for FILESETINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for FILESETINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILESETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
