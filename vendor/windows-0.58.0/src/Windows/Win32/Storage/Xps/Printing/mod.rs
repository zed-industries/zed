#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn StartXpsPrintJob<P0, P1, P2, P3, P4>(printername: P0, jobname: P1, outputfilename: P2, progressevent: P3, completionevent: P4, printablepageson: &[u8], xpsprintjob: *mut Option<IXpsPrintJob>, documentstream: *mut Option<IXpsPrintJobStream>, printticketstream: *mut Option<IXpsPrintJobStream>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P4: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("xpsprint.dll" "system" fn StartXpsPrintJob(printername : windows_core::PCWSTR, jobname : windows_core::PCWSTR, outputfilename : windows_core::PCWSTR, progressevent : super::super::super::Foundation:: HANDLE, completionevent : super::super::super::Foundation:: HANDLE, printablepageson : *const u8, printablepagesoncount : u32, xpsprintjob : *mut * mut core::ffi::c_void, documentstream : *mut * mut core::ffi::c_void, printticketstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    StartXpsPrintJob(printername.param().abi(), jobname.param().abi(), outputfilename.param().abi(), progressevent.param().abi(), completionevent.param().abi(), core::mem::transmute(printablepageson.as_ptr()), printablepageson.len().try_into().unwrap(), core::mem::transmute(xpsprintjob), core::mem::transmute(documentstream), core::mem::transmute(printticketstream)).ok()
}
#[inline]
pub unsafe fn StartXpsPrintJob1<P0, P1, P2, P3, P4>(printername: P0, jobname: P1, outputfilename: P2, progressevent: P3, completionevent: P4, xpsprintjob: *mut Option<IXpsPrintJob>, printcontentreceiver: *mut Option<super::IXpsOMPackageTarget>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P4: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("xpsprint.dll" "system" fn StartXpsPrintJob1(printername : windows_core::PCWSTR, jobname : windows_core::PCWSTR, outputfilename : windows_core::PCWSTR, progressevent : super::super::super::Foundation:: HANDLE, completionevent : super::super::super::Foundation:: HANDLE, xpsprintjob : *mut * mut core::ffi::c_void, printcontentreceiver : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    StartXpsPrintJob1(printername.param().abi(), jobname.param().abi(), outputfilename.param().abi(), progressevent.param().abi(), completionevent.param().abi(), core::mem::transmute(xpsprintjob), core::mem::transmute(printcontentreceiver)).ok()
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintDocumentPackageStatusEvent, IPrintDocumentPackageStatusEvent_Vtbl, 0xed90c8ad_5c34_4d05_a1ec_0e8a9b3ad7af);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintDocumentPackageStatusEvent {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintDocumentPackageStatusEvent, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageStatusEvent {
    pub unsafe fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PackageStatusUpdated)(windows_core::Interface::as_raw(self), packagestatus).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPrintDocumentPackageStatusEvent_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub PackageStatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *const PrintDocumentPackageStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintDocumentPackageTarget, IPrintDocumentPackageTarget_Vtbl, 0x1b8efec4_3019_4c27_964e_367202156906);
impl core::ops::Deref for IPrintDocumentPackageTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTarget, windows_core::IUnknown);
impl IPrintDocumentPackageTarget {
    pub unsafe fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPackageTargetTypes)(windows_core::Interface::as_raw(self), targetcount, targettypes).ok()
    }
    pub unsafe fn GetPackageTarget<T>(&self, guidtargettype: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetPackageTarget)(windows_core::Interface::as_raw(self), guidtargettype, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPrintDocumentPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageTargetTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPackageTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintDocumentPackageTarget2, IPrintDocumentPackageTarget2_Vtbl, 0xc560298a_535c_48f9_866a_632540660cb4);
impl core::ops::Deref for IPrintDocumentPackageTarget2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTarget2, windows_core::IUnknown);
impl IPrintDocumentPackageTarget2 {
    pub unsafe fn GetIsTargetIppPrinter(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsTargetIppPrinter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTargetIppPrintDevice<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetTargetIppPrintDevice)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPrintDocumentPackageTarget2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsTargetIppPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetTargetIppPrintDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintDocumentPackageTargetFactory, IPrintDocumentPackageTargetFactory_Vtbl, 0xd2959bf7_b31b_4a3d_9600_712eb1335ba4);
impl core::ops::Deref for IPrintDocumentPackageTargetFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTargetFactory, windows_core::IUnknown);
impl IPrintDocumentPackageTargetFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDocumentPackageTargetForPrintJob<P0, P1, P2, P3>(&self, printername: P0, jobname: P1, joboutputstream: P2, jobprintticketstream: P3) -> windows_core::Result<IPrintDocumentPackageTarget>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
        P3: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDocumentPackageTargetForPrintJob)(windows_core::Interface::as_raw(self), printername.param().abi(), jobname.param().abi(), joboutputstream.param().abi(), jobprintticketstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPrintDocumentPackageTargetFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDocumentPackageTargetForPrintJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDocumentPackageTargetForPrintJob: usize,
}
windows_core::imp::define_interface!(IXpsPrintJob, IXpsPrintJob_Vtbl, 0x5ab89b06_8194_425f_ab3b_d7a96e350161);
impl core::ops::Deref for IXpsPrintJob {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsPrintJob, windows_core::IUnknown);
impl IXpsPrintJob {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetJobStatus(&self, jobstatus: *mut XPS_JOB_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetJobStatus)(windows_core::Interface::as_raw(self), jobstatus).ok()
    }
}
#[repr(C)]
pub struct IXpsPrintJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJobStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_JOB_STATUS) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IXpsPrintJobStream, IXpsPrintJobStream_Vtbl, 0x7a77dc5f_45d6_4dff_9307_d8cb846347ca);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IXpsPrintJobStream {
    type Target = super::super::super::System::Com::ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IXpsPrintJobStream, windows_core::IUnknown, super::super::super::System::Com::ISequentialStream);
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStream {
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IXpsPrintJobStream_Vtbl {
    pub base__: super::super::super::System::Com::ISequentialStream_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: windows_core::GUID = windows_core::GUID::from_u128(0x9cae40a8_ded1_41c9_a9fd_d735ef33aeda);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: windows_core::GUID = windows_core::GUID::from_u128(0x0056bb72_8c9c_4612_bd0f_93012a87099d);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: windows_core::GUID = windows_core::GUID::from_u128(0x63dbd720_8b14_4577_b074_7bb11b596d28);
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(2i32);
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(1i32);
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(3i32);
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(0i32);
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(2i32);
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(1i32);
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(3i32);
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PrintDocumentPackageCompletion(pub i32);
impl windows_core::TypeKind for PrintDocumentPackageCompletion {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PrintDocumentPackageCompletion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrintDocumentPackageCompletion").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_JOB_COMPLETION(pub i32);
impl windows_core::TypeKind for XPS_JOB_COMPLETION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_JOB_COMPLETION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_JOB_COMPLETION").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: windows_core::HRESULT,
}
impl windows_core::TypeKind for PrintDocumentPackageStatus {
    type TypeKind = windows_core::CopyType;
}
impl Default for PrintDocumentPackageStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PrintDocumentPackageTarget: windows_core::GUID = windows_core::GUID::from_u128(0x4842669e_9947_46ea_8ba2_d8cce432c2ca);
pub const PrintDocumentPackageTargetFactory: windows_core::GUID = windows_core::GUID::from_u128(0x348ef17d_6c81_4982_92b4_ee188a43867a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: windows_core::HRESULT,
}
impl windows_core::TypeKind for XPS_JOB_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_JOB_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
