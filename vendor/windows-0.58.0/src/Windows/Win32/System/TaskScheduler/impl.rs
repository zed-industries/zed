#[cfg(feature = "Win32_System_Com")]
pub trait IAction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAction {}
#[cfg(feature = "Win32_System_Com")]
impl IAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAction_Vtbl
    where
        Identity: IAction_Impl,
    {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAction_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAction_Impl::SetId(this, core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> windows_core::HRESULT
        where
            Identity: IAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAction_Impl::Type(this, core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IAction>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Create(&self, r#type: TASK_ACTION_TYPE) -> windows_core::Result<IAction>;
    fn Remove(&self, index: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Context(&self, pcontext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetContext(&self, context: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IActionCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActionCollection_Vtbl
    where
        Identity: IActionCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionCollection_Impl::Count(this, core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActionCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionCollection_Impl::XmlText(this, core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionCollection_Impl::SetXmlText(this, core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActionCollection_Impl::Create(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    ppaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn Context<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionCollection_Impl::Context(this, core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionCollection_Impl::SetContext(this, core::mem::transmute(&context)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActionCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBootTrigger_Impl: Sized + ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBootTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IBootTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBootTrigger_Vtbl
    where
        Identity: IBootTrigger_Impl,
    {
        unsafe extern "system" fn Delay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IBootTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBootTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IBootTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBootTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>(), Delay: Delay::<Identity, OFFSET>, SetDelay: SetDelay::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBootTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComHandlerAction_Impl: Sized + IAction_Impl {
    fn ClassId(&self, pclsid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClassId(&self, clsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetData(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IComHandlerAction {}
#[cfg(feature = "Win32_System_Com")]
impl IComHandlerAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComHandlerAction_Vtbl
    where
        Identity: IComHandlerAction_Impl,
    {
        unsafe extern "system" fn ClassId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IComHandlerAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComHandlerAction_Impl::ClassId(this, core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetClassId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IComHandlerAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComHandlerAction_Impl::SetClassId(this, core::mem::transmute(&clsid)).into()
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IComHandlerAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComHandlerAction_Impl::Data(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IComHandlerAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComHandlerAction_Impl::SetData(this, core::mem::transmute(&data)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            ClassId: ClassId::<Identity, OFFSET>,
            SetClassId: SetClassId::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComHandlerAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDailyTrigger_Impl: Sized + ITrigger_Impl {
    fn DaysInterval(&self, pdays: *mut i16) -> windows_core::Result<()>;
    fn SetDaysInterval(&self, days: i16) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDailyTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IDailyTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDailyTrigger_Vtbl
    where
        Identity: IDailyTrigger_Impl,
    {
        unsafe extern "system" fn DaysInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT
        where
            Identity: IDailyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDailyTrigger_Impl::DaysInterval(this, core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT
        where
            Identity: IDailyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDailyTrigger_Impl::SetDaysInterval(this, core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDailyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDailyTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDailyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDailyTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysInterval: DaysInterval::<Identity, OFFSET>,
            SetDaysInterval: SetDaysInterval::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDailyTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEmailAction_Impl: Sized + IAction_Impl {
    fn Server(&self, pserver: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetServer(&self, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Subject(&self, psubject: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSubject(&self, subject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn To(&self, pto: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTo(&self, to: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Cc(&self, pcc: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetCc(&self, cc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Bcc(&self, pbcc: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBcc(&self, bcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReplyTo(&self, preplyto: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetReplyTo(&self, replyto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn From(&self, pfrom: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetFrom(&self, from: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HeaderFields(&self) -> windows_core::Result<ITaskNamedValueCollection>;
    fn SetHeaderFields(&self, pheaderfields: Option<&ITaskNamedValueCollection>) -> windows_core::Result<()>;
    fn Body(&self, pbody: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBody(&self, body: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Attachments(&self, pattachements: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn SetAttachments(&self, pattachements: *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEmailAction {}
#[cfg(feature = "Win32_System_Com")]
impl IEmailAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEmailAction_Vtbl
    where
        Identity: IEmailAction_Impl,
    {
        unsafe extern "system" fn Server<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::Server(this, core::mem::transmute_copy(&pserver)).into()
        }
        unsafe extern "system" fn SetServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetServer(this, core::mem::transmute(&server)).into()
        }
        unsafe extern "system" fn Subject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubject: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::Subject(this, core::mem::transmute_copy(&psubject)).into()
        }
        unsafe extern "system" fn SetSubject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetSubject(this, core::mem::transmute(&subject)).into()
        }
        unsafe extern "system" fn To<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pto: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::To(this, core::mem::transmute_copy(&pto)).into()
        }
        unsafe extern "system" fn SetTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, to: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetTo(this, core::mem::transmute(&to)).into()
        }
        unsafe extern "system" fn Cc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::Cc(this, core::mem::transmute_copy(&pcc)).into()
        }
        unsafe extern "system" fn SetCc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cc: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetCc(this, core::mem::transmute(&cc)).into()
        }
        unsafe extern "system" fn Bcc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::Bcc(this, core::mem::transmute_copy(&pbcc)).into()
        }
        unsafe extern "system" fn SetBcc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcc: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetBcc(this, core::mem::transmute(&bcc)).into()
        }
        unsafe extern "system" fn ReplyTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplyto: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::ReplyTo(this, core::mem::transmute_copy(&preplyto)).into()
        }
        unsafe extern "system" fn SetReplyTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, replyto: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetReplyTo(this, core::mem::transmute(&replyto)).into()
        }
        unsafe extern "system" fn From<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrom: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::From(this, core::mem::transmute_copy(&pfrom)).into()
        }
        unsafe extern "system" fn SetFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, from: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetFrom(this, core::mem::transmute(&from)).into()
        }
        unsafe extern "system" fn HeaderFields<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppheaderfields: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEmailAction_Impl::HeaderFields(this) {
                Ok(ok__) => {
                    ppheaderfields.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderFields<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheaderfields: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetHeaderFields(this, windows_core::from_raw_borrowed(&pheaderfields)).into()
        }
        unsafe extern "system" fn Body<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::Body(this, core::mem::transmute_copy(&pbody)).into()
        }
        unsafe extern "system" fn SetBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, body: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetBody(this, core::mem::transmute(&body)).into()
        }
        unsafe extern "system" fn Attachments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::Attachments(this, core::mem::transmute_copy(&pattachements)).into()
        }
        unsafe extern "system" fn SetAttachments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IEmailAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmailAction_Impl::SetAttachments(this, core::mem::transmute_copy(&pattachements)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            Server: Server::<Identity, OFFSET>,
            SetServer: SetServer::<Identity, OFFSET>,
            Subject: Subject::<Identity, OFFSET>,
            SetSubject: SetSubject::<Identity, OFFSET>,
            To: To::<Identity, OFFSET>,
            SetTo: SetTo::<Identity, OFFSET>,
            Cc: Cc::<Identity, OFFSET>,
            SetCc: SetCc::<Identity, OFFSET>,
            Bcc: Bcc::<Identity, OFFSET>,
            SetBcc: SetBcc::<Identity, OFFSET>,
            ReplyTo: ReplyTo::<Identity, OFFSET>,
            SetReplyTo: SetReplyTo::<Identity, OFFSET>,
            From: From::<Identity, OFFSET>,
            SetFrom: SetFrom::<Identity, OFFSET>,
            HeaderFields: HeaderFields::<Identity, OFFSET>,
            SetHeaderFields: SetHeaderFields::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            Attachments: Attachments::<Identity, OFFSET>,
            SetAttachments: SetAttachments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEmailAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
pub trait IEnumWorkItems_Impl: Sized {
    fn Next(&self, celt: u32, rgpwsznames: *mut *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWorkItems>;
}
impl windows_core::RuntimeName for IEnumWorkItems {}
impl IEnumWorkItems_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumWorkItems_Vtbl
    where
        Identity: IEnumWorkItems_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWorkItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWorkItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgpwsznames), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumWorkItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWorkItems_Impl::Skip(this, core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWorkItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWorkItems_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumworkitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWorkItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWorkItems_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenumworkitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
        iid == &<IEnumWorkItems as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEventTrigger_Impl: Sized + ITrigger_Impl {
    fn Subscription(&self, pquery: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSubscription(&self, query: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ValueQueries(&self) -> windows_core::Result<ITaskNamedValueCollection>;
    fn SetValueQueries(&self, pnamedxpaths: Option<&ITaskNamedValueCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEventTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IEventTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventTrigger_Vtbl
    where
        Identity: IEventTrigger_Impl,
    {
        unsafe extern "system" fn Subscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquery: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventTrigger_Impl::Subscription(this, core::mem::transmute_copy(&pquery)).into()
        }
        unsafe extern "system" fn SetSubscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventTrigger_Impl::SetSubscription(this, core::mem::transmute(&query)).into()
        }
        unsafe extern "system" fn Delay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn ValueQueries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnamedxpaths: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventTrigger_Impl::ValueQueries(this) {
                Ok(ok__) => {
                    ppnamedxpaths.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueQueries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamedxpaths: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventTrigger_Impl::SetValueQueries(this, windows_core::from_raw_borrowed(&pnamedxpaths)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            Subscription: Subscription::<Identity, OFFSET>,
            SetSubscription: SetSubscription::<Identity, OFFSET>,
            Delay: Delay::<Identity, OFFSET>,
            SetDelay: SetDelay::<Identity, OFFSET>,
            ValueQueries: ValueQueries::<Identity, OFFSET>,
            SetValueQueries: SetValueQueries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExecAction_Impl: Sized + IAction_Impl {
    fn Path(&self, ppath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPath(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Arguments(&self, pargument: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetArguments(&self, argument: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WorkingDirectory(&self, pworkingdirectory: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetWorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IExecAction {}
#[cfg(feature = "Win32_System_Com")]
impl IExecAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IExecAction_Vtbl
    where
        Identity: IExecAction_Impl,
    {
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IExecAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction_Impl::Path(this, core::mem::transmute_copy(&ppath)).into()
        }
        unsafe extern "system" fn SetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IExecAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction_Impl::SetPath(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn Arguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pargument: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IExecAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction_Impl::Arguments(this, core::mem::transmute_copy(&pargument)).into()
        }
        unsafe extern "system" fn SetArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, argument: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IExecAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction_Impl::SetArguments(this, core::mem::transmute(&argument)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pworkingdirectory: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IExecAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction_Impl::WorkingDirectory(this, core::mem::transmute_copy(&pworkingdirectory)).into()
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IExecAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction_Impl::SetWorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            Arguments: Arguments::<Identity, OFFSET>,
            SetArguments: SetArguments::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExecAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExecAction2_Impl: Sized + IExecAction_Impl {
    fn HideAppWindow(&self, phideappwindow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetHideAppWindow(&self, hideappwindow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IExecAction2 {}
#[cfg(feature = "Win32_System_Com")]
impl IExecAction2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IExecAction2_Vtbl
    where
        Identity: IExecAction2_Impl,
    {
        unsafe extern "system" fn HideAppWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phideappwindow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IExecAction2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction2_Impl::HideAppWindow(this, core::mem::transmute_copy(&phideappwindow)).into()
        }
        unsafe extern "system" fn SetHideAppWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hideappwindow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IExecAction2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExecAction2_Impl::SetHideAppWindow(this, core::mem::transmute_copy(&hideappwindow)).into()
        }
        Self {
            base__: IExecAction_Vtbl::new::<Identity, OFFSET>(),
            HideAppWindow: HideAppWindow::<Identity, OFFSET>,
            SetHideAppWindow: SetHideAppWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExecAction2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID || iid == &<IExecAction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIdleSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn IdleDuration(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetIdleDuration(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WaitTimeout(&self, ptimeout: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetWaitTimeout(&self, timeout: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopOnIdleEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStopOnIdleEnd(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RestartOnIdle(&self, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRestartOnIdle(&self, restart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IIdleSettings {}
#[cfg(feature = "Win32_System_Com")]
impl IIdleSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIdleSettings_Vtbl
    where
        Identity: IIdleSettings_Impl,
    {
        unsafe extern "system" fn IdleDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::IdleDuration(this, core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetIdleDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::SetIdleDuration(this, core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn WaitTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::WaitTimeout(this, core::mem::transmute_copy(&ptimeout)).into()
        }
        unsafe extern "system" fn SetWaitTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::SetWaitTimeout(this, core::mem::transmute(&timeout)).into()
        }
        unsafe extern "system" fn StopOnIdleEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::StopOnIdleEnd(this, core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopOnIdleEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::SetStopOnIdleEnd(this, core::mem::transmute_copy(&stop)).into()
        }
        unsafe extern "system" fn RestartOnIdle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::RestartOnIdle(this, core::mem::transmute_copy(&prestart)).into()
        }
        unsafe extern "system" fn SetRestartOnIdle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IIdleSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdleSettings_Impl::SetRestartOnIdle(this, core::mem::transmute_copy(&restart)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IdleDuration: IdleDuration::<Identity, OFFSET>,
            SetIdleDuration: SetIdleDuration::<Identity, OFFSET>,
            WaitTimeout: WaitTimeout::<Identity, OFFSET>,
            SetWaitTimeout: SetWaitTimeout::<Identity, OFFSET>,
            StopOnIdleEnd: StopOnIdleEnd::<Identity, OFFSET>,
            SetStopOnIdleEnd: SetStopOnIdleEnd::<Identity, OFFSET>,
            RestartOnIdle: RestartOnIdle::<Identity, OFFSET>,
            SetRestartOnIdle: SetRestartOnIdle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdleSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIdleTrigger_Impl: Sized + ITrigger_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IIdleTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IIdleTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIdleTrigger_Vtbl
    where
        Identity: IIdleTrigger_Impl,
    {
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdleTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILogonTrigger_Impl: Sized + ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ILogonTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ILogonTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILogonTrigger_Vtbl
    where
        Identity: ILogonTrigger_Impl,
    {
        unsafe extern "system" fn Delay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILogonTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILogonTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILogonTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILogonTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn UserId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILogonTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILogonTrigger_Impl::UserId(this, core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILogonTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILogonTrigger_Impl::SetUserId(this, core::mem::transmute(&user)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            Delay: Delay::<Identity, OFFSET>,
            SetDelay: SetDelay::<Identity, OFFSET>,
            UserId: UserId::<Identity, OFFSET>,
            SetUserId: SetUserId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILogonTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMaintenanceSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetPeriod(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Period(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDeadline(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Deadline(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetExclusive(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Exclusive(&self, target: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMaintenanceSettings {}
#[cfg(feature = "Win32_System_Com")]
impl IMaintenanceSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMaintenanceSettings_Vtbl
    where
        Identity: IMaintenanceSettings_Impl,
    {
        unsafe extern "system" fn SetPeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMaintenanceSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMaintenanceSettings_Impl::SetPeriod(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Period<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMaintenanceSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMaintenanceSettings_Impl::Period(this, core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetDeadline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMaintenanceSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMaintenanceSettings_Impl::SetDeadline(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Deadline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMaintenanceSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMaintenanceSettings_Impl::Deadline(this, core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetExclusive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMaintenanceSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMaintenanceSettings_Impl::SetExclusive(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Exclusive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMaintenanceSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMaintenanceSettings_Impl::Exclusive(this, core::mem::transmute_copy(&target)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetPeriod: SetPeriod::<Identity, OFFSET>,
            Period: Period::<Identity, OFFSET>,
            SetDeadline: SetDeadline::<Identity, OFFSET>,
            Deadline: Deadline::<Identity, OFFSET>,
            SetExclusive: SetExclusive::<Identity, OFFSET>,
            Exclusive: Exclusive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMaintenanceSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMonthlyDOWTrigger_Impl: Sized + ITrigger_Impl {
    fn DaysOfWeek(&self, pdays: *mut i16) -> windows_core::Result<()>;
    fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()>;
    fn WeeksOfMonth(&self, pweeks: *mut i16) -> windows_core::Result<()>;
    fn SetWeeksOfMonth(&self, weeks: i16) -> windows_core::Result<()>;
    fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()>;
    fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()>;
    fn RunOnLastWeekOfMonth(&self, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnLastWeekOfMonth(&self, lastweek: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMonthlyDOWTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IMonthlyDOWTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMonthlyDOWTrigger_Vtbl
    where
        Identity: IMonthlyDOWTrigger_Impl,
    {
        unsafe extern "system" fn DaysOfWeek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::DaysOfWeek(this, core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::SetDaysOfWeek(this, core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweeks: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::WeeksOfMonth(this, core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, weeks: i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::SetWeeksOfMonth(this, core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmonths: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::MonthsOfYear(this, core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, months: i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::SetMonthsOfYear(this, core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastWeekOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::RunOnLastWeekOfMonth(this, core::mem::transmute_copy(&plastweek)).into()
        }
        unsafe extern "system" fn SetRunOnLastWeekOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastweek: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::SetRunOnLastWeekOfMonth(this, core::mem::transmute_copy(&lastweek)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMonthlyDOWTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyDOWTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Identity, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, OFFSET>,
            WeeksOfMonth: WeeksOfMonth::<Identity, OFFSET>,
            SetWeeksOfMonth: SetWeeksOfMonth::<Identity, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, OFFSET>,
            RunOnLastWeekOfMonth: RunOnLastWeekOfMonth::<Identity, OFFSET>,
            SetRunOnLastWeekOfMonth: SetRunOnLastWeekOfMonth::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonthlyDOWTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMonthlyTrigger_Impl: Sized + ITrigger_Impl {
    fn DaysOfMonth(&self, pdays: *mut i32) -> windows_core::Result<()>;
    fn SetDaysOfMonth(&self, days: i32) -> windows_core::Result<()>;
    fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()>;
    fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()>;
    fn RunOnLastDayOfMonth(&self, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnLastDayOfMonth(&self, lastday: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMonthlyTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IMonthlyTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMonthlyTrigger_Vtbl
    where
        Identity: IMonthlyTrigger_Impl,
    {
        unsafe extern "system" fn DaysOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::DaysOfMonth(this, core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::SetDaysOfMonth(this, core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmonths: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::MonthsOfYear(this, core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, months: i16) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::SetMonthsOfYear(this, core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastDayOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::RunOnLastDayOfMonth(this, core::mem::transmute_copy(&plastday)).into()
        }
        unsafe extern "system" fn SetRunOnLastDayOfMonth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastday: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::SetRunOnLastDayOfMonth(this, core::mem::transmute_copy(&lastday)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMonthlyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonthlyTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysOfMonth: DaysOfMonth::<Identity, OFFSET>,
            SetDaysOfMonth: SetDaysOfMonth::<Identity, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, OFFSET>,
            RunOnLastDayOfMonth: RunOnLastDayOfMonth::<Identity, OFFSET>,
            SetRunOnLastDayOfMonth: SetRunOnLastDayOfMonth::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonthlyTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetworkSettings {}
#[cfg(feature = "Win32_System_Com")]
impl INetworkSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetworkSettings_Vtbl
    where
        Identity: INetworkSettings_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetworkSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetworkSettings_Impl::Name(this, core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetworkSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetworkSettings_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetworkSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetworkSettings_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetworkSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetworkSettings_Impl::SetId(this, core::mem::transmute(&id)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrincipal_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayName(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogonType(&self, plogon: *mut TASK_LOGON_TYPE) -> windows_core::Result<()>;
    fn SetLogonType(&self, logon: TASK_LOGON_TYPE) -> windows_core::Result<()>;
    fn GroupId(&self, pgroup: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetGroupId(&self, group: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RunLevel(&self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> windows_core::Result<()>;
    fn SetRunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrincipal {}
#[cfg(feature = "Win32_System_Com")]
impl IPrincipal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrincipal_Vtbl
    where
        Identity: IPrincipal_Impl,
    {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::SetId(this, core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::DisplayName(this, core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::SetDisplayName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn UserId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::UserId(this, core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::SetUserId(this, core::mem::transmute(&user)).into()
        }
        unsafe extern "system" fn LogonType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::LogonType(this, core::mem::transmute_copy(&plogon)).into()
        }
        unsafe extern "system" fn SetLogonType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logon: TASK_LOGON_TYPE) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::SetLogonType(this, core::mem::transmute_copy(&logon)).into()
        }
        unsafe extern "system" fn GroupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroup: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::GroupId(this, core::mem::transmute_copy(&pgroup)).into()
        }
        unsafe extern "system" fn SetGroupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::SetGroupId(this, core::mem::transmute(&group)).into()
        }
        unsafe extern "system" fn RunLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::RunLevel(this, core::mem::transmute_copy(&prunlevel)).into()
        }
        unsafe extern "system" fn SetRunLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT
        where
            Identity: IPrincipal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal_Impl::SetRunLevel(this, core::mem::transmute_copy(&runlevel)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            UserId: UserId::<Identity, OFFSET>,
            SetUserId: SetUserId::<Identity, OFFSET>,
            LogonType: LogonType::<Identity, OFFSET>,
            SetLogonType: SetLogonType::<Identity, OFFSET>,
            GroupId: GroupId::<Identity, OFFSET>,
            SetGroupId: SetGroupId::<Identity, OFFSET>,
            RunLevel: RunLevel::<Identity, OFFSET>,
            SetRunLevel: SetRunLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrincipal as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrincipal2_Impl: Sized + super::Com::IDispatch_Impl {
    fn ProcessTokenSidType(&self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()>;
    fn SetProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()>;
    fn RequiredPrivilegeCount(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_RequiredPrivilege(&self, index: i32, pprivilege: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddRequiredPrivilege(&self, privilege: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrincipal2 {}
#[cfg(feature = "Win32_System_Com")]
impl IPrincipal2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrincipal2_Vtbl
    where
        Identity: IPrincipal2_Impl,
    {
        unsafe extern "system" fn ProcessTokenSidType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT
        where
            Identity: IPrincipal2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal2_Impl::ProcessTokenSidType(this, core::mem::transmute_copy(&pprocesstokensidtype)).into()
        }
        unsafe extern "system" fn SetProcessTokenSidType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT
        where
            Identity: IPrincipal2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal2_Impl::SetProcessTokenSidType(this, core::mem::transmute_copy(&processtokensidtype)).into()
        }
        unsafe extern "system" fn RequiredPrivilegeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrincipal2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal2_Impl::RequiredPrivilegeCount(this, core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_RequiredPrivilege<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprivilege: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal2_Impl::get_RequiredPrivilege(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pprivilege)).into()
        }
        unsafe extern "system" fn AddRequiredPrivilege<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, privilege: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrincipal2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrincipal2_Impl::AddRequiredPrivilege(this, core::mem::transmute(&privilege)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ProcessTokenSidType: ProcessTokenSidType::<Identity, OFFSET>,
            SetProcessTokenSidType: SetProcessTokenSidType::<Identity, OFFSET>,
            RequiredPrivilegeCount: RequiredPrivilegeCount::<Identity, OFFSET>,
            get_RequiredPrivilege: get_RequiredPrivilege::<Identity, OFFSET>,
            AddRequiredPrivilege: AddRequiredPrivilege::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrincipal2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Controls")]
pub trait IProvideTaskPage_Impl: Sized {
    fn GetPage(&self, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL) -> windows_core::Result<super::super::UI::Controls::HPROPSHEETPAGE>;
}
#[cfg(feature = "Win32_UI_Controls")]
impl windows_core::RuntimeName for IProvideTaskPage {}
#[cfg(feature = "Win32_UI_Controls")]
impl IProvideTaskPage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvideTaskPage_Vtbl
    where
        Identity: IProvideTaskPage_Impl,
    {
        unsafe extern "system" fn GetPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT
        where
            Identity: IProvideTaskPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProvideTaskPage_Impl::GetPage(this, core::mem::transmute_copy(&tptype), core::mem::transmute_copy(&fpersistchanges)) {
                Ok(ok__) => {
                    phpage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPage: GetPage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideTaskPage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegisteredTask_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<TASK_STATE>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Run(&self, params: &windows_core::VARIANT) -> windows_core::Result<IRunningTask>;
    fn RunEx(&self, params: &windows_core::VARIANT, flags: i32, sessionid: i32, user: &windows_core::BSTR) -> windows_core::Result<IRunningTask>;
    fn GetInstances(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection>;
    fn LastRunTime(&self) -> windows_core::Result<f64>;
    fn LastTaskResult(&self) -> windows_core::Result<i32>;
    fn NumberOfMissedRuns(&self) -> windows_core::Result<i32>;
    fn NextRunTime(&self) -> windows_core::Result<f64>;
    fn Definition(&self) -> windows_core::Result<ITaskDefinition>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
    fn Stop(&self, flags: i32) -> windows_core::Result<()>;
    fn GetRunTimes(&self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRegisteredTask {}
#[cfg(feature = "Win32_System_Com")]
impl IRegisteredTask_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRegisteredTask_Vtbl
    where
        Identity: IRegisteredTask_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::Name(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::Path(this) {
                Ok(ok__) => {
                    ppath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TASK_STATE) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::Enabled(this) {
                Ok(ok__) => {
                    penabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredTask_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Run<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: core::mem::MaybeUninit<windows_core::VARIANT>, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::Run(this, core::mem::transmute(&params)) {
                Ok(ok__) => {
                    pprunningtask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: core::mem::MaybeUninit<windows_core::VARIANT>, flags: i32, sessionid: i32, user: core::mem::MaybeUninit<windows_core::BSTR>, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::RunEx(this, core::mem::transmute(&params), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&sessionid), core::mem::transmute(&user)) {
                Ok(ok__) => {
                    pprunningtask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::GetInstances(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pprunningtasks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRunTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastruntime: *mut f64) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::LastRunTime(this) {
                Ok(ok__) => {
                    plastruntime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTaskResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasttaskresult: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::LastTaskResult(this) {
                Ok(ok__) => {
                    plasttaskresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfMissedRuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumberofmissedruns: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::NumberOfMissedRuns(this) {
                Ok(ok__) => {
                    pnumberofmissedruns.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRunTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnextruntime: *mut f64) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::NextRunTime(this) {
                Ok(ok__) => {
                    pnextruntime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Definition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::Definition(this) {
                Ok(ok__) => {
                    ppdefinition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::Xml(this) {
                Ok(ok__) => {
                    pxml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinformation: i32, psddl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTask_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&securityinformation)) {
                Ok(ok__) => {
                    psddl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredTask_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredTask_Impl::Stop(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetRunTimes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IRegisteredTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredTask_Impl::GetRunTimes(this, core::mem::transmute_copy(&pststart), core::mem::transmute_copy(&pstend), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pruntimes)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            RunEx: RunEx::<Identity, OFFSET>,
            GetInstances: GetInstances::<Identity, OFFSET>,
            LastRunTime: LastRunTime::<Identity, OFFSET>,
            LastTaskResult: LastTaskResult::<Identity, OFFSET>,
            NumberOfMissedRuns: NumberOfMissedRuns::<Identity, OFFSET>,
            NextRunTime: NextRunTime::<Identity, OFFSET>,
            Definition: Definition::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredTask as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegisteredTaskCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRegisteredTaskCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IRegisteredTaskCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRegisteredTaskCollection_Vtbl
    where
        Identity: IRegisteredTaskCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRegisteredTaskCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTaskCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, ppregisteredtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredTaskCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTaskCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    ppregisteredtask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredTaskCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredTaskCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredTaskCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegistrationInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Author(&self, pauthor: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetAuthor(&self, author: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self, pversion: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetVersion(&self, version: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Date(&self, pdate: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDate(&self, date: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Documentation(&self, pdocumentation: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDocumentation(&self, documentation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn URI(&self, puri: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetURI(&self, uri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SecurityDescriptor(&self, psddl: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetSecurityDescriptor(&self, sddl: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Source(&self, psource: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSource(&self, source: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRegistrationInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IRegistrationInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRegistrationInfo_Vtbl
    where
        Identity: IRegistrationInfo_Impl,
    {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::Description(this, core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetDescription(this, core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn Author<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthor: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::Author(this, core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn SetAuthor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, author: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetAuthor(this, core::mem::transmute(&author)).into()
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::Version(this, core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetVersion(this, core::mem::transmute(&version)).into()
        }
        unsafe extern "system" fn Date<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::Date(this, core::mem::transmute_copy(&pdate)).into()
        }
        unsafe extern "system" fn SetDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, date: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetDate(this, core::mem::transmute(&date)).into()
        }
        unsafe extern "system" fn Documentation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdocumentation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::Documentation(this, core::mem::transmute_copy(&pdocumentation)).into()
        }
        unsafe extern "system" fn SetDocumentation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentation: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetDocumentation(this, core::mem::transmute(&documentation)).into()
        }
        unsafe extern "system" fn XmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::XmlText(this, core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetXmlText(this, core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn URI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::URI(this, core::mem::transmute_copy(&puri)).into()
        }
        unsafe extern "system" fn SetURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetURI(this, core::mem::transmute(&uri)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psddl: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SecurityDescriptor(this, core::mem::transmute_copy(&psddl)).into()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl)).into()
        }
        unsafe extern "system" fn Source<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::Source(this, core::mem::transmute_copy(&psource)).into()
        }
        unsafe extern "system" fn SetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationInfo_Impl::SetSource(this, core::mem::transmute(&source)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            SetAuthor: SetAuthor::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            Date: Date::<Identity, OFFSET>,
            SetDate: SetDate::<Identity, OFFSET>,
            Documentation: Documentation::<Identity, OFFSET>,
            SetDocumentation: SetDocumentation::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
            URI: URI::<Identity, OFFSET>,
            SetURI: SetURI::<Identity, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            Source: Source::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegistrationInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegistrationTrigger_Impl: Sized + ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRegistrationTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IRegistrationTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRegistrationTrigger_Vtbl
    where
        Identity: IRegistrationTrigger_Impl,
    {
        unsafe extern "system" fn Delay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRegistrationTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegistrationTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>(), Delay: Delay::<Identity, OFFSET>, SetDelay: SetDelay::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegistrationTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRepetitionPattern_Impl: Sized + super::Com::IDispatch_Impl {
    fn Interval(&self, pinterval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetInterval(&self, interval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Duration(&self, pduration: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopAtDurationEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStopAtDurationEnd(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRepetitionPattern {}
#[cfg(feature = "Win32_System_Com")]
impl IRepetitionPattern_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRepetitionPattern_Vtbl
    where
        Identity: IRepetitionPattern_Impl,
    {
        unsafe extern "system" fn Interval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinterval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRepetitionPattern_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRepetitionPattern_Impl::Interval(this, core::mem::transmute_copy(&pinterval)).into()
        }
        unsafe extern "system" fn SetInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRepetitionPattern_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRepetitionPattern_Impl::SetInterval(this, core::mem::transmute(&interval)).into()
        }
        unsafe extern "system" fn Duration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduration: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRepetitionPattern_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRepetitionPattern_Impl::Duration(this, core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRepetitionPattern_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRepetitionPattern_Impl::SetDuration(this, core::mem::transmute(&duration)).into()
        }
        unsafe extern "system" fn StopAtDurationEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRepetitionPattern_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRepetitionPattern_Impl::StopAtDurationEnd(this, core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopAtDurationEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRepetitionPattern_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRepetitionPattern_Impl::SetStopAtDurationEnd(this, core::mem::transmute_copy(&stop)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Interval: Interval::<Identity, OFFSET>,
            SetInterval: SetInterval::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            StopAtDurationEnd: StopAtDurationEnd::<Identity, OFFSET>,
            SetStopAtDurationEnd: SetStopAtDurationEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRepetitionPattern as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRunningTask_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InstanceGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<TASK_STATE>;
    fn CurrentAction(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn EnginePID(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRunningTask {}
#[cfg(feature = "Win32_System_Com")]
impl IRunningTask_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRunningTask_Vtbl
    where
        Identity: IRunningTask_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTask_Impl::Name(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTask_Impl::InstanceGuid(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTask_Impl::Path(this) {
                Ok(ok__) => {
                    ppath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TASK_STATE) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTask_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTask_Impl::CurrentAction(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunningTask_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunningTask_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn EnginePID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRunningTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTask_Impl::EnginePID(this) {
                Ok(ok__) => {
                    ppid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            InstanceGuid: InstanceGuid::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            EnginePID: EnginePID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningTask as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRunningTaskCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<IRunningTask>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRunningTaskCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IRunningTaskCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRunningTaskCollection_Vtbl
    where
        Identity: IRunningTaskCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRunningTaskCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTaskCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunningTaskCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTaskCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    pprunningtask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunningTaskCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningTaskCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningTaskCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IScheduledWorkItem_Impl: Sized {
    fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut Option<ITaskTrigger>) -> windows_core::Result<()>;
    fn DeleteTrigger(&self, itrigger: u16) -> windows_core::Result<()>;
    fn GetTriggerCount(&self) -> windows_core::Result<u16>;
    fn GetTrigger(&self, itrigger: u16) -> windows_core::Result<ITaskTrigger>;
    fn GetTriggerString(&self, itrigger: u16) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRunTimes(&self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetNextRunTime(&self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> windows_core::Result<()>;
    fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> windows_core::Result<()>;
    fn Run(&self) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
    fn EditWorkItem(&self, hparent: super::super::Foundation::HWND, dwreserved: u32) -> windows_core::Result<()>;
    fn GetMostRecentRunTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetStatus(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetExitCode(&self) -> windows_core::Result<u32>;
    fn SetComment(&self, pwszcomment: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetComment(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCreator(&self, pwszcreator: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> windows_core::Result<()>;
    fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> windows_core::Result<()>;
    fn SetErrorRetryCount(&self, wretrycount: u16) -> windows_core::Result<()>;
    fn GetErrorRetryCount(&self) -> windows_core::Result<u16>;
    fn SetErrorRetryInterval(&self, wretryinterval: u16) -> windows_core::Result<()>;
    fn GetErrorRetryInterval(&self) -> windows_core::Result<u16>;
    fn SetFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn SetAccountInformation(&self, pwszaccountname: &windows_core::PCWSTR, pwszpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccountInformation(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IScheduledWorkItem {}
impl IScheduledWorkItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IScheduledWorkItem_Vtbl
    where
        Identity: IScheduledWorkItem_Impl,
    {
        unsafe extern "system" fn CreateTrigger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::CreateTrigger(this, core::mem::transmute_copy(&pinewtrigger), core::mem::transmute_copy(&pptrigger)).into()
        }
        unsafe extern "system" fn DeleteTrigger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrigger: u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::DeleteTrigger(this, core::mem::transmute_copy(&itrigger)).into()
        }
        unsafe extern "system" fn GetTriggerCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcount: *mut u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetTriggerCount(this) {
                Ok(ok__) => {
                    pwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrigger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrigger: u16, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetTrigger(this, core::mem::transmute_copy(&itrigger)) {
                Ok(ok__) => {
                    pptrigger.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriggerString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetTriggerString(this, core::mem::transmute_copy(&itrigger)) {
                Ok(ok__) => {
                    ppwsztrigger.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunTimes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::GetRunTimes(this, core::mem::transmute_copy(&pstbegin), core::mem::transmute_copy(&pstend), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&rgsttasktimes)).into()
        }
        unsafe extern "system" fn GetNextRunTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::GetNextRunTime(this, core::mem::transmute_copy(&pstnextrun)).into()
        }
        unsafe extern "system" fn SetIdleWait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetIdleWait(this, core::mem::transmute_copy(&widleminutes), core::mem::transmute_copy(&wdeadlineminutes)).into()
        }
        unsafe extern "system" fn GetIdleWait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::GetIdleWait(this, core::mem::transmute_copy(&pwidleminutes), core::mem::transmute_copy(&pwdeadlineminutes)).into()
        }
        unsafe extern "system" fn Run<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::Run(this).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::Terminate(this).into()
        }
        unsafe extern "system" fn EditWorkItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::EditWorkItem(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetMostRecentRunTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetMostRecentRunTime(this) {
                Ok(ok__) => {
                    pstlastrun.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetStatus(this) {
                Ok(ok__) => {
                    phrstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExitCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwexitcode: *mut u32) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetExitCode(this) {
                Ok(ok__) => {
                    pdwexitcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomment: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetComment(this, core::mem::transmute(&pwszcomment)).into()
        }
        unsafe extern "system" fn GetComment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszcomment: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetComment(this) {
                Ok(ok__) => {
                    ppwszcomment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcreator: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetCreator(this, core::mem::transmute(&pwszcreator)).into()
        }
        unsafe extern "system" fn GetCreator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszcreator: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetCreator(this) {
                Ok(ok__) => {
                    ppwszcreator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkItemData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetWorkItemData(this, core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&rgbdata)).into()
        }
        unsafe extern "system" fn GetWorkItemData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::GetWorkItemData(this, core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&prgbdata)).into()
        }
        unsafe extern "system" fn SetErrorRetryCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wretrycount: u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetErrorRetryCount(this, core::mem::transmute_copy(&wretrycount)).into()
        }
        unsafe extern "system" fn GetErrorRetryCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwretrycount: *mut u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetErrorRetryCount(this) {
                Ok(ok__) => {
                    pwretrycount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorRetryInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wretryinterval: u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetErrorRetryInterval(this, core::mem::transmute_copy(&wretryinterval)).into()
        }
        unsafe extern "system" fn GetErrorRetryInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwretryinterval: *mut u16) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetErrorRetryInterval(this) {
                Ok(ok__) => {
                    pwretryinterval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszaccountname: windows_core::PCWSTR, pwszpassword: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduledWorkItem_Impl::SetAccountInformation(this, core::mem::transmute(&pwszaccountname), core::mem::transmute(&pwszpassword)).into()
        }
        unsafe extern "system" fn GetAccountInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszaccountname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IScheduledWorkItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduledWorkItem_Impl::GetAccountInformation(this) {
                Ok(ok__) => {
                    ppwszaccountname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTrigger: CreateTrigger::<Identity, OFFSET>,
            DeleteTrigger: DeleteTrigger::<Identity, OFFSET>,
            GetTriggerCount: GetTriggerCount::<Identity, OFFSET>,
            GetTrigger: GetTrigger::<Identity, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, OFFSET>,
            GetNextRunTime: GetNextRunTime::<Identity, OFFSET>,
            SetIdleWait: SetIdleWait::<Identity, OFFSET>,
            GetIdleWait: GetIdleWait::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            EditWorkItem: EditWorkItem::<Identity, OFFSET>,
            GetMostRecentRunTime: GetMostRecentRunTime::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetExitCode: GetExitCode::<Identity, OFFSET>,
            SetComment: SetComment::<Identity, OFFSET>,
            GetComment: GetComment::<Identity, OFFSET>,
            SetCreator: SetCreator::<Identity, OFFSET>,
            GetCreator: GetCreator::<Identity, OFFSET>,
            SetWorkItemData: SetWorkItemData::<Identity, OFFSET>,
            GetWorkItemData: GetWorkItemData::<Identity, OFFSET>,
            SetErrorRetryCount: SetErrorRetryCount::<Identity, OFFSET>,
            GetErrorRetryCount: GetErrorRetryCount::<Identity, OFFSET>,
            SetErrorRetryInterval: SetErrorRetryInterval::<Identity, OFFSET>,
            GetErrorRetryInterval: GetErrorRetryInterval::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, OFFSET>,
            GetAccountInformation: GetAccountInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScheduledWorkItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISessionStateChangeTrigger_Impl: Sized + ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StateChange(&self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()>;
    fn SetStateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISessionStateChangeTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ISessionStateChangeTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISessionStateChangeTrigger_Vtbl
    where
        Identity: ISessionStateChangeTrigger_Impl,
    {
        unsafe extern "system" fn Delay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISessionStateChangeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionStateChangeTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISessionStateChangeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionStateChangeTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn UserId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISessionStateChangeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionStateChangeTrigger_Impl::UserId(this, core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISessionStateChangeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionStateChangeTrigger_Impl::SetUserId(this, core::mem::transmute(&user)).into()
        }
        unsafe extern "system" fn StateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT
        where
            Identity: ISessionStateChangeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionStateChangeTrigger_Impl::StateChange(this, core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn SetStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT
        where
            Identity: ISessionStateChangeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionStateChangeTrigger_Impl::SetStateChange(this, core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            Delay: Delay::<Identity, OFFSET>,
            SetDelay: SetDelay::<Identity, OFFSET>,
            UserId: UserId::<Identity, OFFSET>,
            SetUserId: SetUserId::<Identity, OFFSET>,
            StateChange: StateChange::<Identity, OFFSET>,
            SetStateChange: SetStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISessionStateChangeTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShowMessageAction_Impl: Sized + IAction_Impl {
    fn Title(&self, ptitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTitle(&self, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MessageBody(&self, pmessagebody: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetMessageBody(&self, messagebody: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IShowMessageAction {}
#[cfg(feature = "Win32_System_Com")]
impl IShowMessageAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IShowMessageAction_Vtbl
    where
        Identity: IShowMessageAction_Impl,
    {
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IShowMessageAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IShowMessageAction_Impl::Title(this, core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IShowMessageAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IShowMessageAction_Impl::SetTitle(this, core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn MessageBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessagebody: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IShowMessageAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IShowMessageAction_Impl::MessageBody(this, core::mem::transmute_copy(&pmessagebody)).into()
        }
        unsafe extern "system" fn SetMessageBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagebody: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IShowMessageAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IShowMessageAction_Impl::SetMessageBody(this, core::mem::transmute(&messagebody)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            Title: Title::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            MessageBody: MessageBody::<Identity, OFFSET>,
            SetMessageBody: SetMessageBody::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShowMessageAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
pub trait ITask_Impl: Sized + IScheduledWorkItem_Impl {
    fn SetApplicationName(&self, pwszapplicationname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetApplicationName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetParameters(&self, pwszparameters: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetParameters(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetWorkingDirectory(&self, pwszworkingdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetWorkingDirectory(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetPriority(&self, dwpriority: u32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<u32>;
    fn SetTaskFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetTaskFlags(&self) -> windows_core::Result<u32>;
    fn SetMaxRunTime(&self, dwmaxruntimems: u32) -> windows_core::Result<()>;
    fn GetMaxRunTime(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ITask {}
impl ITask_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITask_Vtbl
    where
        Identity: ITask_Impl,
    {
        unsafe extern "system" fn SetApplicationName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszapplicationname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITask_Impl::SetApplicationName(this, core::mem::transmute(&pwszapplicationname)).into()
        }
        unsafe extern "system" fn GetApplicationName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszapplicationname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITask_Impl::GetApplicationName(this) {
                Ok(ok__) => {
                    ppwszapplicationname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszparameters: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITask_Impl::SetParameters(this, core::mem::transmute(&pwszparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITask_Impl::GetParameters(this) {
                Ok(ok__) => {
                    ppwszparameters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszworkingdirectory: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITask_Impl::SetWorkingDirectory(this, core::mem::transmute(&pwszworkingdirectory)).into()
        }
        unsafe extern "system" fn GetWorkingDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszworkingdirectory: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITask_Impl::GetWorkingDirectory(this) {
                Ok(ok__) => {
                    ppwszworkingdirectory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpriority: u32) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITask_Impl::SetPriority(this, core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpriority: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITask_Impl::GetPriority(this) {
                Ok(ok__) => {
                    pdwpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITask_Impl::SetTaskFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTaskFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITask_Impl::GetTaskFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRunTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxruntimems: u32) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITask_Impl::SetMaxRunTime(this, core::mem::transmute_copy(&dwmaxruntimems)).into()
        }
        unsafe extern "system" fn GetMaxRunTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxruntimems: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITask_Impl::GetMaxRunTime(this) {
                Ok(ok__) => {
                    pdwmaxruntimems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IScheduledWorkItem_Vtbl::new::<Identity, OFFSET>(),
            SetApplicationName: SetApplicationName::<Identity, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            GetParameters: GetParameters::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            GetWorkingDirectory: GetWorkingDirectory::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetTaskFlags: SetTaskFlags::<Identity, OFFSET>,
            GetTaskFlags: GetTaskFlags::<Identity, OFFSET>,
            SetMaxRunTime: SetMaxRunTime::<Identity, OFFSET>,
            GetMaxRunTime: GetMaxRunTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITask as windows_core::Interface>::IID || iid == &<IScheduledWorkItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskDefinition_Impl: Sized + super::Com::IDispatch_Impl {
    fn RegistrationInfo(&self) -> windows_core::Result<IRegistrationInfo>;
    fn SetRegistrationInfo(&self, pregistrationinfo: Option<&IRegistrationInfo>) -> windows_core::Result<()>;
    fn Triggers(&self) -> windows_core::Result<ITriggerCollection>;
    fn SetTriggers(&self, ptriggers: Option<&ITriggerCollection>) -> windows_core::Result<()>;
    fn Settings(&self) -> windows_core::Result<ITaskSettings>;
    fn SetSettings(&self, psettings: Option<&ITaskSettings>) -> windows_core::Result<()>;
    fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetData(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Principal(&self) -> windows_core::Result<IPrincipal>;
    fn SetPrincipal(&self, pprincipal: Option<&IPrincipal>) -> windows_core::Result<()>;
    fn Actions(&self) -> windows_core::Result<IActionCollection>;
    fn SetActions(&self, pactions: Option<&IActionCollection>) -> windows_core::Result<()>;
    fn XmlText(&self, pxml: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, xml: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskDefinition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskDefinition_Vtbl
    where
        Identity: ITaskDefinition_Impl,
    {
        unsafe extern "system" fn RegistrationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregistrationinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskDefinition_Impl::RegistrationInfo(this) {
                Ok(ok__) => {
                    ppregistrationinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistrationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pregistrationinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::SetRegistrationInfo(this, windows_core::from_raw_borrowed(&pregistrationinfo)).into()
        }
        unsafe extern "system" fn Triggers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptriggers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskDefinition_Impl::Triggers(this) {
                Ok(ok__) => {
                    pptriggers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriggers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptriggers: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::SetTriggers(this, windows_core::from_raw_borrowed(&ptriggers)).into()
        }
        unsafe extern "system" fn Settings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskDefinition_Impl::Settings(this) {
                Ok(ok__) => {
                    ppsettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psettings: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::SetSettings(this, windows_core::from_raw_borrowed(&psettings)).into()
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::Data(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::SetData(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn Principal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprincipal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskDefinition_Impl::Principal(this) {
                Ok(ok__) => {
                    ppprincipal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrincipal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprincipal: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::SetPrincipal(this, windows_core::from_raw_borrowed(&pprincipal)).into()
        }
        unsafe extern "system" fn Actions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppactions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskDefinition_Impl::Actions(this) {
                Ok(ok__) => {
                    ppactions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactions: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::SetActions(this, windows_core::from_raw_borrowed(&pactions)).into()
        }
        unsafe extern "system" fn XmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::XmlText(this, core::mem::transmute_copy(&pxml)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskDefinition_Impl::SetXmlText(this, core::mem::transmute(&xml)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RegistrationInfo: RegistrationInfo::<Identity, OFFSET>,
            SetRegistrationInfo: SetRegistrationInfo::<Identity, OFFSET>,
            Triggers: Triggers::<Identity, OFFSET>,
            SetTriggers: SetTriggers::<Identity, OFFSET>,
            Settings: Settings::<Identity, OFFSET>,
            SetSettings: SetSettings::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            Principal: Principal::<Identity, OFFSET>,
            SetPrincipal: SetPrincipal::<Identity, OFFSET>,
            Actions: Actions::<Identity, OFFSET>,
            SetActions: SetActions::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskDefinition as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskFolder_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder>;
    fn GetFolders(&self, flags: i32) -> windows_core::Result<ITaskFolderCollection>;
    fn CreateFolder(&self, subfoldername: &windows_core::BSTR, sddl: &windows_core::VARIANT) -> windows_core::Result<ITaskFolder>;
    fn DeleteFolder(&self, subfoldername: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
    fn GetTask(&self, path: &windows_core::BSTR) -> windows_core::Result<IRegisteredTask>;
    fn GetTasks(&self, flags: i32) -> windows_core::Result<IRegisteredTaskCollection>;
    fn DeleteTask(&self, name: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
    fn RegisterTask(&self, path: &windows_core::BSTR, xmltext: &windows_core::BSTR, flags: i32, userid: &windows_core::VARIANT, password: &windows_core::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &windows_core::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn RegisterTaskDefinition(&self, path: &windows_core::BSTR, pdefinition: Option<&ITaskDefinition>, flags: i32, userid: &windows_core::VARIANT, password: &windows_core::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &windows_core::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskFolder {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskFolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskFolder_Vtbl
    where
        Identity: ITaskFolder_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::Name(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::Path(this) {
                Ok(ok__) => {
                    ppath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::GetFolder(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    ppfolder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, ppfolders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::GetFolders(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppfolders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subfoldername: core::mem::MaybeUninit<windows_core::BSTR>, sddl: core::mem::MaybeUninit<windows_core::VARIANT>, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::CreateFolder(this, core::mem::transmute(&subfoldername), core::mem::transmute(&sddl)) {
                Ok(ok__) => {
                    ppfolder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subfoldername: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskFolder_Impl::DeleteFolder(this, core::mem::transmute(&subfoldername), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::GetTask(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    pptask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pptasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::GetTasks(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pptasks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskFolder_Impl::DeleteTask(this, core::mem::transmute(&name), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn RegisterTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, xmltext: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32, userid: core::mem::MaybeUninit<windows_core::VARIANT>, password: core::mem::MaybeUninit<windows_core::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: core::mem::MaybeUninit<windows_core::VARIANT>, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::RegisterTask(this, core::mem::transmute(&path), core::mem::transmute(&xmltext), core::mem::transmute_copy(&flags), core::mem::transmute(&userid), core::mem::transmute(&password), core::mem::transmute_copy(&logontype), core::mem::transmute(&sddl)) {
                Ok(ok__) => {
                    pptask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterTaskDefinition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, pdefinition: *mut core::ffi::c_void, flags: i32, userid: core::mem::MaybeUninit<windows_core::VARIANT>, password: core::mem::MaybeUninit<windows_core::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: core::mem::MaybeUninit<windows_core::VARIANT>, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::RegisterTaskDefinition(this, core::mem::transmute(&path), windows_core::from_raw_borrowed(&pdefinition), core::mem::transmute_copy(&flags), core::mem::transmute(&userid), core::mem::transmute(&password), core::mem::transmute_copy(&logontype), core::mem::transmute(&sddl)) {
                Ok(ok__) => {
                    pptask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinformation: i32, psddl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolder_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&securityinformation)) {
                Ok(ok__) => {
                    psddl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32) -> windows_core::HRESULT
        where
            Identity: ITaskFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskFolder_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl), core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetFolders: GetFolders::<Identity, OFFSET>,
            CreateFolder: CreateFolder::<Identity, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, OFFSET>,
            GetTask: GetTask::<Identity, OFFSET>,
            GetTasks: GetTasks::<Identity, OFFSET>,
            DeleteTask: DeleteTask::<Identity, OFFSET>,
            RegisterTask: RegisterTask::<Identity, OFFSET>,
            RegisterTaskDefinition: RegisterTaskDefinition::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskFolder as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskFolderCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<ITaskFolder>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskFolderCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskFolderCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskFolderCollection_Vtbl
    where
        Identity: ITaskFolderCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITaskFolderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolderCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolderCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    ppfolder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskFolderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskFolderCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskFolderCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ITaskHandler_Impl: Sized {
    fn Start(&self, phandlerservices: Option<&windows_core::IUnknown>, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITaskHandler {}
impl ITaskHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskHandler_Vtbl
    where
        Identity: ITaskHandler_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandlerservices: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskHandler_Impl::Start(this, windows_core::from_raw_borrowed(&phandlerservices), core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretcode: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITaskHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskHandler_Impl::Stop(this) {
                Ok(ok__) => {
                    pretcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskHandler_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskHandler_Impl::Resume(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskHandler as windows_core::Interface>::IID
    }
}
pub trait ITaskHandlerStatus_Impl: Sized {
    fn UpdateStatus(&self, percentcomplete: i16, statusmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskCompleted(&self, taskerrcode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITaskHandlerStatus {}
impl ITaskHandlerStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskHandlerStatus_Vtbl
    where
        Identity: ITaskHandlerStatus_Impl,
    {
        unsafe extern "system" fn UpdateStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, percentcomplete: i16, statusmessage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskHandlerStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskHandlerStatus_Impl::UpdateStatus(this, core::mem::transmute_copy(&percentcomplete), core::mem::transmute(&statusmessage)).into()
        }
        unsafe extern "system" fn TaskCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskerrcode: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITaskHandlerStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskHandlerStatus_Impl::TaskCompleted(this, core::mem::transmute_copy(&taskerrcode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UpdateStatus: UpdateStatus::<Identity, OFFSET>,
            TaskCompleted: TaskCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskHandlerStatus as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskNamedValueCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_Item(&self, index: i32) -> windows_core::Result<ITaskNamedValuePair>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Create(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<ITaskNamedValuePair>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskNamedValueCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskNamedValueCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskNamedValueCollection_Vtbl
    where
        Identity: ITaskNamedValueCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValueCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskNamedValueCollection_Impl::Count(this, core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pppair: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValueCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskNamedValueCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pppair.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValueCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskNamedValueCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::BSTR>, pppair: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValueCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskNamedValueCollection_Impl::Create(this, core::mem::transmute(&name), core::mem::transmute(&value)) {
                Ok(ok__) => {
                    pppair.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValueCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskNamedValueCollection_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValueCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskNamedValueCollection_Impl::Clear(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskNamedValueCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskNamedValuePair_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self, pvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskNamedValuePair {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskNamedValuePair_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskNamedValuePair_Vtbl
    where
        Identity: ITaskNamedValuePair_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValuePair_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskNamedValuePair_Impl::Name(this, core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValuePair_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskNamedValuePair_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValuePair_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskNamedValuePair_Impl::Value(this, core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskNamedValuePair_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskNamedValuePair_Impl::SetValue(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskNamedValuePair as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ITaskScheduler_Impl: Sized {
    fn SetTargetComputer(&self, pwszcomputer: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTargetComputer(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Enum(&self) -> windows_core::Result<IEnumWorkItems>;
    fn Activate(&self, pwszname: &windows_core::PCWSTR, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn Delete(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn NewWorkItem(&self, pwsztaskname: &windows_core::PCWSTR, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn AddWorkItem(&self, pwsztaskname: &windows_core::PCWSTR, pworkitem: Option<&IScheduledWorkItem>) -> windows_core::Result<()>;
    fn IsOfType(&self, pwszname: &windows_core::PCWSTR, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITaskScheduler {}
impl ITaskScheduler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskScheduler_Vtbl
    where
        Identity: ITaskScheduler_Impl,
    {
        unsafe extern "system" fn SetTargetComputer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomputer: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskScheduler_Impl::SetTargetComputer(this, core::mem::transmute(&pwszcomputer)).into()
        }
        unsafe extern "system" fn GetTargetComputer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszcomputer: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskScheduler_Impl::GetTargetComputer(this) {
                Ok(ok__) => {
                    ppwszcomputer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumworkitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskScheduler_Impl::Enum(this) {
                Ok(ok__) => {
                    ppenumworkitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskScheduler_Impl::Activate(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskScheduler_Impl::Delete(this, core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn NewWorkItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztaskname: windows_core::PCWSTR, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskScheduler_Impl::NewWorkItem(this, core::mem::transmute(&pwsztaskname), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWorkItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztaskname: windows_core::PCWSTR, pworkitem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskScheduler_Impl::AddWorkItem(this, core::mem::transmute(&pwsztaskname), windows_core::from_raw_borrowed(&pworkitem)).into()
        }
        unsafe extern "system" fn IsOfType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, riid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITaskScheduler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskScheduler_Impl::IsOfType(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTargetComputer: SetTargetComputer::<Identity, OFFSET>,
            GetTargetComputer: GetTargetComputer::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            NewWorkItem: NewWorkItem::<Identity, OFFSET>,
            AddWorkItem: AddWorkItem::<Identity, OFFSET>,
            IsOfType: IsOfType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskScheduler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskService_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder>;
    fn GetRunningTasks(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection>;
    fn NewTask(&self, flags: u32) -> windows_core::Result<ITaskDefinition>;
    fn Connect(&self, servername: &windows_core::VARIANT, user: &windows_core::VARIANT, domain: &windows_core::VARIANT, password: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Connected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TargetServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ConnectedUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ConnectedDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HighestVersion(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskService {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskService_Vtbl
    where
        Identity: ITaskService_Impl,
    {
        unsafe extern "system" fn GetFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::GetFolder(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    ppfolder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningTasks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::GetRunningTasks(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pprunningtasks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32, ppdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::NewTask(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppdefinition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servername: core::mem::MaybeUninit<windows_core::VARIANT>, user: core::mem::MaybeUninit<windows_core::VARIANT>, domain: core::mem::MaybeUninit<windows_core::VARIANT>, password: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskService_Impl::Connect(this, core::mem::transmute(&servername), core::mem::transmute(&user), core::mem::transmute(&domain), core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn Connected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::Connected(this) {
                Ok(ok__) => {
                    pconnected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::TargetServer(this) {
                Ok(ok__) => {
                    pserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::ConnectedUser(this) {
                Ok(ok__) => {
                    puser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomain: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::ConnectedDomain(this) {
                Ok(ok__) => {
                    pdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighestVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITaskService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskService_Impl::HighestVersion(this) {
                Ok(ok__) => {
                    pversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetRunningTasks: GetRunningTasks::<Identity, OFFSET>,
            NewTask: NewTask::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Connected: Connected::<Identity, OFFSET>,
            TargetServer: TargetServer::<Identity, OFFSET>,
            ConnectedUser: ConnectedUser::<Identity, OFFSET>,
            ConnectedDomain: ConnectedDomain::<Identity, OFFSET>,
            HighestVersion: HighestVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskService as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn AllowDemandStart(&self, pallowdemandstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetAllowDemandStart(&self, allowdemandstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RestartInterval(&self, prestartinterval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestartInterval(&self, restartinterval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RestartCount(&self, prestartcount: *mut i32) -> windows_core::Result<()>;
    fn SetRestartCount(&self, restartcount: i32) -> windows_core::Result<()>;
    fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> windows_core::Result<()>;
    fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> windows_core::Result<()>;
    fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDisallowStartIfOnBatteries(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowHardTerminate(&self, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetAllowHardTerminate(&self, allowhardterminate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn StartWhenAvailable(&self, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStartWhenAvailable(&self, startwhenavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetExecutionTimeLimit(&self, executiontimelimit: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDeleteExpiredTaskAfter(&self, expirationdelay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Priority(&self, ppriority: *mut i32) -> windows_core::Result<()>;
    fn SetPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> windows_core::Result<()>;
    fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> windows_core::Result<()>;
    fn Hidden(&self, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetHidden(&self, hidden: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IdleSettings(&self) -> windows_core::Result<IIdleSettings>;
    fn SetIdleSettings(&self, pidlesettings: Option<&IIdleSettings>) -> windows_core::Result<()>;
    fn RunOnlyIfIdle(&self, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnlyIfIdle(&self, runonlyifidle: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn WakeToRun(&self, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetWakeToRun(&self, wake: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NetworkSettings(&self) -> windows_core::Result<INetworkSettings>;
    fn SetNetworkSettings(&self, pnetworksettings: Option<&INetworkSettings>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskSettings_Vtbl
    where
        Identity: ITaskSettings_Impl,
    {
        unsafe extern "system" fn AllowDemandStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowdemandstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::AllowDemandStart(this, core::mem::transmute_copy(&pallowdemandstart)).into()
        }
        unsafe extern "system" fn SetAllowDemandStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowdemandstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetAllowDemandStart(this, core::mem::transmute_copy(&allowdemandstart)).into()
        }
        unsafe extern "system" fn RestartInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestartinterval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::RestartInterval(this, core::mem::transmute_copy(&prestartinterval)).into()
        }
        unsafe extern "system" fn SetRestartInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restartinterval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetRestartInterval(this, core::mem::transmute(&restartinterval)).into()
        }
        unsafe extern "system" fn RestartCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestartcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::RestartCount(this, core::mem::transmute_copy(&prestartcount)).into()
        }
        unsafe extern "system" fn SetRestartCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restartcount: i32) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetRestartCount(this, core::mem::transmute_copy(&restartcount)).into()
        }
        unsafe extern "system" fn MultipleInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::MultipleInstances(this, core::mem::transmute_copy(&ppolicy)).into()
        }
        unsafe extern "system" fn SetMultipleInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetMultipleInstances(this, core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn StopIfGoingOnBatteries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::StopIfGoingOnBatteries(this, core::mem::transmute_copy(&pstopifonbatteries)).into()
        }
        unsafe extern "system" fn SetStopIfGoingOnBatteries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stopifonbatteries: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetStopIfGoingOnBatteries(this, core::mem::transmute_copy(&stopifonbatteries)).into()
        }
        unsafe extern "system" fn DisallowStartIfOnBatteries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::DisallowStartIfOnBatteries(this, core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartIfOnBatteries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetDisallowStartIfOnBatteries(this, core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn AllowHardTerminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::AllowHardTerminate(this, core::mem::transmute_copy(&pallowhardterminate)).into()
        }
        unsafe extern "system" fn SetAllowHardTerminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowhardterminate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetAllowHardTerminate(this, core::mem::transmute_copy(&allowhardterminate)).into()
        }
        unsafe extern "system" fn StartWhenAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::StartWhenAvailable(this, core::mem::transmute_copy(&pstartwhenavailable)).into()
        }
        unsafe extern "system" fn SetStartWhenAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startwhenavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetStartWhenAvailable(this, core::mem::transmute_copy(&startwhenavailable)).into()
        }
        unsafe extern "system" fn XmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::XmlText(this, core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetXmlText(this, core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn RunOnlyIfNetworkAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::RunOnlyIfNetworkAvailable(this, core::mem::transmute_copy(&prunonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfNetworkAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, runonlyifnetworkavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetRunOnlyIfNetworkAvailable(this, core::mem::transmute_copy(&runonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexecutiontimelimit: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::ExecutionTimeLimit(this, core::mem::transmute_copy(&pexecutiontimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, executiontimelimit: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetExecutionTimeLimit(this, core::mem::transmute(&executiontimelimit)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::Enabled(this, core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DeleteExpiredTaskAfter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpirationdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::DeleteExpiredTaskAfter(this, core::mem::transmute_copy(&pexpirationdelay)).into()
        }
        unsafe extern "system" fn SetDeleteExpiredTaskAfter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, expirationdelay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetDeleteExpiredTaskAfter(this, core::mem::transmute(&expirationdelay)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::Priority(this, core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetPriority(this, core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Compatibility<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::Compatibility(this, core::mem::transmute_copy(&pcompatlevel)).into()
        }
        unsafe extern "system" fn SetCompatibility<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetCompatibility(this, core::mem::transmute_copy(&compatlevel)).into()
        }
        unsafe extern "system" fn Hidden<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::Hidden(this, core::mem::transmute_copy(&phidden)).into()
        }
        unsafe extern "system" fn SetHidden<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hidden: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetHidden(this, core::mem::transmute_copy(&hidden)).into()
        }
        unsafe extern "system" fn IdleSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidlesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskSettings_Impl::IdleSettings(this) {
                Ok(ok__) => {
                    ppidlesettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdleSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidlesettings: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetIdleSettings(this, windows_core::from_raw_borrowed(&pidlesettings)).into()
        }
        unsafe extern "system" fn RunOnlyIfIdle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::RunOnlyIfIdle(this, core::mem::transmute_copy(&prunonlyifidle)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfIdle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, runonlyifidle: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetRunOnlyIfIdle(this, core::mem::transmute_copy(&runonlyifidle)).into()
        }
        unsafe extern "system" fn WakeToRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::WakeToRun(this, core::mem::transmute_copy(&pwake)).into()
        }
        unsafe extern "system" fn SetWakeToRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wake: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetWakeToRun(this, core::mem::transmute_copy(&wake)).into()
        }
        unsafe extern "system" fn NetworkSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworksettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskSettings_Impl::NetworkSettings(this) {
                Ok(ok__) => {
                    ppnetworksettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksettings: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings_Impl::SetNetworkSettings(this, windows_core::from_raw_borrowed(&pnetworksettings)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AllowDemandStart: AllowDemandStart::<Identity, OFFSET>,
            SetAllowDemandStart: SetAllowDemandStart::<Identity, OFFSET>,
            RestartInterval: RestartInterval::<Identity, OFFSET>,
            SetRestartInterval: SetRestartInterval::<Identity, OFFSET>,
            RestartCount: RestartCount::<Identity, OFFSET>,
            SetRestartCount: SetRestartCount::<Identity, OFFSET>,
            MultipleInstances: MultipleInstances::<Identity, OFFSET>,
            SetMultipleInstances: SetMultipleInstances::<Identity, OFFSET>,
            StopIfGoingOnBatteries: StopIfGoingOnBatteries::<Identity, OFFSET>,
            SetStopIfGoingOnBatteries: SetStopIfGoingOnBatteries::<Identity, OFFSET>,
            DisallowStartIfOnBatteries: DisallowStartIfOnBatteries::<Identity, OFFSET>,
            SetDisallowStartIfOnBatteries: SetDisallowStartIfOnBatteries::<Identity, OFFSET>,
            AllowHardTerminate: AllowHardTerminate::<Identity, OFFSET>,
            SetAllowHardTerminate: SetAllowHardTerminate::<Identity, OFFSET>,
            StartWhenAvailable: StartWhenAvailable::<Identity, OFFSET>,
            SetStartWhenAvailable: SetStartWhenAvailable::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
            RunOnlyIfNetworkAvailable: RunOnlyIfNetworkAvailable::<Identity, OFFSET>,
            SetRunOnlyIfNetworkAvailable: SetRunOnlyIfNetworkAvailable::<Identity, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            DeleteExpiredTaskAfter: DeleteExpiredTaskAfter::<Identity, OFFSET>,
            SetDeleteExpiredTaskAfter: SetDeleteExpiredTaskAfter::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Compatibility: Compatibility::<Identity, OFFSET>,
            SetCompatibility: SetCompatibility::<Identity, OFFSET>,
            Hidden: Hidden::<Identity, OFFSET>,
            SetHidden: SetHidden::<Identity, OFFSET>,
            IdleSettings: IdleSettings::<Identity, OFFSET>,
            SetIdleSettings: SetIdleSettings::<Identity, OFFSET>,
            RunOnlyIfIdle: RunOnlyIfIdle::<Identity, OFFSET>,
            SetRunOnlyIfIdle: SetRunOnlyIfIdle::<Identity, OFFSET>,
            WakeToRun: WakeToRun::<Identity, OFFSET>,
            SetWakeToRun: SetWakeToRun::<Identity, OFFSET>,
            NetworkSettings: NetworkSettings::<Identity, OFFSET>,
            SetNetworkSettings: SetNetworkSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskSettings2_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskSettings2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskSettings2_Vtbl
    where
        Identity: ITaskSettings2_Impl,
    {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings2_Impl::DisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings2_Impl::SetDisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings2_Impl::UseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings2_Impl::SetUseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&useunifiedengine)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskSettings3_Impl: Sized + ITaskSettings_Impl {
    fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings>;
    fn SetMaintenanceSettings(&self, pmaintenancesettings: Option<&IMaintenanceSettings>) -> windows_core::Result<()>;
    fn CreateMaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings>;
    fn Volatile(&self, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetVolatile(&self, volatile: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITaskSettings3 {}
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskSettings3_Vtbl
    where
        Identity: ITaskSettings3_Impl,
    {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings3_Impl::DisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings3_Impl::SetDisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings3_Impl::UseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings3_Impl::SetUseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&useunifiedengine)).into()
        }
        unsafe extern "system" fn MaintenanceSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmaintenancesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskSettings3_Impl::MaintenanceSettings(this) {
                Ok(ok__) => {
                    ppmaintenancesettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaintenancesettings: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings3_Impl::SetMaintenanceSettings(this, windows_core::from_raw_borrowed(&pmaintenancesettings)).into()
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmaintenancesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskSettings3_Impl::CreateMaintenanceSettings(this) {
                Ok(ok__) => {
                    ppmaintenancesettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volatile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings3_Impl::Volatile(this, core::mem::transmute_copy(&pvolatile)).into()
        }
        unsafe extern "system" fn SetVolatile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, volatile: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITaskSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskSettings3_Impl::SetVolatile(this, core::mem::transmute_copy(&volatile)).into()
        }
        Self {
            base__: ITaskSettings_Vtbl::new::<Identity, OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, OFFSET>,
            MaintenanceSettings: MaintenanceSettings::<Identity, OFFSET>,
            SetMaintenanceSettings: SetMaintenanceSettings::<Identity, OFFSET>,
            CreateMaintenanceSettings: CreateMaintenanceSettings::<Identity, OFFSET>,
            Volatile: Volatile::<Identity, OFFSET>,
            SetVolatile: SetVolatile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITaskSettings as windows_core::Interface>::IID
    }
}
pub trait ITaskTrigger_Impl: Sized {
    fn SetTrigger(&self, ptrigger: *const TASK_TRIGGER) -> windows_core::Result<()>;
    fn GetTrigger(&self, ptrigger: *mut TASK_TRIGGER) -> windows_core::Result<()>;
    fn GetTriggerString(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ITaskTrigger {}
impl ITaskTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskTrigger_Vtbl
    where
        Identity: ITaskTrigger_Impl,
    {
        unsafe extern "system" fn SetTrigger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> windows_core::HRESULT
        where
            Identity: ITaskTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskTrigger_Impl::SetTrigger(this, core::mem::transmute_copy(&ptrigger)).into()
        }
        unsafe extern "system" fn GetTrigger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> windows_core::HRESULT
        where
            Identity: ITaskTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskTrigger_Impl::GetTrigger(this, core::mem::transmute_copy(&ptrigger)).into()
        }
        unsafe extern "system" fn GetTriggerString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwsztrigger: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITaskTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskTrigger_Impl::GetTriggerString(this) {
                Ok(ok__) => {
                    ppwsztrigger.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTrigger: SetTrigger::<Identity, OFFSET>,
            GetTrigger: GetTrigger::<Identity, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskTrigger as windows_core::Interface>::IID
    }
}
pub trait ITaskVariables_Impl: Sized {
    fn GetInput(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOutput(&self, input: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetContext(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITaskVariables {}
impl ITaskVariables_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITaskVariables_Vtbl
    where
        Identity: ITaskVariables_Impl,
    {
        unsafe extern "system" fn GetInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskVariables_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskVariables_Impl::GetInput(this) {
                Ok(ok__) => {
                    pinput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, input: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskVariables_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITaskVariables_Impl::SetOutput(this, core::mem::transmute(&input)).into()
        }
        unsafe extern "system" fn GetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITaskVariables_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITaskVariables_Impl::GetContext(this) {
                Ok(ok__) => {
                    pcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInput: GetInput::<Identity, OFFSET>,
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskVariables as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITimeTrigger_Impl: Sized + ITrigger_Impl {
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITimeTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ITimeTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITimeTrigger_Vtbl
    where
        Identity: ITimeTrigger_Impl,
    {
        unsafe extern "system" fn RandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITimeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimeTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITimeTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimeTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimeTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITrigger_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> windows_core::Result<()>;
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Repetition(&self) -> windows_core::Result<IRepetitionPattern>;
    fn SetRepetition(&self, prepeat: Option<&IRepetitionPattern>) -> windows_core::Result<()>;
    fn ExecutionTimeLimit(&self, ptimelimit: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetExecutionTimeLimit(&self, timelimit: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartBoundary(&self, pstart: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetStartBoundary(&self, start: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EndBoundary(&self, pend: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetEndBoundary(&self, end: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ITrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITrigger_Vtbl
    where
        Identity: ITrigger_Impl,
    {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::Type(this, core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::SetId(this, core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn Repetition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepeat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITrigger_Impl::Repetition(this) {
                Ok(ok__) => {
                    pprepeat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepetition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prepeat: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::SetRepetition(this, windows_core::from_raw_borrowed(&prepeat)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimelimit: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::ExecutionTimeLimit(this, core::mem::transmute_copy(&ptimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timelimit: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::SetExecutionTimeLimit(this, core::mem::transmute(&timelimit)).into()
        }
        unsafe extern "system" fn StartBoundary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstart: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::StartBoundary(this, core::mem::transmute_copy(&pstart)).into()
        }
        unsafe extern "system" fn SetStartBoundary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::SetStartBoundary(this, core::mem::transmute(&start)).into()
        }
        unsafe extern "system" fn EndBoundary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pend: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::EndBoundary(this, core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetEndBoundary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::SetEndBoundary(this, core::mem::transmute(&end)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::Enabled(this, core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrigger_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Repetition: Repetition::<Identity, OFFSET>,
            SetRepetition: SetRepetition::<Identity, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, OFFSET>,
            StartBoundary: StartBoundary::<Identity, OFFSET>,
            SetStartBoundary: SetStartBoundary::<Identity, OFFSET>,
            EndBoundary: EndBoundary::<Identity, OFFSET>,
            SetEndBoundary: SetEndBoundary::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITriggerCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_Item(&self, index: i32) -> windows_core::Result<ITrigger>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> windows_core::Result<ITrigger>;
    fn Remove(&self, index: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITriggerCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ITriggerCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITriggerCollection_Vtbl
    where
        Identity: ITriggerCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITriggerCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITriggerCollection_Impl::Count(this, core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITriggerCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITriggerCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pptrigger.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITriggerCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITriggerCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITriggerCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITriggerCollection_Impl::Create(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    pptrigger.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITriggerCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITriggerCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITriggerCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITriggerCollection_Impl::Clear(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITriggerCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWeeklyTrigger_Impl: Sized + ITrigger_Impl {
    fn DaysOfWeek(&self, pdays: *mut i16) -> windows_core::Result<()>;
    fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()>;
    fn WeeksInterval(&self, pweeks: *mut i16) -> windows_core::Result<()>;
    fn SetWeeksInterval(&self, weeks: i16) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWeeklyTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl IWeeklyTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWeeklyTrigger_Vtbl
    where
        Identity: IWeeklyTrigger_Impl,
    {
        unsafe extern "system" fn DaysOfWeek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT
        where
            Identity: IWeeklyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWeeklyTrigger_Impl::DaysOfWeek(this, core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT
        where
            Identity: IWeeklyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWeeklyTrigger_Impl::SetDaysOfWeek(this, core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweeks: *mut i16) -> windows_core::HRESULT
        where
            Identity: IWeeklyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWeeklyTrigger_Impl::WeeksInterval(this, core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, weeks: i16) -> windows_core::HRESULT
        where
            Identity: IWeeklyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWeeklyTrigger_Impl::SetWeeksInterval(this, core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWeeklyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWeeklyTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWeeklyTrigger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWeeklyTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Identity, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, OFFSET>,
            WeeksInterval: WeeksInterval::<Identity, OFFSET>,
            SetWeeksInterval: SetWeeksInterval::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeeklyTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
