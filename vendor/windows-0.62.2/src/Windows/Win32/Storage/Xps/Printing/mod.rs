#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn StartXpsPrintJob<P0, P1, P2>(printername: P0, jobname: P1, outputfilename: P2, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, printablepageson: &[u8], xpsprintjob: *mut Option<IXpsPrintJob>, documentstream: *mut Option<IXpsPrintJobStream>, printticketstream: *mut Option<IXpsPrintJobStream>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xpsprint.dll" "system" fn StartXpsPrintJob(printername : windows_core::PCWSTR, jobname : windows_core::PCWSTR, outputfilename : windows_core::PCWSTR, progressevent : super::super::super::Foundation:: HANDLE, completionevent : super::super::super::Foundation:: HANDLE, printablepageson : *const u8, printablepagesoncount : u32, xpsprintjob : *mut * mut core::ffi::c_void, documentstream : *mut * mut core::ffi::c_void, printticketstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StartXpsPrintJob(printername.param().abi(), jobname.param().abi(), outputfilename.param().abi(), progressevent, completionevent, core::mem::transmute(printablepageson.as_ptr()), printablepageson.len().try_into().unwrap(), core::mem::transmute(xpsprintjob), core::mem::transmute(documentstream), core::mem::transmute(printticketstream)).ok() }
}
#[inline]
pub unsafe fn StartXpsPrintJob1<P0, P1, P2>(printername: P0, jobname: P1, outputfilename: P2, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut Option<IXpsPrintJob>, printcontentreceiver: *mut Option<super::IXpsOMPackageTarget>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xpsprint.dll" "system" fn StartXpsPrintJob1(printername : windows_core::PCWSTR, jobname : windows_core::PCWSTR, outputfilename : windows_core::PCWSTR, progressevent : super::super::super::Foundation:: HANDLE, completionevent : super::super::super::Foundation:: HANDLE, xpsprintjob : *mut * mut core::ffi::c_void, printcontentreceiver : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StartXpsPrintJob1(printername.param().abi(), jobname.param().abi(), outputfilename.param().abi(), progressevent, completionevent, core::mem::transmute(xpsprintjob), core::mem::transmute(printcontentreceiver)).ok() }
}
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: windows_core::GUID = windows_core::GUID::from_u128(0x9cae40a8_ded1_41c9_a9fd_d735ef33aeda);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: windows_core::GUID = windows_core::GUID::from_u128(0x0056bb72_8c9c_4612_bd0f_93012a87099d);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: windows_core::GUID = windows_core::GUID::from_u128(0x63dbd720_8b14_4577_b074_7bb11b596d28);
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
        unsafe { (windows_core::Interface::vtable(self).PackageStatusUpdated)(windows_core::Interface::as_raw(self), packagestatus).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageStatusEvent_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub PackageStatusUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *const PrintDocumentPackageStatus) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintDocumentPackageStatusEvent_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintDocumentPackageStatusEvent_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PackageStatusUpdated<Identity: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDocumentPackageStatusEvent_Impl::PackageStatusUpdated(this, core::mem::transmute_copy(&packagestatus)).into()
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PackageStatusUpdated: PackageStatusUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageStatusEvent as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintDocumentPackageStatusEvent {}
windows_core::imp::define_interface!(IPrintDocumentPackageTarget, IPrintDocumentPackageTarget_Vtbl, 0x1b8efec4_3019_4c27_964e_367202156906);
windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTarget, windows_core::IUnknown);
impl IPrintDocumentPackageTarget {
    pub unsafe fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPackageTargetTypes)(windows_core::Interface::as_raw(self), targetcount as _, targettypes as _).ok() }
    }
    pub unsafe fn GetPackageTarget<T>(&self, guidtargettype: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPackageTarget)(windows_core::Interface::as_raw(self), guidtargettype, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageTargetTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPackageTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintDocumentPackageTarget_Impl: windows_core::IUnknownImpl {
    fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPackageTarget(&self, guidtargettype: *const windows_core::GUID, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IPrintDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPackageTargetTypes<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDocumentPackageTarget_Impl::GetPackageTargetTypes(this, core::mem::transmute_copy(&targetcount), core::mem::transmute_copy(&targettypes)).into()
            }
        }
        unsafe extern "system" fn GetPackageTarget<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtargettype: *const windows_core::GUID, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDocumentPackageTarget_Impl::GetPackageTarget(this, core::mem::transmute_copy(&guidtargettype), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvtarget)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDocumentPackageTarget_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageTargetTypes: GetPackageTargetTypes::<Identity, OFFSET>,
            GetPackageTarget: GetPackageTarget::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintDocumentPackageTarget {}
windows_core::imp::define_interface!(IPrintDocumentPackageTarget2, IPrintDocumentPackageTarget2_Vtbl, 0xc560298a_535c_48f9_866a_632540660cb4);
windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTarget2, windows_core::IUnknown);
impl IPrintDocumentPackageTarget2 {
    pub unsafe fn GetIsTargetIppPrinter(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsTargetIppPrinter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTargetIppPrintDevice<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetTargetIppPrintDevice)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsTargetIppPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetTargetIppPrintDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintDocumentPackageTarget2_Impl: windows_core::IUnknownImpl {
    fn GetIsTargetIppPrinter(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetTargetIppPrintDevice(&self, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IPrintDocumentPackageTarget2_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsTargetIppPrinter<Identity: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isippprinter: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintDocumentPackageTarget2_Impl::GetIsTargetIppPrinter(this) {
                    Ok(ok__) => {
                        isippprinter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetIppPrintDevice<Identity: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDocumentPackageTarget2_Impl::GetTargetIppPrintDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvtarget)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsTargetIppPrinter: GetIsTargetIppPrinter::<Identity, OFFSET>,
            GetTargetIppPrintDevice: GetTargetIppPrintDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintDocumentPackageTarget2 {}
windows_core::imp::define_interface!(IPrintDocumentPackageTargetFactory, IPrintDocumentPackageTargetFactory_Vtbl, 0xd2959bf7_b31b_4a3d_9600_712eb1335ba4);
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDocumentPackageTargetForPrintJob)(windows_core::Interface::as_raw(self), printername.param().abi(), jobname.param().abi(), joboutputstream.param().abi(), jobprintticketstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTargetFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDocumentPackageTargetForPrintJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDocumentPackageTargetForPrintJob: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageTargetFactory_Impl: windows_core::IUnknownImpl {
    fn CreateDocumentPackageTargetForPrintJob(&self, printername: &windows_core::PCWSTR, jobname: &windows_core::PCWSTR, joboutputstream: windows_core::Ref<super::super::super::System::Com::IStream>, jobprintticketstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IPrintDocumentPackageTarget>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageTargetFactory_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Identity: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printername: windows_core::PCWSTR, jobname: windows_core::PCWSTR, joboutputstream: *mut core::ffi::c_void, jobprintticketstream: *mut core::ffi::c_void, docpackagetarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintDocumentPackageTargetFactory_Impl::CreateDocumentPackageTargetForPrintJob(this, core::mem::transmute(&printername), core::mem::transmute(&jobname), core::mem::transmute_copy(&joboutputstream), core::mem::transmute_copy(&jobprintticketstream)) {
                    Ok(ok__) => {
                        docpackagetarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDocumentPackageTargetForPrintJob: CreateDocumentPackageTargetForPrintJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTargetFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintDocumentPackageTargetFactory {}
windows_core::imp::define_interface!(IXpsPrintJob, IXpsPrintJob_Vtbl, 0x5ab89b06_8194_425f_ab3b_d7a96e350161);
windows_core::imp::interface_hierarchy!(IXpsPrintJob, windows_core::IUnknown);
impl IXpsPrintJob {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetJobStatus(&self, jobstatus: *mut XPS_JOB_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetJobStatus)(windows_core::Interface::as_raw(self), jobstatus as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJobStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_JOB_STATUS) -> windows_core::HRESULT,
}
pub trait IXpsPrintJob_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn GetJobStatus(&self, jobstatus: *mut XPS_JOB_STATUS) -> windows_core::Result<()>;
}
impl IXpsPrintJob_Vtbl {
    pub const fn new<Identity: IXpsPrintJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPrintJob_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn GetJobStatus<Identity: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPrintJob_Impl::GetJobStatus(this, core::mem::transmute_copy(&jobstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            GetJobStatus: GetJobStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPrintJob as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsPrintJob {}
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
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJobStream_Vtbl {
    pub base__: super::super::super::System::Com::ISequentialStream_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStream_Impl: super::super::super::System::Com::ISequentialStream_Impl {
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStream_Vtbl {
    pub const fn new<Identity: IXpsPrintJobStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IXpsPrintJobStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPrintJobStream_Impl::Close(this).into()
            }
        }
        Self { base__: super::super::super::System::Com::ISequentialStream_Vtbl::new::<Identity, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPrintJobStream as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::ISequentialStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsPrintJobStream {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintDocumentPackageCompletion(pub i32);
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(2i32);
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(1i32);
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(3i32);
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: windows_core::HRESULT,
}
pub const PrintDocumentPackageTarget: windows_core::GUID = windows_core::GUID::from_u128(0x4842669e_9947_46ea_8ba2_d8cce432c2ca);
pub const PrintDocumentPackageTargetFactory: windows_core::GUID = windows_core::GUID::from_u128(0x348ef17d_6c81_4982_92b4_ee188a43867a);
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(2i32);
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_JOB_COMPLETION(pub i32);
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(3i32);
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: windows_core::HRESULT,
}
