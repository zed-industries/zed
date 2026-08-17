#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplication_Impl: Sized + super::Com::IDispatch_Impl {
    fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList>;
    fn Id(&self) -> windows_core::Result<i32>;
    fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShared(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Flags(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIApplication {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIApplication_Vtbl
    where
        Identity: IRDPSRAPIApplication_Impl,
    {
        unsafe extern "system" fn Windows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindowlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplication_Impl::Windows(this) {
                Ok(ok__) => {
                    pwindowlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplication_Impl::Id(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplication_Impl::Shared(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIApplication_Impl::SetShared(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplication_Impl::Name(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplication_Impl::Flags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Windows: Windows::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Shared: Shared::<Identity, OFFSET>,
            SetShared: SetShared::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIApplication as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationFilter_Impl: Sized + super::Com::IDispatch_Impl {
    fn Applications(&self) -> windows_core::Result<IRDPSRAPIApplicationList>;
    fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIApplicationFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIApplicationFilter_Vtbl
    where
        Identity: IRDPSRAPIApplicationFilter_Impl,
    {
        unsafe extern "system" fn Applications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplications: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplicationFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplicationFilter_Impl::Applications(this) {
                Ok(ok__) => {
                    papplications.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Windows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindows: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplicationFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplicationFilter_Impl::Windows(this) {
                Ok(ok__) => {
                    pwindows.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplicationFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplicationFilter_Impl::Enabled(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplicationFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIApplicationFilter_Impl::SetEnabled(this, core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Applications: Applications::<Identity, OFFSET>,
            Windows: Windows::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationFilter as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationList_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIApplication>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIApplicationList {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIApplicationList_Vtbl
    where
        Identity: IRDPSRAPIApplicationList_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplicationList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplicationList_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, papplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIApplicationList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIApplicationList_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                Ok(ok__) => {
                    papplication.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), _NewEnum: _NewEnum::<Identity, OFFSET>, get_Item: get_Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationList as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendee_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<i32>;
    fn RemoteName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ControlLevel(&self) -> windows_core::Result<CTRL_LEVEL>;
    fn SetControlLevel(&self, pnewval: CTRL_LEVEL) -> windows_core::Result<()>;
    fn Invitation(&self) -> windows_core::Result<IRDPSRAPIInvitation>;
    fn TerminateConnection(&self) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn ConnectivityInfo(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIAttendee {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendee_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIAttendee_Vtbl
    where
        Identity: IRDPSRAPIAttendee_Impl,
    {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendee_Impl::Id(this) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendee_Impl::RemoteName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut CTRL_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendee_Impl::ControlLevel(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: CTRL_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIAttendee_Impl::SetControlLevel(this, core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn Invitation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendee_Impl::Invitation(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIAttendee_Impl::TerminateConnection(this).into()
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendee_Impl::Flags(this) {
                Ok(ok__) => {
                    plflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectivityInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendee_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendee_Impl::ConnectivityInfo(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            RemoteName: RemoteName::<Identity, OFFSET>,
            ControlLevel: ControlLevel::<Identity, OFFSET>,
            SetControlLevel: SetControlLevel::<Identity, OFFSET>,
            Invitation: Invitation::<Identity, OFFSET>,
            TerminateConnection: TerminateConnection::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            ConnectivityInfo: ConnectivityInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendee as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeDisconnectInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Attendee(&self) -> windows_core::Result<IRDPSRAPIAttendee>;
    fn Reason(&self) -> windows_core::Result<ATTENDEE_DISCONNECT_REASON>;
    fn Code(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIAttendeeDisconnectInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIAttendeeDisconnectInfo_Vtbl
    where
        Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl,
    {
        unsafe extern "system" fn Attendee<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendeeDisconnectInfo_Impl::Attendee(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendeeDisconnectInfo_Impl::Reason(this) {
                Ok(ok__) => {
                    preason.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendeeDisconnectInfo_Impl::Code(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Attendee: Attendee::<Identity, OFFSET>,
            Reason: Reason::<Identity, OFFSET>,
            Code: Code::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeDisconnectInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, id: i32) -> windows_core::Result<IRDPSRAPIAttendee>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIAttendeeManager {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIAttendeeManager_Vtbl
    where
        Identity: IRDPSRAPIAttendeeManager_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendeeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendeeManager_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAttendeeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAttendeeManager_Impl::get_Item(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), _NewEnum: _NewEnum::<Identity, OFFSET>, get_Item: get_Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRDPSRAPIAudioStream_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<i64>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn GetBuffer(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> windows_core::Result<()>;
    fn FreeBuffer(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRDPSRAPIAudioStream {}
impl IRDPSRAPIAudioStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIAudioStream_Vtbl
    where
        Identity: IRDPSRAPIAudioStream_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAudioStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIAudioStream_Impl::Initialize(this) {
                Ok(ok__) => {
                    pnperiodinhundrednsintervals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAudioStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIAudioStream_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAudioStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIAudioStream_Impl::Stop(this).into()
        }
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAudioStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIAudioStream_Impl::GetBuffer(this, core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&ptimestamp)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIAudioStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIAudioStream_Impl::FreeBuffer(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAudioStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIClipboardUseEvents_Impl: Sized {
    fn OnPasteFromClipboard(&self, clipboardformat: u32, pattendee: Option<&super::Com::IDispatch>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIClipboardUseEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIClipboardUseEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIClipboardUseEvents_Vtbl
    where
        Identity: IRDPSRAPIClipboardUseEvents_Impl,
    {
        unsafe extern "system" fn OnPasteFromClipboard<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipboardformat: u32, pattendee: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIClipboardUseEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIClipboardUseEvents_Impl::OnPasteFromClipboard(this, core::mem::transmute_copy(&clipboardformat), windows_core::from_raw_borrowed(&pattendee)) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPasteFromClipboard: OnPasteFromClipboard::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIClipboardUseEvents as windows_core::Interface>::IID
    }
}
pub trait IRDPSRAPIDebug_Impl: Sized {
    fn SetCLXCmdLine(&self, clxcmdline: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CLXCmdLine(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IRDPSRAPIDebug {}
impl IRDPSRAPIDebug_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIDebug_Vtbl
    where
        Identity: IRDPSRAPIDebug_Impl,
    {
        unsafe extern "system" fn SetCLXCmdLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clxcmdline: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIDebug_Impl::SetCLXCmdLine(this, core::mem::transmute(&clxcmdline)).into()
        }
        unsafe extern "system" fn CLXCmdLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclxcmdline: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIDebug_Impl::CLXCmdLine(this) {
                Ok(ok__) => {
                    pclxcmdline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCLXCmdLine: SetCLXCmdLine::<Identity, OFFSET>,
            CLXCmdLine: CLXCmdLine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIDebug as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIFrameBuffer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Width(&self) -> windows_core::Result<i32>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn Bpp(&self) -> windows_core::Result<i32>;
    fn GetFrameBufferBits(&self, x: i32, y: i32, width: i32, heigth: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIFrameBuffer {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIFrameBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIFrameBuffer_Vtbl
    where
        Identity: IRDPSRAPIFrameBuffer_Impl,
    {
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIFrameBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIFrameBuffer_Impl::Width(this) {
                Ok(ok__) => {
                    plwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plheight: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIFrameBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIFrameBuffer_Impl::Height(this) {
                Ok(ok__) => {
                    plheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bpp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbpp: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIFrameBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIFrameBuffer_Impl::Bpp(this) {
                Ok(ok__) => {
                    plbpp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameBufferBits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIFrameBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIFrameBuffer_Impl::GetFrameBufferBits(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&width), core::mem::transmute_copy(&heigth)) {
                Ok(ok__) => {
                    ppbits.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Width: Width::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            Bpp: Bpp::<Identity, OFFSET>,
            GetFrameBufferBits: GetFrameBufferBits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIFrameBuffer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIInvitation_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectionString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GroupName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Password(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AttendeeLimit(&self) -> windows_core::Result<i32>;
    fn SetAttendeeLimit(&self, newval: i32) -> windows_core::Result<()>;
    fn Revoked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRevoked(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIInvitation {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIInvitation_Vtbl
    where
        Identity: IRDPSRAPIInvitation_Impl,
    {
        unsafe extern "system" fn ConnectionString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitation_Impl::ConnectionString(this) {
                Ok(ok__) => {
                    pbstrval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitation_Impl::GroupName(this) {
                Ok(ok__) => {
                    pbstrval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitation_Impl::Password(this) {
                Ok(ok__) => {
                    pbstrval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttendeeLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitation_Impl::AttendeeLimit(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttendeeLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIInvitation_Impl::SetAttendeeLimit(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Revoked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitation_Impl::Revoked(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevoked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIInvitation_Impl::SetRevoked(this, core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ConnectionString: ConnectionString::<Identity, OFFSET>,
            GroupName: GroupName::<Identity, OFFSET>,
            Password: Password::<Identity, OFFSET>,
            AttendeeLimit: AttendeeLimit::<Identity, OFFSET>,
            SetAttendeeLimit: SetAttendeeLimit::<Identity, OFFSET>,
            Revoked: Revoked::<Identity, OFFSET>,
            SetRevoked: SetRevoked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitation as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIInvitationManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: &windows_core::VARIANT) -> windows_core::Result<IRDPSRAPIInvitation>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn CreateInvitation(&self, bstrauthstring: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, attendeelimit: i32) -> windows_core::Result<IRDPSRAPIInvitation>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIInvitationManager {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitationManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIInvitationManager_Vtbl
    where
        Identity: IRDPSRAPIInvitationManager_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitationManager_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: core::mem::MaybeUninit<windows_core::VARIANT>, ppinvitation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitationManager_Impl::get_Item(this, core::mem::transmute(&item)) {
                Ok(ok__) => {
                    ppinvitation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitationManager_Impl::Count(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInvitation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, attendeelimit: i32, ppinvitation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIInvitationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIInvitationManager_Impl::CreateInvitation(this, core::mem::transmute(&bstrauthstring), core::mem::transmute(&bstrgroupname), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&attendeelimit)) {
                Ok(ok__) => {
                    ppinvitation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            CreateInvitation: CreateInvitation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitationManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRDPSRAPIPerfCounterLogger_Impl: Sized {
    fn LogValue(&self, lvalue: i64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRDPSRAPIPerfCounterLogger {}
impl IRDPSRAPIPerfCounterLogger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIPerfCounterLogger_Vtbl
    where
        Identity: IRDPSRAPIPerfCounterLogger_Impl,
    {
        unsafe extern "system" fn LogValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvalue: i64) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIPerfCounterLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIPerfCounterLogger_Impl::LogValue(this, core::mem::transmute_copy(&lvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LogValue: LogValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLogger as windows_core::Interface>::IID
    }
}
pub trait IRDPSRAPIPerfCounterLoggingManager_Impl: Sized {
    fn CreateLogger(&self, bstrcountername: &windows_core::BSTR) -> windows_core::Result<IRDPSRAPIPerfCounterLogger>;
}
impl windows_core::RuntimeName for IRDPSRAPIPerfCounterLoggingManager {}
impl IRDPSRAPIPerfCounterLoggingManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIPerfCounterLoggingManager_Vtbl
    where
        Identity: IRDPSRAPIPerfCounterLoggingManager_Impl,
    {
        unsafe extern "system" fn CreateLogger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcountername: core::mem::MaybeUninit<windows_core::BSTR>, pplogger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIPerfCounterLoggingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIPerfCounterLoggingManager_Impl::CreateLogger(this, core::mem::transmute(&bstrcountername)) {
                Ok(ok__) => {
                    pplogger.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateLogger: CreateLogger::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLoggingManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISessionProperties_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Property(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn put_Property(&self, propertyname: &windows_core::BSTR, newval: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPISessionProperties {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISessionProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPISessionProperties_Vtbl
    where
        Identity: IRDPSRAPISessionProperties_Impl,
    {
        unsafe extern "system" fn get_Property<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISessionProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISessionProperties_Impl::get_Property(this, core::mem::transmute(&propertyname)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Property<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, newval: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISessionProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISessionProperties_Impl::put_Property(this, core::mem::transmute(&propertyname), core::mem::transmute(&newval)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Property: get_Property::<Identity, OFFSET>,
            put_Property: put_Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPISessionProperties as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISharingSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn SetColorDepth(&self, colordepth: i32) -> windows_core::Result<()>;
    fn ColorDepth(&self) -> windows_core::Result<i32>;
    fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties>;
    fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn ConnectToClient(&self, bstrconnectionstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::Result<()>;
    fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPISharingSession {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPISharingSession_Vtbl
    where
        Identity: IRDPSRAPISharingSession_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::Open(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::Close(this).into()
        }
        unsafe extern "system" fn SetColorDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colordepth: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::SetColorDepth(this, core::mem::transmute_copy(&colordepth)).into()
        }
        unsafe extern "system" fn ColorDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolordepth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISharingSession_Impl::ColorDepth(this) {
                Ok(ok__) => {
                    pcolordepth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISharingSession_Impl::Properties(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attendees<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISharingSession_Impl::Attendees(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISharingSession_Impl::Invitations(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISharingSession_Impl::ApplicationFilter(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISharingSession_Impl::VirtualChannelManager(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::Resume(this).into()
        }
        unsafe extern "system" fn ConnectToClient<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectionstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::ConnectToClient(this, core::mem::transmute(&bstrconnectionstring)).into()
        }
        unsafe extern "system" fn SetDesktopSharedRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::SetDesktopSharedRect(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&right), core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn GetDesktopSharedRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession_Impl::GetDesktopSharedRect(this, core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetColorDepth: SetColorDepth::<Identity, OFFSET>,
            ColorDepth: ColorDepth::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Attendees: Attendees::<Identity, OFFSET>,
            Invitations: Invitations::<Identity, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            ConnectToClient: ConnectToClient::<Identity, OFFSET>,
            SetDesktopSharedRect: SetDesktopSharedRect::<Identity, OFFSET>,
            GetDesktopSharedRect: GetDesktopSharedRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISharingSession2_Impl: Sized + IRDPSRAPISharingSession_Impl {
    fn ConnectUsingTransportStream(&self, pstream: Option<&IRDPSRAPITransportStream>, bstrgroup: &windows_core::BSTR, bstrauthenticatedattendeename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FrameBuffer(&self) -> windows_core::Result<IRDPSRAPIFrameBuffer>;
    fn SendControlLevelChangeResponse(&self, pattendee: Option<&IRDPSRAPIAttendee>, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPISharingSession2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPISharingSession2_Vtbl
    where
        Identity: IRDPSRAPISharingSession2_Impl,
    {
        unsafe extern "system" fn ConnectUsingTransportStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, bstrgroup: core::mem::MaybeUninit<windows_core::BSTR>, bstrauthenticatedattendeename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession2_Impl::ConnectUsingTransportStream(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute(&bstrgroup), core::mem::transmute(&bstrauthenticatedattendeename)).into()
        }
        unsafe extern "system" fn FrameBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPISharingSession2_Impl::FrameBuffer(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlLevelChangeResponse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattendee: *mut core::ffi::c_void, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPISharingSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPISharingSession2_Impl::SendControlLevelChangeResponse(this, windows_core::from_raw_borrowed(&pattendee), core::mem::transmute_copy(&requestedlevel), core::mem::transmute_copy(&reasoncode)).into()
        }
        Self {
            base__: IRDPSRAPISharingSession_Vtbl::new::<Identity, OFFSET>(),
            ConnectUsingTransportStream: ConnectUsingTransportStream::<Identity, OFFSET>,
            FrameBuffer: FrameBuffer::<Identity, OFFSET>,
            SendControlLevelChangeResponse: SendControlLevelChangeResponse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRDPSRAPISharingSession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPITcpConnectionInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Protocol(&self) -> windows_core::Result<i32>;
    fn LocalPort(&self) -> windows_core::Result<i32>;
    fn LocalIP(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PeerPort(&self) -> windows_core::Result<i32>;
    fn PeerIP(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPITcpConnectionInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPITcpConnectionInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPITcpConnectionInfo_Vtbl
    where
        Identity: IRDPSRAPITcpConnectionInfo_Impl,
    {
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprotocol: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITcpConnectionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITcpConnectionInfo_Impl::Protocol(this) {
                Ok(ok__) => {
                    plprotocol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plport: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITcpConnectionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITcpConnectionInfo_Impl::LocalPort(this) {
                Ok(ok__) => {
                    plport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalIP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsrlocalip: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITcpConnectionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITcpConnectionInfo_Impl::LocalIP(this) {
                Ok(ok__) => {
                    pbsrlocalip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plport: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITcpConnectionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITcpConnectionInfo_Impl::PeerPort(this) {
                Ok(ok__) => {
                    plport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerIP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrip: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITcpConnectionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITcpConnectionInfo_Impl::PeerIP(this) {
                Ok(ok__) => {
                    pbstrip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Protocol: Protocol::<Identity, OFFSET>,
            LocalPort: LocalPort::<Identity, OFFSET>,
            LocalIP: LocalIP::<Identity, OFFSET>,
            PeerPort: PeerPort::<Identity, OFFSET>,
            PeerIP: PeerIP::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITcpConnectionInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStream_Impl: Sized {
    fn AllocBuffer(&self, maxpayload: i32) -> windows_core::Result<IRDPSRAPITransportStreamBuffer>;
    fn FreeBuffer(&self, pbuffer: Option<&IRDPSRAPITransportStreamBuffer>) -> windows_core::Result<()>;
    fn WriteBuffer(&self, pbuffer: Option<&IRDPSRAPITransportStreamBuffer>) -> windows_core::Result<()>;
    fn ReadBuffer(&self, pbuffer: Option<&IRDPSRAPITransportStreamBuffer>) -> windows_core::Result<()>;
    fn Open(&self, pcallbacks: Option<&IRDPSRAPITransportStreamEvents>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRDPSRAPITransportStream {}
impl IRDPSRAPITransportStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPITransportStream_Vtbl
    where
        Identity: IRDPSRAPITransportStream_Impl,
    {
        unsafe extern "system" fn AllocBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxpayload: i32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITransportStream_Impl::AllocBuffer(this, core::mem::transmute_copy(&maxpayload)) {
                Ok(ok__) => {
                    ppbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStream_Impl::FreeBuffer(this, windows_core::from_raw_borrowed(&pbuffer)).into()
        }
        unsafe extern "system" fn WriteBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStream_Impl::WriteBuffer(this, windows_core::from_raw_borrowed(&pbuffer)).into()
        }
        unsafe extern "system" fn ReadBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStream_Impl::ReadBuffer(this, windows_core::from_raw_borrowed(&pbuffer)).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbacks: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStream_Impl::Open(this, windows_core::from_raw_borrowed(&pcallbacks)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStream_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocBuffer: AllocBuffer::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStream as windows_core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStreamBuffer_Impl: Sized {
    fn Storage(&self) -> windows_core::Result<*mut u8>;
    fn StorageSize(&self) -> windows_core::Result<i32>;
    fn PayloadSize(&self) -> windows_core::Result<i32>;
    fn SetPayloadSize(&self, lval: i32) -> windows_core::Result<()>;
    fn PayloadOffset(&self) -> windows_core::Result<i32>;
    fn SetPayloadOffset(&self, lretval: i32) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn SetFlags(&self, lflags: i32) -> windows_core::Result<()>;
    fn Context(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetContext(&self, pcontext: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRDPSRAPITransportStreamBuffer {}
impl IRDPSRAPITransportStreamBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPITransportStreamBuffer_Vtbl
    where
        Identity: IRDPSRAPITransportStreamBuffer_Impl,
    {
        unsafe extern "system" fn Storage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbstorage: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITransportStreamBuffer_Impl::Storage(this) {
                Ok(ok__) => {
                    ppbstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxstore: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITransportStreamBuffer_Impl::StorageSize(this) {
                Ok(ok__) => {
                    plmaxstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayloadSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITransportStreamBuffer_Impl::PayloadSize(this) {
                Ok(ok__) => {
                    plretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStreamBuffer_Impl::SetPayloadSize(this, core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PayloadOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITransportStreamBuffer_Impl::PayloadOffset(this) {
                Ok(ok__) => {
                    plretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretval: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStreamBuffer_Impl::SetPayloadOffset(this, core::mem::transmute_copy(&lretval)).into()
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITransportStreamBuffer_Impl::Flags(this) {
                Ok(ok__) => {
                    plflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStreamBuffer_Impl::SetFlags(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Context<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPITransportStreamBuffer_Impl::Context(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPITransportStreamBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStreamBuffer_Impl::SetContext(this, windows_core::from_raw_borrowed(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Storage: Storage::<Identity, OFFSET>,
            StorageSize: StorageSize::<Identity, OFFSET>,
            PayloadSize: PayloadSize::<Identity, OFFSET>,
            SetPayloadSize: SetPayloadSize::<Identity, OFFSET>,
            PayloadOffset: PayloadOffset::<Identity, OFFSET>,
            SetPayloadOffset: SetPayloadOffset::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamBuffer as windows_core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStreamEvents_Impl: Sized {
    fn OnWriteCompleted(&self, pbuffer: Option<&IRDPSRAPITransportStreamBuffer>);
    fn OnReadCompleted(&self, pbuffer: Option<&IRDPSRAPITransportStreamBuffer>);
    fn OnStreamClosed(&self, hrreason: windows_core::HRESULT);
}
impl windows_core::RuntimeName for IRDPSRAPITransportStreamEvents {}
impl IRDPSRAPITransportStreamEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPITransportStreamEvents_Vtbl
    where
        Identity: IRDPSRAPITransportStreamEvents_Impl,
    {
        unsafe extern "system" fn OnWriteCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void)
        where
            Identity: IRDPSRAPITransportStreamEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStreamEvents_Impl::OnWriteCompleted(this, windows_core::from_raw_borrowed(&pbuffer))
        }
        unsafe extern "system" fn OnReadCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void)
        where
            Identity: IRDPSRAPITransportStreamEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStreamEvents_Impl::OnReadCompleted(this, windows_core::from_raw_borrowed(&pbuffer))
        }
        unsafe extern "system" fn OnStreamClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrreason: windows_core::HRESULT)
        where
            Identity: IRDPSRAPITransportStreamEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPITransportStreamEvents_Impl::OnStreamClosed(this, core::mem::transmute_copy(&hrreason))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnWriteCompleted: OnWriteCompleted::<Identity, OFFSET>,
            OnReadCompleted: OnReadCompleted::<Identity, OFFSET>,
            OnStreamClosed: OnStreamClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIViewer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Connect(&self, bstrconnectionstring: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager>;
    fn SetSmartSizing(&self, vbsmartsizing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SmartSizing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RequestControl(&self, ctrllevel: CTRL_LEVEL) -> windows_core::Result<()>;
    fn SetDisconnectedText(&self, bstrdisconnectedtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisconnectedText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestColorDepthChange(&self, bpp: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties>;
    fn StartReverseConnectListener(&self, bstrconnectionstring: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIViewer {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIViewer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIViewer_Vtbl
    where
        Identity: IRDPSRAPIViewer_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectionstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIViewer_Impl::Connect(this, core::mem::transmute(&bstrconnectionstring), core::mem::transmute(&bstrname), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIViewer_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn Attendees<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::Attendees(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::Invitations(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::ApplicationFilter(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::VirtualChannelManager(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartSizing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbsmartsizing: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIViewer_Impl::SetSmartSizing(this, core::mem::transmute_copy(&vbsmartsizing)).into()
        }
        unsafe extern "system" fn SmartSizing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbsmartsizing: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::SmartSizing(this) {
                Ok(ok__) => {
                    pvbsmartsizing.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIViewer_Impl::RequestControl(this, core::mem::transmute_copy(&ctrllevel)).into()
        }
        unsafe extern "system" fn SetDisconnectedText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdisconnectedtext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIViewer_Impl::SetDisconnectedText(this, core::mem::transmute(&bstrdisconnectedtext)).into()
        }
        unsafe extern "system" fn DisconnectedText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisconnectedtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::DisconnectedText(this) {
                Ok(ok__) => {
                    pbstrdisconnectedtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestColorDepthChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpp: i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIViewer_Impl::RequestColorDepthChange(this, core::mem::transmute_copy(&bpp)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::Properties(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartReverseConnectListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectionstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, pbstrreverseconnectstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIViewer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIViewer_Impl::StartReverseConnectListener(this, core::mem::transmute(&bstrconnectionstring), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)) {
                Ok(ok__) => {
                    pbstrreverseconnectstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Attendees: Attendees::<Identity, OFFSET>,
            Invitations: Invitations::<Identity, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, OFFSET>,
            SetSmartSizing: SetSmartSizing::<Identity, OFFSET>,
            SmartSizing: SmartSizing::<Identity, OFFSET>,
            RequestControl: RequestControl::<Identity, OFFSET>,
            SetDisconnectedText: SetDisconnectedText::<Identity, OFFSET>,
            DisconnectedText: DisconnectedText::<Identity, OFFSET>,
            RequestColorDepthChange: RequestColorDepthChange::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            StartReverseConnectListener: StartReverseConnectListener::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIViewer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIVirtualChannel_Impl: Sized + super::Com::IDispatch_Impl {
    fn SendData(&self, bstrdata: &windows_core::BSTR, lattendeeid: i32, channelsendflags: u32) -> windows_core::Result<()>;
    fn SetAccess(&self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn Priority(&self) -> windows_core::Result<CHANNEL_PRIORITY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIVirtualChannel {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIVirtualChannel_Vtbl
    where
        Identity: IRDPSRAPIVirtualChannel_Impl,
    {
        unsafe extern "system" fn SendData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>, lattendeeid: i32, channelsendflags: u32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIVirtualChannel_Impl::SendData(this, core::mem::transmute(&bstrdata), core::mem::transmute_copy(&lattendeeid), core::mem::transmute_copy(&channelsendflags)).into()
        }
        unsafe extern "system" fn SetAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIVirtualChannel_Impl::SetAccess(this, core::mem::transmute_copy(&lattendeeid), core::mem::transmute_copy(&accesstype)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIVirtualChannel_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIVirtualChannel_Impl::Flags(this) {
                Ok(ok__) => {
                    plflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIVirtualChannel_Impl::Priority(this) {
                Ok(ok__) => {
                    ppriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SendData: SendData::<Identity, OFFSET>,
            SetAccess: SetAccess::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannel as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIVirtualChannelManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: &windows_core::VARIANT) -> windows_core::Result<IRDPSRAPIVirtualChannel>;
    fn CreateVirtualChannel(&self, bstrchannelname: &windows_core::BSTR, priority: CHANNEL_PRIORITY, channelflags: u32) -> windows_core::Result<IRDPSRAPIVirtualChannel>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIVirtualChannelManager {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannelManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIVirtualChannelManager_Vtbl
    where
        Identity: IRDPSRAPIVirtualChannelManager_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannelManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIVirtualChannelManager_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: core::mem::MaybeUninit<windows_core::VARIANT>, pchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannelManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIVirtualChannelManager_Impl::get_Item(this, core::mem::transmute(&item)) {
                Ok(ok__) => {
                    pchannel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrchannelname: core::mem::MaybeUninit<windows_core::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIVirtualChannelManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIVirtualChannelManager_Impl::CreateVirtualChannel(this, core::mem::transmute(&bstrchannelname), core::mem::transmute_copy(&priority), core::mem::transmute_copy(&channelflags)) {
                Ok(ok__) => {
                    ppchannel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannelManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIWindow_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<i32>;
    fn Application(&self) -> windows_core::Result<IRDPSRAPIApplication>;
    fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShared(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Show(&self) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIWindow {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIWindow_Vtbl
    where
        Identity: IRDPSRAPIWindow_Impl,
    {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIWindow_Impl::Id(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIWindow_Impl::Application(this) {
                Ok(ok__) => {
                    papplication.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIWindow_Impl::Shared(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIWindow_Impl::SetShared(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIWindow_Impl::Name(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPSRAPIWindow_Impl::Show(this).into()
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIWindow_Impl::Flags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Application: Application::<Identity, OFFSET>,
            Shared: Shared::<Identity, OFFSET>,
            SetShared: SetShared::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIWindow as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIWindowList_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIWindow>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIWindowList {}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindowList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPSRAPIWindowList_Vtbl
    where
        Identity: IRDPSRAPIWindowList_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindowList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIWindowList_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, pwindow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPSRAPIWindowList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRDPSRAPIWindowList_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                Ok(ok__) => {
                    pwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), _NewEnum: _NewEnum::<Identity, OFFSET>, get_Item: get_Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIWindowList as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRDPViewerInputSink_Impl: Sized {
    fn SendMouseButtonEvent(&self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: super::super::Foundation::VARIANT_BOOL, xpos: u32, ypos: u32) -> windows_core::Result<()>;
    fn SendMouseMoveEvent(&self, xpos: u32, ypos: u32) -> windows_core::Result<()>;
    fn SendMouseWheelEvent(&self, wheelrotation: u16) -> windows_core::Result<()>;
    fn SendKeyboardEvent(&self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: super::super::Foundation::VARIANT_BOOL, vbrepeat: super::super::Foundation::VARIANT_BOOL, vbextended: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SendSyncEvent(&self, syncflags: u32) -> windows_core::Result<()>;
    fn BeginTouchFrame(&self) -> windows_core::Result<()>;
    fn AddTouchInput(&self, contactid: u32, event: u32, x: i32, y: i32) -> windows_core::Result<()>;
    fn EndTouchFrame(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRDPViewerInputSink {}
impl IRDPViewerInputSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRDPViewerInputSink_Vtbl
    where
        Identity: IRDPViewerInputSink_Impl,
    {
        unsafe extern "system" fn SendMouseButtonEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: super::super::Foundation::VARIANT_BOOL, xpos: u32, ypos: u32) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::SendMouseButtonEvent(this, core::mem::transmute_copy(&buttontype), core::mem::transmute_copy(&vbbuttondown), core::mem::transmute_copy(&xpos), core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseMoveEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpos: u32, ypos: u32) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::SendMouseMoveEvent(this, core::mem::transmute_copy(&xpos), core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseWheelEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wheelrotation: u16) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::SendMouseWheelEvent(this, core::mem::transmute_copy(&wheelrotation)).into()
        }
        unsafe extern "system" fn SendKeyboardEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: super::super::Foundation::VARIANT_BOOL, vbrepeat: super::super::Foundation::VARIANT_BOOL, vbextended: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::SendKeyboardEvent(this, core::mem::transmute_copy(&codetype), core::mem::transmute_copy(&keycode), core::mem::transmute_copy(&vbkeyup), core::mem::transmute_copy(&vbrepeat), core::mem::transmute_copy(&vbextended)).into()
        }
        unsafe extern "system" fn SendSyncEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncflags: u32) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::SendSyncEvent(this, core::mem::transmute_copy(&syncflags)).into()
        }
        unsafe extern "system" fn BeginTouchFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::BeginTouchFrame(this).into()
        }
        unsafe extern "system" fn AddTouchInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::AddTouchInput(this, core::mem::transmute_copy(&contactid), core::mem::transmute_copy(&event), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn EndTouchFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRDPViewerInputSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRDPViewerInputSink_Impl::EndTouchFrame(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendMouseButtonEvent: SendMouseButtonEvent::<Identity, OFFSET>,
            SendMouseMoveEvent: SendMouseMoveEvent::<Identity, OFFSET>,
            SendMouseWheelEvent: SendMouseWheelEvent::<Identity, OFFSET>,
            SendKeyboardEvent: SendKeyboardEvent::<Identity, OFFSET>,
            SendSyncEvent: SendSyncEvent::<Identity, OFFSET>,
            BeginTouchFrame: BeginTouchFrame::<Identity, OFFSET>,
            AddTouchInput: AddTouchInput::<Identity, OFFSET>,
            EndTouchFrame: EndTouchFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPViewerInputSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IRDPSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IRDPSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IRDPSessionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IRDPSessionEvents_Vtbl
    where
        Identity: _IRDPSessionEvents_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IRDPSessionEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
