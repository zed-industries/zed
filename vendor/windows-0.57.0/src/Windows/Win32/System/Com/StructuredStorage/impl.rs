pub trait IDirectWriterLock_Impl: Sized {
    fn WaitForWriteAccess(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn ReleaseWriteAccess(&self) -> windows_core::Result<()>;
    fn HaveWriteAccess(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectWriterLock {}
impl IDirectWriterLock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: isize>() -> IDirectWriterLock_Vtbl {
        unsafe extern "system" fn WaitForWriteAccess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectWriterLock_Impl::WaitForWriteAccess(this, core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn ReleaseWriteAccess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectWriterLock_Impl::ReleaseWriteAccess(this).into()
        }
        unsafe extern "system" fn HaveWriteAccess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectWriterLock_Impl::HaveWriteAccess(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WaitForWriteAccess: WaitForWriteAccess::<Identity, Impl, OFFSET>,
            ReleaseWriteAccess: ReleaseWriteAccess::<Identity, Impl, OFFSET>,
            HaveWriteAccess: HaveWriteAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectWriterLock as windows_core::Interface>::IID
    }
}
pub trait IEnumSTATPROPSETSTG_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
impl windows_core::RuntimeName for IEnumSTATPROPSETSTG {}
impl IEnumSTATPROPSETSTG_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>() -> IEnumSTATPROPSETSTG_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATPROPSETSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATPROPSETSTG_Impl::Skip(this, core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATPROPSETSTG_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSTATPROPSETSTG_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATPROPSETSTG as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Variant")]
pub trait IEnumSTATPROPSTG_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::RuntimeName for IEnumSTATPROPSTG {}
#[cfg(feature = "Win32_System_Variant")]
impl IEnumSTATPROPSTG_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>() -> IEnumSTATPROPSTG_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATPROPSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATPROPSTG_Impl::Skip(this, core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATPROPSTG_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSTATPROPSTG_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATPROPSTG as windows_core::Interface>::IID
    }
}
pub trait IEnumSTATSTG_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATSTG>;
}
impl windows_core::RuntimeName for IEnumSTATSTG {}
impl IEnumSTATSTG_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>() -> IEnumSTATSTG_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATSTG_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSTATSTG_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSTATSTG_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATSTG as windows_core::Interface>::IID
    }
}
pub trait IFillLockBytes_Impl: Sized {
    fn FillAppend(&self, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn FillAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn SetFillSize(&self, ulsize: u64) -> windows_core::Result<()>;
    fn Terminate(&self, bcanceled: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFillLockBytes {}
impl IFillLockBytes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: isize>() -> IFillLockBytes_Vtbl {
        unsafe extern "system" fn FillAppend<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFillLockBytes_Impl::FillAppend(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                Ok(ok__) => {
                    core::ptr::write(pcbwritten, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFillLockBytes_Impl::FillAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                Ok(ok__) => {
                    core::ptr::write(pcbwritten, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFillLockBytes_Impl::SetFillSize(this, core::mem::transmute_copy(&ulsize)).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFillLockBytes_Impl::Terminate(this, core::mem::transmute_copy(&bcanceled)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FillAppend: FillAppend::<Identity, Impl, OFFSET>,
            FillAt: FillAt::<Identity, Impl, OFFSET>,
            SetFillSize: SetFillSize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFillLockBytes as windows_core::Interface>::IID
    }
}
pub trait ILayoutStorage_Impl: Sized {
    fn LayoutScript(&self, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> windows_core::Result<()>;
    fn BeginMonitor(&self) -> windows_core::Result<()>;
    fn EndMonitor(&self) -> windows_core::Result<()>;
    fn ReLayoutDocfile(&self, pwcsnewdfname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReLayoutDocfileOnILockBytes(&self, pilockbytes: Option<&ILockBytes>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILayoutStorage {}
impl ILayoutStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: isize>() -> ILayoutStorage_Vtbl {
        unsafe extern "system" fn LayoutScript<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILayoutStorage_Impl::LayoutScript(this, core::mem::transmute_copy(&pstoragelayout), core::mem::transmute_copy(&nentries), core::mem::transmute_copy(&glfinterleavedflag)).into()
        }
        unsafe extern "system" fn BeginMonitor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILayoutStorage_Impl::BeginMonitor(this).into()
        }
        unsafe extern "system" fn EndMonitor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILayoutStorage_Impl::EndMonitor(this).into()
        }
        unsafe extern "system" fn ReLayoutDocfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsnewdfname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILayoutStorage_Impl::ReLayoutDocfile(this, core::mem::transmute(&pwcsnewdfname)).into()
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pilockbytes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILayoutStorage_Impl::ReLayoutDocfileOnILockBytes(this, windows_core::from_raw_borrowed(&pilockbytes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LayoutScript: LayoutScript::<Identity, Impl, OFFSET>,
            BeginMonitor: BeginMonitor::<Identity, Impl, OFFSET>,
            EndMonitor: EndMonitor::<Identity, Impl, OFFSET>,
            ReLayoutDocfile: ReLayoutDocfile::<Identity, Impl, OFFSET>,
            ReLayoutDocfileOnILockBytes: ReLayoutDocfileOnILockBytes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILayoutStorage as windows_core::Interface>::IID
    }
}
pub trait ILockBytes_Impl: Sized {
    fn ReadAt(&self, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::Result<()>;
    fn WriteAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn SetSize(&self, cb: u64) -> windows_core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILockBytes {}
impl ILockBytes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>() -> ILockBytes_Vtbl {
        unsafe extern "system" fn ReadAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILockBytes_Impl::ReadAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn WriteAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILockBytes_Impl::WriteAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILockBytes_Impl::Flush(this).into()
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILockBytes_Impl::SetSize(this, core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn LockRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILockBytes_Impl::LockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn UnlockRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILockBytes_Impl::UnlockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn Stat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILockBytes_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute_copy(&grfstatflag)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadAt: ReadAt::<Identity, Impl, OFFSET>,
            WriteAt: WriteAt::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            LockRegion: LockRegion::<Identity, Impl, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILockBytes as windows_core::Interface>::IID
    }
}
pub trait IMemoryAllocator_Impl: Sized {
    fn Allocate(&self, cbsize: u32) -> *mut core::ffi::c_void;
    fn Free(&self, pv: *mut core::ffi::c_void);
}
impl IMemoryAllocator_Vtbl {
    pub const fn new<Impl: IMemoryAllocator_Impl>() -> IMemoryAllocator_Vtbl {
        unsafe extern "system" fn Allocate<Impl: IMemoryAllocator_Impl>(this: *mut core::ffi::c_void, cbsize: u32) -> *mut core::ffi::c_void {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IMemoryAllocator_Impl::Allocate(this, core::mem::transmute_copy(&cbsize))
        }
        unsafe extern "system" fn Free<Impl: IMemoryAllocator_Impl>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IMemoryAllocator_Impl::Free(this, core::mem::transmute_copy(&pv))
        }
        Self { Allocate: Allocate::<Impl>, Free: Free::<Impl> }
    }
}
#[doc(hidden)]
struct IMemoryAllocator_ImplVtbl<T: IMemoryAllocator_Impl>(std::marker::PhantomData<T>);
impl<T: IMemoryAllocator_Impl> IMemoryAllocator_ImplVtbl<T> {
    const VTABLE: IMemoryAllocator_Vtbl = IMemoryAllocator_Vtbl::new::<T>();
}
impl IMemoryAllocator {
    pub fn new<'a, T: IMemoryAllocator_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IMemoryAllocator_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IPersistStorage_Impl: Sized + super::IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn InitNew(&self, pstg: Option<&IStorage>) -> windows_core::Result<()>;
    fn Load(&self, pstg: Option<&IStorage>) -> windows_core::Result<()>;
    fn Save(&self, pstgsave: Option<&IStorage>, fsameasload: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pstgnew: Option<&IStorage>) -> windows_core::Result<()>;
    fn HandsOffStorage(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPersistStorage {}
impl IPersistStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: isize>() -> IPersistStorage_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistStorage_Impl::IsDirty(this)
        }
        unsafe extern "system" fn InitNew<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistStorage_Impl::InitNew(this, windows_core::from_raw_borrowed(&pstg)).into()
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistStorage_Impl::Load(this, windows_core::from_raw_borrowed(&pstg)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstgsave: *mut core::ffi::c_void, fsameasload: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistStorage_Impl::Save(this, windows_core::from_raw_borrowed(&pstgsave), core::mem::transmute_copy(&fsameasload)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstgnew: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistStorage_Impl::SaveCompleted(this, windows_core::from_raw_borrowed(&pstgnew)).into()
        }
        unsafe extern "system" fn HandsOffStorage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistStorage_Impl::HandsOffStorage(this).into()
        }
        Self {
            base__: super::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStorage as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID
    }
}
pub trait IPropertyBag_Impl: Sized {
    fn Read(&self, pszpropname: &windows_core::PCWSTR, pvar: *mut windows_core::VARIANT, perrorlog: Option<&super::IErrorLog>) -> windows_core::Result<()>;
    fn Write(&self, pszpropname: &windows_core::PCWSTR, pvar: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyBag {}
impl IPropertyBag_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag_Impl, const OFFSET: isize>() -> IPropertyBag_Vtbl {
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyBag_Impl::Read(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar), windows_core::from_raw_borrowed(&perrorlog)).into()
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyBag_Impl::Write(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, Impl, OFFSET>, Write: Write::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Variant")]
pub trait IPropertyBag2_Impl: Sized {
    fn Read(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: Option<&super::IErrorLog>, pvarvalue: *mut windows_core::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn CountProperties(&self) -> windows_core::Result<u32>;
    fn GetPropertyInfo(&self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> windows_core::Result<()>;
    fn LoadObject(&self, pstrname: &windows_core::PCWSTR, dwhint: u32, punkobject: Option<&windows_core::IUnknown>, perrlog: Option<&super::IErrorLog>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::RuntimeName for IPropertyBag2 {}
#[cfg(feature = "Win32_System_Variant")]
impl IPropertyBag2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: isize>() -> IPropertyBag2_Vtbl {
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>, phrerror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyBag2_Impl::Read(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), windows_core::from_raw_borrowed(&perrlog), core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&phrerror)).into()
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyBag2_Impl::Write(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn CountProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcproperties: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertyBag2_Impl::CountProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(pcproperties, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyBag2_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&iproperty), core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pcproperties)).into()
        }
        unsafe extern "system" fn LoadObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR, dwhint: u32, punkobject: *mut core::ffi::c_void, perrlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyBag2_Impl::LoadObject(this, core::mem::transmute(&pstrname), core::mem::transmute_copy(&dwhint), windows_core::from_raw_borrowed(&punkobject), windows_core::from_raw_borrowed(&perrlog)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            CountProperties: CountProperties::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            LoadObject: LoadObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyBag2 as windows_core::Interface>::IID
    }
}
pub trait IPropertySetStorage_Impl: Sized {
    fn Create(&self, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32) -> windows_core::Result<IPropertyStorage>;
    fn Open(&self, rfmtid: *const windows_core::GUID, grfmode: u32) -> windows_core::Result<IPropertyStorage>;
    fn Delete(&self, rfmtid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
impl windows_core::RuntimeName for IPropertySetStorage {}
impl IPropertySetStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: isize>() -> IPropertySetStorage_Vtbl {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertySetStorage_Impl::Create(this, core::mem::transmute_copy(&rfmtid), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&grfmode)) {
                Ok(ok__) => {
                    core::ptr::write(ppprstg, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID, grfmode: u32, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertySetStorage_Impl::Open(this, core::mem::transmute_copy(&rfmtid), core::mem::transmute_copy(&grfmode)) {
                Ok(ok__) => {
                    core::ptr::write(ppprstg, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertySetStorage_Impl::Delete(this, core::mem::transmute_copy(&rfmtid)).into()
        }
        unsafe extern "system" fn Enum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertySetStorage_Impl::Enum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySetStorage as windows_core::Interface>::IID
    }
}
pub trait IPropertyStorage_Impl: Sized {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const windows_core::PROPVARIANT, propidnamefirst: u32) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyStorage {}
impl IPropertyStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>() -> IPropertyStorage_Vtbl {
        unsafe extern "system" fn ReadMultiple<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar)).into()
        }
        unsafe extern "system" fn WriteMultiple<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, propidnamefirst: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
        }
        unsafe extern "system" fn DeleteMultiple<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn WritePropertyNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::Revert(this).into()
        }
        unsafe extern "system" fn Enum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertyStorage_Impl::Enum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn Stat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertyStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatpsstg)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, Impl, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, Impl, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, Impl, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, Impl, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, Impl, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            SetTimes: SetTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStorage as windows_core::Interface>::IID
    }
}
pub trait IRootStorage_Impl: Sized {
    fn SwitchToFile(&self, pszfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRootStorage {}
impl IRootStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRootStorage_Impl, const OFFSET: isize>() -> IRootStorage_Vtbl {
        unsafe extern "system" fn SwitchToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRootStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRootStorage_Impl::SwitchToFile(this, core::mem::transmute(&pszfile)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SwitchToFile: SwitchToFile::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRootStorage as windows_core::Interface>::IID
    }
}
pub trait IStorage_Impl: Sized {
    fn CreateStream(&self, pwcsname: &windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> windows_core::Result<super::IStream>;
    fn OpenStream(&self, pwcsname: &windows_core::PCWSTR, reserved1: *const core::ffi::c_void, grfmode: super::STGM, reserved2: u32) -> windows_core::Result<super::IStream>;
    fn CreateStorage(&self, pwcsname: &windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> windows_core::Result<IStorage>;
    fn OpenStorage(&self, pwcsname: &windows_core::PCWSTR, pstgpriority: Option<&IStorage>, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32) -> windows_core::Result<IStorage>;
    fn CopyTo(&self, ciidexclude: u32, rgiidexclude: *const windows_core::GUID, snbexclude: *const *const u16, pstgdest: Option<&IStorage>) -> windows_core::Result<()>;
    fn MoveElementTo(&self, pwcsname: &windows_core::PCWSTR, pstgdest: Option<&IStorage>, pwcsnewname: &windows_core::PCWSTR, grfflags: &STGMOVE) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn EnumElements(&self, reserved1: u32, reserved2: *const core::ffi::c_void, reserved3: u32) -> windows_core::Result<IEnumSTATSTG>;
    fn DestroyElement(&self, pwcsname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RenameElement(&self, pwcsoldname: &windows_core::PCWSTR, pwcsnewname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetElementTimes(&self, pwcsname: &windows_core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStorage {}
impl IStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>() -> IStorage_Vtbl {
        unsafe extern "system" fn CreateStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorage_Impl::CreateStream(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2)) {
                Ok(ok__) => {
                    core::ptr::write(ppstm, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, reserved1: *const core::ffi::c_void, grfmode: super::STGM, reserved2: u32, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorage_Impl::OpenStream(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved2)) {
                Ok(ok__) => {
                    core::ptr::write(ppstm, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorage_Impl::CreateStorage(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2)) {
                Ok(ok__) => {
                    core::ptr::write(ppstg, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStorage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, pstgpriority: *mut core::ffi::c_void, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorage_Impl::OpenStorage(this, core::mem::transmute(&pwcsname), windows_core::from_raw_borrowed(&pstgpriority), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&snbexclude), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    core::ptr::write(ppstg, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const windows_core::GUID, snbexclude: *const *const u16, pstgdest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::CopyTo(this, core::mem::transmute_copy(&ciidexclude), core::mem::transmute_copy(&rgiidexclude), core::mem::transmute_copy(&snbexclude), windows_core::from_raw_borrowed(&pstgdest)).into()
        }
        unsafe extern "system" fn MoveElementTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, pstgdest: *mut core::ffi::c_void, pwcsnewname: windows_core::PCWSTR, grfflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::MoveElementTo(this, core::mem::transmute(&pwcsname), windows_core::from_raw_borrowed(&pstgdest), core::mem::transmute(&pwcsnewname), core::mem::transmute(&grfflags)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::Revert(this).into()
        }
        unsafe extern "system" fn EnumElements<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved1: u32, reserved2: *const core::ffi::c_void, reserved3: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorage_Impl::EnumElements(this, core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2), core::mem::transmute_copy(&reserved3)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::DestroyElement(this, core::mem::transmute(&pwcsname)).into()
        }
        unsafe extern "system" fn RenameElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsoldname: windows_core::PCWSTR, pwcsnewname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::RenameElement(this, core::mem::transmute(&pwcsoldname), core::mem::transmute(&pwcsnewname)).into()
        }
        unsafe extern "system" fn SetElementTimes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::SetElementTimes(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn SetStateBits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::SetStateBits(this, core::mem::transmute_copy(&grfstatebits), core::mem::transmute_copy(&grfmask)).into()
        }
        unsafe extern "system" fn Stat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute_copy(&grfstatflag)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            OpenStream: OpenStream::<Identity, Impl, OFFSET>,
            CreateStorage: CreateStorage::<Identity, Impl, OFFSET>,
            OpenStorage: OpenStorage::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            MoveElementTo: MoveElementTo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            EnumElements: EnumElements::<Identity, Impl, OFFSET>,
            DestroyElement: DestroyElement::<Identity, Impl, OFFSET>,
            RenameElement: RenameElement::<Identity, Impl, OFFSET>,
            SetElementTimes: SetElementTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            SetStateBits: SetStateBits::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorage as windows_core::Interface>::IID
    }
}
