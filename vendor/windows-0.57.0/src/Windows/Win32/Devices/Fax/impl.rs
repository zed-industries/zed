#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccount_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccountName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Folders(&self) -> windows_core::Result<IFaxAccountFolders>;
    fn ListenToAccountEvents(&self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::Result<()>;
    fn RegisteredEvents(&self) -> windows_core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccount {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccount_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>() -> IFaxAccount_Vtbl {
        unsafe extern "system" fn AccountName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstraccountname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccount_Impl::AccountName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstraccountname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfolders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccount_Impl::Folders(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfolders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListenToAccountEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccount_Impl::ListenToAccountEvents(this, core::mem::transmute_copy(&eventtypes)).into()
        }
        unsafe extern "system" fn RegisteredEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccount_Impl::RegisteredEvents(this) {
                Ok(ok__) => {
                    core::ptr::write(pregisteredevents, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccountName: AccountName::<Identity, Impl, OFFSET>,
            Folders: Folders::<Identity, Impl, OFFSET>,
            ListenToAccountEvents: ListenToAccountEvents::<Identity, Impl, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccount as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountFolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&self) -> windows_core::Result<IFaxAccountOutgoingQueue>;
    fn IncomingQueue(&self) -> windows_core::Result<IFaxAccountIncomingQueue>;
    fn IncomingArchive(&self) -> windows_core::Result<IFaxAccountIncomingArchive>;
    fn OutgoingArchive(&self) -> windows_core::Result<IFaxAccountOutgoingArchive>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccountFolders {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountFolders_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>() -> IFaxAccountFolders_Vtbl {
        unsafe extern "system" fn OutgoingQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountFolders_Impl::OutgoingQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingqueue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountFolders_Impl::IncomingQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingqueue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountFolders_Impl::IncomingArchive(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingarchive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountFolders_Impl::OutgoingArchive(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingarchive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Identity, Impl, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, Impl, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, Impl, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountFolders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountIncomingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingMessage>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccountIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingArchive_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>() -> IFaxAccountIncomingArchive_Vtbl {
        unsafe extern "system" fn SizeLow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountIncomingArchive_Impl::SizeLow(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizelow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountIncomingArchive_Impl::SizeHigh(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizehigh, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountIncomingArchive_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn GetMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountIncomingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingmessageiterator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxincomingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountIncomingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountIncomingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountIncomingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&self) -> windows_core::Result<IFaxIncomingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingJob>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccountIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>() -> IFaxAccountIncomingQueue_Vtbl {
        unsafe extern "system" fn GetJobs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountIncomingQueue_Impl::GetJobs(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingjobs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxincomingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountIncomingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountIncomingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountNotify_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&self, pfaxaccount: Option<&IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobRemoved(&self, pfaxaccount: Option<&IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobChanged(&self, pfaxaccount: Option<&IFaxAccount>, bstrjobid: &windows_core::BSTR, pjobstatus: Option<&IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnOutgoingJobAdded(&self, pfaxaccount: Option<&IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobRemoved(&self, pfaxaccount: Option<&IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobChanged(&self, pfaxaccount: Option<&IFaxAccount>, bstrjobid: &windows_core::BSTR, pjobstatus: Option<&IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnIncomingMessageAdded(&self, pfaxaccount: Option<&IFaxAccount>, bstrmessageid: &windows_core::BSTR, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnIncomingMessageRemoved(&self, pfaxaccount: Option<&IFaxAccount>, bstrmessageid: &windows_core::BSTR, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnOutgoingMessageAdded(&self, pfaxaccount: Option<&IFaxAccount>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingMessageRemoved(&self, pfaxaccount: Option<&IFaxAccount>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnServerShutDown(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccountNotify {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>() -> IFaxAccountNotify_Vtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnIncomingJobAdded(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnIncomingJobRemoved(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnIncomingJobChanged(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrjobid), windows_core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnOutgoingJobAdded(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnOutgoingJobRemoved(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnOutgoingJobChanged(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrjobid), windows_core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnIncomingMessageAdded(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrmessageid), core::mem::transmute_copy(&faddedtoreceivefolder)).into()
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnIncomingMessageRemoved(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrmessageid), core::mem::transmute_copy(&fremovedfromreceivefolder)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnOutgoingMessageAdded(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnOutgoingMessageRemoved(this, windows_core::from_raw_borrowed(&pfaxaccount), core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnServerShutDown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountNotify_Impl::OnServerShutDown(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, Impl, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, Impl, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, Impl, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, Impl, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, Impl, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, Impl, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, Impl, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, Impl, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountNotify as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountOutgoingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingMessage>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccountOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingArchive_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>() -> IFaxAccountOutgoingArchive_Vtbl {
        unsafe extern "system" fn SizeLow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountOutgoingArchive_Impl::SizeLow(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizelow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountOutgoingArchive_Impl::SizeHigh(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizehigh, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountOutgoingArchive_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn GetMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountOutgoingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingmessageiterator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxoutgoingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountOutgoingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountOutgoingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&self) -> windows_core::Result<IFaxOutgoingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingJob>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccountOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>() -> IFaxAccountOutgoingQueue_Vtbl {
        unsafe extern "system" fn GetJobs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountOutgoingQueue_Impl::GetJobs(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingjobs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxoutgoingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountOutgoingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountSet_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetAccounts(&self) -> windows_core::Result<IFaxAccounts>;
    fn GetAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<IFaxAccount>;
    fn AddAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<IFaxAccount>;
    fn RemoveAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccountSet {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>() -> IFaxAccountSet_Vtbl {
        unsafe extern "system" fn GetAccounts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxaccounts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountSet_Impl::GetAccounts(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxaccounts, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccountname: core::mem::MaybeUninit<windows_core::BSTR>, pfaxaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountSet_Impl::GetAccount(this, core::mem::transmute(&bstraccountname)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxaccount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccountname: core::mem::MaybeUninit<windows_core::BSTR>, pfaxaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccountSet_Impl::AddAccount(this, core::mem::transmute(&bstraccountname)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxaccount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccountname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxAccountSet_Impl::RemoveAccount(this, core::mem::transmute(&bstraccountname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAccounts: GetAccounts::<Identity, Impl, OFFSET>,
            GetAccount: GetAccount::<Identity, Impl, OFFSET>,
            AddAccount: AddAccount::<Identity, Impl, OFFSET>,
            RemoveAccount: RemoveAccount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountSet as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccounts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxAccount>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxAccounts {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccounts_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>() -> IFaxAccounts_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccounts_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccounts_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxaccount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxAccounts_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccounts as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxActivity_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IncomingMessages(&self) -> windows_core::Result<i32>;
    fn RoutingMessages(&self) -> windows_core::Result<i32>;
    fn OutgoingMessages(&self) -> windows_core::Result<i32>;
    fn QueuedMessages(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxActivity {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>() -> IFaxActivity_Vtbl {
        unsafe extern "system" fn IncomingMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plincomingmessages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxActivity_Impl::IncomingMessages(this) {
                Ok(ok__) => {
                    core::ptr::write(plincomingmessages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plroutingmessages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxActivity_Impl::RoutingMessages(this) {
                Ok(ok__) => {
                    core::ptr::write(plroutingmessages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploutgoingmessages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxActivity_Impl::OutgoingMessages(this) {
                Ok(ok__) => {
                    core::ptr::write(ploutgoingmessages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueuedMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plqueuedmessages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxActivity_Impl::QueuedMessages(this) {
                Ok(ok__) => {
                    core::ptr::write(plqueuedmessages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxActivity_Impl::Refresh(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IncomingMessages: IncomingMessages::<Identity, Impl, OFFSET>,
            RoutingMessages: RoutingMessages::<Identity, Impl, OFFSET>,
            OutgoingMessages: OutgoingMessages::<Identity, Impl, OFFSET>,
            QueuedMessages: QueuedMessages::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxActivity as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxActivityLogging_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LogIncoming(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogIncoming(&self, blogincoming: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogOutgoing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogOutgoing(&self, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DatabasePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDatabasePath(&self, bstrdatabasepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxActivityLogging {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivityLogging_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>() -> IFaxActivityLogging_Vtbl {
        unsafe extern "system" fn LogIncoming<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblogincoming: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxActivityLogging_Impl::LogIncoming(this) {
                Ok(ok__) => {
                    core::ptr::write(pblogincoming, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogIncoming<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blogincoming: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxActivityLogging_Impl::SetLogIncoming(this, core::mem::transmute_copy(&blogincoming)).into()
        }
        unsafe extern "system" fn LogOutgoing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblogoutgoing: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxActivityLogging_Impl::LogOutgoing(this) {
                Ok(ok__) => {
                    core::ptr::write(pblogoutgoing, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogOutgoing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxActivityLogging_Impl::SetLogOutgoing(this, core::mem::transmute_copy(&blogoutgoing)).into()
        }
        unsafe extern "system" fn DatabasePath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdatabasepath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxActivityLogging_Impl::DatabasePath(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdatabasepath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatabasePath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdatabasepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxActivityLogging_Impl::SetDatabasePath(this, core::mem::transmute(&bstrdatabasepath)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxActivityLogging_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxActivityLogging_Impl::Save(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LogIncoming: LogIncoming::<Identity, Impl, OFFSET>,
            SetLogIncoming: SetLogIncoming::<Identity, Impl, OFFSET>,
            LogOutgoing: LogOutgoing::<Identity, Impl, OFFSET>,
            SetLogOutgoing: SetLogOutgoing::<Identity, Impl, OFFSET>,
            DatabasePath: DatabasePath::<Identity, Impl, OFFSET>,
            SetDatabasePath: SetDatabasePath::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxActivityLogging as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxConfiguration_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ArchiveLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArchiveLocation(&self, bstrarchivelocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HighQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()>;
    fn LowQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()>;
    fn ArchiveAgeLimit(&self) -> windows_core::Result<i32>;
    fn SetArchiveAgeLimit(&self, larchiveagelimit: i32) -> windows_core::Result<()>;
    fn ArchiveSizeLow(&self) -> windows_core::Result<i32>;
    fn ArchiveSizeHigh(&self) -> windows_core::Result<i32>;
    fn OutgoingQueueBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueueBlocked(&self, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OutgoingQueuePaused(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueuePaused(&self, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowPersonalCoverPages(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseDeviceTSID(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn SetRetries(&self, lretries: i32) -> windows_core::Result<()>;
    fn RetryDelay(&self) -> windows_core::Result<i32>;
    fn SetRetryDelay(&self, lretrydelay: i32) -> windows_core::Result<()>;
    fn DiscountRateStart(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateStart(&self, datediscountratestart: f64) -> windows_core::Result<()>;
    fn DiscountRateEnd(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> windows_core::Result<()>;
    fn OutgoingQueueAgeLimit(&self) -> windows_core::Result<i32>;
    fn SetOutgoingQueueAgeLimit(&self, loutgoingqueueagelimit: i32) -> windows_core::Result<()>;
    fn Branding(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IncomingQueueBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingQueueBlocked(&self, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AutoCreateAccountOnConnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoCreateAccountOnConnect(&self, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IncomingFaxesArePublic(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingFaxesArePublic(&self, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxConfiguration {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>() -> IFaxConfiguration_Vtbl {
        unsafe extern "system" fn UseArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::UseArchive(this) {
                Ok(ok__) => {
                    core::ptr::write(pbusearchive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetUseArchive(this, core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrarchivelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::ArchiveLocation(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrarchivelocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrarchivelocation: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetArchiveLocation(this, core::mem::transmute(&bstrarchivelocation)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::SizeQuotaWarning(this) {
                Ok(ok__) => {
                    core::ptr::write(pbsizequotawarning, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetSizeQuotaWarning(this, core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhighquotawatermark: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::HighQuotaWaterMark(this) {
                Ok(ok__) => {
                    core::ptr::write(plhighquotawatermark, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhighquotawatermark: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetHighQuotaWaterMark(this, core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllowquotawatermark: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::LowQuotaWaterMark(this) {
                Ok(ok__) => {
                    core::ptr::write(pllowquotawatermark, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowquotawatermark: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetLowQuotaWaterMark(this, core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn ArchiveAgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarchiveagelimit: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::ArchiveAgeLimit(this) {
                Ok(ok__) => {
                    core::ptr::write(plarchiveagelimit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveAgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, larchiveagelimit: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetArchiveAgeLimit(this, core::mem::transmute_copy(&larchiveagelimit)).into()
        }
        unsafe extern "system" fn ArchiveSizeLow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::ArchiveSizeLow(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizelow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveSizeHigh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::ArchiveSizeHigh(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizehigh, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingQueueBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboutgoingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::OutgoingQueueBlocked(this) {
                Ok(ok__) => {
                    core::ptr::write(pboutgoingblocked, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetOutgoingQueueBlocked(this, core::mem::transmute_copy(&boutgoingblocked)).into()
        }
        unsafe extern "system" fn OutgoingQueuePaused<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboutgoingpaused: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::OutgoingQueuePaused(this) {
                Ok(ok__) => {
                    core::ptr::write(pboutgoingpaused, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueuePaused<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetOutgoingQueuePaused(this, core::mem::transmute_copy(&boutgoingpaused)).into()
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::AllowPersonalCoverPages(this) {
                Ok(ok__) => {
                    core::ptr::write(pballowpersonalcoverpages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetAllowPersonalCoverPages(this, core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::UseDeviceTSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbusedevicetsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetUseDeviceTSID(this, core::mem::transmute_copy(&busedevicetsid)).into()
        }
        unsafe extern "system" fn Retries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::Retries(this) {
                Ok(ok__) => {
                    core::ptr::write(plretries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretries: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetRetries(this, core::mem::transmute_copy(&lretries)).into()
        }
        unsafe extern "system" fn RetryDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretrydelay: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::RetryDelay(this) {
                Ok(ok__) => {
                    core::ptr::write(plretrydelay, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretrydelay: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetRetryDelay(this, core::mem::transmute_copy(&lretrydelay)).into()
        }
        unsafe extern "system" fn DiscountRateStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountratestart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::DiscountRateStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatediscountratestart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountratestart: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetDiscountRateStart(this, core::mem::transmute_copy(&datediscountratestart)).into()
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountrateend: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::DiscountRateEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatediscountrateend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountrateend: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetDiscountRateEnd(this, core::mem::transmute_copy(&datediscountrateend)).into()
        }
        unsafe extern "system" fn OutgoingQueueAgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::OutgoingQueueAgeLimit(this) {
                Ok(ok__) => {
                    core::ptr::write(ploutgoingqueueagelimit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueAgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loutgoingqueueagelimit: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetOutgoingQueueAgeLimit(this, core::mem::transmute_copy(&loutgoingqueueagelimit)).into()
        }
        unsafe extern "system" fn Branding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::Branding(this) {
                Ok(ok__) => {
                    core::ptr::write(pbbranding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetBranding(this, core::mem::transmute_copy(&bbranding)).into()
        }
        unsafe extern "system" fn IncomingQueueBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbincomingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::IncomingQueueBlocked(this) {
                Ok(ok__) => {
                    core::ptr::write(pbincomingblocked, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingQueueBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetIncomingQueueBlocked(this, core::mem::transmute_copy(&bincomingblocked)).into()
        }
        unsafe extern "system" fn AutoCreateAccountOnConnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbautocreateaccountonconnect: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::AutoCreateAccountOnConnect(this) {
                Ok(ok__) => {
                    core::ptr::write(pbautocreateaccountonconnect, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCreateAccountOnConnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetAutoCreateAccountOnConnect(this, core::mem::transmute_copy(&bautocreateaccountonconnect)).into()
        }
        unsafe extern "system" fn IncomingFaxesArePublic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbincomingfaxesarepublic: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxConfiguration_Impl::IncomingFaxesArePublic(this) {
                Ok(ok__) => {
                    core::ptr::write(pbincomingfaxesarepublic, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingFaxesArePublic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::SetIncomingFaxesArePublic(this, core::mem::transmute_copy(&bincomingfaxesarepublic)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxConfiguration_Impl::Save(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveLocation: ArchiveLocation::<Identity, Impl, OFFSET>,
            SetArchiveLocation: SetArchiveLocation::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            ArchiveAgeLimit: ArchiveAgeLimit::<Identity, Impl, OFFSET>,
            SetArchiveAgeLimit: SetArchiveAgeLimit::<Identity, Impl, OFFSET>,
            ArchiveSizeLow: ArchiveSizeLow::<Identity, Impl, OFFSET>,
            ArchiveSizeHigh: ArchiveSizeHigh::<Identity, Impl, OFFSET>,
            OutgoingQueueBlocked: OutgoingQueueBlocked::<Identity, Impl, OFFSET>,
            SetOutgoingQueueBlocked: SetOutgoingQueueBlocked::<Identity, Impl, OFFSET>,
            OutgoingQueuePaused: OutgoingQueuePaused::<Identity, Impl, OFFSET>,
            SetOutgoingQueuePaused: SetOutgoingQueuePaused::<Identity, Impl, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, Impl, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            SetRetries: SetRetries::<Identity, Impl, OFFSET>,
            RetryDelay: RetryDelay::<Identity, Impl, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, Impl, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, Impl, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, Impl, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, Impl, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, Impl, OFFSET>,
            OutgoingQueueAgeLimit: OutgoingQueueAgeLimit::<Identity, Impl, OFFSET>,
            SetOutgoingQueueAgeLimit: SetOutgoingQueueAgeLimit::<Identity, Impl, OFFSET>,
            Branding: Branding::<Identity, Impl, OFFSET>,
            SetBranding: SetBranding::<Identity, Impl, OFFSET>,
            IncomingQueueBlocked: IncomingQueueBlocked::<Identity, Impl, OFFSET>,
            SetIncomingQueueBlocked: SetIncomingQueueBlocked::<Identity, Impl, OFFSET>,
            AutoCreateAccountOnConnect: AutoCreateAccountOnConnect::<Identity, Impl, OFFSET>,
            SetAutoCreateAccountOnConnect: SetAutoCreateAccountOnConnect::<Identity, Impl, OFFSET>,
            IncomingFaxesArePublic: IncomingFaxesArePublic::<Identity, Impl, OFFSET>,
            SetIncomingFaxesArePublic: SetIncomingFaxesArePublic::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxConfiguration as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDevice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<i32>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProviderUniqueName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PoweredOff(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceivingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SendingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UsedRoutingMethods(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SendEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSendEnabled(&self, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReceiveMode(&self) -> windows_core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM>;
    fn SetReceiveMode(&self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::Result<()>;
    fn RingsBeforeAnswer(&self) -> windows_core::Result<i32>;
    fn SetRingsBeforeAnswer(&self, lringsbeforeanswer: i32) -> windows_core::Result<()>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCSID(&self, bstrcsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTSID(&self, bstrtsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetExtensionProperty(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn SetExtensionProperty(&self, bstrguid: &windows_core::BSTR, vproperty: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn UseRoutingMethod(&self, bstrmethodguid: &windows_core::BSTR, buse: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RingingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerCall(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxDevice {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>() -> IFaxDevice_Vtbl {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(plid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::DeviceName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdevicename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderUniqueName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprovideruniquename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::ProviderUniqueName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrprovideruniquename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PoweredOff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpoweredoff: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::PoweredOff(this) {
                Ok(ok__) => {
                    core::ptr::write(pbpoweredoff, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivingNow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreceivingnow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::ReceivingNow(this) {
                Ok(ok__) => {
                    core::ptr::write(pbreceivingnow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendingNow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsendingnow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::SendingNow(this) {
                Ok(ok__) => {
                    core::ptr::write(pbsendingnow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedRoutingMethods<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvusedroutingmethods: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::UsedRoutingMethods(this) {
                Ok(ok__) => {
                    core::ptr::write(pvusedroutingmethods, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn SendEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsendenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::SendEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbsendenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSendEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::SetSendEnabled(this, core::mem::transmute_copy(&bsendenabled)).into()
        }
        unsafe extern "system" fn ReceiveMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::ReceiveMode(this) {
                Ok(ok__) => {
                    core::ptr::write(preceivemode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::SetReceiveMode(this, core::mem::transmute_copy(&receivemode)).into()
        }
        unsafe extern "system" fn RingsBeforeAnswer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringsbeforeanswer: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::RingsBeforeAnswer(this) {
                Ok(ok__) => {
                    core::ptr::write(plringsbeforeanswer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingsBeforeAnswer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringsbeforeanswer: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::SetRingsBeforeAnswer(this, core::mem::transmute_copy(&lringsbeforeanswer)).into()
        }
        unsafe extern "system" fn CSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::CSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::SetCSID(this, core::mem::transmute(&bstrcsid)).into()
        }
        unsafe extern "system" fn TSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::TSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::SetTSID(this, core::mem::transmute(&bstrtsid)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::Save(this).into()
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, pvproperty: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::GetExtensionProperty(this, core::mem::transmute(&bstrguid)) {
                Ok(ok__) => {
                    core::ptr::write(pvproperty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, vproperty: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::SetExtensionProperty(this, core::mem::transmute(&bstrguid), core::mem::transmute(&vproperty)).into()
        }
        unsafe extern "system" fn UseRoutingMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethodguid: core::mem::MaybeUninit<windows_core::BSTR>, buse: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::UseRoutingMethod(this, core::mem::transmute(&bstrmethodguid), core::mem::transmute_copy(&buse)).into()
        }
        unsafe extern "system" fn RingingNow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbringingnow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevice_Impl::RingingNow(this) {
                Ok(ok__) => {
                    core::ptr::write(pbringingnow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDevice_Impl::AnswerCall(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            ProviderUniqueName: ProviderUniqueName::<Identity, Impl, OFFSET>,
            PoweredOff: PoweredOff::<Identity, Impl, OFFSET>,
            ReceivingNow: ReceivingNow::<Identity, Impl, OFFSET>,
            SendingNow: SendingNow::<Identity, Impl, OFFSET>,
            UsedRoutingMethods: UsedRoutingMethods::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SendEnabled: SendEnabled::<Identity, Impl, OFFSET>,
            SetSendEnabled: SetSendEnabled::<Identity, Impl, OFFSET>,
            ReceiveMode: ReceiveMode::<Identity, Impl, OFFSET>,
            SetReceiveMode: SetReceiveMode::<Identity, Impl, OFFSET>,
            RingsBeforeAnswer: RingsBeforeAnswer::<Identity, Impl, OFFSET>,
            SetRingsBeforeAnswer: SetRingsBeforeAnswer::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            SetCSID: SetCSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            SetTSID: SetTSID::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, Impl, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, Impl, OFFSET>,
            UseRoutingMethod: UseRoutingMethod::<Identity, Impl, OFFSET>,
            RingingNow: RingingNow::<Identity, Impl, OFFSET>,
            AnswerCall: AnswerCall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDevice as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceIds_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<i32>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, ldeviceid: i32) -> windows_core::Result<()>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn SetOrder(&self, ldeviceid: i32, lneworder: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxDeviceIds {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceIds_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>() -> IFaxDeviceIds_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceIds_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceIds_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pldeviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceIds_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldeviceid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDeviceIds_Impl::Add(this, core::mem::transmute_copy(&ldeviceid)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDeviceIds_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn SetOrder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDeviceIds_Impl::SetOrder(this, core::mem::transmute_copy(&ldeviceid), core::mem::transmute_copy(&lneworder)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SetOrder: SetOrder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDeviceIds as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceProvider_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UniqueName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TapiProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn MajorBuild(&self) -> windows_core::Result<i32>;
    fn MinorBuild(&self) -> windows_core::Result<i32>;
    fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(&self) -> windows_core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&self) -> windows_core::Result<i32>;
    fn DeviceIds(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxDeviceProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>() -> IFaxDeviceProvider_Vtbl {
        unsafe extern "system" fn FriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfriendlyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfriendlyname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrimagename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::ImageName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrimagename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruniquename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::UniqueName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstruniquename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapiProviderName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtapiprovidername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::TapiProviderName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtapiprovidername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::MajorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::MinorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorbuild: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::MajorBuild(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorbuild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorbuild: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::MinorBuild(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorbuild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::Debug(this) {
                Ok(ok__) => {
                    core::ptr::write(pbdebug, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pliniterrorcode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::InitErrorCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pliniterrorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdeviceids: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProvider_Impl::DeviceIds(this) {
                Ok(ok__) => {
                    core::ptr::write(pvdeviceids, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            UniqueName: UniqueName::<Identity, Impl, OFFSET>,
            TapiProviderName: TapiProviderName::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, Impl, OFFSET>,
            DeviceIds: DeviceIds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDeviceProvider as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceProviders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxDeviceProvider>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxDeviceProviders {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProviders_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>() -> IFaxDeviceProviders_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProviders_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxdeviceprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProviders_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxdeviceprovider, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDeviceProviders_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDeviceProviders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDevices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxDevice>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_ItemById(&self, lid: i32) -> windows_core::Result<IFaxDevice>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxDevices {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>() -> IFaxDevices_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevices_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevices_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxdevice, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevices_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemById<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lid: i32, ppfaxdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDevices_Impl::get_ItemById(this, core::mem::transmute_copy(&lid)) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxdevice, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_ItemById: get_ItemById::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDevices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDocument_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Body(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBody(&self, bstrbody: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Sender(&self) -> windows_core::Result<IFaxSender>;
    fn Recipients(&self) -> windows_core::Result<IFaxRecipients>;
    fn CoverPage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCoverPage(&self, bstrcoverpage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubject(&self, bstrsubject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Note(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNote(&self, bstrnote: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ScheduleTime(&self) -> windows_core::Result<f64>;
    fn SetScheduleTime(&self, datescheduletime: f64) -> windows_core::Result<()>;
    fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReceiptAddress(&self, bstrreceiptaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDocumentName(&self, bstrdocumentname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CallHandle(&self) -> windows_core::Result<i32>;
    fn SetCallHandle(&self, lcallhandle: i32) -> windows_core::Result<()>;
    fn CoverPageType(&self) -> windows_core::Result<FAX_COVERPAGE_TYPE_ENUM>;
    fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> windows_core::Result<()>;
    fn ScheduleType(&self) -> windows_core::Result<FAX_SCHEDULE_TYPE_ENUM>;
    fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> windows_core::Result<()>;
    fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> windows_core::Result<()>;
    fn GroupBroadcastReceipts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGroupBroadcastReceipts(&self, busegrouping: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> windows_core::Result<()>;
    fn TapiConnection(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn putref_TapiConnection(&self, ptapiconnection: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn Submit(&self, bstrfaxservername: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn ConnectedSubmit(&self, pfaxserver: Option<&IFaxServer>) -> windows_core::Result<windows_core::VARIANT>;
    fn AttachFaxToReceipt(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAttachFaxToReceipt(&self, battachfax: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxDocument {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>() -> IFaxDocument_Vtbl {
        unsafe extern "system" fn Body<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbody: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::Body(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrbody, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbody: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetBody(this, core::mem::transmute(&bstrbody)).into()
        }
        unsafe extern "system" fn Sender<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsender: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::Sender(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxsender, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxrecipients: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::Recipients(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxrecipients, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoverPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcoverpage: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::CoverPage(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcoverpage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcoverpage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetCoverPage(this, core::mem::transmute(&bstrcoverpage)).into()
        }
        unsafe extern "system" fn Subject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::Subject(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubject: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetSubject(this, core::mem::transmute(&bstrsubject)).into()
        }
        unsafe extern "system" fn Note<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnote: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::Note(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrnote, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNote<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnote: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetNote(this, core::mem::transmute(&bstrnote)).into()
        }
        unsafe extern "system" fn ScheduleTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatescheduletime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::ScheduleTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatescheduletime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datescheduletime: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetScheduleTime(this, core::mem::transmute_copy(&datescheduletime)).into()
        }
        unsafe extern "system" fn ReceiptAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreceiptaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::ReceiptAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrreceiptaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreceiptaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetReceiptAddress(this, core::mem::transmute(&bstrreceiptaddress)).into()
        }
        unsafe extern "system" fn DocumentName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocumentname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::DocumentName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdocumentname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdocumentname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetDocumentName(this, core::mem::transmute(&bstrdocumentname)).into()
        }
        unsafe extern "system" fn CallHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallhandle: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::CallHandle(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallhandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcallhandle: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetCallHandle(this, core::mem::transmute_copy(&lcallhandle)).into()
        }
        unsafe extern "system" fn CoverPageType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::CoverPageType(this) {
                Ok(ok__) => {
                    core::ptr::write(pcoverpagetype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPageType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetCoverPageType(this, core::mem::transmute_copy(&coverpagetype)).into()
        }
        unsafe extern "system" fn ScheduleType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::ScheduleType(this) {
                Ok(ok__) => {
                    core::ptr::write(pscheduletype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetScheduleType(this, core::mem::transmute_copy(&scheduletype)).into()
        }
        unsafe extern "system" fn ReceiptType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::ReceiptType(this) {
                Ok(ok__) => {
                    core::ptr::write(preceipttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetReceiptType(this, core::mem::transmute_copy(&receipttype)).into()
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusegrouping: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::GroupBroadcastReceipts(this) {
                Ok(ok__) => {
                    core::ptr::write(pbusegrouping, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupBroadcastReceipts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busegrouping: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetGroupBroadcastReceipts(this, core::mem::transmute_copy(&busegrouping)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::Priority(this) {
                Ok(ok__) => {
                    core::ptr::write(ppriority, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetPriority(this, core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn TapiConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptapiconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::TapiConnection(this) {
                Ok(ok__) => {
                    core::ptr::write(pptapiconnection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_TapiConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptapiconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::putref_TapiConnection(this, windows_core::from_raw_borrowed(&ptapiconnection)).into()
        }
        unsafe extern "system" fn Submit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxservername: core::mem::MaybeUninit<windows_core::BSTR>, pvfaxoutgoingjobids: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::Submit(this, core::mem::transmute(&bstrfaxservername)) {
                Ok(ok__) => {
                    core::ptr::write(pvfaxoutgoingjobids, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedSubmit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, pvfaxoutgoingjobids: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::ConnectedSubmit(this, windows_core::from_raw_borrowed(&pfaxserver)) {
                Ok(ok__) => {
                    core::ptr::write(pvfaxoutgoingjobids, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachFaxToReceipt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbattachfax: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument_Impl::AttachFaxToReceipt(this) {
                Ok(ok__) => {
                    core::ptr::write(pbattachfax, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachFaxToReceipt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, battachfax: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument_Impl::SetAttachFaxToReceipt(this, core::mem::transmute_copy(&battachfax)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipients: Recipients::<Identity, Impl, OFFSET>,
            CoverPage: CoverPage::<Identity, Impl, OFFSET>,
            SetCoverPage: SetCoverPage::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            Note: Note::<Identity, Impl, OFFSET>,
            SetNote: SetNote::<Identity, Impl, OFFSET>,
            ScheduleTime: ScheduleTime::<Identity, Impl, OFFSET>,
            SetScheduleTime: SetScheduleTime::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            SetReceiptAddress: SetReceiptAddress::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            SetDocumentName: SetDocumentName::<Identity, Impl, OFFSET>,
            CallHandle: CallHandle::<Identity, Impl, OFFSET>,
            SetCallHandle: SetCallHandle::<Identity, Impl, OFFSET>,
            CoverPageType: CoverPageType::<Identity, Impl, OFFSET>,
            SetCoverPageType: SetCoverPageType::<Identity, Impl, OFFSET>,
            ScheduleType: ScheduleType::<Identity, Impl, OFFSET>,
            SetScheduleType: SetScheduleType::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            SetReceiptType: SetReceiptType::<Identity, Impl, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            SetGroupBroadcastReceipts: SetGroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            TapiConnection: TapiConnection::<Identity, Impl, OFFSET>,
            putref_TapiConnection: putref_TapiConnection::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            ConnectedSubmit: ConnectedSubmit::<Identity, Impl, OFFSET>,
            AttachFaxToReceipt: AttachFaxToReceipt::<Identity, Impl, OFFSET>,
            SetAttachFaxToReceipt: SetAttachFaxToReceipt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDocument as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDocument2_Impl: Sized + IFaxDocument_Impl {
    fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Bodies(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetBodies(&self, vbodies: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Submit2(&self, bstrfaxservername: &windows_core::BSTR, pvfaxoutgoingjobids: *mut windows_core::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::Result<()>;
    fn ConnectedSubmit2(&self, pfaxserver: Option<&IFaxServer>, pvfaxoutgoingjobids: *mut windows_core::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>() -> IFaxDocument2_Vtbl {
        unsafe extern "system" fn SubmissionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmissionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument2_Impl::SubmissionId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubmissionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bodies<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbodies: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxDocument2_Impl::Bodies(this) {
                Ok(ok__) => {
                    core::ptr::write(pvbodies, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodies<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbodies: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument2_Impl::SetBodies(this, core::mem::transmute(&vbodies)).into()
        }
        unsafe extern "system" fn Submit2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxservername: core::mem::MaybeUninit<windows_core::BSTR>, pvfaxoutgoingjobids: *mut core::mem::MaybeUninit<windows_core::VARIANT>, plerrorbodyfile: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument2_Impl::Submit2(this, core::mem::transmute(&bstrfaxservername), core::mem::transmute_copy(&pvfaxoutgoingjobids), core::mem::transmute_copy(&plerrorbodyfile)).into()
        }
        unsafe extern "system" fn ConnectedSubmit2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, pvfaxoutgoingjobids: *mut core::mem::MaybeUninit<windows_core::VARIANT>, plerrorbodyfile: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxDocument2_Impl::ConnectedSubmit2(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute_copy(&pvfaxoutgoingjobids), core::mem::transmute_copy(&plerrorbodyfile)).into()
        }
        Self {
            base__: IFaxDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Bodies: Bodies::<Identity, Impl, OFFSET>,
            SetBodies: SetBodies::<Identity, Impl, OFFSET>,
            Submit2: Submit2::<Identity, Impl, OFFSET>,
            ConnectedSubmit2: ConnectedSubmit2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDocument2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxDocument as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxEventLogging_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn InitEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInitEventsLevel(&self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn InboundEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInboundEventsLevel(&self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn OutboundEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetOutboundEventsLevel(&self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn GeneralEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetGeneralEventsLevel(&self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxEventLogging {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxEventLogging_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>() -> IFaxEventLogging_Vtbl {
        unsafe extern "system" fn InitEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxEventLogging_Impl::InitEventsLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(piniteventlevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxEventLogging_Impl::SetInitEventsLevel(this, core::mem::transmute_copy(&initeventlevel)).into()
        }
        unsafe extern "system" fn InboundEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxEventLogging_Impl::InboundEventsLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(pinboundeventlevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInboundEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxEventLogging_Impl::SetInboundEventsLevel(this, core::mem::transmute_copy(&inboundeventlevel)).into()
        }
        unsafe extern "system" fn OutboundEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxEventLogging_Impl::OutboundEventsLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(poutboundeventlevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxEventLogging_Impl::SetOutboundEventsLevel(this, core::mem::transmute_copy(&outboundeventlevel)).into()
        }
        unsafe extern "system" fn GeneralEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxEventLogging_Impl::GeneralEventsLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(pgeneraleventlevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneralEventsLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxEventLogging_Impl::SetGeneralEventsLevel(this, core::mem::transmute_copy(&generaleventlevel)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxEventLogging_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxEventLogging_Impl::Save(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitEventsLevel: InitEventsLevel::<Identity, Impl, OFFSET>,
            SetInitEventsLevel: SetInitEventsLevel::<Identity, Impl, OFFSET>,
            InboundEventsLevel: InboundEventsLevel::<Identity, Impl, OFFSET>,
            SetInboundEventsLevel: SetInboundEventsLevel::<Identity, Impl, OFFSET>,
            OutboundEventsLevel: OutboundEventsLevel::<Identity, Impl, OFFSET>,
            SetOutboundEventsLevel: SetOutboundEventsLevel::<Identity, Impl, OFFSET>,
            GeneralEventsLevel: GeneralEventsLevel::<Identity, Impl, OFFSET>,
            SetGeneralEventsLevel: SetGeneralEventsLevel::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxEventLogging as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxFolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&self) -> windows_core::Result<IFaxOutgoingQueue>;
    fn IncomingQueue(&self) -> windows_core::Result<IFaxIncomingQueue>;
    fn IncomingArchive(&self) -> windows_core::Result<IFaxIncomingArchive>;
    fn OutgoingArchive(&self) -> windows_core::Result<IFaxOutgoingArchive>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxFolders {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxFolders_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>() -> IFaxFolders_Vtbl {
        unsafe extern "system" fn OutgoingQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxFolders_Impl::OutgoingQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingqueue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxFolders_Impl::IncomingQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingqueue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxFolders_Impl::IncomingArchive(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingarchive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxFolders_Impl::OutgoingArchive(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingarchive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Identity, Impl, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, Impl, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, Impl, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxFolders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRouting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetExtensions(&self) -> windows_core::Result<IFaxInboundRoutingExtensions>;
    fn GetMethods(&self) -> windows_core::Result<IFaxInboundRoutingMethods>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxInboundRouting {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRouting_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: isize>() -> IFaxInboundRouting_Vtbl {
        unsafe extern "system" fn GetExtensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxinboundroutingextensions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRouting_Impl::GetExtensions(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxinboundroutingextensions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethods<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxinboundroutingmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRouting_Impl::GetMethods(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxinboundroutingmethods, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetExtensions: GetExtensions::<Identity, Impl, OFFSET>,
            GetMethods: GetMethods::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRouting as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingExtension_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UniqueName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn MajorBuild(&self) -> windows_core::Result<i32>;
    fn MinorBuild(&self) -> windows_core::Result<i32>;
    fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(&self) -> windows_core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&self) -> windows_core::Result<i32>;
    fn Methods(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxInboundRoutingExtension {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>() -> IFaxInboundRoutingExtension_Vtbl {
        unsafe extern "system" fn FriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfriendlyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfriendlyname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrimagename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::ImageName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrimagename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruniquename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::UniqueName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstruniquename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::MajorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::MinorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorbuild: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::MajorBuild(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorbuild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorbuild: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::MinorBuild(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorbuild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::Debug(this) {
                Ok(ok__) => {
                    core::ptr::write(pbdebug, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pliniterrorcode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::InitErrorCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pliniterrorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmethods: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtension_Impl::Methods(this) {
                Ok(ok__) => {
                    core::ptr::write(pvmethods, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            UniqueName: UniqueName::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, Impl, OFFSET>,
            Methods: Methods::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtension as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingExtensions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxInboundRoutingExtension>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxInboundRoutingExtensions {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtensions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>() -> IFaxInboundRoutingExtensions_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtensions_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxinboundroutingextension: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtensions_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxinboundroutingextension, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingExtensions_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtensions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingMethod_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GUID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FunctionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExtensionFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExtensionImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxInboundRoutingMethod {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethod_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>() -> IFaxInboundRoutingMethod_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethod_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethod_Impl::GUID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrguid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FunctionName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfunctionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethod_Impl::FunctionName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfunctionname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionFriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextensionfriendlyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethod_Impl::ExtensionFriendlyName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrextensionfriendlyname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionImageName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextensionimagename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethod_Impl::ExtensionImageName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrextensionimagename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethod_Impl::Priority(this) {
                Ok(ok__) => {
                    core::ptr::write(plpriority, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxInboundRoutingMethod_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxInboundRoutingMethod_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxInboundRoutingMethod_Impl::Save(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            GUID: GUID::<Identity, Impl, OFFSET>,
            FunctionName: FunctionName::<Identity, Impl, OFFSET>,
            ExtensionFriendlyName: ExtensionFriendlyName::<Identity, Impl, OFFSET>,
            ExtensionImageName: ExtensionImageName::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethod as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingMethods_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxInboundRoutingMethod>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxInboundRoutingMethods {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethods_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>() -> IFaxInboundRoutingMethods_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethods_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxinboundroutingmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethods_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxinboundroutingmethod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxInboundRoutingMethods_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethods as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ArchiveFolder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArchiveFolder(&self, bstrarchivefolder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HighQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()>;
    fn LowQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()>;
    fn AgeLimit(&self) -> windows_core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()>;
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingMessage>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingArchive_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>() -> IFaxIncomingArchive_Vtbl {
        unsafe extern "system" fn UseArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::UseArchive(this) {
                Ok(ok__) => {
                    core::ptr::write(pbusearchive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::SetUseArchive(this, core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveFolder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrarchivefolder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::ArchiveFolder(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrarchivefolder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrarchivefolder: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::SetArchiveFolder(this, core::mem::transmute(&bstrarchivefolder)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::SizeQuotaWarning(this) {
                Ok(ok__) => {
                    core::ptr::write(pbsizequotawarning, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::SetSizeQuotaWarning(this, core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhighquotawatermark: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::HighQuotaWaterMark(this) {
                Ok(ok__) => {
                    core::ptr::write(plhighquotawatermark, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhighquotawatermark: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::SetHighQuotaWaterMark(this, core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllowquotawatermark: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::LowQuotaWaterMark(this) {
                Ok(ok__) => {
                    core::ptr::write(pllowquotawatermark, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowquotawatermark: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::SetLowQuotaWaterMark(this, core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn AgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plagelimit: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::AgeLimit(this) {
                Ok(ok__) => {
                    core::ptr::write(plagelimit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lagelimit: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::SetAgeLimit(this, core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn SizeLow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::SizeLow(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizelow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::SizeHigh(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizehigh, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingArchive_Impl::Save(this).into()
        }
        unsafe extern "system" fn GetMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingmessageiterator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxincomingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, Impl, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingJob_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Size(&self) -> windows_core::Result<i32>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentPage(&self) -> windows_core::Result<i32>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CallerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn JobType(&self) -> windows_core::Result<FAX_JOB_TYPE_ENUM>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxIncomingJob {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>() -> IFaxIncomingJob_Vtbl {
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(plsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcurrentpage: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::CurrentPage(this) {
                Ok(ok__) => {
                    core::ptr::write(plcurrentpage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::DeviceId(this) {
                Ok(ok__) => {
                    core::ptr::write(pldeviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::ExtendedStatusCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pextendedstatuscode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextendedstatus: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::ExtendedStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrextendedstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::AvailableOperations(this) {
                Ok(ok__) => {
                    core::ptr::write(pavailableoperations, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::Retries(this) {
                Ok(ok__) => {
                    core::ptr::write(plretries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::TransmissionStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionstart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::TransmissionEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::CSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::TSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcallerid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::CallerId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcallerid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrroutinginformation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::RoutingInformation(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrroutinginformation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJob_Impl::JobType(this) {
                Ok(ok__) => {
                    core::ptr::write(pjobtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingJob_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingJob_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn CopyTiff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingJob_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Size: Size::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
            JobType: JobType::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingJob as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingJobs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxIncomingJob>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxIncomingJobs {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJobs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>() -> IFaxIncomingJobs_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJobs_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxincomingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJobs_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingJobs_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingJobs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CallerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxIncomingMessage {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>() -> IFaxIncomingMessage_Vtbl {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::Pages(this) {
                Ok(ok__) => {
                    core::ptr::write(plpages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(plsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::DeviceName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdevicename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::Retries(this) {
                Ok(ok__) => {
                    core::ptr::write(plretries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::TransmissionStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionstart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::TransmissionEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::CSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::TSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcallerid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::CallerId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcallerid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrroutinginformation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage_Impl::RoutingInformation(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrroutinginformation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage_Impl::Delete(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingMessage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessage2_Impl: Sized + IFaxIncomingMessage_Impl {
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubject(&self, bstrsubject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SenderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSenderName(&self, bstrsendername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SenderFaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSenderFaxNumber(&self, bstrsenderfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHasCoverPage(&self, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Recipients(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRecipients(&self, bstrrecipients: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WasReAssigned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Read(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReAssign(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxIncomingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>() -> IFaxIncomingMessage2_Vtbl {
        unsafe extern "system" fn Subject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage2_Impl::Subject(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubject: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::SetSubject(this, core::mem::transmute(&bstrsubject)).into()
        }
        unsafe extern "system" fn SenderName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsendername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage2_Impl::SenderName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsendername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsendername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::SetSenderName(this, core::mem::transmute(&bstrsendername)).into()
        }
        unsafe extern "system" fn SenderFaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsenderfaxnumber: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage2_Impl::SenderFaxNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsenderfaxnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderFaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsenderfaxnumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::SetSenderFaxNumber(this, core::mem::transmute(&bstrsenderfaxnumber)).into()
        }
        unsafe extern "system" fn HasCoverPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage2_Impl::HasCoverPage(this) {
                Ok(ok__) => {
                    core::ptr::write(pbhascoverpage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHasCoverPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::SetHasCoverPage(this, core::mem::transmute_copy(&bhascoverpage)).into()
        }
        unsafe extern "system" fn Recipients<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrecipients: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage2_Impl::Recipients(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrrecipients, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecipients<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrecipients: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::SetRecipients(this, core::mem::transmute(&bstrrecipients)).into()
        }
        unsafe extern "system" fn WasReAssigned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbwasreassigned: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage2_Impl::WasReAssigned(this) {
                Ok(ok__) => {
                    core::ptr::write(pbwasreassigned, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessage2_Impl::Read(this) {
                Ok(ok__) => {
                    core::ptr::write(pbread, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::SetRead(this, core::mem::transmute_copy(&bread)).into()
        }
        unsafe extern "system" fn ReAssign<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::ReAssign(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::Save(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessage2_Impl::Refresh(this).into()
        }
        Self {
            base__: IFaxIncomingMessage_Vtbl::new::<Identity, Impl, OFFSET>(),
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            SenderName: SenderName::<Identity, Impl, OFFSET>,
            SetSenderName: SetSenderName::<Identity, Impl, OFFSET>,
            SenderFaxNumber: SenderFaxNumber::<Identity, Impl, OFFSET>,
            SetSenderFaxNumber: SetSenderFaxNumber::<Identity, Impl, OFFSET>,
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            SetHasCoverPage: SetHasCoverPage::<Identity, Impl, OFFSET>,
            Recipients: Recipients::<Identity, Impl, OFFSET>,
            SetRecipients: SetRecipients::<Identity, Impl, OFFSET>,
            WasReAssigned: WasReAssigned::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            SetRead: SetRead::<Identity, Impl, OFFSET>,
            ReAssign: ReAssign::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingMessage2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxIncomingMessage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessageIterator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Message(&self) -> windows_core::Result<IFaxIncomingMessage>;
    fn PrefetchSize(&self) -> windows_core::Result<i32>;
    fn SetPrefetchSize(&self, lprefetchsize: i32) -> windows_core::Result<()>;
    fn AtEOF(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MoveFirst(&self) -> windows_core::Result<()>;
    fn MoveNext(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxIncomingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessageIterator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>() -> IFaxIncomingMessageIterator_Vtbl {
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessageIterator_Impl::Message(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprefetchsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessageIterator_Impl::PrefetchSize(this) {
                Ok(ok__) => {
                    core::ptr::write(plprefetchsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessageIterator_Impl::SetPrefetchSize(this, core::mem::transmute_copy(&lprefetchsize)).into()
        }
        unsafe extern "system" fn AtEOF<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingMessageIterator_Impl::AtEOF(this) {
                Ok(ok__) => {
                    core::ptr::write(pbeof, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveFirst<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessageIterator_Impl::MoveFirst(this).into()
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingMessageIterator_Impl::MoveNext(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Message: Message::<Identity, Impl, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, Impl, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, Impl, OFFSET>,
            AtEOF: AtEOF::<Identity, Impl, OFFSET>,
            MoveFirst: MoveFirst::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingMessageIterator as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Blocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetJobs(&self) -> windows_core::Result<IFaxIncomingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingJob>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>() -> IFaxIncomingQueue_Vtbl {
        unsafe extern "system" fn Blocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingQueue_Impl::Blocked(this) {
                Ok(ok__) => {
                    core::ptr::write(pbblocked, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingQueue_Impl::SetBlocked(this, core::mem::transmute_copy(&bblocked)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingQueue_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxIncomingQueue_Impl::Save(this).into()
        }
        unsafe extern "system" fn GetJobs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingQueue_Impl::GetJobs(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingjobs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxincomingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxIncomingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxincomingjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Blocked: Blocked::<Identity, Impl, OFFSET>,
            SetBlocked: SetBlocked::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxJobStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn CurrentPage(&self) -> windows_core::Result<i32>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn JobType(&self) -> windows_core::Result<FAX_JOB_TYPE_ENUM>;
    fn ScheduledTime(&self) -> windows_core::Result<f64>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CallerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxJobStatus {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxJobStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>() -> IFaxJobStatus_Vtbl {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::Pages(this) {
                Ok(ok__) => {
                    core::ptr::write(plpages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(plsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcurrentpage: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::CurrentPage(this) {
                Ok(ok__) => {
                    core::ptr::write(plcurrentpage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::DeviceId(this) {
                Ok(ok__) => {
                    core::ptr::write(pldeviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::CSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::TSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::ExtendedStatusCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pextendedstatuscode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextendedstatus: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::ExtendedStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrextendedstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::AvailableOperations(this) {
                Ok(ok__) => {
                    core::ptr::write(pavailableoperations, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::Retries(this) {
                Ok(ok__) => {
                    core::ptr::write(plretries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::JobType(this) {
                Ok(ok__) => {
                    core::ptr::write(pjobtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatescheduledtime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::ScheduledTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatescheduledtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::TransmissionStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionstart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::TransmissionEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcallerid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::CallerId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcallerid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrroutinginformation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxJobStatus_Impl::RoutingInformation(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrroutinginformation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            JobType: JobType::<Identity, Impl, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxJobStatus as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxLoggingOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EventLogging(&self) -> windows_core::Result<IFaxEventLogging>;
    fn ActivityLogging(&self) -> windows_core::Result<IFaxActivityLogging>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxLoggingOptions {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxLoggingOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>() -> IFaxLoggingOptions_Vtbl {
        unsafe extern "system" fn EventLogging<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxeventlogging: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxLoggingOptions_Impl::EventLogging(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxeventlogging, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityLogging<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxactivitylogging: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxLoggingOptions_Impl::ActivityLogging(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxactivitylogging, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventLogging: EventLogging::<Identity, Impl, OFFSET>,
            ActivityLogging: ActivityLogging::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxLoggingOptions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRouting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetGroups(&self) -> windows_core::Result<IFaxOutboundRoutingGroups>;
    fn GetRules(&self) -> windows_core::Result<IFaxOutboundRoutingRules>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutboundRouting {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRouting_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>() -> IFaxOutboundRouting_Vtbl {
        unsafe extern "system" fn GetGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutboundroutinggroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRouting_Impl::GetGroups(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutboundroutinggroups, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRules<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutboundroutingrules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRouting_Impl::GetRules(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutboundroutingrules, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGroups: GetGroups::<Identity, Impl, OFFSET>,
            GetRules: GetRules::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRouting as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<FAX_GROUP_STATUS_ENUM>;
    fn DeviceIds(&self) -> windows_core::Result<IFaxDeviceIds>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutboundRoutingGroup {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingGroup_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingGroup_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxdeviceids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingGroup_Impl::DeviceIds(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxdeviceids, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            DeviceIds: DeviceIds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxOutboundRoutingGroup>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IFaxOutboundRoutingGroup>;
    fn Remove(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutboundRoutingGroups {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroups_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingGroups_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingGroups_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxoutboundroutinggroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingGroups_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutboundroutinggroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingGroups_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pfaxoutboundroutinggroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingGroups_Impl::Add(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutboundroutinggroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingGroups_Impl::Remove(this, core::mem::transmute(&vindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroups as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CountryCode(&self) -> windows_core::Result<i32>;
    fn AreaCode(&self) -> windows_core::Result<i32>;
    fn Status(&self) -> windows_core::Result<FAX_RULE_STATUS_ENUM>;
    fn UseDevice(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDevice(&self, busedevice: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn SetDeviceId(&self, deviceid: i32) -> windows_core::Result<()>;
    fn GroupName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGroupName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutboundRoutingRule {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRule_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingRule_Vtbl {
        unsafe extern "system" fn CountryCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcountrycode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRule_Impl::CountryCode(this) {
                Ok(ok__) => {
                    core::ptr::write(plcountrycode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreaCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plareacode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRule_Impl::AreaCode(this) {
                Ok(ok__) => {
                    core::ptr::write(plareacode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRule_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusedevice: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRule_Impl::UseDevice(this) {
                Ok(ok__) => {
                    core::ptr::write(pbusedevice, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busedevice: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingRule_Impl::SetUseDevice(this, core::mem::transmute_copy(&busedevice)).into()
        }
        unsafe extern "system" fn DeviceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRule_Impl::DeviceId(this) {
                Ok(ok__) => {
                    core::ptr::write(pldeviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingRule_Impl::SetDeviceId(this, core::mem::transmute_copy(&deviceid)).into()
        }
        unsafe extern "system" fn GroupName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrgroupname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRule_Impl::GroupName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrgroupname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingRule_Impl::SetGroupName(this, core::mem::transmute(&bstrgroupname)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingRule_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingRule_Impl::Save(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CountryCode: CountryCode::<Identity, Impl, OFFSET>,
            AreaCode: AreaCode::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            UseDevice: UseDevice::<Identity, Impl, OFFSET>,
            SetUseDevice: SetUseDevice::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            GroupName: GroupName::<Identity, Impl, OFFSET>,
            SetGroupName: SetGroupName::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<IFaxOutboundRoutingRule>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ItemByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> windows_core::Result<IFaxOutboundRoutingRule>;
    fn RemoveByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> windows_core::Result<()>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn Add(&self, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: &windows_core::BSTR, ldeviceid: i32) -> windows_core::Result<IFaxOutboundRoutingRule>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutboundRoutingRules {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRules_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingRules_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRules_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRules_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutboundroutingrule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRules_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByCountryAndArea<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRules_Impl::ItemByCountryAndArea(this, core::mem::transmute_copy(&lcountrycode), core::mem::transmute_copy(&lareacode)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutboundroutingrule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveByCountryAndArea<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingRules_Impl::RemoveByCountryAndArea(this, core::mem::transmute_copy(&lcountrycode), core::mem::transmute_copy(&lareacode)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutboundRoutingRules_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutboundRoutingRules_Impl::Add(this, core::mem::transmute_copy(&lcountrycode), core::mem::transmute_copy(&lareacode), core::mem::transmute_copy(&busedevice), core::mem::transmute(&bstrgroupname), core::mem::transmute_copy(&ldeviceid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutboundroutingrule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ItemByCountryAndArea: ItemByCountryAndArea::<Identity, Impl, OFFSET>,
            RemoveByCountryAndArea: RemoveByCountryAndArea::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRules as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ArchiveFolder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArchiveFolder(&self, bstrarchivefolder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HighQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()>;
    fn LowQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()>;
    fn AgeLimit(&self) -> windows_core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()>;
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingMessage>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingArchive_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>() -> IFaxOutgoingArchive_Vtbl {
        unsafe extern "system" fn UseArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::UseArchive(this) {
                Ok(ok__) => {
                    core::ptr::write(pbusearchive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::SetUseArchive(this, core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveFolder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrarchivefolder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::ArchiveFolder(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrarchivefolder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrarchivefolder: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::SetArchiveFolder(this, core::mem::transmute(&bstrarchivefolder)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::SizeQuotaWarning(this) {
                Ok(ok__) => {
                    core::ptr::write(pbsizequotawarning, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::SetSizeQuotaWarning(this, core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhighquotawatermark: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::HighQuotaWaterMark(this) {
                Ok(ok__) => {
                    core::ptr::write(plhighquotawatermark, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhighquotawatermark: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::SetHighQuotaWaterMark(this, core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllowquotawatermark: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::LowQuotaWaterMark(this) {
                Ok(ok__) => {
                    core::ptr::write(pllowquotawatermark, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowquotawatermark: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::SetLowQuotaWaterMark(this, core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn AgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plagelimit: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::AgeLimit(this) {
                Ok(ok__) => {
                    core::ptr::write(plagelimit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lagelimit: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::SetAgeLimit(this, core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn SizeLow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::SizeLow(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizelow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::SizeHigh(this) {
                Ok(ok__) => {
                    core::ptr::write(plsizehigh, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingArchive_Impl::Save(this).into()
        }
        unsafe extern "system" fn GetMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingmessageiterator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxoutgoingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, Impl, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJob_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OriginalScheduledTime(&self) -> windows_core::Result<f64>;
    fn SubmissionTime(&self) -> windows_core::Result<f64>;
    fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&self) -> windows_core::Result<IFaxSender>;
    fn Recipient(&self) -> windows_core::Result<IFaxRecipient>;
    fn CurrentPage(&self) -> windows_core::Result<i32>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn ScheduledTime(&self) -> windows_core::Result<f64>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GroupBroadcastReceipts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Restart(&self) -> windows_core::Result<()>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingJob {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>() -> IFaxOutgoingJob_Vtbl {
        unsafe extern "system" fn Subject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Subject(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocumentname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::DocumentName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdocumentname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Pages(this) {
                Ok(ok__) => {
                    core::ptr::write(plpages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(plsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmissionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::SubmissionId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubmissionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::OriginalScheduledTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdateoriginalscheduledtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatesubmissiontime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::SubmissionTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatesubmissiontime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::ReceiptType(this) {
                Ok(ok__) => {
                    core::ptr::write(preceipttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Priority(this) {
                Ok(ok__) => {
                    core::ptr::write(ppriority, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsender: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Sender(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxsender, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Recipient(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxrecipient, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcurrentpage: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::CurrentPage(this) {
                Ok(ok__) => {
                    core::ptr::write(plcurrentpage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::DeviceId(this) {
                Ok(ok__) => {
                    core::ptr::write(pldeviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::ExtendedStatusCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pextendedstatuscode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextendedstatus: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::ExtendedStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrextendedstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::AvailableOperations(this) {
                Ok(ok__) => {
                    core::ptr::write(pavailableoperations, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::Retries(this) {
                Ok(ok__) => {
                    core::ptr::write(plretries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatescheduledtime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::ScheduledTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatescheduledtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::TransmissionStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionstart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::TransmissionEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::CSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::TSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgroupbroadcastreceipts: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob_Impl::GroupBroadcastReceipts(this) {
                Ok(ok__) => {
                    core::ptr::write(pbgroupbroadcastreceipts, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingJob_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingJob_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Restart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingJob_Impl::Restart(this).into()
        }
        unsafe extern "system" fn CopyTiff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingJob_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingJob_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingJob_Impl::Cancel(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Subject: Subject::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipient: Recipient::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Restart: Restart::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingJob as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJob2_Impl: Sized + IFaxOutgoingJob_Impl {
    fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ScheduleType(&self) -> windows_core::Result<FAX_SCHEDULE_TYPE_ENUM>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingJob2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>() -> IFaxOutgoingJob2_Vtbl {
        unsafe extern "system" fn HasCoverPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob2_Impl::HasCoverPage(this) {
                Ok(ok__) => {
                    core::ptr::write(pbhascoverpage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreceiptaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob2_Impl::ReceiptAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrreceiptaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJob2_Impl::ScheduleType(this) {
                Ok(ok__) => {
                    core::ptr::write(pscheduletype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IFaxOutgoingJob_Vtbl::new::<Identity, Impl, OFFSET>(),
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            ScheduleType: ScheduleType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingJob2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxOutgoingJob as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJobs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &windows_core::VARIANT) -> windows_core::Result<IFaxOutgoingJob>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingJobs {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJobs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>() -> IFaxOutgoingJobs_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJobs_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: core::mem::MaybeUninit<windows_core::VARIANT>, pfaxoutgoingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJobs_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingJobs_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingJobs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn OriginalScheduledTime(&self) -> windows_core::Result<f64>;
    fn SubmissionTime(&self) -> windows_core::Result<f64>;
    fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&self) -> windows_core::Result<IFaxSender>;
    fn Recipient(&self) -> windows_core::Result<IFaxRecipient>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingMessage {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>() -> IFaxOutgoingMessage_Vtbl {
        unsafe extern "system" fn SubmissionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmissionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::SubmissionId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubmissionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Subject(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocumentname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::DocumentName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdocumentname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Retries(this) {
                Ok(ok__) => {
                    core::ptr::write(plretries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Pages(this) {
                Ok(ok__) => {
                    core::ptr::write(plpages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(plsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::OriginalScheduledTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdateoriginalscheduledtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatesubmissiontime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::SubmissionTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatesubmissiontime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Priority(this) {
                Ok(ok__) => {
                    core::ptr::write(ppriority, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsender: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Sender(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxsender, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::Recipient(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxrecipient, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::DeviceName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdevicename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::TransmissionStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionstart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::TransmissionEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatetransmissionend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::CSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage_Impl::TSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessage_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessage_Impl::Delete(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipient: Recipient::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessage2_Impl: Sized + IFaxOutgoingMessage_Impl {
    fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Read(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>() -> IFaxOutgoingMessage2_Vtbl {
        unsafe extern "system" fn HasCoverPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage2_Impl::HasCoverPage(this) {
                Ok(ok__) => {
                    core::ptr::write(pbhascoverpage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage2_Impl::ReceiptType(this) {
                Ok(ok__) => {
                    core::ptr::write(preceipttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreceiptaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage2_Impl::ReceiptAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrreceiptaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessage2_Impl::Read(this) {
                Ok(ok__) => {
                    core::ptr::write(pbread, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessage2_Impl::SetRead(this, core::mem::transmute_copy(&bread)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessage2_Impl::Save(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessage2_Impl::Refresh(this).into()
        }
        Self {
            base__: IFaxOutgoingMessage_Vtbl::new::<Identity, Impl, OFFSET>(),
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            SetRead: SetRead::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxOutgoingMessage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessageIterator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Message(&self) -> windows_core::Result<IFaxOutgoingMessage>;
    fn AtEOF(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn PrefetchSize(&self) -> windows_core::Result<i32>;
    fn SetPrefetchSize(&self, lprefetchsize: i32) -> windows_core::Result<()>;
    fn MoveFirst(&self) -> windows_core::Result<()>;
    fn MoveNext(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessageIterator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>() -> IFaxOutgoingMessageIterator_Vtbl {
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessageIterator_Impl::Message(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEOF<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessageIterator_Impl::AtEOF(this) {
                Ok(ok__) => {
                    core::ptr::write(pbeof, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprefetchsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingMessageIterator_Impl::PrefetchSize(this) {
                Ok(ok__) => {
                    core::ptr::write(plprefetchsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessageIterator_Impl::SetPrefetchSize(this, core::mem::transmute_copy(&lprefetchsize)).into()
        }
        unsafe extern "system" fn MoveFirst<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessageIterator_Impl::MoveFirst(this).into()
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingMessageIterator_Impl::MoveNext(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Message: Message::<Identity, Impl, OFFSET>,
            AtEOF: AtEOF::<Identity, Impl, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, Impl, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, Impl, OFFSET>,
            MoveFirst: MoveFirst::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingMessageIterator as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Blocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Paused(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPaused(&self, bpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowPersonalCoverPages(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseDeviceTSID(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn SetRetries(&self, lretries: i32) -> windows_core::Result<()>;
    fn RetryDelay(&self) -> windows_core::Result<i32>;
    fn SetRetryDelay(&self, lretrydelay: i32) -> windows_core::Result<()>;
    fn DiscountRateStart(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateStart(&self, datediscountratestart: f64) -> windows_core::Result<()>;
    fn DiscountRateEnd(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> windows_core::Result<()>;
    fn AgeLimit(&self) -> windows_core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()>;
    fn Branding(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetJobs(&self) -> windows_core::Result<IFaxOutgoingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingJob>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>() -> IFaxOutgoingQueue_Vtbl {
        unsafe extern "system" fn Blocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::Blocked(this) {
                Ok(ok__) => {
                    core::ptr::write(pbblocked, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetBlocked(this, core::mem::transmute_copy(&bblocked)).into()
        }
        unsafe extern "system" fn Paused<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpaused: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::Paused(this) {
                Ok(ok__) => {
                    core::ptr::write(pbpaused, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPaused<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetPaused(this, core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::AllowPersonalCoverPages(this) {
                Ok(ok__) => {
                    core::ptr::write(pballowpersonalcoverpages, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetAllowPersonalCoverPages(this, core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::UseDeviceTSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbusedevicetsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetUseDeviceTSID(this, core::mem::transmute_copy(&busedevicetsid)).into()
        }
        unsafe extern "system" fn Retries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::Retries(this) {
                Ok(ok__) => {
                    core::ptr::write(plretries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretries: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetRetries(this, core::mem::transmute_copy(&lretries)).into()
        }
        unsafe extern "system" fn RetryDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretrydelay: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::RetryDelay(this) {
                Ok(ok__) => {
                    core::ptr::write(plretrydelay, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretrydelay: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetRetryDelay(this, core::mem::transmute_copy(&lretrydelay)).into()
        }
        unsafe extern "system" fn DiscountRateStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountratestart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::DiscountRateStart(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatediscountratestart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountratestart: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetDiscountRateStart(this, core::mem::transmute_copy(&datediscountratestart)).into()
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountrateend: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::DiscountRateEnd(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatediscountrateend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountrateend: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetDiscountRateEnd(this, core::mem::transmute_copy(&datediscountrateend)).into()
        }
        unsafe extern "system" fn AgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plagelimit: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::AgeLimit(this) {
                Ok(ok__) => {
                    core::ptr::write(plagelimit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lagelimit: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetAgeLimit(this, core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn Branding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::Branding(this) {
                Ok(ok__) => {
                    core::ptr::write(pbbranding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::SetBranding(this, core::mem::transmute_copy(&bbranding)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxOutgoingQueue_Impl::Save(this).into()
        }
        unsafe extern "system" fn GetJobs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::GetJobs(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingjobs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pfaxoutgoingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxOutgoingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                Ok(ok__) => {
                    core::ptr::write(pfaxoutgoingjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Blocked: Blocked::<Identity, Impl, OFFSET>,
            SetBlocked: SetBlocked::<Identity, Impl, OFFSET>,
            Paused: Paused::<Identity, Impl, OFFSET>,
            SetPaused: SetPaused::<Identity, Impl, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, Impl, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            SetRetries: SetRetries::<Identity, Impl, OFFSET>,
            RetryDelay: RetryDelay::<Identity, Impl, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, Impl, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, Impl, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, Impl, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, Impl, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            Branding: Branding::<Identity, Impl, OFFSET>,
            SetBranding: SetBranding::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxReceiptOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AuthenticationType(&self) -> windows_core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM>;
    fn SetAuthenticationType(&self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::Result<()>;
    fn SMTPServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPServer(&self, bstrsmtpserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SMTPPort(&self) -> windows_core::Result<i32>;
    fn SetSMTPPort(&self, lsmtpport: i32) -> windows_core::Result<()>;
    fn SMTPSender(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPSender(&self, bstrsmtpsender: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SMTPUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPUser(&self, bstrsmtpuser: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowedReceipts(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetAllowedReceipts(&self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> windows_core::Result<()>;
    fn SMTPPassword(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPPassword(&self, bstrsmtppassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn UseForInboundRouting(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseForInboundRouting(&self, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxReceiptOptions {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxReceiptOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>() -> IFaxReceiptOptions_Vtbl {
        unsafe extern "system" fn AuthenticationType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::AuthenticationType(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetAuthenticationType(this, core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn SMTPServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtpserver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::SMTPServer(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsmtpserver, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtpserver: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetSMTPServer(this, core::mem::transmute(&bstrsmtpserver)).into()
        }
        unsafe extern "system" fn SMTPPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsmtpport: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::SMTPPort(this) {
                Ok(ok__) => {
                    core::ptr::write(plsmtpport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsmtpport: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetSMTPPort(this, core::mem::transmute_copy(&lsmtpport)).into()
        }
        unsafe extern "system" fn SMTPSender<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtpsender: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::SMTPSender(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsmtpsender, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPSender<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtpsender: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetSMTPSender(this, core::mem::transmute(&bstrsmtpsender)).into()
        }
        unsafe extern "system" fn SMTPUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtpuser: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::SMTPUser(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsmtpuser, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtpuser: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetSMTPUser(this, core::mem::transmute(&bstrsmtpuser)).into()
        }
        unsafe extern "system" fn AllowedReceipts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::AllowedReceipts(this) {
                Ok(ok__) => {
                    core::ptr::write(pallowedreceipts, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedReceipts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetAllowedReceipts(this, core::mem::transmute_copy(&allowedreceipts)).into()
        }
        unsafe extern "system" fn SMTPPassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtppassword: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::SMTPPassword(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsmtppassword, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtppassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetSMTPPassword(this, core::mem::transmute(&bstrsmtppassword)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::Save(this).into()
        }
        unsafe extern "system" fn UseForInboundRouting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuseforinboundrouting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxReceiptOptions_Impl::UseForInboundRouting(this) {
                Ok(ok__) => {
                    core::ptr::write(pbuseforinboundrouting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseForInboundRouting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxReceiptOptions_Impl::SetUseForInboundRouting(this, core::mem::transmute_copy(&buseforinboundrouting)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AuthenticationType: AuthenticationType::<Identity, Impl, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, Impl, OFFSET>,
            SMTPServer: SMTPServer::<Identity, Impl, OFFSET>,
            SetSMTPServer: SetSMTPServer::<Identity, Impl, OFFSET>,
            SMTPPort: SMTPPort::<Identity, Impl, OFFSET>,
            SetSMTPPort: SetSMTPPort::<Identity, Impl, OFFSET>,
            SMTPSender: SMTPSender::<Identity, Impl, OFFSET>,
            SetSMTPSender: SetSMTPSender::<Identity, Impl, OFFSET>,
            SMTPUser: SMTPUser::<Identity, Impl, OFFSET>,
            SetSMTPUser: SetSMTPUser::<Identity, Impl, OFFSET>,
            AllowedReceipts: AllowedReceipts::<Identity, Impl, OFFSET>,
            SetAllowedReceipts: SetAllowedReceipts::<Identity, Impl, OFFSET>,
            SMTPPassword: SMTPPassword::<Identity, Impl, OFFSET>,
            SetSMTPPassword: SetSMTPPassword::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            UseForInboundRouting: UseForInboundRouting::<Identity, Impl, OFFSET>,
            SetUseForInboundRouting: SetUseForInboundRouting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxReceiptOptions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxRecipient_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxRecipient {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>() -> IFaxRecipient_Vtbl {
        unsafe extern "system" fn FaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfaxnumber: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxRecipient_Impl::FaxNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfaxnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxRecipient_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxRecipient_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxRecipient_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxRecipient as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxRecipients_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<IFaxRecipient>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstrfaxnumber: &windows_core::BSTR, bstrrecipientname: &windows_core::BSTR) -> windows_core::Result<IFaxRecipient>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxRecipients {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipients_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>() -> IFaxRecipients_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxRecipients_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxRecipients_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxrecipient, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxRecipients_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: core::mem::MaybeUninit<windows_core::BSTR>, bstrrecipientname: core::mem::MaybeUninit<windows_core::BSTR>, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxRecipients_Impl::Add(this, core::mem::transmute(&bstrfaxnumber), core::mem::transmute(&bstrrecipientname)) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxrecipient, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxRecipients_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxRecipients as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSecurity_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDescriptor(&self, vdescriptor: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn GrantedRights(&self) -> windows_core::Result<FAX_ACCESS_RIGHTS_ENUM>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn InformationType(&self) -> windows_core::Result<i32>;
    fn SetInformationType(&self, linformationtype: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxSecurity {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>() -> IFaxSecurity_Vtbl {
        unsafe extern "system" fn Descriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdescriptor: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSecurity_Impl::Descriptor(this) {
                Ok(ok__) => {
                    core::ptr::write(pvdescriptor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdescriptor: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity_Impl::SetDescriptor(this, core::mem::transmute(&vdescriptor)).into()
        }
        unsafe extern "system" fn GrantedRights<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSecurity_Impl::GrantedRights(this) {
                Ok(ok__) => {
                    core::ptr::write(pgrantedrights, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity_Impl::Save(this).into()
        }
        unsafe extern "system" fn InformationType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plinformationtype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSecurity_Impl::InformationType(this) {
                Ok(ok__) => {
                    core::ptr::write(plinformationtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linformationtype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity_Impl::SetInformationType(this, core::mem::transmute_copy(&linformationtype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Descriptor: Descriptor::<Identity, Impl, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, Impl, OFFSET>,
            GrantedRights: GrantedRights::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            InformationType: InformationType::<Identity, Impl, OFFSET>,
            SetInformationType: SetInformationType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxSecurity as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSecurity2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDescriptor(&self, vdescriptor: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn GrantedRights(&self) -> windows_core::Result<FAX_ACCESS_RIGHTS_ENUM_2>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn InformationType(&self) -> windows_core::Result<i32>;
    fn SetInformationType(&self, linformationtype: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxSecurity2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>() -> IFaxSecurity2_Vtbl {
        unsafe extern "system" fn Descriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdescriptor: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSecurity2_Impl::Descriptor(this) {
                Ok(ok__) => {
                    core::ptr::write(pvdescriptor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdescriptor: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity2_Impl::SetDescriptor(this, core::mem::transmute(&vdescriptor)).into()
        }
        unsafe extern "system" fn GrantedRights<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSecurity2_Impl::GrantedRights(this) {
                Ok(ok__) => {
                    core::ptr::write(pgrantedrights, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity2_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity2_Impl::Save(this).into()
        }
        unsafe extern "system" fn InformationType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plinformationtype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSecurity2_Impl::InformationType(this) {
                Ok(ok__) => {
                    core::ptr::write(plinformationtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linformationtype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSecurity2_Impl::SetInformationType(this, core::mem::transmute_copy(&linformationtype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Descriptor: Descriptor::<Identity, Impl, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, Impl, OFFSET>,
            GrantedRights: GrantedRights::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            InformationType: InformationType::<Identity, Impl, OFFSET>,
            SetInformationType: SetInformationType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxSecurity2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSender_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BillingCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBillingCode(&self, bstrbillingcode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn City(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCity(&self, bstrcity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Company(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCompany(&self, bstrcompany: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Country(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCountry(&self, bstrcountry: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Email(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEmail(&self, bstremail: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HomePhone(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHomePhone(&self, bstrhomephone: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTSID(&self, bstrtsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OfficePhone(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOfficePhone(&self, bstrofficephone: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OfficeLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOfficeLocation(&self, bstrofficelocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetState(&self, bstrstate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StreetAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStreetAddress(&self, bstrstreetaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTitle(&self, bstrtitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ZipCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetZipCode(&self, bstrzipcode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoadDefaultSender(&self) -> windows_core::Result<()>;
    fn SaveDefaultSender(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxSender {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxSender_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>() -> IFaxSender_Vtbl {
        unsafe extern "system" fn BillingCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbillingcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::BillingCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrbillingcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBillingCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbillingcode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetBillingCode(this, core::mem::transmute(&bstrbillingcode)).into()
        }
        unsafe extern "system" fn City<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcity: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::City(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcity: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetCity(this, core::mem::transmute(&bstrcity)).into()
        }
        unsafe extern "system" fn Company<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcompany: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::Company(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcompany, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompany<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcompany: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetCompany(this, core::mem::transmute(&bstrcompany)).into()
        }
        unsafe extern "system" fn Country<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcountry: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::Country(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcountry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCountry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcountry: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetCountry(this, core::mem::transmute(&bstrcountry)).into()
        }
        unsafe extern "system" fn Department<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdepartment: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::Department(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdepartment, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetDepartment(this, core::mem::transmute(&bstrdepartment)).into()
        }
        unsafe extern "system" fn Email<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstremail: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::Email(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstremail, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremail: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetEmail(this, core::mem::transmute(&bstremail)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfaxnumber: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::FaxNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfaxnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn HomePhone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhomephone: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::HomePhone(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrhomephone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePhone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomephone: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetHomePhone(this, core::mem::transmute(&bstrhomephone)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn TSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::TSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetTSID(this, core::mem::transmute(&bstrtsid)).into()
        }
        unsafe extern "system" fn OfficePhone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrofficephone: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::OfficePhone(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrofficephone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficePhone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrofficephone: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetOfficePhone(this, core::mem::transmute(&bstrofficephone)).into()
        }
        unsafe extern "system" fn OfficeLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrofficelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::OfficeLocation(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrofficelocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrofficelocation: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetOfficeLocation(this, core::mem::transmute(&bstrofficelocation)).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstate: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetState(this, core::mem::transmute(&bstrstate)).into()
        }
        unsafe extern "system" fn StreetAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstreetaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::StreetAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrstreetaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreetAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstreetaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetStreetAddress(this, core::mem::transmute(&bstrstreetaddress)).into()
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::Title(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtitle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetTitle(this, core::mem::transmute(&bstrtitle)).into()
        }
        unsafe extern "system" fn ZipCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrzipcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxSender_Impl::ZipCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrzipcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZipCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrzipcode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SetZipCode(this, core::mem::transmute(&bstrzipcode)).into()
        }
        unsafe extern "system" fn LoadDefaultSender<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::LoadDefaultSender(this).into()
        }
        unsafe extern "system" fn SaveDefaultSender<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxSender_Impl::SaveDefaultSender(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BillingCode: BillingCode::<Identity, Impl, OFFSET>,
            SetBillingCode: SetBillingCode::<Identity, Impl, OFFSET>,
            City: City::<Identity, Impl, OFFSET>,
            SetCity: SetCity::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            SetCompany: SetCompany::<Identity, Impl, OFFSET>,
            Country: Country::<Identity, Impl, OFFSET>,
            SetCountry: SetCountry::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            Email: Email::<Identity, Impl, OFFSET>,
            SetEmail: SetEmail::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            HomePhone: HomePhone::<Identity, Impl, OFFSET>,
            SetHomePhone: SetHomePhone::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            SetTSID: SetTSID::<Identity, Impl, OFFSET>,
            OfficePhone: OfficePhone::<Identity, Impl, OFFSET>,
            SetOfficePhone: SetOfficePhone::<Identity, Impl, OFFSET>,
            OfficeLocation: OfficeLocation::<Identity, Impl, OFFSET>,
            SetOfficeLocation: SetOfficeLocation::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            StreetAddress: StreetAddress::<Identity, Impl, OFFSET>,
            SetStreetAddress: SetStreetAddress::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            ZipCode: ZipCode::<Identity, Impl, OFFSET>,
            SetZipCode: SetZipCode::<Identity, Impl, OFFSET>,
            LoadDefaultSender: LoadDefaultSender::<Identity, Impl, OFFSET>,
            SaveDefaultSender: SaveDefaultSender::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxSender as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Connect(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDeviceProviders(&self) -> windows_core::Result<IFaxDeviceProviders>;
    fn GetDevices(&self) -> windows_core::Result<IFaxDevices>;
    fn InboundRouting(&self) -> windows_core::Result<IFaxInboundRouting>;
    fn Folders(&self) -> windows_core::Result<IFaxFolders>;
    fn LoggingOptions(&self) -> windows_core::Result<IFaxLoggingOptions>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn MajorBuild(&self) -> windows_core::Result<i32>;
    fn MinorBuild(&self) -> windows_core::Result<i32>;
    fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Activity(&self) -> windows_core::Result<IFaxActivity>;
    fn OutboundRouting(&self) -> windows_core::Result<IFaxOutboundRouting>;
    fn ReceiptOptions(&self) -> windows_core::Result<IFaxReceiptOptions>;
    fn Security(&self) -> windows_core::Result<IFaxSecurity>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn GetExtensionProperty(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn SetExtensionProperty(&self, bstrguid: &windows_core::BSTR, vproperty: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::Result<()>;
    fn RegisterDeviceProvider(&self, bstrguid: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR, bstrimagename: &windows_core::BSTR, tspname: &windows_core::BSTR, lfspiversion: i32) -> windows_core::Result<()>;
    fn UnregisterDeviceProvider(&self, bstruniquename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterInboundRoutingExtension(&self, bstrextensionname: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR, bstrimagename: &windows_core::BSTR, vmethods: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn UnregisterInboundRoutingExtension(&self, bstrextensionuniquename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisteredEvents(&self) -> windows_core::Result<FAX_SERVER_EVENTS_TYPE_ENUM>;
    fn APIVersion(&self) -> windows_core::Result<FAX_SERVER_APIVERSION_ENUM>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxServer {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>() -> IFaxServer_Vtbl {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::Connect(this, core::mem::transmute(&bstrservername)).into()
        }
        unsafe extern "system" fn ServerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrservername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::ServerName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrservername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxdeviceproviders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::GetDeviceProviders(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxdeviceproviders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::GetDevices(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxdevices, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundRouting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxinboundrouting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::InboundRouting(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxinboundrouting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxfolders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::Folders(this) {
                Ok(ok__) => {
                    core::ptr::write(pfaxfolders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxloggingoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::LoggingOptions(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxloggingoptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::MajorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::MinorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorbuild: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::MajorBuild(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorbuild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorbuild: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::MinorBuild(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorbuild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::Debug(this) {
                Ok(ok__) => {
                    core::ptr::write(pbdebug, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::Activity(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxactivity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundRouting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxoutboundrouting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::OutboundRouting(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxoutboundrouting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxreceiptoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::ReceiptOptions(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxreceiptoptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::Security(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxsecurity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, pvproperty: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::GetExtensionProperty(this, core::mem::transmute(&bstrguid)) {
                Ok(ok__) => {
                    core::ptr::write(pvproperty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, vproperty: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::SetExtensionProperty(this, core::mem::transmute(&bstrguid), core::mem::transmute(&vproperty)).into()
        }
        unsafe extern "system" fn ListenToServerEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::ListenToServerEvents(this, core::mem::transmute_copy(&eventtypes)).into()
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, bstrfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>, bstrimagename: core::mem::MaybeUninit<windows_core::BSTR>, tspname: core::mem::MaybeUninit<windows_core::BSTR>, lfspiversion: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::RegisterDeviceProvider(this, core::mem::transmute(&bstrguid), core::mem::transmute(&bstrfriendlyname), core::mem::transmute(&bstrimagename), core::mem::transmute(&tspname), core::mem::transmute_copy(&lfspiversion)).into()
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruniquename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::UnregisterDeviceProvider(this, core::mem::transmute(&bstruniquename)).into()
        }
        unsafe extern "system" fn RegisterInboundRoutingExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrextensionname: core::mem::MaybeUninit<windows_core::BSTR>, bstrfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>, bstrimagename: core::mem::MaybeUninit<windows_core::BSTR>, vmethods: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::RegisterInboundRoutingExtension(this, core::mem::transmute(&bstrextensionname), core::mem::transmute(&bstrfriendlyname), core::mem::transmute(&bstrimagename), core::mem::transmute(&vmethods)).into()
        }
        unsafe extern "system" fn UnregisterInboundRoutingExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrextensionuniquename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServer_Impl::UnregisterInboundRoutingExtension(this, core::mem::transmute(&bstrextensionuniquename)).into()
        }
        unsafe extern "system" fn RegisteredEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::RegisteredEvents(this) {
                Ok(ok__) => {
                    core::ptr::write(peventtypes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn APIVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer_Impl::APIVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(papiversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            ServerName: ServerName::<Identity, Impl, OFFSET>,
            GetDeviceProviders: GetDeviceProviders::<Identity, Impl, OFFSET>,
            GetDevices: GetDevices::<Identity, Impl, OFFSET>,
            InboundRouting: InboundRouting::<Identity, Impl, OFFSET>,
            Folders: Folders::<Identity, Impl, OFFSET>,
            LoggingOptions: LoggingOptions::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Activity: Activity::<Identity, Impl, OFFSET>,
            OutboundRouting: OutboundRouting::<Identity, Impl, OFFSET>,
            ReceiptOptions: ReceiptOptions::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, Impl, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, Impl, OFFSET>,
            ListenToServerEvents: ListenToServerEvents::<Identity, Impl, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, Impl, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, Impl, OFFSET>,
            RegisterInboundRoutingExtension: RegisterInboundRoutingExtension::<Identity, Impl, OFFSET>,
            UnregisterInboundRoutingExtension: UnregisterInboundRoutingExtension::<Identity, Impl, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, Impl, OFFSET>,
            APIVersion: APIVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServer2_Impl: Sized + IFaxServer_Impl {
    fn Configuration(&self) -> windows_core::Result<IFaxConfiguration>;
    fn CurrentAccount(&self) -> windows_core::Result<IFaxAccount>;
    fn FaxAccountSet(&self) -> windows_core::Result<IFaxAccountSet>;
    fn Security2(&self) -> windows_core::Result<IFaxSecurity2>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxServer2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>() -> IFaxServer2_Vtbl {
        unsafe extern "system" fn Configuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer2_Impl::Configuration(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxconfiguration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcurrentaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer2_Impl::CurrentAccount(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcurrentaccount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaxAccountSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxaccountset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer2_Impl::FaxAccountSet(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxaccountset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsecurity2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFaxServer2_Impl::Security2(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfaxsecurity2, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IFaxServer_Vtbl::new::<Identity, Impl, OFFSET>(),
            Configuration: Configuration::<Identity, Impl, OFFSET>,
            CurrentAccount: CurrentAccount::<Identity, Impl, OFFSET>,
            FaxAccountSet: FaxAccountSet::<Identity, Impl, OFFSET>,
            Security2: Security2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServer2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxServer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerNotify_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxServerNotify {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify_Impl, const OFFSET: isize>() -> IFaxServerNotify_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServerNotify as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerNotify2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&self, pfaxserver: Option<&IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobRemoved(&self, pfaxserver: Option<&IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobChanged(&self, pfaxserver: Option<&IFaxServer2>, bstrjobid: &windows_core::BSTR, pjobstatus: Option<&IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnOutgoingJobAdded(&self, pfaxserver: Option<&IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobRemoved(&self, pfaxserver: Option<&IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobChanged(&self, pfaxserver: Option<&IFaxServer2>, bstrjobid: &windows_core::BSTR, pjobstatus: Option<&IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnIncomingMessageAdded(&self, pfaxserver: Option<&IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingMessageRemoved(&self, pfaxserver: Option<&IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingMessageAdded(&self, pfaxserver: Option<&IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingMessageRemoved(&self, pfaxserver: Option<&IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnReceiptOptionsChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnActivityLoggingConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnSecurityConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnEventLoggingConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutgoingQueueConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutgoingArchiveConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnIncomingArchiveConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnDevicesConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutboundRoutingGroupsConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutboundRoutingRulesConfigChange(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnServerActivityChange(&self, pfaxserver: Option<&IFaxServer2>, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> windows_core::Result<()>;
    fn OnQueuesStatusChange(&self, pfaxserver: Option<&IFaxServer2>, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnNewCall(&self, pfaxserver: Option<&IFaxServer2>, lcallid: i32, ldeviceid: i32, bstrcallerid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnServerShutDown(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
    fn OnDeviceStatusChange(&self, pfaxserver: Option<&IFaxServer2>, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnGeneralServerConfigChanged(&self, pfaxserver: Option<&IFaxServer2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFaxServerNotify2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>() -> IFaxServerNotify2_Vtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnIncomingJobAdded(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnIncomingJobRemoved(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnIncomingJobChanged(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrjobid), windows_core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutgoingJobAdded(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutgoingJobRemoved(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: core::mem::MaybeUninit<windows_core::BSTR>, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutgoingJobChanged(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrjobid), windows_core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnIncomingMessageAdded(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnIncomingMessageRemoved(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutgoingMessageAdded(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutgoingMessageRemoved(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnReceiptOptionsChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnReceiptOptionsChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnActivityLoggingConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnActivityLoggingConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnSecurityConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnSecurityConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnEventLoggingConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnEventLoggingConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutgoingQueueConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutgoingQueueConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutgoingArchiveConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutgoingArchiveConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnIncomingArchiveConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnIncomingArchiveConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnDevicesConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnDevicesConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutboundRoutingGroupsConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutboundRoutingGroupsConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutboundRoutingRulesConfigChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnOutboundRoutingRulesConfigChange(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnServerActivityChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnServerActivityChange(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute_copy(&lincomingmessages), core::mem::transmute_copy(&lroutingmessages), core::mem::transmute_copy(&loutgoingmessages), core::mem::transmute_copy(&lqueuedmessages)).into()
        }
        unsafe extern "system" fn OnQueuesStatusChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnQueuesStatusChange(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute_copy(&boutgoingqueueblocked), core::mem::transmute_copy(&boutgoingqueuepaused), core::mem::transmute_copy(&bincomingqueueblocked)).into()
        }
        unsafe extern "system" fn OnNewCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, lcallid: i32, ldeviceid: i32, bstrcallerid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnNewCall(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute_copy(&lcallid), core::mem::transmute_copy(&ldeviceid), core::mem::transmute(&bstrcallerid)).into()
        }
        unsafe extern "system" fn OnServerShutDown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnServerShutDown(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnDeviceStatusChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnDeviceStatusChange(this, windows_core::from_raw_borrowed(&pfaxserver), core::mem::transmute_copy(&ldeviceid), core::mem::transmute_copy(&bpoweredoff), core::mem::transmute_copy(&bsending), core::mem::transmute_copy(&breceiving), core::mem::transmute_copy(&bringing)).into()
        }
        unsafe extern "system" fn OnGeneralServerConfigChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFaxServerNotify2_Impl::OnGeneralServerConfigChanged(this, windows_core::from_raw_borrowed(&pfaxserver)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, Impl, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, Impl, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, Impl, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, Impl, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, Impl, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, Impl, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, Impl, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, Impl, OFFSET>,
            OnReceiptOptionsChange: OnReceiptOptionsChange::<Identity, Impl, OFFSET>,
            OnActivityLoggingConfigChange: OnActivityLoggingConfigChange::<Identity, Impl, OFFSET>,
            OnSecurityConfigChange: OnSecurityConfigChange::<Identity, Impl, OFFSET>,
            OnEventLoggingConfigChange: OnEventLoggingConfigChange::<Identity, Impl, OFFSET>,
            OnOutgoingQueueConfigChange: OnOutgoingQueueConfigChange::<Identity, Impl, OFFSET>,
            OnOutgoingArchiveConfigChange: OnOutgoingArchiveConfigChange::<Identity, Impl, OFFSET>,
            OnIncomingArchiveConfigChange: OnIncomingArchiveConfigChange::<Identity, Impl, OFFSET>,
            OnDevicesConfigChange: OnDevicesConfigChange::<Identity, Impl, OFFSET>,
            OnOutboundRoutingGroupsConfigChange: OnOutboundRoutingGroupsConfigChange::<Identity, Impl, OFFSET>,
            OnOutboundRoutingRulesConfigChange: OnOutboundRoutingRulesConfigChange::<Identity, Impl, OFFSET>,
            OnServerActivityChange: OnServerActivityChange::<Identity, Impl, OFFSET>,
            OnQueuesStatusChange: OnQueuesStatusChange::<Identity, Impl, OFFSET>,
            OnNewCall: OnNewCall::<Identity, Impl, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, Impl, OFFSET>,
            OnDeviceStatusChange: OnDeviceStatusChange::<Identity, Impl, OFFSET>,
            OnGeneralServerConfigChanged: OnGeneralServerConfigChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServerNotify2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_IO")]
pub trait IStiDevice_Impl: Sized {
    fn Initialize(&self, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: &windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> windows_core::Result<()>;
    fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::Result<()>;
    fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::Result<()>;
    fn DeviceReset(&self) -> windows_core::Result<()>;
    fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::Result<()>;
    fn Escape(&self, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()>;
    fn GetLastError(&self) -> windows_core::Result<u32>;
    fn LockDevice(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn UnLockDevice(&self) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::Result<()>;
    fn GetLastNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::Result<()>;
    fn UnSubscribe(&self) -> windows_core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::RuntimeName for IStiDevice {}
#[cfg(feature = "Win32_System_IO")]
impl IStiDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>() -> IStiDevice_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwversion), core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::GetCapabilities(this, core::mem::transmute_copy(&pdevcaps)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::GetStatus(this, core::mem::transmute_copy(&pdevstatus)).into()
        }
        unsafe extern "system" fn DeviceReset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::DeviceReset(this).into()
        }
        unsafe extern "system" fn Diagnostic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut STI_DIAG) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::Diagnostic(this, core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::Escape(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStiDevice_Impl::GetLastError(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwlastdeviceerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::LockDevice(this, core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn UnLockDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::UnLockDevice(this).into()
        }
        unsafe extern "system" fn RawReadData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn Subscribe<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::Subscribe(this, core::mem::transmute_copy(&lpsubsribe)).into()
        }
        unsafe extern "system" fn GetLastNotificationData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotify: *mut STINOTIFY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::GetLastNotificationData(this, core::mem::transmute_copy(&lpnotify)).into()
        }
        unsafe extern "system" fn UnSubscribe<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::UnSubscribe(this).into()
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDevice_Impl::GetLastErrorInfo(this, core::mem::transmute_copy(&plasterrorinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            DeviceReset: DeviceReset::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            LockDevice: LockDevice::<Identity, Impl, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            Subscribe: Subscribe::<Identity, Impl, OFFSET>,
            GetLastNotificationData: GetLastNotificationData::<Identity, Impl, OFFSET>,
            UnSubscribe: UnSubscribe::<Identity, Impl, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStiDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_IO")]
pub trait IStiDeviceControl_Impl: Sized {
    fn Initialize(&self, dwdevicetype: u32, dwmode: u32, pwszportname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawDeviceControl(&self, escapefunction: u32, lpindata: *mut core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()>;
    fn GetLastError(&self, lpdwlasterror: *mut u32) -> windows_core::Result<()>;
    fn GetMyDevicePortName(&self, lpszdevicepath: windows_core::PWSTR, cwdevicepathsize: u32) -> windows_core::Result<()>;
    fn GetMyDeviceHandle(&self, lph: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetMyDeviceOpenMode(&self, pdwopenmode: *mut u32) -> windows_core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: u32, pszmessage: &windows_core::PCWSTR, dwerrorcode: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::RuntimeName for IStiDeviceControl {}
#[cfg(feature = "Win32_System_IO")]
impl IStiDeviceControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>() -> IStiDeviceControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::Initialize(this, core::mem::transmute_copy(&dwdevicetype), core::mem::transmute_copy(&dwmode), core::mem::transmute(&pwszportname), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RawReadData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawDeviceControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: u32, lpindata: *mut core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::RawDeviceControl(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwlasterror: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::GetLastError(this, core::mem::transmute_copy(&lpdwlasterror)).into()
        }
        unsafe extern "system" fn GetMyDevicePortName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszdevicepath: windows_core::PWSTR, cwdevicepathsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::GetMyDevicePortName(this, core::mem::transmute_copy(&lpszdevicepath), core::mem::transmute_copy(&cwdevicepathsize)).into()
        }
        unsafe extern "system" fn GetMyDeviceHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::GetMyDeviceHandle(this, core::mem::transmute_copy(&lph)).into()
        }
        unsafe extern "system" fn GetMyDeviceOpenMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwopenmode: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::GetMyDeviceOpenMode(this, core::mem::transmute_copy(&pdwopenmode)).into()
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: u32, pszmessage: windows_core::PCWSTR, dwerrorcode: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiDeviceControl_Impl::WriteToErrorLog(this, core::mem::transmute_copy(&dwmessagetype), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&dwerrorcode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            RawDeviceControl: RawDeviceControl::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GetMyDevicePortName: GetMyDevicePortName::<Identity, Impl, OFFSET>,
            GetMyDeviceHandle: GetMyDeviceHandle::<Identity, Impl, OFFSET>,
            GetMyDeviceOpenMode: GetMyDeviceOpenMode::<Identity, Impl, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStiDeviceControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
pub trait IStiUSD_Impl: Sized {
    fn Initialize(&self, pheldcb: Option<&IStiDeviceControl>, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn GetCapabilities(&self) -> windows_core::Result<STI_USD_CAPS>;
    fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::Result<()>;
    fn DeviceReset(&self) -> windows_core::Result<()>;
    fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::Result<()>;
    fn Escape(&self, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()>;
    fn GetLastError(&self) -> windows_core::Result<u32>;
    fn LockDevice(&self) -> windows_core::Result<()>;
    fn UnLockDevice(&self) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn SetNotificationHandle(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl windows_core::RuntimeName for IStiUSD {}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl IStiUSD_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>() -> IStiUSD_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheldcb: *mut core::ffi::c_void, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::Initialize(this, windows_core::from_raw_borrowed(&pheldcb), core::mem::transmute_copy(&dwstiversion), core::mem::transmute_copy(&hparameterskey)).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStiUSD_Impl::GetCapabilities(this) {
                Ok(ok__) => {
                    core::ptr::write(pdevcaps, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::GetStatus(this, core::mem::transmute_copy(&pdevstatus)).into()
        }
        unsafe extern "system" fn DeviceReset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::DeviceReset(this).into()
        }
        unsafe extern "system" fn Diagnostic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut STI_DIAG) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::Diagnostic(this, core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::Escape(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&cboutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStiUSD_Impl::GetLastError(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwlastdeviceerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::LockDevice(this).into()
        }
        unsafe extern "system" fn UnLockDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::UnLockDevice(this).into()
        }
        unsafe extern "system" fn RawReadData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn SetNotificationHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::SetNotificationHandle(this, core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetNotificationData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotify: *mut STINOTIFY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::GetNotificationData(this, core::mem::transmute_copy(&lpnotify)).into()
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStiUSD_Impl::GetLastErrorInfo(this, core::mem::transmute_copy(&plasterrorinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            DeviceReset: DeviceReset::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            LockDevice: LockDevice::<Identity, Impl, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            SetNotificationHandle: SetNotificationHandle::<Identity, Impl, OFFSET>,
            GetNotificationData: GetNotificationData::<Identity, Impl, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStiUSD as windows_core::Interface>::IID
    }
}
pub trait IStillImageW_Impl: Sized {
    fn Initialize(&self, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> windows_core::Result<()>;
    fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, pwszdevicename: &windows_core::PCWSTR, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateDevice(&self, pwszdevicename: &windows_core::PCWSTR, dwmode: u32, pdevice: *mut Option<IStiDevice>, punkouter: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDeviceValue(&self, pwszdevicename: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> windows_core::Result<()>;
    fn SetDeviceValue(&self, pwszdevicename: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn GetSTILaunchInformation(&self, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RegisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR, pwszcommandline: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnableHwNotifications(&self, pwszdevicename: &windows_core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetHwNotificationState(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn RefreshDeviceBus(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LaunchApplicationForDevice(&self, pwszdevicename: &windows_core::PCWSTR, pwszappname: &windows_core::PCWSTR, pstinotify: *const STINOTIFY) -> windows_core::Result<()>;
    fn SetupDeviceParameters(&self, param0: *mut STI_DEVICE_INFORMATIONW) -> windows_core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: u32, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStillImageW {}
impl IStillImageW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>() -> IStillImageW_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute_copy(&dwversion)).into()
        }
        unsafe extern "system" fn GetDeviceList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::GetDeviceList(this, core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwitemsreturned), core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::GetDeviceInfo(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, dwmode: u32, pdevice: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::CreateDevice(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdevice), windows_core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn GetDeviceValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::GetDeviceValue(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn SetDeviceValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::SetDeviceValue(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn GetSTILaunchInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::GetSTILaunchInformation(this, core::mem::transmute_copy(&pwszdevicename), core::mem::transmute_copy(&pdweventcode), core::mem::transmute_copy(&pwszeventname)).into()
        }
        unsafe extern "system" fn RegisterLaunchApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszappname: windows_core::PCWSTR, pwszcommandline: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::RegisterLaunchApplication(this, core::mem::transmute(&pwszappname), core::mem::transmute(&pwszcommandline)).into()
        }
        unsafe extern "system" fn UnregisterLaunchApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszappname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::UnregisterLaunchApplication(this, core::mem::transmute(&pwszappname)).into()
        }
        unsafe extern "system" fn EnableHwNotifications<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::EnableHwNotifications(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&bnewstate)).into()
        }
        unsafe extern "system" fn GetHwNotificationState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStillImageW_Impl::GetHwNotificationState(this, core::mem::transmute(&pwszdevicename)) {
                Ok(ok__) => {
                    core::ptr::write(pbcurrentstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshDeviceBus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::RefreshDeviceBus(this, core::mem::transmute(&pwszdevicename)).into()
        }
        unsafe extern "system" fn LaunchApplicationForDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pwszappname: windows_core::PCWSTR, pstinotify: *const STINOTIFY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::LaunchApplicationForDevice(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pwszappname), core::mem::transmute_copy(&pstinotify)).into()
        }
        unsafe extern "system" fn SetupDeviceParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::SetupDeviceParameters(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: u32, pszmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStillImageW_Impl::WriteToErrorLog(this, core::mem::transmute_copy(&dwmessagetype), core::mem::transmute(&pszmessage)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetDeviceList: GetDeviceList::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            GetDeviceValue: GetDeviceValue::<Identity, Impl, OFFSET>,
            SetDeviceValue: SetDeviceValue::<Identity, Impl, OFFSET>,
            GetSTILaunchInformation: GetSTILaunchInformation::<Identity, Impl, OFFSET>,
            RegisterLaunchApplication: RegisterLaunchApplication::<Identity, Impl, OFFSET>,
            UnregisterLaunchApplication: UnregisterLaunchApplication::<Identity, Impl, OFFSET>,
            EnableHwNotifications: EnableHwNotifications::<Identity, Impl, OFFSET>,
            GetHwNotificationState: GetHwNotificationState::<Identity, Impl, OFFSET>,
            RefreshDeviceBus: RefreshDeviceBus::<Identity, Impl, OFFSET>,
            LaunchApplicationForDevice: LaunchApplicationForDevice::<Identity, Impl, OFFSET>,
            SetupDeviceParameters: SetupDeviceParameters::<Identity, Impl, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStillImageW as windows_core::Interface>::IID
    }
}
